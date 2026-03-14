use crate::{
    IosDispatcher, IosDisplay, IosKeyboardLayout, IosWindow, ios_keyboard_mapper,
    main_screen_bounds,
};
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Keymap, Menu, MenuItem, NoopTextSystem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Task,
    ThermalState, WindowAppearance, WindowParams,
};
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

#[derive(Default)]
struct IosPlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_state_change: Option<Box<dyn FnMut()>>,
}

/// iOS platform implementation for GPUI.
///
/// Phase 1 status: lifecycle and executor plumbing are complete. UIKit window
/// creation is wired up in Phase 1.3 via the Swift→Rust FFI in SceneDelegate.
pub struct IosPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    display: Rc<dyn PlatformDisplay>,
    active_window: RefCell<Option<AnyWindowHandle>>,
    callbacks: Mutex<IosPlatformCallbacks>,
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(IosDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        // TODO Phase 1.3: replace NoopTextSystem with CoreText implementation.
        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(NoopTextSystem::new());

        let bounds = main_screen_bounds();
        let display: Rc<dyn PlatformDisplay> = Rc::new(IosDisplay::new(bounds));

        Self {
            background_executor,
            foreground_executor,
            text_system,
            display,
            active_window: RefCell::new(None),
            callbacks: Mutex::new(IosPlatformCallbacks::default()),
        }
    }
}

impl Platform for IosPlatform {
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
        // On iOS the run loop is owned by UIKit. GPUI is bootstrapped from
        // SceneDelegate.sceneDidBecomeActive via the zed_ios_main() FFI call.
        // We just invoke the launching callback here; UIKit drives the loop.
        on_finish_launching();
    }

    fn quit(&self) {
        // iOS does not support programmatic quit; Home button is the only exit.
        log::warn!("IosPlatform::quit called — iOS does not support programmatic quit");
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {}

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

    fn active_window(&self) -> Option<AnyWindowHandle> {
        *self.active_window.borrow()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let window = IosWindow::new(params, self.display.clone());
        *self.active_window.borrow_mut() = Some(handle);
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        // TODO Phase 1.3: query UITraitCollection.userInterfaceStyle from the
        // current UIWindowScene.
        WindowAppearance::Light
    }

    fn open_url(&self, _url: &str) {
        // TODO Phase 1.3: UIApplication.shared.open(_:options:completionHandler:)
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.callbacks.lock().open_urls = Some(callback);
    }

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        // URL schemes are registered in Info.plist; no runtime registration needed.
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        // Used only for SSH key import in Phase 2. Implement via
        // UIDocumentPickerViewController when needed.
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow::anyhow!(
            "prompt_for_paths: UIDocumentPicker not yet implemented"
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
            "prompt_for_new_path is not supported on iOS"
        )))
        .ok();
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {
        // Remote paths exist on the host, not the iPad filesystem.
    }

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().reopen = Some(callback);
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.callbacks.lock().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.callbacks.lock().validate_app_menu_command = Some(callback);
    }

    fn thermal_state(&self) -> ThermalState {
        // TODO Phase 1.3: query NSProcessInfo.processInfo.thermalState
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().thermal_state_change = Some(callback);
    }

    fn compositor_name(&self) -> &'static str {
        "Metal (iOS)"
    }

    fn app_path(&self) -> Result<PathBuf> {
        // iOS sandbox: app bundle is at NSBundle.mainBundle.bundlePath
        Err(anyhow::anyhow!(
            "app_path: not yet implemented for iOS — use NSBundle.mainBundle"
        ))
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "path_for_auxiliary_executable: subprocess spawning is prohibited on iOS"
        ))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {
        // UIPointerStyle is set per UIPointerInteraction region in Phase 1.3.
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        // TODO Phase 1.3: UIPasteboard.general
        None
    }

    fn write_to_clipboard(&self, _item: ClipboardItem) {
        // TODO Phase 1.3: UIPasteboard.general
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        // SSH keys go to the iOS Keychain via SecItemAdd in crates/zed-ios.
        Task::ready(Err(anyhow::anyhow!(
            "write_credentials: use zed_ios::keychain for SSH key storage"
        )))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow::anyhow!(
            "delete_credentials: use zed_ios::keychain for SSH key storage"
        )))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(IosKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        ios_keyboard_mapper()
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().keyboard_layout_change = Some(callback);
    }
}
