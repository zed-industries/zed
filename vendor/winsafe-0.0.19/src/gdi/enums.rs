use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageBitmap`](crate::prelude::gdi_Hinstance::LoadImageBitmap).
#[derive(Clone)]
pub enum IdObmStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OBM`](crate::co::OBM) constant for an OEM bitmap.
	Obm(co::OBM),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdObmStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Obm(obm) => MAKEINTRESOURCE(obm.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageCursor`](crate::prelude::gdi_Hinstance::LoadImageCursor).
#[derive(Clone)]
pub enum IdOcrStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OCR`](crate::co::OCR) constant for an OEM cursor.
	Ocr(co::OCR),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOcrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Ocr(ocr) => MAKEINTRESOURCE(ocr.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageIcon`](crate::prelude::gdi_Hinstance::LoadImageIcon).
#[derive(Clone)]
pub enum IdOicStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OIC`](crate::co::OIC) constant for an OEM icon.
	Oic(co::OIC),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOicStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Oic(oic) => MAKEINTRESOURCE(oic.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}
