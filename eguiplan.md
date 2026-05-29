

# PLAN — IMPLEMENTACIÓN DE egui EN MANGOFETCH

## Contexto arquitectónico crítico

Antes de cualquier cosa: el ROADMAP dice **Tauri v2**. Tú me preguntas **egui**. Son filosofías opuestas — Tauri es WebView + IPC, egui es immediate mode GPU puro. Necesito que confirmes que egui reemplaza la decisión de Tauri, o si van a coexistir (`mangofetch-gui-tauri` y `mangofetch-gui-egui`). El plan asume **egui reemplaza Tauri** porque tienen stack incompatible y mantener dos GUIs es deuda enorme.

---

## FASE 0 — Workspace restructuring

### 0.1 — Nuevo crate: `mangofetch-gui`
Crear `K:\source\repos\mangofetch-cli\mangofetch-gui\` como nuevo miembro del workspace.

En el `Cargo.toml` raíz, agregar `"mangofetch-gui"` al array `members`. **No** agregarlo a `default-members` — el default sigue siendo el CLI.

### 0.2 — `mangofetch-gui/Cargo.toml`
Dependencias mínimas del crate nuevo:

```toml
[package]
name = "mangofetch-gui"
version = "0.5.5"  # sync con workspace
edition = "2021"

[[bin]]
name = "mangofetch-gui"
path = "src/main.rs"

[dependencies]
mangofetch-core = { path = "../mangofetch-core", version = "0.5.5", features = ["platforms"] }
eframe = { version = "0.31", features = ["default_fonts", "persistence"] }
egui = "0.31"
egui_extras = { version = "0.31", features = ["all_loaders"] }
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
tracing = "0.1"
dirs = "6"
chrono = { version = "0.4", features = ["serde"] }
```

**Por qué `eframe` y no `egui` directo:** `eframe` maneja el event loop nativo (winit), contexto GL/wgpu, y persistencia de ventana. Sin él habría que reinventar todo eso.

### 0.3 — Feature flag en `mangofetch-core`
En `mangofetch-core/Cargo.toml`, agregar un feature `gui` vacío por ahora — reservado para exponer APIs adicionales si el GUI necesita más que el core actual. No se activa aún.

---

## FASE 1 — Bridge: egui ↔ tokio

Este es el problema arquitectónico más importante. `mangofetch-core` es 100% async tokio. egui corre en el UI thread y **no puede bloquear**. La solución es un canal unidireccional de mensajes.

### 1.1 — Crear `mangofetch-gui/src/bridge.rs`

Define dos enums:

**`GuiCommand`** — mensajes del GUI hacia el core:
```rust
pub enum GuiCommand {
    StartDownload { url: String, output_dir: String, quality: Option<String>, audio_only: bool },
    PauseDownload { id: u64 },
    ResumeDownload { id: u64 },
    RemoveDownload { id: u64 },
    RefreshQueue,
    CheckDependencies,
    FetchMediaInfo { url: String },
    Shutdown,
}
```

**`CoreEvent`** — eventos del core hacia el GUI:
```rust
pub enum CoreEvent {
    QueueUpdated(Vec<QueueItemInfo>),
    DownloadProgress { id: u64, progress: f32, speed: f64, eta: Option<u64> },
    DownloadComplete { id: u64, title: String },
    DownloadError { id: u64, error: String },
    MediaInfoFetched(Result<MediaInfo, String>),
    DependencyStatus { ytdlp: bool, ffmpeg: bool },
    LogLine(String),
}
```

### 1.2 — Crear `mangofetch-gui/src/runtime.rs`

Lanza el tokio runtime en un thread separado y expone los canales:

```rust
pub struct AppRuntime {
    pub cmd_tx: mpsc::UnboundedSender<GuiCommand>,
    pub event_rx: mpsc::UnboundedReceiver<CoreEvent>,
}
```

El thread de runtime:
1. Crea el `DownloadQueue` y `PlatformRegistry`
2. Escucha `GuiCommand` en loop
3. Por cada comando, ejecuta la operación core async y envía `CoreEvent` de vuelta
4. Polling del queue state cada 250ms → emite `QueueUpdated`

**Clave:** usar `std::thread::spawn` para el thread del runtime, **no** `tokio::spawn` — así egui conserva el control total del UI thread.

---

## FASE 2 — App state (`mangofetch-gui/src/app.rs`)

### 2.1 — Struct `MangoFetchApp`

Implementa `eframe::App`. Campos principales:

```rust
pub struct MangoFetchApp {
    runtime: AppRuntime,
    
    // Queue state (siempre el último QueueUpdated recibido)
    queue_items: Vec<QueueItemInfo>,
    
