use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, Menu, MenuItem, OwnedMenu, PathPromptOptions, Platform,
    PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, Task, ThermalState, WindowAppearance, WindowParams,
};

use super::{
    AndroidDispatcher, AndroidDisplay, AndroidKeyboardLayout, AndroidWindow, MainThreadMailbox,
    current_native_window,
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
/// The current scaffold exposes real executors, a real text system, and stub
/// implementations for everything that requires deep JNI/SurfaceFlinger
/// integration (windowing, clipboard, file pickers, credentials). Each stubbed
/// method is shaped to slot a real implementation in without changing
/// signatures.
pub struct AndroidPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    main_mailbox: Arc<MainThreadMailbox>,
    active_display: Rc<AndroidDisplay>,
    active_window: RefCell<Option<AnyWindowHandle>>,
    callbacks: RefCell<PlatformCallbacks>,
    menus: RefCell<Vec<OwnedMenu>>,
    headless: bool,
}

impl AndroidPlatform {
    pub fn new(headless: bool) -> Self {
        let (dispatcher, main_mailbox) = AndroidDispatcher::new();
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        // CosmicTextSystem walks the system font database. Android's
        // fontconfig + /system/fonts is auto-discovered by cosmic-text via
        // fontdb. "Roboto" is the platform default and is guaranteed to be
        // present on every Android device.
        let text_system: Arc<dyn PlatformTextSystem> =
            Arc::new(gpui_wgpu::CosmicTextSystem::new("Roboto"));

        let active_display = Rc::new(AndroidDisplay::new());

        Self {
            background_executor,
            foreground_executor,
            text_system,
            main_mailbox,
            active_display,
            active_window: RefCell::new(None),
            callbacks: RefCell::new(PlatformCallbacks::default()),
            menus: RefCell::new(Vec::new()),
            headless,
        }
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
        on_finish_launching();

        // In production, the JNI bridge will pump the main thread mailbox via
        // an `ALooper` callback. As a fallback (e.g. when running under an
        // android-activity GameActivity event loop or in CI) we drain the
        // mailbox here on the calling thread until `quit` is signalled.
        while self.main_mailbox.drain_blocking() {}

        if let Some(mut quit) = self.callbacks.borrow_mut().quit.take() {
            quit();
        }
    }

    fn quit(&self) {
        self.main_mailbox.signal_stop();
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        // Android applications cannot restart themselves; the platform
        // handles that via Play Store updates or the system's own restart
        // semantics.
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
        if current_native_window().is_none() {
            anyhow::bail!(
                "AndroidPlatform::open_window: no NativeWindow registered \
                 (call gpui_android::set_native_window from surfaceCreated first)"
            );
        }

        let window = AndroidWindow::new(handle, params, self.active_display.clone());
        *self.active_window.borrow_mut() = Some(handle);
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        // Wired up to `Configuration.uiMode & UI_MODE_NIGHT_MASK` once JNI
        // bridge is in place. Default to light to match the UI on a fresh
        // Android install.
        WindowAppearance::Light
    }

    fn open_url(&self, url: &str) {
        log::warn!("AndroidPlatform::open_url is not implemented (would open {url})");
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

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

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
        // The "app" on Android is the .apk; we report the current executable
        // path which is the loaded `.so`. Callers usually only need this for
        // restart, which we don't support anyway.
        Ok(std::env::current_exe()?)
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "path_for_auxiliary_executable is not implemented on Android (would resolve {name} via Context.getApplicationInfo().nativeLibraryDir)"
        ))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {
        // Pointer cursors only have meaning when a hardware mouse is attached
        // (e.g. DeX or ChromeOS). Ignored for now.
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
            "credential storage is not implemented on Android (AndroidKeyStore integration pending)"
        )))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "credential storage is not implemented on Android (AndroidKeyStore integration pending)"
        )))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_platform_constructs_and_quits_cleanly() {
        let platform = AndroidPlatform::new(true);
        platform.quit();
        platform.run(Box::new(|| {}));
    }
}
