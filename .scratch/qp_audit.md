# QP Audit: omniget
## Analysis of Robustness, Performance, and Quality

| Category | Critical Finding | Impact | Solution Reference |
| :--- | :--- | :--- | :--- |
| **Robustez** | **Polución de Variables de Entorno**: `setup_environment` elimina `PYTHONHOME` y `PYTHONPATH` globalmente para el proceso. | **Crítico**: Si `omniget` lanza subprocesos que no son sus propios scripts, estos fallarán al no encontrar sus librerías. | Pasar las variables limpias solo al `Command` que ejecuta el script específico, no a todo el entorno del proceso. |
| **Funcionamiento** | **Zombis de Descarga**: El gestor de descargas en Rust no maneja `SIGINT` de forma que limpie archivos `.part` incompletos. | **Medio**: Acumulación de basura en la carpeta de datos tras interrupciones. | Implementar un Drop handler en Rust o un sistema de limpieza de temporales al inicio de la aplicación. |
| **Utilidad** | **Consumo de Memoria de WebView**: Al ser Tauri, el renderizado puede escalar rápido con muchos resultados. | **Alto**: Uso de RAM excesivo al listar miles de paquetes con iconos pesados. | Implementar Virtual Scrolling para el listado de paquetes y lazy loading de recursos visuales. |

## General Codebase Audit (Deep Pass)

| Category | Finding | Impact | Recommendation |
| :--- | :--- | :--- | :--- |
| **Robustez** | **Panics vía `.unwrap()`**: Uso extensivo de `.unwrap()` en `native_host.rs` y `recovery.rs`. | **Crítico**: El proceso de Rust colapsará ante cualquier error de IO o JSON malformado, en lugar de enviar un mensaje de error al frontend. | Reemplazar `.unwrap()` por manejo de errores basado en `Result` y propagación mediante `?`. |
| **Concurrencia** | **Poisoning de Mutex**: Llamadas a `.lock().unwrap()` en estados compartidos globales. | **Alto**: Si un hilo entra en pánico mientras sostiene el lock, el resto de la aplicación fallará al intentar acceder al estado (Mutex poisoned). | Usar `match` o `if let` para manejar el caso de Mutex envenenado y permitir la recuperación del estado. |
| **Calidad** | **Manejo de Archivos Temporales**: Creación de archivos sin asegurar su borrado en caso de fallo crítico. | **Medio**: Llenado gradual de disco con metadatos y archivos de host nativo. | Usar crates como `tempfile` que aseguran el borrado automático al salir del scope. |
