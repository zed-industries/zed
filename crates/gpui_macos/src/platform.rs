use crate::{
    AppleActivity, AppleDispatcher, BoolExt, MacDisplay, MacKeyboardLayout, MacKeyboardMapper,
    MacWindow, events::key_to_native, main_thread_marker, ns_string, pasteboard::Pasteboard,
    renderer, set_active_window_cursor_style,
};
use anyhow::{Context as _, anyhow};
use block2::RcBlock;
use core_foundation::{runloop::CFRunLoopRun, string::CFStringRef};
use ctor::ctor;
use dispatch2::DispatchQueue;
use futures::channel::oneshot;
use gpui::{
    Action, ActivationPolicy, ActivityGuard, AnyWindowHandle, BackgroundExecutor, ClipboardItem,
    CursorStyle, ForegroundExecutor, GraphicalEnvironment, KeyContext, Keymap, Menu, MenuItem,
    OsMenu, OwnedMenu, PathPromptOptions, Platform, PlatformDisplay, PlatformKeyboardLayout,
    PlatformKeyboardMapper, PlatformTextSystem, PlatformWindow, Result, SystemMenuType, Task,
    ThermalState, WindowAppearance, WindowKind, WindowParams, WindowingRequest,
    popup::PopupNotSupportedError,
};
use gpui_apple::{
    keychain,
    thermal::{self, ThermalObserver},
};
use gpui_util::{ResultExt, new_std_command};
use itertools::Itertools;
use objc2::{
    Encode, MainThreadMarker, MainThreadOnly, class, msg_send,
    rc::{Retained, autoreleasepool},
    runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Ivar, Sel},
    sel,
};
use objc2_app_kit::{
    NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSAppearanceNameVibrantDark,
    NSAppearanceNameVibrantLight, NSApplication, NSApplicationActivationPolicy,
    NSControlStateValueOn, NSCursor, NSDocumentController, NSEventModifierFlags, NSMenu,
    NSMenuItem, NSModalResponse, NSModalResponseOK, NSOpenPanel, NSSavePanel, NSScroller,
    NSScrollerStyle, NSWorkspace, NSWorkspaceDidWakeNotification, NSWorkspaceWillSleepNotification,
};
use objc2_foundation::{
    NSActivityOptions, NSArray, NSBundle, NSError, NSInteger, NSProcessInfo, NSString, NSURL,
    NSUTF8StringEncoding,
};
use parking_lot::Mutex;
use semver::Version;
use std::{
    cell::Cell,
    ffi::{CStr, OsStr, c_void},
    os::{raw::c_char, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
    ptr,
    rc::Rc,
    slice, str,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

const MAC_PLATFORM_IVAR: &CStr = c"platform";
static mut APP_CLASS: *const AnyClass = ptr::null();
static mut APP_DELEGATE_CLASS: *const AnyClass = ptr::null();

#[ctor(unsafe)]
unsafe fn build_classes() {
    unsafe {
        APP_CLASS = {
            let mut decl = ClassBuilder::new(c"GPUIApplication", class!(NSApplication))
                .expect("failed to register GPUIApplication");
            decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
            decl.register()
        }
    };
    unsafe {
        APP_DELEGATE_CLASS = {
            let mut decl = ClassBuilder::new(c"GPUIApplicationDelegate", class!(NSResponder))
                .expect("failed to register GPUIApplicationDelegate");
            decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
            decl.add_method(
                sel!(applicationWillFinishLaunching:),
                will_finish_launching as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(applicationDidFinishLaunching:),
                did_finish_launching as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(applicationShouldHandleReopen:hasVisibleWindows:),
                should_handle_reopen as extern "C-unwind" fn(_, Sel, *mut AnyObject, Bool) -> Bool,
            );
            decl.add_method(
                sel!(applicationWillTerminate:),
                will_terminate as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(handleGPUIMenuItem:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            // Add menu item handlers so that OS save panels have the correct key commands
            decl.add_method(
                sel!(cut:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(copy:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(paste:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(selectAll:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(undo:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(redo:),
                handle_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(validateMenuItem:),
                validate_menu_item as extern "C-unwind" fn(_, Sel, *mut AnyObject) -> Bool,
            );
            decl.add_method(
                sel!(menuWillOpen:),
                menu_will_open as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );
            decl.add_method(
                sel!(applicationDockMenu:),
                handle_dock_menu as extern "C-unwind" fn(_, Sel, *mut AnyObject) -> *mut AnyObject,
            );
            decl.add_method(
                sel!(application:openURLs:),
                open_urls as extern "C-unwind" fn(_, Sel, *mut AnyObject, *mut AnyObject),
            );

            decl.add_method(
                sel!(onKeyboardLayoutChange:),
                on_keyboard_layout_change as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );

            decl.add_method(
                sel!(onScreenParametersChange:),
                on_screen_parameters_change as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );

            decl.add_method(
                sel!(onSystemSleep:),
                on_system_sleep as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );

            decl.add_method(
                sel!(onSystemWake:),
                on_system_wake as extern "C-unwind" fn(_, Sel, *mut AnyObject),
            );

            decl.register()
        }
    }
}

pub struct MacPlatform(Mutex<MacPlatformState>, MainThreadMarker);

pub(crate) struct MacPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    renderer_context: renderer::Context,
    headless: bool,
    activation_policy: ActivationPolicy,
    application_created: bool,
    general_pasteboard: Pasteboard,
    find_pasteboard: Pasteboard,
    reopen: Option<Box<dyn FnMut()>>,
    on_keyboard_layout_change: Option<Box<dyn FnMut()>>,
    thermal_observer: Option<ThermalObserver>,
    on_displays_changed: Option<Box<dyn FnMut()>>,
    on_system_sleep: Option<Box<dyn FnMut()>>,
    on_system_wake: Option<Box<dyn FnMut()>>,
    system_power_observers_registered: bool,
    quit: Option<Box<dyn FnMut() -> bool>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
    dock_menu: Option<Retained<NSMenu>>,
    menus: Option<Vec<OwnedMenu>>,
    keyboard_mapper: Rc<MacKeyboardMapper>,
    /// Mirrors `[NSCursor setHiddenUntilMouseMoves:]` state, which AppKit doesn't expose.
    cursor_visible: Arc<AtomicBool>,
    system_notifications: crate::system_notifications::SystemNotificationState,
}

/// The activation policy that stands for a windowing mode: an app is headless while it has no
/// Dock icon or menu bar.
fn activation_policy_for(request: &WindowingRequest) -> ActivationPolicy {
    match request {
        WindowingRequest::Headless => ActivationPolicy::Accessory,
        WindowingRequest::Windowed(_) => ActivationPolicy::Regular,
    }
}

fn native_activation_policy(policy: ActivationPolicy) -> NSApplicationActivationPolicy {
    match policy {
        ActivationPolicy::Regular => NSApplicationActivationPolicy::Regular,
        ActivationPolicy::Accessory => NSApplicationActivationPolicy::Accessory,
    }
}

impl MacPlatform {
    pub fn new(headless: bool) -> Self {
        let marker = main_thread_marker().expect("Mac platform not created on main thread");
        let dispatcher = Arc::new(AppleDispatcher::new());

        #[cfg(feature = "font-kit")]
        let text_system = Arc::new(gpui_apple::AppleTextSystem::new());

        #[cfg(not(feature = "font-kit"))]
        let text_system = {
            if !headless {
                log::warn!(
                    "gpui_macos was compiled without the `font-kit` feature, so no text will be rendered."
                );
            }
            Arc::new(gpui::NoopTextSystem::new())
        };

        let keyboard_layout = MacKeyboardLayout::new();
        let keyboard_mapper = Rc::new(MacKeyboardMapper::new(keyboard_layout.id()));

        let state = Mutex::new(MacPlatformState {
            headless,
            activation_policy: ActivationPolicy::Regular,
            application_created: false,
            text_system,
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            renderer_context: renderer::Context::default(),
            general_pasteboard: Pasteboard::general(),
            find_pasteboard: Pasteboard::find(),
            reopen: None,
            quit: None,
            menu_command: None,
            validate_menu_command: None,
            will_open_menu: None,
            menu_actions: Default::default(),
            open_urls: None,
            finish_launching: None,
            dock_menu: None,
            on_keyboard_layout_change: None,
            thermal_observer: None,
            on_displays_changed: None,
            on_system_sleep: None,
            on_system_wake: None,
            system_power_observers_registered: false,
            menus: None,
            keyboard_mapper,
            cursor_visible: Arc::new(AtomicBool::new(true)),
            system_notifications: crate::system_notifications::SystemNotificationState::new(),
        });
        Self(state, marker)
    }

    unsafe fn create_menu_bar(
        &self,
        menus: &Vec<Menu>,
        delegate: *mut AnyObject,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> Retained<NSMenu> {
        unsafe {
            let application_menu = NSMenu::new(self.1);
            let _: () = msg_send![&application_menu, setDelegate: delegate];

            for menu_config in menus {
                let menu = NSMenu::new(self.1);
                let menu_title = NSString::from_str(&menu_config.name);
                menu.setTitle(&menu_title);
                let _: () = msg_send![&menu, setDelegate: delegate];

                for item_config in &menu_config.items {
                    menu.addItem(&self.create_menu_item(item_config, delegate, actions, keymap));
                }

                let menu_item = NSMenuItem::new(self.1);
                menu_item.setTitle(&menu_title);
                menu_item.setSubmenu(Some(&menu));
                application_menu.addItem(&menu_item);

                if menu_config.name == "Window" {
                    let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
                    let _: () = msg_send![app, setWindowsMenu: &*menu];
                }
            }

            application_menu
        }
    }

    unsafe fn create_dock_menu(
        &self,
        menu_items: Vec<MenuItem>,
        delegate: *mut AnyObject,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> Retained<NSMenu> {
        unsafe {
            let dock_menu = NSMenu::new(self.1);
            let _: () = msg_send![&dock_menu, setDelegate: delegate];
            for item_config in menu_items {
                dock_menu.addItem(&self.create_menu_item(&item_config, delegate, actions, keymap));
            }

            dock_menu
        }
    }

    unsafe fn create_menu_item(
        &self,
        item: &MenuItem,
        delegate: *mut AnyObject,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> Retained<NSMenuItem> {
        static DEFAULT_CONTEXT: OnceLock<Vec<KeyContext>> = OnceLock::new();

        unsafe {
            match item {
                MenuItem::Separator => NSMenuItem::separatorItem(self.1),
                MenuItem::Action {
                    name,
                    action,
                    os_action,
                    checked,
                    disabled,
                } => {
                    // Note that this is intentionally using earlier bindings, whereas typically
                    // later ones take display precedence. See the discussion on
                    // https://github.com/zed-industries/zed/issues/23621
                    let keystrokes = keymap
                        .bindings_for_action(action.as_ref())
                        .find_or_first(|binding| {
                            binding.predicate().is_none_or(|predicate| {
                                predicate.eval(DEFAULT_CONTEXT.get_or_init(|| {
                                    let mut workspace_context =
                                        KeyContext::new_with_defaults(gpui::OperatingSystem::Mac);
                                    workspace_context.add("Workspace");
                                    let mut pane_context =
                                        KeyContext::new_with_defaults(gpui::OperatingSystem::Mac);
                                    pane_context.add("Pane");
                                    let mut editor_context =
                                        KeyContext::new_with_defaults(gpui::OperatingSystem::Mac);
                                    editor_context.add("Editor");

                                    pane_context.extend(&editor_context);
                                    workspace_context.extend(&pane_context);
                                    vec![workspace_context]
                                }))
                            })
                        })
                        .map(|binding| binding.keystrokes());

                    let selector = match os_action {
                        Some(gpui::OsAction::Cut) => sel!(cut:),
                        Some(gpui::OsAction::Copy) => sel!(copy:),
                        Some(gpui::OsAction::Paste) => sel!(paste:),
                        Some(gpui::OsAction::SelectAll) => sel!(selectAll:),
                        // "undo:" and "redo:" are always disabled in our case, as
                        // we don't have a NSTextView/NSTextField to enable them on.
                        Some(gpui::OsAction::Undo) => sel!(handleGPUIMenuItem:),
                        Some(gpui::OsAction::Redo) => sel!(handleGPUIMenuItem:),
                        None => sel!(handleGPUIMenuItem:),
                    };

                    let item;
                    if let Some(keystrokes) = keystrokes {
                        if keystrokes.len() == 1 {
                            let keystroke = &keystrokes[0];
                            let mut mask = NSEventModifierFlags::empty();
                            for (modifier, flag) in &[
                                (
                                    keystroke.modifiers().platform,
                                    NSEventModifierFlags::Command,
                                ),
                                (keystroke.modifiers().control, NSEventModifierFlags::Control),
                                (keystroke.modifiers().alt, NSEventModifierFlags::Option),
                                (keystroke.modifiers().shift, NSEventModifierFlags::Shift),
                            ] {
                                if *modifier {
                                    mask |= *flag;
                                }
                            }

                            item = NSMenuItem::initWithTitle_action_keyEquivalent(
                                NSMenuItem::alloc(self.1),
                                &NSString::from_str(name),
                                Some(selector),
                                &NSString::from_str(key_to_native(keystroke.key()).as_ref()),
                            );
                            if Self::os_version() >= Version::new(12, 0, 0) {
                                item.setAllowsAutomaticKeyEquivalentLocalization(false);
                            }
                            item.setKeyEquivalentModifierMask(mask);
                        } else {
                            item = NSMenuItem::initWithTitle_action_keyEquivalent(
                                NSMenuItem::alloc(self.1),
                                &NSString::from_str(name),
                                Some(selector),
                                &NSString::from_str(""),
                            );
                        }
                    } else {
                        item = NSMenuItem::initWithTitle_action_keyEquivalent(
                            NSMenuItem::alloc(self.1),
                            &NSString::from_str(name),
                            Some(selector),
                            &NSString::from_str(""),
                        );
                    }

                    if *checked {
                        item.setState(NSControlStateValueOn);
                    }
                    item.setEnabled(!disabled);

                    let tag = actions.len() as NSInteger;
                    item.setTag(tag);
                    actions.push(action.boxed_clone());
                    item
                }
                MenuItem::Submenu(Menu {
                    name,
                    items,
                    disabled,
                }) => {
                    let item = NSMenuItem::new(self.1);
                    let submenu = NSMenu::new(self.1);
                    let _: () = msg_send![&submenu, setDelegate: delegate];
                    for item in items {
                        submenu.addItem(&self.create_menu_item(item, delegate, actions, keymap));
                    }
                    item.setSubmenu(Some(&submenu));
                    item.setEnabled(!disabled);
                    item.setTitle(&NSString::from_str(name));
                    item
                }
                MenuItem::SystemMenu(OsMenu { name, menu_type }) => {
                    let item = NSMenuItem::new(self.1);
                    let submenu = NSMenu::new(self.1);
                    let _: () = msg_send![&submenu, setDelegate: delegate];
                    item.setSubmenu(Some(&submenu));
                    item.setTitle(&NSString::from_str(name));

                    match menu_type {
                        SystemMenuType::Services => {
                            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
                            let _: () = msg_send![app, setServicesMenu: &*item];
                        }
                    }

                    item
                }
            }
        }
    }

    fn os_version() -> Version {
        let version = NSProcessInfo::processInfo().operatingSystemVersion();
        Version::new(
            version.majorVersion as u64,
            version.minorVersion as u64,
            version.patchVersion as u64,
        )
    }

    // Before the app finishes launching there is no delegate yet;
    // `did_finish_launching` registers the observers then.
    fn ensure_system_power_observers(&self) {
        if self.0.lock().system_power_observers_registered {
            return;
        }

        // The shared application must be created through `APP_CLASS`, or `run`
        // finds a plain `NSApplication` without the `platform` ivar; only the
        // delegate lookup goes through the typed binding.
        // SAFETY: APP_CLASS is registered during startup and `sharedApplication`
        // returns a live NSApplication instance.
        let delegate = unsafe {
            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
            (*(app as *const NSApplication)).delegate()
        };
        if let Some(delegate) = delegate {
            register_system_power_observers(delegate.as_ref());
            self.0.lock().system_power_observers_registered = true;
        }
    }
}

impl Platform for MacPlatform {
    fn platform_info(&self) -> gpui::PlatformInfo {
        gpui::PlatformInfo {
            os: gpui::OperatingSystem::Mac,
            target: gpui::PlatformTarget::Native,
        }
    }

    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> gpui::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        let mut state = self.0.lock();
        if state.headless {
            drop(state);
            on_finish_launching();
            unsafe { CFRunLoopRun() };
        } else {
            state.finish_launching = Some(on_finish_launching);
            drop(state);
        }

        unsafe {
            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
            // An accessory app must not register in the Dock during launch, so its
            // policy has to precede the run loop. `Regular` is applied in
            // `did_finish_launching` instead: setting it this early leaves the menu
            // bar of an unbundled app launched from a terminal unclickable.
            let policy = {
                let mut state = self.0.lock();
                state.application_created = true;
                state.activation_policy
            };
            if policy == ActivationPolicy::Accessory {
                let _: Bool = msg_send![app, setActivationPolicy: native_activation_policy(policy)];
            }
            let app_delegate: *mut AnyObject = msg_send![APP_DELEGATE_CLASS, new];
            let _: () = msg_send![app, setDelegate: app_delegate];

            let self_ptr = self as *const Self as *mut c_void;
            mac_platform_ivar(&*app)
                .load_ptr::<*mut c_void>(&*app)
                .write(self_ptr);
            mac_platform_ivar(&*app_delegate)
                .load_ptr::<*mut c_void>(&*app_delegate)
                .write(self_ptr);

            autoreleasepool(|_| {
                let _: () = msg_send![app, run];
            });

            mac_platform_ivar(&*app)
                .load_ptr::<*mut c_void>(&*app)
                .write(ptr::null_mut());
            let app_delegate: *mut AnyObject = msg_send![app, delegate];
            mac_platform_ivar(&*app_delegate)
                .load_ptr::<*mut c_void>(&*app_delegate)
                .write(ptr::null_mut());
        }
        self.0.lock().application_created = false;
    }

    fn quit(&self) {
        // Quitting the app causes us to close windows, which invokes `Window::on_close` callbacks
        // synchronously before this method terminates. If we call `Platform::quit` while holding a
        // borrow of the app state (which most of the time we will do), we will end up
        // double-borrowing the app state in the `on_close` callbacks for our open windows. To solve
        // this, we make quitting the application asynchronous so that we aren't holding borrows to
        // the app state on the stack when we actually terminate the app.

        unsafe {
            DispatchQueue::main().exec_async_f(ptr::null_mut(), quit);
        }

        extern "C" fn quit(_: *mut c_void) {
            unsafe {
                let app: *mut AnyObject = msg_send![class!(NSApplication), sharedApplication];
                let _: () = msg_send![app, terminate: ptr::null_mut::<AnyObject>()];
            }
        }
    }

    fn restart(&self, binary_path: Option<PathBuf>, arguments: Vec<std::ffi::OsString>) {
        use std::os::unix::process::CommandExt as _;

        let app_pid = std::process::id().to_string();
        let app_path = binary_path
            .or_else(|| {
                self.app_path()
                    .ok()
                    // When the app is not bundled, `app_path` returns the
                    // directory containing the executable. Disregard this
                    // and get the path to the executable itself.
                    .and_then(|path| (path.extension()?.to_str()? == "app").then_some(path))
            })
            .unwrap_or_else(|| std::env::current_exe().unwrap());

        // Wait until this process has exited and then re-open this path.
        let script = r#"
            while kill -0 $0 2> /dev/null; do
                sleep 0.1
            done
            app_path="$1"
            shift
            if (($# > 0)); then
                open "$app_path" --args "$@"
            else
                open "$app_path"
            fi
        "#;

        #[allow(
            clippy::disallowed_methods,
            reason = "We are restarting ourselves, using std command thus is fine"
        )]
        let restart_process = new_std_command("/bin/bash")
            .arg("-c")
            .arg(script)
            .arg(app_pid)
            .arg(app_path)
            .args(arguments)
            .process_group(0)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
    }

    #[allow(deprecated)]
    fn activate(&self, ignoring_other_apps: bool) {
        let app = NSApplication::sharedApplication(self.1);
        app.activateIgnoringOtherApps(ignoring_other_apps);
    }

    fn set_activation_policy(&self, policy: ActivationPolicy) {
        let mut state = self.0.lock();
        state.activation_policy = policy;
        let should_apply = state.application_created && !state.headless;
        drop(state);
        if should_apply {
            unsafe {
                let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
                let _: Bool = msg_send![app, setActivationPolicy: native_activation_policy(policy)];
            }
        }
    }

    fn set_initial_windowing(&self, request: WindowingRequest) {
        self.set_activation_policy(activation_policy_for(&request));
    }

    fn request_windowing(&self, request: WindowingRequest) -> Task<Result<()>> {
        let state = self.0.lock();
        if state.headless {
            return Task::ready(Err(anyhow!(
                "a platform created headless has no application to switch windowing modes"
            )));
        }
        let policy = activation_policy_for(&request);
        if state.activation_policy == policy {
            return Task::ready(Err(match request {
                WindowingRequest::Headless => anyhow!("already headless"),
                WindowingRequest::Windowed(_) => anyhow!("already windowed"),
            }));
        }
        drop(state);
        self.set_activation_policy(policy);
        Task::ready(Ok(()))
    }

    fn graphical_environment(&self) -> Option<GraphicalEnvironment> {
        let state = self.0.lock();
        (!state.headless && state.activation_policy == ActivationPolicy::Regular)
            .then(GraphicalEnvironment::detect)
    }

    fn hide(&self) {
        NSApplication::sharedApplication(self.1).hide(None);
    }

    fn hide_other_apps(&self) {
        NSApplication::sharedApplication(self.1).hideOtherApplications(None);
    }

    fn unhide_other_apps(&self) {
        NSApplication::sharedApplication(self.1).unhideAllApplications(None);
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(MacDisplay::primary()))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        MacDisplay::all()
            .map(|screen| Rc::new(screen) as Rc<_>)
            .collect()
    }

    fn on_displays_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_displays_changed = Some(callback);
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        let min_version = objc2_foundation::NSOperatingSystemVersion {
            majorVersion: 12,
            minorVersion: 3,
            patchVersion: 0,
        };
        NSProcessInfo::processInfo().isOperatingSystemAtLeastVersion(min_version)
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn gpui::ScreenCaptureSource>>>> {
        crate::screen_capture::get_sources(self.1)
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        MacWindow::active_window()
    }

    // Returns the windows ordered front-to-back, meaning that the active
    // window is the first one in the returned vec.
    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        Some(MacWindow::ordered_windows())
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        // Native popups are not implemented on macOS yet. Rejecting lets callers fall back to
        // gpui's in-window popovers.
        if let WindowKind::AnchoredPopup(_) = options.kind {
            return Err(PopupNotSupportedError.into());
        }

        let (cursor_visible, foreground_executor, background_executor, renderer_context) = {
            let guard = self.0.lock();
            (
                guard.cursor_visible.clone(),
                guard.foreground_executor.clone(),
                guard.background_executor.clone(),
                guard.renderer_context.clone(),
            )
        };

        Ok(Box::new(MacWindow::open(
            handle,
            options,
            cursor_visible,
            foreground_executor,
            background_executor,
            renderer_context,
            self.1,
        )?))
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let app = NSApplication::sharedApplication(self.1);
            let appearance: Option<Retained<NSAppearance>> = msg_send![&app, effectiveAppearance];
            crate::window_appearance::window_appearance_from_native(appearance.as_deref())
        }
    }

    fn set_window_appearance(&self, appearance: Option<WindowAppearance>) {
        unsafe {
            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
            // `None` clears the override by setting a nil appearance, so the app
            // falls back to tracking the system-wide light/dark setting.
            let ns_appearance = appearance.and_then(|appearance| {
                let name = match appearance {
                    WindowAppearance::Light => NSAppearanceNameAqua,
                    WindowAppearance::Dark => NSAppearanceNameDarkAqua,
                    WindowAppearance::VibrantLight => NSAppearanceNameVibrantLight,
                    WindowAppearance::VibrantDark => NSAppearanceNameVibrantDark,
                };
                NSAppearance::appearanceNamed(name)
            });
            let _: () = msg_send![app, setAppearance: ns_appearance.as_deref()];
        }
    }

    fn open_url(&self, url: &str) {
        let Some(url) = NSURL::URLWithString(&NSString::from_str(url)) else {
            log::error!("Failed to create NSURL from string: {url}");
            return;
        };
        NSWorkspace::sharedWorkspace().openURL(&url);
    }

    fn register_url_scheme(&self, scheme: &str) -> Task<anyhow::Result<()>> {
        // API only available post Monterey
        // https://developer.apple.com/documentation/appkit/nsworkspace/3753004-setdefaultapplicationaturl
        let (done_tx, done_rx) = oneshot::channel();
        if Self::os_version() < Version::new(12, 0, 0) {
            return Task::ready(Err(anyhow!(
                "macOS 12.0 or later is required to register URL schemes"
            )));
        }

        let Some(bundle_id) = NSBundle::mainBundle().bundleIdentifier() else {
            return Task::ready(Err(anyhow!("Can only register URL scheme in bundled apps")));
        };

        let workspace = NSWorkspace::sharedWorkspace();
        let Some(app) = workspace.URLForApplicationWithBundleIdentifier(&bundle_id) else {
            return Task::ready(Err(anyhow!(
                "Cannot register URL scheme until app is installed"
            )));
        };

        let scheme = NSString::from_str(scheme);

        let done_tx = Cell::new(Some(done_tx));
        let handler = RcBlock::new(move |error: *mut NSError| {
            let result = if let Some(error) = unsafe { error.as_ref() } {
                Err(anyhow!(
                    "Failed to register: {}",
                    error.localizedDescription()
                ))
            } else {
                Ok(())
            };

            if let Some(done_tx) = done_tx.take() {
                _ = done_tx.send(result);
            }
        });

        workspace.setDefaultApplicationAtURL_toOpenURLsWithScheme_completionHandler(
            &app,
            &scheme,
            Some(&handler),
        );

        self.background_executor().spawn(async { done_rx.await? })
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                // AppKit can return nil despite the nonnull annotation used by objc2's
                // generated binding. This runs on the main thread via the foreground executor.
                // TODO: Switch back to NSOpenPanel::openPanel when objc2-app-kit 0.4
                // is published and adopted; it fixes the return type to be nullable:
                // https://github.com/madsmtm/objc2/commit/b92857df57aa85dc41e9887534b1ccfa3fe902e4
                let panel: Option<Retained<NSOpenPanel>> =
                    unsafe { msg_send![class!(NSOpenPanel), openPanel] };
                let Some(panel) = panel else {
                    // The receiver may have been dropped while the panel was being created.
                    _ = done_tx.send(Err(anyhow!("macOS could not create the open dialog")));
                    return;
                };
                panel.setCanChooseDirectories(options.directories);
                panel.setCanChooseFiles(options.files);
                panel.setAllowsMultipleSelection(options.multiple);

                panel.setCanCreateDirectories(true);
                panel.setResolvesAliases(false);

                let done_tx = Cell::new(Some(done_tx));
                let handler = RcBlock::new({
                    let panel = panel.clone();
                    move |response: NSModalResponse| {
                        let Some(done_tx) = done_tx.take() else {
                            return;
                        };

                        let result = (response == NSModalResponseOK).then(|| {
                            panel
                                .URLs()
                                .iter()
                                .filter(|url| url.isFileURL())
                                .filter_map(|url| url.to_file_path())
                                .collect::<Vec<_>>()
                        });
                        _ = done_tx.send(Ok(result));
                    }
                });

                if let Some(prompt) = options.prompt {
                    panel.setPrompt(Some(&NSString::from_str(prompt.as_str())));
                }

                panel.beginWithCompletionHandler(&handler);
            })
            .detach();
        done_rx
    }

    fn prompt_for_new_path(
        &self,
        directory: &Path,
        suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let url = NSURL::from_directory_path(directory);
        let suggested_name = suggested_name.map(NSString::from_str);
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                // Like openPanel, savePanel may return nil despite its nonnull annotation.
                // The foreground executor ensures this call runs on the main thread.
                // TODO: Switch back to NSSavePanel::savePanel when objc2-app-kit 0.4
                // is published and adopted; it fixes the return type to be nullable:
                // https://github.com/madsmtm/objc2/commit/b92857df57aa85dc41e9887534b1ccfa3fe902e4
                let panel: Option<Retained<NSSavePanel>> =
                    unsafe { msg_send![class!(NSSavePanel), savePanel] };
                let Some(panel) = panel else {
                    // The receiver may have been dropped while the panel was being created.
                    _ = done_tx.send(Err(anyhow!("macOS could not create the save dialog")));
                    return;
                };
                panel.setDirectoryURL(url.as_deref());

                if let Some(suggested_name) = suggested_name {
                    panel.setNameFieldStringValue(&suggested_name);
                }

                let done_tx = Cell::new(Some(done_tx));
                let handler = RcBlock::new({
                    let panel = panel.clone();
                    move |response: NSModalResponse| {
                        let Some(done_tx) = done_tx.take() else {
                            return;
                        };

                        let result = if response == NSModalResponseOK {
                            panel
                                .URL()
                                .filter(|url| url.isFileURL())
                                .and_then(|url| url.to_file_path())
                                .map(|mut path| {
                                    let Some(filename) = path.file_name() else {
                                        return path;
                                    };
                                    let chunks = filename
                                        .as_bytes()
                                        .split(|&b| b == b'.')
                                        .collect::<Vec<_>>();

                                    // https://github.com/zed-industries/zed/issues/16969
                                    // Workaround a bug in macOS Sequoia that adds an extra file-extension
                                    // sometimes. e.g. `a.sql` becomes `a.sql.s` or `a.txtx` becomes `a.txtx.txt`
                                    //
                                    // This is conditional on OS version because I'd like to get rid of it, so that
                                    // you can manually create a file called `a.sql.s`. That said it seems better
                                    // to break that use-case than breaking `a.sql`.
                                    if let &[_, second, third] = chunks.as_slice()
                                        && second.starts_with(third)
                                        && Self::os_version() >= Version::new(15, 0, 0)
                                    {
                                        path.set_extension("");
                                    }

                                    path
                                })
                        } else {
                            None
                        };

                        _ = done_tx.send(Ok(result));
                    }
                });

                panel.beginWithCompletionHandler(&handler);
            })
            .detach();

        done_rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        true
    }

    fn reveal_path(&self, path: &Path) {
        let path = path.to_path_buf();
        self.0
            .lock()
            .background_executor
            .spawn(async move {
                let full_path = NSString::from_str(path.to_str().unwrap_or(""));
                let root_full_path = NSString::from_str("");
                NSWorkspace::sharedWorkspace()
                    .selectFile_inFileViewerRootedAtPath(Some(&full_path), &root_full_path);
            })
            .detach();
    }

    fn open_with_system(&self, path: &Path) {
        let path = path.to_owned();
        self.0
            .lock()
            .background_executor
            .spawn(async move {
                #[allow(
                    clippy::disallowed_methods,
                    reason = "running on a background thread, so blocking is fine"
                )]
                new_std_command("open")
                    .arg("--")
                    .arg(path)
                    .status()
                    .context("invoking open command")
                    .log_err();
            })
            .detach();
    }

    fn on_quit(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().reopen = Some(callback);
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_keyboard_layout_change = Some(callback);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.0.lock().menu_command = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().will_open_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.0.lock().validate_menu_command = Some(callback);
    }

    fn on_thermal_state_change(&self, callback: Box<dyn FnMut()>) {
        let observer = ThermalObserver::new(&self.foreground_executor(), callback);
        self.0.lock().thermal_observer = Some(observer);
    }

    fn on_system_sleep(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_system_sleep = Some(callback);
        self.ensure_system_power_observers();
    }

    fn on_system_wake(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_system_wake = Some(callback);
        self.ensure_system_power_observers();
    }

    fn thermal_state(&self) -> ThermalState {
        thermal::thermal_state()
    }

    fn prevent_idle_sleep(&self, reason: &str) -> Task<Result<ActivityGuard>> {
        Task::ready(Ok(AppleActivity::begin(
            reason,
            NSActivityOptions::UserInitiated,
        )))
    }

    fn show_system_notification(&self, notification: gpui::SystemNotification) {
        let mut state = self.0.lock();
        let executor = state.foreground_executor.clone();
        state.system_notifications.show(&executor, notification);
    }

    fn dismiss_system_notification(&self, tag: &str) {
        let mut state = self.0.lock();
        let executor = state.foreground_executor.clone();
        state.system_notifications.dismiss(&executor, tag);
    }

    fn on_system_notification_response(
        &self,
        callback: Box<dyn FnMut(gpui::SystemNotificationResponse)>,
    ) {
        let mut state = self.0.lock();
        let executor = state.foreground_executor.clone();
        state.system_notifications.on_response(&executor, callback);
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(MacKeyboardLayout::new())
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        self.0.lock().keyboard_mapper.clone()
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle: *mut AnyObject = msg_send![class!(NSBundle), mainBundle];
            anyhow::ensure!(!bundle.is_null(), "app is not running inside a bundle");
            Ok(path_from_objc(msg_send![bundle, bundlePath]))
        }
    }

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        unsafe {
            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            let delegate: *mut AnyObject = msg_send![app, delegate];
            let menu = self.create_menu_bar(&menus, delegate, actions, keymap);
            drop(state);
            let _: () = msg_send![app, setMainMenu: &*menu];
        }
        self.0.lock().menus = Some(menus.into_iter().map(|menu| menu.owned()).collect());
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        self.0.lock().menus.clone()
    }

    fn set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap) {
        unsafe {
            let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            let delegate: *mut AnyObject = msg_send![app, delegate];
            state.dock_menu = Some(self.create_dock_menu(menu, delegate, actions, keymap));
        }
    }

    fn add_recent_document(&self, path: &Path) {
        if let Some(path_str) = path.to_str() {
            let document_controller = NSDocumentController::sharedDocumentController(self.1);
            let url = NSURL::fileURLWithPath(&NSString::from_str(path_str));
            document_controller.noteNewRecentDocumentURL(&url);
        }
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unsafe {
            let bundle: *mut AnyObject = msg_send![class!(NSBundle), mainBundle];
            anyhow::ensure!(!bundle.is_null(), "app is not running inside a bundle");
            let name = ns_string(name);
            let url: *mut AnyObject = msg_send![bundle, URLForAuxiliaryExecutable: name];
            anyhow::ensure!(!url.is_null(), "resource not found");
            ns_url_to_path(url)
        }
    }

    /// Match cursor style to one of the styles available
    /// in macOS's [NSCursor](https://developer.apple.com/documentation/appkit/nscursor).
    fn set_cursor_style(&self, style: CursorStyle) {
        unsafe {
            set_active_window_cursor_style(style);
        }
    }

    fn hide_cursor_until_mouse_moves(&self) {
        let cursor_visible = self.0.lock().cursor_visible.clone();
        if !cursor_visible.swap(false, Ordering::Relaxed) {
            return;
        }
        NSCursor::setHiddenUntilMouseMoves(true);
    }

    fn is_cursor_visible(&self) -> bool {
        self.0.lock().cursor_visible.load(Ordering::Relaxed)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        NSScroller::preferredScrollerStyle(self.1) == NSScrollerStyle::Overlay
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let state = self.0.lock();
        state.general_pasteboard.read()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        let state = self.0.lock();
        state.general_pasteboard.write(item);
    }

    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem> {
        let state = self.0.lock();
        state.find_pasteboard.read()
    }

    fn write_to_find_pasteboard(&self, item: ClipboardItem) {
        let state = self.0.lock();
        state.find_pasteboard.write(item);
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor()
            .spawn(async move { Ok(keychain::write_credentials(&url, &username, &password)?) })
    }

    fn write_credentials_if_not_exists(
        &self,
        url: &str,
        username: &str,
        password: &[u8],
    ) -> Task<Result<bool>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor().spawn(async move {
            Ok(keychain::write_credentials_if_not_exists(
                &url, &username, &password,
            )?)
        })
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
}

