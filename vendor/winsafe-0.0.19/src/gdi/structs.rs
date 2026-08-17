#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::privs::*;
use crate::guard::*;
use crate::prelude::*;

/// [`BITMAP`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmap)
/// struct.
#[repr(C)]
pub struct BITMAP {
	pub bmType: i32,
	pub bmWidth: i32,
	pub bmHeight: i32,
	pub bmWidthBytes: i32,
	pub bmPlanes: u16,
	pub bmBitsPixel: u16,
	pub bmBits: *mut u8,
}

impl_default!(BITMAP);

/// [`BITMAPFILEHEADER`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapfileheader)
/// struct.
#[repr(C, packed(2))]
pub struct BITMAPFILEHEADER {
	bfType: u16,
	pub bfSize: u32,
	bfReserved1: u16,
	bfReserved2: u16,
	pub bfOffBits: u32,
}

impl Default for BITMAPFILEHEADER {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.bfType = 0x4d42; // BM
		obj
	}
}

impl BITMAPFILEHEADER {
	/// Serializes the struct into `&[u8]`.
	pub const fn serialize(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				self as *const _ as _,
				std::mem::size_of::<Self>(),
			)
		}
	}
}

/// [`BITMAPINFO`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo)
/// struct.
#[repr(C)]
pub struct BITMAPINFO {
	pub bmiHeader: BITMAPINFOHEADER,
	pub bmiColors: [RGBQUAD; 1],
}

impl VariableSized for BITMAPINFO {}

impl Default for BITMAPINFO {
	fn default() -> Self {
		Self {
			bmiHeader: BITMAPINFOHEADER::default(),
			bmiColors: [RGBQUAD::default()],
		}
	}
}

/// [`BITMAPINFOHEADER`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
/// struct.
#[repr(C)]
pub struct BITMAPINFOHEADER {
	biSize: u32,
	pub biWidth: i32,
	pub biHeight: i32,
	pub biPlanes: u16,
	pub biBitCount: u16,
	pub biCompression: co::BI,
	pub biSizeImage: u32,
	pub biXPelsPerMeter: i32,
	pub biYPelsPerMeter: i32,
	pub biClrUsed: u32,
	pub biClrImportant: u32,
}

impl_default_with_size!(BITMAPINFOHEADER, biSize);

impl BITMAPINFOHEADER {
	/// Serializes the struct into `&[u8]`.
	pub const fn serialize(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				self as *const _ as _,
				std::mem::size_of::<Self>(),
			)
		}
	}
}

/// [`LOGBRUSH`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logbrush)
/// struct.
#[repr(C)]
pub struct LOGBRUSH {
	pub lbStyle: co::BSS,
	pub lbColor: COLORREF,
	pub lbHatch: usize, // weird field
}

impl_default!(LOGBRUSH);

/// [`LOGFONT`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw)
/// struct.
#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct LOGFONT {
	pub lfHeight: i32,
	pub lfWidth: i32,
	pub lfEscapement: i32,
	pub lfOrientation: i32,
	pub lfWeight: co::FW,
	pub lfItalic: u8,
	pub lfUnderline: u8,
	pub lfStrikeOut: u8,
	pub lfCharSet: co::CHARSET,
	pub lfOutPrecision: co::OUT_PRECIS,
	pub lfClipPrecision: co::CLIP,
	pub lfQuality: co::QUALITY,
	pub lfPitchAndFamily: co::PITCH,
	lfFaceName: [u16; LF_FACESIZE],
}

impl LOGFONT {
	pub_fn_string_arr_get_set!(lfFaceName, set_lfFaceName);
}

/// [`LOGPALETTE`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logpalette)
/// struct.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut log_pal = w::LOGPALETTE::new(0x300, &[
///     w::PALETTEENTRY { peRed: 1, peGreen: 2, peBlue: 3, ..Default::default() },
///     w::PALETTEENTRY { peRed: 10, peGreen: 20, peBlue: 30, ..Default::default() },
/// ])?;
///
/// // Setting a new entry value
/// log_pal.palPalEntry_mut()[0].peRed = 255;
///
/// // Printing all entry values
/// for entry in log_pal.palPalEntry().iter() {
///     println!("{} {} {}", entry.peRed, entry.peGreen, entry.peBlue);
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[repr(C)]
pub struct LOGPALETTE {
	pub palVersion: u16,
	pub(in crate::gdi) palNumEntries: u16,
	palPalEntry: [PALETTEENTRY; 1],
}

