use super::{events::key_to_native, BoolExt};
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, ForegroundExecutor,
    Keymap, MacDispatcher, MacDisplay, MacTextSystem, MacWindow, Menu, MenuItem, PathPromptOptions,
    Platform, PlatformDisplay, PlatformTextSystem, PlatformWindow, Result, SemanticVersion, Task,
    WindowAppearance, WindowParams,
};
use anyhow::{anyhow, bail};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSEventModifierFlags, NSMenu, NSMenuItem, NSModalResponse, NSOpenPanel, NSPasteboard,
        NSPasteboardTypeString, NSSavePanel, NSWindow,
    },
    base::{id, nil, selector, BOOL, YES},
    foundation::{
        NSArray, NSAutoreleasePool, NSBundle, NSData, NSInteger, NSProcessInfo, NSString,
        NSUInteger, NSURL,
    },
};
use core_foundation::{
    base::{CFRelease, CFType, CFTypeRef, OSStatus, TCFType as _},
    boolean::CFBoolean,
    data::CFData,
    dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary},
    string::{CFString, CFStringRef},
};
use ctor::ctor;
use futures::channel::oneshot;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use parking_lot::Mutex;
use ptr::null_mut;
use std::{
    cell::Cell,
    convert::TryInto,
    ffi::{c_void, CStr, OsStr},
    os::{raw::c_char, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
    process::Command,
    ptr,
    rc::Rc,
    slice, str,
    sync::Arc,
};
use time::UtcOffset;

use super::renderer;

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;

const MAC_PLATFORM_IVAR: &str = "platform";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor]
unsafe fn build_classes() {
    APP_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.register()
    };

    APP_DELEGATE_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSResponder)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationShouldHandleReopen:hasVisibleWindows:),
            should_handle_reopen as extern "C" fn(&mut Object, Sel, id, bool),
        );
        decl.add_method(
            sel!(applicationWillTerminate:),
            will_terminate as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(handleGPUIMenuItem:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        // Add menu item handlers so that OS save panels have the correct key commands
        decl.add_method(
            sel!(cut:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(copy:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(paste:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(selectAll:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(undo:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(redo:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(validateMenuItem:),
            validate_menu_item as extern "C" fn(&mut Object, Sel, id) -> bool,
        );
        decl.add_method(
            sel!(menuWillOpen:),
            menu_will_open as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDockMenu:),
            handle_dock_menu as extern "C" fn(&mut Object, Sel, id) -> id,
        );
        decl.add_method(
            sel!(application:openURLs:),
            open_urls as extern "C" fn(&mut Object, Sel, id, id),
        );

        decl.register()
    }
}

pub(crate) struct MacPlatform(Mutex<MacPlatformState>);

pub(crate) struct MacPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<MacTextSystem>,
    renderer_context: renderer::Context,
    pasteboard: id,
    text_hash_pasteboard_type: id,
    metadata_pasteboard_type: id,
    reopen: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
    dock_menu: Option<id>,
}

impl Default for MacPlatform {
    fn default() -> Self {
        Self::new()
    }
}

impl MacPlatform {
    pub(crate) fn new() -> Self {
        let dispatcher = Arc::new(MacDispatcher::new());
        Self(Mutex::new(MacPlatformState {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(MacTextSystem::new()),
            renderer_context: renderer::Context::default(),
            pasteboard: unsafe { NSPasteboard::generalPasteboard(nil) },
            text_hash_pasteboard_type: unsafe { ns_string("zed-text-hash") },
            metadata_pasteboard_type: unsafe { ns_string("zed-metadata") },
            reopen: None,
            quit: None,
            menu_command: None,
            validate_menu_command: None,
            will_open_menu: None,
            menu_actions: Default::default(),
            open_urls: None,
            finish_launching: None,
            dock_menu: None,
        }))
    }

    unsafe fn read_from_pasteboard(&self, pasteboard: *mut Object, kind: id) -> Option<&[u8]> {
        let data = pasteboard.dataForType(kind);
        if data == nil {
            None
        } else {
            Some(slice::from_raw_parts(
                data.bytes() as *mut u8,
                data.length() as usize,
            ))
        }
    }

    unsafe fn create_menu_bar(
        &self,
        menus: Vec<Menu>,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        let application_menu = NSMenu::new(nil).autorelease();
        application_menu.setDelegate_(delegate);

        for menu_config in menus {
            let menu = NSMenu::new(nil).autorelease();
            menu.setTitle_(ns_string(menu_config.name));
            menu.setDelegate_(delegate);

            for item_config in menu_config.items {
                menu.addItem_(Self::create_menu_item(
                    item_config,
                    delegate,
                    actions,
                    keymap,
                ));
            }

            let menu_item = NSMenuItem::new(nil).autorelease();
            menu_item.setSubmenu_(menu);
            application_menu.addItem_(menu_item);

            if menu_config.name == "Window" {
                let app: id = msg_send![APP_CLASS, sharedApplication];
                app.setWindowsMenu_(menu);
            }
        }

        application_menu
    }

    unsafe fn create_dock_menu(
        &self,
        menu_items: Vec<MenuItem>,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        let dock_menu = NSMenu::new(nil);
        dock_menu.setDelegate_(delegate);
        for item_config in menu_items {
            dock_menu.addItem_(Self::create_menu_item(
                item_config,
                delegate,
                actions,
                keymap,
            ));
        }

        dock_menu
    }

    unsafe fn create_menu_item(
        item: MenuItem,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        match item {
            MenuItem::Separator => NSMenuItem::separatorItem(nil),
            MenuItem::Action {
                name,
                action,
                os_action,
            } => {
                let keystrokes = keymap
                    .bindings_for_action(action.as_ref())
                    .next()
                    .map(|binding| binding.keystrokes());

                let selector = match os_action {
                    Some(crate::OsAction::Cut) => selector("cut:"),
                    Some(crate::OsAction::Copy) => selector("copy:"),
                    Some(crate::OsAction::Paste) => selector("paste:"),
                    Some(crate::OsAction::SelectAll) => selector("selectAll:"),
                    Some(crate::OsAction::Undo) => selector("undo:"),
                    Some(crate::OsAction::Redo) => selector("redo:"),
                    None => selector("handleGPUIMenuItem:"),
                };

                let item;
                if let Some(keystrokes) = keystrokes {
                    if keystrokes.len() == 1 {
                        let keystroke = &keystrokes[0];
                        let mut mask = NSEventModifierFlags::empty();
                        for (modifier, flag) in &[
                            (
                                keystroke.modifiers.platform,
                                NSEventModifierFlags::NSCommandKeyMask,
                            ),
                            (
                                keystroke.modifiers.control,
                                NSEventModifierFlags::NSControlKeyMask,
                            ),
                            (
                                keystroke.modifiers.alt,
                                NSEventModifierFlags::NSAlternateKeyMask,
                            ),
                            (
                                keystroke.modifiers.shift,
                                NSEventModifierFlags::NSShiftKeyMask,
                            ),
                        ] {
                            if *modifier {
                                mask |= *flag;
                            }
                        }

                        item = NSMenuItem::alloc(nil)
                            .initWithTitle_action_keyEquivalent_(
                                ns_string(name),
                                selector,
                                ns_string(key_to_native(&keystroke.key).as_ref()),
                            )
                            .autorelease();
                        item.setKeyEquivalentModifierMask_(mask);
                    }
                    // For multi-keystroke bindings, render the keystroke as part of the title.
                    else {
                        use std::fmt::Write;

                        let mut name = format!("{name} [");
                        for (i, keystroke) in keystrokes.iter().enumerate() {
                            if i > 0 {
                                name.push(' ');
                            }
                            write!(&mut name, "{}", keystroke).unwrap();
                        }
                        name.push(']');

                        item = NSMenuItem::alloc(nil)
                            .initWithTitle_action_keyEquivalent_(
                                ns_string(&name),
                                selector,
                                ns_string(""),
                            )
                            .autorelease();
                    }
                } else {
                    item = NSMenuItem::alloc(nil)
                        .initWithTitle_action_keyEquivalent_(
                            ns_string(name),
                            selector,
                            ns_string(""),
                        )
                        .autorelease();
                }

                let tag = actions.len() as NSInteger;
                let _: () = msg_send![item, setTag: tag];
                actions.push(action);
                item
            }
            MenuItem::Submenu(Menu { name, items }) => {
                let item = NSMenuItem::new(nil).autorelease();
                let submenu = NSMenu::new(nil).autorelease();
                submenu.setDelegate_(delegate);
                for item in items {
                    submenu.addItem_(Self::create_menu_item(item, delegate, actions, keymap));
                }
                item.setSubmenu_(submenu);
                item.setTitle_(ns_string(name));
                item
            }
        }
    }
}

impl Platform for MacPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> crate::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        self.0.lock().finish_launching = Some(on_finish_launching);

        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];
            app.setDelegate_(app_delegate);

            let self_ptr = self as *const Self as *const c_void;
            (*app).set_ivar(MAC_PLATFORM_IVAR, self_ptr);
            (*app_delegate).set_ivar(MAC_PLATFORM_IVAR, self_ptr);

            let pool = NSAutoreleasePool::new(nil);
            app.run();
            pool.drain();

            (*app).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
            (*app.delegate()).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
        }
    }

    fn quit(&self) {
        // Quitting the app causes us to close windows, which invokes `Window::on_close` callbacks
        // synchronously before this method terminates. If we call `Platform::quit` while holding a
        // borrow of the app state (which most of the time we will do), we will end up
        // double-borrowing the app state in the `on_close` callbacks for our open windows. To solve
        // this, we make quitting the application asynchronous so that we aren't holding borrows to
        // the app state on the stack when we actually terminate the app.

        use super::dispatcher::{dispatch_get_main_queue, dispatch_sys::dispatch_async_f};

        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), ptr::null_mut(), Some(quit));
        }

        unsafe extern "C" fn quit(_: *mut c_void) {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, terminate: nil];
        }
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        use std::os::unix::process::CommandExt as _;

        let app_pid = std::process::id().to_string();
        let app_path = self
            .app_path()
            .ok()
            // When the app is not bundled, `app_path` returns the
            // directory containing the executable. Disregard this
            // and get the path to the executable itself.
            .and_then(|path| (path.extension()?.to_str()? == "app").then_some(path))
            .unwrap_or_else(|| std::env::current_exe().unwrap());

        // Wait until this process has exited and then re-open this path.
        let script = r#"
            while kill -0 $0 2> /dev/null; do
                sleep 0.1
            done
            open "$1"
        "#;

        let restart_process = Command::new("/bin/bash")
            .arg("-c")
            .arg(script)
            .arg(app_pid)
            .arg(app_path)
            .process_group(0)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
    }

    fn activate(&self, ignoring_other_apps: bool) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.activateIgnoringOtherApps_(ignoring_other_apps.to_objc());
        }
    }

    fn hide(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, hide: nil];
        }
    }

    fn hide_other_apps(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, hideOtherApplications: nil];
        }
    }

    fn unhide_other_apps(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, unhideAllApplications: nil];
        }
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(MacDisplay::primary()))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        MacDisplay::all()
            .map(|screen| Rc::new(screen) as Rc<_>)
            .collect()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        MacWindow::active_window()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        // Clippy thinks that this evaluates to `()`, for some reason.
        #[allow(clippy::unit_arg, clippy::clone_on_copy)]
        let renderer_context = self.0.lock().renderer_context.clone();
        Box::new(MacWindow::open(
            handle,
            options,
            self.foreground_executor(),
            renderer_context,
        ))
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let appearance: id = msg_send![app, effectiveAppearance];
            WindowAppearance::from_native(appearance)
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let url = NSURL::alloc(nil)
                .initWithString_(ns_string(url))
                .autorelease();
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            msg_send![workspace, openURL: url]
        }
    }

    fn register_url_scheme(&self, scheme: &str) -> Task<anyhow::Result<()>> {
        // API only available post Monterey
        // https://developer.apple.com/documentation/appkit/nsworkspace/3753004-setdefaultapplicationaturl
        let (done_tx, done_rx) = oneshot::channel();
        if self.os_version().ok() < Some(SemanticVersion::new(12, 0, 0)) {
            return Task::ready(Err(anyhow!(
                "macOS 12.0 or later is required to register URL schemes"
            )));
        }

        let bundle_id = unsafe {
            let bundle: id = msg_send![class!(NSBundle), mainBundle];
            let bundle_id: id = msg_send![bundle, bundleIdentifier];
            if bundle_id == nil {
                return Task::ready(Err(anyhow!("Can only register URL scheme in bundled apps")));
            }
            bundle_id
        };

        unsafe {
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let scheme: id = ns_string(scheme);
            let app: id = msg_send![workspace, URLForApplicationWithBundleIdentifier: bundle_id];
            if app == nil {
                return Task::ready(Err(anyhow!(
                    "Cannot register URL scheme until app is installed"
                )));
            }
            let done_tx = Cell::new(Some(done_tx));
            let block = ConcreteBlock::new(move |error: id| {
                let result = if error == nil {
                    Ok(())
                } else {
                    let msg: id = msg_send![error, localizedDescription];
                    Err(anyhow!("Failed to register: {:?}", msg))
                };

                if let Some(done_tx) = done_tx.take() {
                    let _ = done_tx.send(result);
                }
            });
            let block = block.copy();
            let _: () = msg_send![workspace, setDefaultApplicationAtURL: app toOpenURLsWithScheme: scheme completionHandler: block];
        }

        self.background_executor()
            .spawn(async { crate::Flatten::flatten(done_rx.await.map_err(|e| anyhow!(e))) })
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let panel = NSOpenPanel::openPanel(nil);
                    panel.setCanChooseDirectories_(options.directories.to_objc());
                    panel.setCanChooseFiles_(options.files.to_objc());
                    panel.setAllowsMultipleSelection_(options.multiple.to_objc());
                    panel.setCanCreateDirectories(true.to_objc());
                    panel.setResolvesAliases_(false.to_objc());
                    let done_tx = Cell::new(Some(done_tx));
                    let block = ConcreteBlock::new(move |response: NSModalResponse| {
                        let result = if response == NSModalResponse::NSModalResponseOk {
                            let mut result = Vec::new();
                            let urls = panel.URLs();
                            for i in 0..urls.count() {
                                let url = urls.objectAtIndex(i);
                                if url.isFileURL() == YES {
                                    if let Ok(path) = ns_url_to_path(url) {
                                        result.push(path)
                                    }
                                }
                            }
                            Some(result)
                        } else {
                            None
                        };

                        if let Some(done_tx) = done_tx.take() {
                            let _ = done_tx.send(result);
                        }
                    });
                    let block = block.copy();
                    let _: () = msg_send![panel, beginWithCompletionHandler: block];
                }
            })
            .detach();
        done_rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        let directory = directory.to_owned();
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let panel = NSSavePanel::savePanel(nil);
                    let path = ns_string(directory.to_string_lossy().as_ref());
                    let url = NSURL::fileURLWithPath_isDirectory_(nil, path, true.to_objc());
                    panel.setDirectoryURL(url);

                    let done_tx = Cell::new(Some(done_tx));
                    let block = ConcreteBlock::new(move |response: NSModalResponse| {
                        let mut result = None;
                        if response == NSModalResponse::NSModalResponseOk {
                            let url = panel.URL();
                            if url.isFileURL() == YES {
                                result = ns_url_to_path(panel.URL()).ok()
                            }
                        }

                        if let Some(done_tx) = done_tx.take() {
                            let _ = done_tx.send(result);
                        }
                    });
                    let block = block.copy();
                    let _: () = msg_send![panel, beginWithCompletionHandler: block];
                }
            })
            .detach();

        done_rx
    }

    fn reveal_path(&self, path: &Path) {
        unsafe {
            let path = path.to_path_buf();
            self.0
                .lock()
                .background_executor
                .spawn(async move {
                    let full_path = ns_string(path.to_str().unwrap_or(""));
                    let root_full_path = ns_string("");
                    let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
                    let _: BOOL = msg_send![
                        workspace,
                        selectFile: full_path
                        inFileViewerRootedAtPath: root_full_path
                    ];
                })
                .detach();
        }
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().reopen = Some(callback);
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

    fn os_name(&self) -> &'static str {
        "macOS"
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        unsafe {
            let process_info = NSProcessInfo::processInfo(nil);
            let version = process_info.operatingSystemVersion();
            Ok(SemanticVersion::new(
                version.majorVersion as usize,
                version.minorVersion as usize,
                version.patchVersion as usize,
            ))
        }
    }

    fn app_version(&self) -> Result<SemanticVersion> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            if bundle.is_null() {
                Err(anyhow!("app is not running inside a bundle"))
            } else {
                let version: id = msg_send![bundle, objectForInfoDictionaryKey: ns_string("CFBundleShortVersionString")];
                if version.is_null() {
                    bail!("bundle does not have version");
                }
                let len = msg_send![version, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
                let bytes = version.UTF8String() as *const u8;
                let version = str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap();
                version.parse()
            }
        }
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            if bundle.is_null() {
                Err(anyhow!("app is not running inside a bundle"))
            } else {
                Ok(path_from_objc(msg_send![bundle, bundlePath]))
            }
        }
    }

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            app.setMainMenu_(self.create_menu_bar(menus, app.delegate(), actions, keymap));
        }
    }

    fn set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            let new = self.create_dock_menu(menu, app.delegate(), actions, keymap);
            if let Some(old) = state.dock_menu.replace(new) {
                CFRelease(old as _)
            }
        }
    }

    fn add_recent_document(&self, path: &Path) {
        if let Some(path_str) = path.to_str() {
            unsafe {
                let document_controller: id =
                    msg_send![class!(NSDocumentController), sharedDocumentController];
                let url: id = NSURL::fileURLWithPath_(nil, ns_string(path_str));
                let _: () = msg_send![document_controller, noteNewRecentDocumentURL:url];
            }
        }
    }

    fn local_timezone(&self) -> UtcOffset {
        unsafe {
            let local_timezone: id = msg_send![class!(NSTimeZone), localTimeZone];
            let seconds_from_gmt: NSInteger = msg_send![local_timezone, secondsFromGMT];
            UtcOffset::from_whole_seconds(seconds_from_gmt.try_into().unwrap()).unwrap()
        }
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            if bundle.is_null() {
                Err(anyhow!("app is not running inside a bundle"))
            } else {
                let name = ns_string(name);
                let url: id = msg_send![bundle, URLForAuxiliaryExecutable: name];
                if url.is_null() {
                    Err(anyhow!("resource not found"))
                } else {
                    ns_url_to_path(url)
                }
            }
        }
    }

    /// Match cursor style to one of the styles available
    /// in macOS's [NSCursor](https://developer.apple.com/documentation/appkit/nscursor).
    fn set_cursor_style(&self, style: CursorStyle) {
        unsafe {
            let new_cursor: id = match style {
                CursorStyle::Arrow => msg_send![class!(NSCursor), arrowCursor],
                CursorStyle::IBeam => msg_send![class!(NSCursor), IBeamCursor],
                CursorStyle::Crosshair => msg_send![class!(NSCursor), crosshairCursor],
                CursorStyle::ClosedHand => msg_send![class!(NSCursor), closedHandCursor],
                CursorStyle::OpenHand => msg_send![class!(NSCursor), openHandCursor],
                CursorStyle::PointingHand => msg_send![class!(NSCursor), pointingHandCursor],
                CursorStyle::ResizeLeft => msg_send![class!(NSCursor), resizeLeftCursor],
                CursorStyle::ResizeRight => msg_send![class!(NSCursor), resizeRightCursor],
                CursorStyle::ResizeLeftRight => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorStyle::ResizeColumn => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorStyle::ResizeUp => msg_send![class!(NSCursor), resizeUpCursor],
                CursorStyle::ResizeDown => msg_send![class!(NSCursor), resizeDownCursor],
                CursorStyle::ResizeUpDown => msg_send![class!(NSCursor), resizeUpDownCursor],
                CursorStyle::ResizeRow => msg_send![class!(NSCursor), resizeUpDownCursor],
                CursorStyle::DisappearingItem => {
                    msg_send![class!(NSCursor), disappearingItemCursor]
                }
                CursorStyle::IBeamCursorForVerticalLayout => {
                    msg_send![class!(NSCursor), IBeamCursorForVerticalLayout]
                }
                CursorStyle::OperationNotAllowed => {
                    msg_send![class!(NSCursor), operationNotAllowedCursor]
                }
                CursorStyle::DragLink => msg_send![class!(NSCursor), dragLinkCursor],
                CursorStyle::DragCopy => msg_send![class!(NSCursor), dragCopyCursor],
                CursorStyle::ContextualMenu => msg_send![class!(NSCursor), contextualMenuCursor],
            };

            let old_cursor: id = msg_send![class!(NSCursor), currentCursor];
            if new_cursor != old_cursor {
                let _: () = msg_send![new_cursor, set];
            }
        }
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        #[allow(non_upper_case_globals)]
        const NSScrollerStyleOverlay: NSInteger = 1;

        unsafe {
            let style: NSInteger = msg_send![class!(NSScroller), preferredScrollerStyle];
            style == NSScrollerStyleOverlay
        }
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        let state = self.0.lock();
        unsafe {
            state.pasteboard.clearContents();

            let text_bytes = NSData::dataWithBytes_length_(
                nil,
                item.text.as_ptr() as *const c_void,
                item.text.len() as u64,
            );
            state
                .pasteboard
                .setData_forType(text_bytes, NSPasteboardTypeString);

            if let Some(metadata) = item.metadata.as_ref() {
                let hash_bytes = ClipboardItem::text_hash(&item.text).to_be_bytes();
                let hash_bytes = NSData::dataWithBytes_length_(
                    nil,
                    hash_bytes.as_ptr() as *const c_void,
                    hash_bytes.len() as u64,
                );
                state
                    .pasteboard
                    .setData_forType(hash_bytes, state.text_hash_pasteboard_type);

                let metadata_bytes = NSData::dataWithBytes_length_(
                    nil,
                    metadata.as_ptr() as *const c_void,
                    metadata.len() as u64,
                );
                state
                    .pasteboard
                    .setData_forType(metadata_bytes, state.metadata_pasteboard_type);
            }
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let state = self.0.lock();
        unsafe {
            if let Some(text_bytes) =
                self.read_from_pasteboard(state.pasteboard, NSPasteboardTypeString)
            {
                let text = String::from_utf8_lossy(text_bytes).to_string();
                let hash_bytes = self
                    .read_from_pasteboard(state.pasteboard, state.text_hash_pasteboard_type)
                    .and_then(|bytes| bytes.try_into().ok())
                    .map(u64::from_be_bytes);
                let metadata_bytes = self
                    .read_from_pasteboard(state.pasteboard, state.metadata_pasteboard_type)
                    .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok());

                if let Some((hash, metadata)) = hash_bytes.zip(metadata_bytes) {
                    if hash == ClipboardItem::text_hash(&text) {
                        Some(ClipboardItem {
                            text,
                            metadata: Some(metadata),
                        })
                    } else {
                        Some(ClipboardItem {
                            text,
                            metadata: None,
                        })
                    }
                } else {
                    Some(ClipboardItem {
                        text,
                        metadata: None,
                    })
                }
            } else {
                None
            }
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

                // First, check if there are already credentials for the given server. If so, then
                // update the username and password.
                let mut verb = "updating";
                let mut query_attrs = CFMutableDictionary::with_capacity(2);
                query_attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let mut attrs = CFMutableDictionary::with_capacity(4);
                attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                attrs.set(kSecAttrAccount as *const _, username.as_CFTypeRef());
                attrs.set(kSecValueData as *const _, password.as_CFTypeRef());

                let mut status = SecItemUpdate(
                    query_attrs.as_concrete_TypeRef(),
                    attrs.as_concrete_TypeRef(),
                );

                // If there were no existing credentials for the given server, then create them.
                if status == errSecItemNotFound {
                    verb = "creating";
                    status = SecItemAdd(attrs.as_concrete_TypeRef(), ptr::null_mut());
                }

                if status != errSecSuccess {
                    return Err(anyhow!("{} password failed: {}", verb, status));
                }
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

                // Find any credentials for the given server URL.
                let mut attrs = CFMutableDictionary::with_capacity(5);
                attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                attrs.set(kSecReturnAttributes as *const _, cf_true);
                attrs.set(kSecReturnData as *const _, cf_true);

                let mut result = CFTypeRef::from(ptr::null());
                let status = SecItemCopyMatching(attrs.as_concrete_TypeRef(), &mut result);
                match status {
                    security::errSecSuccess => {}
                    security::errSecItemNotFound | security::errSecUserCanceled => return Ok(None),
                    _ => return Err(anyhow!("reading password failed: {}", status)),
                }

                let result = CFType::wrap_under_create_rule(result)
                    .downcast::<CFDictionary>()
                    .ok_or_else(|| anyhow!("keychain item was not a dictionary"))?;
                let username = result
                    .find(kSecAttrAccount as *const _)
                    .ok_or_else(|| anyhow!("account was missing from keychain item"))?;
                let username = CFType::wrap_under_get_rule(*username)
                    .downcast::<CFString>()
                    .ok_or_else(|| anyhow!("account was not a string"))?;
                let password = result
                    .find(kSecValueData as *const _)
                    .ok_or_else(|| anyhow!("password was missing from keychain item"))?;
                let password = CFType::wrap_under_get_rule(*password)
                    .downcast::<CFData>()
                    .ok_or_else(|| anyhow!("password was not a string"))?;

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
                let mut query_attrs = CFMutableDictionary::with_capacity(2);
                query_attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let status = SecItemDelete(query_attrs.as_concrete_TypeRef());

                if status != errSecSuccess {
                    return Err(anyhow!("delete password failed: {}", status));
                }
            }
            Ok(())
        })
    }
}

