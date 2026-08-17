// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
use std::os::unix::io::RawFd;
use std::{cell::RefCell, mem::transmute, num::NonZeroU32, time::Duration};

use crate::ffi::{self, gboolean, gpointer};
#[cfg(all(not(unix), docsrs))]
use libc::c_int as RawFd;

#[cfg(unix)]
use crate::IOCondition;
use crate::{thread_guard::ThreadGuard, translate::*, ControlFlow, MainContext, Source};

// rustdoc-stripper-ignore-next
/// The id of a source that is returned by `idle_add` and `timeout_add`.
///
/// This type does not implement `Clone` to prevent calling [`SourceId::remove`]
/// multiple times on the same source.
#[derive(Debug, Eq, PartialEq)]
pub struct SourceId(NonZeroU32);

impl SourceId {
    // rustdoc-stripper-ignore-next
    /// Returns the internal source ID.
    pub fn as_raw(&self) -> u32 {
        self.0.get()
    }

    // rustdoc-stripper-ignore-next
    /// Removes the source with the given id `source_id` from the default main context.
    ///
    /// It is a programmer error to attempt to remove a non-existent source.
    #[doc(alias = "g_source_remove")]
    pub fn remove(self) {
        unsafe {
            result_from_gboolean!(
                ffi::g_source_remove(self.as_raw()),
                "Failed to remove source"
            )
            .unwrap()
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for SourceId {
    #[inline]
    unsafe fn from_glib(val: u32) -> Self {
        debug_assert_ne!(val, 0);
        Self(NonZeroU32::new_unchecked(val))
    }
}

// rustdoc-stripper-ignore-next
/// Process identifier
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[doc(alias = "GPid")]
pub struct Pid(pub ffi::GPid);

unsafe impl Send for Pid {}
unsafe impl Sync for Pid {}

#[doc(hidden)]
impl IntoGlib for Pid {
    type GlibType = ffi::GPid;

    #[inline]
    fn into_glib(self) -> ffi::GPid {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GPid> for Pid {
    #[inline]
    unsafe fn from_glib(val: ffi::GPid) -> Self {
        Self(val)
    }
}

unsafe extern "C" fn trampoline<F: FnMut() -> ControlFlow + Send + 'static>(
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (*func.borrow_mut())().into_glib()
}

unsafe extern "C" fn trampoline_local<F: FnMut() -> ControlFlow + 'static>(
    func: gpointer,
) -> gboolean {
    let func: &ThreadGuard<RefCell<F>> = &*(func as *const ThreadGuard<RefCell<F>>);
    (*func.get_ref().borrow_mut())().into_glib()
}

unsafe extern "C" fn destroy_closure<F: FnMut() -> ControlFlow + Send + 'static>(ptr: gpointer) {
    let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn destroy_closure_local<F: FnMut() -> ControlFlow + 'static>(ptr: gpointer) {
    let _ = Box::<ThreadGuard<RefCell<F>>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> ControlFlow + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

fn into_raw_local<F: FnMut() -> ControlFlow + 'static>(func: F) -> gpointer {
    let func: Box<ThreadGuard<RefCell<F>>> = Box::new(ThreadGuard::new(RefCell::new(func)));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_child_watch<F: FnMut(Pid, i32) + Send + 'static>(
    pid: ffi::GPid,
    status: i32,
    func: gpointer,
) {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (*func.borrow_mut())(Pid(pid), status)
}

unsafe extern "C" fn trampoline_child_watch_local<F: FnMut(Pid, i32) + 'static>(
    pid: ffi::GPid,
    status: i32,
    func: gpointer,
) {
    let func: &ThreadGuard<RefCell<F>> = &*(func as *const ThreadGuard<RefCell<F>>);
    (*func.get_ref().borrow_mut())(Pid(pid), status)
}

unsafe extern "C" fn destroy_closure_child_watch<F: FnMut(Pid, i32) + Send + 'static>(
    ptr: gpointer,
) {
    let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn destroy_closure_child_watch_local<F: FnMut(Pid, i32) + 'static>(
    ptr: gpointer,
) {
    let _ = Box::<ThreadGuard<RefCell<F>>>::from_raw(ptr as *mut _);
}

fn into_raw_child_watch<F: FnMut(Pid, i32) + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

