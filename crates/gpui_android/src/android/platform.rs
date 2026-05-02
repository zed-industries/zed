use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use android_activity::{AndroidApp, MainEvent, PollEvent};
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DevicePixels,
    DummyKeyboardMapper, ForegroundExecutor, Keymap, Menu, MenuItem, OwnedMenu, PathPromptOptions,
    Platform, PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, RequestFrameOptions, Size, Task, ThermalState, WindowAppearance, WindowParams,
};
use gpui_wgpu::GpuContext;

use super::{
    AndroidDispatcher, AndroidDisplay, AndroidKeyboardLayout, AndroidWindow, MainThreadMailbox,
    android_app, clipboard, input, intents, keystore,
};

#[derive(Default)]
struct PlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_state_change: Option<Box<dyn FnMut()>>,
}

/// GPUI [`Platform`] implementation for Android.
///
/// `AndroidPlatform` is the type-erased return value of
/// [`current_platform`](super::current_platform). It owns:
///
/// - A two-thread executor pair ([`BackgroundExecutor`] backed by an
///   [`AndroidDispatcher`] thread pool, [`ForegroundExecutor`] backed by a
///   [`MainThreadMailbox`] drained from the activity's poll loop).
/// - A `cosmic-text`-backed [`PlatformTextSystem`] pre-loaded with every
///   font from `/system/fonts`, `/data/fonts` and `/product/fonts`.
/// - The GPU context shared by every window's [`gpui_wgpu::WgpuRenderer`].
/// - A live handle to the (currently single) GPUI window.
///
/// # Lifecycle
///
/// [`Platform::run`] takes ownership of the calling thread:
///
/// 1. Waits for `MainEvent::InitWindow` (so the user's `cx.open_window` call
///    sees a real `NativeWindow`).
/// 2. Invokes the user's launch closure.
/// 3. Enters the activity poll loop, dispatching input events / lifecycle
///    events / mailbox runnables until `MainEvent::Destroy` arrives or
///    [`Platform::quit`] is called.
///
/// # Singleton-by-convention
///
/// `AndroidPlatform` doesn't enforce uniqueness, but `android-activity` only
/// spawns one `android_main` thread per process and the Android system only
/// gives you one Activity at a time. Treat it as a singleton.
pub struct AndroidPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    main_mailbox: Arc<MainThreadMailbox>,
    active_display: Rc<AndroidDisplay>,
    active_window: RefCell<Option<AnyWindowHandle>>,
    /// Strong handle to the live window, kept here so the activity-event
    /// pump can route lifecycle/input events without going through the
    /// trait-object indirection.
    window: RefCell<Option<Rc<AndroidWindow>>>,
    callbacks: RefCell<PlatformCallbacks>,
    menus: RefCell<Vec<OwnedMenu>>,
    gpu_context: GpuContext,
    headless: bool,
}

impl AndroidPlatform {
    /// Allocate executors, a text system populated from `/system/fonts`, a
    /// GPU context and the placeholder display. Cheap — no Vulkan device is
    /// created until a window is opened.
    ///
    /// Pass `headless = true` if you want a usable [`Platform`] without
    /// actually rendering (executors + text system still work). The Android
    /// port has no separate headless backend; `headless` just gates
    /// [`Platform::open_window`] from creating a wgpu surface.
    ///
    /// You almost never construct this directly — call
    /// [`gpui_platform::application`] (or [`current_platform`](super::current_platform))
    /// instead.
    ///
    /// [`gpui_platform::application`]: https://docs.rs/gpui_platform/latest/gpui_platform/fn.application.html
    pub fn new(headless: bool) -> Self {
        let (dispatcher, main_mailbox) = AndroidDispatcher::new();
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        // Roboto is bundled with every Android system image. We use
        // `new_without_system_fonts` because `fontdb` 0.23's
        // `load_system_fonts()` has no Android branch — its cfg gates skip
        // `target_os = "android"` entirely, leaving the database empty.
        // Below we load `/system/fonts/` manually.
        let text_system_concrete =
            gpui_wgpu::CosmicTextSystem::new_without_system_fonts("Roboto");
        load_android_system_fonts(&text_system_concrete);
        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(text_system_concrete);

        let active_display = Rc::new(AndroidDisplay::new());

        Self {
            background_executor,
            foreground_executor,
            text_system,
            main_mailbox,
            active_display,
            active_window: RefCell::new(None),
            window: RefCell::new(None),
            callbacks: RefCell::new(PlatformCallbacks::default()),
            menus: RefCell::new(Vec::new()),
            gpu_context: Rc::new(std::cell::RefCell::new(None)),
            headless,
        }
    }

