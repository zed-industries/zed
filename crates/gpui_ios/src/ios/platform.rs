//! iOS Platform implementation.
//!
//! This implements the Platform trait for iOS using UIKit.
//! Key differences from macOS:
//! - Uses UIApplication instead of NSApplication
//! - No menu bar (iOS apps don't have traditional menus)
//! - No windowed mode (iOS apps are always fullscreen on their display)
//! - Touch-based input instead of mouse
//! - System keyboard handling differs significantly

use super::application::IosApplicationState;
use super::{IosDisplay, IosWindow};
use anyhow::anyhow;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, AppLifecyclePhase, BackgroundExecutor, ClipboardItem, CursorStyle,
    DummyKeyboardMapper, ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform,
    PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, Result, Task, ThermalState, WindowAppearance, WindowParams,
};
use gpui_apple::{
    AppleDispatcher, keychain,
    thermal::{self, ThermalObserver},
};
use objc2::{MainThreadMarker, MainThreadOnly, rc::Retained};
use objc2_foundation::{NSBundle, NSDictionary, NSString, NSURL};
use objc2_ui_kit::{
    UIApplication, UIPasteboard, UITraitEnvironment, UIUserInterfaceStyle, UIViewController,
};
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub struct IosPlatform(Rc<IosPlatformState>);

pub(crate) struct IosPlatformState {
    pub(super) application: IosApplicationState,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    thermal_observer: RefCell<Option<ThermalObserver>>,
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(AppleDispatcher::new());

        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(gpui_apple::AppleTextSystem::new());

        Self(Rc::new(IosPlatformState {
            application: IosApplicationState::default(),
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system,
            thermal_observer: RefCell::new(None),
        }))
    }

    fn root_view_controller(&self) -> Option<Retained<UIViewController>> {
        let window = if let Some(scene) = self.0.application.window_scene() {
            let windows = scene.windows();
            windows
                .iter()
                .find(|window| window.isKeyWindow())
                .or_else(|| windows.firstObject())
        } else {
            // Legacy hosts can create a GPUI window without supplying a scene.
            #[allow(deprecated)]
            UIApplication::sharedApplication(
                MainThreadMarker::new().expect("UIKit requires the main thread"),
            )
            .keyWindow()
        }?;
        window.rootViewController()
    }

    fn presented_view_controller(&self) -> Option<Retained<UIViewController>> {
        let mut view_controller = self.root_view_controller()?;
        while let Some(presented) = view_controller.presentedViewController() {
            view_controller = presented;
        }
        Some(view_controller)
    }

    fn dismiss_presented_browser(&self) {
        let mut view_controller = match self.root_view_controller() {
            Some(view_controller) => view_controller,
            None => return,
        };

        loop {
            let Some(presented) = view_controller.presentedViewController() else {
                return;
            };

            if presented.downcast_ref::<SFSafariViewController>().is_some() {
                presented.dismissViewControllerAnimated_completion(true, None);
                return;
            }

            view_controller = presented;
        }
    }

    fn open_url_with_system(url: &NSURL) {
        unsafe {
            let app = UIApplication::sharedApplication(
                MainThreadMarker::new().expect("UIKit requires the main thread"),
            );
            app.openURL_options_completionHandler(url, &NSDictionary::new(), None);
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
        self.0.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.0.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        super::application::run(&self.0, on_finish_launching);
    }

    fn quit(&self) {
        // iOS apps cannot programmatically quit - they can only be terminated by the user
        // or the system. We can suspend to background though.
        log::warn!("iOS apps cannot programmatically quit");
    }

    fn restart(&self, _binary_path: Option<PathBuf>, _arguments: Vec<std::ffi::OsString>) {
        // iOS apps cannot restart themselves
        log::warn!("iOS apps cannot restart themselves");
    }

    fn activate(&self, _ignoring_other_apps: bool) {
        self.dismiss_presented_browser();
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
        let window = Box::new(IosWindow::new(handle, options, &self.0)?);
        window.register();
        Ok(window)
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let Some(controller) = self.root_view_controller() else {
                return WindowAppearance::Light;
            };
            match controller.traitCollection().userInterfaceStyle() {
                UIUserInterfaceStyle::Dark => WindowAppearance::Dark,
                _ => WindowAppearance::Light,
            }
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let Some(native_url) = NSURL::URLWithString(&NSString::from_str(url)) else {
                log::error!("GPUI iOS: Could not parse URL: {url}");
                return;
            };

            if url.starts_with("https://") || url.starts_with("http://") {
                if let Some(view_controller) = self.presented_view_controller() {
                    let browser = SFSafariViewController::init_with_url(
                        SFSafariViewController::alloc(view_controller.mtm()),
                        &native_url,
                    );
                    view_controller.presentViewController_animated_completion(&browser, true, None);
                    return;
                }
            }

            Self::open_url_with_system(&native_url);
        }
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.application.open_urls.set(callback);
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

    fn on_quit(&self, mut callback: Box<dyn FnMut() -> bool>) {
        self.0.application.quit.set(Box::new(move || {
            // UIKit's termination notification cannot be vetoed.
            callback();
        }));
    }

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles app reopening through scene lifecycle
    }

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_sleep(&self, _callback: Box<dyn FnMut()>) {}

    fn prevent_idle_sleep(&self, _reason: &str) -> Task<Result<gpui::ActivityGuard>> {
        Task::ready(Err(anyhow!(
            "Idle sleep prevention is not implemented for iOS"
        )))
    }

    fn on_app_lifecycle(&self, callback: Box<dyn FnMut(AppLifecyclePhase)>) {
        self.0.application.app_lifecycle.set(callback);
    }

    fn on_memory_warning(&self, callback: Box<dyn FnMut()>) {
        self.0.application.memory_warning.set(callback);
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
        Ok(PathBuf::from(
            NSBundle::mainBundle().bundlePath().to_string(),
        ))
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
        let pasteboard = UIPasteboard::generalPasteboard();
        if let Some(text) = item.text() {
            unsafe { pasteboard.setString(Some(&NSString::from_str(&text))) };
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let pasteboard = UIPasteboard::generalPasteboard();
        Some(ClipboardItem::new_string(
            unsafe { pasteboard.string() }?.to_string(),
        ))
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor()
            .spawn(async move { Ok(keychain::write_credentials(&url, &username, &password)?) })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let url = url.to_string();
        self.background_executor()
            .spawn(async move { Ok(keychain::read_credentials(&url)?) })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let url = url.to_string();
        self.background_executor()
            .spawn(async move { Ok(keychain::delete_credentials(&url)?) })
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {
        // iOS handles keyboard layout changes differently
    }

    fn thermal_state(&self) -> ThermalState {
        thermal::thermal_state()
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        let observer = ThermalObserver::new(&self.0.foreground_executor, callback);
        self.0.thermal_observer.replace(Some(observer));
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(IosKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }
}

// objc2-safari-services 0.3.2 only includes the macOS SafariServices API.
// Keep the missing iOS initializer typed here until the generated crate includes it.
#[link(name = "SafariServices", kind = "framework")]
unsafe extern "C" {}

objc2::extern_class!(
    #[unsafe(super(
        UIViewController,
        objc2_ui_kit::UIResponder,
        objc2_foundation::NSObject
    ))]
    #[thread_kind = MainThreadOnly]
    struct SFSafariViewController;
);

impl SFSafariViewController {
    objc2::extern_methods!(
        #[unsafe(method(initWithURL:))]
        #[unsafe(method_family = init)]
        unsafe fn init_with_url(this: objc2::rc::Allocated<Self>, url: &NSURL) -> Retained<Self>;
    );
}
