/// Based on rfd crate's gtk backend. Useful to show GTK Widgets asynchronously and by blocking.

use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use gobject_sys::GCallback;
use gtk_sys::{GtkDialog, GtkResponseType};
use std::ffi::c_void;
use std::os::raw::c_char;


pub(crate) trait AsGtkDialog {
    fn gtk_dialog_ptr(&self) -> *mut gtk_sys::GtkDialog;
    unsafe fn show(&self);
}

impl AsGtkDialog for *mut gtk_sys::GtkDialog {
    fn gtk_dialog_ptr(&self) -> *mut gtk_sys::GtkDialog {
        self.cast()
    }
    unsafe fn show(&self) {
        gtk_sys::gtk_widget_show_all(self.cast());
    }
}

static GTK_THREAD: OnceLock<GtkGlobalThread> = OnceLock::new();

/// GTK functions are not thread-safe, and must all be called from the thread that initialized GTK. To ensure this, we
/// spawn one thread the first time a GTK dialog is opened and keep it open for the entire lifetime of the application,
/// as GTK cannot be de-initialized or re-initialized on another thread. You're stuck on the thread on which you first
/// initialize GTK.

pub struct GtkGlobalThread {
    running: Arc<AtomicBool>,
}

impl GtkGlobalThread {
    /// Return the global, lazily-initialized instance of the global GTK thread.
    pub(super) fn instance() -> &'static Self {
        GTK_THREAD.get_or_init(|| Self::new())
    }

    fn new() -> Self {
        // When the GtkGlobalThread is eventually dropped, we will set `running` to false and wake up the loop so
        // gtk_main_iteration unblocks and we exit the thread on the next iteration.
        let running = Arc::new(AtomicBool::new(true));
        let thread_running = Arc::clone(&running);

        spawn(move || {
            let initialized =
                unsafe { gtk_sys::gtk_init_check(ptr::null_mut(), ptr::null_mut()) == 1 };
            if !initialized {
                return;
            }

            loop {
                if !thread_running.load(Ordering::Acquire) {
                    break;
                }

                unsafe {
                    gtk_sys::gtk_main_iteration();
                }
            }
        });

        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Run a function on the GTK thread, blocking on the result which is then passed back.
    pub(super) fn run_blocking<
        T: Send + Clone + std::fmt::Debug + 'static,
        F: FnOnce() -> T + Send + 'static,
    >(
        &self,
        cb: F,
    ) -> T {
        let data: Arc<(Mutex<Option<T>>, _)> = Arc::new((Mutex::new(None), Condvar::new()));
        let thread_data = Arc::clone(&data);
        let mut cb = Some(cb);
        unsafe {
            connect_idle(move || {
                // connect_idle takes a FnMut; convert our FnOnce into that by ensuring we only call it once
                let res = cb.take().expect("Callback should only be called once")();

                // pass the result back to the main thread
                let (lock, cvar) = &*thread_data;
                *lock.lock().unwrap() = Some(res);
                cvar.notify_all();

                glib_sys::GFALSE
            });
        };

        // wait for GTK thread to execute the callback and place the result into `data`
        let lock_res = data
            .1
            .wait_while(data.0.lock().unwrap(), |res| res.is_none())
            .unwrap();
        lock_res.as_ref().unwrap().clone()
    }

    /// Launch a function on the GTK thread without blocking.
    pub(super) fn run<F: FnOnce() + Send + 'static>(&self, cb: F) {
        let mut cb = Some(cb);
        unsafe {
            connect_idle(move || {
                cb.take().expect("Callback should only be called once")();
                glib_sys::GFALSE
            });
        };
    }
}

impl Drop for GtkGlobalThread {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Release);
        unsafe { glib_sys::g_main_context_wakeup(std::ptr::null_mut()) };
    }
}

struct FutureState<R, D> {
    waker: Option<Waker>,
    data: Option<R>,
    dialog: Option<D>,
}

unsafe impl<R, D> Send for FutureState<R, D> {}