unsafe fn path_from_objc(path: *mut AnyObject) -> PathBuf {
    let path = unsafe { &*path.cast::<NSString>() };
    let len = path.lengthOfBytesUsingEncoding(NSUTF8StringEncoding);
    let bytes = path.UTF8String().cast::<u8>();
    let path = str::from_utf8(unsafe { slice::from_raw_parts(bytes, len) }).unwrap();
    PathBuf::from(path)
}

unsafe fn get_mac_platform(object: &AnyObject) -> &MacPlatform {
    unsafe {
        let platform_ptr = *mac_platform_ivar(object).load::<*mut c_void>(object);
        assert!(!platform_ptr.is_null());
        &*(platform_ptr as *const MacPlatform)
    }
}

fn mac_platform_ivar(object: &AnyObject) -> &Ivar {
    let ivar = object
        .class()
        .instance_variable(MAC_PLATFORM_IVAR)
        .expect("platform ivar is missing");
    assert!(
        <*mut c_void>::ENCODING.equivalent_to_str(
            ivar.type_encoding()
                .to_str()
                .expect("platform ivar encoding is not UTF-8")
        ),
        "platform ivar has an unexpected type encoding"
    );
    ivar
}

extern "C-unwind" fn will_finish_launching(_this: &AnyObject, _: Sel, _: *mut AnyObject) {
    unsafe {
        let user_defaults: *mut AnyObject = msg_send![class!(NSUserDefaults), standardUserDefaults];

        // The autofill heuristic controller causes slowdown and high CPU usage.
        // We don't know exactly why. This disables the full heuristic controller.
        //
        // Adapted from: https://github.com/ghostty-org/ghostty/pull/8625
        let name = ns_string("NSAutoFillHeuristicControllerEnabled");
        let existing_value: *mut AnyObject = msg_send![user_defaults, objectForKey: name];
        if existing_value.is_null() {
            let false_value: *mut AnyObject = msg_send![class!(NSNumber), numberWithBool: Bool::NO];
            let _: () = msg_send![user_defaults, setObject: false_value, forKey: name];
        }
    }
}

