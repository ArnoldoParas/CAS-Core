use super::model::{Dependencia, Equipo};
use crate::db_service::DbService;

use firestore::errors::FirestoreError;

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

    pub async fn get_all_equipments(&self) -> Result<Vec<Equipo>, FirestoreError> {
        let equipos = self
            .client
            .fluent()
            .select()
            .from("FIME")
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

    // Otros métodos relacionados con equipos...
    }
}
