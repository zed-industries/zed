mod gdi_traits;
mod hbitmap;
mod hbrush;
mod hdc;
mod hfont;
mod hinstance;
mod hpalette;
mod hpen;
mod hrgn;

pub mod decl {
	pub use super::hfont::HFONT;
	pub use super::hpalette::HPALETTE;
	pub use super::hpen::HPEN;
}

pub mod traits {
	pub use super::gdi_traits::*;
	pub use super::hbitmap::gdi_Hbitmap;
	pub use super::hbrush::gdi_Hbrush;
	pub use super::hdc::gdi_Hdc;
	pub use super::hfont::gdi_Hfont;
	pub use super::hinstance::gdi_Hinstance;
	pub use super::hpalette::gdi_Hpalette;
	pub use super::hpen::gdi_Hpen;
	pub use super::hrgn::gdi_Hrgn;
}