extern "C-unwind" fn did_finish_launching(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    unsafe {
        let app: *mut AnyObject = msg_send![APP_CLASS, sharedApplication];
        let policy = get_mac_platform(this).0.lock().activation_policy;
        let _: Bool = msg_send![app, setActivationPolicy: native_activation_policy(policy)];

        let notification_center: *mut AnyObject =
            msg_send![class!(NSNotificationCenter), defaultCenter];
        let name = ns_string("NSTextInputContextKeyboardSelectionDidChangeNotification");
        let _: () = msg_send![notification_center, addObserver: this,
            selector: sel!(onKeyboardLayoutChange:),
            name: name,
            object: ptr::null_mut::<AnyObject>()
        ];

        // Posted when a display is connected, disconnected, rearranged, or
        // changes mode, which includes its refresh rate.
        let _: () = msg_send![notification_center, addObserver: this,
            selector: sel!(onScreenParametersChange:),
            name: ns_string("NSApplicationDidChangeScreenParametersNotification"),
            object: ptr::null_mut::<AnyObject>()
        ];

        let platform = get_mac_platform(this);
        let callback = {
            let mut state = platform.0.lock();
            if (state.on_system_sleep.is_some() || state.on_system_wake.is_some())
                && !state.system_power_observers_registered
            {
                register_system_power_observers(this);
                state.system_power_observers_registered = true;
            }
            state.finish_launching.take()
        };
        if let Some(callback) = callback {
            callback();
        }
    }
}

