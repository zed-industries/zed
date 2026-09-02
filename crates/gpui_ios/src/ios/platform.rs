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
use anyhow::{Context as _, anyhow};
use core_foundation::{
    base::{CFType, CFTypeRef, OSStatus, TCFType},
    boolean::CFBoolean,
    data::CFData,
    dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary},
    string::{CFString, CFStringRef},
};
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, AppLifecyclePhase, BackgroundExecutor, ClipboardItem, CursorStyle,
    DummyKeyboardMapper, ForegroundExecutor, GestureKinds, GestureTuning, Keymap, Menu, MenuItem,
    PathPromptOptions, Platform, PlatformDisplay, PlatformGestures, PlatformKeyboardLayout,
    PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Result, ScrollPhysics, Task,
    ThermalState, WindowAppearance, WindowParams,
};
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use parking_lot::Mutex;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    ptr,
    rc::Rc,
    sync::Arc,
};

pub struct IosPlatform(Mutex<IosPlatformState>);

struct IosGestures;

impl PlatformGestures for IosGestures {
    fn tuning(&self) -> GestureTuning {
        GestureTuning {
            scroll_physics: ScrollPhysics::ios(),
            ..GestureTuning::default()
        }
    }

    fn native_recognizers(&self) -> GestureKinds {
        GestureKinds {
            pan: true,
            ..GestureKinds::NONE
        }
    }
}

pub(crate) struct IosPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    thermal_state_callback: Option<Box<dyn FnMut()>>,
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(IosDispatcher);

        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(super::IosTextSystem::new());

        Self(Mutex::new(IosPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system,
            thermal_state_callback: None,
        }))
    }

    fn root_view_controller() -> Option<*mut AnyObject> {
        unsafe {
            let scene = super::ffi::window_scene();
            if scene.is_null() {
                return None;
            }

            let windows: *mut AnyObject = msg_send![scene, windows];
            if windows.is_null() {
                return None;
            }

            let count: usize = msg_send![windows, count];
            let mut fallback_window: *mut AnyObject = ptr::null_mut();
            for index in 0..count {
                let window: *mut AnyObject = msg_send![windows, objectAtIndex: index];
                if fallback_window.is_null() {
                    fallback_window = window;
                }

                let is_key_window: bool = msg_send![window, isKeyWindow];
                if is_key_window {
                    let view_controller: *mut AnyObject = msg_send![window, rootViewController];
                    return (!view_controller.is_null()).then_some(view_controller);
                }
            }

            if fallback_window.is_null() {
                return None;
            }

            let view_controller: *mut AnyObject = msg_send![fallback_window, rootViewController];
            (!view_controller.is_null()).then_some(view_controller)
        }
    }

    fn presented_view_controller() -> Option<*mut AnyObject> {
        unsafe {
            let mut view_controller = Self::root_view_controller()?;
            loop {
                let presented: *mut AnyObject = msg_send![view_controller, presentedViewController];
                if presented.is_null() {
                    return Some(view_controller);
                }
                view_controller = presented;
            }
        }
    }

    fn dismiss_presented_browser() {
        unsafe {
            let mut view_controller = match Self::root_view_controller() {
                Some(view_controller) => view_controller,
                None => return,
            };

            loop {
                let presented: *mut AnyObject = msg_send![view_controller, presentedViewController];
                if presented.is_null() {
                    return;
                }

                let is_browser: bool =
                    msg_send![presented, isKindOfClass: class!(SFSafariViewController)];
                if is_browser {
                    let _: () = msg_send![
                        presented,
                        dismissViewControllerAnimated: true,
                        completion: ptr::null::<AnyObject>()
                    ];
                    return;
                }

                view_controller = presented;
            }
        }
    }

    fn open_url_with_system(url: *mut AnyObject) {
        unsafe {
            let app: *mut AnyObject = msg_send![class!(UIApplication), sharedApplication];
            let _: () = msg_send![
                app,
                openURL: url,
                options: ptr::null::<AnyObject>(),
                completionHandler: ptr::null::<AnyObject>()
            ];
        }
    }
}

/// A simple iOS keyboard layout.
struct IosKeyboardLayout;

impl PlatformKeyboardLayout for IosKeyboardLayout {
    fn id(&self) -> &str {
        "ios-default"
    }

