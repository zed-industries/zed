#![doc = include_str!("../README.md")]
#![allow(unsafe_code)]

mod error;

pub use error::Error;

#[cfg(feature = "debug-print")]
#[macro_export]
macro_rules! debug_print {
    ($s:literal) => {
        let cstr = concat!(file!(), ":", line!(), " ", $s, "\n");
        $crate::write_stderr(cstr);
    };
}

#[cfg(not(feature = "debug-print"))]
#[macro_export]
macro_rules! debug_print {
    ($s:literal) => {};
}

/// Writes the specified string directly to stderr.
///
/// This is safe to be called from within a compromised context.
#[inline]
pub fn write_stderr(s: &'static str) {
    unsafe {
        #[cfg(target_os = "windows")]
        libc::write(2, s.as_ptr().cast(), s.len() as u32);

        #[cfg(not(target_os = "windows"))]
        libc::write(2, s.as_ptr().cast(), s.len());
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(unix, not(target_os = "macos")))] {
        /// The sole purpose of the unix module is to hook `pthread_create` to ensure
        /// an alternate stack is installed for every native thread in case of a
        /// stack overflow. This doesn't apply to `MacOS` as it uses exception ports,
        /// which are always delivered to a specific thread owned by the exception
        /// handler
        pub mod unix;
    }
}

pub use crash_context::CrashContext;

/// The result of the user code executed during a crash event
pub enum CrashEventResult {
    /// The event was handled in some way
    Handled(bool),
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        all(target_os = "windows", target_arch = "x86_64"),
    ))]
    /// The handler wishes to jump somewhere else, presumably to return
    /// execution and skip the code that caused the exception
    Jump {
        /// The location to jump back to, retrieved via sig/setjmp
        jmp_buf: *mut jmp::JmpBuf,
        /// The value that will be returned from the sig/setjmp call that we
        /// jump to. Note that if the value is 0 it will be corrected to 1
        value: i32,
    },
}

impl From<bool> for CrashEventResult {
    fn from(b: bool) -> Self {
        Self::Handled(b)
    }
}

/// User implemented trait for handling a crash event that has ocurred.
///
/// # Safety
///
/// This trait is marked unsafe as care needs to be taken when implementing it
/// due to the [`Self::on_crash`] method being run in a compromised context. In
/// general, it is advised to do as _little_ as possible when handling a
/// crash, with more complicated or dangerous (in a compromised context) code
/// being intialized before the [`CrashHandler`] is installed, or hoisted out to
/// another process entirely.
///
/// ## Linux
///
/// Notably, only a small subset of libc functions are
/// [async signal safe](https://man7.org/linux/man-pages/man7/signal-safety.7.html)
/// and calling non-safe ones can have undefined behavior, including such common
/// ones as `malloc` (especially if using a multi-threaded allocator).
///
/// ## Windows
///
/// Windows [structured exceptions](https://docs.microsoft.com/en-us/windows/win32/debug/structured-exception-handling)
/// don't have the a notion similar to signal safety, but it is again recommended
/// to do as little work as possible in response to an exception.
///
/// ## Macos
///
/// Mac uses exception ports (sorry, can't give a good link here since Apple
/// documentation is terrible) which are handled by a thread owned by the
/// exception handler which makes them slightly safer to handle than UNIX signals,
/// but it is again recommended to do as little work as possible.
pub unsafe trait CrashEvent: Send + Sync {
    /// Method invoked when a crash occurs.
    ///
    /// Returning true indicates your handler has processed the crash and that
    /// no further handlers should run.
    fn on_crash(&self, context: &CrashContext) -> CrashEventResult;
}

/// Creates a [`CrashEvent`] using the supplied closure as the implementation.
///
/// The supplied closure will be called for both real crash events as well as
/// those simulated by calling `simulate_signal/exception`, which is why it is
/// not `FnOnce`
///
/// # Safety
///
/// See the [`CrashEvent`] Safety section for information on why this is `unsafe`.
#[inline]
pub unsafe fn make_crash_event<F>(closure: F) -> Box<dyn CrashEvent>
where
    F: Send + Sync + Fn(&CrashContext) -> CrashEventResult + 'static,
{
    struct Wrapper<F> {
        inner: F,
    }

    unsafe impl<F> CrashEvent for Wrapper<F>
    where
        F: Send + Sync + Fn(&CrashContext) -> CrashEventResult,
    {
        fn on_crash(&self, context: &CrashContext) -> CrashEventResult {
            (self.inner)(context)
        }
    }

    Box::new(Wrapper { inner: closure })
}

/// Creates a [`CrashEvent`] using the supplied closure as the implementation.
///
/// This uses an `FnOnce` closure instead of `Fn` like `[make_crash_event]`, but
/// means this closure can only be used for the first crash, and cannot be used
/// in a situation where user-triggered crashes via the `simulate_signal/exception`
/// methods are used.
///
/// # Safety
///
/// See the [`CrashEvent`] Safety section for information on why this is `unsafe`.
#[inline]
pub unsafe fn make_single_crash_event<F>(closure: F) -> Box<dyn CrashEvent>
where
    F: Send + Sync + FnOnce(&CrashContext) -> CrashEventResult + 'static,
{
    struct Wrapper<F> {
        // technically mutexes are not async signal safe on linux, but this is
        // an internal-only detail that will be safe _unless_ the callback invoked
        // by the user also crashes, but if that occurs...that's on them
        inner: parking_lot::Mutex<Option<F>>,
    }

    unsafe impl<F> CrashEvent for Wrapper<F>
    where
        F: Send + Sync + FnOnce(&CrashContext) -> CrashEventResult,
    {
        fn on_crash(&self, context: &CrashContext) -> CrashEventResult {
            if let Some(inner) = self.inner.lock().take() {
                (inner)(context)
            } else {
                false.into()
            }
        }
    }

    Box::new(Wrapper {
        inner: parking_lot::Mutex::new(Some(closure)),
    })
}

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;

        pub use linux::{CrashHandler, Signal, jmp};
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        #[cfg(target_arch = "x86_64")]
        pub use windows::jmp;

        pub use windows::{CrashHandler, ExceptionCode};
    } else if #[cfg(target_os = "macos")] {
        mod mac;

        pub use mac::{CrashHandler, ExceptionType};
    }
}
