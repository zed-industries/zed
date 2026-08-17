use crate::prelude::*;
#[cfg(has_host_compiler_backend)]
use crate::runtime::vm::VMContext;
#[cfg(has_host_compiler_backend)]
use core::{mem, ptr::NonNull};

#[cfg(has_host_compiler_backend)]
pub use crate::runtime::vm::sys::capi::{self, wasmtime_longjmp};

pub type SignalHandler = Box<dyn Fn() + Send + Sync>;

#[cfg(has_host_compiler_backend)]
pub unsafe fn wasmtime_setjmp(
    jmp_buf: *mut *const u8,
    callback: extern "C" fn(*mut u8, NonNull<VMContext>) -> bool,
    payload: *mut u8,
    callee: NonNull<VMContext>,
) -> bool {
    unsafe {
        let callback = mem::transmute::<
            extern "C" fn(*mut u8, NonNull<VMContext>) -> bool,
            extern "C" fn(*mut u8, *mut u8) -> bool,
        >(callback);
        capi::wasmtime_setjmp(jmp_buf, callback, payload, callee.as_ptr().cast())
    }
}

#[cfg(has_native_signals)]
pub struct TrapHandler;

#[cfg(has_native_signals)]
impl TrapHandler {
    pub unsafe fn new(_macos_use_mach_ports: bool) -> TrapHandler {
        unsafe {
            capi::wasmtime_init_traps(handle_trap);
        }
        TrapHandler
    }

    pub fn validate_config(&self, _macos_use_mach_ports: bool) {}
}

#[cfg(has_native_signals)]
extern "C" fn handle_trap(pc: usize, fp: usize, has_faulting_addr: bool, faulting_addr: usize) {
    use crate::runtime::vm::traphandlers::{TrapRegisters, TrapTest, tls};

    tls::with(|info| {
        let info = match info {
            Some(info) => info,
            None => return,
        };
        let faulting_addr = if has_faulting_addr {
            Some(faulting_addr)
        } else {
            None
        };
        let regs = TrapRegisters { pc, fp };
        let test = info.test_if_trap(regs, faulting_addr, |_handler| {
            panic!("custom signal handlers are not supported on this platform");
        });
        match test {
            TrapTest::NotWasm => {}
            TrapTest::HandledByEmbedder => unreachable!(),
            TrapTest::Trap { jmp_buf } => unsafe { wasmtime_longjmp(jmp_buf) },
        }
    })
}

pub fn lazy_per_thread_init() {}