/// `observer` must be the app delegate, whose class declares `onSystemSleep:`
/// and `onSystemWake:` (see `build_classes`).
fn register_system_power_observers(observer: &AnyObject) {
    let center = NSWorkspace::sharedWorkspace().notificationCenter();
    // SAFETY: both selectors are declared on the delegate class with the
    // notification-handler signature these observers are invoked with.
    unsafe {
        center.addObserver_selector_name_object(
            observer,
            objc2::sel!(onSystemSleep:),
            Some(NSWorkspaceWillSleepNotification),
            None,
        );
        center.addObserver_selector_name_object(
            observer,
            objc2::sel!(onSystemWake:),
            Some(NSWorkspaceDidWakeNotification),
            None,
        );
    }
}

extern "C-unwind" fn should_handle_reopen(
    this: &AnyObject,
    _: Sel,
    _: *mut AnyObject,
    has_open_windows: Bool,
) -> Bool {
    if !has_open_windows.as_bool() {
        let platform = unsafe { get_mac_platform(this) };
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.reopen.take() {
            drop(lock);
            callback();
            platform.0.lock().reopen.get_or_insert(callback);
        }
    }
    Bool::YES
}

extern "C-unwind" fn will_terminate(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    if let Some(mut callback) = lock.quit.take() {
        drop(lock);
        callback();
        platform.0.lock().quit.get_or_insert(callback);
    }
}

