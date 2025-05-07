use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipo {
    pub id_equipo: String,
    pub equipo: String,
    pub marca: String,
    pub modelo: String,
    pub referencia_externa: String,
    pub ultimo_registro: Vec<DateTime<Utc>>,
    pub grupo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependencia {
    pub count: u64,
    #[serde(default)]
    pub groups: Vec<DependenciaGrupo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependenciaGrupo {
    pub encargado: String,
    pub grupo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mantenimiento {
    pub enero: u16,
    pub febrero: u16,
    pub marzo: u16,
    pub abril: u16,
    pub mayo: u16,
    pub junio: u16,
    pub julio: u16,
    pub agosto: u16,
    pub septiembre: u16,
    pub octubre: u16,
    pub noviembre: u16,
    pub diciembre: u16,
}
