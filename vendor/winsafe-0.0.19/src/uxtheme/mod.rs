#![cfg(feature = "uxtheme")]

mod funcs;
mod handles;
mod structs;

pub(in crate::uxtheme) mod ffi;
pub mod co;
pub mod guard;

pub mod decl {
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
}
