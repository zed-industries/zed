use super::{BoolExt as _, Dispatcher, FontSystem, Window};
use crate::{
    executor, keymap,
    platform::{self, CursorStyle},
    Action, ClipboardItem, Event, Menu, MenuItem,
};
use anyhow::{anyhow, Result};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSEventModifierFlags, NSMenu, NSMenuItem, NSModalResponse, NSOpenPanel, NSPasteboard,
        NSPasteboardTypeString, NSSavePanel, NSWindow,
    },
    base::{id, nil, selector, YES},
    foundation::{
        NSArray, NSAutoreleasePool, NSBundle, NSData, NSInteger, NSString, NSUInteger, NSURL,
    },
};
use core_foundation::{
    base::{CFType, CFTypeRef, OSStatus, TCFType as _},
    boolean::CFBoolean,
    data::CFData,
    dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary},
    string::{CFString, CFStringRef},
};
use ctor::ctor;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use postage::oneshot;
use ptr::null_mut;
use std::{
    cell::{Cell, RefCell},
    convert::TryInto,
    ffi::{c_void, CStr, OsStr},
    os::{raw::c_char, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
    ptr,
    rc::Rc,
    slice, str,
    sync::Arc,
};
use time::UtcOffset;

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;

const MAC_PLATFORM_IVAR: &'static str = "platform";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor]
unsafe fn build_classes() {
    APP_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&mut Object, Sel, id),
        );
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
            sel!(applicationDidBecomeActive:),
            did_become_active as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidResignActive:),
            did_resign_active as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationWillTerminate:),
            will_terminate as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(handleGPUIMenuItem:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(validateMenuItem:),
            validate_menu_item as extern "C" fn(&mut Object, Sel, id) -> bool,
        );
        decl.add_method(
            sel!(application:openURLs:),
            open_urls as extern "C" fn(&mut Object, Sel, id, id),
        );
        decl.register()
    }
}

#[derive(Default)]
pub struct MacForegroundPlatform(RefCell<MacForegroundPlatformState>);

#[derive(Default)]
pub struct MacForegroundPlatformState {
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(crate::Event) -> bool>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce() -> ()>>,
    menu_actions: Vec<Box<dyn Action>>,
}

impl MacForegroundPlatform {
    unsafe fn create_menu_bar(&self, menus: Vec<Menu>, keystroke_matcher: &keymap::Matcher) -> id {
        let menu_bar = NSMenu::new(nil).autorelease();
        let mut state = self.0.borrow_mut();

        state.menu_actions.clear();

        for menu_config in menus {
            let menu_bar_item = NSMenuItem::new(nil).autorelease();
            let menu = NSMenu::new(nil).autorelease();
            let menu_name = menu_config.name;

            menu.setTitle_(ns_string(menu_name));

            for item_config in menu_config.items {
                let item;

                match item_config {
                    MenuItem::Separator => {
                        item = NSMenuItem::separatorItem(nil);
                    }
                    MenuItem::Action { name, action } => {
                        let mut keystroke = None;
                        if let Some(binding) = keystroke_matcher
                            .bindings_for_action_type(action.as_any().type_id())
                            .next()
                        {
                            if binding.keystrokes().len() == 1 {
                                keystroke = binding.keystrokes().first()
                            }
                        }

                        if let Some(keystroke) = keystroke {
                            let mut mask = NSEventModifierFlags::empty();
                            for (modifier, flag) in &[
                                (keystroke.cmd, NSEventModifierFlags::NSCommandKeyMask),
                                (keystroke.ctrl, NSEventModifierFlags::NSControlKeyMask),
                                (keystroke.alt, NSEventModifierFlags::NSAlternateKeyMask),
                            ] {
                                if *modifier {
                                    mask |= *flag;
                                }
                            }

                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector("handleGPUIMenuItem:"),
                                    ns_string(&keystroke.key),
                                )
                                .autorelease();
                            item.setKeyEquivalentModifierMask_(mask);
                        } else {
                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector("handleGPUIMenuItem:"),
                                    ns_string(""),
                                )
                                .autorelease();
                        }

                        let tag = state.menu_actions.len() as NSInteger;
                        let _: () = msg_send![item, setTag: tag];
                        state.menu_actions.push(action);
                    }
                }

                menu.addItem_(item);
            }

