// import { save } from "@tauri-apps/plugin-dialog";
// import { invoke } from "@tauri-apps/api/core";

// export default function PDF() {
//   async function generate_pdf() {
//     const path = await save({
//       title: "Guardar archivo PDF",
//       defaultPath: "documento.pdf",
//       filters: [
//         {
//           name: "PDF",
//           extensions: ["pdf"],
//         },
//       ],
//     });

//     if (path) {
//       // Llama al comando `pdf` y pásale la ruta seleccionada
//       // await invoke("pdf", { ruta: path });
//       console.log("Ruta seleccionada:", path);
//     }
//   }

//   return (
//     <div>
//       <h1>PDF</h1>
//       <button onClick={generate_pdf}>PDF! :D</button>
//     </div>
//   );
// }

import { useState } from "react";
import { save } from "@tauri-apps/plugin-dialog";

export default function PDF() {
  const [style, setStyle] = useState("Type1");
  const [dependence, setDependence] = useState("FIME");
  const [pages, setPages] = useState(1);

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

      // Aquí puedes usar invoke en el futuro:
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
            <option value="Type2">Type2</option>
          </select>
        </label>
      </div>

      <div>
        <label>
          Dependencia:
          <select value={dependence} onChange={(e) => setDependence(e.target.value)}>
            <option value="FIME">FIME</option>
            <option value="FCFM">FCFM</option>
            <option value="FACPYA">FACPYA</option>
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
