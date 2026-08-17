mod ffi;
mod signal;
mod state;

/// High level exception types
///
/// `exception_types.h`
#[repr(i32)]
pub enum ExceptionType {
    /// Could not access memory. (SIGSEGV/SIGBUS)
    ///
    /// Code field contains `kern_return_t` describing error.
    /// Subcode field contains bad memory address.
    BadAccess = 1,
    /// Instruction failed. (SIGILL)
    ///
    /// Illegal or undfined instruction or operand.
    BadInstruction = 2,
    /// Arithmetic exception (SIGFPE)
    ///
    /// Exact nature of the exception is in code field.
    Arithmetic = 3,
    /// Emulation instruction
    ///
    /// Emulation support instruction encountered
    /// Details in code and subcode fields.
    Emulation = 4,
    /// Software generated exception
    ///
    /// Exaction exception is in the code field.
    /// Codes 0 - 0xffff reserved to hardware.
    /// Codes 0x10000 - 0x1ffff reserved for OS emulation (Unix)
    Software = 5,
    /// Trace, breakpoint, etc
    ///
    /// Details in the code field
    Breakpoint = 6,
    /// System calls
    SysCall = 7,
    /// Mach system calls
    MachSysCall = 8,
    /// RPC alert
    RpcAlert = 9,
    /// Abnormal process exit
    Crash = 10,
    /// Hit resource consumption limit
    ///
    /// Exact resource is in the code field.
    Resource = 11,
    /// Violated guarded resource protections
    Guard = 12,
    /// Abnormal process exited to corpse state
    CorpseNotify = 13,
}

/// A Macos exception handler
pub struct CrashHandler;

#[allow(clippy::unused_self)]
impl CrashHandler {
    /// Attaches the exception handler.
    ///
    /// The provided callback will be invoked if an exception is caught,
    /// providing a [`crate::CrashContext`] with the details of the thread where
    /// the exception was thrown.
    pub fn attach(on_crash: Box<dyn crate::CrashEvent>) -> Result<Self, crate::Error> {
        state::attach(on_crash)?;
        Ok(Self)
    }

    /// Detaches the handler.
    ///
    /// This is done automatically when [`CrashHandler`] is dropped.
    #[inline]
    pub fn detach(self) {
        state::detach(false);
    }

    // Raises the specified user exception
    #[inline]
    pub fn simulate_exception(&self, exception_info: Option<crash_context::ExceptionInfo>) -> bool {
        state::simulate_exception(exception_info)
    }
}

impl Drop for CrashHandler {
    fn drop(&mut self) {
        state::detach(false);
    }
}
