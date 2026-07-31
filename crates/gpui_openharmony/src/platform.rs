use crate::{
    display::OpenHarmonyDisplay,
    dispatcher::OpenHarmonyDispatcher,
    events::{OpenHarmonyHostEvent, OpenHarmonyKeyEvent, OpenHarmonyTouchEvent, SurfaceInfo},
    keyboard::{OpenHarmonyKeyboardLayout, OpenHarmonyKeyboardMapper},
    text_system::text_system,
    window::{OpenHarmonyWindow, OpenHarmonyWindowHandle},
};
use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, AppLifecyclePhase, BackgroundExecutor, Bounds, ClipboardItem, CursorStyle,
    DisplayId, ForegroundExecutor, Keymap, Menu, MenuItem, OwnedMenu, PathPromptOptions, Platform,
    PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, ScreenCaptureSource, SystemNotification, SystemNotificationResponse,
    Task, ThermalState, WindowAppearance, WindowButtonLayout, WindowParams, point, px,
};
use gpui_wgpu::GpuContext;
use smallvec::SmallVec;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

/// The GPUI platform implementation for OpenHarmony.
///
/// This is designed to be driven by an OpenHarmony host. The host is
/// responsible for creating and managing the XComponent surface, converting
/// native events into [`OpenHarmonyHostEvent`] values, and calling
/// [`OpenHarmonyPlatform::process_events`] regularly (for example, on each
/// VSync or XComponent callback).
pub struct OpenHarmonyPlatform {
    dispatcher: Arc<OpenHarmonyDispatcher>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    display: RefCell<Option<Rc<OpenHarmonyDisplay>>>,
    windows: RefCell<HashMap<AnyWindowHandle, Rc<OpenHarmonyWindow>>>,
    active_window: RefCell<Option<AnyWindowHandle>>,
    surface_info: RefCell<Option<SurfaceInfo>>,
    on_finish_launching: RefCell<Option<Box<dyn FnOnce() + 'static>>>,
    appearance: Cell<WindowAppearance>,
    appearance_override: Cell<Option<WindowAppearance>>,
    on_app_lifecycle: RefCell<Option<Box<dyn FnMut(AppLifecyclePhase)>>>,
    on_memory_warning: RefCell<Option<Box<dyn FnMut()>>>,
    on_quit: RefCell<Option<Box<dyn FnMut()>>>,
    on_reopen: RefCell<Option<Box<dyn FnMut()>>>,
    on_system_wake: RefCell<Option<Box<dyn FnMut()>>>,
    on_thermal_state_change: RefCell<Option<Box<dyn FnMut()>>>,
    on_keyboard_layout_change: RefCell<Option<Box<dyn FnMut()>>>,
    needs_process: Arc<AtomicBool>,
    gpu_context: GpuContext,
}

impl OpenHarmonyPlatform {
    /// Creates a new OpenHarmony platform.
    ///
    /// The returned `Rc` can be passed to [`gpui::App::new_inaccessible`] or
    /// used directly to receive surface and event callbacks from the host.
    pub fn new() -> Rc<Self> {
        let needs_process = Arc::new(AtomicBool::new(false));
        let wake = {
            let needs_process = needs_process.clone();
            Arc::new(move || {
                needs_process.store(true, Ordering::Release);
            }) as Arc<dyn Fn() + Send + Sync>
        };
        let dispatcher = Arc::new(OpenHarmonyDispatcher::new(wake));
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());

