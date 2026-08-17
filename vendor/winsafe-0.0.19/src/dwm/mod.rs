#![cfg(feature = "dwm")]

mod funcs;
mod handles;

pub(in crate::dwm) mod ffi;
pub mod co;

pub mod decl {
	pub use super::funcs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