    // UI state
    active_tab: Tab,          // Home | Queue | History | Settings | About
    url_input: String,
    quality_input: String,
    show_add_modal: bool,
    preview_info: Option<MediaInfo>,
    is_fetching_preview: bool,
    
    // Theming — MonolithUI portado a egui
    visuals: MonolithVisuals,  // ver Fase 3
    
    // Stats
    total_speed: f64,
    active_count: usize,
    
    // Log ring buffer
    log_lines: VecDeque<String>,  // cap 500
    
    // Settings
    settings: AppSettings,
}
```

### 2.2 — `eframe::App::update()` — el loop principal

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // 1. Drenar todos los CoreEvent pendientes
    self.drain_events();
    
    // 2. Solicitar repaint continuo si hay downloads activos
    if self.active_count > 0 {
        ctx.request_repaint_after(Duration::from_millis(250));
    }
    
    // 3. Render
    self.render_top_bar(ctx);
    self.render_side_nav(ctx);   // o top nav según setting
    self.render_main_panel(ctx);
    self.render_status_bar(ctx);
    
    // 4. Modals por encima de todo
    if self.show_add_modal { self.render_add_modal(ctx); }
}
```

---

## FASE 3 — MonolithUI → egui Visuals

egui tiene su propio sistema de estilos (`egui::Visuals`, `egui::Style`). Hay que mapearlo a los tokens de MonolithUI.

### 3.1 — Crear `mangofetch-gui/src/theme.rs`

Define `MonolithVisuals` que aplica el sistema de colores al contexto egui:

```rust
pub fn apply_monolith_dark(ctx: &egui::Context, brand: BrandPreset) {
    let mut visuals = egui::Visuals::dark();
    
    // Surface ramp → egui panels
    visuals.window_fill        = hex("#0c0e12");  // surface-1
    visuals.panel_fill         = hex("#060608");  // surface-0
    visuals.faint_bg_color     = hex("#131720");  // surface-2
    visuals.extreme_bg_color   = hex("#060608");  // surface-0
    
    // El color primario del brand activo
    let primary = brand.primary();  // eg. #22d3ee para plasma-core
    
    // Widgets
    visuals.widgets.inactive.bg_fill   = hex("#1c2130");  // surface-3
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, hex("#ffffff14")); // border-default
    visuals.widgets.hovered.bg_fill    = hex("#252a3a");  // surface-4
    visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, hex("#ffffff38")); // border-hover
    visuals.widgets.active.bg_fill     = with_alpha(primary, 0.20);  // accent-primary-bg
    visuals.widgets.active.bg_stroke   = Stroke::new(1.5, with_alpha(primary, 0.40));
    
    // Selection
    visuals.selection.bg_fill  = with_alpha(primary, 0.15);
    visuals.selection.stroke   = Stroke::new(1.0, primary);
    
    // Hyperlinks → brand primary
    visuals.hyperlink_color = primary;
    
    // Rounding — --ui-r-md = 6px
    visuals.window_rounding   = Rounding::same(8.0);  // ui-r-lg
    visuals.menu_rounding     = Rounding::same(6.0);  // ui-r-md
    visuals.widgets.inactive.rounding = Rounding::same(4.0);  // ui-r-sm
    
    ctx.set_visuals(visuals);
    
    // Fonts: DM Sans + DM Mono (cargar desde bytes embebidos con include_bytes!)
    // Ver 3.2
}
```

**Limitación importante que Gemini debe saber:** egui no soporta `box-shadow`. El skeuomorfismo de sombras interiores del sistema MonolithUI **no existe en egui**. La adaptación se hace via:
- Colores de fondo más oscuros en los "wells" (inputs)
- `Stroke` más grueso en elementos activos
- Diferencia de luminosidad entre `inactive`/`hovered`/`active`

### 3.2 — Fuentes embebidas
```rust
// En main.rs, antes de crear el App:
let mut fonts = egui::FontDefinitions::default();
fonts.font_data.insert("DM Sans".to_owned(),
    egui::FontData::from_static(include_bytes!("../assets/DM_Sans.ttf")));
fonts.font_data.insert("DM Mono".to_owned(),
    egui::FontData::from_static(include_bytes!("../assets/DM_Mono.ttf")));
fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
    .insert(0, "DM Sans".to_owned());
fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
    .insert(0, "DM Mono".to_owned());
ctx.set_fonts(fonts);
```

Agregar los archivos `.ttf` a `mangofetch-gui/assets/`. Las fuentes ya existen en el proyecto web — copiarlas.

---

## FASE 4 — Layout y componentes UI

### 4.1 — Top command bar (`render_top_bar`)

```
[●  MANGOFETCH] [v0.5.5] [BRANCH: master*]        [⊕ Add URL] [theme] [⚙]
```

