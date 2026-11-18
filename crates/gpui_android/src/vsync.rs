use android_activity::ndk::looper::{ForeignLooper, ThreadLooper};
use android_activity::{AndroidAppWaker, ndk_sys};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

/// Wakes the main thread once per display refresh while running.
///
/// `AChoreographer` delivers its callbacks through the calling thread's
/// looper. `android-activity` owns the main thread's looper and treats
/// callbacks it did not register as spurious, so the choreographer gets a
/// thread and looper of its own.
pub(crate) struct Vsync {
    state: Arc<VsyncState>,
    looper: ForeignLooper,
}

struct VsyncState {
    main_thread: AndroidAppWaker,
    /// A refresh has happened that the main thread has not yet taken.
    pending: AtomicBool,
    running: AtomicBool,
    /// A frame callback is registered with the choreographer.
    posted: AtomicBool,
}

impl VsyncState {
    fn post_frame_callback(self: &Arc<Self>) {
        // SAFETY: this runs on the vsync thread, which has a looper, and
        // `on_frame` receives a pointer to state that the thread keeps alive
        // for as long as it polls.
        unsafe {
            ndk_sys::AChoreographer_postFrameCallback64(
                ndk_sys::AChoreographer_getInstance(),
                Some(on_frame),
                Arc::as_ptr(self) as *mut c_void,
            );
        }
    }
}

unsafe extern "C" fn on_frame(_frame_time_nanos: i64, data: *mut c_void) {
    // SAFETY: `data` is the `Arc<VsyncState>` pointer passed by
    // `post_frame_callback`; borrowing it as an `Arc` without taking
    // ownership leaves the thread's reference count untouched.
    let state = std::mem::ManuallyDrop::new(unsafe { Arc::from_raw(data as *const VsyncState) });
    state.pending.store(true, Ordering::Release);
    state.main_thread.wake();
    if state.running.load(Ordering::Acquire) {
        state.post_frame_callback();
    } else {
        state.posted.store(false, Ordering::Release);
    }
}

impl Vsync {
    pub(crate) fn new(main_thread: AndroidAppWaker) -> Self {
        let state = Arc::new(VsyncState {
            main_thread,
            pending: AtomicBool::new(false),
            running: AtomicBool::new(false),
            posted: AtomicBool::new(false),
        });
        let (looper_sender, looper_receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("Vsync".to_owned())
            .spawn({
                let state = Arc::clone(&state);
                move || {
                    let looper = ThreadLooper::prepare();
                    looper_sender.send(looper.as_foreign().clone()).ok();
                    loop {
                        if state.running.load(Ordering::Acquire)
                            && !state.posted.swap(true, Ordering::AcqRel)
                        {
                            state.post_frame_callback();
                        }
                        if looper.poll_once().is_err() {
                            log::error!("vsync looper failed; frames will stop");
                            break;
                        }
                    }
                }
            })
            .expect("failed to spawn the vsync thread");
        let looper = looper_receiver
            .recv()
            .expect("vsync thread exited before preparing its looper");
        Self { state, looper }
    }

    /// Starts or stops refresh wakeups, which cost power while nothing can be shown.
    pub(crate) fn set_running(&self, running: bool) {
        self.state.running.store(running, Ordering::Release);
        self.looper.wake();
    }

    /// Returns whether the display refreshed since the last call.
    pub(crate) fn take_pending(&self) -> bool {
        self.state.pending.swap(false, Ordering::AcqRel)
    }
}
