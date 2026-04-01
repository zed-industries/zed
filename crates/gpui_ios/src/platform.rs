use crate::{
    IosDispatcher, IosDisplay, IosKeyboardLayout, IosTextSystem, IosWindow, ios_keyboard_mapper,
    display_link::DisplayLink,
    main_screen_bounds_and_scale,
    metal_renderer::{InstanceBufferPool, MetalRenderer},
};
use anyhow::Result;
use futures::channel::oneshot;
use gpui::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DevicePixels,
    ForegroundExecutor, Keymap, Menu, MenuItem, PathPromptOptions, Platform,
    PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, Task, ThermalState, WindowAppearance, WindowParams,
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    ffi::{CStr, c_void},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use crate::metal_renderer::Context as RendererContext;

// ─── iPadOS menu bar support ──────────────────────────────────────────────────

/// A single leaf entry in the Zed menu hierarchy, flattened for storage.
#[derive(Clone)]
enum IosMenuRow {
    Separator { identifier: String },
    Action { title: String, action_name: &'static str, identifier: String },
    SubmenuStart { title: String, identifier: String },
    SubmenuEnd,
}

/// A top-level menu stored for reconstruction when UIKit requests a menu rebuild.
#[derive(Clone)]
struct IosTopMenu {
    title: String,
    identifier: String,
    rows: Vec<IosMenuRow>,
}

/// Stores the current menu structure for UIKit to read when `buildMenuWithBuilder:` fires.
static IOS_MENUS: Mutex<Vec<IosTopMenu>> = Mutex::new(Vec::new());

thread_local! {
    /// Callback that dispatches a GPUI action by its type name string.
    /// Set from `zed_ios` init so it captures an `AsyncApp` without requiring `Send`.
    static MENU_ACTION_DISPATCHER: RefCell<Option<Box<dyn Fn(&str)>>> = RefCell::new(None);
}

/// Register a closure that will be called with a GPUI action name whenever the
/// user selects a menu item. Called once during app initialization.
pub fn set_menu_action_dispatcher(dispatcher: Box<dyn Fn(&str) + 'static>) {
    MENU_ACTION_DISPATCHER.with(|cell| *cell.borrow_mut() = Some(dispatcher));
}

/// Flatten a `Menu` and its nested items into `IosMenuRow`s, appending to `out`.
fn flatten_menu_items(menu: &Menu, parent_id: &str, out: &mut Vec<IosMenuRow>) {
    for item in &menu.items {
        match item {
            MenuItem::Separator => {
                let identifier = format!("{}.sep.{}", parent_id, out.len());
                out.push(IosMenuRow::Separator { identifier });
            }
            MenuItem::Action { name, action, .. } => {
                let title = name.to_string();
                let slug = title.to_lowercase().replace(' ', "-").replace("…", "");
                let identifier = format!("{}.{}", parent_id, slug);
                out.push(IosMenuRow::Action {
                    title,
                    action_name: action.name(),
                    identifier,
                });
            }
            MenuItem::Submenu(submenu) => {
                let slug = submenu.name.to_lowercase().replace(' ', "-");
                let sub_id = format!("{}.{}", parent_id, slug);
                out.push(IosMenuRow::SubmenuStart {
                    title: submenu.name.to_string(),
                    identifier: sub_id.clone(),
                });
                flatten_menu_items(submenu, &sub_id, out);
                out.push(IosMenuRow::SubmenuEnd);
            }
            MenuItem::SystemMenu(_) => {
                // No iPadOS equivalent; skip.
            }
        }
    }
}

/// Create an `NSString` from a Rust `str`. The returned pointer is autoreleased.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn menu_ns_string(s: &str) -> *mut Object {
    let ns: *mut Object = msg_send![class!(NSString), alloc];
    msg_send![ns, initWithBytes: s.as_ptr() length: s.len() encoding: 4usize]
}

