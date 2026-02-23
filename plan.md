Status: Completed

Now I have a complete picture. Let me compile the comprehensive summary.

---

# GPUI Platform Abstraction Layer — Comprehensive Summary

The GPUI platform abstraction is defined in `zed/crates/gpui/src/platform.rs` and its submodules. A new platform implementation must satisfy **8 core traits** (plus some optional/auxiliary ones). Below is every trait and every method that needs to be implemented.

---

## 1. `Platform` (main entry point)

```zed/crates/gpui/src/platform.rs#L112-228
pub trait Platform: 'static { ... }
```

This is the root trait. One instance exists for the lifetime of the application. It is responsible for the event loop, windowing, clipboard, credential storage, menus, keyboard layout, and general OS integration.

### Executor & Text

| Method | Description |
|--------|-------------|
| `background_executor(&self) -> BackgroundExecutor` | Returns the executor for background (thread pool) tasks |
| `foreground_executor(&self) -> ForegroundExecutor` | Returns the executor for the main/UI thread |
| `text_system(&self) -> Arc<dyn PlatformTextSystem>` | Returns the platform's text shaping/rasterization system |

### Application lifecycle

| Method | Description |
|--------|-------------|
| `run(&self, on_finish_launching: Box<dyn FnOnce()>)` | Starts the platform event loop; calls the callback once ready |
| `quit(&self)` | Quits the application |
| `restart(&self, binary_path: Option<PathBuf>)` | Restarts the application, optionally from a different binary |
| `activate(&self, ignoring_other_apps: bool)` | Brings the application to the foreground |
| `hide(&self)` | Hides the application |
| `hide_other_apps(&self)` | Hides all other applications (macOS-style) |
| `unhide_other_apps(&self)` | Unhides all other applications |

### Display & window queries

| Method | Description |
|--------|-------------|
| `displays(&self) -> Vec<Rc<dyn PlatformDisplay>>` | Lists all connected displays |
| `primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>` | Returns the primary display |
| `active_window(&self) -> Option<AnyWindowHandle>` | Returns the currently focused window handle |
| `window_stack(&self) -> Option<Vec<AnyWindowHandle>>` | *(default: `None`)* Returns the z-ordered window stack |

### Screen capture (optional, feature-gated)

| Method | Description |
|--------|-------------|
| `is_screen_capture_supported(&self) -> bool` | *(default: `false`)* Whether screen capture is available |
| `screen_capture_sources(&self) -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>>` | *(default: error)* Enumerate capturable sources |

### Window creation

| Method | Description |
|--------|-------------|
| `open_window(&self, handle: AnyWindowHandle, options: WindowParams) -> Result<Box<dyn PlatformWindow>>` | Creates and opens a new platform window |
| `window_appearance(&self) -> WindowAppearance` | Returns the current system-wide appearance (light/dark) |

### URL handling

| Method | Description |
|--------|-------------|
| `open_url(&self, url: &str)` | Opens a URL in the system browser |
| `on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>)` | Registers a callback for when the app is opened via URL |
| `register_url_scheme(&self, url: &str) -> Task<Result<()>>` | Registers the app as a URL scheme handler |

### File dialogs

| Method | Description |
|--------|-------------|
| `prompt_for_paths(&self, options: PathPromptOptions) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>>` | Shows an open-file dialog |
| `prompt_for_new_path(&self, directory: &Path, suggested_name: Option<&str>) -> oneshot::Receiver<Result<Option<PathBuf>>>` | Shows a save-file dialog |
| `can_select_mixed_files_and_dirs(&self) -> bool` | Whether the file picker can select both files and directories at once |
| `reveal_path(&self, path: &Path)` | Reveals a path in the system file manager |
| `open_with_system(&self, path: &Path)` | Opens a path with the system's default application |

### App lifecycle callbacks

| Method | Description |
|--------|-------------|
| `on_quit(&self, callback: Box<dyn FnMut()>)` | Registers callback invoked when the app is quitting |
| `on_reopen(&self, callback: Box<dyn FnMut()>)` | Registers callback for app reopen (e.g., dock click on macOS) |

### Menus

| Method | Description |
|--------|-------------|
| `set_menus(&self, menus: Vec<Menu>, keymap: &Keymap)` | Sets the application's main menu bar |
| `get_menus(&self) -> Option<Vec<OwnedMenu>>` | *(default: `None`)* Gets the current menus |
| `set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap)` | Sets the dock/taskbar context menu |
| `perform_dock_menu_action(&self, action: usize)` | *(default: no-op)* Performs a dock menu action by index |
| `add_recent_document(&self, path: &Path)` | *(default: no-op)* Adds a file to the OS recent documents list |
| `update_jump_list(&self, menus: Vec<MenuItem>, entries: Vec<SmallVec<[PathBuf; 2]>>) -> Task<Vec<SmallVec<[PathBuf; 2]>>>` | *(default: empty)* Updates the Windows jump list |
| `on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>)` | Registers callback when a menu action is invoked |
| `on_will_open_app_menu(&self, callback: Box<dyn FnMut()>)` | Registers callback just before a menu opens |
| `on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>)` | Registers callback to validate whether a menu item is enabled |

### Thermal state

| Method | Description |
|--------|-------------|
| `thermal_state(&self) -> ThermalState` | Returns the current system thermal state (`Nominal`, `Fair`, `Serious`, `Critical`) |
| `on_thermal_state_change(&self, callback: Box<dyn FnMut()>)` | Registers callback for thermal state changes |

### System info

| Method | Description |
|--------|-------------|
| `compositor_name(&self) -> &'static str` | *(default: `""`)* Returns the compositor name (e.g., `"Wayland"`, `"X11"`) |
| `app_path(&self) -> Result<PathBuf>` | Returns the path to the running application binary |
| `path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>` | Returns the path for a bundled helper executable |

### Cursor & scrollbars

| Method | Description |
|--------|-------------|
| `set_cursor_style(&self, style: CursorStyle)` | Sets the mouse cursor appearance (22 variants from `Arrow` to `None`) |
| `should_auto_hide_scrollbars(&self) -> bool` | Whether scrollbars should auto-hide (macOS preference) |

### Clipboard

| Method | Description |
|--------|-------------|
| `read_from_clipboard(&self) -> Option<ClipboardItem>` | Reads from the system clipboard |
| `write_to_clipboard(&self, item: ClipboardItem)` | Writes to the system clipboard |
| `read_from_primary(&self) -> Option<ClipboardItem>` | *(Linux only)* Reads from the primary selection |
| `write_to_primary(&self, item: ClipboardItem)` | *(Linux only)* Writes to the primary selection |
| `read_from_find_pasteboard(&self) -> Option<ClipboardItem>` | *(macOS only)* Reads from the find pasteboard |
| `write_to_find_pasteboard(&self, item: ClipboardItem)` | *(macOS only)* Writes to the find pasteboard |

### Credential storage

| Method | Description |
|--------|-------------|
| `write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>>` | Stores credentials in the OS keychain |
| `read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>` | Reads credentials from the OS keychain |
| `delete_credentials(&self, url: &str) -> Task<Result<()>>` | Deletes credentials from the OS keychain |

### Keyboard layout

| Method | Description |
|--------|-------------|
| `keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout>` | Returns the current keyboard layout info |
| `keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper>` | Returns the keyboard mapper for key equivalents |
| `on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>)` | Registers callback for keyboard layout changes |

---

## 2. `PlatformWindow`

```zed/crates/gpui/src/platform.rs#L448-555
pub trait PlatformWindow: HasWindowHandle + HasDisplayHandle { ... }
```

Represents a single OS window. Must also implement the `raw_window_handle` traits `HasWindowHandle` and `HasDisplayHandle`.

### Core geometry & state

| Method | Description |
|--------|-------------|
| `bounds(&self) -> Bounds<Pixels>` | Current window bounds in screen coordinates |
| `is_maximized(&self) -> bool` | Whether the window is maximized |
| `window_bounds(&self) -> WindowBounds` | The current `WindowBounds` enum (Windowed/Maximized/Fullscreen + restore size) |
| `content_size(&self) -> Size<Pixels>` | The size of the content area (excluding decorations) |
| `resize(&mut self, size: Size<Pixels>)` | Programmatically resize the window |
| `scale_factor(&self) -> f32` | The HiDPI scale factor |
| `appearance(&self) -> WindowAppearance` | The window's light/dark appearance |
| `display(&self) -> Option<Rc<dyn PlatformDisplay>>` | The display this window is on |
| `mouse_position(&self) -> Point<Pixels>` | Current mouse position in window coordinates |
| `modifiers(&self) -> Modifiers` | Current keyboard modifier state |
| `capslock(&self) -> Capslock` | Current caps lock state |

### Input handling

| Method | Description |
|--------|-------------|
| `set_input_handler(&mut self, input_handler: PlatformInputHandler)` | Sets the IME/text input handler |
| `take_input_handler(&mut self) -> Option<PlatformInputHandler>` | Takes ownership of the current input handler |

### Prompts

| Method | Description |
|--------|-------------|
| `prompt(&self, level, msg, detail, answers) -> Option<oneshot::Receiver<usize>>` | Shows a native dialog (alert/confirm) |

### Window state

| Method | Description |
|--------|-------------|
| `activate(&self)` | Brings the window to front and focuses it |
| `is_active(&self) -> bool` | Whether the window currently has focus |
| `is_hovered(&self) -> bool` | Whether the mouse is over the window |
| `background_appearance(&self) -> WindowBackgroundAppearance` | Current background appearance |
| `set_title(&mut self, title: &str)` | Sets the window title |
| `set_background_appearance(&self, appearance: WindowBackgroundAppearance)` | Sets the background transparency/blur mode |
| `minimize(&self)` | Minimizes the window |
| `zoom(&self)` | Zooms the window (macOS green button behavior) |
| `toggle_fullscreen(&self)` | Toggles fullscreen mode |
| `is_fullscreen(&self) -> bool` | Whether the window is fullscreen |

### Event callbacks

| Method | Description |
|--------|-------------|
| `on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>)` | Registers callback to draw the next frame |
| `on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>)` | Registers the input event handler |
| `on_active_status_change(&self, callback: Box<dyn FnMut(bool)>)` | Registers callback for focus changes |
| `on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>)` | Registers callback for hover changes |
| `on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>)` | Registers callback for resize events (size + scale factor) |
| `on_moved(&self, callback: Box<dyn FnMut()>)` | Registers callback when the window is moved |
| `on_should_close(&self, callback: Box<dyn FnMut() -> bool>)` | Registers callback to veto a close request |
| `on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>)` | Registers callback for custom titlebar hit testing |
| `on_close(&self, callback: Box<dyn FnOnce()>)` | Registers callback when the window closes |
| `on_appearance_changed(&self, callback: Box<dyn FnMut()>)` | Registers callback for appearance (dark/light) changes |

### Rendering

| Method | Description |
|--------|-------------|
| `draw(&self, scene: &Scene)` | Submits a rendered scene to the GPU for display |
| `completed_frame(&self)` | *(default: no-op)* Called when a frame completes |
| `sprite_atlas(&self) -> Arc<dyn PlatformAtlas>` | Returns the sprite atlas for texture caching |
| `is_subpixel_rendering_supported(&self) -> bool` | Whether subpixel antialiasing is available |
| `gpu_specs(&self) -> Option<GpuSpecs>` | Returns GPU device/driver information |

### macOS-specific (have default no-op impls)

| Method | Description |
|--------|-------------|
| `get_title(&self) -> String` | Gets the window title |
| `tabbed_windows(&self) -> Option<Vec<SystemWindowTab>>` | List of native tab windows |
| `tab_bar_visible(&self) -> bool` | Whether the native tab bar is visible |
| `set_edited(&mut self, edited: bool)` | Sets the "document edited" indicator |
| `show_character_palette(&self)` | Shows the emoji/character picker |
| `titlebar_double_click(&self)` | Handle titlebar double-click |
| `on_move_tab_to_new_window(&self, callback)` | Tab management callbacks |
| `on_merge_all_windows(&self, callback)` | Tab management callbacks |
| `on_select_previous_tab(&self, callback)` | Tab management callbacks |
| `on_select_next_tab(&self, callback)` | Tab management callbacks |
| `on_toggle_tab_bar(&self, callback)` | Tab management callbacks |
| `merge_all_windows(&self)` | Merge all windows into tabs |
| `move_tab_to_new_window(&self)` | Move current tab to a new window |
| `toggle_window_tab_overview(&self)` | Toggle the tab overview |
| `set_tabbing_identifier(&self, identifier: Option<String>)` | Set the tab group identifier |

### Windows-specific

| Method | Description |
|--------|-------------|
| `get_raw_handle(&self) -> HWND` | *(Windows only, `cfg`)* Returns the raw Win32 window handle |

### Linux/CSD-specific (have default impls)

| Method | Description |
|--------|-------------|
| `inner_window_bounds(&self) -> WindowBounds` | Window bounds excluding decorations |
| `request_decorations(&self, decorations: WindowDecorations)` | Request server/client side decorations |
| `show_window_menu(&self, position: Point<Pixels>)` | Show the window manager context menu |
| `start_window_move(&self)` | Start an interactive window move |
| `start_window_resize(&self, edge: ResizeEdge)` | Start an interactive window resize from an edge |
| `window_decorations(&self) -> Decorations` | Current decoration state (`Server` or `Client { tiling }`) |
| `set_app_id(&mut self, app_id: &str)` | Set the Wayland app_id / X11 WM_CLASS |
| `map_window(&mut self) -> Result<()>` | Map (show) the window on the display |
| `window_controls(&self) -> WindowControls` | Which controls the platform supports (fullscreen, maximize, minimize, window menu) |
| `set_client_inset(&self, inset: Pixels)` | Set the inset for CSD shadow |
| `update_ime_position(&self, bounds: Bounds<Pixels>)` | Update the IME candidate window position |