pub(super) struct GtkDialogFuture<R, D> {
    state: Arc<Mutex<FutureState<R, D>>>,
}

unsafe impl<R, D> Send for GtkDialogFuture<R, D> {}

impl<R: Default + 'static, D: AsGtkDialog + 'static> GtkDialogFuture<R, D> {
    pub fn new<B, F>(build: B, cb: F) -> Self
        where
            B: FnOnce() -> D + Send + 'static,
            F: Fn(&mut D, i32) -> R + Send + 'static,
    {
        let state = Arc::new(Mutex::new(FutureState {
            waker: None,
            data: None,
            dialog: None,
        }));

        {
            let state = state.clone();
            let callback = {
                let state = state.clone();

                move |res_id| {
                    let mut state = state.lock().unwrap();

                    if let Some(mut dialog) = state.dialog.take() {
                        state.data = Some(cb(&mut dialog, res_id));
                    }

                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                }
            };

            GtkGlobalThread::instance().run(move || {
                let mut state = state.lock().unwrap();
                state.dialog = Some(build());

                unsafe {
                    let dialog = state.dialog.as_ref().unwrap();
                    dialog.show();

                    let ptr = dialog.gtk_dialog_ptr();
                    connect_response(ptr as *mut _, callback);
                }
            });
        }

        Self { state }
    }
}

impl<R, D> std::future::Future for GtkDialogFuture<R, D> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.data.is_some() {
            Poll::Ready(state.data.take().unwrap())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

unsafe fn connect_idle<F: FnMut() -> glib_sys::gboolean + Send + 'static>(f: F) {
    unsafe extern "C" fn response_trampoline<F: FnMut() -> glib_sys::gboolean + Send + 'static>(
        f: glib_sys::gpointer,
    ) -> glib_sys::gboolean {
        let f: &mut F = &mut *(f as *mut F);

        f()
    }
    let f_box: Box<F> = Box::new(f);

    unsafe extern "C" fn destroy_closure<F>(ptr: *mut std::ffi::c_void) {
        // destroy
        let _ = Box::<F>::from_raw(ptr as *mut _);
    }

    glib_sys::g_idle_add_full(
        glib_sys::G_PRIORITY_DEFAULT_IDLE,
        Some(response_trampoline::<F>),
        Box::into_raw(f_box) as glib_sys::gpointer,
        Some(destroy_closure::<F>),
    );
}

unsafe fn connect_raw<F>(
    receiver: *mut gobject_sys::GObject,
    signal_name: *const c_char,
    trampoline: GCallback,
    closure: *mut F,
) {
    use std::mem;

    use glib_sys::gpointer;

    unsafe extern "C" fn destroy_closure<F>(ptr: *mut c_void, _: *mut gobject_sys::GClosure) {
        // destroy
        let _ = Box::<F>::from_raw(ptr as *mut _);
    }
    assert_eq!(mem::size_of::<*mut F>(), mem::size_of::<gpointer>());
    assert!(trampoline.is_some());
    let handle = gobject_sys::g_signal_connect_data(
        receiver,
        signal_name,
        trampoline,
        closure as *mut _,
        Some(destroy_closure::<F>),
        0,
    );
    assert!(handle > 0);
}

unsafe fn connect_response<F: Fn(GtkResponseType) + 'static>(dialog: *mut GtkDialog, f: F) {
    use std::mem::transmute;

    unsafe extern "C" fn response_trampoline<F: Fn(GtkResponseType) + 'static>(
        _this: *mut gtk_sys::GtkDialog,
        res: GtkResponseType,
        f: glib_sys::gpointer,
    ) {
        let f: &F = &*(f as *const F);

        f(res);
    }
    let f: Box<F> = Box::new(f);
    connect_raw(
        dialog as *mut _,
        b"response\0".as_ptr() as *const _,
        Some(transmute::<_, unsafe extern "C" fn()>(
            response_trampoline::<F> as *const (),
        )),
        Box::into_raw(f),
    );
}