/// Build a `UIMenu` for `rows` (a flat slice within the flat representation).
/// Returns `(UIMenu *, rows_consumed)`.
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn build_ui_menu(
    title: &str,
    identifier: &str,
    rows: &[IosMenuRow],
) -> (*mut Object, usize) {
    let children: *mut Object = msg_send![class!(NSMutableArray), array];
    let zed_action_sel: objc::runtime::Sel = sel!(zedMenuAction:);
    let mut i = 0;
    while i < rows.len() {
        match &rows[i] {
            IosMenuRow::Separator { identifier: sep_id } => {
                let sep_id_ns = menu_ns_string(sep_id);
                let nil_image: *mut Object = std::ptr::null_mut();
                let empty: *mut Object = msg_send![class!(NSMutableArray), array];
                let sep: *mut Object =
                    msg_send![class!(UIMenu), menuWithTitle: menu_ns_string("")
                                                      image: nil_image
                                                 identifier: sep_id_ns
                                                    options: 1usize  // UIMenuOptionsDisplayInline
                                                   children: empty];
                let _: () = msg_send![children, addObject: sep];
                i += 1;
            }
            IosMenuRow::Action { title, action_name, identifier } => {
                let title_ns = menu_ns_string(title);
                let id_ns = menu_ns_string(identifier);
                // NSDictionary @{@"action": action_name_ns}
                let key_ns = menu_ns_string("action");
                let val_ns = menu_ns_string(action_name);
                let prop: *mut Object = msg_send![class!(NSDictionary),
                    dictionaryWithObject: val_ns
                               forKey: key_ns];
                let cmd: *mut Object = msg_send![class!(UICommand),
                    commandWithTitle: title_ns
                             image: std::ptr::null::<Object>()
                            action: zed_action_sel
                      propertyList: prop];
                // Assign a stable UICommand identifier so iPadOS can track it.
                let _: () = msg_send![cmd, setDiscoverabilityTitle: id_ns];
                let _: () = msg_send![children, addObject: cmd];
                i += 1;
            }
            IosMenuRow::SubmenuStart { title, identifier } => {
                let (submenu, consumed) =
                    build_ui_menu(title, identifier, &rows[i + 1..]);
                let _: () = msg_send![children, addObject: submenu];
                i += 1 + consumed;
            }
            IosMenuRow::SubmenuEnd => {
                // Consumed by the SubmenuStart handler above; stop here.
                i += 1;
                break;
            }
        }
    }

    let title_ns = menu_ns_string(title);
    let id_ns = menu_ns_string(identifier);
    let nil_image: *mut Object = std::ptr::null_mut();
    let menu: *mut Object = msg_send![class!(UIMenu),
        menuWithTitle: title_ns
        image: nil_image
        identifier: id_ns
        options: 0usize
        children: children];
    (menu, i)
}

/// Build and install Zed's menus into the given `UIMenuBuilder*`.
///
/// # Safety
/// Must be called from the UIKit main thread inside `buildMenu(with:)`.
/// Build and install Zed's menus into a `UIMenuBuilder*` passed from Swift.
/// Accepts a `*mut c_void` so callers don't need to import the `objc` crate.
///
/// # Safety
/// Must be called on the UIKit main thread inside `buildMenu(with:)`.
/// `builder_ptr` must be a valid, non-null `UIMenuBuilder` instance.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn build_ios_menus(builder_ptr: *mut c_void) {
    unsafe {
        let builder = builder_ptr as *mut Object;
        let menus = IOS_MENUS.lock().clone();
        let app_menu_id = menu_ns_string("com.apple.menu.application");
        let mut after_id: *mut Object = app_menu_id;
        for top in &menus {
            let (ui_menu, _) = build_ui_menu(&top.title, &top.identifier, &top.rows);
            let _: () =
                msg_send![builder, insertSiblingMenu: ui_menu afterMenuForIdentifier: after_id];
            after_id = menu_ns_string(&top.identifier);
        }
    }
}

/// Called by `zedMenuAction:` on `ZedMetalView` to dispatch a menu-bar action.
/// `sender` is the `UICommand` whose `propertyList` contains `{"action": "<name>"}`.
///
/// # Safety
/// Must be called on the UIKit main thread.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn dispatch_menu_action(sender: *mut Object) {
    unsafe {
        let prop: *mut Object = msg_send![sender, propertyList];
        if prop.is_null() {
            return;
        }
        let key = menu_ns_string("action");
        let val: *mut Object = msg_send![prop, objectForKey: key];
        if val.is_null() {
            return;
        }
        let ptr: *const i8 = msg_send![val, UTF8String];
        if ptr.is_null() {
            return;
        }
        let action_name = match CStr::from_ptr(ptr).to_str() {
            Ok(s) => s,
            Err(_) => return,
        };
        MENU_ACTION_DISPATCHER.with(|cell| {
            if let Some(dispatcher) = cell.borrow().as_ref() {
                dispatcher(action_name);
            }
        });
    }
}

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
    renderer_context: RendererContext,
}

