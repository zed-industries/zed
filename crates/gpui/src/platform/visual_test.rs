//! Visual test platform that combines real rendering (macOs-only for now) with controllable TestDispatcher.
//!
//! This platform is used for visual tests that need:
//! - Real rendering (e.g. Metal/compositor) for accurate screenshots
//! - Deterministic task scheduling via TestDispatcher
//! - Controllable time via `advance_clock`

#[cfg(feature = "screen-capture")]
use crate::ScreenCaptureSource;
use crate::{
    AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor, Keymap,
    MacPlatform, Menu, MenuItem, OwnedMenu, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Task,
    TestDispatcher, WindowAppearance, WindowParams,
};
use anyhow::Result;
use futures::channel::oneshot;
use parking_lot::Mutex;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

/// A platform that combines real Mac rendering with controllable TestDispatcher.
///
/// This allows visual tests to:
/// - Render real UI via Metal for accurate screenshots
/// - Control task scheduling deterministically via TestDispatcher
/// - Advance simulated time for testing time-based behaviors (tooltips, animations, etc.)
pub struct VisualTestPlatform {
    dispatcher: TestDispatcher,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    mac_platform: MacPlatform,
    clipboard: Mutex<Option<ClipboardItem>>,
    find_pasteboard: Mutex<Option<ClipboardItem>>,
}

impl VisualTestPlatform {
    /// Creates a new VisualTestPlatform with the given random seed.
    ///
    /// The seed is used for deterministic random number generation in the TestDispatcher.
    pub fn new(seed: u64) -> Self {
        let dispatcher = TestDispatcher::new(seed);
        let arc_dispatcher = Arc::new(dispatcher.clone());

        let background_executor = BackgroundExecutor::new(arc_dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(arc_dispatcher);

        let mac_platform = MacPlatform::new(false);

        Self {
            dispatcher,
            background_executor,
            foreground_executor,
            mac_platform,
            clipboard: Mutex::new(None),
            find_pasteboard: Mutex::new(None),
        }
    }

    /// Returns a reference to the TestDispatcher for controlling task scheduling and time.
    pub fn dispatcher(&self) -> &TestDispatcher {
        &self.dispatcher
    }
}

impl Platform for VisualTestPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.mac_platform.text_system()
    }

    fn run(&self, _on_finish_launching: Box<dyn 'static + FnOnce()>) {
        panic!("VisualTestPlatform::run should not be called in tests")
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.mac_platform.displays()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.mac_platform.primary_display()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.mac_platform.active_window()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        self.mac_platform.window_stack()
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        self.mac_platform.is_screen_capture_supported()
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
        self.mac_platform.screen_capture_sources()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        self.mac_platform.open_window(handle, options)
    }

    fn window_appearance(&self) -> WindowAppearance {
        self.mac_platform.window_appearance()
    }

    fn open_url(&self, url: &str) {
        self.mac_platform.open_url(url)
    }

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
        true
    }

    fn reveal_path(&self, path: &Path) {
        self.mac_platform.reveal_path(path)
    }

    fn open_with_system(&self, path: &Path) {
        self.mac_platform.open_with_system(path)
    }

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        None
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn crate::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn crate::Action) -> bool>) {}

    fn app_path(&self) -> Result<PathBuf> {
        self.mac_platform.app_path()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        self.mac_platform.path_for_auxiliary_executable(name)
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        self.mac_platform.set_cursor_style(style)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        self.mac_platform.should_auto_hide_scrollbars()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.clipboard.lock().clone()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.clipboard.lock() = Some(item);
    }

    #[cfg(target_os = "macos")]
    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem> {
        self.find_pasteboard.lock().clone()
    }

    #[cfg(target_os = "macos")]
    fn write_to_find_pasteboard(&self, item: ClipboardItem) {
        *self.find_pasteboard.lock() = Some(item);
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        self.mac_platform.keyboard_layout()
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        self.mac_platform.keyboard_mapper()
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}
}