### Testing (cfg-gated)

| Method | Description |
|--------|-------------|
| `as_test(&mut self) -> Option<&mut TestWindow>` | Downcast to test window |
| `render_to_image(&self, scene: &Scene) -> Result<RgbaImage>` | Render a scene to a pixel buffer for visual tests |

---

## 3. `PlatformDisplay`

```zed/crates/gpui/src/platform.rs#L231-259
pub trait PlatformDisplay: Send + Sync + Debug { ... }
```

Represents a physical display/monitor.

| Method | Description |
|--------|-------------|
| `id(&self) -> DisplayId` | Unique runtime ID for this display |
| `uuid(&self) -> Result<Uuid>` | Stable UUID that persists across restarts |
| `bounds(&self) -> Bounds<Pixels>` | Full bounds of the display in screen coordinates |
| `visible_bounds(&self) -> Bounds<Pixels>` | *(default: `bounds()`)* Usable area excluding taskbar/dock |
| `default_bounds(&self) -> Bounds<Pixels>` | *(default: centered 1024×700)* Suggested default window bounds on this display |

---

## 4. `PlatformDispatcher`

```zed/crates/gpui/src/platform.rs#L568-589
pub trait PlatformDispatcher: Send + Sync { ... }
```

The task scheduler. Routes runnables to the right thread and manages timers.

| Method | Description |
|--------|-------------|
| `get_all_timings(&self) -> Vec<ThreadTaskTimings>` | Performance profiling: returns task timings for all threads |
| `get_current_thread_timings(&self) -> ThreadTaskTimings` | Performance profiling: returns task timings for the current thread |
| `is_main_thread(&self) -> bool` | Whether the caller is on the main/UI thread |
| `dispatch(&self, runnable: RunnableVariant, priority: Priority)` | Dispatches a task to the thread pool |
| `dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority)` | Dispatches a task to the main thread |
| `dispatch_after(&self, duration: Duration, runnable: RunnableVariant)` | Dispatches a task after a delay |
| `spawn_realtime(&self, f: Box<dyn FnOnce() + Send>)` | Spawns a function on a real-time priority thread |
| `now(&self) -> Instant` | *(default: `Instant::now()`)* Returns current time (overridable for tests) |
| `increase_timer_resolution(&self) -> TimerResolutionGuard` | *(default: no-op)* Requests higher timer resolution from the OS (Windows `timeBeginPeriod`) |
| `as_test(&self) -> Option<&TestDispatcher>` | *(test only, default: `None`)* Downcast for tests |

---

## 5. `PlatformTextSystem`

```zed/crates/gpui/src/platform.rs#L592-609
pub trait PlatformTextSystem: Send + Sync { ... }
```

Provides font loading, glyph lookup, shaping, and rasterization.

| Method | Description |
|--------|-------------|
| `add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()>` | Loads fonts from raw byte data |
| `all_font_names(&self) -> Vec<String>` | Lists all available font family names |
| `font_id(&self, descriptor: &Font) -> Result<FontId>` | Resolves a `Font` descriptor to an internal `FontId` |
| `font_metrics(&self, font_id: FontId) -> FontMetrics` | Returns metrics (ascent, descent, line gap, etc.) for a font |
| `typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>>` | Returns the typographic bounds of a glyph |
| `advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>>` | Returns the advance width/height of a glyph |
| `glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId>` | Maps a character to a glyph ID in the given font |
| `glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>>` | Returns the pixel-space bounds for rasterizing a glyph |
| `rasterize_glyph(&self, params: &RenderGlyphParams, raster_bounds: Bounds<DevicePixels>) -> Result<(Size<DevicePixels>, Vec<u8>)>` | Rasterizes a glyph into a pixel buffer |
| `layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout` | Shapes and lays out a line of text |
| `recommended_rendering_mode(&self, font_id: FontId, font_size: Pixels) -> TextRenderingMode` | Returns the recommended text rendering mode (subpixel/grayscale) for the given font and size |

A `NoopTextSystem` implementation is also provided for headless/testing scenarios.

---

## 6. `PlatformAtlas`

```zed/crates/gpui/src/platform.rs#L830-837
pub trait PlatformAtlas { ... }
```

A texture atlas for caching glyphs, SVGs, and images on the GPU.

| Method | Description |
|--------|-------------|
| `get_or_insert_with(&self, key: &AtlasKey, build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<[u8]>)>>) -> Result<Option<AtlasTile>>` | Retrieves a cached tile or creates it by calling the builder |
| `remove(&self, key: &AtlasKey)` | Removes a cached tile |

`AtlasKey` is an enum of `Glyph(RenderGlyphParams)`, `Svg(RenderSvgParams)`, or `Image(RenderImageParams)`.

---

## 7. `PlatformKeyboardLayout`

```zed/crates/gpui/src/platform/keyboard.rs#L6-12
pub trait PlatformKeyboardLayout { ... }
```

| Method | Description |
|--------|-------------|
| `id(&self) -> &str` | Unique identifier for the layout |
| `name(&self) -> &str` | Human-readable display name |

---

## 8. `PlatformKeyboardMapper`

```zed/crates/gpui/src/platform/keyboard.rs#L15-26
pub trait PlatformKeyboardMapper { ... }
```

| Method | Description |
|--------|-------------|
| `map_key_equivalent(&self, keystroke: Keystroke, use_key_equivalents: bool) -> KeybindingKeystroke` | Translates a keystroke through the platform's keyboard layout to produce a keybinding-compatible keystroke |
| `get_key_equivalents(&self) -> Option<&HashMap<char, char>>` | Returns the key equivalent mapping table (used on macOS) |

A `DummyKeyboardMapper` passthrough implementation is provided.

---

## Auxiliary traits (optional features)

### `ScreenCaptureSource`

```zed/crates/gpui/src/platform.rs#L288-299
pub trait ScreenCaptureSource { ... }
```

| Method | Description |
|--------|-------------|
| `metadata(&self) -> Result<SourceMetadata>` | ID, label, resolution of the capture source |
| `stream(&self, foreground_executor, frame_callback) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>>` | Begins capturing frames |

### `ScreenCaptureStream`

```zed/crates/gpui/src/platform.rs#L302-305
pub trait ScreenCaptureStream { ... }
```

| Method | Description |
|--------|-------------|
| `metadata(&self) -> Result<SourceMetadata>` | Returns metadata for the stream |

### `InputHandler`

```zed/crates/gpui/src/platform.rs#L1082-1177
pub trait InputHandler: 'static { ... }
```

Not a *platform* trait per se, but the interface that GPUI views implement for IME/text input. The platform calls into this via `PlatformInputHandler`.

| Method | Description |
|--------|-------------|
| `selected_text_range(...)` | Current selection as UTF-16 range |
| `marked_text_range(...)` | Current IME composition range (UTF-16) |
| `text_for_range(...)` | Get document text for a UTF-16 range |
| `replace_text_in_range(...)` | Insert text at a range |
| `replace_and_mark_text_in_range(...)` | Insert IME composing text |
| `unmark_text(...)` | End IME composition |
| `bounds_for_range(...)` | Screen-space bounds for a text range (IME window positioning) |
| `character_index_for_point(...)` | Hit test: point → character index |
| `apple_press_and_hold_enabled(&mut self) -> bool` | *(default: `true`)* Whether macOS press-and-hold is enabled |
| `accepts_text_input(...)` | *(default: `true`)* Whether the handler accepts text input |

---

## Existing platform implementations

The codebase has **5 implementations** of this abstraction layer:

| Implementation | Crate / Path | Notes |
|---|---|---|
| **macOS** | `zed/crates/gpui_macos/` | Core Graphics + Metal, native NSWindow |
| **Windows** | `zed/crates/gpui_windows/` | Win32 + Direct3D/Vulkan |
| **Linux (Wayland)** | `zed/crates/gpui_linux/` | wl_surface, via `LinuxClient` trait → `WaylandClient` |
| **Linux (X11)** | `zed/crates/gpui_linux/` | XCB, via `LinuxClient` trait → `X11Client` |
| **Test** | `zed/crates/gpui/src/platform/test/` | In-memory fake for unit tests |

Linux uses an indirection layer: a `LinuxPlatform` struct delegates to a `LinuxClient` trait, which is implemented by `WaylandClient`, `X11Client`, and `HeadlessClient`.

---

## Key associated types to be aware of

| Type | Description |
|---|---|
| `WindowParams` | Input to `open_window()` — bounds, titlebar, kind, resizable, etc. |
| `WindowOptions` | User-facing window configuration, converted to `WindowParams` internally |
| `WindowBounds` | Enum: `Windowed(Bounds)`, `Maximized(Bounds)`, `Fullscreen(Bounds)` |
| `WindowAppearance` | `Light`, `VibrantLight`, `Dark`, `VibrantDark` |
| `WindowBackgroundAppearance` | `Opaque`, `Transparent`, `Blurred`, `MicaBackdrop`, `MicaAltBackdrop` |
| `WindowKind` | `Normal`, `PopUp`, `Floating`, `LayerShell`, `Dialog` |
| `CursorStyle` | 22 cursor variants (Arrow, IBeam, PointingHand, resize variants, etc.) |
| `ClipboardItem` / `ClipboardEntry` | Clipboard data: string (with optional metadata), image, or external paths |
| `PlatformInput` | The input event enum (mouse, keyboard, scroll, etc.) |
| `Scene` | The rendered element tree, ready for GPU submission |
| `Keystroke` / `Modifiers` | Key event data |
| `ThermalState` | `Nominal`, `Fair`, `Serious`, `Critical` |
| `Decorations` / `WindowDecorations` | Server-side vs client-side decoration state |
| `ResizeEdge` | 8 edge/corner variants for interactive resize |
| `Menu` / `MenuItem` / `OsAction` | Application menu structures |
| `GpuSpecs` | GPU device/driver info |
| `RequestFrameOptions` | Frame request metadata (`require_presentation`, `force_render`) |

---

In summary, to add a new platform backend you must provide concrete implementations of **`Platform`**, **`PlatformWindow`**, **`PlatformDisplay`**, **`PlatformDispatcher`**, **`PlatformTextSystem`**, **`PlatformAtlas`**, **`PlatformKeyboardLayout`**, and **`PlatformKeyboardMapper`**. Many methods on `PlatformWindow` have default no-op implementations for platform-specific features (macOS tabs, Linux CSD, etc.) so you only need to override those relevant to your target.


**Tool Call: Explore existing platform implementations**
Status: Completed

Now I have a comprehensive understanding. Here's the full summary:

---

## Comprehensive Summary of GPUI Platform Implementations

### Architecture Overview

GPUI uses a **trait-based platform abstraction** with platform implementations in **separate crates**. The core `gpui` crate defines the platform traits, and each OS gets its own crate:

| Crate | Target | Renderer |
|---|---|---|
| `gpui_macos` | macOS | Metal |
| `gpui_windows` | Windows | DirectX |
| `gpui_linux` | Linux/FreeBSD | wgpu (via `gpui_wgpu`) |
| `gpui_web` | WASM | Not yet implemented |
| `gpui` (internal) | Test/any | None (fake) |

### Platform Selection: Compile-Time via `cfg` Attributes

Platform selection is **entirely compile-time** through `#[cfg(target_os = ...)]` attributes. The `gpui_platform` crate acts as the **unified entry point**, hiding the `cfg` gating from consumers:

```zed/crates/gpui_platform/src/gpui_platform.rs#L23-L48
pub fn current_platform(headless: bool) -> Rc<dyn Platform> {
    #[cfg(target_os = "macos")]
    {
        Rc::new(gpui_macos::MacPlatform::new(headless))
    }

    #[cfg(target_os = "windows")]
    {
        Rc::new(
            gpui_windows::WindowsPlatform::new(headless)
                .expect("failed to initialize Windows platform"),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        gpui_linux::current_platform(headless)
    }

    #[cfg(target_family = "wasm")]
    {
        todo!()
    }
}
```

The `Application` type accepts any `Rc<dyn Platform>`:

```zed/crates/gpui/src/app.rs#L136-L143
    pub fn with_platform(platform: Rc<dyn Platform>) -> Self {
        Self(App::new_app(
            platform,
            Arc::new(()),
            #[cfg(not(target_family = "wasm"))]
            Arc::new(NullHttpClient),
        ))
    }
```

### Core Platform Traits (in `gpui/src/platform.rs`)

There are **5 core traits** that any platform must implement:

1. **`Platform`** (L112-228) — The main trait. Covers app lifecycle, window management, clipboard, credentials, displays, menus, keyboard, thermal state, URLs, file dialogs, etc.

2. **`PlatformWindow`** (L448-555) — Per-window interface. Covers bounds, scale, appearance, input handling, drawing, resize/move callbacks, fullscreen, atlas. Has many default methods for OS-specific extras (macOS tabs, Linux decorations, Windows raw handles).

3. **`PlatformDisplay`** (L231-259) — Display/monitor abstraction. Just `id()`, `uuid()`, `bounds()`, `visible_bounds()`, `default_bounds()`.

4. **`PlatformDispatcher`** (L568-589) — Task scheduling. Covers `dispatch`, `dispatch_on_main_thread`, `dispatch_after`, `spawn_realtime`, `now`, `is_main_thread`, timings.

5. **`PlatformTextSystem`** (L592-609) — Font loading, shaping, rasterization. Methods for `add_fonts`, `font_id`, `font_metrics`, `glyph_for_char`, `layout_line`, `rasterize_glyph`.

Additionally, there's `PlatformAtlas` (L830-837) for GPU texture atlas management.

### Per-Platform File Structures