    fn handle_main_event(&self, app: &AndroidApp, event: MainEvent<'_>) {
        match event {
            MainEvent::InitWindow { .. } | MainEvent::WindowResized { .. } => {
                if let Some(native_window) = app.native_window() {
                    let physical_size = Size {
                        width: DevicePixels(native_window.width()),
                        height: DevicePixels(native_window.height()),
                    };
                    let scale_factor = scale_factor_from_app(app);
                    self.active_display
                        .set_bounds(logical_bounds(physical_size, scale_factor));
                    if let Some(window) = self.window.borrow().as_ref() {
                        match event {
                            MainEvent::InitWindow { .. } => {
                                if let Err(error) =
                                    window.attach_surface(native_window, physical_size)
                                {
                                    log::error!("Failed to attach Android surface: {error:#}");
                                }
                            }
                            MainEvent::WindowResized { .. } => {
                                window.update_size(physical_size, scale_factor);
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            MainEvent::TerminateWindow { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.detach_surface();
                }
            }
            MainEvent::GainedFocus | MainEvent::Resume { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.dispatch_active_status(true);
                }
            }
            MainEvent::LostFocus | MainEvent::Pause => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.dispatch_active_status(false);
                }
            }
            MainEvent::ConfigChanged { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.set_appearance(appearance_from_app(app));
                }
            }
            MainEvent::RedrawNeeded { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.dispatch_request_frame(RequestFrameOptions::default());
                }
            }
            MainEvent::ContentRectChanged { .. } | MainEvent::InsetsChanged { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    let rect = app.content_rect();
                    window.update_content_rect(rect, scale_factor_from_app(app));
                }
            }
            _ => {}
        }
    }

    fn pump_input(&self, app: &AndroidApp) {
        let scale_factor = self
            .window
            .borrow()
            .as_ref()
            .map(|w| w.scale_factor())
            .unwrap_or(1.0);

        // android-activity's input iterator hands us events with a
        // `&InputEvent`; we translate and dispatch each one synchronously.
        let mut iter = match app.input_events_iter() {
            Ok(iter) => iter,
            Err(error) => {
                log::warn!("input_events_iter failed: {error:?}");
                return;
            }
        };

        loop {
            let read_event = iter.next(|event| {
                match input::translate(event, scale_factor) {
                    input::Translated::Inputs(events) => {
                        for translated in events {
                            if let Some(window) = self.window.borrow().as_ref() {
                                window.dispatch_input(translated);
                            }
                        }
                    }
                    input::Translated::TextState(state) => {
                        if let Some(window) = self.window.borrow().as_ref() {
                            window.dispatch_text_event(state);
                        }
                    }
                    input::Translated::None => {}
                }
                android_activity::InputStatus::Handled
            });
            if !read_event {
                break;
            }
        }
    }
}

/// Convert physical window metrics + scale factor into a logical bounds tuple
/// for `PlatformDisplay`.
fn logical_bounds(
    physical: Size<DevicePixels>,
    scale_factor: f32,
) -> gpui::Bounds<gpui::Pixels> {
    use gpui::px;
    gpui::Bounds {
        origin: gpui::Point::default(),
        size: Size {
            width: px(physical.width.0 as f32 / scale_factor),
            height: px(physical.height.0 as f32 / scale_factor),
        },
    }
}

fn scale_factor_from_app(app: &AndroidApp) -> f32 {
    app.config()
        .density()
        .map(|d| d as f32 / 160.0)
        .unwrap_or(1.0)
}

fn appearance_from_app(app: &AndroidApp) -> WindowAppearance {
    // android-activity exposes the runtime ndk::Configuration; tracking
    // night-mode here lets us switch GPUI's appearance without going through
    // a full JNI round-trip on every frame.
    match app.config().ui_mode_night() {
        ndk::configuration::UiModeNight::Yes => WindowAppearance::Dark,
        _ => WindowAppearance::Light,
    }
}

