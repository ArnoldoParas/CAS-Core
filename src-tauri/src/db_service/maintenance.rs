// Aquí puedes incluir métodos específicos para mantenimientos
use crate::db_service::DbService;
use firestore::errors::FirestoreError;
use chrono::{DateTime, Utc};

impl DbService {
    // Métodos relacionados con mantenimientos
    
    #[allow(dependency_on_unit_never_type_fallback)]
    pub async fn register_maintenance(&self, equipo_id: &str, dependency: &str, fecha: DateTime<Utc>) -> Result<(), FirestoreError> {
        let mut equipo = self.get_equipment_by_id(equipo_id, dependency).await.expect("Equipo no encontrado").unwrap();
        
        // Actualizar último registro
        equipo.ultimo_registro.push(fecha);
        
        self.client
            .fluent()
            .update()
            .in_col(dependency)
            .document_id(equipo_id)
            .object(&equipo)
            .execute()
            .await?;
            
        self.update_maintenance_statistics(fecha, dependency.to_string()).await?;
        
        Ok(())
    }
}