extern "C-unwind" fn on_keyboard_layout_change(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    let keyboard_layout = MacKeyboardLayout::new();
    lock.keyboard_mapper = Rc::new(MacKeyboardMapper::new(keyboard_layout.id()));
    if let Some(mut callback) = lock.on_keyboard_layout_change.take() {
        drop(lock);
        callback();
        platform
            .0
            .lock()
            .on_keyboard_layout_change
            .get_or_insert(callback);
    }
}

extern "C-unwind" fn on_screen_parameters_change(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    // Defer to the next run loop iteration to avoid re-entrant borrows of the App RefCell,
    // as NSNotificationCenter delivers this notification synchronously and it may fire while
    // the App is already borrowed (same pattern as quit() above).
    let platform = unsafe { get_mac_platform(this) };
    let platform_ptr = platform as *const MacPlatform as *mut c_void;
    unsafe {
        DispatchQueue::main().exec_async_f(platform_ptr, on_screen_parameters_change);
    }

    extern "C" fn on_screen_parameters_change(context: *mut c_void) {
        let platform = unsafe { &*(context as *const MacPlatform) };
        let callback = platform.0.lock().on_displays_changed.take();
        if let Some(mut callback) = callback {
            callback();
            platform
                .0
                .lock()
                .on_displays_changed
                .get_or_insert(callback);
        }
    }
}

