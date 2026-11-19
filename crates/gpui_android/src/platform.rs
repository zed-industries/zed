use crate::dispatcher::AndroidDispatcher;
use crate::display::AndroidDisplay;
use crate::events::{self, TouchIds};
use crate::insets;
use crate::keyboard::AndroidKeyboardLayout;
use crate::links;
use crate::vsync::Vsync;
use crate::window::{AndroidWindow, AndroidWindowInner};
use android_activity::input::KeyCharacterMap;
use android_activity::{AndroidApp, MainEvent, PollEvent};
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow,
    PriorityQueueReceiver, RunnableVariant, Task, ThermalState, WindowAppearance, WindowParams,
};
use gpui_wgpu::GpuContext;
use jni::objects::{JObject, JValue};
use jni::{jni_sig, jni_str};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, OnceLock};

static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

/// Stores the `AndroidApp` handed to `android_main` so that
/// `gpui_platform::current_platform` (which takes no arguments) can reach it.
/// Must be called before constructing the platform.
pub fn init(app: AndroidApp) {
    ANDROID_APP.set(app).ok();
}

pub struct AndroidPlatform {
    app: AndroidApp,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    main_receiver: PriorityQueueReceiver<RunnableVariant>,
    text_system: Arc<dyn PlatformTextSystem>,
    gpu_context: GpuContext,
    display: Rc<AndroidDisplay>,
    active_window: RefCell<Option<(AnyWindowHandle, Rc<AndroidWindowInner>)>>,
    callbacks: RefCell<AndroidPlatformCallbacks>,
    pending_launch: RefCell<Option<Box<dyn 'static + FnOnce()>>>,
    quit_requested: Cell<bool>,
    key_maps: RefCell<HashMap<i32, KeyCharacterMap>>,
    touch_ids: RefCell<TouchIds>,
    vsync: Vsync,
    /// A URL the app was opened with that nothing has observed yet.
    opened_url: Cell<Option<String>>,
}

#[derive(Default)]
struct AndroidPlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    app_lifecycle: Option<Box<dyn FnMut(gpui::AppLifecyclePhase)>>,
    quit: Option<Box<dyn FnMut() -> bool>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_state_change: Option<Box<dyn FnMut()>>,
}

impl AndroidPlatform {
    pub fn new(_headless: bool) -> Self {
        let app = ANDROID_APP
            .get()
            .expect("gpui_android::init(app) must be called from android_main before building the platform")
            .clone();
        insets::watch(&app);

        let (main_sender, main_receiver) = PriorityQueueReceiver::new();
        let dispatcher = Arc::new(AndroidDispatcher::new(main_sender, app.create_waker()));
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        let text_system = Arc::new(gpui_wgpu::CosmicTextSystem::new_without_system_fonts(
            "Roboto",
        ));
        if let Err(error) = text_system.add_fonts(system_fonts()) {
            log::error!("failed to load Android system fonts: {error:#}");
        }

        Self {
            background_executor,
            foreground_executor,
            main_receiver,
            text_system,
            gpu_context: GpuContext::default(),
            display: Rc::new(AndroidDisplay::new(&app)),
            active_window: RefCell::new(None),
            callbacks: RefCell::new(AndroidPlatformCallbacks::default()),
            pending_launch: RefCell::new(None),
            quit_requested: Cell::new(false),
            key_maps: RefCell::new(HashMap::new()),
            touch_ids: RefCell::new(TouchIds::default()),
            vsync: Vsync::new(app.create_waker()),
            opened_url: Cell::new(None),
            app,
        }
    }

    fn window(&self) -> Option<Rc<AndroidWindowInner>> {
        self.active_window
            .borrow()
            .as_ref()
            .map(|(_, inner)| Rc::clone(inner))
    }

    fn handle_main_event(&self, event: MainEvent<'_>) {
        match event {
            MainEvent::InitWindow { .. } => {
                if let Some(window) = self.window() {
                    window.handle_surface_created();
                }
                let launch = self.pending_launch.borrow_mut().take();
                if let Some(launch) = launch {
                    launch();
                }
            }
            MainEvent::TerminateWindow { .. } => {
                if let Some(window) = self.window() {
                    window.handle_surface_destroyed();
                }
            }
            MainEvent::WindowResized { .. } | MainEvent::ContentRectChanged { .. } => {
                log::info!(
                    "window/content-rect change: content_rect={:?}",
                    self.app.content_rect()
                );
                if let Some(window) = self.window() {
                    window.update_size();
                }
            }
            MainEvent::InsetsChanged { .. } => {
                log::info!("insets changed: content_rect={:?}", self.app.content_rect());
            }
            MainEvent::ConfigChanged { .. } => {
                self.display.update_max_refresh_rate(&self.app);
                if let Some(window) = self.window() {
                    window.update_size();
                    window.set_appearance(self.window_appearance());
                }
            }
            MainEvent::RedrawNeeded { .. } => {
                if let Some(window) = self.window() {
                    window.request_frame(true);
                }
            }
            MainEvent::InputAvailable => self.process_input(),
            MainEvent::GainedFocus => {
                if let Some(window) = self.window() {
                    window.set_active(true);
                }
            }
            MainEvent::LostFocus => {
                if let Some(window) = self.window() {
                    window.set_active(false);
                }
            }
            MainEvent::Destroy => self.quit_requested.set(true),
            MainEvent::Start => self.notify_app_lifecycle(gpui::AppLifecyclePhase::Foreground),
            MainEvent::Resume { .. } => {
                self.notify_app_lifecycle(gpui::AppLifecyclePhase::Active);
                // Both launching the app with a link and opening one while it
                // runs are followed by a resume.
                if let Some(url) = links::take_opened_url(&self.app) {
                    self.opened_url.set(Some(url));
                    self.deliver_opened_url();
                }
            }
            MainEvent::Pause => self.notify_app_lifecycle(gpui::AppLifecyclePhase::Inactive),
            MainEvent::Stop => self.notify_app_lifecycle(gpui::AppLifecyclePhase::Background),
            _ => {}
        }
    }