impl Platform for AndroidPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        let Some(app) = android_app() else {
            log::warn!(
                "AndroidPlatform::run called without a registered AndroidApp; \
                 calling on_finish_launching directly and falling back to a \
                 mailbox-only loop"
            );
            on_finish_launching();
            while self.main_mailbox.drain_blocking() {}
            return;
        };

        // Hold on to `on_finish_launching` until a `NativeWindow` is alive.
        // GPUI's `Application::run -> on_finish_launching -> cx.open_window`
        // chain assumes the window's `sprite_atlas` and renderer are usable
        // immediately, which on Android is only true once
        // `MainEvent::InitWindow` has been delivered.
        let mut on_finish_launching: Option<Box<dyn 'static + FnOnce()>> =
            Some(on_finish_launching);
        let mut launched = false;

        loop {
            // Drain anything queued via dispatch_on_main_thread before yielding
            // to android-activity, so user code that wraps blocking work in
            // `cx.spawn(...)` makes progress promptly.
            self.main_mailbox.drain();

            let mut should_quit = false;
            app.poll_events(Some(Duration::from_millis(16)), |event| match event {
                PollEvent::Wake => {}
                PollEvent::Timeout => {}
                PollEvent::Main(main_event) => {
                    if matches!(main_event, MainEvent::Destroy) {
                        should_quit = true;
                    } else if matches!(main_event, MainEvent::InputAvailable) {
                        self.pump_input(&app);
                    } else {
                        self.handle_main_event(&app, main_event);
                    }
                }
                _ => {}
            });

            // Surface is alive — safe to run the user's launch closure now.
            if !launched && app.native_window().is_some() {
                if let Some(callback) = on_finish_launching.take() {
                    callback();
                }
                launched = true;
            }

            // Drive `request_frame` continuously so GPUI's internal dirty
            // tracker can re-render after `cx.notify()`. GPUI is smart
            // enough to skip the actual draw when nothing has changed, so
            // calling this every ~16ms is the same shape every other GPUI
            // backend uses (`requestAnimationFrame` on web, surface frame
            // callbacks on wayland) — without it, click handlers fire but
            // the screen never updates, which looks for all the world like
            // touch is broken.
            if launched {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.dispatch_request_frame(RequestFrameOptions::default());
                }
            }

            if should_quit || self.main_mailbox.is_stopped() {
                break;
            }
        }

        if let Some(mut quit) = self.callbacks.borrow_mut().quit.take() {
            quit();
        }
    }

    fn quit(&self) {
        self.main_mailbox.signal_stop();
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        log::warn!("AndroidPlatform::restart is a no-op on Android");
    }

    fn activate(&self, _ignoring_other_apps: bool) {}
    fn hide(&self) {}
    fn hide_other_apps(&self) {}
    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.active_display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.active_display.clone())
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        *self.active_window.borrow()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        if self.headless {
            anyhow::bail!("AndroidPlatform::open_window: cannot open a window in headless mode");
        }
        let app = android_app().context_for(
            "AndroidPlatform::open_window: no AndroidApp registered \
             (did you forget to call gpui_android::set_android_app from android_main?)",
        )?;

        let scale_factor = scale_factor_from_app(&app);
        let window = Rc::new(AndroidWindow::new(
            handle,
            params,
            self.active_display.clone(),
            scale_factor,
            self.gpu_context.clone(),
        ));

        // If a surface is already alive (e.g. open_window called after the
        // first MainEvent::InitWindow), wire it up immediately.
        if let Some(native_window) = app.native_window() {
            let physical_size = Size {
                width: DevicePixels(native_window.width()),
                height: DevicePixels(native_window.height()),
            };
            self.active_display
                .set_bounds(logical_bounds(physical_size, scale_factor));
            window
                .attach_surface(native_window, physical_size)
                .map_err(|error| {
                    anyhow::anyhow!("failed to attach initial Android surface: {error:#}")
                })?;
        }

        *self.window.borrow_mut() = Some(window.clone());
        *self.active_window.borrow_mut() = Some(handle);
        Ok(Box::new(AndroidWindowHandle(window)))
    }

    fn window_appearance(&self) -> WindowAppearance {
        android_app()
            .map(|app| appearance_from_app(&app))
            .unwrap_or(WindowAppearance::Light)
    }

    fn open_url(&self, url: &str) {
        if let Some(app) = android_app() {
            intents::open_url(&app, url);
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.callbacks.borrow_mut().open_urls = Some(callback);
    }

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "register_url_scheme is not implemented on Android (declare schemes in AndroidManifest.xml)"
        )))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Err(anyhow::anyhow!(
            "prompt_for_paths is not implemented on Android (Storage Access Framework integration pending)"
        )));
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Err(anyhow::anyhow!(
            "prompt_for_new_path is not implemented on Android (Storage Access Framework integration pending)"
        )));
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, path: &Path) {
        if let Some(app) = android_app() {
            intents::reveal_path(&app, path);
        }
    }

    fn open_with_system(&self, path: &Path) {
        if let Some(app) = android_app() {
            intents::open_with_system(&app, path);
        }
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().reopen = Some(callback);
    }

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        *self.menus.borrow_mut() = menus.into_iter().map(|menu| menu.owned()).collect();
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        Some(self.menus.borrow().clone())
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.callbacks.borrow_mut().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.callbacks.borrow_mut().validate_app_menu_command = Some(callback);
    }

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().thermal_state_change = Some(callback);
    }

    fn compositor_name(&self) -> &'static str {
        "android"
    }

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        let app = android_app().ok_or_else(|| anyhow::anyhow!("AndroidApp not registered"))?;
        let dir = native_library_dir(&app)?;
        let candidate = std::path::Path::new(&dir).join(format!("lib{name}.so"));
        if candidate.exists() {
            Ok(candidate)
        } else {
            Err(anyhow::anyhow!(
                "auxiliary executable {name} not found in nativeLibraryDir ({dir})"
            ))
        }
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let app = android_app()?;
        clipboard::read(&app)
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        if let Some(app) = android_app() {
            clipboard::write(&app, item);
        }
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_owned();
        let username = username.to_owned();
        let password = password.to_vec();
        self.background_executor.spawn(async move {
            let app = android_app()
                .ok_or_else(|| anyhow::anyhow!("AndroidApp not registered"))?;
            keystore::write(&app, &url, &username, &password)
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let url = url.to_owned();
        self.background_executor.spawn(async move {
            let Some(app) = android_app() else {
                return Ok(None);
            };
            keystore::read(&app, &url)
        })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let url = url.to_owned();
        self.background_executor.spawn(async move {
            let app = android_app()
                .ok_or_else(|| anyhow::anyhow!("AndroidApp not registered"))?;
            keystore::delete(&app, &url)
        })
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(AndroidKeyboardLayout::new())
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().keyboard_layout_change = Some(callback);
    }
}

