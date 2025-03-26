import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Tarjeta from "./Tarjeta";

export default function General() {
  const [equipos, setEquipos] = useState([]);
  const [loading, setLoading] = useState(false);
  const [Msg, setMsg] = useState("");
  const [num, setNum] = useState("");

  async function get_all() {
    try {
      setLoading(true);
      const response = await invoke("get_all");
      // Convertir la respuesta a objeto si viene como string
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      setEquipos(data);
      // setMsg(response);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  }

  async function insert() {
    setMsg(await invoke("insert"));

  }

  async function delete_by_id() {
    setMsg(await invoke("delete_by_id"));
  }

  async function insert_e() {
    // Convert the string input to a number
    const numValue = parseInt(num, 10);

    if (isNaN(numValue) || numValue <= 0) {
      setMsg("Por favor ingrese un número válido mayor que cero.");
      return;
    }
    
    setLoading(true);
    await invoke("insert_e", { num: numValue });
    setMsg(`Se insertaron ${numValue} equipos con éxito.`);
    setNum(""); // Clear the input after successful insertion
  }

  return (
    <div>
      <h1>Gestión de Equipos</h1>
      <h2>Rust side</h2>
      <div className="button_row">
        <button onClick={get_all}>
          Listar Equipos
        </button>
        <button onClick={insert}>
          Insertar Equipo
        </button>
        <button onClick={delete_by_id}>
          Eliminar Equipo
        </button>
      </div>

      {loading && <p>Cargando...</p>}

      {!loading && equipos.length === 0 && (
        <p>
          No hay equipos para mostrar. Haga clic en "Listar Equipos" para cargar la información.
        </p>
      )}

      <p>{Msg}</p>

      <form
        onSubmit={(e) => {
          e.preventDefault();
          insert_e();
        }}>
        <input
          type="number"
          value={num}
          onChange={(e) => setNum(e.target.value)}
          placeholder="Número de equipos"
          min="1"
          required
        />
        <button type="submit">Insertar</button>
      </form>

      <div className="equipos-grid">
        {equipos.map((equipo) => (
          <Tarjeta key={equipo.id_equipo} equipo={equipo} />
        ))}
      </div>
    </div>
  );
}