fn into_raw_child_watch_local<F: FnMut(Pid, i32) + 'static>(func: F) -> gpointer {
    let func: Box<ThreadGuard<RefCell<F>>> = Box::new(ThreadGuard::new(RefCell::new(func)));
    Box::into_raw(func) as gpointer
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
unsafe extern "C" fn trampoline_unix_fd<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
>(
    fd: i32,
    condition: ffi::GIOCondition,
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (*func.borrow_mut())(fd, from_glib(condition)).into_glib()
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
unsafe extern "C" fn trampoline_unix_fd_local<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
>(
    fd: i32,
    condition: ffi::GIOCondition,
    func: gpointer,
) -> gboolean {
    let func: &ThreadGuard<RefCell<F>> = &*(func as *const ThreadGuard<RefCell<F>>);
    (*func.get_ref().borrow_mut())(fd, from_glib(condition)).into_glib()
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
unsafe extern "C" fn destroy_closure_unix_fd<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
unsafe extern "C" fn destroy_closure_unix_fd_local<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<ThreadGuard<RefCell<F>>>::from_raw(ptr as *mut _);
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
fn into_raw_unix_fd<F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static>(
    func: F,
) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
fn into_raw_unix_fd_local<F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static>(
    func: F,
) -> gpointer {
    let func: Box<ThreadGuard<RefCell<F>>> = Box::new(ThreadGuard::new(RefCell::new(func)));
    Box::into_raw(func) as gpointer
}

// rustdoc-stripper-ignore-next
/// Transform a generic FnOnce into a closure that can be used as callback in various glib methods
///
/// The resulting function can only be called once and will panic otherwise. It will return `ControlFlow::Break`
/// in order to prevent being called twice.
#[inline(always)]
fn fnmut_callback_wrapper(
    func: impl FnOnce() + Send + 'static,
) -> impl FnMut() -> ControlFlow + Send + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning ControlFlow::Break");
        func();
        ControlFlow::Break
    }
}