/// Adapter so we can return a `Box<dyn PlatformWindow>` from `open_window`
/// while the platform itself keeps an `Rc<AndroidWindow>` for event routing.
struct AndroidWindowHandle(Rc<AndroidWindow>);

impl raw_window_handle::HasWindowHandle for AndroidWindowHandle {
    fn window_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError>
    {
        self.0.window_handle()
    }
}

impl raw_window_handle::HasDisplayHandle for AndroidWindowHandle {
    fn display_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError>
    {
        self.0.display_handle()
    }
}

impl PlatformWindow for AndroidWindowHandle {
    fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.0.bounds()
    }
    fn is_maximized(&self) -> bool {
        self.0.is_maximized()
    }
    fn window_bounds(&self) -> gpui::WindowBounds {
        self.0.window_bounds()
    }
    fn content_size(&self) -> gpui::Size<gpui::Pixels> {
        self.0.content_size()
    }
    fn resize(&mut self, size: gpui::Size<gpui::Pixels>) {
        if let Some(window) = Rc::get_mut(&mut self.0) {
            window.resize(size);
        }
    }
    fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }
    fn appearance(&self) -> WindowAppearance {
        self.0.appearance()
    }
    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.0.display()
    }
    fn mouse_position(&self) -> gpui::Point<gpui::Pixels> {
        self.0.mouse_position()
    }
    fn modifiers(&self) -> gpui::Modifiers {
        self.0.modifiers()
    }
    fn capslock(&self) -> gpui::Capslock {
        self.0.capslock()
    }
    fn set_input_handler(&mut self, handler: gpui::PlatformInputHandler) {
        if let Some(window) = Rc::get_mut(&mut self.0) {
            window.set_input_handler(handler);
        }
    }
    fn take_input_handler(&mut self) -> Option<gpui::PlatformInputHandler> {
        Rc::get_mut(&mut self.0).and_then(|w| w.take_input_handler())
    }
    fn prompt(
        &self,
        level: gpui::PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[gpui::PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        self.0.prompt(level, msg, detail, answers)
    }
    fn activate(&self) {
        self.0.activate()
    }
    fn is_active(&self) -> bool {
        self.0.is_active()
    }
    fn is_hovered(&self) -> bool {
        self.0.is_hovered()
    }
    fn background_appearance(&self) -> gpui::WindowBackgroundAppearance {
        self.0.background_appearance()
    }
    fn set_title(&mut self, title: &str) {
        if let Some(window) = Rc::get_mut(&mut self.0) {
            window.set_title(title);
        }
    }
    fn set_background_appearance(&self, b: gpui::WindowBackgroundAppearance) {
        self.0.set_background_appearance(b)
    }
    fn minimize(&self) {
        self.0.minimize()
    }
    fn zoom(&self) {
        self.0.zoom()
    }
    fn toggle_fullscreen(&self) {
        self.0.toggle_fullscreen()
    }
    fn is_fullscreen(&self) -> bool {
        self.0.is_fullscreen()
    }
    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.0.on_request_frame(callback)
    }
    fn on_input(
        &self,
        callback: Box<dyn FnMut(gpui::PlatformInput) -> gpui::DispatchEventResult>,
    ) {
        self.0.on_input(callback)
    }
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.on_active_status_change(callback)
    }
    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.on_hover_status_change(callback)
    }
    fn on_resize(&self, callback: Box<dyn FnMut(gpui::Size<gpui::Pixels>, f32)>) {
        self.0.on_resize(callback)
    }
    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.on_moved(callback)
    }
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.on_should_close(callback)
    }
    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<gpui::WindowControlArea>>,
    ) {
        self.0.on_hit_test_window_control(callback)
    }
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.on_close(callback)
    }
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.on_appearance_changed(callback)
    }
    fn draw(&self, scene: &gpui::Scene) {
        self.0.draw(scene)
    }
    fn sprite_atlas(&self) -> Arc<dyn gpui::PlatformAtlas> {
        self.0.sprite_atlas()
    }
    fn is_subpixel_rendering_supported(&self) -> bool {
        self.0.is_subpixel_rendering_supported()
    }
    fn gpu_specs(&self) -> Option<gpui::GpuSpecs> {
        self.0.gpu_specs()
    }
    fn update_ime_position(&self, bounds: gpui::Bounds<gpui::Pixels>) {
        self.0.update_ime_position(bounds)
    }
    fn play_system_bell(&self) {
        self.0.play_system_bell()
    }
}

