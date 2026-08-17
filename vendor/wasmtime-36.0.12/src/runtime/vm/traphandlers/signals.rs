//! Trap handling support when `has_native_signals` is enabled.
//!
//! This module is conditionally included in the above `traphandlers` module and
//! contains support and shared routines for working with signals-based traps.
//! Each platform will have its own `signals-based-traps` configuration and
//! thise module serves as a shared entrypoint for initialization entrypoints
//! (`init_traps`) and testing if a trapping opcode is wasm (`test_if_trap`).

use crate::sync::RwLock;
use crate::vm::sys::traphandlers::TrapHandler;

/// Platform-specific trap-handler state.
///
/// This state is protected by a lock to synchronize access to it. Right now
/// it's a `RwLock` but it could be a `Mutex`, and `RwLock` is just chosen for
/// convenience as it's what's implemented in no_std. The performance here
/// should not be of consequence.
///
/// This is initialized to `None` and then set as part of `init_traps`.
static TRAP_HANDLER: RwLock<Option<TrapHandler>> = RwLock::new(None);

/// This function is required to be called before any WebAssembly is entered.
/// This will configure global state such as signal handlers to prepare the
/// process to receive wasm traps.
///
/// # Panics
///
/// This function will panic on macOS if it is called twice or more times with
/// different values of `macos_use_mach_ports`.
///
/// This function will also panic if the `std` feature is disabled and it's
/// called concurrently.
pub fn init_traps(macos_use_mach_ports: bool) {
    let mut lock = TRAP_HANDLER.write();
    match lock.as_mut() {
        Some(state) => state.validate_config(macos_use_mach_ports),
        None => *lock = Some(unsafe { TrapHandler::new(macos_use_mach_ports) }),
    }
}

/// De-initializes platform-specific state for trap handling.
///
/// # Panics
///
/// This function will also panic if the `std` feature is disabled and it's
/// called concurrently.
///
/// # Aborts
///
/// This may abort the process on some platforms where trap handling state
/// cannot be unloaded.
///
/// # Unsafety
///
/// This is not safe to be called unless all wasm code is unloaded. This is not
/// safe to be called on some platforms, like Unix, when other libraries
/// installed their own signal handlers after `init_traps` was called.
///
/// There's more reasons for unsafety here than those articulated above,
/// generally this can only be called "if you know what you're doing".
pub unsafe fn deinit_traps() {
    let mut lock = TRAP_HANDLER.write();
    let _ = lock.take();
}
