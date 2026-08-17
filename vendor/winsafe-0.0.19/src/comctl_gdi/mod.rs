#![cfg(all(feature = "comctl", feature = "gdi"))]

mod structs;

pub mod messages;

pub mod decl {
	pub use super::structs::*;
}
