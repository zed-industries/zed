use crate::{Error, Signal};
use std::{mem, ptr};

// std::cmp::max is not const :(
const fn get_stack_size() -> usize {
    if libc::SIGSTKSZ > 16 * 1024 {
        libc::SIGSTKSZ
    } else {
        16 * 1024
    }
}

/// The size of the alternate stack that is mapped for every thread.
///
/// This has a minimum size of 16k, which might seem a bit large, but this
/// memory will only ever be committed in case we actually get a stack overflow,
/// which is (hopefully) exceedingly rare
const SIG_STACK_SIZE: usize = get_stack_size();

/// kill
pub(crate) const SI_USER: i32 = 0;

struct StackSave {
    old: Option<libc::stack_t>,
    new: libc::stack_t,
}

unsafe impl Send for StackSave {}

static STACK_SAVE: parking_lot::Mutex<Option<StackSave>> = parking_lot::const_mutex(None);

/// Create an alternative stack to run the signal handlers on. This is done since
/// the signal might have been caused by a stack overflow.
pub unsafe fn install_sigaltstack() -> Result<(), Error> {
    unsafe {
        // Check to see if the existing sigaltstack, and if it exists, is it big
        // enough. If so we don't need to allocate our own.
        let mut old_stack = mem::zeroed();
        let r = libc::sigaltstack(ptr::null(), &mut old_stack);
        assert_eq!(
            r,
            0,
            "learning about sigaltstack failed: {}",
            std::io::Error::last_os_error()
        );

        if old_stack.ss_flags & libc::SS_DISABLE == 0 && old_stack.ss_size >= SIG_STACK_SIZE {
            return Ok(());
        }

        // ... but failing that we need to allocate our own, so do all that
        // here.
        let guard_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let alloc_size = guard_size + SIG_STACK_SIZE;

        let ptr = libc::mmap(
            ptr::null_mut(),
            alloc_size,
            libc::PROT_NONE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if ptr == libc::MAP_FAILED {
            return Err(Error::OutOfMemory);
        }

        // Prepare the stack with readable/writable memory and then register it
        // with `sigaltstack`.
        let stack_ptr = (ptr as usize + guard_size) as *mut libc::c_void;
        let r = libc::mprotect(
            stack_ptr,
            SIG_STACK_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
        );
        assert_eq!(
            r,
            0,
            "mprotect to configure memory for sigaltstack failed: {}",
            std::io::Error::last_os_error()
        );
        let new_stack = libc::stack_t {
            ss_sp: stack_ptr,
            ss_flags: 0,
            ss_size: SIG_STACK_SIZE,
        };
        let r = libc::sigaltstack(&new_stack, ptr::null_mut());
        assert_eq!(
            r,
            0,
            "registering new sigaltstack failed: {}",
            std::io::Error::last_os_error()
        );

        *STACK_SAVE.lock() = Some(StackSave {
            old: (old_stack.ss_flags & libc::SS_DISABLE != 0).then_some(old_stack),
            new: new_stack,
        });

        Ok(())
    }
}

pub unsafe fn restore_sigaltstack() {
    let mut ssl = STACK_SAVE.lock();

    // Only restore the old_stack if the current alternative stack is the one
    // installed by the call to install_sigaltstack.
    if let Some(ss) = &mut *ssl {
        unsafe {
            let mut current_stack = mem::zeroed();
            if libc::sigaltstack(ptr::null(), &mut current_stack) == -1 {
                return;
            }

            if current_stack.ss_sp == ss.new.ss_sp {
                if let Some(old) = ss.old {
                    // Restore the old alt stack if there was one
                    if libc::sigaltstack(&old, ptr::null_mut()) == -1 {
                        return;
                    }
                } else {
                    // Restore to the default alt stack otherwise
                    let mut disable: libc::stack_t = mem::zeroed();
                    disable.ss_flags = libc::SS_DISABLE;
                    if libc::sigaltstack(&disable, ptr::null_mut()) == -1 {
                        return;
                    }
                }
            }

            let r = libc::munmap(ss.new.ss_sp, ss.new.ss_size);
            debug_assert_eq!(r, 0, "munmap failed during thread shutdown");
            *ssl = None;
        }
    }
}

/// Restores the signal handler for the specified signal back to its default
/// handler, which _should_ perform the default signal action as seen in
/// <https://man7.org/linux/man-pages/man7/signal.7.html>
#[inline]
unsafe fn install_default_handler(sig: Signal) {
    unsafe { set_handler(sig, libc::SIG_DFL) };
}

#[inline]
pub(crate) unsafe fn ignore_signal(sig: Signal) {
    unsafe { set_handler(sig, libc::SIG_IGN) };
}

unsafe fn set_handler(sig: Signal, action: usize) {
    // Android L+ expose signal and sigaction symbols that override the system
    // ones. There is a bug in these functions where a request to set the handler
    // to SIG_DFL is ignored. In that case, an infinite loop is entered as the
    // signal is repeatedly sent to breakpad's signal handler.
    // To work around this, directly call the system's sigaction.
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "android")] {
                let mut sa: libc::sigaction = mem::zeroed();
                libc::sigemptyset(&mut sa.sa_mask);
                sa.sa_sigaction = action;
                sa.sa_flags = libc::SA_RESTART;
                libc::syscall(
                    libc::SYS_rt_sigaction,
                    sig as i32,
                    &sa,
                    ptr::null::<libc::sigaction>(),
                    mem::size_of::<libc::sigset_t>(),
                );
            } else {
                libc::signal(sig as i32, action);
            }
        }
    }
}

