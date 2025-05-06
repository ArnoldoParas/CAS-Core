import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";

export default function PDF() {
  const [style, setStyle] = useState("Type1");
  const [dependence, setDependence] = useState("FIME");
  const [pages, setPages] = useState(1);
  const [dependencies, setDependencies] = useState([]); // Estado para almacenar las dependencias

  useEffect(() => {
    // Obtener las dependencias al cargar el componente
    async function fetchDependencies() {
      const t = await invoke("get_all_d");
      console.log("Datos de la base de datos:", t);
      setDependencies(t); // Guardar las dependencias en el estado
    }

    fetchDependencies();
  }, []);

  async function generate_pdf() {
    const path = await save({
      title: "Guardar archivo PDF",
      defaultPath: "documento.pdf",
      filters: [
        {
          name: "PDF",
          extensions: ["pdf"],
        },
      ],
    });

    if (path) {
      const amount = pages * 40;

      const data = {
        style,
        dependence,
        amount,
      };

      console.log("Ruta seleccionada:", path);
      console.log("Datos del formulario:", data);

      // await invoke("pdf", { ruta: path, ...data });
    }
  }

  return (
    <div>
      <h1>Generar PDF de etiquetas</h1>

      <div>
        <label>
          Estilo:
          <select value={style} onChange={(e) => setStyle(e.target.value)}>
            <option value="Type1">Type1</option>
          </select>
        </label>
      </div>

      <div>
        <label>
          Dependencia:
          <select value={dependence} onChange={(e) => setDependence(e.target.value)}>
            {dependencies.map((dep, index) => (
              <option key={index} value={dep}>
                {dep}
              </option>
            ))}
          </select>
        </label>
      </div>

      <div>
        <label>
          Páginas:
          <input
            type="number"
            min={1}
            value={pages}
            onChange={(e) => setPages(parseInt(e.target.value) || 1)}
          />
        </label>
      </div>

      <button onClick={generate_pdf}>Seleccionar ruta y generar PDF</button>
    </div>
  );
}
