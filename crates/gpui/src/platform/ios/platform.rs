//! iOS Platform implementation.
//!
//! This implements the Platform trait for iOS using UIKit.
//! Key differences from macOS:
//! - Uses UIApplication instead of NSApplication
//! - No menu bar (iOS apps don't have traditional menus)
//! - No windowed mode (iOS apps are always fullscreen on their display)
//! - Touch-based input instead of mouse
//! - System keyboard handling differs significantly

use super::{IosDispatcher, IosDisplay, IosWindow};
use crate::platform::blade;
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay, PlatformKeyboardLayout,
    PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Result, Task, WindowAppearance,
    WindowParams,
};
use anyhow::anyhow;
use futures::channel::oneshot;
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use parking_lot::Mutex;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub(crate) struct IosPlatform(Mutex<IosPlatformState>);

pub(crate) struct IosPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    renderer_context: blade::Context,
    finish_launching: Option<Box<dyn FnOnce()>>,
    quit_callback: Option<Box<dyn FnMut()>>,
    open_urls_callback: Option<Box<dyn FnMut(Vec<String>)>>,
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(IosDispatcher);

        #[cfg(feature = "font-kit")]
        let text_system = Arc::new(crate::platform::ios::IosTextSystem::new());

        #[cfg(not(feature = "font-kit"))]
        let text_system = Arc::new(crate::NoopTextSystem::new());

        Self(Mutex::new(IosPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system,
            renderer_context: blade::Context::default(),
            finish_launching: None,
            quit_callback: None,
            open_urls_callback: None,
        }))
    }
}