// Deferred to the main queue like the wake and thermal notifications. The
// machine keeps running the main run loop for well over a frame after the
// observers return, so the block runs before it suspends.
extern "C-unwind" fn on_system_sleep(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    // SAFETY: this is the registered app delegate carrying MAC_PLATFORM_IVAR.
    let platform = unsafe { get_mac_platform(this) };
    let platform_ptr = platform as *const MacPlatform as *mut c_void;
    // SAFETY: platform lives for the process lifetime while callbacks are registered.
    unsafe {
        DispatchQueue::main().exec_async_f(platform_ptr, on_system_sleep);
    }

    extern "C" fn on_system_sleep(context: *mut c_void) {
        // SAFETY: context is the MacPlatform pointer queued above.
        let platform = unsafe { &*(context as *const MacPlatform) };
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.on_system_sleep.take() {
            drop(lock);
            callback();
            platform.0.lock().on_system_sleep.get_or_insert(callback);
        }
    }
}

extern "C-unwind" fn on_system_wake(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    // SAFETY: this is the registered app delegate carrying MAC_PLATFORM_IVAR.
    let platform = unsafe { get_mac_platform(this) };
    let platform_ptr = platform as *const MacPlatform as *mut c_void;
    // SAFETY: platform lives for the process lifetime while callbacks are registered.
    unsafe {
        DispatchQueue::main().exec_async_f(platform_ptr, on_system_wake);
    }

    extern "C" fn on_system_wake(context: *mut c_void) {
        // SAFETY: context is the MacPlatform pointer queued above.
        let platform = unsafe { &*(context as *const MacPlatform) };
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.on_system_wake.take() {
            drop(lock);
            callback();
            platform.0.lock().on_system_wake.get_or_insert(callback);
        }
    }
}