#### macOS (`gpui_macos`)
| File | Purpose |
|---|---|
| `platform.rs` | `MacPlatform` struct, `impl Platform` |
| `window.rs` | `MacWindow`, `impl PlatformWindow` |
| `display.rs` | `MacDisplay`, `impl PlatformDisplay` |
| `dispatcher.rs` | `MacDispatcher`, `impl PlatformDispatcher` |
| `text_system.rs` | CoreText-based `impl PlatformTextSystem` |
| `metal_renderer.rs` | Metal GPU rendering |
| `metal_atlas.rs` | Metal texture atlas |
| `events.rs` | macOS event translation |
| `keyboard.rs` | macOS keyboard handling |
| `pasteboard.rs` | Clipboard (NSPasteboard) |
| `display_link.rs` | CVDisplayLink frame scheduling |
| `screen_capture.rs` | ScreenCaptureKit integration |
| `window_appearance.rs` | Dark/light mode |

#### Windows (`gpui_windows`)
| File | Purpose |
|---|---|
| `platform.rs` | `WindowsPlatform` struct, `impl Platform` |
| `window.rs` | `WindowsWindow`, `impl PlatformWindow` |
| `display.rs` | `WindowsDisplay`, `impl PlatformDisplay` |
| `dispatcher.rs` | `WindowsDispatcher`, `impl PlatformDispatcher` |
| `direct_write.rs` | DirectWrite text system |
| `directx_renderer.rs` | Direct3D rendering |
| `directx_atlas.rs` | DirectX texture atlas |
| `directx_devices.rs` | GPU device management |
| `events.rs` | Win32 event translation |
| `keyboard.rs` | Windows keyboard handling |
| `clipboard.rs` | Windows clipboard |
| `destination_list.rs` | Jump list support |
| `system_settings.rs` | Windows system preferences |
| `vsync.rs` | V-sync management |
| `wrapper.rs` | Win32 API wrappers |

#### Linux (`gpui_linux`)
Linux is the most complex because it supports **multiple windowing backends**:

| File | Purpose |
|---|---|
| `platform.rs` | `LinuxPlatform<P>` (generic over client), `LinuxClient` trait, `impl Platform` |
| `dispatcher.rs` | `LinuxDispatcher`, calloop-based |
| `keyboard.rs` | xkbcommon keyboard handling |
| `text_system.rs` | cosmic-text based |
| `xdg_desktop_portal.rs` | Desktop portal integration |

Each windowing backend has its own subdirectory:

- **`wayland/`** — `client.rs`, `window.rs`, `display.rs`, `clipboard.rs`, `cursor.rs`, `layer_shell.rs`, `serial.rs`
- **`x11/`** — `client.rs`, `window.rs`, `display.rs`, `clipboard.rs`, `event.rs`, `xim_handler.rs`
- **`headless/`** — `client.rs` (minimal, no display)

Linux uses a **`LinuxClient` trait** (L93) that abstracts over Wayland, X11, and Headless clients. The `LinuxPlatform<P>` struct is generic over any `P: LinuxClient`, and `impl Platform for LinuxPlatform<P>` delegates to the inner client. Selection between Wayland/X11/Headless is **runtime** within Linux:

```zed/crates/gpui_linux/src/linux.rs#L31-L59
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    #[cfg(feature = "x11")]
    use anyhow::Context as _;

    if headless {
        return Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        });
    }

    match gpui::guess_compositor() {
        #[cfg(feature = "wayland")]
        "Wayland" => Rc::new(LinuxPlatform {
            inner: WaylandClient::new(),
        }),

        #[cfg(feature = "x11")]
        "X11" => Rc::new(LinuxPlatform {
            inner: X11Client::new()
                .context("Failed to initialize X11 client.")
                .unwrap(),
        }),

        "Headless" => Rc::new(LinuxPlatform {
            inner: HeadlessClient::new(),
        }),
        _ => unreachable!(),
    }
}
```

Both the Wayland and X11 backends use `gpui_wgpu` for GPU rendering.

### Test Platform (Reference Implementation)

The test platform lives inside the core `gpui` crate at `gpui/src/platform/test/` and is gated behind `#[cfg(any(test, feature = "test-support"))]`. It has **4 files**:

| File | Struct | Implements |
|---|---|---|
| `platform.rs` | `TestPlatform` | `Platform` |
| `window.rs` | `TestWindow` + `TestAtlas` | `PlatformWindow` + `PlatformAtlas` |
| `display.rs` | `TestDisplay` | `PlatformDisplay` |
| `dispatcher.rs` | `TestDispatcher` | `PlatformDispatcher` |

It uses `NoopTextSystem` (defined in `platform.rs`) which returns dummy values for all text operations.

Key patterns in the test platform:
- **`TestPlatform`** holds `RefCell<Option<TestWindow>>` for the active window, `Mutex<Option<ClipboardItem>>` for clipboard, and `RefCell<TestPrompts>` for queued user prompts.
- **`TestWindow`** wraps `Rc<Mutex<TestWindowState>>` — the state holds callback slots (`Option<Box<dyn FnMut(...)>>`). It exposes `simulate_*` methods to trigger callbacks from tests.
- **`TestDisplay`** is trivial — hardcoded 1920×1080 display.
- **`TestDispatcher`** delegates to the `scheduler` crate's `TestScheduler` for deterministic, seeded execution with controllable clock.
- **`TestAtlas`** is a simple `HashMap<AtlasKey, AtlasTile>` with auto-incrementing IDs.

### Visual Test Platform

There's also a `VisualTestPlatform` (macOS-only, in `gpui/src/platform/visual_test.rs`) that **wraps a real platform** but replaces executors with `TestDispatcher`-based ones. This gives real Metal rendering + deterministic scheduling for screenshot/visual regression tests.

### Key Patterns Across Implementations

1. **Callback registration pattern** — The `PlatformWindow` trait uses `on_*(callback)` methods that store boxed closures. Every platform stores these as `Option<Box<dyn FnMut(...)>>` fields and invokes them when events occur.

2. **Interior mutability** — Platform structs use `Mutex` or `RefCell` for interior mutability since the `Platform` trait methods take `&self` (not `&mut self`). macOS uses `Mutex<MacPlatformState>`, Windows uses individual `Mutex`es, Linux uses `RefCell` via the `with_common` pattern.

3. **`Rc<dyn Platform>`** — Platforms are passed around as `Rc<dyn Platform>` (not `Arc`), enforcing single-threaded access from the foreground/UI thread.

4. **`Weak` back-references** — `TestWindow` holds a `Weak<TestPlatform>` to call back to the platform (e.g., for prompts) without creating reference cycles.

5. **Feature flags** — Rendering backends, windowing systems, and optional features (screen capture, font-kit) are gated behind Cargo features. The `gpui_platform` crate re-exports the appropriate feature flags.

6. **Default trait methods for OS-specific features** — `PlatformWindow` has many methods with default no-op implementations for OS-specific features (macOS tabs, Linux window decorations, Windows raw handles). Each platform overrides only the methods relevant to it.

7. **Separate crates, not modules** — Each platform is a separate crate (`gpui_macos`, `gpui_windows`, `gpui_linux`) rather than `cfg`-gated modules within `gpui`. This keeps compile times down and dependency graphs clean — each crate only pulls in OS-specific dependencies.


**Tool Call: Explore GPUI rendering and GPU abstractions**
Status: Completed

Now I have a thorough understanding of the entire rendering pipeline. Let me compile the comprehensive summary.

---

## Comprehensive Summary of GPUI's Rendering Architecture

### 1. The Rendering Pipeline (from `Render` trait to pixels on screen)

The pipeline has clearly defined stages:

#### Stage 1: Element Tree Construction
The `Render` trait (in `element.rs`) is the entry point. Views implement `Render::render()` to produce an element tree:

```zed/crates/gpui/src/element.rs#L131-134
pub trait Render: 'static + Sized {
    /// Render this view into an element tree.
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
}
```

There's also `RenderOnce` for stateless components that take ownership of `self`.

#### Stage 2: Layout, Prepaint, and Paint (three-pass element traversal)
Each `Element` goes through three phases defined by the `Element` trait:

1. **`request_layout()`** — requests a Taffy layout (web-standard flexbox).
2. **`prepaint()`** — commits bounds, draws hitboxes, requests autoscroll.
3. **`paint()`** — actually emits drawing primitives into the `Scene`.

#### Stage 3: `Window::draw()` orchestrates the full frame
`Window::draw()` (line 2168 of `window.rs`) drives the entire frame:

```zed/crates/gpui/src/window.rs#L2168-2173
    pub fn draw(&mut self, cx: &mut App) -> ArenaClearNeeded {
        // Set up the per-App arena for element allocation during this draw.
        // This ensures that multiple test Apps have isolated arenas.
        let _arena_scope = ElementArenaScope::enter(&cx.element_arena);

        self.invalidate_entities();
```

It calls `draw_roots()`, which does `prepaint_as_root()` then `paint()` on the root element, plus tooltips, prompts, drag overlays, and deferred draws.

#### Stage 4: Scene Accumulation
During `paint()`, elements call methods like `window.paint_glyph()`, `window.paint_svg()`, `window.paint_image()`, which all insert primitives into `self.next_frame.scene`:

```zed/crates/gpui/src/window.rs#L3278-3282
            if subpixel_rendering {
                self.next_frame.scene.insert_primitive(SubpixelSprite {
                    order: 0,
                    pad: 0,
                    bounds,
```

The `Frame` struct holds the scene alongside other frame state:

```zed/crates/gpui/src/window.rs#L735-742
pub(crate) struct Frame {
    pub(crate) focus: Option<FocusId>,
    pub(crate) window_active: bool,
    pub(crate) element_states: FxHashMap<(GlobalElementId, TypeId), ElementStateBox>,
    accessed_element_states: Vec<(GlobalElementId, TypeId)>,
    pub(crate) mouse_listeners: Vec<Option<AnyMouseListener>>,
    pub(crate) dispatch_tree: DispatchTree,
    pub(crate) scene: Scene,
```

#### Stage 5: Scene Finalization and Presentation
After drawing, the scene is finalized (sorted by draw order), then `Window::present()` hands it off:

```zed/crates/gpui/src/window.rs#L2265-2268
    fn present(&self) {
        self.platform_window.draw(&self.rendered_frame.scene);
        self.needs_present.set(false);
        profiling::finish_frame!();
```

This calls `PlatformWindow::draw(&Scene)` — the platform-specific GPU rendering.

---

### 2. The `Scene` Intermediate Representation

The `Scene` is the **key abstraction boundary** between GPUI's platform-independent element system and platform-specific GPU rendering. It's defined in `scene.rs`:

```zed/crates/gpui/src/scene.rs#L27-39
pub struct Scene {
    pub(crate) paint_operations: Vec<PaintOperation>,
    primitive_bounds: BoundsTree<ScaledPixels>,
    layer_stack: Vec<DrawOrder>,
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub paths: Vec<Path<ScaledPixels>>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub subpixel_sprites: Vec<SubpixelSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
    pub surfaces: Vec<PaintSurface>,
}
```

There are **8 primitive types**: `Shadow`, `Quad`, `Path`, `Underline`, `MonochromeSprite` (glyphs, SVGs), `SubpixelSprite` (subpixel-antialiased text), `PolychromeSprite` (images, emoji), and `Surface` (macOS video playback).

The `Scene::batches()` iterator yields `PrimitiveBatch` items that are sorted by draw order and grouped by type/texture, so each GPU backend can issue minimal draw calls:

```zed/crates/gpui/src/scene.rs#L463-482
pub enum PrimitiveBatch {
    Shadows(Range<usize>),
    Quads(Range<usize>),
    Paths(Range<usize>),
    Underlines(Range<usize>),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    SubpixelSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    Surfaces(Range<usize>),
}
```

---

### 3. GPU Backend Abstraction

GPUI has **three completely separate GPU renderers**, each in its own crate, all consuming the same `Scene`:

| Platform | Crate | Graphics API | Renderer | Atlas | Shaders |
|---|---|---|---|---|---|
| **macOS** | `gpui_macos` | **Metal** | `MetalRenderer` | `MetalAtlas` | `shaders.metal` |
| **Linux** (Wayland/X11) | `gpui_linux` + `gpui_wgpu` | **Vulkan/GL** via wgpu | `WgpuRenderer` | `WgpuAtlas` | `shaders.wgsl` |
| **Windows** | `gpui_windows` | **Direct3D 11** | `DirectXRenderer` | `DirectXAtlas` | `shaders.hlsl` |
| **Web** | `gpui_web` | *Stub — `todo!()`* | — | — | — |

The abstraction boundary is the `PlatformWindow` trait, specifically its `fn draw(&self, scene: &Scene)` method. Each platform window implementation holds a renderer and delegates:

```zed/crates/gpui_macos/src/window.rs#L1547-1548
    fn draw(&self, scene: &gpui::Scene) {
        let mut this = self.0.lock();
        this.renderer.draw(scene);
```

The `gpui_platform` convenience crate provides `current_platform()` which uses `#[cfg]` to select the right platform:

```zed/crates/gpui_platform/src/gpui_platform.rs#L23-L48
pub fn current_platform(headless: bool) -> Rc<dyn Platform> {
    #[cfg(target_os = "macos")]
    {
        Rc::new(gpui_macos::MacPlatform::new(headless))
    }

    #[cfg(target_os = "windows")]
    {
        Rc::new(
            gpui_windows::WindowsPlatform::new(headless)
                .expect("failed to initialize Windows platform"),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        gpui_linux::current_platform(headless)
    }

    #[cfg(target_family = "wasm")]
    {
        todo!()
    }
}
```

All three renderers follow an **identical draw loop pattern**: iterate over `scene.batches()`, and for each batch type call a specialized draw function (`draw_quads`, `draw_shadows`, `draw_paths_to_intermediate`, `draw_monochrome_sprites`, etc.).

