use android_activity::ndk::looper::{ForeignLooper, ThreadLooper};
use android_activity::{AndroidAppWaker, ndk_sys};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::mpsc;

/// Wakes the main thread at the next display refresh, once per request.
///
/// `AChoreographer` delivers its callbacks through the calling thread's
/// looper. `android-activity` owns the main thread's looper and treats
/// callbacks it did not register as spurious, so the choreographer gets a
/// thread and looper of its own.
#[derive(Clone)]
pub(crate) struct Vsync {
    state: Arc<VsyncState>,
    looper: ForeignLooper,
}

struct VsyncState {
    main_thread: AndroidAppWaker,
    /// A refresh has happened that the main thread has not yet taken.
    pending: AtomicBool,
    /// One of [`IDLE`], [`REQUESTED`] or [`POSTED`].
    callback: AtomicU8,
}

const IDLE: u8 = 0;
/// The main thread wants a refresh that the vsync thread has yet to ask the
/// choreographer for.
const REQUESTED: u8 = 1;
/// A frame callback is registered with the choreographer.
const POSTED: u8 = 2;

unsafe extern "C" fn on_frame(_frame_time_nanos: i64, data: *mut c_void) {
    // SAFETY: `data` is the `VsyncState` pointer passed to
    // `AChoreographer_postFrameCallback64` by the vsync thread, which holds
    // an `Arc` to it for as long as it polls its looper.
    let state = unsafe { &*(data as *const VsyncState) };
    // Idle first, so that a request made in response to this refresh
    // registers a callback for the following one.
    state.callback.store(IDLE, Ordering::Release);
    state.pending.store(true, Ordering::Release);
    state.main_thread.wake();
}

impl Vsync {
    pub(crate) fn new(main_thread: AndroidAppWaker) -> Self {
        let state = Arc::new(VsyncState {
            main_thread,
            pending: AtomicBool::new(false),
            callback: AtomicU8::new(IDLE),
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
                        if state
                            .callback
                            .compare_exchange(
                                REQUESTED,
                                POSTED,
                                Ordering::AcqRel,
                                Ordering::Acquire,
                            )
                            .is_ok()
                        {
                            // SAFETY: this thread has a looper, and `on_frame`
                            // receives a pointer to state that this thread
                            // keeps alive for as long as it polls.
                            unsafe {
                                ndk_sys::AChoreographer_postFrameCallback64(
                                    ndk_sys::AChoreographer_getInstance(),
                                    Some(on_frame),
                                    Arc::as_ptr(&state) as *mut c_void,
                                );
                            }
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

    /// Asks for one wakeup at the next display refresh. Requests made before
    /// that refresh share it; without a request the display refreshes unobserved.
    pub(crate) fn request(&self) {
        if self
            .state
            .callback
            .compare_exchange(IDLE, REQUESTED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.looper.wake();
        }
    }

    /// Returns whether a requested refresh happened since the last call.
    pub(crate) fn take_pending(&self) -> bool {
        self.state.pending.swap(false, Ordering::AcqRel)
    }
}
