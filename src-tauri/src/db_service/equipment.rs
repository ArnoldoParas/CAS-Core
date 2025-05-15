use super::model::{Dependencia, DependenciaGrupo, Equipo};
use crate::db_service::DbService;

use firestore::{errors::FirestoreError, struct_path::paths};

impl DbService {
    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn insert(&self, document: Equipo, dependency: String) -> Result<(), FirestoreError> {
        self.client
            .fluent()
            .insert()
            .into(&dependency)
            .document_id(&document.id_equipo)
            .object(&document)
            .execute()
            .await?;

        if let Some(ultima_fecha) = document.ultimo_registro.last() {
            self.update_maintenance_statistics(*ultima_fecha, dependency)
                .await?;
        }

        Ok(())
    }

    pub async fn get_all_equipments(&self, dependency: String) -> Result<Vec<Equipo>, FirestoreError> {
        let equipos = self
            .client
            .fluent()
            .select()
            .from(dependency.as_str())
            .obj()
            .query()
            .await?;
        Ok(equipos)
    }

    pub async fn get_equipment_by_id(
        &self,
        id: &str,
        dependency: &str,
    ) -> Result<Option<Equipo>, FirestoreError> {
        let equipo = self
            .client
            .fluent()
            .select()
            .by_id_in(dependency)
            .obj()
            .one(&id)
            .await?;
        Ok(equipo)
    }

    pub async fn get_dependency_data_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Dependencia>, FirestoreError> {
        let dependency = self
            .client
            .fluent()
            .select()
            .by_id_in("dependencias")
            .obj()
            .one(&id)
            .await?;
        Ok(dependency)
    }

    pub async fn get_all_dependency_names(&self) -> Result<Vec<String>, FirestoreError> {
        let dependencies: Vec<String> = self
            .client
            .fluent()
            .select()
            .from("dependencias")
            .query()
            .await?
            .into_iter()
            .map(|doc| doc.name) // Mapea los resultados para obtener solo los nombres
            .collect();

        Ok(dependencies)
    }

    pub async fn get_dependency_count(&self, dependency: &str) -> Result<u64, FirestoreError> {
        let doc: Option<Dependencia> = self
            .client
            .fluent()
            .select()
            .by_id_in("dependencias")
            .obj()
            .one(dependency)
            .await?;

        let count = doc.unwrap().count;
        Ok(count)

    }

    pub async fn delete_equipment_by_id(
        &self,
        id: String,
        dependency: String,
    ) -> Result<(), FirestoreError> {
        self.client
            .fluent()
            .delete()
            .from(dependency.as_str()) 
            .document_id(id) 
            .execute()
            .await?;

        Ok(())
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn update_document_by_id(
        &self,
        document: Equipo,
        dependency: String,
    ) -> Result<(), FirestoreError> {
        self.client
            .fluent()
            .update()
            .fields(paths!(
                Equipo::equipo,
                Equipo::marca,
                Equipo::modelo,
                Equipo::referencia_externa,
                Equipo::grupo
            ))
            .in_col(&dependency)
            .document_id(&document.id_equipo)
            .object(&document)
            .execute()
            .await?;

        Ok(())
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn update_dependency_by_id(
        &self,
        document: DependenciaGrupo,
        dependency: String,
    ) -> Result<(), FirestoreError> {
        // 1. Obtener la dependencia existente
        let mut existing_dependency: Dependencia = self
                    .client
                    .fluent()
                    .select()
                    .by_id_in("dependencias")
                    .obj()
                    .one(&dependency)
                    .await.unwrap().unwrap();
        // 2. Actualizar el vector `grupos`
        if let Some(existing_group) = existing_dependency
            .grupos
            .iter_mut()
            .find(|g| g.grupo == document.grupo)
        {
            // Si el grupo ya existe, actualiza el encargado
            existing_group.encargado = document.encargado;
        } else {
            // Si el grupo no existe, agrégalo al vector
            existing_dependency.grupos.push(document);
        }

        // 3. Guardar la dependencia actualizada
        self.client
            .fluent()
            .update()
            .fields(paths!(Dependencia::grupos))
            .in_col("dependencias")
            .document_id(&dependency)
            .object(&existing_dependency)
            .execute()
            .await?;

        Ok(())
    }
}
