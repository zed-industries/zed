#![cfg(feature = "kernel")]

mod aliases;
mod base_traits;
mod enums;
mod ffi;
mod funcs;
mod handles;
mod proc;
mod structs;
mod utilities;

pub(in crate::kernel) mod iterators;
pub(crate) mod ffi_types;
pub(crate) mod privs;
pub mod co;
pub mod guard;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}

pub mod traits {
	pub use super::base_traits::*;
	pub use super::handles::traits::*;
}