            menu_bar_item.setSubmenu_(menu);
            menu_bar.addItem_(menu_bar_item);
        }

        menu_bar
    }
}

impl platform::ForegroundPlatform for MacForegroundPlatform {
    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.0.borrow_mut().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.0.borrow_mut().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.borrow_mut().quit = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        self.0.borrow_mut().event = Some(callback);
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.borrow_mut().open_urls = Some(callback);
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce() -> ()>) {
        self.0.borrow_mut().finish_launching = Some(on_finish_launching);

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

    fn on_menu_command(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.0.borrow_mut().menu_command = Some(callback);
    }

    fn on_validate_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.0.borrow_mut().validate_menu_command = Some(callback);
    }

    fn set_menus(&self, menus: Vec<Menu>, keystroke_matcher: &keymap::Matcher) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            app.setMainMenu_(self.create_menu_bar(menus, keystroke_matcher));
        }
    }

    fn prompt_for_paths(
        &self,
        options: platform::PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        unsafe {
            let panel = NSOpenPanel::openPanel(nil);
            panel.setCanChooseDirectories_(options.directories.to_objc());
            panel.setCanChooseFiles_(options.files.to_objc());
            panel.setAllowsMultipleSelection_(options.multiple.to_objc());
            panel.setResolvesAliases_(false.to_objc());
            let (done_tx, done_rx) = oneshot::channel();
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

                if let Some(mut done_tx) = done_tx.take() {
                    let _ = postage::sink::Sink::try_send(&mut done_tx, result);
                }
            });
            let block = block.copy();
            let _: () = msg_send![panel, beginWithCompletionHandler: block];
            done_rx
        }
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        unsafe {
            let panel = NSSavePanel::savePanel(nil);
            let path = ns_string(directory.to_string_lossy().as_ref());
            let url = NSURL::fileURLWithPath_isDirectory_(nil, path, true.to_objc());
            panel.setDirectoryURL(url);

            let (done_tx, done_rx) = oneshot::channel();
            let done_tx = Cell::new(Some(done_tx));
            let block = ConcreteBlock::new(move |response: NSModalResponse| {
                let mut result = None;
                if response == NSModalResponse::NSModalResponseOk {
                    let url = panel.URL();
                    if url.isFileURL() == YES {
                        result = ns_url_to_path(panel.URL()).ok()
                    }
                }

                if let Some(mut done_tx) = done_tx.take() {
                    let _ = postage::sink::Sink::try_send(&mut done_tx, result);
                }
            });
            let block = block.copy();
            let _: () = msg_send![panel, beginWithCompletionHandler: block];
            done_rx
        }
    }
}

pub struct MacPlatform {
    dispatcher: Arc<Dispatcher>,
    fonts: Arc<FontSystem>,
    pasteboard: id,
    text_hash_pasteboard_type: id,
    metadata_pasteboard_type: id,
}

impl MacPlatform {
    pub fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(FontSystem::new()),
            pasteboard: unsafe { NSPasteboard::generalPasteboard(nil) },
            text_hash_pasteboard_type: unsafe { ns_string("zed-text-hash") },
            metadata_pasteboard_type: unsafe { ns_string("zed-metadata") },
        }
    }

    unsafe fn read_from_pasteboard(&self, kind: id) -> Option<&[u8]> {
        let data = self.pasteboard.dataForType(kind);
        if data == nil {
            None
        } else {
            Some(slice::from_raw_parts(
                data.bytes() as *mut u8,
                data.length() as usize,
            ))
        }
    }
}

unsafe impl Send for MacPlatform {}
unsafe impl Sync for MacPlatform {}

