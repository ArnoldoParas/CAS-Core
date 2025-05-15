import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { downloadDir } from '@tauri-apps/api/path';

// MUI 
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';

// Iconos
import SaveAsIcon from '@mui/icons-material/SaveAs';
import SaveIcon from '@mui/icons-material/Save';

export default function PDF() {
  const [style, setStyle] = useState('');
  const [dependence, setDependence] = useState('');
  const [pages, setPages] = useState(1);
  const [dependencies, setDependencies] = useState([]);
  const [generatedPdfData, setGeneratedPdfData] = useState(null);
  const [pdfReady, setPdfReady] = useState(false);

  // Dropdown 
  const [styleOpen, setStyleOpen] = useState(false);
  const [depOpen, setDepOpen] = useState(false);

  useEffect(() => {
    async function fetchDependencies() {
      const v_dependencies = await invoke("get_all_d");
      setDependencies(v_dependencies);
    }

    fetchDependencies();
  }, []);

  async function generate_pdf() {
    if (!style || !dependence) {
      alert("Selecciona estilo y dependencia antes de continuar.");
      return;
    }

    const amount = pages * 40;
    const data = {
      Label: {
        style,
        dependence,
        amount,
        start: 1
      }
    };

    try {
      const success = await invoke("pdf", { data });
      setPdfReady(success);
    } catch (error) {
      console.error("Error generating PDF:", error);
    }
  }

  async function savePdfFile() {
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
        setGeneratedPdfData(null);
      } catch (error) {
        console.error("Error saving PDF:", error);
      }
    }
  }

  const inputStyle = {
    backgroundColor: '#f5f5f5',
  };

  return (
    <Box
      sx={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        p: 2,
      }}
    >
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          gap: 2,
          maxWidth: 300,
          width: '100%',
        }}
      >
        <h1 style={{ textAlign: 'center' }}>Generar PDF de etiquetas</h1>

        {/* Estilo */}
        <FormControl size="small" fullWidth>
          <InputLabel id="style-select-label">Estilo</InputLabel>
          <Select
            labelId="style-select-label"
            id="style-select"
            open={styleOpen}
            onOpen={() => setStyleOpen(true)}
            onClose={() => setStyleOpen(false)}
            value={style}
            label="Estilo"
            onChange={(e) => setStyle(e.target.value)}
            sx={inputStyle}
          >
            <MenuItem value="">
              <em>Seleccionar</em>
            </MenuItem>
            <MenuItem value="Type1">Type1</MenuItem>
          </Select>
        </FormControl>

        {/* Dependencia */}
        <FormControl size="small" fullWidth>
          <InputLabel id="dependence-select-label">Dependencia</InputLabel>
          <Select
            labelId="dependence-select-label"
            id="dependence-select"
            open={depOpen}
            onOpen={() => setDepOpen(true)}
            onClose={() => setDepOpen(false)}
            value={dependence}
            label="Dependencia"
            onChange={(e) => setDependence(e.target.value)}
            sx={inputStyle}
          >
            <MenuItem value="">
              <em>Seleccionar</em>
            </MenuItem>
            {dependencies.map((dep, index) => (
              <MenuItem key={index} value={dep}>
                {dep}
              </MenuItem>
            ))}
          </Select>
        </FormControl>


        <TextField
          label="Páginas"
          type="number"
          size="small"
          inputProps={{ min: 1 }}
          value={pages}
          onChange={(e) => setPages(parseInt(e.target.value) || 1)}
          sx={inputStyle}
          fullWidth
        />


        <Button
          variant="contained"
          color="primary"
          onClick={generate_pdf}
          startIcon={<SaveAsIcon />}
        >
          Generar PDF
        </Button>


        {pdfReady && (
          <Button
            variant="outlined"
            color="secondary"
            onClick={savePdfFile}
            startIcon={<SaveIcon />}
          >
            Guardar PDF
          </Button>
        )}
      </Box>
    </Box>
  );
}
