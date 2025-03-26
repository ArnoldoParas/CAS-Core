# Cas Core

## Requisitos para ejecutar el programa

### 1. Instalar Rust

Para esto descarga el `.exe` disponible en la pagina web oficial de Rust
[aqui](https://www.rust-lang.org/tools/install).

> Rust necesita de las herramientas de construcción para C++ de Visual Studio, en la instalacion de rust te notifica y te da la opcion de descargarlas.

### 2. Instalar Bun

Bun es el entorno de ejecución de JavaScript elegido para este proyecto, para descargarlo puedes ir a su pagina web [aqui](https://bun.sh/).

Para windows hay que pegar en el cmd:
```powershell
powershell -c "irm bun.sh/install.ps1 | iex"
```
Para linux & macOS:
```powershell
curl -fsSL https://bun.sh/install | bash
```

## Instrucciones para Desarrollo con .env y Variables de Entorno

### 1. Configurar archivo `.env`

Crea un archivo `.env` en la raíz del proyecto con:
```env
PROJECT_ID="id del proyecto"
GOOGLE_APPLICATION_CREDENTIALS="C:\\Program Files\\CasCore\\config\\service.json"
```
> Usar `\\` en rutas Windows para escape correcto.

### 2. Configuración de Windows

Crear directorio (PowerShell como Admin):
```powershell
mkdir "C:\Program Files\CasCore\config"
```

Copiar credenciales:
```powershell
Copy-Item -Path ".\service.json" -Destination "C:\Program Files\CasCore\config\"
```
O bien pega las credenciales en la carpeta.


## Limpieza al desinstalar
1. Eliminar archivo .env
2. Eliminar carpeta de credenciales:
3. Eliminar variable de entorno del sistema (si se configuró permanente)