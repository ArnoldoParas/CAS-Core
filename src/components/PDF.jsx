import { invoke } from "@tauri-apps/api/core";

export default function PDF() {
  async function generate_pdf() {
    await invoke ("pdf")
  }

  return (
    <div>
      <h1>PDF</h1>
      <button onClick={generate_pdf}>PDF! :D</button>
    </div>
  );
}