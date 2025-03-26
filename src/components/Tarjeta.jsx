import React from "react";

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
    <div className="tarjeta">
      <div className="tarjeta-header">
        <h2 className="tarjeta-id">{equipo.id_equipo}</h2>
        <span className="tarjeta-grupo">{equipo.grupo}</span>
      </div>
      
      <div className="tarjeta-contenido">
        <div className="tarjeta-fila">
          <span className="tarjeta-etiqueta">Tipo:</span>
          <span className="tarjeta-valor">{equipo.equipo}</span>
        </div>
        
        <div className="tarjeta-fila">
          <span className="tarjeta-etiqueta">Marca:</span>
          <span className="tarjeta-valor">{equipo.marca}</span>
        </div>
        
        <div className="tarjeta-fila">
          <span className="tarjeta-etiqueta">Modelo:</span>
          <span className="tarjeta-valor">{equipo.modelo}</span>
        </div>
        
        <div className="tarjeta-fila">
          <span className="tarjeta-etiqueta">Ref. Externa:</span>
          <span className="tarjeta-valor">{equipo.referencia_externa}</span>
        </div>
        
        <div className="tarjeta-fila">
          <span className="tarjeta-etiqueta">Último registro:</span>
          <span className="tarjeta-valor">
            {equipo.ultimo_registro && equipo.ultimo_registro.length > 0 
              ? formatearFecha(equipo.ultimo_registro[0]) 
              : "No disponible"}
          </span>
        </div>
      </div>
    </div>
  );
}

export default Tarjeta;