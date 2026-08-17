#![cfg(feature = "user")]

mod aliases;
mod enums;
mod funcs;
mod handles;
mod msg_traits;
mod proc;
mod structs;

pub(in crate::user) mod ffi;
pub(in crate::user) mod iterators;
pub(crate) mod privs;
pub mod co;
pub mod guard;
pub mod messages;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::handles::traits::*;
	pub use super::msg_traits::*;
}
