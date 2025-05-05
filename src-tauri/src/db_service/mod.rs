use dotenv::dotenv;
use firestore::FirestoreDb;
use std::{env::var, sync::OnceLock};
use tauri::async_runtime::Mutex;

// Importar submódulos
pub mod equipment;
pub mod maintenance;
pub mod model;
pub mod statistics;
mod tests;

#[derive(Clone)]
pub struct DbService {
    client: FirestoreDb,
}

impl DbService {
    // Obtener la instancia global de DbService
    pub async fn global() -> &'static Mutex<Option<Self>> {
        static INSTANCE: OnceLock<Mutex<Option<DbService>>> = OnceLock::new();

        INSTANCE.get_or_init(|| Mutex::new(None))
    }

    // Inicializar la conexión global si aún no está inicializada
    pub async fn initialize() -> Result<(), String> {
        let mut global_service = Self::global().await.lock().await;

        if global_service.is_none() {
            dotenv().ok();

            // Check if env vars are set
            let project_id = var("PROJECT_ID")
                .map_err(|_| "PROJECT_ID environment variable not set".to_string())?;

            let credentials_path = var("GOOGLE_APPLICATION_CREDENTIALS").map_err(|_| {
                "GOOGLE_APPLICATION_CREDENTIALS environment variable not set".to_string()
            })?;

            // Verify file exists and is readable
            if !std::path::Path::new(&credentials_path).exists() {
                return Err(format!(
                    "Credentials file not found at: {}",
                    credentials_path
                ));
            }

            // Try to read the file content to verify permissions
            match std::fs::read_to_string(&credentials_path) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to read credentials file: {}", e)),
            }

            match FirestoreDb::new(project_id).await {
                Ok(client) => {
                    *global_service = Some(DbService { client });
                    Ok(())
                }
                Err(e) => Err(format!("Error al inicializar cliente Firestore: {}", e)),
            }
        } else {
            Ok(())
        }
    }

    pub async fn get_instance() -> Option<Self> {
        let global_service = Self::global().await.lock().await;
        global_service.clone()
    }
}