        Rc::new(Self {
            dispatcher,
            background_executor,
            foreground_executor,
            text_system: text_system(),
            display: RefCell::new(None),
            windows: RefCell::new(HashMap::default()),
            active_window: RefCell::new(None),
            surface_info: RefCell::new(None),
            on_finish_launching: RefCell::new(None),
            appearance: Cell::new(WindowAppearance::Light),
            appearance_override: Cell::new(None),
            on_app_lifecycle: RefCell::new(None),
            on_memory_warning: RefCell::new(None),
            on_quit: RefCell::new(None),
            on_reopen: RefCell::new(None),
            on_system_wake: RefCell::new(None),
            on_thermal_state_change: RefCell::new(None),
            on_keyboard_layout_change: RefCell::new(None),
            needs_process,
            gpu_context: Rc::new(RefCell::new(None)),
        })
    }

    /// Returns the platform's dispatcher.
    pub(crate) fn dispatcher(&self) -> Arc<OpenHarmonyDispatcher> {
        self.dispatcher.clone()
    }

    /// Notifies the platform that the host has created a surface.
    ///
    /// This should be called from the XComponent `OnSurfaceCreated` callback.
    pub fn surface_created(&self, info: SurfaceInfo) {
        let display = Rc::new(OpenHarmonyDisplay::new(
            DisplayId::new(0),
            info.size,
            info.scale_factor,
        ));
        self.display.replace(Some(display));
        self.surface_info.replace(Some(info));

        if let Some(callback) = self.on_finish_launching.borrow_mut().take() {
            self.foreground_executor
                .spawn(async move {
                    callback();
                })
                .detach();
        }
    }

    /// Notifies the platform that the host surface size or scale changed.
    pub fn surface_changed(&self, info: SurfaceInfo) {
        if let Some(display) = self.display.borrow().as_ref() {
            display.set_size(info.size);
            display.set_scale_factor(info.scale_factor);
        }

        if let Some(window) = self.windows.borrow().values().next() {
            window.set_size(info.size);
            window.set_scale_factor(info.scale_factor);
        }

        if let Some(surface) = self.surface_info.borrow_mut().as_mut() {
            surface.size = info.size;
            surface.scale_factor = info.scale_factor;
        }
    }

    /// Notifies the platform that the host surface is being destroyed.
    pub fn surface_destroyed(&self) {
        if let Some(window) = self.windows.borrow().values().next() {
            window.on_surface_destroyed();
        }
        self.surface_info.replace(None);
        self.display.replace(None);
    }

    /// Dispatches a host event into GPUI.
    pub fn dispatch_event(&self, event: OpenHarmonyHostEvent) {
        match event {
            OpenHarmonyHostEvent::SurfaceCreated(info) => self.surface_created(info),
            OpenHarmonyHostEvent::SurfaceChanged(info) => self.surface_changed(info),
            OpenHarmonyHostEvent::SurfaceDestroyed => self.surface_destroyed(),
            OpenHarmonyHostEvent::Touch(touch) => self.dispatch_touch(touch),
            OpenHarmonyHostEvent::KeyDown(key) => self.dispatch_key_down(key),
            OpenHarmonyHostEvent::KeyUp(key) => self.dispatch_key_up(key),
            OpenHarmonyHostEvent::Lifecycle(phase) => {
                if let Some(callback) = self.on_app_lifecycle.borrow_mut().as_mut() {
                    callback(phase);
                }
            }
            OpenHarmonyHostEvent::MemoryWarning => {
                if let Some(callback) = self.on_memory_warning.borrow_mut().as_mut() {
                    callback();
                }
            }
        }
    }

    fn dispatch_touch(&self, touch: OpenHarmonyTouchEvent) {
        if let Some(input) = OpenHarmonyHostEvent::Touch(touch.clone()).to_platform_input() {
            if let Some(window) = self.active_window() {
                window.dispatch_input(input);
                if let Some(mouse_input) = crate::events::touch_to_mouse(&touch) {
                    window.dispatch_input(mouse_input);
                }
            }
        }
    }

    fn dispatch_key_down(&self, key: OpenHarmonyKeyEvent) {
        if let Some(input) = OpenHarmonyHostEvent::KeyDown(key).to_platform_input() {
            if let Some(window) = self.active_window() {
                window.dispatch_input(input);
            }
        }
    }

    fn dispatch_key_up(&self, key: OpenHarmonyKeyEvent) {
        if let Some(input) = OpenHarmonyHostEvent::KeyUp(key).to_platform_input() {
            if let Some(window) = self.active_window() {
                window.dispatch_input(input);
            }
        }
    }

    /// Processes any pending main-thread work and clears the wake flag.
    ///
    /// The host should call this whenever the platform is woken (via the
    /// internal wake callback) or at the end of each frame.
    pub fn process_events(&self) {
        self.needs_process.store(false, Ordering::Release);
        self.dispatcher.process_main_thread_queue();
    }

    /// Returns whether the platform has pending main-thread work.
    pub fn needs_process(&self) -> bool {
        self.needs_process.load(Ordering::Acquire)
    }

    fn active_window(&self) -> Option<Rc<OpenHarmonyWindow>> {
        let active = *self.active_window.borrow();
        active.and_then(|handle| self.windows.borrow().get(&handle).cloned())
    }

    fn set_appearance_for_all_windows(&self, appearance: WindowAppearance) {
        for window in self.windows.borrow().values() {
            window.set_appearance(appearance);
        }
    }
}

impl Platform for OpenHarmonyPlatform {
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
        self.on_finish_launching.replace(Some(on_finish_launching));