    fn notify_app_lifecycle(&self, phase: gpui::AppLifecyclePhase) {
        // Taken out for the call so the callback can register a replacement.
        let callback = self.callbacks.borrow_mut().app_lifecycle.take();
        if let Some(mut callback) = callback {
            callback(phase);
            self.callbacks
                .borrow_mut()
                .app_lifecycle
                .get_or_insert(callback);
        }
    }

    fn deliver_opened_url(&self) {
        let callback = self.callbacks.borrow_mut().open_urls.take();
        if let Some(mut callback) = callback {
            if let Some(url) = self.opened_url.take() {
                callback(vec![url]);
            }
            self.callbacks
                .borrow_mut()
                .open_urls
                .get_or_insert(callback);
        }
    }

    fn process_input(&self) {
        let Some(window) = self.window() else {
            return;
        };
        let mut iter = match self.app.input_events_iter() {
            Ok(iter) => iter,
            Err(error) => {
                log::error!("failed to get input events iterator: {error:?}");
                return;
            }
        };
        let mut key_maps = self.key_maps.borrow_mut();
        let mut touch_ids = self.touch_ids.borrow_mut();
        loop {
            let more = iter.next(|event| {
                events::handle_input_event(event, &window, &mut touch_ids, &mut key_maps)
            });
            if !more {
                break;
            }
        }
    }

    fn drain_main_runnables(&self) {
        let receiver = self.main_receiver.clone();
        for runnable in receiver.try_iter() {
            match runnable {
                Ok(runnable) => {
                    runnable.run();
                }
                Err(_) => break,
            }
        }
    }

    fn handle_display_refresh(&self) {
        let Some(window) = self.window() else {
            return;
        };
        if let Some(system_insets) = insets::take_reported() {
            window.set_system_insets(system_insets);
        }
        if self.vsync.take_pending() {
            window.request_frame(false);
        }
        window.reconcile_keyboard();
        window.vote_frame_rate(self.display.max_refresh_rate());
    }
}

fn system_fonts() -> Vec<Cow<'static, [u8]>> {
    let mut fonts = Vec::new();
    let Ok(entries) = std::fs::read_dir("/system/fonts") else {
        log::warn!("/system/fonts is not readable");
        return fonts;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let wanted = name.starts_with("Roboto-")
            || name.starts_with("RobotoStatic-")
            || name == "NotoColorEmoji.ttf"
            || name.starts_with("NotoSansSymbols-");
        if !wanted {
            continue;
        }
        match std::fs::read(&path) {
            Ok(bytes) => fonts.push(Cow::Owned(bytes)),
            Err(error) => log::warn!("failed to read font {path:?}: {error}"),
        }
    }
    if fonts.is_empty() {
        log::warn!(
            "no Roboto fonts found in /system/fonts; text rendering will fail unless the app bundles fonts"
        );
    }
    fonts
}