// rustdoc-stripper-ignore-next
/// Transform a generic FnOnce into a closure that can be used as callback in various glib methods
///
/// The resulting function can only be called once and will panic otherwise. It will return `ControlFlow::Break`
/// in order to prevent being called twice.
///
/// Different to `fnmut_callback_wrapper()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
#[inline(always)]
fn fnmut_callback_wrapper_local(
    func: impl FnOnce() + 'static,
) -> impl FnMut() -> ControlFlow + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning glib::ControlFlow::Break");
        func();
        ControlFlow::Break
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add<F>(func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_idle_add_full(
            ffi::G_PRIORITY_DEFAULT_IDLE,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly with `priority` until it returns
/// `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_full<F>(priority: Priority, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_idle_add_full(
            priority.into_glib(),
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// In comparison to `idle_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_idle_add_full")]
#[doc(alias = "g_idle_add_once")]
pub fn idle_add_once<F>(func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    idle_add(fnmut_callback_wrapper(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `idle_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the default main context.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_local<F>(func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_idle_add_full(
            ffi::G_PRIORITY_DEFAULT_IDLE,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly with `priority` until it returns
/// `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `idle_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the default main context.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_local_full<F>(priority: Priority, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_idle_add_full(
            priority.into_glib(),
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `idle_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `idle_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_local_once<F>(func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    idle_add_local(fnmut_callback_wrapper_local(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add<F>(interval: Duration, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_timeout_add_full(
            ffi::G_PRIORITY_DEFAULT,
            interval.as_millis() as _,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds with `priority`
/// until it returns `ControlFlow::Break`. Precise timing is not guaranteed, the
/// timeout may be delayed by other events. Prefer `timeout_add_seconds` when
/// millisecond precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_full<F>(interval: Duration, priority: Priority, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_timeout_add_full(
            priority.into_glib(),
            interval.as_millis() as _,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// In comparison to `timeout_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_timeout_add_full")]
#[doc(alias = "g_timeout_add_once")]
pub fn timeout_add_once<F>(interval: Duration, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    timeout_add(interval, fnmut_callback_wrapper(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `timeout_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_local<F>(interval: Duration, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_timeout_add_full(
            ffi::G_PRIORITY_DEFAULT,
            interval.as_millis() as _,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds with `priority`
/// until it returns `ControlFlow::Break`. Precise timing is not guaranteed, the
/// timeout may be delayed by other events. Prefer `timeout_add_seconds` when
/// millisecond precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `timeout_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_local_full<F>(interval: Duration, priority: Priority, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_timeout_add_full(
            priority.into_glib(),
            interval.as_millis() as _,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `timeout_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `timeout_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_local_once<F>(interval: Duration, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    timeout_add_local(interval, fnmut_callback_wrapper_local(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_timeout_add_seconds_full(
            ffi::G_PRIORITY_DEFAULT,
            interval,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// In comparison to `timeout_add_seconds()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_once<F>(interval: u32, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    timeout_add_seconds(interval, fnmut_callback_wrapper(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `timeout_add_seconds()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_local<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_timeout_add_seconds_full(
            ffi::G_PRIORITY_DEFAULT,
            interval,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `timeout_add_seconds()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `timeout_add_seconds_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_local_once<F>(interval: u32, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    timeout_add_seconds_local(interval, fnmut_callback_wrapper_local(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
#[doc(alias = "g_child_watch_add_full")]
pub fn child_watch_add<F>(pid: Pid, func: F) -> SourceId
where
    F: FnMut(Pid, i32) + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_child_watch_add_full(
            ffi::G_PRIORITY_DEFAULT,
            pid.0,
            Some(trampoline_child_watch::<F>),
            into_raw_child_watch(func),
            Some(destroy_closure_child_watch::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
///
/// Different to `child_watch_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_child_watch_add_full")]
pub fn child_watch_add_local<F>(pid: Pid, func: F) -> SourceId
where
    F: FnMut(Pid, i32) + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_child_watch_add_full(
            ffi::G_PRIORITY_DEFAULT,
            pid.0,
            Some(trampoline_child_watch_local::<F>),
            into_raw_child_watch_local(func),
            Some(destroy_closure_child_watch_local::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_signal_add_full(
            ffi::G_PRIORITY_DEFAULT,
            signum,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// In comparison to `unix_signal_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    unix_signal_add(signum, fnmut_callback_wrapper(func))
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_signal_add_full(
            ffi::G_PRIORITY_DEFAULT,
            signum,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `unix_signal_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    unix_signal_add_local(signum, fnmut_callback_wrapper_local(func))
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_fd_add_full(
            ffi::G_PRIORITY_DEFAULT,
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly with `priority` while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_full<F>(
    fd: RawFd,
    priority: Priority,
    condition: IOCondition,
    func: F,
) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_fd_add_full(
            priority.into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_fd_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_local<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_fd_add_full(
            ffi::G_PRIORITY_DEFAULT,
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd_local::<F>),
            into_raw_unix_fd_local(func),
            Some(destroy_closure_unix_fd_local::<F>),
        ))
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly with `priority` while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_fd_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_local_full<F>(
    fd: RawFd,
    priority: Priority,
    condition: IOCondition,
    func: F,
) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
{
    unsafe {
        let context = MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_fd_add_full(
            priority.into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd_local::<F>),
            into_raw_unix_fd_local(func),
            Some(destroy_closure_unix_fd_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// The priority of sources
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Priority(i32);

impl Priority {
    #[doc(alias = "G_PRIORITY_HIGH")]
    pub const HIGH: Self = Self(ffi::G_PRIORITY_HIGH);
    #[doc(alias = "G_PRIORITY_DEFAULT")]
    pub const DEFAULT: Self = Self(ffi::G_PRIORITY_DEFAULT);
    #[doc(alias = "G_PRIORITY_HIGH_IDLE")]
    pub const HIGH_IDLE: Self = Self(ffi::G_PRIORITY_HIGH_IDLE);
    #[doc(alias = "G_PRIORITY_DEFAULT_IDLE")]
    pub const DEFAULT_IDLE: Self = Self(ffi::G_PRIORITY_DEFAULT_IDLE);
    #[doc(alias = "G_PRIORITY_LOW")]
    pub const LOW: Self = Self(ffi::G_PRIORITY_LOW);
}

impl Default for Priority {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[doc(hidden)]
impl IntoGlib for Priority {
    type GlibType = i32;

    #[inline]
    fn into_glib(self) -> i32 {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<i32> for Priority {
    #[inline]
    unsafe fn from_glib(val: i32) -> Self {
        Self::from(val)
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the return `Source` is attached to when it's idle.
///
/// `func` will be called repeatedly until it returns `ControlFlow::Break`.
#[doc(alias = "g_idle_source_new")]
pub fn idle_source_new<F>(name: Option<&str>, priority: Priority, func: F) -> Source
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_idle_source_new();
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
#[doc(alias = "g_timeout_source_new")]
pub fn timeout_source_new<F>(
    interval: Duration,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_timeout_source_new(interval.as_millis() as _);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `ControlFlow::Break`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
#[doc(alias = "g_timeout_source_new_seconds")]
pub fn timeout_source_new_seconds<F>(
    interval: u32,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_timeout_source_new_seconds(interval);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
#[doc(alias = "g_child_watch_source_new")]
pub fn child_watch_source_new<F>(
    pid: Pid,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut(Pid, i32) + Send + 'static,
{
    unsafe {
        let source = ffi::g_child_watch_source_new(pid.0);
        ffi::g_source_set_callback(
            source,
            Some(transmute::<
                *mut (),
                unsafe extern "C" fn(ffi::gpointer) -> ffi::gboolean,
            >(trampoline_child_watch::<F> as *mut ())),
            into_raw_child_watch(func),
            Some(destroy_closure_child_watch::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_source_new")]
pub fn unix_signal_source_new<F>(
    signum: i32,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_signal_source_new(signum);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
#[doc(alias = "g_unix_fd_source_new")]
pub fn unix_fd_source_new<F>(
    fd: RawFd,
    condition: IOCondition,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_fd_source_new(fd, condition.into_glib());
        ffi::g_source_set_callback(
            source,
            Some(transmute::<
                *const (),
                unsafe extern "C" fn(ffi::gpointer) -> ffi::gboolean,
            >(trampoline_unix_fd::<F> as *const ())),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

impl Source {
    #[doc(alias = "g_source_attach")]
    pub fn attach(&self, context: Option<&MainContext>) -> SourceId {
        unsafe {
            from_glib(ffi::g_source_attach(
                self.to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_source_get_context")]
    #[doc(alias = "get_context")]
    #[doc(alias = "g_source_dup_context")]
    pub fn context(&self) -> Option<MainContext> {
        #[cfg(feature = "v2_86")]
        unsafe {
            from_glib_full(ffi::g_source_dup_context(self.to_glib_none().0))
        }
        #[cfg(not(feature = "v2_86"))]
        unsafe {
            from_glib_none(ffi::g_source_get_context(self.to_glib_none().0))
        }
    }
}
