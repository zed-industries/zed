mod bstr;
mod others;
mod propvariant;
mod variant;
mod variant_traits;

pub mod decl {
	pub use super::bstr::BSTR;
	pub use super::others::*;
	pub use super::propvariant::PROPVARIANT;
	pub use super::variant::VARIANT;
}

pub mod traits {
	pub use super::variant_traits::*;
}