/// The various signals we attempt to handle
const EXCEPTION_SIGNALS: [Signal; 6] = [
    Signal::Abort,
    Signal::Bus,
    Signal::Fpe,
    Signal::Illegal,
    Signal::Segv,
    Signal::Trap,
];

static OLD_HANDLERS: parking_lot::Mutex<Option<[libc::sigaction; 6]>> =
    parking_lot::const_mutex(None);

/// Restores all of the signal handlers back to their previous values, or the
/// default if the previous value cannot be restored
pub unsafe fn restore_handlers() {
    let mut ohl = OLD_HANDLERS.lock();

    if let Some(old) = &*ohl {
        unsafe {
            for (sig, action) in EXCEPTION_SIGNALS.into_iter().zip(old.iter()) {
                if libc::sigaction(sig as i32, action, ptr::null_mut()) == -1 {
                    install_default_handler(sig);
                }
            }
        }
    }

    ohl.take();
}

pub unsafe fn install_handlers() {
    let mut ohl = OLD_HANDLERS.lock();

    if ohl.is_some() {
        return;
    }

    unsafe {
        // Attempt store all of the current handlers so we can restore them later
        let mut old_handlers: [mem::MaybeUninit<libc::sigaction>; 6] =
            mem::MaybeUninit::uninit().assume_init();

        for (sig, handler) in EXCEPTION_SIGNALS
            .iter()
            .copied()
            .zip(old_handlers.iter_mut())
        {
            let mut old = mem::zeroed();
            if libc::sigaction(sig as i32, ptr::null(), &mut old) == -1 {
                return;
            }
            *handler = mem::MaybeUninit::new(old);
        }

        let mut sa: libc::sigaction = mem::zeroed();
        libc::sigemptyset(&mut sa.sa_mask);

        // Mask all exception signals when we're handling one of them.
        for sig in EXCEPTION_SIGNALS {
            libc::sigaddset(&mut sa.sa_mask, sig as i32);
        }

        sa.sa_sigaction = signal_handler as usize;
        sa.sa_flags = libc::SA_ONSTACK | libc::SA_SIGINFO;

        // Use our signal_handler for all of the signals we wish to catch
        for sig in EXCEPTION_SIGNALS {
            // At this point it is impractical to back out changes, and so failure to
            // install a signal is intentionally ignored.
            let _ = libc::sigaction(sig as i32, &sa, ptr::null_mut());
        }

        // Everything is initialized. Transmute the array to the
        // initialized type.
        let old_handlers = old_handlers.map(|h| h.assume_init());
        *ohl = Some(old_handlers);
    }
}

pub(super) fn attach(on_crash: Box<dyn crate::CrashEvent>) -> Result<(), Error> {
    let mut lock = HANDLER.lock();

    if lock.is_some() {
        return Err(Error::HandlerAlreadyInstalled);
    }

    // SAFETY: syscalls
    unsafe {
        install_sigaltstack()?;
        install_handlers();
    }

    *lock = Some(HandlerInner::new(on_crash));

    Ok(())
}

/// Detaches our signal handle, restoring the previously installed or default
/// handlers
pub(super) fn detach() {
    let mut lock = HANDLER.lock();
    if lock.is_some() {
        // SAFETY: syscalls
        unsafe {
            restore_sigaltstack();
            restore_handlers();
        }
        lock.take();
    }
}

pub(super) static HANDLER: parking_lot::Mutex<Option<HandlerInner>> =
    parking_lot::const_mutex(None);

