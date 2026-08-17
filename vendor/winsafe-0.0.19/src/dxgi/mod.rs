#![cfg(feature = "dxgi")]

mod com_interfaces;
mod funcs;
mod structs;

pub(in crate::dxgi) mod ffi;
pub(in crate::dxgi) mod iterators;
pub mod co;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}
