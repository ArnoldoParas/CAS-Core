use super::model::Equipo;
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

    // pub async fn get_all_dependencies(&self) -> Result<Vec<String>, FirestoreError> {
    //     // Esta es otra forma de obtener los metadatos
    //     let collection_path = format!("projects/{}/databases/(default)/documents/dependencias", 
    //                                  self.client.project_id());
        
    //     // Obtener los documentos directamente de la API REST
    //     let response = self.client
    //         .get_documents(&collection_path, false)
    //         .await?;
        
    //     // Extraer los IDs de los documentos
    //     let ids: Vec<String> = response.documents
    //         .into_iter()
    //         .filter_map(|doc| {
    //             let path = doc.name;
    //             path.split('/').last().map(String::from)
    //         })
    //         .collect();
        
    //     Ok(ids)
    // }

    // Otros métodos relacionados con equipos...
}