unsafe fn path_from_objc(path: id) -> PathBuf {
    let len = msg_send![path, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
    let bytes = path.UTF8String() as *const u8;
    let path = str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap();
    PathBuf::from(path)
}

unsafe fn get_mac_platform(object: &mut Object) -> &MacPlatform {
    let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
    assert!(!platform_ptr.is_null());
    &*(platform_ptr as *const MacPlatform)
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let app: id = msg_send![APP_CLASS, sharedApplication];
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        let platform = get_mac_platform(this);
        let callback = platform.0.lock().finish_launching.take();
        if let Some(callback) = callback {
            callback();
        }
    }
}

extern "C" fn should_handle_reopen(this: &mut Object, _: Sel, _: id, has_open_windows: bool) {
    if !has_open_windows {
        let platform = unsafe { get_mac_platform(this) };
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.reopen.take() {
            drop(lock);
            callback();
            platform.0.lock().reopen.get_or_insert(callback);
        }
    }
}

extern "C" fn will_terminate(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    if let Some(mut callback) = lock.quit.take() {
        drop(lock);
        callback();
        platform.0.lock().quit.get_or_insert(callback);
    }
}

extern "C" fn open_urls(this: &mut Object, _: Sel, _: id, urls: id) {
    let urls = unsafe {
        (0..urls.count())
            .filter_map(|i| {
                let url = urls.objectAtIndex(i);
                match CStr::from_ptr(url.absoluteString().UTF8String() as *mut c_char).to_str() {
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

extern "C" fn handle_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
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

extern "C" fn validate_menu_item(this: &mut Object, _: Sel, item: id) -> bool {
    unsafe {
        let mut result = false;
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.validate_menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
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
        result
    }
}

extern "C" fn menu_will_open(this: &mut Object, _: Sel, _: id) {
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

extern "C" fn handle_dock_menu(this: &mut Object, _: Sel, _: id) -> id {
    unsafe {
        let platform = get_mac_platform(this);
        let mut state = platform.0.lock();
        if let Some(id) = state.dock_menu {
            id
        } else {
            nil
        }
    }
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}

unsafe fn ns_url_to_path(url: id) -> Result<PathBuf> {
    let path: *mut c_char = msg_send![url, fileSystemRepresentation];
    if path.is_null() {
        Err(anyhow!(
            "url is not a file path: {}",
            CStr::from_ptr(url.absoluteString().UTF8String()).to_string_lossy()
        ))
    } else {
        Ok(PathBuf::from(OsStr::from_bytes(
            CStr::from_ptr(path).to_bytes(),
        )))
    }
}

mod security {
    #![allow(non_upper_case_globals)]
    use super::*;

    #[link(name = "Security", kind = "framework")]
    extern "C" {
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

    pub const errSecSuccess: OSStatus = 0;
    pub const errSecUserCanceled: OSStatus = -128;
    pub const errSecItemNotFound: OSStatus = -25300;
}

#[cfg(test)]
mod tests {
    use crate::ClipboardItem;

    use super::*;

    #[test]
    fn test_clipboard() {
        let platform = build_platform();
        assert_eq!(platform.read_from_clipboard(), None);

        let item = ClipboardItem::new("1".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem::new("2".to_string()).with_metadata(vec![3, 4]);
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let text_from_other_app = "text from other app";
        unsafe {
            let bytes = NSData::dataWithBytes_length_(
                nil,
                text_from_other_app.as_ptr() as *const c_void,
                text_from_other_app.len() as u64,
            );
            platform
                .0
                .lock()
                .pasteboard
                .setData_forType(bytes, NSPasteboardTypeString);
        }
        assert_eq!(
            platform.read_from_clipboard(),
            Some(ClipboardItem::new(text_from_other_app.to_string()))
        );
    }

    fn build_platform() -> MacPlatform {
        let platform = MacPlatform::new();
        platform.0.lock().pasteboard = unsafe { NSPasteboard::pasteboardWithUniqueName(nil) };
        platform
    }
}
