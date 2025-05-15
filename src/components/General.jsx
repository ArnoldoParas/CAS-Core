import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";
import { Button } from "primereact/button";
import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";
import { Dialog } from "primereact/dialog"; // Importa el componente Dialog

import "primereact/resources/themes/lara-light-indigo/theme.css"; // Tema de PrimeReact
import "primereact/resources/primereact.min.css"; // Estilos de PrimeReact
import "primeicons/primeicons.css"; // Iconos de PrimeReact

// Material UI
import Stack from "@mui/material/Stack";
import ButtonMUI from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import Alert from "@mui/material/Alert";
import CircularProgress from "@mui/material/CircularProgress";

// Íconos
import FormatListNumberedIcon from "@mui/icons-material/FormatListNumbered";
import AddToQueueIcon from "@mui/icons-material/AddToQueue";
import EditIcon from "@mui/icons-material/Edit"; // Importa el ícono de edición
import editIcon from "../assets/icons/edit.svg";
import trashIcon from "../assets/icons/trash.svg";

export default function General() {
  const [equipos, setEquipos] = useState([]);
  const [loading, setLoading] = useState(false);
  const [Msg, setMsg] = useState("");
  const [dependencies, setDependencies] = useState([]);
  const [dependence, setDependence] = useState("");
  const [showDialog, setShowDialog] = useState(false); // Estado para controlar el diálogo
  const [formData, setFormData] = useState({
    id_equipo: "",
    equipo: "",
    marca: "",
    modelo: "",
    referencia_externa: "",
    ultimo_registro: [],
  });
  const [selectedEquipo, setSelectedEquipo] = useState(null); // Estado para el equipo seleccionado
  const [dependenciaData, setDependenciaData] = useState([]);
  const [activeTable, setActiveTable] = useState(""); // Estado para controlar la tabla activa ("equipos" o "dependencia")
  const [selectedGrupo, setSelectedGrupo] = useState(null); // Estado para el grupo seleccionado
  const [grupoFormData, setGrupoFormData] = useState({
    encargado: "",
    grupo: "",
  }); // Estado para los datos del formulario de dependencia
  const [showGrupoDialog, setShowGrupoDialog] = useState(false); // Controla la visibilidad del diálogo

  useEffect(() => {
    async function fetchDependencies() {
      try {
        const v_dependencies = await invoke("get_all_d");
        setDependencies(v_dependencies);
      } catch (err) {
        console.error("Error al obtener dependencias:", err);
      }
    }

    fetchDependencies();
  }, []);

  async function get_all() {
    try {
      setLoading(true);
      const response = await invoke("get_all", { dependency: dependence });
      const data = typeof response === "string" ? JSON.parse(response) : response;
      setEquipos(data);
    } catch (err) {
      console.error(err);
      setMsg("Error al obtener los equipos.");
    } finally {
      setLoading(false);
    }
  }

  async function delete_by_id(id) {
    try {
      setMsg(await invoke("delete_by_id", { id: id , dependency: dependence}));
      await get_all(); 
    } catch (err) {
      console.error(err);
      setMsg("Error al eliminar el equipo.");
    }
  }

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  const handleSubmit = async () => {
    console.log("Datos enviados al backend:", formData);
    try {
      setLoading(true);
      await invoke("insert", { equipo: JSON.stringify(formData), dependency: dependence });
      setMsg("Equipo insertado con éxito.");
      setShowDialog(false);
      await get_all();
    } catch (err) {
      console.error(err);
      setMsg("Error al insertar el equipo.");
    } finally {
      setLoading(false);
    }
  };

  const handleUpdate = async () => {
    console.log("Datos enviados al backend para actualizar:", formData);
    try {
      setLoading(true);
      await invoke("update_equipo", { equipo: JSON.stringify(formData), dependency: dependence });
      setMsg("Equipo actualizado con éxito.");
      setShowDialog(false);
      await get_all();
    } catch (err) {
      console.error(err);
      setMsg("Error al actualizar el equipo.");
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateDependency = async () => {
    console.log("Datos enviados al backend para actualizar:", grupoFormData);
    try {
      setLoading(true);
      await invoke("update_dependency_groups", { document: JSON.stringify(grupoFormData), dependency: dependence });
      setMsg("Dependencia actualizada con éxito.");
      setShowGrupoDialog(false); // Cierra el diálogo
      await handleEditDependency(); // Recarga los datos de la tabla de dependencias
    } catch (err) {
      console.error("Error al actualizar la dependencia:", err);
      setMsg("Error al actualizar la dependencia.");
    } finally {
      setLoading(false);
    }
  };

  const handleEditDependency = async () => {
    try {
      const response = await invoke("update_dependency", { dependency: dependence });
      const grupos = typeof response === "string" ? JSON.parse(response) : response;

      if (Array.isArray(grupos)) {
        setDependenciaData(grupos);
      } else {
        console.error("Los datos recibidos no son un array:", grupos);
        setDependenciaData([]);
      }

      setActiveTable("dependencia");
    } catch (err) {
      console.error("Error al obtener los datos de la dependencia:", err);
      setMsg("Error al cargar los datos de la dependencia.");
    }
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" gutterBottom align="center">
        Directorio
      </Typography>

      <Box display="flex" justifyContent="center" marginBottom={2}>
        <FormControl size="small" sx={{ minWidth: 200 }}>
          <InputLabel id="dependence-select-label">Dependencia</InputLabel>
          <Select
            labelId="dependence-select-label"
            id="dependence-select"
            value={dependence}
            label="Dependencia"
            onChange={(e) => setDependence(e.target.value)}
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
      </Box>

      <Box display="flex" justifyContent="center" marginBottom={2}>
        <Stack direction="row" spacing={2}>
          <ButtonMUI
            variant="contained"
            color="success"
            startIcon={<FormatListNumberedIcon />}
            onClick={() => {
              setActiveTable("equipos");
              get_all();
            }}
          >
            Listar Equipos
          </ButtonMUI>
          {dependence && (
            <ButtonMUI
              variant="contained"
              color="success"
              startIcon={<AddToQueueIcon />}
              onClick={() => setShowDialog(true)}
            >
              Insertar Equipo
            </ButtonMUI>
          )}
          {dependence && (
            <ButtonMUI
              variant="contained"
              color="primary"
              startIcon={<EditIcon />}
              onClick={handleEditDependency}
            >
              Editar Dependencia
            </ButtonMUI>
          )}
        </Stack>
      </Box>

      {/* Diálogo con el formulario */}
      <Dialog
        header={selectedEquipo ? "Editar Equipo" : "Insertar Equipo"}
        visible={showDialog}
        style={{ width: "30vw" }}
        onHide={() => {
          setShowDialog(false);
          setSelectedEquipo(null);
        }}
      >
        <Box component="form" noValidate autoComplete="off">
          <TextField
            label="ID Equipo"
            name="id_equipo"
            value={formData.id_equipo}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
            disabled={!!selectedEquipo} // Deshabilita el campo si se está editando
          />
          <TextField
            label="Equipo"
            name="equipo"
            value={formData.equipo}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
          />
          <TextField
            label="Marca"
            name="marca"
            value={formData.marca}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
          />
          <TextField
            label="Modelo"
            name="modelo"
            value={formData.modelo}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
          />
          <TextField
            label="Referencia Externa"
            name="referencia_externa"
            value={formData.referencia_externa}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
          />
          <TextField
            label="Grupo"
            name="grupo"
            value={formData.grupo}
            onChange={handleInputChange}
            fullWidth
            margin="normal"
          />
          <Box display="flex" justifyContent="flex-end" marginTop={2}>
            <ButtonMUI
              variant="contained"
              color="primary"
              onClick={selectedEquipo ? handleUpdate : handleSubmit} // Llama a la función adecuada
              disabled={
                !formData.id_equipo ||
                !formData.equipo ||
                !formData.marca ||
                !formData.modelo ||
                !formData.referencia_externa ||
                !formData.grupo
              }
            >
              {selectedEquipo ? "Actualizar" : "Guardar"}
            </ButtonMUI>
          </Box>
        </Box>
      </Dialog>

      {/* Diálogo para editar grupo */}
      <Dialog
        header="Editar Dependencia"
        visible={showGrupoDialog}
        style={{ width: "30vw" }}
        onHide={() => {
          setShowGrupoDialog(false);
          setSelectedGrupo(null); // Limpia el grupo seleccionado al cerrar
        }}
      >
        <Box component="form" noValidate autoComplete="off">
          <TextField
            label="Encargado"
            name="encargado"
            value={grupoFormData.encargado}
            onChange={(e) =>
              setGrupoFormData({ ...grupoFormData, encargado: e.target.value })
            }
            fullWidth
            margin="normal"
          />
          <TextField
            label="Grupo"
            name="grupo"
            value={grupoFormData.grupo}
            onChange={(e) =>
              setGrupoFormData({ ...grupoFormData, grupo: e.target.value })
            }
            fullWidth
            margin="normal"
          />
          <Box display="flex" justifyContent="flex-end" marginTop={2}>
            <ButtonMUI
              variant="contained"
              color="primary"
              onClick={handleUpdateDependency}
              disabled={!grupoFormData.encargado || !grupoFormData.grupo} // Deshabilita si algún campo está vacío
            >
              Actualizar
            </ButtonMUI>
          </Box>
        </Box>
      </Dialog>

      {loading && (
        <Box display="flex" justifyContent="center" marginTop={2}>
          <Stack sx={{ color: "grey.400" }} spacing={2} direction="row">
            <CircularProgress color="success" />
          </Stack>
        </Box>
      )}

      {Msg && (
        <Box display="flex" justifyContent="center" marginTop={2}>
          <Stack sx={{ width: "25%" }} spacing={2}>
            <Alert
              variant="filled"
              severity={
                Msg.toLowerCase().includes("error")
                  ? "error"
                  : Msg.toLowerCase().includes("éxito") ||
                    Msg.toLowerCase().includes("insertaron")
                  ? "success"
                  : "info"
              }
            >
              {Msg}
            </Alert>
          </Stack>
        </Box>
      )}

      {activeTable === "equipos" && (
        <Box
          sx={{
            marginTop: 2,
          }}
        >
          {equipos.length > 0 ? (
            <DataTable
              value={equipos}
              paginator
              rows={10}
              tableStyle={{ minWidth: "50rem" }}
            >
              <Column field="id_equipo" header="ID"></Column>
              <Column field="equipo" header="Tipo"></Column>
              <Column field="marca" header="Marca"></Column>
              <Column field="modelo" header="Modelo"></Column>
              <Column field="referencia_externa" header="Ref. Externa"></Column>
              <Column
                body={(rowData) => (
                  <Button
                    className="p-button-rounded p-button-secondary"
                    onClick={() => {
                      setSelectedEquipo(rowData); // Establece el equipo seleccionado
                      setFormData(rowData); // Carga los datos en el formulario
                      setShowDialog(true); // Abre el diálogo
                    }}
                  >
                    <img
                      src={editIcon}
                      alt="Edit Icon"
                      style={{ width: "25px", height: "25px" }}
                    />
                  </Button>
                )}
              ></Column>
              <Column
                body={(rowData) => (
                  <Button
                    className="p-button-rounded p-button-secondary"
                    onClick={() => delete_by_id(rowData.id_equipo)}
                  >
                    <img
                      src={trashIcon}
                      alt="Trash Icon"
                      style={{ width: "25px", height: "25px" }}
                    />
                  </Button>
                )}
              ></Column>
            </DataTable>
          ) : (
            <Box display="flex" justifyContent="center" marginTop={2}>
              <Stack sx={{ width: "50%" }} spacing={2}>
                <Alert variant="filled" severity="info">
                  No hay equipos para mostrar. Haga clic en "Listar Equipos" para
                  cargar la información.
                </Alert>
              </Stack>
            </Box>
          )}
        </Box>
      )}

      {activeTable === "dependencia" && (
        <Box sx={{ marginTop: 4 }}>
          <Typography variant="h5" gutterBottom>
            Editar Dependencia: {dependence}
          </Typography>
          <DataTable
            value={Array.isArray(dependenciaData) ? dependenciaData : []}
            paginator
            rows={5}
            tableStyle={{ minWidth: "50rem" }}
          >
            <Column field="grupo" header="Grupo"></Column>
            <Column field="encargado" header="Encargado"></Column>
            <Column
              header="Acciones"
              body={(rowData) => (
                <Button
                  className="p-button-rounded p-button-secondary"
                  onClick={() => {
                    setSelectedGrupo(rowData); // Establece el grupo seleccionado
                    setGrupoFormData(rowData); // Carga los datos en el formulario
                    setShowGrupoDialog(true); // Abre el diálogo
                  }}
                >
                  <img
                    src={editIcon}
                    alt="Edit Icon"
                    style={{ width: "25px", height: "25px" }}
                  />
                </Button>
              )}
            ></Column>
          </DataTable>
        </Box>
      )}
    </Box>
  );
}
