//! Hooks for responding to hard-abort and reinit events on the Wasm instance.
//!
//! # Hard abort
//!
//! A hard abort occurs when the WebAssembly instance encounters a
//! non-recoverable error — an `unreachable` instruction, out-of-memory, or
//! stack overflow — that cannot be caught by Rust's `catch_unwind`.  The
//! instance is poisoned and no further exports can be called.
//!
//! Use [`set_on_abort`] to register a callback that runs at the moment of
//! termination.  Returns the previously registered handler (`None` if unset),
//! mirroring the `std::panic::set_hook` convention.
//!
//! **Only available when built with `panic=unwind`.**
//! [`set_on_abort`] returns `None` and the callback will never fire on
//! `panic=abort` builds. Support for `panic=abort` may be added in a future
//! release.
//!
//! # Reinit
//!
//! [`schedule_reinit()`] signals that the instance should be reinitialized.
//! The next call to any export detects this, creates a fresh
//! `WebAssembly.Instance` from the same module.
//!
//! Works with both `panic=unwind` and `panic=abort` builds.
//!
//! The reinit machinery is automatically emitted when [`schedule_reinit()`] is
//! used — no CLI flag is required. `--experimental-reset-state-function` is
//! only needed for the public `__wbg_reset_state()` export.
#[doc(hidden)]
pub use crate::__rt::schedule_reinit;
#[doc(hidden)]
pub use crate::__rt::set_on_abort;
