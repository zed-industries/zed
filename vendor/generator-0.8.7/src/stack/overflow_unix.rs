use crate::rt::{guard, ContextStack};

use crate::yield_::yield_now;
use libc::{sigaction, sighandler_t, SA_ONSTACK, SA_SIGINFO, SIGBUS, SIGSEGV};
use std::mem;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::sync::{Mutex, Once};

static SIG_ACTION: Mutex<MaybeUninit<sigaction>> = Mutex::new(MaybeUninit::uninit());

// Signal handler for the SIGSEGV and SIGBUS handlers. We've got guard pages
// (unmapped pages) at the end of every thread's stack, so if a thread ends
// up running into the guard page it'll trigger this handler. We want to
// detect these cases and print out a helpful error saying that the stack
// has overflowed. All other signals, however, should go back to what they
// were originally supposed to do.
//
// If this is not a stack overflow, the handler un-registers itself and
// then returns (to allow the original signal to be delivered again).
// Returning from this kind of signal handler is technically not defined
// to work when reading the POSIX spec strictly, but in practice it turns
// out many large systems and all implementations allow returning from a
// signal handler to work. For a more detailed explanation see the
// comments on https://github.com/rust-lang/rust/issues/26458.
//
// A pointer to the exception context is passed as the third argument. This
// context is usually compatible with libc::ucontext_t. However some architectures
// (like powerpc64) do not provide the ucontext_t type in glibc. Since we do not
// use the context information, it is represented as a generic pointer.
// see: https://github.com/rust-lang/libc/pull/3986 / https://github.com/rust-lang/libc/pull/3986
unsafe extern "C" fn signal_handler(
    signum: libc::c_int,
    info: *mut libc::siginfo_t,
    ctx: *mut libc::c_void, // workaroung for ppc64le missing ucontext_t in rust libc. See: https://github.com/rust-lang/libc/issues/3964
) {
    let _ctx = &mut *ctx;
    let addr = (*info).si_addr() as usize;
    let stack_guard = guard::current();

    if !stack_guard.contains(&addr) {
        println!("{}", std::backtrace::Backtrace::force_capture());
        // SIG_ACTION is available after we registered our handler
        let old_action = SIG_ACTION.lock().unwrap();
        sigaction(signum, old_action.assume_init_ref(), null_mut());

        // we are unable to handle this
        return;
    }

    eprintln!(
        "\ncoroutine in thread '{}' has overflowed its stack\n",
        std::thread::current().name().unwrap_or("<unknown>")
    );

    ContextStack::current().top().err = Some(Box::new(crate::Error::StackErr));

    let mut sigset: libc::sigset_t = mem::zeroed();
    libc::sigemptyset(&mut sigset);
    libc::sigaddset(&mut sigset, signum);
    libc::sigprocmask(libc::SIG_UNBLOCK, &sigset, null_mut());

    yield_now();

    std::process::abort();
}

#[cold]
unsafe fn init() {
    let mut action: sigaction = mem::zeroed();

    action.sa_flags = SA_SIGINFO | SA_ONSTACK;
    action.sa_sigaction = signal_handler as sighandler_t;

    let mut old_action = SIG_ACTION.lock().unwrap();

    for signal in [SIGSEGV, SIGBUS] {
        sigaction(signal, &action, old_action.assume_init_mut());
    }
}

pub fn init_once() {
    static INIT_ONCE: Once = Once::new();

    INIT_ONCE.call_once(|| unsafe {
        init();
    })
}
