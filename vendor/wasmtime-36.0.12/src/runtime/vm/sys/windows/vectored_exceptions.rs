use crate::prelude::*;
use crate::runtime::vm::traphandlers::{TrapRegisters, TrapTest, tls};
use std::ffi::c_void;
use std::io;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::System::Diagnostics::Debug::*;
use windows_sys::Win32::System::Kernel::*;

/// Function which may handle custom signals while processing traps.
pub type SignalHandler = Box<dyn Fn(*mut EXCEPTION_POINTERS) -> bool + Send + Sync>;

pub struct TrapHandler {
    handle: *mut c_void,
}

unsafe impl Send for TrapHandler {}
unsafe impl Sync for TrapHandler {}

impl TrapHandler {
    pub unsafe fn new(_macos_use_mach_ports: bool) -> TrapHandler {
        // our trap handler needs to go first, so that we can recover from
        // wasm faults and continue execution, so pass `1` as a true value
        // here.
        let handle = unsafe { AddVectoredExceptionHandler(1, Some(exception_handler)) };
        if handle.is_null() {
            panic!(
                "failed to add exception handler: {}",
                io::Error::last_os_error()
            );
        }
        TrapHandler { handle }
    }

    pub fn validate_config(&self, _macos_use_mach_ports: bool) {}
}

impl Drop for TrapHandler {
    fn drop(&mut self) {
        unsafe {
            let rc = RemoveVectoredExceptionHandler(self.handle);
            if rc == 0 {
                eprintln!(
                    "failed to remove exception handler: {}",
                    io::Error::last_os_error()
                );
                libc::abort();
            }
        }
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "too fiddly to handle and wouldn't help much anyway"
)]
unsafe extern "system" fn exception_handler(exception_info: *mut EXCEPTION_POINTERS) -> i32 {
    // Check the kind of exception, since we only handle a subset within
    // wasm code. If anything else happens we want to defer to whatever
    // the rest of the system wants to do for this exception.
    let record = unsafe { &*(*exception_info).ExceptionRecord };
    if record.ExceptionCode != EXCEPTION_ACCESS_VIOLATION
        && record.ExceptionCode != EXCEPTION_ILLEGAL_INSTRUCTION
        && record.ExceptionCode != EXCEPTION_INT_DIVIDE_BY_ZERO
        && record.ExceptionCode != EXCEPTION_INT_OVERFLOW
    {
        return ExceptionContinueSearch;
    }

    // FIXME: this is what the previous C++ did to make sure that TLS
    // works by the time we execute this trap handling code. This isn't
    // exactly super easy to call from Rust though and it's not clear we
    // necessarily need to do so. Leaving this here in case we need this
    // in the future, but for now we can probably wait until we see a
    // strange fault before figuring out how to reimplement this in
    // Rust.
    //
    // if (!NtCurrentTeb()->Reserved1[sThreadLocalArrayPointerIndex]) {
    //     return ExceptionContinueSearch;
    // }

    // This is basically the same as the unix version above, only with a
    // few parameters tweaked here and there.
    tls::with(|info| {
        let info = match info {
            Some(info) => info,
            None => return ExceptionContinueSearch,
        };
        let context = unsafe { &*(*exception_info).ContextRecord };
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                let regs = TrapRegisters {
                    pc: context.Rip as usize,
                    fp: context.Rbp as usize,
                };
            } else if #[cfg(target_arch = "aarch64")] {
                let regs = TrapRegisters {
                    pc: context.Pc as usize,
                    fp: unsafe { context.Anonymous.Anonymous.Fp as usize },
                };
            } else if #[cfg(target_arch = "x86")] {
                let regs = TrapRegisters {
                    pc: context.Eip as usize,
                    fp: context.Ebp as usize,
                };
            } else {
                compile_error!("unsupported platform");
            }
        }
        // For access violations the first element in `ExceptionInformation` is
        // an indicator as to whether the fault was a read/write. The second
        // element is the address of the inaccessible data causing this
        // violation.
        let faulting_addr = if record.ExceptionCode == EXCEPTION_ACCESS_VIOLATION {
            assert!(record.NumberParameters >= 2);
            Some(record.ExceptionInformation[1])
        } else {
            None
        };
        match info.test_if_trap(regs, faulting_addr, |handler| handler(exception_info)) {
            TrapTest::NotWasm => ExceptionContinueSearch,
            TrapTest::HandledByEmbedder => ExceptionContinueExecution,
            TrapTest::Trap { jmp_buf } => unsafe { super::traphandlers::wasmtime_longjmp(jmp_buf) },
        }
    })
}