/// This is the actual function installed for each signal we support, invoked
/// by the kernel
unsafe extern "C" fn signal_handler(
    sig: Signal,
    info: *mut libc::siginfo_t,
    uc: *mut libc::c_void,
) {
    unsafe {
        let info = &mut *info;
        let uc = &mut *uc;

        enum Action {
            RestoreDefault,
            RestorePrevious,
            Jump((*mut super::jmp::JmpBuf, i32)),
        }

        let action = {
            // We might run inside a process where some other buggy code saves and
            // restores signal handlers temporarily with `signal` instead of `sigaction`.
            // This loses the `SA_SIGINFO` flag associated with this function. As a
            // consequence, the values of `info` and `uc` become totally bogus,
            // generally inducing a crash.
            //
            // The following code tries to detect this case. When it does, it
            // resets the signal handlers with `sigaction` & `SA_SIGINFO` and returns.
            // This forces the signal to be thrown again, but this time the kernel
            // will call the function with the right arguments.
            {
                let mut cur_handler = mem::zeroed();
                if libc::sigaction(sig as i32, ptr::null_mut(), &mut cur_handler) == 0
                    && cur_handler.sa_sigaction == signal_handler as usize
                    && cur_handler.sa_flags & libc::SA_SIGINFO == 0
                {
                    // Reset signal handler with the correct flags.
                    libc::sigemptyset(&mut cur_handler.sa_mask);
                    libc::sigaddset(&mut cur_handler.sa_mask, sig as i32);

                    cur_handler.sa_sigaction = signal_handler as usize;
                    cur_handler.sa_flags = libc::SA_ONSTACK | libc::SA_SIGINFO;

                    if libc::sigaction(sig as i32, &cur_handler, ptr::null_mut()) == -1 {
                        // When resetting the handler fails, try to reset the
                        // default one to avoid an infinite loop here.
                        install_default_handler(sig);
                    }

                    // exit the handler as we should be called again soon
                    return;
                }
            }

            let handler = HANDLER.lock();

            if let Some(handler) = &*handler {
                match handler.handle_signal(info, uc) {
                    crate::CrashEventResult::Handled(true) => Action::RestoreDefault,
                    crate::CrashEventResult::Handled(false) => Action::RestorePrevious,
                    crate::CrashEventResult::Jump { jmp_buf, value } => {
                        Action::Jump((jmp_buf, value))
                    }
                }
            } else {
                Action::RestorePrevious
            }
        };

        // Upon returning from this signal handler, sig will become unmasked and
        // then it will be retriggered. If one of the ExceptionHandlers handled
        // it successfully, restore the default handler. Otherwise, restore the
        // previously installed handler. Then, when the signal is retriggered,
        // it will be delivered to the appropriate handler.
        match action {
            Action::RestoreDefault => {
                debug_print!("installing default handler");
                install_default_handler(sig);
            }
            Action::RestorePrevious => {
                debug_print!("restoring handlers");
                restore_handlers();
            }
            Action::Jump((jmp_buf, value)) => {
                debug_print!("jumping");
                super::jmp::siglongjmp(jmp_buf, value);
            }
        }

        debug_print!("finishing signal handler");

        if info.si_code <= 0 || sig == Signal::Abort {
            // This signal was triggered by somebody sending us the signal with kill().
            // In order to retrigger it, we have to queue a new signal by calling
            // kill() ourselves.  The special case (si_pid == 0 && sig == SIGABRT) is
            // due to the kernel sending a SIGABRT from a user request via SysRQ.
            let tid = libc::syscall(libc::SYS_gettid) as i32;
            if libc::syscall(libc::SYS_tgkill, std::process::id(), tid, sig) < 0 {
                // If we failed to kill ourselves (e.g. because a sandbox disallows us
                // to do so), we instead resort to terminating our process. This will
                // result in an incorrect exit code.
                libc::_exit(1);
            }
        } else {
            // This was a synchronous signal triggered by a hard fault (e.g. SIGSEGV).
            // No need to reissue the signal. It will automatically trigger again,
            // when we return from the signal handler.
        }
    }
}

/// The size of `CrashContext` can be too big w.r.t the size of alternatate stack
/// for `signal_handler`. Keep the crash context as a .bss field.
static CRASH_CONTEXT: parking_lot::Mutex<crash_context::CrashContext> =
    parking_lot::const_mutex(unsafe { mem::zeroed() });

pub(super) struct HandlerInner {
    handler: Box<dyn crate::CrashEvent>,
    pub(super) dump_process: Option<u32>,
}