/// Walk Android's well-known font directories and feed every TrueType /
/// OpenType file we find into cosmic-text. Required because `fontdb` 0.23's
/// `load_system_fonts()` has no Android cfg branch — without this the
/// database is empty and even `Roboto` can't be resolved.
fn load_android_system_fonts(text_system: &gpui_wgpu::CosmicTextSystem) {
    use std::borrow::Cow;
    use gpui::PlatformTextSystem;

    const FONT_DIRS: &[&str] = &["/system/fonts", "/system/font", "/data/fonts", "/product/fonts"];

    let mut bytes = Vec::new();
    for dir in FONT_DIRS {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_ascii_lowercase());
            if !matches!(ext.as_deref(), Some("ttf") | Some("ttc") | Some("otf") | Some("otc")) {
                continue;
            }
            match std::fs::read(&path) {
                Ok(data) => bytes.push(Cow::Owned(data)),
                Err(error) => log::warn!(
                    "Failed to read system font {}: {error}",
                    path.display()
                ),
            }
        }
    }

    if bytes.is_empty() {
        log::warn!(
            "No fonts found in {}; text rendering will fail until fonts are added manually",
            FONT_DIRS.join(", ")
        );
        return;
    }
    let count = bytes.len();
    if let Err(error) = text_system.add_fonts(bytes) {
        log::error!("Failed to register Android system fonts: {error:#}");
    } else {
        log::info!("Loaded {count} Android system fonts");
    }
}

/// `Context.getApplicationInfo().nativeLibraryDir` — used to resolve auxiliary
/// `.so` plugins.
fn native_library_dir(app: &AndroidApp) -> Result<String> {
    use jni::{jni_sig, jni_str, objects::JValue};
    use super::jni_glue::{java_string_to_rust, with_activity};
    with_activity(app, |env, activity| {
        let info = env
            .call_method(
                activity,
                jni_str!("getApplicationInfo"),
                jni_sig!(() -> "android.content.pm.ApplicationInfo"),
                &[],
            )?
            .l()?;
        let dir_obj = env
            .get_field(
                &info,
                jni_str!("nativeLibraryDir"),
                jni_sig!("java.lang.String"),
            )?
            .l()?;
        let _ = JValue::Bool;
        java_string_to_rust(env, dir_obj)
    })
}

/// Helper that turns `Option<T>` into `anyhow::Result<T>` with a fixed message
/// without dragging the full `anyhow::Context` extension into scope.
trait ContextFor<T> {
    fn context_for(self, msg: &'static str) -> anyhow::Result<T>;
}

impl<T> ContextFor<T> for Option<T> {
    fn context_for(self, msg: &'static str) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!(msg))
    }
}
