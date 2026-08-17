#![cfg(all(feature = "comctl", feature = "shell"))]

mod handles;

pub mod traits {
	pub use super::handles::traits::*;
}
