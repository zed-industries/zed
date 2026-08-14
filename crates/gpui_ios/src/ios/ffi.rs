//! Native entry points used by an iOS application delegate.

use gpui::{App, AppLifecyclePhase, Application, ApplicationHandle};
use std::{cell::UnsafeCell, ffi::c_void, rc::Rc, sync::OnceLock};

type AppCallback = Box<dyn FnOnce(&mut App)>;

struct IosCallbacks {
    finish_launching: Option<Box<dyn FnOnce()>>,
    app: Option<AppCallback>,
    quit: Option<Box<dyn FnMut()>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    app_lifecycle: Option<Box<dyn FnMut(AppLifecyclePhase)>>,
    memory_warning: Option<Box<dyn FnMut()>>,
}

struct IosAppState {
    callbacks: UnsafeCell<IosCallbacks>,
    application: UnsafeCell<Option<ApplicationHandle>>,
}

unsafe impl Send for IosAppState {}
unsafe impl Sync for IosAppState {}

pub(crate) struct WindowListWrapper(pub(crate) UnsafeCell<Vec<*const super::window::IosWindow>>);

unsafe impl Send for WindowListWrapper {}
unsafe impl Sync for WindowListWrapper {}

static IOS_APP_STATE: OnceLock<IosAppState> = OnceLock::new();
pub(crate) static IOS_WINDOW_LIST: OnceLock<WindowListWrapper> = OnceLock::new();

fn app_state() -> &'static IosAppState {
    IOS_APP_STATE.get_or_init(|| IosAppState {
        callbacks: UnsafeCell::new(IosCallbacks {
            finish_launching: None,
            app: None,
            quit: None,
            open_urls: None,
            app_lifecycle: None,
            memory_warning: None,
        }),
        application: UnsafeCell::new(None),
    })
}

fn window_list() -> &'static WindowListWrapper {
    IOS_WINDOW_LIST.get_or_init(|| WindowListWrapper(UnsafeCell::new(Vec::new())))
}

fn ios_window(window: *mut c_void) -> Option<&'static super::window::IosWindow> {
    unsafe { (window as *const super::window::IosWindow).as_ref() }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_initialize() -> *mut c_void {
    app_state();
    window_list();
    std::ptr::dangling_mut::<c_void>()
}

pub(crate) fn register_window(window: *const super::window::IosWindow) {
    unsafe {
        (*window_list().0.get()).push(window);
    }
}

