use std::{env::var, sync::OnceLock};
use dotenv::dotenv;
use tauri::async_runtime::Mutex;
use firestore::FirestoreDb;

// Importar submódulos
pub mod equipment;
pub mod maintenance;
pub mod statistics;
pub mod model;

#[derive(Clone)]
pub struct DbService {
    client: FirestoreDb
}

impl DbService {
    // Obtener la instancia global de DbService
    pub async fn global() -> &'static Mutex<Option<Self>> {
        static INSTANCE: OnceLock<Mutex<Option<DbService>>> = OnceLock::new();
        
        INSTANCE.get_or_init(|| {
            Mutex::new(None)
        })
    }

    // Inicializar la conexión global si aún no está inicializada
    pub async fn initialize() -> Result<(), String> {
        let mut global_service = Self::global().await.lock().await;
        
        if global_service.is_none() {
            dotenv().ok();
            
            match FirestoreDb::new(String::from(var("PROJECT_ID").unwrap())).await {
                Ok(client) => {
                    *global_service = Some(DbService { client });
                    Ok(())
                },
                Err(_) => Err("Error al inicializar cliente Firestore".to_string())
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