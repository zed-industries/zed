#![allow(missing_docs)]

use core::mem;

pub fn prevent_unwind<F, R>(label: &'static str, foreign_call: F) -> R
where
    F: FnOnce() -> R,
{
    // Goal is to make it impossible to propagate a panic across the C interface
    // of an extern "Rust" function, which would be Undefined Behavior. We
    // transform such panicks into a deterministic abort instead. When cxx is
    // built in an application using panic=abort, this guard object is compiled
    // out because its destructor is statically unreachable. When built with
    // panic=unwind, an unwind from the foreign call will attempt to drop the
    // guard object leading to a double panic, which is defined by Rust to
    // abort. In no_std programs, on most platforms the current mechanism for
    // this is for core::intrinsics::abort to invoke an invalid instruction. On
    // Unix, the process will probably terminate with a signal like SIGABRT,
    // SIGILL, SIGTRAP, SIGSEGV or SIGBUS. The precise behaviour is not
    // guaranteed and not stable, but is safe.
    let guard = Guard { label };

    let ret = foreign_call();

    // If we made it here, no uncaught panic occurred during the foreign call.
    mem::forget(guard);
    ret
}

struct Guard {
    label: &'static str,
}

impl Drop for Guard {
    #[cold]
    fn drop(&mut self) {
        panic!("panic in ffi function {}, aborting.", self.label);
    }
}
