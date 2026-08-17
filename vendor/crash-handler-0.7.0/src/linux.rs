pub mod jmp;
mod state;

use crate::Error;

/// The signals that we support catching and raising
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Signal {
    Abort = libc::SIGABRT,
    Bus = libc::SIGBUS,
    Fpe = libc::SIGFPE,
    Illegal = libc::SIGILL,
    Segv = libc::SIGSEGV,
    Trap = libc::SIGTRAP,
}

impl Signal {
    #[inline]
    pub fn ignore(self) {
        unsafe {
            state::ignore_signal(self);
        }
    }
}

/// A Linux/Android signal handler
pub struct CrashHandler;

#[allow(clippy::unused_self)]
impl CrashHandler {
    /// Attaches the signal handler.
    ///
    /// The provided callback will be invoked if a signal is caught, providing a
    /// [`crate::CrashContext`] with the details of the thread where the
    /// signal was raised.
    ///
    /// The callback runs in a compromised context, so it is highly recommended
    /// to not perform actions that may fail due to corrupted state that caused
    /// or is a symptom of the original signal. This includes doing heap
    /// allocations from the same allocator as the crashing code.
    pub fn attach(on_crash: Box<dyn crate::CrashEvent>) -> Result<Self, Error> {
        state::attach(on_crash)?;
        Ok(Self)
    }

    /// Detaches the handler.
    ///
    /// This is done automatically when this [`CrashHandler`] is dropped.
    #[inline]
    pub fn detach(self) {
        state::detach();
    }

    /// Set the process that is allowed to perform `ptrace` operations on the
    /// current process.
    ///
    /// If you want to write a minidump from a child/external process when
    /// a crash occurs in this process, you can use this method to set that
    /// process as the only process allowed to use ptrace on this process.
    ///
    /// The process set by this method will be used by calling
    /// `prctl(PR_SET_PTRACER, <the pid you want to ptrace this process>, ...)`
    /// before handing off control to your user callback, presumably to trigger
    /// dumping of your process via the specified process. By default if this
    /// method is not called, `PR_SET_PTRACER_ANY` is used to allow any process
    /// to dump the current process.
    ///
    /// Note that this is only needed if `/proc/sys/kernel/yama/ptrace_scope`
    /// is 1 "restricted ptrace", but there is no harm in setting this if it is
    /// in another mode.
    ///
    /// See <https://www.kernel.org/doc/Documentation/security/Yama.txt> for
    /// the full documentation.
    #[inline]
    pub fn set_ptracer(&self, pid: Option<u32>) {
        let mut lock = state::HANDLER.lock();

        if let Some(handler) = &mut *lock {
            handler.dump_process = pid;
        }
    }

    /// Sends the specified user signal.
    pub fn simulate_signal(&self, signal: u32) -> crate::CrashEventResult {
        // Normally this would be an unsafe function, since this unsafe encompasses
        // the entirety of the body, however the user is really not required to
        // uphold any guarantees on their end, so no real need to declare the
        // function itself unsafe.
        unsafe {
            let mut siginfo: libc::signalfd_siginfo = std::mem::zeroed();
            siginfo.ssi_signo = signal;
            siginfo.ssi_code = state::SI_USER;
            siginfo.ssi_pid = std::process::id();

            let mut context = std::mem::zeroed();
            crash_context::crash_context_getcontext(&mut context);

            let lock = state::HANDLER.lock();
            if let Some(handler) = &*lock {
                handler.handle_signal(
                    &mut *(&mut siginfo as *mut libc::signalfd_siginfo).cast::<libc::siginfo_t>(),
                    &mut *(&mut context as *mut crash_context::ucontext_t).cast::<libc::c_void>(),
                )
            } else {
                crate::CrashEventResult::Handled(false)
            }
        }
    }
}

impl Drop for CrashHandler {
    fn drop(&mut self) {
        state::detach();
    }
}
