import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { downloadDir } from '@tauri-apps/api/path';

export default function PDF() {
  const [style, setStyle] = useState("Type1");
  const [dependence, setDependence] = useState("FIME");
  const [pages, setPages] = useState(1);
  const [dependencies, setDependencies] = useState([]);
  const [generatedPdfData, setGeneratedPdfData] = useState(null);
  const [pdfReady, setPdfReady] = useState(false);

  useEffect(() => {
    // Obtener las dependencias al cargar el componente
    async function fetchDependencies() {
      const v_dependencies = await invoke("get_all_d");
      console.log("Datos de la base de datos:", v_dependencies);
      setDependencies(v_dependencies); // Guardar las dependencias en el estado
    }

    fetchDependencies();
  }, []);

  async function generate_pdf() {
    console.log("Generating PDF..."); // Debug log
    const amount = pages * 40;
    const data = {
      Label: { // Wrap in Label variant to match Rust enum
        style,
        dependence,
        amount,
        start: 1 // Required by Rust struct
      }
    };

    try {
      console.log("Calling Tauri command with data:", data); // Debug log
      const success = await invoke("pdf", { data });
      console.log("PDF generation result:", success); // Debug log
      setPdfReady(success);
    } catch (error) {
      console.error("Error generating PDF:", error);
    }
  }

  async function savePdfFile() {
    const now = new Date();
    const dateStr = now.toISOString()
        .replace(/[:.]/g, '-')
        .replace('T', '_')
        .slice(0, 19); // Format: YYYY-MM-DD_HH-mm-ss

    const path = await save({
      title: "Etiquetas-CAS",
      defaultPath: await downloadDir(),
      filters: [
        {
          name: "PDF",
          extensions: ["pdf"],
        },
      ],
    });

    if (path) {
      try {
        await invoke("save_pdf", { path, pdfData: generatedPdfData });
        setGeneratedPdfData(null); // Reset after saving
        console.log("PDF path:", path);
      } catch (error) {
        console.error("Error saving PDF:", error);
      }
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

      <button onClick={generate_pdf}>Generar PDF</button>
      {pdfReady && (
        <button onClick={savePdfFile}>Guardar PDF</button>
      )}
    </div>
  );
}
