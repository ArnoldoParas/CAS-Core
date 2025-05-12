#[allow(unused_imports)]
use crate::db_service::{self, DbService};

#[tokio::test]
async fn get_dependency_count() {
  db_service::DbService::initialize()
    .await
    .expect("Error inicializando base de datos");

  let db_service = DbService::get_instance()
    .await
    .expect("No se pudo obtener la instancia de DbService");
  
  let count = db_service.get_dependency_count("FARQ").await.unwrap();
  println!("Dependency count: {}", count);
  assert!(count == 7);
}