impl IosPlatform {
    pub fn new() -> Self {
        let dispatcher = Arc::new(IosDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        let text_system: Arc<dyn PlatformTextSystem> = Arc::new(IosTextSystem::new());

        let (bounds, _scale_factor) = main_screen_bounds_and_scale();
        let display: Rc<dyn PlatformDisplay> = Rc::new(IosDisplay::new(bounds));

        let renderer_context = Arc::new(Mutex::new(InstanceBufferPool::default()));

        Self {
            background_executor,
            foreground_executor,
            text_system,
            display,
            active_window: RefCell::new(None),
            callbacks: Mutex::new(IosPlatformCallbacks::default()),
            renderer_context,
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

    fn set_active_window_handle(&self, handle: AnyWindowHandle) {
        *self.active_window.borrow_mut() = Some(handle);
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        if self.active_window.borrow().is_some() {
            log::error!(
                "[gpui_ios] rejected second open_window call — iOS only supports a single window"
            );
            anyhow::bail!(
                "iOS only supports a single window; use Window::replace_root to change content"
            );
        }
        let window =
            IosWindow::new(params, self.display.clone(), self.renderer_context.clone())?;
        *self.active_window.borrow_mut() = Some(handle);
        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let key_window: *mut Object = msg_send![app, keyWindow];
            if key_window.is_null() {
                return WindowAppearance::Light;
            }
            let trait_collection: *mut Object = msg_send![key_window, traitCollection];
            // UIUserInterfaceStyleDark == 2
            let style: usize = msg_send![trait_collection, userInterfaceStyle];
            if style == 2 {
                WindowAppearance::Dark
            } else {
                WindowAppearance::Light
            }
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let url_str: *mut Object = msg_send![class!(NSString), alloc];
            let url_str: *mut Object = msg_send![url_str,
                initWithBytes: url.as_ptr()
                length: url.len()
                encoding: 4usize // NSUTF8StringEncoding
            ];
            let ns_url: *mut Object = msg_send![class!(NSURL), URLWithString: url_str];
            if !ns_url.is_null() {
                let empty_dict: *mut Object = msg_send![class!(NSDictionary), dictionary];
                let _: () = msg_send![app,
                    openURL: ns_url
                    options: empty_dict
                    completionHandler: std::ptr::null::<c_void>()
                ];
            }
            let _: () = msg_send![url_str, release];
        }
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

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        let top_menus: Vec<IosTopMenu> = menus
            .iter()
            .map(|menu| {
                let slug = menu.name.to_lowercase().replace(' ', "-");
                let identifier = format!("dev.zed.menu.{}", slug);
                let mut rows = Vec::new();
                flatten_menu_items(menu, &identifier, &mut rows);
                IosTopMenu { title: menu.name.to_string(), identifier, rows }
            })
            .collect();
        *IOS_MENUS.lock() = top_menus;
        unsafe {
            let system: *mut Object = msg_send![class!(UIMenuSystem), mainSystem];
            let _: () = msg_send![system, setNeedsRebuild];
        }
    }

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
        unsafe {
            let info: *mut Object = msg_send![class!(NSProcessInfo), processInfo];
            // NSProcessInfoThermalState: 0=Nominal, 1=Fair, 2=Serious, 3=Critical
            let state: isize = msg_send![info, thermalState];
            match state {
                1 => ThermalState::Fair,
                2 => ThermalState::Serious,
                3 => ThermalState::Critical,
                _ => ThermalState::Nominal,
            }
        }
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.lock().thermal_state_change = Some(callback);
    }

    fn compositor_name(&self) -> &'static str {
        "Metal (iOS)"
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle: *mut Object = msg_send![class!(NSBundle), mainBundle];
            let path: *mut Object = msg_send![bundle, bundlePath];
            let ptr: *const i8 = msg_send![path, UTF8String];
            if ptr.is_null() {
                return Err(anyhow::anyhow!("NSBundle.mainBundle.bundlePath returned nil"));
            }
            Ok(PathBuf::from(
                std::ffi::CStr::from_ptr(ptr).to_str()?,
            ))
        }
    }

    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        Err(anyhow::anyhow!(
            "path_for_auxiliary_executable: subprocess spawning is prohibited on iOS"
        ))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        crate::window::set_cursor_style(style);
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        true
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unsafe {
            let pasteboard: *mut Object = msg_send![class!(UIPasteboard), generalPasteboard];
            let string: *mut Object = msg_send![pasteboard, string];
            if string.is_null() {
                return None;
            }
            let ptr: *const i8 = msg_send![string, UTF8String];
            if ptr.is_null() {
                return None;
            }
            let text = std::ffi::CStr::from_ptr(ptr).to_str().ok()?.to_owned();
            Some(ClipboardItem::new_string(text))
        }
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        let Some(text) = item.text() else { return };
        unsafe {
            let pasteboard: *mut Object = msg_send![class!(UIPasteboard), generalPasteboard];
            let ns_str: *mut Object = msg_send![class!(NSString), alloc];
            let ns_str: *mut Object = msg_send![ns_str,
                initWithBytes: text.as_ptr()
                length: text.len()
                encoding: 4usize // NSUTF8StringEncoding
            ];
            let _: () = msg_send![pasteboard, setString: ns_str];
            let _: () = msg_send![ns_str, release];
        }
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        // SSH keys go to the iOS Keychain via SecItemAdd in crates/zed_ios.
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