Implementar como `egui::TopBottomPanel::top("command_bar")`.
- Altura fija: 32px
- Background: `surface-2` (#131720)
- Separador inferior: `border-default`
- Los "segments" son `egui::Button` con `frame(false)` + hover highlight manual

### 4.2 — Side navigation (`render_side_nav`)

`egui::SidePanel::left("nav")` con ancho fijo 200px.
Tabs: Home | Queue | History | Settings | About | Logs

Cada tab = `egui::SelectableLabel` con custom styling:
- Inactive: texto `text-tertiary`, sin borde
- Active: `border-left: 2px solid brand-primary` — **en egui esto se logra con `painter.rect_filled` en una franja de 2px a la izquierda del item** antes de renderizar el label

### 4.3 — Main panel (`render_main_panel`)

`egui::CentralPanel` para el contenido. Cada tab renderiza su propia función:

- **Tab::Queue** — `egui::Grid` o `TableBuilder` (de egui_extras) con columnas: #, Title, Platform, Status, Progress, Speed. La progress bar se hace con `egui::ProgressBar`.
- **Tab::Home** — Cards de acción con `egui::Frame` + `egui::Button` grandes
- **Tab::Settings** — Lista scrollable con `egui::ScrollArea`, cada setting como fila con label izquierda + control derecha (toggle/combobox/slider)
- **Tab::Logs** — `egui::ScrollArea` con `egui::Label` para cada línea, auto-scroll al bottom

### 4.4 — Status bar (`render_status_bar`)

`egui::TopBottomPanel::bottom("status_bar")` con altura 24px.
Mismos módulos que el TUI: mode | tab | cpu | ram | speed | queue | time.
Cada módulo = texto mono separado por `│`.

### 4.5 — Modal Add URL

`egui::Window::new("Add URL")` con `collapsible(false)`, `resizable(false)`, centrado.
Dos campos: URL input + Quality input (opcional).
Botones: Cancel | Fetch Info → Preview | Download.

---

## FASE 5 — Entry point (`mangofetch-gui/src/main.rs`)

```rust
fn main() -> eframe::Result<()> {
    let runtime = AppRuntime::start();  // lanza tokio thread
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MangoFetch")
            .with_inner_size([1200.0, 720.0])
            .with_min_inner_size([800.0, 500.0])
            .with_icon(/* cargar ícono */),
        ..Default::default()
    };
    
    eframe::run_native(
        "MangoFetch",
        options,
        Box::new(|cc| {
            theme::apply_monolith_dark(&cc.egui_ctx, BrandPreset::PlasmCore);
            theme::load_fonts(&cc.egui_ctx);
            Ok(Box::new(MangoFetchApp::new(runtime, cc)))
        }),
    )
}
```

---

## FASE 6 — Persistencia de ventana

egui/eframe soporta persistencia nativa via `eframe::Storage`. Activar con `eframe::NativeOptions { persist_window: true, .. }`.

Implementar `eframe::App::save()` para serializar `AppSettings` a disco cuando el usuario cierra la ventana — mismo mecanismo que el CLI ya usa (`AppSettings::save_to_disk()`).

---

## ORDEN DE EJECUCIÓN PARA GEMINI

1. **Fase 0** — Crear `mangofetch-gui/` con `Cargo.toml`, estructura de directorios y `assets/`
2. **Fase 1** — `bridge.rs` + `runtime.rs` — sin UI, solo el canal. Compilar y testear con un `println!` que imprime eventos.
3. **Fase 3** — `theme.rs` + fuentes. Testear con una ventana vacía que muestre los colores correctos.
4. **Fase 5** — `main.rs` mínimo con ventana vacía que corra.
5. **Fase 4.1 + 4.4** — Command bar y status bar (estructura visual primero).
6. **Fase 4.2** — Side nav con tabs navegables.
7. **Fase 4.3 Queue** — La tabla de downloads, que es el core de la app.
8. **Fase 2** — `app.rs` completo integrando todo.
9. **Fase 4.3 resto** — Home, Settings, Logs, About.
10. **Fase 6** — Persistencia.

---

## LO QUE NO SE TOCA

- `mangofetch-core` — cero cambios de lógica
- `mangofetch-cli` — cero cambios
- `mangofetch-plugin-sdk` — cero cambios
- El TUI ratatui existente — coexiste, no se depreca

---

## ADVERTENCIA PARA GEMINI

egui es **immediate mode** — no hay "componentes" con estado interno. Todo el estado vive en `MangoFetchApp`. Si intentas crear widgets con estado propio, vas mal. Cada frame redibuja todo desde cero. Esto es intencional y es la fortaleza de egui para UIs de datos en tiempo real como esta.