pub(crate) fn unregister_window(window: *const super::window::IosWindow) {
    unsafe {
        (*window_list().0.get()).retain(|registered_window| *registered_window != window);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_get_window() -> *mut c_void {
    unsafe {
        (*window_list().0.get())
            .last()
            .copied()
            .map_or(std::ptr::null_mut(), |window| window.cast_mut().cast())
    }
}

pub(crate) fn set_finish_launching_callback(callback: Box<dyn FnOnce()>) {
    unsafe {
        (*app_state().callbacks.get()).finish_launching = Some(callback);
    }
}

pub(crate) fn set_quit_callback(callback: Box<dyn FnMut()>) {
    unsafe {
        (*app_state().callbacks.get()).quit = Some(callback);
    }
}

pub(crate) fn set_open_urls_callback(callback: Box<dyn FnMut(Vec<String>)>) {
    unsafe {
        (*app_state().callbacks.get()).open_urls = Some(callback);
    }
}

pub(crate) fn set_app_lifecycle_callback(callback: Box<dyn FnMut(AppLifecyclePhase)>) {
    unsafe {
        (*app_state().callbacks.get()).app_lifecycle = Some(callback);
    }
}

pub(crate) fn set_memory_warning_callback(callback: Box<dyn FnMut()>) {
    unsafe {
        (*app_state().callbacks.get()).memory_warning = Some(callback);
    }
}

pub fn set_app_callback(callback: AppCallback) {
    unsafe {
        (*app_state().callbacks.get()).app = Some(callback);
    }
}

fn take_app_callback() -> Option<AppCallback> {
    unsafe { (*app_state().callbacks.get()).app.take() }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_did_finish_launching(_app: *mut c_void) {
    let callback = unsafe { (*app_state().callbacks.get()).finish_launching.take() };
    if let Some(callback) = callback {
        callback();
    } else {
        log::error!("GPUI iOS: Application launch callback was not registered");
    }
}

fn notify_windows_active(is_active: bool) {
    unsafe {
        for &window in &*window_list().0.get() {
            if let Some(window) = window.as_ref() {
                window.notify_active_status_change(is_active);
            }
        }
    }
}

fn notify_app_lifecycle(phase: AppLifecyclePhase) {
    let callback = unsafe { (*app_state().callbacks.get()).app_lifecycle.take() };
    if let Some(mut callback) = callback {
        callback(phase);
        let callbacks = unsafe { &mut *app_state().callbacks.get() };
        if callbacks.app_lifecycle.is_none() {
            callbacks.app_lifecycle = Some(callback);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_will_enter_foreground(_app: *mut c_void) {
    notify_windows_active(true);
    notify_app_lifecycle(AppLifecyclePhase::Foreground);
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_did_become_active(_app: *mut c_void) {
    notify_windows_active(true);
    notify_app_lifecycle(AppLifecyclePhase::Active);
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_will_resign_active(_app: *mut c_void) {
    notify_windows_active(false);
    notify_app_lifecycle(AppLifecyclePhase::Inactive);
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_did_enter_background(_app: *mut c_void) {
    notify_windows_active(false);
    notify_app_lifecycle(AppLifecyclePhase::Background);
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_did_receive_memory_warning(_app: *mut c_void) {
    let callback = unsafe { (*app_state().callbacks.get()).memory_warning.take() };
    if let Some(mut callback) = callback {
        callback();
        let callbacks = unsafe { &mut *app_state().callbacks.get() };
        if callbacks.memory_warning.is_none() {
            callbacks.memory_warning = Some(callback);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_will_terminate(_app: *mut c_void) {
    let callback = unsafe { (*app_state().callbacks.get()).quit.take() };
    if let Some(mut callback) = callback {
        callback();
        let callbacks = unsafe { &mut *app_state().callbacks.get() };
        if callbacks.quit.is_none() {
            callbacks.quit = Some(callback);
        }
    }
    unsafe {
        (*app_state().application.get()).take();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_handle_touch(
    window: *mut c_void,
    touch: *mut c_void,
    event: *mut c_void,
) {
    let Some(window) = ios_window(window) else {
        return;
    };
    if touch.is_null() {
        return;
    }

    window.handle_touch(
        touch.cast::<objc2::runtime::AnyObject>(),
        event.cast::<objc2::runtime::AnyObject>(),
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_request_frame(window: *mut c_void) {
    let Some(window) = ios_window(window) else {
        return;
    };
    window.request_frame();
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_show_keyboard(window: *mut c_void) {
    if let Some(window) = ios_window(window) {
        window.show_keyboard();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_hide_keyboard(window: *mut c_void) {
    if let Some(window) = ios_window(window) {
        window.hide_keyboard();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_handle_text_input(window: *mut c_void, text: *mut c_void) {
    let Some(window) = ios_window(window) else {
        return;
    };
    if !text.is_null() {
        window.handle_text_input(text.cast::<objc2::runtime::AnyObject>());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_handle_key_event(
    window: *mut c_void,
    key_code: u32,
    modifiers: u32,
    is_key_down: bool,
) {
    if let Some(window) = ios_window(window) {
        window.handle_key_event(key_code, modifiers, is_key_down);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_handle_open_url(url: *mut c_void) {
    if url.is_null() {
        return;
    }

    let url = unsafe {
        let utf8: *const std::ffi::c_char =
            objc2::msg_send![url.cast::<objc2::runtime::AnyObject>(), UTF8String];
        if utf8.is_null() {
            return;
        }
        std::ffi::CStr::from_ptr(utf8)
            .to_string_lossy()
            .into_owned()
    };

    let callback = unsafe { (*app_state().callbacks.get()).open_urls.take() };
    if let Some(mut callback) = callback {
        callback(vec![url]);
        let callbacks = unsafe { &mut *app_state().callbacks.get() };
        if callbacks.open_urls.is_none() {
            callbacks.open_urls = Some(callback);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_run() {
    run_app();
}

pub fn run_app() {
    gpui_ios_initialize();

    let platform = Rc::new(super::IosPlatform::new());
    let application = Application::with_platform(platform).run_embedded(|cx: &mut App| {
        if let Some(callback) = take_app_callback() {
            callback(cx);
        } else {
            log::error!("GPUI iOS: App callback was not registered");
        }
    });
    unsafe {
        *app_state().application.get() = Some(application);
    }

    gpui_ios_did_finish_launching(std::ptr::null_mut());
}
