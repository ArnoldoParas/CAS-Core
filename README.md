# Cas Core

# Instrucciones para Desarrollo con .env y Variables de Entorno


### 1. Configurar archivo `.env`
Crea un archivo `.env` en la raíz del proyecto con:

```env
PROJECT_ID="tu_id_de_proyecto_real"
GOOGLE_APPLICATION_CREDENTIALS="C:\\Program Files\\CasCore\\config\\service.json"
```
> Usar \\ en rutas Windows para escape correcto.

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

Establecer permisos:

```powershell
icacls "C:\Program Files\CasCore\config\service.json" /grant:r "$env:USERNAME:(R)"
```

## Limpieza al desinstalar
1. Eliminar archivo .env
2. Eliminar carpeta de credenciales:
3. Eliminar variable de entorno del sistema (si se configuró permanente)