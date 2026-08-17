pub mod jmp;
mod signal;
mod state;

use crate::Error;

/// Possible exception codes values for the the `exception_code` field
/// in the crash context.
///
/// This is mainly for testing purposes, and is not exhaustive nor really accurate,
/// as eg. a distinction is made between a divide by zero between integers and
/// floats.
#[derive(Copy, Clone)]
#[repr(i32)]
#[allow(overflowing_literals)]
pub enum ExceptionCode {
    Abort = 0x40000015,             // STATUS_FATAL_APP_EXIT
    Fpe = -1073741676,              // EXCEPTION_INT_DIVIDE_BY_ZERO
    Illegal = -1073741795,          // EXCEPTION_ILLEGAL_INSTRUCTION
    Segv = -1073741819,             // EXCEPTION_ACCESS_VIOLATION
    StackOverflow = -1073741571,    // EXCEPTION_STACK_OVERFLOW
    Trap = -2147483645,             // EXCEPTION_BREAKPOINT
    InvalidParameter = -1073741811, // STATUS_INVALID_PARAMETER
    Purecall = -1073741787,         // STATUS_NONCONTINUABLE_EXCEPTION
    User = 0xcca11ed, // https://github.com/chromium/crashpad/blob/fca8871ca3fb721d3afab370ca790122f9333bfd/util/win/exception_codes.h#L32
    HeapCorruption = 0xc0000374, // STATUS_HEAP_CORRUPTION
}

/// A Windows exception handler
pub struct CrashHandler;

#[allow(clippy::unused_self)]
impl CrashHandler {
    /// Attaches the crash handler.
    ///
    /// The provided callback will be invoked if an exception is caught,
    /// providing a [`crate::CrashContext`] with the details of the thread where
    /// the exception was thrown.
    pub fn attach(on_crash: Box<dyn crate::CrashEvent>) -> Result<Self, Error> {
        state::attach(on_crash)?;
        Ok(Self)
    }

    /// Detaches this handler, removing it from the handler stack.
    ///
    /// This is done automatically when this [`CrashHandler`] is dropped.
    #[inline]
    pub fn detach(self) {
        state::detach();
    }

    /// Creates an exception with the specified exception code that is passed
    /// through the user provided callback.
    pub fn simulate_exception(&self, exception_code: Option<i32>) -> crate::CrashEventResult {
        // Normally this would be an unsafe function, since this unsafe encompasses
        // the entirety of the body, however the user is really not required to
        // uphold any guarantees on their end, so no real need to declare the
        // function itself unsafe.
        unsafe { state::simulate_exception(exception_code) }
    }
}

impl Drop for CrashHandler {
    fn drop(&mut self) {
        state::detach();
    }
}
