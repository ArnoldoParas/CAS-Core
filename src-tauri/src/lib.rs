// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod db_service;
pub mod typst_renderer;

use chrono::{DateTime, Datelike, Utc};
use rand::Rng;
use tauri::async_runtime::block_on;

use db_service::{model::Equipo, DbService};
use typst_renderer::generate_example_pdf;

#[tauri::command]
async fn get_all() -> String {
    let db_service = DbService::new()
        .await;
    
    let equipos = db_service.get_all_equipments().await.unwrap();
    let equipos = match serialize_equipos(equipos) {
        Ok(json_string) => json_string,
        Err(e) => e.to_string(),
    };
    equipos
}

fn serialize_equipos(equipos: Vec<Equipo>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&equipos)
}

#[tauri::command]
fn insert() -> String {
    "INSERT :o".to_string()
}

#[tauri::command]
fn delete_by_id() -> String {
    "DELETE >:(".to_string()
}

#[tauri::command]
async fn insert_e(num: u16) -> Result<(), String> {
    let db_service = DbService::new().await;
    
    let mut id = 0;
    for i in 0..num {        
        let test = asdf(&id).await;
            
        db_service.insert(test, "FAKE2".to_string())
            .await.expect("Error al insertar equipo");
        println!("nuevo equipo: {}", i+1);
        id += 1;
    }
    
    Ok(())
}

async fn asdf(id: &i32) -> Equipo {
    let mut rng = rand::rng();


    let equipos = vec!["Monitor".to_string(), "CPU".to_string(), "All In One".to_string(), "Impresora".to_string()];
    let marcas = vec!["Asus".to_string(), "Dell".to_string(), "Hp".to_string(), "BenQ".to_string()];
    let modelos = vec!["V350Z".to_string(), "W185q".to_string(), "Compacq 6700 G9".to_string(), "Optiplex 870".to_string()];
    let grupos = vec!["4208".to_string(), "Coordinacion de FOGU".to_string(), "Subdireccion Academica".to_string(), "4201".to_string()];
    let registros = vec![
        DateTime::parse_from_rfc3339("2025-01-25T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-02-25T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-03-27T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-04-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-05-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-06-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-07-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-08-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-09-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-10-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-11-01T21:37:44Z").unwrap().with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-12-01T21:37:44Z").unwrap().with_timezone(&Utc),
    ];
    Equipo {
        id_equipo: format!("FAKE2025-{:04}", id),
        equipo: equipos[rng.random_range(0..4)].clone(),
        marca: marcas[rng.random_range(0..4)].clone(),
        modelo: modelos[rng.random_range(0..4)].clone(),
        referencia_externa: format!("{}", rng.random_range(1000000..=9999999)),
        ultimo_registro: vec![registros[rng.random_range(0..12)].clone()],
        grupo: grupos[rng.random_range(0..4)].clone(),
    }
}

#[tauri::command]
fn pdf() {
    generate_example_pdf();
}

#[tauri::command]
async fn get_maintenance_stats() -> Result<String, String> {
    let db_service = DbService::new().await;

    let result = match db_service.get_statistics(Utc::now().year()).await {
        Ok(stats) => {
            match serde_json::to_string(&stats) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("Error serializando datos: {}", e))
            }
        },
        Err(e) => Err(format!("Error obteniendo estadísticas: {}", e))
    };
    println!("{:?}",result);
    result
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all, 
            insert, 
            delete_by_id, 
            pdf,
            insert_e,
            get_maintenance_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// let _ = db_service.delete_by_id("FIM-0002".to_string())
//     .await;
// let _ = db_service.update_by_id(insert_doc)
//     .await;

// if let Ok(srch) = db_service.get_by_id("FIM2025-0001".to_string()).await {
//     println!("{:?}", srch);
// };

// if let Ok(equipos) = db_service.get_by_field("marca", "ASUS").await {
//     for equipo in equipos {
//         println!("{:?}", equipo);
//     }
// }