    fn name(&self) -> &str {
        "iOS Default"
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

    fn gestures(&self) -> Option<Rc<dyn PlatformGestures>> {
        Some(Rc::new(IosGestures))
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        super::ffi::set_finish_launching_callback(on_finish_launching);
    }

    fn quit(&self) {
        // iOS apps cannot programmatically quit - they can only be terminated by the user
        // or the system. We can suspend to background though.
        log::warn!("iOS apps cannot programmatically quit");
    }

    fn restart(&self, _binary_path: Option<PathBuf>, _arguments: Vec<OsString>) {
        // iOS apps cannot restart themselves
        log::warn!("iOS apps cannot restart themselves");
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        Self::dismiss_presented_browser();
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
        let window = Box::new(IosWindow::new(handle, options)?);
        // Register the window with FFI layer so Objective-C can access it for rendering
        window.register_with_ffi();
        Ok(window)
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let style: i64 = {
                let app: *mut AnyObject = msg_send![class!(UIApplication), sharedApplication];
                let key_window: *mut AnyObject = msg_send![app, keyWindow];
                if key_window.is_null() {
                    return WindowAppearance::Light;
                }
                let trait_collection: *mut AnyObject = msg_send![key_window, traitCollection];
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
            let url_string = super::util::nsstring(url);
            let native_url: *mut AnyObject = msg_send![class!(NSURL), URLWithString: url_string];
            if native_url.is_null() {
                log::error!("GPUI iOS: Could not parse URL: {url}");
                return;
            }

            if url.starts_with("https://") || url.starts_with("http://") {
                if let Some(view_controller) = Self::presented_view_controller() {
                    let browser: *mut AnyObject = msg_send![class!(SFSafariViewController), alloc];
                    let browser: *mut AnyObject = msg_send![browser, initWithURL: native_url];
                    if !browser.is_null() {
                        let _: () = msg_send![
                            view_controller,
                            presentViewController: browser,
                            animated: true,
                            completion: ptr::null::<AnyObject>()
                        ];
                        return;
                    }
                }
            }

            Self::open_url_with_system(native_url);
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        super::ffi::set_open_urls_callback(callback);
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
        if tx
            .send(Err(anyhow!("File picker not yet implemented for iOS")))
            .is_err()
        {
            log::debug!("GPUI iOS: File picker receiver was dropped");
        }
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        if tx
            .send(Err(anyhow!("Save dialog not yet implemented for iOS")))
            .is_err()
        {
            log::debug!("GPUI iOS: Save dialog receiver was dropped");
        }
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

    fn on_quit(&self, callback: Box<dyn FnMut() -> bool>) {
        super::ffi::set_quit_callback(callback);
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles app reopening through scene lifecycle
    }

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn on_app_lifecycle(&self, callback: Box<dyn FnMut(AppLifecyclePhase)>) {
        super::ffi::set_app_lifecycle_callback(callback);
    }

    fn on_memory_warning(&self, callback: Box<dyn FnMut()>) {
        super::ffi::set_memory_warning_callback(callback);
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
            let bundle: *mut AnyObject = msg_send![class!(NSBundle), mainBundle];
            let path: *mut AnyObject = msg_send![bundle, bundlePath];
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

    fn hide_cursor_until_mouse_moves(&self) {}

    fn is_cursor_visible(&self) -> bool {
        false
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true // iOS always auto-hides scrollbars
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        unsafe {
            let pasteboard: *mut AnyObject = msg_send![class!(UIPasteboard), generalPasteboard];
            if let Some(text) = item.text() {
                let ns_string = super::util::nsstring(&text);
                let _: () = msg_send![pasteboard, setString: ns_string];
            }
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unsafe {
            let pasteboard: *mut AnyObject = msg_send![class!(UIPasteboard), generalPasteboard];
            let string: *mut AnyObject = msg_send![pasteboard, string];
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

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor().spawn(async move {
            unsafe {
                use security::*;

                let url = CFString::from(url.as_str());
                let username = CFString::from(username.as_str());
                let password = CFData::from_buffer(&password);

                let mut query_attributes = CFMutableDictionary::with_capacity(2);
                query_attributes.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attributes.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let mut updated_attributes = CFMutableDictionary::with_capacity(2);
                updated_attributes.set(kSecAttrAccount as *const _, username.as_CFTypeRef());
                updated_attributes.set(kSecValueData as *const _, password.as_CFTypeRef());

                let mut operation = "updating";
                let mut status = SecItemUpdate(
                    query_attributes.as_concrete_TypeRef(),
                    updated_attributes.as_concrete_TypeRef(),
                );
                if status == ERR_SEC_ITEM_NOT_FOUND {
                    operation = "creating";
                    let mut new_item_attributes = CFMutableDictionary::with_capacity(4);
                    new_item_attributes
                        .set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                    new_item_attributes.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                    new_item_attributes.set(kSecAttrAccount as *const _, username.as_CFTypeRef());
                    new_item_attributes.set(kSecValueData as *const _, password.as_CFTypeRef());
                    status = SecItemAdd(new_item_attributes.as_concrete_TypeRef(), ptr::null_mut());
                }
                anyhow::ensure!(
                    status == ERR_SEC_SUCCESS,
                    "{operation} password failed: {status}"
                );
            }
            Ok(())
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let url = url.to_string();
        self.background_executor().spawn(async move {
            let url = CFString::from(url.as_str());
            let cf_true = CFBoolean::true_value().as_CFTypeRef();

            unsafe {
                use security::*;

                let mut attributes = CFMutableDictionary::with_capacity(4);
                attributes.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                attributes.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                attributes.set(kSecReturnAttributes as *const _, cf_true);
                attributes.set(kSecReturnData as *const _, cf_true);

                let mut result = CFTypeRef::from(ptr::null());
                let status = SecItemCopyMatching(attributes.as_concrete_TypeRef(), &mut result);
                match status {
                    ERR_SEC_SUCCESS => {}
                    ERR_SEC_ITEM_NOT_FOUND | ERR_SEC_USER_CANCELED => return Ok(None),
                    _ => anyhow::bail!("reading password failed: {status}"),
                }

                let result = CFType::wrap_under_create_rule(result)
                    .downcast::<CFDictionary>()
                    .context("keychain item was not a dictionary")?;
                let username = result
                    .find(kSecAttrAccount as *const _)
                    .context("account was missing from keychain item")?;
                let username = CFType::wrap_under_get_rule(*username)
                    .downcast::<CFString>()
                    .context("account was not a string")?;
                let password = result
                    .find(kSecValueData as *const _)
                    .context("password was missing from keychain item")?;
                let password = CFType::wrap_under_get_rule(*password)
                    .downcast::<CFData>()
                    .context("password was not data")?;

                Ok(Some((username.to_string(), password.bytes().to_vec())))
            }
        })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let url = url.to_string();
        self.background_executor().spawn(async move {
            unsafe {
                use security::*;

                let url = CFString::from(url.as_str());
                let mut query_attributes = CFMutableDictionary::with_capacity(2);
                query_attributes.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attributes.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let status = SecItemDelete(query_attributes.as_concrete_TypeRef());
                anyhow::ensure!(
                    matches!(status, ERR_SEC_SUCCESS | ERR_SEC_ITEM_NOT_FOUND),
                    "deleting password failed: {status}"
                );
            }
            Ok(())
        })
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles keyboard layout changes differently
    }

    fn thermal_state(&self) -> ThermalState {
        // iOS provides thermal state via ProcessInfo
        // For now, return nominal
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().thermal_state_callback = Some(callback);
        // In a full implementation, we would register for
        // NSProcessInfoThermalStateDidChangeNotification
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(IosKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }
}

mod security {
    #![allow(non_upper_case_globals)]

    use super::*;

    #[link(name = "Security", kind = "framework")]
    unsafe extern "C" {
        pub static kSecClass: CFStringRef;
        pub static kSecClassInternetPassword: CFStringRef;
        pub static kSecAttrServer: CFStringRef;
        pub static kSecAttrAccount: CFStringRef;
        pub static kSecValueData: CFStringRef;
        pub static kSecReturnAttributes: CFStringRef;
        pub static kSecReturnData: CFStringRef;

        pub fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
        pub fn SecItemUpdate(query: CFDictionaryRef, attributes: CFDictionaryRef) -> OSStatus;
        pub fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;
        pub fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    }

    pub const ERR_SEC_SUCCESS: OSStatus = 0;
    pub const ERR_SEC_USER_CANCELED: OSStatus = -128;
    pub const ERR_SEC_ITEM_NOT_FOUND: OSStatus = -25300;
}
