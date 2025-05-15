import React from "react";
import { Card } from 'primereact/card';
import { Tag } from 'primereact/tag';
import { Divider } from 'primereact/divider';

function Tarjeta({ equipo }) {
  // Función para formatear la fecha
  const formatearFecha = (fechaStr) => {
    if (!fechaStr) return "No disponible";
    try {
      const fecha = new Date(fechaStr);
      return fecha.toLocaleString('es-ES');
    } catch (e) {
      return fechaStr;
    }
  };

  return (
    <Card 
      title={`ID: ${equipo.id_equipo}`} 
      subTitle={<Tag value={equipo.grupo || "Sin grupo"} severity="info" />}
      className="mb-3"
    >
      <div className="p-fluid grid">
        <div className="field col-12 md:col-6">
          <label className="font-bold">Tipo</label>
          <div>{equipo.equipo}</div>
        </div>

        <div className="field col-12 md:col-6">
          <label className="font-bold">Marca</label>
          <div>{equipo.marca}</div>
        </div>

        <div className="field col-12 md:col-6">
          <label className="font-bold">Modelo</label>
          <div>{equipo.modelo}</div>
        </div>

        <div className="field col-12 md:col-6">
          <label className="font-bold">Ref. Externa</label>
          <div>{equipo.referencia_externa}</div>
        </div>

        <Divider className="col-12" />

        <div className="field col-12">
          <label className="font-bold">Último registro</label>
          <div>
            {equipo.ultimo_registro && equipo.ultimo_registro.length > 0
              ? formatearFecha(equipo.ultimo_registro[0])
              : "No disponible"}
          </div>
        </div>
      </div>
    </Card>
  );
}

export default Tarjeta;