extern "C-unwind" fn open_urls(this: &AnyObject, _: Sel, _: *mut AnyObject, urls: *mut AnyObject) {
    let urls = unsafe {
        urls.cast::<NSArray<NSURL>>()
            .as_ref()
            .into_iter()
            .flat_map(|urls| urls.iter())
            .filter_map(|url| {
                let string = url.absoluteString()?;
                match CStr::from_ptr(string.UTF8String()).to_str() {
                    Ok(string) => Some(string.to_string()),
                    Err(err) => {
                        log::error!("error converting path to string: {}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    };
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    if let Some(mut callback) = lock.open_urls.take() {
        drop(lock);
        callback(urls);
        platform.0.lock().open_urls.get_or_insert(callback);
    }
}

extern "C-unwind" fn handle_menu_item(this: &AnyObject, _: Sel, item: *mut AnyObject) {
    unsafe {
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.menu_command.take() {
            let tag: NSInteger = if item.is_null() {
                0
            } else {
                msg_send![item, tag]
            };
            let index = tag as usize;
            if let Some(action) = lock.menu_actions.get(index) {
                let action = action.boxed_clone();
                drop(lock);
                callback(&*action);
            }
            platform.0.lock().menu_command.get_or_insert(callback);
        }
    }
}

extern "C-unwind" fn validate_menu_item(this: &AnyObject, _: Sel, item: *mut AnyObject) -> Bool {
    unsafe {
        let mut result = false;
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.validate_menu_command.take() {
            let tag: NSInteger = if item.is_null() {
                0
            } else {
                msg_send![item, tag]
            };
            let index = tag as usize;
            if let Some(action) = lock.menu_actions.get(index) {
                let action = action.boxed_clone();
                drop(lock);
                result = callback(action.as_ref());
            }
            platform
                .0
                .lock()
                .validate_menu_command
                .get_or_insert(callback);
        }
        result.to_objc()
    }
}

extern "C-unwind" fn menu_will_open(this: &AnyObject, _: Sel, _: *mut AnyObject) {
    unsafe {
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.will_open_menu.take() {
            drop(lock);
            callback();
            platform.0.lock().will_open_menu.get_or_insert(callback);
        }
    }
}

extern "C-unwind" fn handle_dock_menu(
    this: &AnyObject,
    _: Sel,
    _: *mut AnyObject,
) -> *mut AnyObject {
    unsafe {
        let platform = get_mac_platform(this);
        let state = platform.0.lock();
        state.dock_menu.as_ref().map_or(ptr::null_mut(), |menu| {
            Retained::as_ptr(menu).cast_mut().cast::<AnyObject>()
        })
    }
}

unsafe fn ns_url_to_path(url: *mut AnyObject) -> Result<PathBuf> {
    let path: *const c_char = unsafe { msg_send![url, fileSystemRepresentation] };
    anyhow::ensure!(!path.is_null(), "url is not a file path: {}", unsafe {
        let string: *mut NSString = msg_send![url, absoluteString];
        let string = string.as_ref().map_or(c"".as_ptr(), NSString::UTF8String);
        let string = if string.is_null() {
            c""
        } else {
            CStr::from_ptr(string)
        };
        string.to_string_lossy()
    });
    Ok(PathBuf::from(OsStr::from_bytes(unsafe {
        CStr::from_ptr(path).to_bytes()
    })))
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    pub(super) fn TISCopyCurrentKeyboardLayoutInputSource() -> *mut AnyObject;
    pub(super) fn TISCopyCurrentKeyboardInputSource() -> *mut AnyObject;
    pub(super) fn TISGetInputSourceProperty(
        inputSource: *mut AnyObject,
        propertyKey: *const c_void,
    ) -> *mut AnyObject;

    pub(super) fn UCKeyTranslate(
        keyLayoutPtr: *const ::std::os::raw::c_void,
        virtualKeyCode: u16,
        keyAction: u16,
        modifierKeyState: u32,
        keyboardType: u32,
        keyTranslateOptions: u32,
        deadKeyState: *mut u32,
        maxStringLength: usize,
        actualStringLength: *mut usize,
        unicodeString: *mut u16,
    ) -> u32;
    pub(super) fn LMGetKbdType() -> u16;
    pub(super) static kTISPropertyUnicodeKeyLayoutData: CFStringRef;
    pub(super) static kTISPropertyInputSourceID: CFStringRef;
    pub(super) static kTISPropertyLocalizedName: CFStringRef;
    pub(super) static kTISPropertyInputSourceIsASCIICapable: CFStringRef;
    pub(super) static kTISPropertyInputSourceType: CFStringRef;
    pub(super) static kTISTypeKeyboardInputMode: CFStringRef;
}