impl Platform for AndroidPlatform {
    fn platform_info(&self) -> gpui::PlatformInfo {
        gpui::PlatformInfo {
            os: gpui::OperatingSystem::Android,
            target: gpui::PlatformTarget::Native,
        }
    }

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
        *self.pending_launch.borrow_mut() = Some(on_finish_launching);
        let app = self.app.clone();
        while !self.quit_requested.get() {
            let timeout = self
                .window()
                .and_then(|window| window.frame_rate_vote_expires_in());
            app.poll_events(timeout, |event| {
                match event {
                    PollEvent::Wake | PollEvent::Timeout => {}
                    PollEvent::Main(main_event) => self.handle_main_event(main_event),
                    _ => {}
                }
                self.drain_main_runnables();
                self.handle_display_refresh();
            });
        }
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(ref mut quit) = callbacks.quit {
            // The activity is already being destroyed, which cannot be vetoed.
            quit();
        }
        // android-activity cannot host another activity in a process whose
        // `android_main` has returned, so a process left running here would
        // show nothing when the app is next opened.
        std::process::exit(0);
    }

    fn quit(&self) {
        self.quit_requested.set(true);
    }

    fn restart(&self, _binary_path: Option<PathBuf>, _arguments: Vec<std::ffi::OsString>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![self.display.clone()]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn on_displays_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.active_window
            .borrow()
            .as_ref()
            .map(|(handle, _)| *handle)
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let window = AndroidWindow::new(
            handle,
            params,
            self.app.clone(),
            self.gpu_context.clone(),
            self.display.clone(),
            self.window_appearance(),
            self.vsync.clone(),
        )?;
        self.display
            .set_size(window.inner.state.borrow().bounds.size);
        *self.active_window.borrow_mut() = Some((handle, Rc::clone(&window.inner)));
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        match self.app.config().ui_mode_night() {
            android_activity::ndk::configuration::UiModeNight::Yes => WindowAppearance::Dark,
            _ => WindowAppearance::Light,
        }
    }

    fn open_url(&self, url: &str) {
        // SAFETY: android-activity guarantees `vm_as_ptr` returns the
        // process's valid `JavaVM` pointer for the lifetime of the app.
        let vm = unsafe { jni::JavaVM::from_raw(self.app.vm_as_ptr().cast()) };
        let activity_ptr = self.app.activity_as_ptr();
        let url_is_web_page = url.starts_with("http://") || url.starts_with("https://");
        let result = vm.attach_current_thread(|env| {
            // SAFETY: `activity_as_ptr` is a global reference to the activity
            // that lives as long as the app. `JObject` does not delete the
            // reference on drop.
            let activity = unsafe { JObject::from_raw(env, activity_ptr as jni::sys::jobject) };
            let url = env.new_string(url)?;
            let uri = env
                .call_static_method(
                    jni_str!("android/net/Uri"),
                    jni_str!("parse"),
                    jni_sig!("(Ljava/lang/String;)Landroid/net/Uri;"),
                    &[JValue::Object(&url)],
                )?
                .l()?;
            let action = env.new_string("android.intent.action.VIEW")?;
            let intent = env.new_object(
                jni_str!("android/content/Intent"),
                jni_sig!("(Ljava/lang/String;Landroid/net/Uri;)V"),
                &[JValue::Object(&action), JValue::Object(&uri)],
            )?;
            // Browsers that support custom tabs show the page over this app
            // rather than switching to their own task, as iOS does with its
            // in-app browser. The extra's presence asks for that; it has no value.
            if url_is_web_page {
                let extras = env.new_object(jni_str!("android/os/Bundle"), jni_sig!("()V"), &[])?;
                let session = env.new_string("android.support.customtabs.extra.SESSION")?;
                env.call_method(
                    &extras,
                    jni_str!("putBinder"),
                    jni_sig!("(Ljava/lang/String;Landroid/os/IBinder;)V"),
                    &[JValue::Object(&session), JValue::Object(&JObject::null())],
                )?;
                env.call_method(
                    &intent,
                    jni_str!("putExtras"),
                    jni_sig!("(Landroid/os/Bundle;)Landroid/content/Intent;"),
                    &[JValue::Object(&extras)],
                )?;
            }
            // Throws when no installed app handles the URL.
            env.call_method(
                &activity,
                jni_str!("startActivity"),
                jni_sig!("(Landroid/content/Intent;)V"),
                &[JValue::Object(&intent)],
            )?;
            Ok::<_, jni::errors::Error>(())
        });
        if let Err(error) = result {
            log::error!("failed to open {url}: {error}");
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.callbacks.borrow_mut().open_urls = Some(callback);
        self.deliver_opened_url();
    }

    fn on_app_lifecycle(&self, callback: Box<dyn FnMut(gpui::AppLifecyclePhase)>) {
        self.callbacks.borrow_mut().app_lifecycle = Some(callback);
    }

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow::anyhow!(
            "prompt_for_paths is not supported on Android"
        )))
        .ok();
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow::anyhow!(
            "prompt_for_new_path is not supported on Android"
        )))
        .ok();
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.borrow_mut().quit = Some(callback);
    }

    fn on_system_sleep(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().reopen = Some(callback);
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

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

    fn prevent_idle_sleep(&self, _reason: &str) -> Task<Result<gpui::ActivityGuard>> {
        Task::ready(Err(anyhow::anyhow!(
            "idle sleep prevention is not implemented on Android"
        )))
    }

    fn compositor_name(&self) -> &'static str {
        "Android"
    }

    fn app_path(&self) -> Result<PathBuf> {
        Err(anyhow::anyhow!("app_path is not available on Android"))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "path_for_auxiliary_executable is not available on Android"
        ))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn hide_cursor_until_mouse_moves(&self) {}

    fn is_cursor_visible(&self) -> bool {
        false
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {}

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "credential storage is not implemented on Android"
        )))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "credential storage is not implemented on Android"
        )))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(AndroidKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().keyboard_layout_change = Some(callback);
    }
}
