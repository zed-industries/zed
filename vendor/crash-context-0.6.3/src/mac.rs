pub mod guard;
pub mod ipc;
pub mod resource;

use mach2::mach_types as mt;

/// Information on the exception that caused the crash
#[derive(Copy, Clone, Debug)]
pub struct ExceptionInfo {
    /// The exception kind
    pub kind: u32,
    /// The exception code
    pub code: u64,
    /// Optional subcode with different meanings depending on the exception type
    /// * `EXC_BAD_ACCESS` - The address that caused the exception
    /// * `EXC_GUARD` - The unique guard identifier that was guarding a resource
    /// * `EXC_RESOURCE` - Additional details depending on the resource type
    pub subcode: Option<u64>,
}

/// Full Macos crash context
#[derive(Debug)]
pub struct CrashContext {
    /// The process which crashed
    pub task: mt::task_t,
    /// The thread in the process that crashed
    pub thread: mt::thread_t,
    /// The thread that handled the exception. This may be useful to ignore.
    pub handler_thread: mt::thread_t,
    /// Optional exception information
    pub exception: Option<ExceptionInfo>,
}