impl HandlerInner {
    #[inline]
    pub(super) fn new(handler: Box<dyn crate::CrashEvent>) -> Self {
        Self {
            handler,
            dump_process: None,
        }
    }

    pub(super) unsafe fn handle_signal(
        &self,
        info: &mut libc::siginfo_t,
        uc: &mut libc::c_void,
    ) -> crate::CrashEventResult {
        unsafe {
            // The siginfo_t in libc is lowest common denominator, but this code is
            // specifically targeting linux/android, which contains the si_pid field
            // that we require
            let nix_info = &*((info as *const libc::siginfo_t).cast::<libc::signalfd_siginfo>());

            debug_print!("acquired siginfo");

            // Allow ourselves to be dumped, if that is what the user handler wishes to do
            let _set_dumpable = SetDumpable::new(self.dump_process);
            debug_print!("set dumpable");
            let mut cc = CRASH_CONTEXT.lock();

            {
                use std::ops::DerefMut;
                #[allow(clippy::explicit_deref_methods)]
                ptr::write_bytes(cc.deref_mut(), 0, 1);
                debug_print!("zeroed crashctx");

                ptr::copy_nonoverlapping(nix_info, &mut cc.siginfo, 1);
                debug_print!("copied siginfo");

                let uc_ptr = &*(uc as *const libc::c_void).cast::<crash_context::ucontext_t>();
                ptr::copy_nonoverlapping(uc_ptr, &mut cc.context, 1);
                debug_print!("copied context");

                cfg_if::cfg_if! {
                    if #[cfg(target_arch = "aarch64")] {
                        let fp_ptr = uc_ptr.uc_mcontext.__reserved.as_ptr().cast::<crash_context::fpsimd_context>();

                        if (*fp_ptr).head.magic == crash_context::FPSIMD_MAGIC {
                            ptr::copy_nonoverlapping(fp_ptr, &mut cc.float_state, 1);
                        }
                    } else if #[cfg(any(target_arch = "riscv64", target_arch = "s390x"))] {
                        cc.float_state = uc_ptr.uc_mcontext.__fpregs;
                    } else if #[cfg(not(target_arch = "arm"))] {
                        if !uc_ptr.uc_mcontext.fpregs.is_null() {
                            ptr::copy_nonoverlapping(uc_ptr.uc_mcontext.fpregs, ((&mut cc.float_state) as *mut crash_context::fpregset_t).cast(), 1);
                        }
                    }
                }

                cc.pid = std::process::id() as i32;
                cc.tid = libc::syscall(libc::SYS_gettid) as i32;
            }

            self.handler.on_crash(&cc)
        }
    }
}

/// We define these constans ourselves rather than use libc as they are missing
/// from eg. Android
const PR_GET_DUMPABLE: i32 = 3;
const PR_SET_DUMPABLE: i32 = 4;
const PR_SET_PTRACER: i32 = 0x59616d61;
const PR_SET_PTRACER_ANY: i32 = -1;

/// Helper that sets the process as dumpable if it is not, and when dropped
/// returns it back to the original state if needed
struct SetDumpable {
    was_dumpable: bool,
}

impl SetDumpable {
    unsafe fn new(dump_process: Option<u32>) -> Self {
        unsafe {
            let is_dumpable = libc::syscall(libc::SYS_prctl, PR_GET_DUMPABLE, 0, 0, 0, 0);
            let was_dumpable = is_dumpable > 0;

            if !was_dumpable {
                libc::syscall(libc::SYS_prctl, PR_SET_DUMPABLE, 1, 0, 0, 0);
            }

            // Set the process that is allowed to do ptrace operations on this process,
            // we either set it to the process that the user specified, or allow
            // any process, which _somewhat_ defeats the purpose of the yama security
            // that this call is needed for
            let ptracer = dump_process.map_or(PR_SET_PTRACER_ANY, |dp| dp as i32);

            // Note that this will fail with EINVAL if the pid does not exist, but
            // that would be on the user. We only need to do this if
            // `/proc/sys/kernel/yama/ptrace_scope` = 1, but should not have a negative
            // impact if it is in any other mode
            libc::syscall(libc::SYS_prctl, PR_SET_PTRACER, ptracer, 0, 0, 0);

            Self { was_dumpable }
        }
    }
}

impl Drop for SetDumpable {
    fn drop(&mut self) {
        unsafe {
            libc::syscall(libc::SYS_prctl, PR_SET_PTRACER, 0, 0, 0, 0);

            if !self.was_dumpable {
                libc::syscall(libc::SYS_prctl, PR_SET_DUMPABLE, 0, 0, 0, 0);
            }
        }
    }
}