// ─── Smoke-test rendering ─────────────────────────────────────────────────────

struct SmokeTestState {
    renderer: MetalRenderer,
    _display_link: DisplayLink,
}

thread_local! {
    static SMOKE_TEST: RefCell<Option<SmokeTestState>> = RefCell::new(None);
}

/// Boot the Metal renderer and start the CADisplayLink frame loop, rendering a
/// solid blue frame on every vsync.
///
/// This exercises the full Metal→UIKit pipeline (Metal renderer → CAMetalLayer
/// sublayer → UIView → UIWindow) without requiring a full GPUI `App` context.
/// Call this from `zed_ios_open_window` during Phase 1 development.
///
/// # Safety
/// Must be called on the main thread after UIKit has created the key UIWindow.
pub fn start_rendering() -> Result<()> {
    let pool = Arc::new(Mutex::new(InstanceBufferPool::default()));
    let mut renderer = MetalRenderer::new(pool);

    // ── Attach the Metal layer to the key UIWindow ──────────────────────────
    let (bounds, scale_factor) = main_screen_bounds_and_scale();
    let device_width = (f32::from(bounds.size.width) * scale_factor).round() as i32;
    let device_height = (f32::from(bounds.size.height) * scale_factor).round() as i32;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct CGPoint { x: f64, y: f64 }
    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct CGSize { width: f64, height: f64 }
    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct CGRect { origin: CGPoint, size: CGSize }

    unsafe {
        let app: *mut Object = msg_send![class!(UIApplication), sharedApplication];
        let key_window: *mut Object = msg_send![app, keyWindow];
        anyhow::ensure!(!key_window.is_null(), "no key UIWindow");

        let window_bounds: CGRect = msg_send![key_window, bounds];

        let superclass = class!(UIView);
        use objc::declare::ClassDecl;
        use std::sync::OnceLock;
        static VIEW_CLASS: OnceLock<&'static objc::runtime::Class> = OnceLock::new();
        let view_class = VIEW_CLASS.get_or_init(|| {
            ClassDecl::new("ZedSmokeView", superclass)
                .expect("ZedSmokeView already registered")
                .register()
        });

        let view: *mut Object = msg_send![*view_class, alloc];
        let view: *mut Object = msg_send![view, initWithFrame: window_bounds];
        let fill_mask: usize = (1 << 1) | (1 << 4);
        let _: () = msg_send![view, setAutoresizingMask: fill_mask];

        let layer_ptr = renderer.layer_ptr();
        let layer_frame = CGRect { origin: CGPoint::default(), size: window_bounds.size };
        let _: () = msg_send![layer_ptr, setFrame: layer_frame];
        let view_layer: *mut Object = msg_send![view, layer];
        let _: () = msg_send![view_layer, addSublayer: layer_ptr];
        let _: () = msg_send![key_window, addSubview: view];

        renderer.update_drawable_size(gpui::size(
            DevicePixels(device_width),
            DevicePixels(device_height),
        ));
    }

    // ── Start the CADisplayLink frame loop ───────────────────────────────────
    extern "C" fn on_frame(_data: *mut c_void) {
        SMOKE_TEST.with(|state| {
            if let Some(state) = state.borrow_mut().as_mut() {
                state.renderer.draw_clear();
            }
        });
    }

    let display_link = DisplayLink::new(std::ptr::null_mut(), on_frame);
    display_link.start();

    SMOKE_TEST.with(|state| {
        *state.borrow_mut() = Some(SmokeTestState {
            renderer,
            _display_link: display_link,
        });
    });

    Ok(())
}
