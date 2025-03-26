use std::{env::var, sync::OnceLock};
use dotenv::dotenv;
use tauri::async_runtime::Mutex;
use firestore::FirestoreDb;

// Importar submódulos
pub mod equipment;
pub mod maintenance;
pub mod statistics;
pub mod model;

pub struct DbService {
    client: FirestoreDb
}

impl DbService {
    pub async fn new() -> Self {
        dotenv().ok();
        // $env:GOOGLE_APPLICATION_CREDENTIALS = "C:\secure\mi-proyecto-firestore.json"
        DbService {
            client: FirestoreDb::new(String::from(var("PROJECT_ID").unwrap()))   
                .await
                .expect("Error al inicializar cliente Firestore")
        }
    }
}