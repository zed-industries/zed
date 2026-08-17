//! Shareable mutable containers.

#[allow(clippy::module_inception)]
mod cell;
mod unsafe_cell;

pub use self::cell::Cell;
pub use self::unsafe_cell::{ConstPtr, MutPtr, UnsafeCell};
