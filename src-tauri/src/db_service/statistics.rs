use super::model::Mantenimiento;
use crate::db_service::DbService;
use chrono::{DateTime, Datelike, Utc};
use firestore::errors::FirestoreError;

impl DbService {
    // Método interno para actualizar estadísticas

    #[allow(dependency_on_unit_never_type_fallback)]
    pub(crate) async fn update_maintenance_statistics(
        &self,
        fecha: DateTime<Utc>,
        _dependency: String,
    ) -> Result<(), FirestoreError> {
        let mes = match fecha.month() {
            1 => "enero",
            2 => "febrero",
            3 => "marzo",
            4 => "abril",
            5 => "mayo",
            6 => "junio",
            7 => "julio",
            8 => "agosto",
            9 => "septiembre",
            10 => "octubre",
            11 => "noviembre",
            12 => "diciembre",
            _ => "Enero", // Error no manejado correctamente
        };

        let documento_estadisticas = format!("mantenimiento_{}", fecha.year());

        // Intentar obtener el documento de estadísticas existente, si no existe lo crea
        let mut estadisticas = self.get_statistics(fecha.year()).await?;

        match mes {
            "enero" => estadisticas.enero += 1,
            "febrero" => estadisticas.febrero += 1,
            "marzo" => estadisticas.marzo += 1,
            "abril" => estadisticas.abril += 1,
            "mayo" => estadisticas.mayo += 1,
            "junio" => estadisticas.junio += 1,
            "julio" => estadisticas.julio += 1,
            "agosto" => estadisticas.agosto += 1,
            "septiembre" => estadisticas.septiembre += 1,
            "octubre" => estadisticas.octubre += 1,
            "noviembre" => estadisticas.noviembre += 1,
            "diciembre" => estadisticas.diciembre += 1,
            _ => {}
        }

        self.client
            .fluent()
            .update()
            .in_col("stats")
            .document_id(&documento_estadisticas)
            .object(&estadisticas)
            .execute()
            .await?;

        Ok(())
    }

    // Método público para obtener estadísticas
    pub async fn get_statistics(&self, year: i32) -> Result<Mantenimiento, FirestoreError> {
        let documento_estadisticas = dbg!(format!("mantenimiento_{}", year));

        let stat_result = self
            .client
            .fluent()
            .select()
            .by_id_in("stats")
            .obj::<Mantenimiento>()
            .one(&documento_estadisticas)
            .await;

        match stat_result {
            Ok(Some(result)) => Ok(result),
            _ => {
                let nueva_estadistica = Mantenimiento {
                    enero: 0,
                    febrero: 0,
                    marzo: 0,
                    abril: 0,
                    mayo: 0,
                    junio: 0,
                    julio: 0,
                    agosto: 0,
                    septiembre: 0,
                    octubre: 0,
                    noviembre: 0,
                    diciembre: 0,
                };

                self.client
                    .fluent()
                    .insert()
                    .into("stats")
                    .document_id(&documento_estadisticas)
                    .object(&nueva_estadistica)
                    .execute::<Mantenimiento>()
                    .await?;

                Ok(nueva_estadistica)
            }
        }
    }
}