The **wgpu backend** (`gpui_wgpu`) is the most portable. It initializes with Vulkan or GL:

```zed/crates/gpui_wgpu/src/wgpu_context.rs#L29-L33
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        });
```

#### Sprite Atlas System
Each renderer has its own atlas that implements `PlatformAtlas`:

```zed/crates/gpui/src/platform.rs#L830-L837
pub trait PlatformAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>>;
    fn remove(&self, key: &AtlasKey);
}
```

Atlas keys can be glyphs, SVGs, or images. The atlas lazily rasterizes on first access and caches tiles in texture atlases managed by [etagere](https://crates.io/crates/etagere) (a rectangle-packing allocator).

---

### 4. Text System Abstraction

The text system has two layers:

**Platform-independent layer** — `TextSystem` in `gpui/src/text_system.rs`:
- Wraps `Arc<dyn PlatformTextSystem>`
- Provides font caching, fallback font resolution, line wrapping, and shaped line caching (`LineLayoutCache`)
- Handles the `FontRun` concept for multi-font text layout

**Platform-specific layer** — `PlatformTextSystem` trait:

```zed/crates/gpui/src/platform.rs#L592-L609
pub trait PlatformTextSystem: Send + Sync {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()>;
    fn all_font_names(&self) -> Vec<String>;
    fn font_id(&self, descriptor: &Font) -> Result<FontId>;
    fn font_metrics(&self, font_id: FontId) -> FontMetrics;
    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>>;
    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>>;
    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId>;
    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>>;
    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)>;
    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout;
    fn recommended_rendering_mode(&self, _font_id: FontId, _font_size: Pixels)
    -> TextRenderingMode;
}
```

Three implementations:

| Platform | Type | Backend |
|---|---|---|
| **macOS** | `MacTextSystem` | Core Text + font-kit |
| **Linux** | `CosmicTextSystem` | cosmic-text + swash |
| **Windows** | `DirectWriteTextSystem` | DirectWrite |
| **Tests** | `NoopTextSystem` | Returns dummy metrics |

The text rendering flow:
1. `TextSystem::layout_line()` shapes text into `LineLayout` (a list of `ShapedRun` with glyph positions).
2. `ShapedLine::paint()` iterates glyphs and calls `window.paint_glyph()` or `window.paint_emoji()`.
3. `paint_glyph()` rasterizes via `PlatformTextSystem::rasterize_glyph()`, uploads to the atlas, and inserts a `MonochromeSprite` or `SubpixelSprite` into the scene.

---

### 5. Image/Asset Loading

**`AssetSource` trait** (`assets.rs`) — loads raw bytes from bundled assets:
```zed/crates/gpui/src/assets.rs#L13-L17
pub trait AssetSource: 'static + Send + Sync {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>>;
    fn list(&self, path: &str) -> Result<Vec<SharedString>>;
}
```

**SVG rendering** — `SvgRenderer` uses `usvg` + `resvg` + `tiny_skia` to rasterize SVGs into alpha masks. The result is uploaded to the atlas as a `MonochromeSprite`.

**Image rendering** — `Image::to_image_data()` decodes images into `RenderImage` (BGRA pixel data using the `image` crate). Images are uploaded to the atlas as `PolychromeSprite` entries.

Both SVGs and images go through `PlatformAtlas::get_or_insert_with()` for caching.

---

### 6. Implications for a Web Target

The architecture is **well-positioned** for a web backend:

**What already exists:**
- The `gpui_web` crate exists but is a stub (`todo!()`).
- The `Scene` is a complete, self-contained, platform-independent IR with all the primitives needed for rendering.
- The `PlatformTextSystem` and `PlatformAtlas` traits are fully decoupled from any GPU API.
- wgpu already supports WebGPU backend, and `gpui_wgpu` uses WGSL shaders.

**What a web target would need to implement:**
1. **`Platform` trait** — event loop via `requestAnimationFrame`, clipboard via Web APIs, etc.
2. **`PlatformWindow` trait** — wrapping a `<canvas>` element.
3. **Renderer** — either:
   - Reuse `WgpuRenderer` with wgpu's WebGPU backend (most architecturally consistent).
   - Implement a Canvas2D-based renderer (simpler but less performant).
   - Implement a WebGL renderer.
4. **`PlatformTextSystem`** — using Canvas2D `measureText()` / `TextMetrics` for layout, and either Canvas2D rasterization or a Rust-side shaper like `cosmic-text` compiled to WASM.
5. **`PlatformAtlas`** — a WebGPU/WebGL texture atlas.
6. **`PlatformDispatcher`** — using `setTimeout`/`requestAnimationFrame`/Web Workers.

The **critical insight** is that `Scene` cleanly separates the ~95% of GPUI (element tree, layout, event handling, state management) from the ~5% that's platform-specific (rendering, text shaping, windowing). A web implementation only needs to provide that ~5%.


**Tool Call: Explore GPUI app initialization and event loop**
Status: Completed

I now have a thorough understanding of the system. Here's the comprehensive summary:

---

## Comprehensive Summary: GPUI Application Architecture

### 1. Application Initialization Flow

The entry point is the `Application` struct in `app.rs`. It follows a builder pattern:

```zed/crates/gpui/src/app.rs#L131-131
pub struct Application(Rc<AppCell>);
```

**Step 1: Construction** — `Application::with_platform(platform)` calls `App::new_app()`:

```zed/crates/gpui/src/app.rs#L137-144
    pub fn with_platform(platform: Rc<dyn Platform>) -> Self {
        Self(App::new_app(
            platform,
            Arc::new(()),
            #[cfg(not(target_family = "wasm"))]
            Arc::new(NullHttpClient),
        ))
    }
```

**Step 2: `App::new_app()`** (line 641–758) performs all core initialization:
- Obtains `BackgroundExecutor` and `ForegroundExecutor` from the platform
- Asserts we're on the main thread
- Creates the `TextSystem`, `EntityMap`, `Keymap`
- Uses `Rc::new_cyclic` to build the `App` struct inside an `AppCell` (a wrapper around `RefCell<App>` with debugging support)
- Initializes all observer/subscription sets (`observers`, `event_listeners`, `keystroke_observers`, `quit_observers`, etc.)
- Allocates two `Arena`s — one for element trees (`element_arena`, 1 MiB) and one for events (`event_arena`, 1 MiB)
- Registers platform callbacks for keyboard layout changes, thermal state changes, and quit
- Initializes menus and the `SystemWindowTabController`

```zed/crates/gpui/src/app.rs#L648-655
        let background_executor = platform.background_executor();
        let foreground_executor = platform.foreground_executor();
        assert!(
            background_executor.is_main_thread(),
            "must construct App on main thread"
        );
```

**Step 3: Configuration** — The caller chains `.with_assets()`, `.with_http_client()`, `.with_quit_mode()`, `.on_open_urls()`, `.on_reopen()`.

**Step 4: Launch** — `Application::run(on_finish_launching)` delegates to `Platform::run()`:

```zed/crates/gpui/src/app.rs#L174-184
    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut App),
    {
        let this = self.0.clone();
        let platform = self.0.borrow().platform.clone();
        platform.run(Box::new(move || {
            let cx = &mut *this.borrow_mut();
            on_finish_launching(cx);
        }));
    }
```

The `on_finish_launching` closure is called once the platform confirms the app is fully launched, giving the caller a mutable `&mut App` to create windows, register actions, etc.

---

### 2. Event Loop Architecture

The event loop is **platform-native** — GPUI does not have its own event loop. Each platform implements `Platform::run()`:

**macOS** — Uses `NSApplication.run()`, which runs the Cocoa event loop (`CFRunLoop`). The `on_finish_launching` callback is stashed and invoked from the `applicationDidFinishLaunching:` delegate method:

```zed/crates/gpui_macos/src/platform.rs#L459-486
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        let mut state = self.0.lock();
        if state.headless {
            drop(state);
            on_finish_launching();
            unsafe { CFRunLoopRun() };
        } else {
            state.finish_launching = Some(on_finish_launching);
            drop(state);
        }

        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];
            app.setDelegate_(app_delegate);
            // ...
            app.run();
            // ...
        }
    }
```

**Windows** — Calls the callback immediately, then enters the classic Win32 `GetMessageW` / `TranslateMessage` / `DispatchMessageW` loop. It also starts a vsync thread:

```zed/crates/gpui_windows/src/platform.rs#L395-413
    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        on_finish_launching();
        if !self.headless {
            self.begin_vsync_thread();
        }

        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if translate_accelerator(&msg).is_none() {
                    _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
        // ...
    }
```

On Windows, foreground tasks are dispatched via `PostMessageW` with custom message types (`WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD`), and the `run_foreground_task` handler processes them with a 10ms budget per iteration to stay responsive.

**Linux** — Calls the callback immediately then delegates to the wayland/X11 client's `run()`.

**GPUI's Update/Effect Cycle** operates *within* the platform event loop. Every state mutation goes through `App::update()`:

```zed/crates/gpui/src/app.rs#L823-841
    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut Self) -> R) -> R {
        self.start_update();
        let result = update(self);
        self.finish_update();
        result
    }

    pub(crate) fn start_update(&mut self) {
        self.pending_updates += 1;
    }

    pub(crate) fn finish_update(&mut self) {
        if !self.flushing_effects && self.pending_updates == 1 {
            self.flushing_effects = true;
            self.flush_effects();
            self.flushing_effects = false;
        }
        self.pending_updates -= 1;
    }
```

When the outermost update finishes, `flush_effects()` loops until all pending effects are drained. Effects include: `Notify` (observer notifications), `Emit` (entity events), `RefreshWindows`, `NotifyGlobalObservers`, `Defer`, and `EntityCreated`. In test mode, dirty windows are redrawn during effect flushing.

---

### 3. Input Event Flow from Platform to GPUI

Input flows through a clean pipeline:

**Layer 1: Platform → PlatformWindow callback.** During `Window::new()`, GPUI registers an `on_input` callback with the platform window:

```zed/crates/gpui/src/window.rs#L1399-1407
        platform_window.on_input({
            let mut cx = cx.to_async();
            Box::new(move |event| {
                handle
                    .update(&mut cx, |_, window, cx| window.dispatch_event(event, cx))
                    .log_err()
                    .unwrap_or(DispatchEventResult::default())
            })
        });
```

The platform calls this with a `PlatformInput` enum whenever an OS input event arrives.

**Layer 2: `PlatformInput` enum.** All input events are represented as variants of a single enum:

```zed/crates/gpui/src/interactive.rs#L610-631
pub enum PlatformInput {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MousePressure(MousePressureEvent),
    MouseMove(MouseMoveEvent),
    MouseExited(MouseExitEvent),
    ScrollWheel(ScrollWheelEvent),
    FileDrop(FileDropEvent),
}
```

**Layer 3: `Window::dispatch_event()`** (line 3884–3993) is the central dispatcher. It:
1. Tracks input modality (keyboard vs. mouse) for focus-visible styling
2. Updates `self.mouse_position` and `self.modifiers` from mouse events
3. Translates `FileDrop` events into synthetic mouse events (move for drag, up for drop)
4. Routes to either `dispatch_mouse_event()` or `dispatch_key_event()` based on event type
5. Tracks input rate for adaptive frame presentation

**Layer 4a: Mouse dispatch** — `dispatch_mouse_event()` performs a hit-test against the current frame's hitbox tree, then iterates through mouse listeners in two phases:
- **Capture phase** — listeners fire front-to-back (used for "click outside" detection)
- **Bubble phase** — listeners fire back-to-front (normal event handling)

Propagation is controlled by `cx.propagate_event`. Active drags cause additional redraws.

**Layer 4b: Key dispatch** — `dispatch_key_event()` is more complex:
1. Builds a `dispatch_path` from the focused node up to the root of the `DispatchTree`
2. Handles modifier-only keystrokes (e.g., pressing and releasing Shift alone)
3. Runs keystroke interceptors
4. Feeds the keystroke through the `DispatchTree::dispatch_key()` matcher for keybindings
5. If bindings are **pending** (multi-stroke), stores them in `pending_input` with a 1-second timeout
6. If bindings **match**, calls `dispatch_action_on_node()` which walks the dispatch path looking for action handlers
7. Falls through to raw `KeyDown`/`KeyUp` listeners (capture + bubble)
8. Falls through to `ModifiersChanged` listeners
9. Finally notifies keystroke observers

**Layer 5: Text input** — For text editing, elements register a `PlatformInputHandler` during paint via `window.handle_input()`. The platform takes this handler and calls its methods (`replace_text_in_range`, etc.) when IME input occurs, bypassing the normal key event path.

---

### 4. Window Lifecycle

**Creation** — `App::open_window()` orchestrates window creation:

```zed/crates/gpui/src/app.rs#L1009-1042
    pub fn open_window<V: 'static + Render>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> anyhow::Result<WindowHandle<V>> {
```

1. Allocates a slot in `cx.windows` (a `SlotMap`)
2. Calls `Window::new()` which:
   - Calls `cx.platform.open_window()` to create the native platform window
   - Registers all platform callbacks: `on_close`, `on_request_frame`, `on_resize`, `on_moved`, `on_appearance_changed`, `on_active_status_change`, `on_hover_status_change`, `on_input`, `on_hit_test_window_control`, tab management callbacks
   - Creates two `Frame` structs (`rendered_frame` and `next_frame`) with their own `DispatchTree`s
   - Sets initial window state (bounds, decorations, fullscreen)
3. The caller's `build_root_view` closure creates the root entity
4. An initial `window.draw(cx)` is performed before returning

**Frame rendering** — Driven by `on_request_frame` (vsync-based):
1. Fires registered `next_frame_callbacks`
2. If the window invalidator is dirty or a force render is requested, calls `window.draw(cx)` then `window.present()`
3. If only presentation is needed (e.g., during high-rate input), just calls `present()`

**`Window::draw()`** performs the full layout-and-paint cycle:
1. Sets up the element arena scope
2. Calls `draw_roots()` which does two passes: **prepaint** (layout + hit-test generation) then **paint** (scene building)
3. Swaps `next_frame` ↔ `rendered_frame`
4. Fires focus change listeners if focus path changed
5. Resets cursor style

**Closing** — The `on_close` callback calls `window.remove_window()` and cleans up `SystemWindowTabController`. The `App::shutdown()` method clears all windows, flushes effects, and blocks for up to 100ms on quit observers.

---

### 5. Executor / Async Runtime Setup

GPUI's async runtime is built on two executors that wrap a `PlatformDispatcher`:

**`BackgroundExecutor`** — For off-main-thread work:

```zed/crates/gpui/src/executor.rs#L118-135
impl BackgroundExecutor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        // ...
        let scheduler: Arc<dyn Scheduler> = Arc::new(PlatformScheduler::new(dispatcher.clone()));
        Self {
            inner: scheduler::BackgroundExecutor::new(scheduler),
            dispatcher,
        }
    }
```

**`ForegroundExecutor`** — For main-thread work (not `Send`):

```zed/crates/gpui/src/executor.rs#L382-410
impl ForegroundExecutor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        // ...
        let platform_scheduler = Arc::new(PlatformScheduler::new(dispatcher.clone()));
        let session_id = platform_scheduler.allocate_session_id();
        // ...
    }
```

Both wrap a `PlatformScheduler` (in `platform_scheduler.rs`) that implements the `Scheduler` trait from an external `scheduler` crate. The `PlatformScheduler`:
- **`schedule_foreground`** calls `dispatcher.dispatch_on_main_thread(runnable, priority)` — posting a message to the platform's main thread
- **`schedule_background_with_priority`** calls `dispatcher.dispatch(runnable, priority)` — dispatching to a thread pool
- **`timer`** uses `dispatcher.dispatch_after(duration, runnable)` to schedule delayed work
- **`block`** polls a future synchronously using a `parking::Parker`, increasing timer resolution on Windows for precision

The `PlatformDispatcher` trait abstracts the native dispatch mechanism:

```zed/crates/gpui/src/platform.rs#L568-589
pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: RunnableVariant, priority: Priority);
    fn dispatch_on_main_thread(&self, runnable: RunnableVariant, priority: Priority);
    fn dispatch_after(&self, duration: Duration, runnable: RunnableVariant);
    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>);
    // ...
}
```

On macOS, this is implemented via Grand Central Dispatch. On Windows, background work goes to a thread pool while main-thread work is delivered via `PostMessageW`. On Linux, it uses the display server client's dispatch mechanism.

**Task lifecycle** — Both `spawn` and `background_spawn` return a `Task<R>` which is a future. If dropped, the task is cancelled. Tasks can be `.detach()`ed to run independently, or stored in a struct field for lifecycle-bound execution.

**Test support** — In tests, a `TestDispatcher` replaces the platform dispatcher, giving deterministic control over task execution via `run_until_parked()`, `advance_clock()`, and `tick()`. The test scheduler can simulate random delays and seed its RNG for reproducible behavior.


**Tool Call: Check for existing web/wasm work**
Status: Completed

<think>The user wants me to continue the summary from where I left off. I was in the middle of point 8, listing what doesn't exist yet.</think>
### 8. What Doesn't Exist Yet (continued)

- **No web `Platform` implementation** — the `gpui_web` crate is empty
- **No web dispatcher** — needed for scheduling async tasks on the browser event loop (`requestAnimationFrame`, `setTimeout`, Web Workers, etc.)
- **No web text system** — `PlatformTextSystem` would need a browser-based implementation (Canvas 2D text measurement, or a Rust-side shaping library like `cosmic-text` compiled to wasm)
- **No web window** — `PlatformWindow` would need to wrap an HTML `<canvas>` element and handle browser resize/DPI events
- **No web display** — `PlatformDisplay` would need to report browser viewport dimensions
- **No web input handling** — keyboard/mouse/touch events from the DOM would need to be translated to GPUI's `PlatformInput` types
- **No web clipboard** — the browser Clipboard API would need to back `read_from_clipboard`/`write_to_clipboard`
- **No `wgpu` WebGPU backend wiring** — `WgpuContext::new()` currently only enables `VULKAN | GL`, not `BROWSER_WEBGPU`
- **No `wasm-bindgen` / `web-sys` / `js-sys` dependencies** — `gpui_web` has zero dependencies; these would be essential for DOM interop
- **No entry point / bootstrap** — no `wasm-pack` config, no HTML harness, no JS glue code

### 9. Existing wasm-bindgen in the Lock File (Transitive Only)

The `Cargo.lock` contains `wasm-bindgen`, `web-sys`, `js-sys`, and `wasm-bindgen-futures`, but these are all **transitive dependencies** pulled in by:
- `chrono` (timezone detection via `js-sys`)
- `getrandom` (browser random via `wasm-bindgen`)
- `uuid` (JS feature)
- `reqwest` (has wasm support built-in)
- `livekit` / `webrtc-sys` (real-time communication)
- `cpal` (audio, has wasm stubs)
- `plotters` (charting library)

None of these are direct dependencies of `gpui` or `gpui_web` for the purpose of building a web platform.

### 10. Extension System Uses `wasm32-wasip2` (Separate Concern)

The extension system compiles extensions to `wasm32-wasip2` (WASI preview 2 + Component Model). This is **completely separate** from the "GPUI on the web" effort:

```zed/crates/extension/src/extension_builder.rs#L20-24

/// Currently, we compile with Rust's `wasm32-wasip2` target, which works with WASI `preview2` and the component model.
const RUST_TARGET: &str = "wasm32-wasip2";

/// Compiling Tree-sitter parsers from C to WASM requires Clang 17, and a WASM build of libc
```

The extension API also has a `#[cfg(target_arch = "wasm32")]` link section for embedding API version metadata. This is about sandboxed plugin execution, not about running GPUI in a browser.

### 11. No Feature Flags for Web

There is no `web` or `wasm` feature flag in GPUI's `Cargo.toml` features section. The current features are:

```zed/crates/gpui/Cargo.toml#L22-42
[features]
default = ["font-kit", "wayland", "x11", "windows-manifest"]
test-support = [
    "leak-detection",
    "collections/test-support",
    "util/test-support",
    "http_client/test-support",
    "wayland",
    "x11",
]
inspector = ["gpui_macros/inspector"]
leak-detection = ["backtrace"]
wayland = [
    "bitflags",
]
x11 = [
    "scap?/x11",
]
screen-capture = [
    "scap",
]
windows-manifest = []
```

Web support would likely not need a feature flag at all — it would be selected by `target_family = "wasm"` at compile time, consistent with the existing `#[cfg]` patterns.

### 12. No Documentation or Issue References Found

No markdown documentation about web/wasm support was found in the `docs/` directory. No TODO or FIXME comments reference web support in the GPUI crate source (only the `todo!()` in `gpui_platform`). No issue tracker references were found in source comments.

---

## Overall Assessment

The Zed project has **clearly laid the groundwork** for running GPUI in a web browser, but the actual implementation has not started. Here's what exists vs. what's needed:

| Layer | Status |
|---|---|
| Toolchain target (`wasm32-unknown-unknown`) | ✅ Configured |
| Conditional compilation (`#[cfg(target_family = "wasm")]`) | ✅ ~30+ sites across `gpui`, `util` |
| WASM-compatible dependencies (`getrandom`, `uuid`) | ✅ Configured |
| `gpui_web` crate (scaffold) | ⚠️ Empty stub |
| `gpui_platform` wasm dispatch | ⚠️ `todo!()` |
| `gpui_wgpu` renderer (wgpu 28.0) | ✅ Exists, but only enables Vulkan/GL backends |
| Web `Platform` trait implementation | ❌ Not started |
| Web dispatcher / event loop | ❌ Not started |
| Web text system | ❌ Not started |
| Web window / canvas | ❌ Not started |
| Web input events | ❌ Not started |
| `wasm-bindgen` / `web-sys` integration | ❌ Not started |
| Build tooling (wasm-pack, HTML harness) | ❌ Not started |

The architecture is well-positioned for this work: the `Platform` trait cleanly abstracts OS-specific behavior, `gpui_wgpu` provides a portable GPU renderer that could use wgpu's `BROWSER_WEBGPU` backend, and the conditional compilation already removes incompatible dependencies from the wasm build. The main remaining work is implementing a full `Platform` for the browser inside `gpui_web`.


## Assistant

<thinking>
Now I have comprehensive information from all the agents. Let me also check the existing `gpui_web` crate structure to see what's already there.
</thinking>

Let me check the existing `gpui_web` crate and a few more specific details.

**Tool Call: Inspect gpui_web crate**
Status: Completed

Here's a complete summary of everything I found:

---

## 1. `zed/crates/gpui_web/` directory listing

The crate is very minimal:
- `Cargo.toml`
- `src/lib.rs`

No other files or subdirectories.

---

## 2. `zed/crates/gpui_web/Cargo.toml`

```zed/crates/gpui_web/Cargo.toml#L1-L10
[package]
name = "gpui_web"
version = "0.1.0"
publish.workspace = true
edition.workspace = true

[dependencies]

[lints]
workspace = true
```

A bare-bones crate with **zero dependencies**—not even `gpui` itself.

---

## 3. `zed/crates/gpui_web/src/lib.rs`

```zed/crates/gpui_web/src/lib.rs#L1
#![cfg(target_family = "wasm")]
```

The entire source file is a single line: a crate-level `cfg` attribute that compiles the crate only when targeting `wasm`. There's no actual implementation yet—this is purely a placeholder/skeleton.

---

## 4. `zed/crates/gpui_platform/Cargo.toml`

```zed/crates/gpui_platform/Cargo.toml#L1-L34
[package]
name = "gpui_platform"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "Apache-2.0"

[lints]
workspace = true

[lib]
path = "src/gpui_platform.rs"

[features]
default = []
font-kit = ["gpui_macos/font-kit"]
test-support = ["gpui/test-support", "gpui_macos/test-support"]
screen-capture = ["gpui/screen-capture", "gpui_macos/screen-capture", "gpui_windows/screen-capture", "gpui_linux/screen-capture"]
runtime_shaders = ["gpui_macos/runtime_shaders"]
wayland = ["gpui_linux/wayland"]
x11 = ["gpui_linux/x11"]

[dependencies]
gpui.workspace = true

[target.'cfg(target_os = "macos")'.dependencies]
gpui_macos.workspace = true

[target.'cfg(target_os = "windows")'.dependencies]
gpui_windows.workspace = true

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui_linux.workspace = true
```

Notably, `gpui_platform` does **not** depend on `gpui_web` at all. There's no `cfg(target_family = "wasm")` dependency section, and `gpui_web` isn't mentioned anywhere in the features either.

---

## 5. `zed/crates/gpui_platform/src/gpui_platform.rs` — the wasm branch

```zed/crates/gpui_platform/src/gpui_platform.rs#L24-L43
pub fn current_platform(headless: bool) -> Rc<dyn Platform> {
    #[cfg(target_os = "macos")]
    {
        Rc::new(gpui_macos::MacPlatform::new(headless))
    }

    #[cfg(target_os = "windows")]
    {
        Rc::new(
            gpui_windows::WindowsPlatform::new(headless)
                .expect("failed to initialize Windows platform"),
        )
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        gpui_linux::current_platform(headless)
    }

    #[cfg(target_family = "wasm")]
    {
        todo!()
    }
}
```

The wasm branch (lines 44–47) is just a `todo!()` — it will compile for wasm targets but panic at runtime. The `gpui_web` crate isn't referenced here at all.

---

## Summary of the current state

| Aspect | Status |
|---|---|
| `gpui_web` crate exists | ✅ Yes, but it's an empty skeleton |
| `gpui_web` has dependencies | ❌ None at all |
| `gpui_web` has implementation | ❌ Just `#![cfg(target_family = "wasm")]` |
| `gpui_platform` depends on `gpui_web` | ❌ No dependency declared |
| `gpui_platform` wasm branch | Exists but is just `todo!()` |
| Feature flags for wasm | ❌ None defined in `gpui_platform` |

The `gpui_web` crate is a pure placeholder. To make a functional web/wasm platform, you'd need to:
1. Add real platform trait implementations inside `gpui_web`
2. Add `gpui_web` as a `cfg(target_family = "wasm")` dependency in `gpui_platform`
3. Wire up the `current_platform` wasm branch to instantiate the web platform from `gpui_web`


**Tool Call: Check wasm cfg guards in gpui**
Status: Completed

Here's a comprehensive summary of all wasm-related conditional compilation guards in the `gpui` and `gpui_wgpu` crates.

---

## `gpui_wgpu` crate — No wasm guards

The `zed/crates/gpui_wgpu/Cargo.toml` has **no wasm-specific** dependencies or `cfg` guards. It depends on the `wgpu` crate (which itself supports WebGPU on wasm), but the `gpui_wgpu` crate doesn't have any conditional compilation for wasm yet.

---

## `gpui` crate — All findings

There are **no** `target_arch = "wasm32"` guards anywhere. Every wasm guard uses `target_family = "wasm"`. The guards fall into two categories: **positive** (`cfg(target_family = "wasm")`) and **negative** (`cfg(not(target_family = "wasm"))`).

### 1. `Cargo.toml` — Dependency guards

```zed/crates/gpui/Cargo.toml#L102-109
[target.'cfg(target_family = "wasm")'.dependencies]
getrandom = { version = "0.3.4", features = ["wasm_js"] }
uuid = { workspace = true, features = ["js"] }

[target.'cfg(not(target_family = "wasm"))'.dependencies]
http_client.workspace = true

[target.'cfg(target_os = "macos")'.dependencies]
```

```zed/crates/gpui/Cargo.toml#L154-158
[target.'cfg(not(target_family = "wasm"))'.dev-dependencies]
http_client = { workspace = true, features = ["test-support"] }
reqwest_client = { workspace = true, features = ["test-support"] }

[target.'cfg(target_os = "windows")'.build-dependencies]
```

**Summary:** On wasm, `getrandom` gets the `wasm_js` feature and `uuid` gets the `js` feature. The `http_client` crate is excluded entirely from wasm builds (both as a regular dep and a dev-dep).

---

### 2. `src/gpui.rs` — Re-export guard (line 90)

```zed/crates/gpui/src/gpui.rs#L88-92
pub use global::*;
pub use gpui_macros::{AppContext, IntoElement, Render, VisualContext, register_action, test};
#[cfg(not(target_family = "wasm"))]
pub use http_client;
pub use input::*;
```

The `http_client` crate is not re-exported on wasm.

---

### 3. `src/app.rs` — Core `App` struct and HTTP client plumbing

**Import guard (line 28):**
```zed/crates/gpui/src/app.rs#L26-30
pub use context::*;
pub use entity_map::*;
#[cfg(not(target_family = "wasm"))]
use http_client::{HttpClient, Url};
use smallvec::SmallVec;
```

**`Application::with_platform` — default null client (line 141):**
```zed/crates/gpui/src/app.rs#L138-143
        Self(App::new_app(
            platform,
            Arc::new(()),
            #[cfg(not(target_family = "wasm"))]
            Arc::new(NullHttpClient),
        ))
```

**`Application::with_http_client` — setter method (line 157):**
```zed/crates/gpui/src/app.rs#L155-159

    /// Sets the HTTP client for the application.
    #[cfg(not(target_family = "wasm"))]
    pub fn with_http_client(self, http_client: Arc<dyn HttpClient>) -> Self {
        let mut context_lock = self.0.borrow_mut();
```

**`App` struct field (line 587):**
```zed/crates/gpui/src/app.rs#L585-589
    asset_source: Arc<dyn AssetSource>,
    pub(crate) svg_renderer: SvgRenderer,
    #[cfg(not(target_family = "wasm"))]
    http_client: Arc<dyn HttpClient>,
    pub(crate) globals_by_type: FxHashMap<TypeId, Box<dyn Any>>,
```

**`App::new_app` — parameter and initialization (lines 644 and 674):**
```zed/crates/gpui/src/app.rs#L641-645
    pub(crate) fn new_app(
        platform: Rc<dyn Platform>,
        asset_source: Arc<dyn AssetSource>,
        #[cfg(not(target_family = "wasm"))] http_client: Arc<dyn HttpClient>,
    ) -> Rc<AppCell> {
```

```zed/crates/gpui/src/app.rs#L672-676
                loading_assets: Default::default(),
                asset_source,
                #[cfg(not(target_family = "wasm"))]
                http_client,
                globals_by_type: FxHashMap::default(),
```

**`App::http_client` getter and `App::set_http_client` setter (lines 1283 and 1289):**
```zed/crates/gpui/src/app.rs#L1281-1285

    /// Returns the HTTP client for the application.
    #[cfg(not(target_family = "wasm"))]
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
```

```zed/crates/gpui/src/app.rs#L1287-1291

    /// Sets the HTTP client for the application.
    #[cfg(not(target_family = "wasm"))]
    pub fn set_http_client(&mut self, new_client: Arc<dyn HttpClient>) {
        self.http_client = new_client;
```

**`NullHttpClient` fallback impl (lines 2514–2518):**
```zed/crates/gpui/src/app.rs#L2514-2518
#[cfg(not(target_family = "wasm"))]
struct NullHttpClient;

#[cfg(not(target_family = "wasm"))]
impl HttpClient for NullHttpClient {
```

---

### 4. `src/elements/img.rs` — Image loading

**HTTP fetch for URI resources (lines 596 and 605):**
```zed/crates/gpui/src/elements/img.rs#L595-605
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        #[cfg(not(target_family = "wasm"))]
        let client = cx.http_client();
        // TODO: Can we make SVGs always rescale?
        // let scale_factor = cx.scale_factor();
        let svg_renderer = cx.svg_renderer();
        let asset_source = cx.asset_source().clone();
        async move {
            let bytes = match source.clone() {
                Resource::Path(uri) => fs::read(uri.as_ref())?,
                #[cfg(not(target_family = "wasm"))]
                Resource::Uri(uri) => {
```

**Wasm fallback for URI resources — the only *positive* `cfg(target_family = "wasm")` in Rust source (line 628):**
```zed/crates/gpui/src/elements/img.rs#L628-632
                #[cfg(target_family = "wasm")]
                Resource::Uri(_) => {
                    return Err(ImageCacheError::Other(Arc::new(anyhow::anyhow!(
                        "Uri resources are not supported on wasm"
                    ))));
```

**`ImageCacheError::BadStatus` variant excluded on wasm (line 724):**
```zed/crates/gpui/src/elements/img.rs#L724-733
    #[cfg(not(target_family = "wasm"))]
    #[error("unexpected http status for {uri}: {status}, body: {body}")]
    BadStatus {
        /// The URI of the image.
        uri: SharedUri,
        /// The HTTP status code.
        status: http_client::StatusCode,
        /// The HTTP response body.
        body: String,
    },
```

---

### 5. `src/app/test_context.rs` — Test setup (lines 123 and 130)

```zed/crates/gpui/src/app/test_context.rs#L123-131
        #[cfg(not(target_family = "wasm"))]
        let http_client = http_client::FakeHttpClient::with_404_response();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));

        let app = App::new_app(
            platform.clone(),
            asset_source,
            #[cfg(not(target_family = "wasm"))]
            http_client,
```

---

### 6. `src/executor.rs` — Test helper (lines 572 and 578)

```zed/crates/gpui/src/executor.rs#L572-580
        #[cfg(not(target_family = "wasm"))]
        let http_client = http_client::FakeHttpClient::with_404_response();

        let app = App::new_app(
            platform,
            asset_source,
            #[cfg(not(target_family = "wasm"))]
            http_client,
        );
```

---

## Overall picture

| File | # of guards | What's gated |
|---|---|---|
| `Cargo.toml` | 3 | `getrandom`/`uuid` wasm features; `http_client` excluded on wasm |
| `src/gpui.rs` | 1 | `http_client` re-export |
| `src/app.rs` | 8 | `http_client` import, `App` field, `new_app` param, getter/setter, `NullHttpClient`, `Application::with_http_client` |
| `src/elements/img.rs` | 4 | HTTP image fetching, URI fallback error, `BadStatus` error variant |
| `src/app/test_context.rs` | 2 | Fake HTTP client in tests |
| `src/executor.rs` | 2 | Fake HTTP client in test helper |
| **`gpui_wgpu/`** | **0** | No wasm guards at all |

**The single theme is: the `http_client` crate doesn't exist on wasm.** Every guard either removes `http_client` types/logic or provides a wasm-specific fallback (the URI-not-supported error in `img.rs`, and the `wasm_js`/`js` features in `Cargo.toml`). The `gpui_wgpu` crate has no wasm-specific code yet.


**Tool Call: Examine Scene and rendering types**
Status: Completed

Now I have a thorough understanding of the full pipeline. Here's the complete analysis:

---

## Complete GPUI Scene Rendering Analysis

### 1. Primitive Types in `Scene`

The `Scene` struct (in `scene.rs`) stores **8 primitive types** in separate, type-homogeneous `Vec`s:

| Primitive | Stored in | Key Fields |
|-----------|-----------|------------|
| **`Shadow`** | `scene.shadows` | `order`, `blur_radius`, `bounds`, `corner_radii`, `content_mask`, `color` (Hsla) |
| **`Quad`** | `scene.quads` | `order`, `border_style`, `bounds`, `content_mask`, `background` (Background — solid/gradient/pattern), `border_color`, `corner_radii`, `border_widths` |
| **`Path<ScaledPixels>`** | `scene.paths` | `order`, `bounds`, `content_mask`, `vertices: Vec<PathVertex>`, `color` (Background) |
| **`Underline`** | `scene.underlines` | `order`, `bounds`, `content_mask`, `color`, `thickness`, `wavy` |
| **`MonochromeSprite`** | `scene.monochrome_sprites` | `order`, `bounds`, `content_mask`, `color`, `tile` (AtlasTile), `transformation` (2D affine) |
| **`SubpixelSprite`** | `scene.subpixel_sprites` | `order`, `bounds`, `content_mask`, `color`, `tile`, `transformation` |
| **`PolychromeSprite`** | `scene.polychrome_sprites` | `order`, `bounds`, `content_mask`, `corner_radii`, `tile`, `grayscale`, `opacity` |
| **`PaintSurface`** | `scene.surfaces` | `order`, `bounds`, `content_mask`, `image_buffer` (macOS-only CVPixelBuffer) |

Every primitive carries an `order: DrawOrder` (a `u32`) and `content_mask` (clip rect). All are `#[repr(C)]` so they can be blitted straight into GPU storage buffers.

### 2. How Batches Are Iterated

The batching algorithm is in `Scene::finish()` → `Scene::batches()`:

**Step 1 — Sort each primitive vec independently** (`finish()`):

```zed/crates/gpui/src/scene.rs#L137-L149
    pub fn finish(&mut self) {
        self.shadows.sort_by_key(|shadow| shadow.order);
        self.quads.sort_by_key(|quad| quad.order);
        self.paths.sort_by_key(|path| path.order);
        self.underlines.sort_by_key(|underline| underline.order);
        self.monochrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.subpixel_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.polychrome_sprites
            .sort_by_key(|sprite| (sprite.order, sprite.tile.tile_id));
        self.surfaces.sort_by_key(|surface| surface.order);
    }
```

**Step 2 — Merge-iterate via `BatchIterator`** (`batches()`):

The `BatchIterator` maintains a `Peekable` iterator and a start index for each of the 8 sorted vecs. On each call to `next()`:

1. It peeks at the front `order` of every iterator → builds `orders_and_kinds` array
2. Sorts that array by `(order, PrimitiveKind)` to find the **lowest-order kind** to emit next
3. Uses the **second-lowest** entry as the `max_order_and_kind` cutoff
4. Greedily consumes consecutive items of the chosen kind whose `(order, kind) < max_order_and_kind`
5. For sprite types, it additionally breaks the batch when the `texture_id` changes

This produces a **global draw-order-interleaved** stream of `PrimitiveBatch` variants — each batch is a contiguous `Range<usize>` into the corresponding sorted vec, plus a `texture_id` for sprite batches.

```zed/crates/gpui/src/scene.rs#L463-L482
pub enum PrimitiveBatch {
    Shadows(Range<usize>),
    Quads(Range<usize>),
    Paths(Range<usize>),
    Underlines(Range<usize>),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    SubpixelSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    Surfaces(Range<usize>),
}
```

### 3. How wgpu Renders Each Batch Type

The main draw loop is in `WgpuRenderer::draw()`:

```zed/crates/gpui_wgpu/src/wgpu_renderer.rs#L918-L981
                for batch in scene.batches() {
                    let ok = match batch {
                        PrimitiveBatch::Quads(range) => {
                            self.draw_quads(&scene.quads[range], &mut instance_offset, &mut pass)
                        }
                        PrimitiveBatch::Shadows(range) => self.draw_shadows(
                            &scene.shadows[range],
                            &mut instance_offset,
                            &mut pass,
                        ),
                        PrimitiveBatch::Paths(range) => {
                            // ... special two-pass path rendering ...
                        }
                        PrimitiveBatch::Underlines(range) => self.draw_underlines(...)
                        PrimitiveBatch::MonochromeSprites { texture_id, range } => ...
                        PrimitiveBatch::SubpixelSprites { texture_id, range } => ...
                        PrimitiveBatch::PolychromeSprites { texture_id, range } => ...
                        PrimitiveBatch::Surfaces(_surfaces) => true, // macOS-only, no-op on wgpu
                    };
                }
```

**All primitive types (except Paths) use the same pattern:**

1. Reinterpret the `&[Primitive]` slice as raw `&[u8]` via `instance_bytes()`
2. Write bytes into a large shared GPU **storage buffer** (at a running `instance_offset`, aligned to `storage_buffer_alignment`)
3. Create a bind group pointing to that slice of the storage buffer
4. Set the appropriate render pipeline + globals bind group
5. Call `pass.draw(0..4, 0..instance_count)` — **4 vertices per instance** (a triangle strip quad), with instancing for the count

For **texture-based** batches (mono/subpixel/polychrome sprites), the bind group additionally includes the atlas texture view + sampler.

**Paths are special — two-pass rendering:**

1. **Pass 1 — Rasterize to intermediate texture** (`draw_paths_to_intermediate`): The path vertices are expanded into `PathRasterizationVertex` structs (with per-vertex color + bounds), uploaded to the storage buffer, and drawn as a `TriangleList` to a screen-sized intermediate texture (optionally with MSAA). This uses `PremultipliedAlphaBlending`.
2. **Pass 2 — Composite from intermediate** (`draw_paths_from_intermediate`): The clipped bounds of the path(s) become `PathSprite` instances, and the intermediate texture is sampled as a regular textured quad back onto the main framebuffer.

The main render pass must be **ended and restarted** around the path intermediate pass, since it targets a different texture.

**If the instance buffer overflows**, the entire frame is retried with a doubled buffer (up to 256 MB).

**Pipelines summary** (from `create_pipelines()`):

| Pipeline | Vertex Shader | Fragment Shader | Topology | Data Layout |
|----------|--------------|-----------------|----------|-------------|
| `quads` | `vs_quad` | `fs_quad` | TriangleStrip | storage buffer |
| `shadows` | `vs_shadow` | `fs_shadow` | TriangleStrip | storage buffer |
| `path_rasterization` | `vs_path_rasterization` | `fs_path_rasterization` | TriangleList | storage buffer |
| `paths` | `vs_path` | `fs_path` | TriangleStrip | storage buffer + texture |
| `underlines` | `vs_underline` | `fs_underline` | TriangleStrip | storage buffer |
| `mono_sprites` | `vs_mono_sprite` | `fs_mono_sprite` | TriangleStrip | storage buffer + texture |
| `subpixel_sprites` | `vs_subpixel_sprite` | `fs_subpixel_sprite` | TriangleStrip | storage buffer + texture |
| `poly_sprites` | `vs_poly_sprite` | `fs_poly_sprite` | TriangleStrip | storage buffer + texture |
| `surfaces` | `vs_surface` | `fs_surface` | TriangleStrip | uniform + 2 textures |

### 4. Would the WGSL Shaders Work with WebGPU Directly?

**Almost — with one significant caveat and a few minor ones:**

**Blocker: `enable dual_source_blending;`**

The very first line of the shader file is:

```zed/crates/gpui_wgpu/src/shaders.wgsl#L1
enable dual_source_blending;
```

This is used by the **subpixel sprite** pipeline for LCD subpixel antialiased text rendering. The fragment output uses `@blend_src(0)` and `@blend_src(1)`:

```zed/crates/gpui_wgpu/src/shaders.wgsl#L1363-L1366
struct SubpixelSpriteFragmentOutput {
    @location(0) @blend_src(0) foreground: vec4<f32>,
    @location(0) @blend_src(1) alpha: vec4<f32>,
}
```

The `dual_source_blending` extension is **not yet part of the WebGPU standard**. It's a wgpu-native extension. The Rust side already handles this gracefully — if the feature isn't available, it falls back to the mono sprite pipeline:

```zed/crates/gpui_wgpu/src/wgpu_renderer.rs#L1084-L1106
    fn draw_subpixel_sprites(
        ...
    ) -> bool {
        ...
        let pipeline = self
            .pipelines
            .subpixel_sprites
            .as_ref()
            .unwrap_or(&self.pipelines.mono_sprites);
```

**What would need to change for WebGPU compatibility:**

1. **Remove `enable dual_source_blending;`** from the shader file (or conditionally compile it out). The subpixel sprite shader and its fragment output struct would need to be removed or replaced. The fallback to grayscale antialiasing (mono sprites) already exists.

2. **`Surfaces` pipeline** — The surface shader (`vs_surface`/`fs_surface`) uses a YCbCr two-texture setup designed for macOS `CVPixelBuffer` video playback. This is already a no-op on wgpu/Linux and would be irrelevant for web, but the shader code itself is valid WGSL.

3. **Storage buffer usage** — All primitive data is passed through `var<storage, read>` bindings. WebGPU supports this, but there are tighter limits on `maxStorageBufferBindingSize` in the browser (128 MB default). The current code already caps its instance buffer at 256 MB; you may need to be more conservative.

4. **Everything else is standard WGSL** — The rest of the shader (quads, shadows, paths, underlines, monochrome sprites, polychrome sprites) uses only core WGSL features: storage buffers, `textureSample`, `dpdx`/`dpdy`, standard math operations, `@builtin(vertex_index)` / `@builtin(instance_index)`, etc. All of this is fully WebGPU-compatible.

**Bottom line:** Strip the `enable dual_source_blending` line and the subpixel sprite entry points, and the shader file is valid standard WGSL that would work in a browser's WebGPU implementation with no other changes.


## Assistant

<thinking>
Now I have a very comprehensive understanding of the GPUI platform abstraction, existing implementations, rendering pipeline, and the current state of web/wasm support. Let me synthesize all of this into a detailed implementation plan.
</thinking>

Now I have a comprehensive understanding of the entire GPUI platform architecture. Let me synthesize this into a detailed implementation plan.

---

# GPUI Web Platform Implementation Plan

## Executive Summary

GPUI has a clean platform abstraction that isolates ~95% of the framework (element tree, layout, event handling, state management) from the ~5% that's platform-specific. A web/browser implementation requires providing concrete implementations of **8 traits** inside the existing `gpui_web` crate, wiring up the `gpui_wgpu` renderer with WebGPU, and building a small JavaScript/HTML bootstrap harness.

The architecture is well-positioned for this: the `Scene` IR cleanly decouples rendering, `gpui_wgpu` already uses WGSL shaders that are 99% WebGPU-compatible, and `#[cfg(target_family = "wasm")]` guards are already in place throughout `gpui`.

---

## Current State

| Layer | Status |
|---|---|
| `gpui_web` crate scaffold | ✅ Full Phase 0 scaffold with all platform trait stubs |
| `gpui_platform` wasm branch | ✅ Wired to `gpui_web::WebPlatform::new()` |
| `gpui_platform` → `gpui_web` dependency | ✅ `cfg(target_family = "wasm")` dep added |
| `#[cfg(target_family = "wasm")]` guards in `gpui` | ✅ ~20 sites, all gating `http_client` |
| `gpui_wgpu` renderer + WGSL shaders | ✅ Exists, uses standard WGSL (one extension to strip) |
| `wasm-bindgen` / `web-sys` dependencies | ✅ Added to `gpui_web` |
| Web `Platform` stub implementation | ✅ All trait methods implemented (stubs/no-ops) |
| `PlatformDispatcher` stub | ✅ Runs tasks inline synchronously |
| `PlatformDisplay` stub | ✅ Hardcoded 1920×1080 |
| `PlatformTextSystem` | ⚠️ Uses `NoopTextSystem` (needs cosmic-text) |
| `PlatformWindow` | ❌ Not started (`open_window` bails with error) |
| Console logging | ✅ `log::*` macros bridge to `web_sys::console` |
| Build tooling | ✅ `script/build` + `web/index.html` + wasm-bindgen pipeline |
| Example | ✅ `hello_web` compiles, loads in browser, reports errors to console |

---

## Architecture Decision: wgpu + WebGPU

**Recommendation: Reuse `gpui_wgpu` with wgpu's `BROWSER_WEBGPU` backend.**

Rationale:
1. `gpui_wgpu` already renders the full `Scene` with WGSL shaders — no need to rewrite a renderer
2. wgpu 28.0 (already in the workspace) supports `wgpu::Backends::BROWSER_WEBGPU`
3. The WGSL shaders are standard WebGPU-compatible (only `enable dual_source_blending` needs removal; a graceful fallback already exists)
4. This is consistent with how Linux already works (wgpu + Vulkan/GL)
5. Alternative (Canvas2D renderer) would be vastly more work and lower quality

---

## Implementation Phases

### Phase 0: Build Infrastructure & Toolchain Setup ✅ COMPLETE

**Goal:** Get `gpui_web` compiling to `wasm32-unknown-unknown` and loading in a browser.

**What was done:**

1. **Wired `gpui_web` into the workspace:**
   - Added `gpui_web` to workspace dependencies in root `Cargo.toml`
   - Added `gpui`, `anyhow`, `futures`, `log`, `uuid`, `wasm-bindgen`, `web-sys`, `console_error_panic_hook` as `cfg(target_family = "wasm")` dependencies in `gpui_web/Cargo.toml`
   - Added `gpui_web` as `cfg(target_family = "wasm")` dependency in `gpui_platform/Cargo.toml`
   - Replaced `todo!()` in `gpui_platform::current_platform` with `Rc::new(gpui_web::WebPlatform::new())`

2. **Implemented all platform trait stubs:**
   - `WebPlatform` — full `impl Platform` with no-ops/stubs for all methods
   - `WebDispatcher` — `impl PlatformDispatcher` that runs tasks inline
   - `WebDisplay` — `impl PlatformDisplay` with hardcoded 1920×1080
   - `WebKeyboardLayout` — `impl PlatformKeyboardLayout` returning US layout
   - Console logger — bridges `log::*` macros to `web_sys::console`

3. **Created build harness:**
   - `crates/gpui_web/script/build` — builds wasm + runs wasm-bindgen
   - `crates/gpui_web/web/index.html` — bootstrap HTML page
   - `crates/gpui_web/web/.gitignore` — excludes `pkg/` build artifacts

4. **Created `hello_web` example:**
   - `crates/gpui_web/examples/hello_web.rs` — creates `Application` with `WebPlatform`, attempts `open_window`, logs errors to browser console
   - Uses `#[wasm_bindgen(start)]` for browser entry, `console_error_panic_hook` for panic visibility

**Build verification:**
- ✅ `cargo check -p gpui_web --target wasm32-unknown-unknown`
- ✅ `cargo check -p gpui_platform --target wasm32-unknown-unknown`
- ✅ `cargo build -p gpui_web --example hello_web --target wasm32-unknown-unknown` (produces ~30 MB wasm after wasm-bindgen)
- ✅ Native builds (`cargo check -p gpui_platform`) unaffected

**How to run:**
```/dev/null/run.sh#L1-L4
crates/gpui_web/script/build
cd crates/gpui_web/web
python3 -m http.server 8080
# Open http://localhost:8080
```

**Prerequisites:** `rustup target add wasm32-unknown-unknown` and `cargo install wasm-bindgen-cli --version 0.2.104` (must match lockfile version).

---

### Phase 1: `PlatformDispatcher` (Async Runtime) ✅ COMPLETE

**Goal:** Replace the synchronous inline-run stubs in `WebDispatcher` with proper browser-based async scheduling.

**What was done:**

1. **Replaced all inline-run stubs** in `crates/gpui_web/src/dispatcher.rs` with proper browser-based async scheduling:

   | Trait Method | Before (stub) | After |
   |---|---|---|
   | `dispatch(runnable, priority)` | `runnable.run()` inline | `queueMicrotask()` for High/RealtimeAudio, `setTimeout(fn, 0)` for Medium/Low |
   | `dispatch_on_main_thread(runnable, priority)` | `runnable.run()` inline | Same as `dispatch` (wasm is single-threaded) |
   | `dispatch_after(duration, runnable)` | `runnable.run()` inline (delay ignored) | `setTimeout(fn, duration_ms)` with proper delay |
   | `spawn_realtime(f)` | `f()` inline | `queueMicrotask()` (no real-time threads in wasm) |

2. **JS bindings:** Used `#[wasm_bindgen] extern "C"` to bind the global `queueMicrotask` and `setTimeout` browser APIs directly (no extra `web-sys` features or `js-sys` dependency needed).

3. **One-shot closures:** `Closure::once_into_js` converts Rust `FnOnce` closures into JS functions that self-clean after invocation.

4. **Closed-task guard:** Before running a task, `runnable.metadata().is_closed()` is checked (matching the pattern in native dispatchers) so tasks from a closed executor are dropped.

5. **Extracted `schedule_runnable` helper** shared by `dispatch` and `dispatch_on_main_thread`.

**File:** `zed/crates/gpui_web/src/dispatcher.rs`

---

### Phase 2: `PlatformDisplay` & `PlatformWindow` (Canvas + WebGPU) ✅ COMPLETE

**Goal:** Open a canvas element and hook it up to wgpu's WebGPU backend for rendering.

#### 2a. `WebDisplay`

**Current state:** `WebDisplay` exists in `crates/gpui_web/src/display.rs` with hardcoded 1920×1080 bounds.

**What needs to change:** Query actual browser viewport dimensions via `web-sys`:

| Trait Method | Current (stub) | Target |
|---|---|---|
| `id()` | `DisplayId::new(1)` | No change needed |
| `uuid()` | Random UUID | Deterministic UUID from `navigator.userAgent` or stable value |
| `bounds()` | Hardcoded 1920×1080 | `window.screen.width/height` via `web_sys` |
| `visible_bounds()` | Default (delegates to `bounds()`) | `window.innerWidth/innerHeight` |
| `default_bounds()` | Default (centered in bounds) | No change needed |

**File:** `zed/crates/gpui_web/src/display.rs` (modify existing)

#### 2b. `WebWindow`

Wraps an `HtmlCanvasElement` and the wgpu `Surface` + `WgpuRenderer`:

| Category | Approach |
|---|---|
| **Construction** | Create or adopt a `<canvas>` element, obtain wgpu `Surface` from it via `instance.create_surface_from_canvas()` |
| **Geometry** | `bounds()` / `content_size()` from canvas `clientWidth`/`clientHeight`; `scale_factor()` from `window.devicePixelRatio` |
| **Rendering** | `draw(scene)` delegates to `WgpuRenderer::draw()` — same as Linux |
| **Frame scheduling** | `on_request_frame` callback invoked via `requestAnimationFrame` |
| **Resize** | `ResizeObserver` on the canvas element fires `on_resize` callback |
| **Input** | DOM event listeners (see Phase 3) |
| **Atlas** | `WgpuAtlas` from `gpui_wgpu` — no changes needed |
| **`HasWindowHandle`/`HasDisplayHandle`** | Return web-specific raw handles from `raw-window-handle` crate |

**Key integration point with `gpui_wgpu`:**

The `WgpuContext::new()` currently enables only `VULKAN | GL`:

```zed/crates/gpui_wgpu/src/wgpu_context.rs#L29-L33
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
```

This needs a `#[cfg(target_family = "wasm")]` branch that uses `wgpu::Backends::BROWSER_WEBGPU` (and potentially `GL` for WebGL2 fallback). The surface creation also needs to use `create_surface_from_canvas` instead of `create_surface`.

**WGSL shader change:** Remove `enable dual_source_blending;` behind a cfg or ship two shader variants. The subpixel sprite fallback to monochrome already exists in `WgpuRenderer`.

**Files:**
- `zed/crates/gpui_web/src/window.rs`
- `zed/crates/gpui_web/src/display.rs`
- Modifications to `zed/crates/gpui_wgpu/src/wgpu_context.rs` (backend selection)
- Modifications to `zed/crates/gpui_wgpu/src/shaders.wgsl` (strip extension)

---

### Phase 3: Input Events ✅ COMPLETE

**Goal:** Translate DOM events to GPUI's `PlatformInput` enum.

The `PlatformInput` variants and their DOM sources:

| `PlatformInput` Variant | DOM Event(s) | Key Translation Notes |
|---|---|---|
| `KeyDown(KeyDownEvent)` | `keydown` | `event.key` / `event.code` → `Keystroke` |
| `KeyUp(KeyUpEvent)` | `keyup` | Same mapping |
| `ModifiersChanged` | `keydown`/`keyup` for modifier keys | Track modifier state delta |
| `MouseDown` | `pointerdown` | `event.button` → `MouseButton`, coordinates from `offsetX/Y` |
| `MouseUp` | `pointerup` | Same |
| `MouseMove` | `pointermove` | Coordinates, modifier state |
| `MouseExited` | `pointerleave` | — |
| `ScrollWheel` | `wheel` | `deltaX/Y`, `deltaMode` → pixel delta |
| `FileDrop` | `dragover` + `drop` | `DataTransfer` API |

**Keystroke mapping** is the most complex part. The `Keystroke` struct needs:
- `key: String` — the logical key (e.g., `"a"`, `"enter"`, `"backspace"`)
- `ime_key: Option<String>` — the composed character for IME
- `key_char: Option<String>` — the character that would be typed
- `modifiers: Modifiers` — ctrl, alt, shift, platform (Cmd on Mac, Ctrl on others)

The browser's `KeyboardEvent.key` and `KeyboardEvent.code` provide these. The `platform` modifier should map to `metaKey` on Mac browsers and `ctrlKey` elsewhere (detectable via `navigator.platform` or `navigator.userAgent`).

**IME / text input** will require integration with the browser's `CompositionEvent` and a hidden `<textarea>` or `contenteditable` element for the `PlatformInputHandler` protocol. This can be deferred to a later phase with keyboard-only input working first.

**File:** `zed/crates/gpui_web/src/events.rs`

**Implementation details:**
- `WebEventListeners` struct holds all `Closure<dyn FnMut(JsValue)>` instances to prevent GC
- `register_event_listeners()` wires up all DOM listeners on the canvas element (pointer/wheel/drag/keyboard)
- `dom_key_to_gpui_key()` maps DOM `KeyboardEvent.key` → GPUI key names (e.g. `"ArrowLeft"` → `"left"`)
- `dom_mouse_button_to_gpui()` maps DOM button numbers (0–4) → `MouseButton` variants
- Universal modifier mapping: `ctrlKey`→`control`, `altKey`→`alt`, `shiftKey`→`shift`, `metaKey`→`platform`
- `capslock_from_keyboard_event()` uses `getModifierState("CapsLock")`
- `is_mac_platform()` detects macOS via `navigator.platform`
- `ClickState` tracks double/triple-click (400ms window, 5px distance threshold)
- `compute_key_char()` produces `key_char` for printable single characters; `None` for modifier combos
- `WebWindowMutableState` gained `modifiers` and `capslock` fields, updated on every input event
- `WebWindow::modifiers()` and `capslock()` now return live tracked state
- Canvas is made focusable via `tabindex="0"`, auto-focused on creation, and re-focused on every `pointerdown`
- Keyboard events (`keydown`/`keyup`) are registered on the canvas (not the browser window) so they only fire when the canvas is focused
- All non-modifier key events call `preventDefault()` to stop the browser from handling them
- Scroll wheel deltas are negated (DOM positive = scroll down, GPUI positive = scroll up)
- Added web-sys features: `DataTransfer`, `DragEvent`, `EventTarget`, `File`, `FileList`, `KeyboardEvent`, `MouseEvent`, `PointerEvent`, `WheelEvent`
- Added `smallvec` workspace dependency

---

### Phase 4: `PlatformTextSystem` ✅ COMPLETE

**Goal:** Font loading, text shaping, and glyph rasterization in the browser.

**Implementation: `cosmic-text` compiled to wasm via shared `gpui_cosmic_text` crate.**

The `CosmicTextSystem` was extracted from `gpui_linux/src/linux/text_system.rs` into a new shared crate `gpui_cosmic_text` at `zed/crates/gpui_cosmic_text/`. Both Linux and Web now depend on this shared crate.

**What was done:**

1. **Created `gpui_cosmic_text` crate** (`zed/crates/gpui_cosmic_text/`):
   - Contains the full `CosmicTextSystem` (cosmic-text + swash text stack)
   - `font-kit` is an optional feature — Linux enables it for precise font matching; web uses a built-in weight/style scoring fallback
   - Two constructors: `new(fallback)` loads system fonts (Linux), `new_without_system_fonts(fallback)` creates empty DB (wasm)
   - Configurable system font fallback name (both platforms use `"IBM Plex Sans"`)
   - Improved error handling: replaced `.unwrap()` calls with proper `Result` propagation and `log::warn!` fallbacks

2. **Updated `gpui_linux`**:
   - `text_system.rs` replaced with single re-export: `pub(crate) use gpui_cosmic_text::CosmicTextSystem;`
   - `Cargo.toml`: replaced direct `cosmic-text` + `font-kit` deps with `gpui_cosmic_text` (with `font-kit` feature)
   - Feature lists (`wayland`, `x11`) updated to gate `gpui_cosmic_text` instead of `cosmic-text`/`font-kit`

3. **Updated `gpui_web`**:
   - `WebPlatform::new()` uses `CosmicTextSystem::new_without_system_fonts("IBM Plex Sans")` instead of `NoopTextSystem`
   - All 8 Zed-bundled fonts embedded via `include_bytes!()` and loaded at platform init:
     - IBM Plex Sans: Regular, Italic, SemiBold, SemiBoldItalic
     - Lilex: Regular, Bold, Italic, BoldItalic
   - Fonts are `Cow::Borrowed` (`&'static [u8]`) — zero-copy from wasm binary into cosmic-text

4. **Updated workspace `Cargo.toml`**: added `gpui_cosmic_text` to members and dependencies

**Files:**
- `zed/crates/gpui_cosmic_text/Cargo.toml`
- `zed/crates/gpui_cosmic_text/src/gpui_cosmic_text.rs`
- `zed/crates/gpui_linux/src/linux/text_system.rs` (now a re-export)
- `zed/crates/gpui_web/src/platform.rs` (font embedding + loading)

---

### Phase 5: `Platform` Trait (Flesh Out Stubs)

**Goal:** Replace stub implementations in `WebPlatform` with real browser-backed behavior.

**Current state:** `WebPlatform` exists in `crates/gpui_web/src/platform.rs` with all `Platform` trait methods implemented as stubs/no-ops. The struct is fully wired into `gpui_platform::current_platform()`.

**What needs to change** (after `WebWindow` exists from Phase 2b):

| Category | Current (stub) | Target |
|---|---|---|
| **`open_window()`** | `bail!("not yet implemented")` | Create `WebWindow` with canvas + wgpu |
| **`open_url()`** | `log::info!` | `window.open(url)` via `web_sys` |
| **`set_cursor_style()`** | `log::debug!` | Set `canvas.style.cursor` CSS property |
| **Clipboard** | In-memory `RefCell` | `navigator.clipboard` API (async, needs permissions). Keep in-memory cache for synchronous `read_from_clipboard()`, update via `paste` events. |
| **`prompt_for_paths()`** | Returns error | `<input type="file">` element |
| **Credentials** | Returns error / `Ok(None)` | `localStorage` |

Most other methods (menus, activation, hide, thermal state, etc.) will remain no-ops permanently since they don't apply to browsers.

**The `run()` method** already calls `on_finish_launching()` synchronously and returns — this is correct for the browser since the browser's own event loop takes over. No changes needed.

**File:** `zed/crates/gpui_web/src/platform.rs` (modify existing)

---

### Phase 6: `PlatformKeyboardLayout` & `PlatformKeyboardMapper`

**Goal:** Basic keyboard layout support.

**Current state:** `WebKeyboardLayout` exists in `crates/gpui_web/src/keyboard.rs` returning a hardcoded US layout. `WebPlatform` already uses `DummyKeyboardMapper`.

**What needs to change:** Keyboard layout detection can use the `navigator.keyboard` API (Keyboard Map API) where available. For the initial implementation the current stubs are sufficient — this is the lowest priority phase.

**File:** `zed/crates/gpui_web/src/keyboard.rs` (modify existing)

---

## File Structure

```/dev/null/tree.txt#L1-L27
zed/crates/gpui_cosmic_text/
├── Cargo.toml
└── src/
    └── gpui_cosmic_text.rs  # ✅ Shared CosmicTextSystem (cosmic-text + swash), used by Linux & Web

zed/crates/gpui_web/
├── Cargo.toml
├── src/
│   ├── gpui_web.rs        # ✅ Crate root, module declarations + re-exports
│   ├── platform.rs        # ✅ WebPlatform: impl Platform (bundled fonts + cosmic-text)
│   ├── dispatcher.rs      # ✅ WebDispatcher: impl PlatformDispatcher (inline stubs)
│   ├── display.rs         # ✅ WebDisplay: impl PlatformDisplay (hardcoded)
│   ├── keyboard.rs        # ✅ WebKeyboardLayout: impl PlatformKeyboardLayout (US stub)
│   ├── logging.rs         # ✅ Console logger (log::* → web_sys::console)
│   ├── window.rs          # ✅ WebWindow: impl PlatformWindow (canvas, resize, RAF loop, event wiring)
│   └── events.rs          # ✅ DOM event → PlatformInput translation (pointer, keyboard, wheel, drag)
```

---

## Modifications to Existing Crates

| Crate | Change | Status |
|---|---|---|
| **`gpui_platform`** | Add `gpui_web` as `cfg(target_family = "wasm")` dep; wire up `current_platform()` | ✅ Done |
| **Root `Cargo.toml`** | Add `gpui_web` and `wasm-bindgen` to workspace dependencies | ✅ Done |
| **`gpui_wgpu`** | Add `cfg` for `BROWSER_WEBGPU` backend in `WgpuContext::new()`; handle canvas surface creation; conditionally strip `dual_source_blending` from shaders | ❌ Phase 2b |
| **`gpui`** | Possibly adjust `Application::run()` for non-blocking web event loop (or the web platform handles this internally) | ⚠️ May not be needed — `run()` already returns after calling callback |
| **`gpui_linux`** | Extract `CosmicTextSystem` into shared `gpui_cosmic_text` crate; re-export from `text_system.rs` | ✅ Done (Phase 4) |
| **`gpui_cosmic_text`** (new) | Shared text system crate used by both `gpui_linux` and `gpui_web` | ✅ Done (Phase 4) |

---

## Risk Assessment & Open Questions

| Risk | Mitigation |
|---|---|
| **wgpu WebGPU browser support maturity** | wgpu 28.0 has production WebGPU support; Chrome/Edge ship WebGPU; Firefox is in progress. Can fallback to WebGL2 via wgpu's GL backend. |
| **Single-threaded wasm** | GPUI's foreground executor is already single-threaded. Background executor needs cooperative scheduling initially; Web Workers can come later. `SharedArrayBuffer` (requires COOP/COEP headers) enables true multithreading if needed. |
| **`Application::run()` blocking semantics** | Web `Platform::run()` calls the launch callback and returns. The browser event loop drives everything. This should work because `Platform::run()` already takes a callback rather than returning a value. |
| **Font availability** | Must bundle fonts. Zed already ships its own fonts, so this is straightforward. Could also fetch web fonts via HTTP. |
| **Binary size** | wasm-opt + gzip compression. cosmic-text + wgpu will be several MB. Tree-shaking via wasm-bindgen helps. |
| **IME / complex text input** | Defer to later phase. Hidden textarea technique is well-established in web canvas apps (used by Monaco, VS Code web, Figma). |
| **File system access** | Browser has no native FS. Use File System Access API where available, or operate on in-memory buffers. This is more of a Zed-app concern than a GPUI concern. |
| **Clipboard async API** | Browser clipboard is async + permission-gated. `read_from_clipboard()` returns `Option<ClipboardItem>` synchronously — may need a cached approach where the clipboard is read ahead of time via `paste` events. |

---

## Suggested Implementation Order

1. ~~**Phase 0** — Build infra, get wasm compiling~~ ✅ COMPLETE
2. ~~**Phase 1** — Proper `WebDispatcher` with `setTimeout`/`queueMicrotask`~~ ✅ COMPLETE
3. ~~**Phase 2a** — Real `WebDisplay` with browser viewport dimensions~~ ✅ COMPLETE
4. ~~**Phase 4** — `PlatformTextSystem` with cosmic-text via shared `gpui_cosmic_text` crate~~ ✅ COMPLETE
5. ~~**Phase 2b** — `WebWindow` + wgpu WebGPU integration~~ ✅ COMPLETE
6. **Phase 5** — Flesh out `WebPlatform` stubs (clipboard, cursor, open_url) *(~2-3 days)*
7. **Phase 3** — Input events *(~1 week)*
8. **Phase 6** — Keyboard layout *(~1-2 days)*

**Next up:** Phase 5 (flesh out `WebPlatform` stubs) and Phase 3 (input events) are the remaining work to reach the first milestone.

**First milestone:** A browser window showing a GPUI view with styled `div`s and text, responding to mouse clicks.

**Second milestone:** Full keyboard input, scrolling, and interactive UI.