impl platform::Platform for MacPlatform {
    fn dispatcher(&self) -> Arc<dyn platform::Dispatcher> {
        self.dispatcher.clone()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.activateIgnoringOtherApps_(ignoring_other_apps.to_objc());
        }
    }

    fn open_window(
        &self,
        id: usize,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Box<dyn platform::Window> {
        Box::new(Window::open(id, options, executor, self.fonts()))
    }

    fn key_window_id(&self) -> Option<usize> {
        Window::key_window_id()
    }

    fn fonts(&self) -> Arc<dyn platform::FontSystem> {
        self.fonts.clone()
    }

    fn quit(&self) {
        // Quitting the app causes us to close windows, which invokes `Window::on_close` callbacks
        // synchronously before this method terminates. If we call `Platform::quit` while holding a
        // borrow of the app state (which most of the time we will do), we will end up
        // double-borrowing the app state in the `on_close` callbacks for our open windows. To solve
        // this, we make quitting the application asynchronous so that we aren't holding borrows to
        // the app state on the stack when we actually terminate the app.

        use super::dispatcher::{dispatch_async_f, dispatch_get_main_queue};

        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), ptr::null_mut(), Some(quit));
        }

        unsafe extern "C" fn quit(_: *mut c_void) {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, terminate: nil];
        }
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        unsafe {
            self.pasteboard.clearContents();

            let text_bytes = NSData::dataWithBytes_length_(
                nil,
                item.text.as_ptr() as *const c_void,
                item.text.len() as u64,
            );
            self.pasteboard
                .setData_forType(text_bytes, NSPasteboardTypeString);

            if let Some(metadata) = item.metadata.as_ref() {
                let hash_bytes = ClipboardItem::text_hash(&item.text).to_be_bytes();
                let hash_bytes = NSData::dataWithBytes_length_(
                    nil,
                    hash_bytes.as_ptr() as *const c_void,
                    hash_bytes.len() as u64,
                );
                self.pasteboard
                    .setData_forType(hash_bytes, self.text_hash_pasteboard_type);

                let metadata_bytes = NSData::dataWithBytes_length_(
                    nil,
                    metadata.as_ptr() as *const c_void,
                    metadata.len() as u64,
                );
                self.pasteboard
                    .setData_forType(metadata_bytes, self.metadata_pasteboard_type);
            }
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unsafe {
            if let Some(text_bytes) = self.read_from_pasteboard(NSPasteboardTypeString) {
                let text = String::from_utf8_lossy(&text_bytes).to_string();
                let hash_bytes = self
                    .read_from_pasteboard(self.text_hash_pasteboard_type)
                    .and_then(|bytes| bytes.try_into().ok())
                    .map(u64::from_be_bytes);
                let metadata_bytes = self
                    .read_from_pasteboard(self.metadata_pasteboard_type)
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

    fn open_url(&self, url: &str) {
        unsafe {
            let url = NSURL::alloc(nil)
                .initWithString_(ns_string(url))
                .autorelease();
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            msg_send![workspace, openURL: url]
        }
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()> {
        let url = CFString::from(url);
        let username = CFString::from(username);
        let password = CFData::from_buffer(password);

        unsafe {
            use security::*;

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
    }

    fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>> {
        let url = CFString::from(url);
        let cf_true = CFBoolean::true_value().as_CFTypeRef();

        unsafe {
            use security::*;

            // Find any credentials for the given server URL.
            let mut attrs = CFMutableDictionary::with_capacity(5);
            attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
            attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());
            attrs.set(kSecReturnAttributes as *const _, cf_true);
            attrs.set(kSecReturnData as *const _, cf_true);

            let mut result = CFTypeRef::from(ptr::null_mut());
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
    }

    fn delete_credentials(&self, url: &str) -> Result<()> {
        let url = CFString::from(url);

        unsafe {
            use security::*;

            let mut query_attrs = CFMutableDictionary::with_capacity(2);
            query_attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
            query_attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());

            let status = SecItemDelete(query_attrs.as_concrete_TypeRef());

            if status != errSecSuccess {
                return Err(anyhow!("delete password failed: {}", status));
            }
        }
        Ok(())
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        unsafe {
            let cursor: id = match style {
                CursorStyle::Arrow => msg_send![class!(NSCursor), arrowCursor],
                CursorStyle::ResizeLeftRight => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorStyle::PointingHand => msg_send![class!(NSCursor), pointingHandCursor],
                CursorStyle::IBeam => msg_send![class!(NSCursor), IBeamCursor],
            };
            let _: () = msg_send![cursor, set];
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

    fn app_version(&self) -> Result<platform::AppVersion> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            if bundle.is_null() {
                Err(anyhow!("app is not running inside a bundle"))
            } else {
                let version: id = msg_send![bundle, objectForInfoDictionaryKey: ns_string("CFBundleShortVersionString")];
                let len = msg_send![version, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
                let bytes = version.UTF8String() as *const u8;
                let version = str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap();
                version.parse()
            }
        }
    }
}

unsafe fn path_from_objc(path: id) -> PathBuf {
    let len = msg_send![path, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
    let bytes = path.UTF8String() as *const u8;
    let path = str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap();
    PathBuf::from(path)
}

unsafe fn get_foreground_platform(object: &mut Object) -> &MacForegroundPlatform {
    let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
    assert!(!platform_ptr.is_null());
    &*(platform_ptr as *const MacForegroundPlatform)
}

extern "C" fn send_event(this: &mut Object, _sel: Sel, native_event: id) {
    unsafe {
        if let Some(event) = Event::from_native(native_event, None) {
            let platform = get_foreground_platform(this);
            if let Some(callback) = platform.0.borrow_mut().event.as_mut() {
                if callback(event) {
                    return;
                }
            }
        }

        msg_send![super(this, class!(NSApplication)), sendEvent: native_event]
    }
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let app: id = msg_send![APP_CLASS, sharedApplication];
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        let platform = get_foreground_platform(this);
        let callback = platform.0.borrow_mut().finish_launching.take();
        if let Some(callback) = callback {
            callback();
        }
    }
}

extern "C" fn did_become_active(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_foreground_platform(this) };
    if let Some(callback) = platform.0.borrow_mut().become_active.as_mut() {
        callback();
    }
}

extern "C" fn did_resign_active(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_foreground_platform(this) };
    if let Some(callback) = platform.0.borrow_mut().resign_active.as_mut() {
        callback();
    }
}

extern "C" fn will_terminate(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_foreground_platform(this) };
    if let Some(callback) = platform.0.borrow_mut().quit.as_mut() {
        callback();
    }
}

extern "C" fn open_urls(this: &mut Object, _: Sel, _: id, urls: id) {
    let urls = unsafe {
        (0..urls.count())
            .into_iter()
            .filter_map(|i| {
                let path = urls.objectAtIndex(i);
                match CStr::from_ptr(path.absoluteString().UTF8String() as *mut c_char).to_str() {
                    Ok(string) => Some(string.to_string()),
                    Err(err) => {
                        log::error!("error converting path to string: {}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    };
    let platform = unsafe { get_foreground_platform(this) };
    if let Some(callback) = platform.0.borrow_mut().open_urls.as_mut() {
        callback(urls);
    }
}

extern "C" fn handle_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_foreground_platform(this);
        let mut platform = platform.0.borrow_mut();
        if let Some(mut callback) = platform.menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = platform.menu_actions.get(index) {
                callback(action.as_ref());
            }
            platform.menu_command = Some(callback);
        }
    }
}

extern "C" fn validate_menu_item(this: &mut Object, _: Sel, item: id) -> bool {
    unsafe {
        let mut result = false;
        let platform = get_foreground_platform(this);
        let mut platform = platform.0.borrow_mut();
        if let Some(mut callback) = platform.validate_menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = platform.menu_actions.get(index) {
                result = callback(action.as_ref());
            }
            platform.validate_menu_command = Some(callback);
        }
        result
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
    use crate::platform::Platform;

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
                .pasteboard
                .setData_forType(bytes, NSPasteboardTypeString);
        }
        assert_eq!(
            platform.read_from_clipboard(),
            Some(ClipboardItem::new(text_from_other_app.to_string()))
        );
    }

    fn build_platform() -> MacPlatform {
        let mut platform = MacPlatform::new();
        platform.pasteboard = unsafe { NSPasteboard::pasteboardWithUniqueName(nil) };
        platform
    }
}
