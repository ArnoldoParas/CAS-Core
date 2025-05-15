import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Tarjeta from "./Tarjeta";

// Material UI
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Alert from '@mui/material/Alert';
import CircularProgress from '@mui/material/CircularProgress';

// Íconos
import FormatListNumberedIcon from '@mui/icons-material/FormatListNumbered';
import AddToQueueIcon from '@mui/icons-material/AddToQueue';
import DeleteSweepIcon from '@mui/icons-material/DeleteSweep';

export default function General() {
  const [equipos, setEquipos] = useState([]);
  const [loading, setLoading] = useState(false);
  const [Msg, setMsg] = useState("");
  const [num, setNum] = useState("");

  async function get_all() {
    try {
      setLoading(true);
      const response = await invoke("get_all");
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      setEquipos(data);
    } catch (err) {
      console.error(err);
      setMsg("Error al obtener los equipos.");
    } finally {
      setLoading(false);
    }
  }

  async function insert() {
    try {
      setMsg(await invoke("insert"));
    } catch (err) {
      console.error(err);
      setMsg("Error al insertar el equipo.");
    }
  }

  async function delete_by_id() {
    try {
      setMsg(await invoke("delete_by_id"));
    } catch (err) {
      console.error(err);
      setMsg("Error al eliminar el equipo.");
    }
  }

  async function insert_e() {
    const numValue = parseInt(num, 10);

    if (isNaN(numValue) || numValue <= 0) {
      setMsg("Por favor ingrese un número válido mayor que cero.");
      return;
    }

    setLoading(true);
    try {
      await invoke("insert_e", { num: numValue });
      setMsg(`Se insertaron ${numValue} equipos con éxito.`);
      setNum("");
    } catch (err) {
      console.error(err);
      setMsg("Error al insertar múltiples equipos.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" gutterBottom align="center">
        Gestión de Equipos
      </Typography>
      <Typography variant="h6" gutterBottom align="center">
        Rust side
      </Typography>

      <Box display="flex" justifyContent="center" marginBottom={2}>
        <Stack direction="row" spacing={2}>
          <Button
            variant="contained"
            color="success"
            startIcon={<FormatListNumberedIcon />}
            onClick={get_all}
          >
            Listar Equipos
          </Button>
          <Button
            variant="contained"
            color="success"
            startIcon={<AddToQueueIcon />}
            onClick={insert}
          >
            Insertar Equipo
          </Button>
          <Button
            variant="outlined"
            color="error"
            startIcon={<DeleteSweepIcon />}
            onClick={delete_by_id}
          >
            Eliminar Equipo
          </Button>
        </Stack>
      </Box>

      {loading && (
        <Box display="flex" justifyContent="center" marginTop={2}>
          <Stack sx={{ color: 'grey.400' }} spacing={2} direction="row">
            <CircularProgress color="success" />
          </Stack>
        </Box>
      )}

      {!loading && equipos.length === 0 && (
        <Box display="flex" justifyContent="center" marginTop={2}>
          <Stack sx={{ width: '50%' }} spacing={2}>
            <Alert variant="filled" severity="info">
              No hay equipos para mostrar. Haga clic en "Listar Equipos" para cargar la información.
            </Alert>
          </Stack>
        </Box>
      )}

      {Msg && (
        <Box display="flex" justifyContent="center" marginTop={2}>
          <Stack sx={{ width: '25%' }} spacing={2}>
            <Alert
              variant="filled"
              severity={
                Msg.toLowerCase().includes("error") ? "error" :
                Msg.toLowerCase().includes("éxito") || Msg.toLowerCase().includes("insertaron") ? "success" :
                "info"
              }
            >
              {Msg}
            </Alert>
          </Stack>
        </Box>
      )}

      <form
        onSubmit={(e) => {
          e.preventDefault();
          insert_e();
        }}
        style={{ marginBottom: '1rem' }}
      >
        <Box display="flex" justifyContent="center">
          <Stack direction="row" spacing={2}>
            <TextField
              type="number"
              label="Número de equipos"
              value={num}
              onChange={(e) => setNum(e.target.value)}
              inputProps={{ min: 1 }}
              required
              sx={{
                backgroundColor: '#f0f0f0',
                borderRadius: 1,
                input: { color: 'black' },
                label: { color: 'gray' },
                '& .MuiOutlinedInput-root': {
                  '& fieldset': { borderColor: 'gray' },
                  '&:hover fieldset': { borderColor: 'black' },
                  '&.Mui-focused fieldset': { borderColor: 'black' },
                }
              }}
            />
            <Button type="submit" variant="contained" color="primary">
              Insertar
            </Button>
          </Stack>
        </Box>
      </form>


      <Box
        className="equipos-grid"
        sx={{
          display: 'flex',
          justifyContent: 'center',
          flexWrap: 'wrap',
          gap: 2,
          marginTop: 2,
        }}
      >
        {equipos.map((equipo) => (
          <Tarjeta key={equipo.id_equipo} equipo={equipo} />
        ))}
      </Box>
    </Box>
  );
}
