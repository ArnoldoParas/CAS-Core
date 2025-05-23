// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod db_service;
pub mod labels;
pub mod typst_renderer;

use chrono::{DateTime, Datelike, Utc};
use rand::Rng;

use db_service::{model::{DependenciaGrupo, Equipo}, DbService};
use typst_renderer::Data;
#[allow(unused_imports)]
use tauri::async_runtime::block_on;

#[tauri::command]
async fn get_all(dependency: String) -> String {
    
    if let Err(e) = db_service::DbService::initialize().await {
        eprintln!("Error inicializando base de datos: {:?}", e);
        return String::new();
    }

    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let equipos = db_service.get_all_equipments(dependency).await.unwrap();
    let equipos = match serialize_equipos(equipos) {
        Ok(json_string) => json_string,
        Err(e) => e.to_string(),
    };
    equipos
}

#[tauri::command]
async fn get_all_d() -> Vec<String> {
    if let Err(e) = db_service::DbService::initialize().await {
        eprintln!("Error inicializando base de datos: {:?}", e);
        return vec![];
    }

    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    // Obtén los nombres de las dependencias
    let equipos = match db_service.get_all_dependency_names().await {
        Ok(equipos) => equipos,
        Err(e) => {
            eprintln!("Error obteniendo nombres de dependencias: {:?}", e);
            return vec![];
        }
    };

    let nombres: Vec<String> = equipos
        .into_iter()
        .map(|ruta| ruta.split('/').last().unwrap_or("").to_string())
        .collect();

    nombres
}

#[tauri::command]
async fn pdf(mut data: Data) -> Result<bool, String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    if let Data::Label(ref mut label_data) = data {
        label_data.start = db_service.get_dependency_count(&label_data.dependence).await.unwrap_or(0) as u16;
        match typst_renderer::generate_pdf(data) {
            Ok(success) => Ok(success),
            Err(e) => Err(e.to_string())
        }
    } else {
        Err("Expected Data::Label variant".to_string())
    }
}

#[tauri::command]
async fn save_pdf(path: String) -> Result<(), String> {
    println!("PDF path: {}", path);
    
    std::fs::copy("output/pdf/output_label.pdf", &path)
        .map(|_| ())
        .map_err(|e| format!("Error saving PDF: {}", e))
}

fn serialize_equipos(equipos: Vec<Equipo>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&equipos)
}

#[tauri::command]
async fn insert(equipo: String, dependency: String) -> Result<String, String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let mut equipo: Equipo = match serde_json::from_str(&equipo) {
        Ok(equipo) => equipo,
        Err(err) => {
            eprintln!("Error deserializando equipo: {}", err);
            return Err("Error al procesar los datos del equipo.".to_string());
        }
    };

    if equipo.ultimo_registro.is_empty() {
        equipo.ultimo_registro.push(Utc::now());
    }

    let _ = db_service.insert(equipo, dependency).await;
    Ok("Equipo insertado con éxito.".to_string())
}

#[tauri::command]
async fn delete_by_id(id: String, dependency: String) -> String {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    println!("Eliminando equipo: {}, dependencia: {}", id, dependency);
    let _ = db_service.delete_equipment_by_id(id, dependency).await;
    "Equipo eliminado".to_string()
}

#[tauri::command]
async fn update_equipo(equipo: String, dependency: String) -> Result<String, String> {
    println!("Actualizando equipo: {}, dependencia: {}", equipo, dependency);
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let equipo: Equipo = match serde_json::from_str(&equipo) {
        Ok(equipo) => equipo,
        Err(err) => {
            eprintln!("Error deserializando equipo: {}", err);
            return Err("Error al procesar los datos del equipo.".to_string());
        }
    };

    let _ = db_service.update_document_by_id(equipo, dependency).await;

    Ok("Equipo actualizado con éxito.".to_string())
}

#[tauri::command]
async fn update_dependency(dependency: String) -> Result<String, String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let dep = db_service.get_dependency_data_by_id(&dependency).await.unwrap().unwrap();

    match serde_json::to_string(&dep.grupos) {
        Ok(json) => {
            print!("Grupos: {}", json);
            Ok(json)
        },
        Err(e) => Err(format!("Error serializando grupos: {}", e)),
    }
}

#[tauri::command]
async fn update_dependency_groups(dependency: String, document: String) -> Result<String, String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let document: DependenciaGrupo = match serde_json::from_str(&document) {
        Ok(equipo) => equipo,
        Err(err) => {
            eprintln!("Error deserializando equipo: {}", err);
            return Err("Error al procesar los datos del equipo.".to_string());
        }
    };

    let _ = db_service.update_dependency_by_id(document, dependency).await.unwrap();
    Ok("Grupo actualizado con éxito.".to_string())
}

#[tauri::command]
async fn insert_e(num: u16) -> Result<(), String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let mut id = 0;
    for i in 0..num {
        let test = asdf(&id).await;

        db_service
            .insert(test, "FAKE2".to_string())
            .await
            .expect("Error al insertar equipo");
        println!("nuevo equipo: {}", i + 1);
        id += 1;
    }

    Ok(())
}

async fn asdf(id: &i32) -> Equipo {
    let mut rng = rand::rng();

    let equipos = vec![
        "Monitor".to_string(),
        "CPU".to_string(),
        "All In One".to_string(),
        "Impresora".to_string(),
    ];
    let marcas = vec![
        "Asus".to_string(),
        "Dell".to_string(),
        "Hp".to_string(),
        "BenQ".to_string(),
    ];
    let modelos = vec![
        "V350Z".to_string(),
        "W185q".to_string(),
        "Compacq 6700 G9".to_string(),
        "Optiplex 870".to_string(),
    ];
    let grupos = vec![
        "4208".to_string(),
        "Coordinacion de FOGU".to_string(),
        "Subdireccion Academica".to_string(),
        "4201".to_string(),
    ];
    let registros = vec![
        DateTime::parse_from_rfc3339("2025-01-25T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-02-25T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-03-27T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-04-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-05-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-06-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-07-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-08-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-09-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-10-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-11-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
        DateTime::parse_from_rfc3339("2025-12-01T21:37:44Z")
            .unwrap()
            .with_timezone(&Utc),
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
async fn get_maintenance_stats() -> Result<String, String> {
    let db_service = DbService::get_instance()
        .await
        .expect("No se pudo obtener la instancia de DbService");

    let result = match db_service.get_statistics(Utc::now().year()).await {
        Ok(stats) => match serde_json::to_string(&stats) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Error serializando datos: {}", e)),
        },
        Err(e) => Err(format!("Error obteniendo estadísticas: {}", e)),
    };
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // if let Err(e) = block_on(async { db_service::DbService::initialize().await }) {
    //     eprintln!("Error inicializando base de datos: {:?}", e);
    // }
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all,
            get_all_d,
            save_pdf,
            insert,
            update_equipo,
            update_dependency,
            update_dependency_groups,
            delete_by_id,
            pdf,
            insert_e,
            get_maintenance_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