impl Platform for IosPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        // Store the callback for later invocation via FFI.
        // The callback will be invoked when gpui_ios_did_finish_launching() is called
        // from the iOS app delegate's applicationDidFinishLaunchingWithOptions:.
        self.0.lock().finish_launching = Some(on_finish_launching);

        // On iOS, the app lifecycle is managed by UIApplicationMain which must be
        // called from main() before any Rust code runs. The Application::run() method
        // is called during app initialization, before UIApplicationMain starts its
        // event loop.
        //
        // The finish_launching callback is stored and will be invoked when the iOS
        // app delegate calls gpui_ios_did_finish_launching() via FFI.
        //
        // Unlike macOS where we call NSApplication.run() here, on iOS we don't need
        // to start the run loop - UIApplicationMain handles that.
        //
        // The callback is forwarded to the FFI layer so it can be invoked from Obj-C.
        if let Some(callback) = self.0.lock().finish_launching.take() {
            super::ffi::set_finish_launching_callback(callback);
        }

        log::info!("GPUI iOS: Platform::run() completed, waiting for app delegate callback");
    }

    fn quit(&self) {
        // iOS apps cannot programmatically quit - they can only be terminated by the user
        // or the system. We can suspend to background though.
        log::warn!("iOS apps cannot programmatically quit");
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        // iOS apps cannot restart themselves
        log::warn!("iOS apps cannot restart themselves");
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        // iOS handles app activation automatically
    }

    fn hide(&self) {
        // iOS apps cannot hide themselves
    }

    fn hide_other_apps(&self) {
        // Not applicable on iOS
    }

    fn unhide_other_apps(&self) {
        // Not applicable on iOS
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        IosDisplay::all()
            .map(|display| Rc::new(display) as Rc<dyn PlatformDisplay>)
            .collect()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(IosDisplay::main()))
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        // iOS typically has one active window
        // This would need to track the current key window
        None
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let renderer_context = self.0.lock().renderer_context.clone();
        let window = Box::new(IosWindow::new(handle, options, renderer_context)?);
        // Register the window with FFI layer so Objective-C can access it for rendering
        window.register_with_ffi();
        Ok(window)
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let style: i64 = {
                let window: *mut Object = msg_send![class!(UIApplication), sharedApplication];
                let key_window: *mut Object = msg_send![window, keyWindow];
                if key_window.is_null() {
                    return WindowAppearance::Light;
                }
                let trait_collection: *mut Object = msg_send![key_window, traitCollection];
                msg_send![trait_collection, userInterfaceStyle]
            };

            // UIUserInterfaceStyle: 0 = unspecified, 1 = light, 2 = dark
            match style {
                2 => WindowAppearance::Dark,
                _ => WindowAppearance::Light,
            }
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let url_string: *mut Object =
                msg_send![class!(NSString), stringWithUTF8String: url.as_ptr()];
            let url: *mut Object = msg_send![class!(NSURL), URLWithString: url_string];
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let _: () = msg_send![app, openURL: url options: std::ptr::null::<Object>() completionHandler: std::ptr::null::<Object>()];
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls_callback = Some(callback);
    }

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        // URL schemes on iOS are registered in Info.plist, not programmatically
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        // iOS uses UIDocumentPickerViewController for file selection
        // This would need to be implemented with proper UIKit integration
        let _ = tx.send(Err(anyhow!("File picker not yet implemented for iOS")));
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        let _ = tx.send(Err(anyhow!("Save dialog not yet implemented for iOS")));
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {
        // iOS doesn't have a file manager like Finder
    }

    fn open_with_system(&self, _path: &Path) {
        // Would use UIDocumentInteractionController or UIActivityViewController
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().quit_callback = Some(callback);
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles app reopening through scene lifecycle
    }

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {
        // iOS doesn't have a menu bar
        // Could potentially integrate with UIMenuBuilder for context menus
    }

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {
        // iOS doesn't have a dock menu
    }

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn Action)>) {
        // Not applicable on iOS
    }

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {
        // Not applicable on iOS
    }

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        // Not applicable on iOS
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle: *mut Object = msg_send![class!(NSBundle), mainBundle];
            let path: *mut Object = msg_send![bundle, bundlePath];
            let utf8: *const i8 = msg_send![path, UTF8String];
            if utf8.is_null() {
                return Err(anyhow!("Failed to get bundle path"));
            }
            let path_str = std::ffi::CStr::from_ptr(utf8).to_str()?;
            Ok(PathBuf::from(path_str))
        }
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        let app_path = self.app_path()?;
        Ok(app_path.join(name))
    }

    fn set_cursor_style(&self, _style: CursorStyle) {
        // iOS doesn't have visible cursors (except for Apple Pencil hover on iPad)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true // iOS always auto-hides scrollbars
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        unsafe {
            let pasteboard: *mut Object = msg_send![class!(UIPasteboard), generalPasteboard];
            if let Some(text) = item.text() {
                let ns_string: *mut Object =
                    msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()];
                let _: () = msg_send![pasteboard, setString: ns_string];
            }
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unsafe {
            let pasteboard: *mut Object = msg_send![class!(UIPasteboard), generalPasteboard];
            let string: *mut Object = msg_send![pasteboard, string];
            if string.is_null() {
                return None;
            }
            let utf8: *const i8 = msg_send![string, UTF8String];
            if utf8.is_null() {
                return None;
            }
            let text = std::ffi::CStr::from_ptr(utf8).to_str().ok()?;
            Some(ClipboardItem::new_string(text.to_string()))
        }
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        // Would use iOS Keychain Services
        Task::ready(Err(anyhow!("Keychain not yet implemented for iOS")))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Err(anyhow!("Keychain not yet implemented for iOS")))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("Keychain not yet implemented for iOS")))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(IosKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(IosKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles keyboard layout changes differently
    }
}

/// iOS keyboard layout implementation.
/// iOS doesn't expose the same keyboard layout APIs as macOS.
pub struct IosKeyboardLayout;

impl PlatformKeyboardLayout for IosKeyboardLayout {
    fn id(&self) -> &str {
        "ios"
    }

    fn name(&self) -> &str {
        "iOS"
    }
}

/// iOS keyboard mapper implementation.
pub struct IosKeyboardMapper;

impl PlatformKeyboardMapper for IosKeyboardMapper {
    fn map_key_equivalent(
        &self,
        keystroke: crate::Keystroke,
        _use_key_equivalents: bool,
    ) -> crate::KeybindingKeystroke {
        crate::KeybindingKeystroke::from_keystroke(keystroke)
    }

    fn get_key_equivalents(&self) -> Option<&collections::HashMap<char, char>> {
        None
    }
}
