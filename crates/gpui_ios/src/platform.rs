use crate::{IosDisplay, IosWindow, id, nil, ns_string};
use anyhow::{Result, anyhow};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DummyKeyboardMapper,
    ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform, PlatformDisplay,
    PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Task,
    ThermalState, WindowAppearance, WindowParams,
};
use gpui_apple::metal_renderer;
use objc::{
    class,
    declare::ClassDecl,
    runtime::{BOOL, Class, Object, Sel, YES},
    sel, sel_impl,
};
use std::{
    cell::RefCell,
    ffi::CString,
    os::raw::{c_char, c_int},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Once},
};

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    fn UIApplicationMain(
        argc: c_int,
        argv: *mut *mut c_char,
        principal_class_name: id,
        delegate_class_name: id,
    ) -> c_int;
}

thread_local! {
    // `UIApplicationMain` takes over the main thread before GPUI's launch
    // callback can run, so park the callback here for the application
    // delegate to pick up in `application:didFinishLaunchingWithOptions:`.
    static FINISH_LAUNCHING: RefCell<Option<Box<dyn FnOnce()>>> = const { RefCell::new(None) };
}

fn application_delegate_class() -> &'static Class {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSObject))
            .expect("GPUIApplicationDelegate class is already registered");
        unsafe {
            decl.add_method(
                sel!(application:didFinishLaunchingWithOptions:),
                did_finish_launching as extern "C" fn(&Object, Sel, id, id) -> BOOL,
            );
        }
        decl.register();
    });
    Class::get("GPUIApplicationDelegate").expect("GPUIApplicationDelegate was just registered")
}

extern "C" fn did_finish_launching(_this: &Object, _: Sel, _application: id, _options: id) -> BOOL {
    if let Some(callback) = FINISH_LAUNCHING.take() {
        callback();
    }
    YES
}

pub struct IosPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<gpui_apple::CoreTextSystem>,
    renderer_context: metal_renderer::Context,
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(gpui_apple::AppleDispatcher::new());
        Self {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(gpui_apple::CoreTextSystem::new()),
            renderer_context: metal_renderer::Context::default(),
        }
    }
}

impl Default for IosPlatform {
    fn default() -> Self {
        Self::new()
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
        FINISH_LAUNCHING.set(Some(on_finish_launching));

        let arguments: Vec<CString> = std::env::args()
            .filter_map(|argument| CString::new(argument).ok())
            .collect();
        let mut argv: Vec<*mut c_char> = arguments
            .iter()
            .map(|argument| argument.as_ptr() as *mut c_char)
            .collect();

        unsafe {
            let delegate_class_name = ns_string(application_delegate_class().name());
            // Never returns; UIKit owns the main run loop from here on.
            UIApplicationMain(
                argv.len() as c_int,
                argv.as_mut_ptr(),
                nil,
                delegate_class_name,
            );
        }
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![Rc::new(IosDisplay::primary())]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(IosDisplay::primary()))
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        _options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(IosWindow::open(self.renderer_context.clone())))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme not implemented on iOS")))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow!("prompt_for_paths not implemented on iOS")))
            .ok();
        rx
    }

    fn prompt_for_new_path(
        &self,
        _directory: &Path,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(anyhow!("prompt_for_new_path not implemented on iOS")))
            .ok();
        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        false
    }

    fn reveal_path(&self, _path: &Path) {}

    fn open_with_system(&self, _path: &Path) {}

    fn on_quit(&self, _callback: Box<dyn FnMut()>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn gpui::Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn gpui::Action) -> bool>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, _callback: Box<dyn FnMut()>) {}

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow!(
            "path_for_auxiliary_executable not implemented on iOS"
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

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("write_credentials not implemented on iOS")))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("delete_credentials not implemented on iOS")))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(IosKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}
}

struct IosKeyboardLayout;

impl PlatformKeyboardLayout for IosKeyboardLayout {
    fn id(&self) -> &str {
        "ios"
    }

    fn name(&self) -> &str {
        "iOS"
    }
}