impl VariableSized for LOGPALETTE {}

impl LOGPALETTE {
	/// Returns a dynamically allocated
	/// [`LogpaletteGuard`](crate::guard::LogpaletteGuard).
	#[must_use]
	pub fn new(
		palVersion: u16,
		entries: &[PALETTEENTRY],
	) -> SysResult<LogpaletteGuard>
	{
		LogpaletteGuard::new(palVersion, entries)
	}

	/// Returns a constant slice over the `palPalEntry` entries.
	#[must_use]
	pub const fn palPalEntry(&self) -> &[PALETTEENTRY] {
		unsafe {
			std::slice::from_raw_parts(
				self.palPalEntry.as_ptr(),
				self.palNumEntries as _,
			)
		}
	}

	/// Returns a mutable slice over the `palPalEntry` entries.
	#[must_use]
	pub fn palPalEntry_mut(&mut self) -> &mut [PALETTEENTRY] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self.palPalEntry.as_mut_ptr(),
				self.palNumEntries as _,
			)
		}
	}
}

/// [`LOGPEN`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logpen)
/// struct.
#[repr(C)]
pub struct LOGPEN {
	pub lopnStyle: co::PS,
	pub lopnWidth: POINT,
	pub lopnColor: COLORREF,
}

impl_default!(LOGPEN);

/// [`NONCLIENTMETRICS`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw)
/// struct.
#[repr(C)]
pub struct NONCLIENTMETRICS {
	cbSize: u32,
	pub iBorderWidth: i32,
	pub iScrollWidth: i32,
	pub iScrollHeight: i32,
	pub iCaptionWidth: i32,
	pub iCaptionHeight: i32,
	pub lfCaptionFont: LOGFONT,
	pub iSmCaptionWidth: i32,
	pub iSmCaptionHeight: i32,
	pub lfSmCaptionFont: LOGFONT,
	pub iMenuWidth: i32,
	pub iMenuHeight: i32,
	pub lfMenuFont: LOGFONT,
	pub lfStatusFont: LOGFONT,
	pub lfMessageFont: LOGFONT,
	pub iPaddedBorderWidth: i32,
}

impl Default for NONCLIENTMETRICS {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as _;

		let is_vista = IsWindowsVistaOrGreater()
			.unwrap_or_else(|err| panic!("{}", err)); // should never happen

		if !is_vista {
			obj.cbSize -= std::mem::size_of::<i32>() as u32
		}
		obj
	}
}

/// [`PALETTEENTRY`](https://learn.microsoft.com/en-us/previous-versions/dd162769(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PALETTEENTRY {
	pub peRed: u8,
	pub peGreen: u8,
	pub peBlue: u8,
	pub peFlags: co::PC,
}

/// [`RGBQUAD`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgbquad)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RGBQUAD {
	pub rgbBlue: u8,
	pub rgbGreen: u8,
	pub rgbRed: u8,
	rgbReserved: u8,
}

/// [`TEXTMETRIC`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-textmetricw)
/// struct.
#[repr(C)]
#[derive(Default, Clone)]
pub struct TEXTMETRIC {
	pub tmHeight: i32,
	pub tmAscent: i32,
	pub tmDescent: i32,
	pub tmInternalLeading: i32,
	pub tmExternalLeading: i32,
	pub tmAveCharWidth: i32,
	pub tmMaxCharWidth: i32,
	pub tmWeight: i32,
	pub tmOverhang: i32,
	pub tmDigitizedAspectX: i32,
	pub tmDigitizedAspectY: i32,
	pub tmFirstChar: u16,
	pub tmLastChar: u16,
	pub tmDefaultChar: u16,
	pub tmBreakChar: u16,
	pub tmItalic: u8,
	pub tmUnderlined: u8,
	pub tmStruckOut: u8,
	pub tmPitchAndFamily: u8,
	pub tmCharSet: u8,
}