        if self.surface_info.borrow().is_some() {
            if let Some(callback) = self.on_finish_launching.borrow_mut().take() {
                self.foreground_executor
                    .spawn(async move {
                        callback();
                    })
                    .detach();
            }
        }
    }

    fn quit(&self) {
        std::process::exit(0);
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        if let Some(display) = self.display.borrow().as_ref() {
            vec![display.clone() as Rc<dyn PlatformDisplay>]
        } else {
            Vec::new()
        }
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.display
            .borrow()
            .as_ref()
            .map(|display| display.clone() as Rc<dyn PlatformDisplay>)
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        *self.active_window.borrow()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        Some(self.windows.borrow().keys().copied().collect())
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let surface_info = self
            .surface_info
            .borrow()
            .as_ref()
            .ok_or_else(|| anyhow!("surface not created; call surface_created first"))?
            .clone();
        let display = self
            .display
            .borrow()
            .as_ref()
            .ok_or_else(|| anyhow!("display not created"))?
            .clone();

        let mut params = params;
        if f32::from(params.bounds.size.width) <= 0.0
            || f32::from(params.bounds.size.height) <= 0.0
        {
            params.bounds = Bounds::new(point(px(0.), px(0.)), surface_info.size);
        }

        let window = Rc::new(OpenHarmonyWindow::new(
            handle,
            params,
            &surface_info,
            display,
            self.gpu_context.clone(),
        )?);
        self.windows.borrow_mut().insert(handle, window.clone());
        self.active_window.replace(Some(handle));

        Ok(Box::new(OpenHarmonyWindowHandle(window)) as Box<dyn PlatformWindow>)
    }

    fn window_appearance(&self) -> WindowAppearance {
        self.appearance_override
            .get()
            .unwrap_or_else(|| self.appearance.get())
    }

    fn set_window_appearance(&self, appearance: Option<WindowAppearance>) {
        self.appearance_override.set(appearance);
        let appearance = self.window_appearance();
        self.set_appearance_for_all_windows(appearance);
    }

    fn button_layout(&self) -> Option<WindowButtonLayout> {
        None
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(None)).ok();
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(None)).ok();
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.on_quit.replace(Some(callback));
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.on_reopen.replace(Some(callback));
    }

    fn on_system_wake(&self, callback: Box<dyn FnMut()>) {
        self.on_system_wake.replace(Some(callback));
    }

    fn on_app_lifecycle(&self, callback: Box<dyn FnMut(AppLifecyclePhase)>) {
        self.on_app_lifecycle.replace(Some(callback));
    }

    fn on_memory_warning(&self, callback: Box<dyn FnMut()>) {
        self.on_memory_warning.replace(Some(callback));
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        None
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn perform_dock_menu_action(&self, _action: usize) {}

    fn add_recent_document(&self, _path: &Path) {}

    fn update_jump_list(
        &self,
        _menus: Vec<MenuItem>,
        _entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Task<Vec<SmallVec<[PathBuf; 2]>>> {
        Task::ready(Vec::new())
    }

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn gpui::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn gpui::Action) -> bool>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.on_thermal_state_change.replace(Some(callback));
    }

    fn set_app_identity(&self, _identifier: &str, _name: &str) {}

    fn show_system_notification(&self, _notification: SystemNotification) {}

    fn dismiss_system_notification(&self, _tag: &str) {}

    fn on_system_notification_response(
        &self,
        _callback: Box<dyn FnMut(SystemNotificationResponse)>,
    ) {
    }

    fn compositor_name(&self) -> &'static str {
        "wgpu"
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow!("app_path not supported on OpenHarmony"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow!(
            "path_for_auxiliary_executable not supported on OpenHarmony"
        ))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn hide_cursor_until_mouse_moves(&self) {}

    fn is_cursor_visible(&self) -> bool {
        true
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {}

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn read_from_primary(&self) -> Option<ClipboardItem> {
        None
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn write_to_primary(&self, _item: ClipboardItem) {}

    fn write_credentials(
        &self,
        _url: &str,
        _username: &str,
        _password: &[u8],
    ) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("write_credentials not supported on OpenHarmony")))
    }

    fn read_credentials(
        &self,
        _url: &str,
    ) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("delete_credentials not supported on OpenHarmony")))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(OpenHarmonyKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(OpenHarmonyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.on_keyboard_layout_change.replace(Some(callback));
    }

    fn is_screen_capture_supported(&self) -> bool {
        false
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(Vec::new())).ok();
        rx
    }
}

/// Returns the default OpenHarmony platform.
pub fn current_platform() -> Rc<dyn Platform> {
    OpenHarmonyPlatform::new()
}
