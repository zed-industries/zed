#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::decl::*;

/// [`IMAGELISTDRAWPARAMS`](https://learn.microsoft.com/en-us/windows/win32/api/commoncontrols/ns-commoncontrols-imagelistdrawparams)
/// struct.
#[repr(C)]
pub struct IMAGELISTDRAWPARAMS {
	cbSize: u32,
	pub himl: HIMAGELIST,
	pub i: i32,
	pub hdcDst: HDC,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub xBitmap: i32,
	pub yBitmap: i32,
	pub rgbBk: COLORREF,
	pub rgbFg: COLORREF,
	pub fStyle: co::ILD,
	pub dwRop: co::ROP,
	pub fState: co::ILS,
	pub Frame: u32,
	pub crEffect: COLORREF,
}

impl_default_with_size!(IMAGELISTDRAWPARAMS, cbSize);

/// [`NMTVASYNCDRAW`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
/// struct.
#[repr(C)]
pub struct NMTVASYNCDRAW<'a> {
	pub hdr: NMHDR,
	pimldp: *mut IMAGELISTDRAWPARAMS,
	pub hr: co::ERROR,
	pub hItem: HTREEITEM,
	pub lParam: isize,
	pub dwRetFlags: co::ADRF,
	pub iRetImageIndex: i32,

	_pimldp: PhantomData<&'a mut IMAGELISTDRAWPARAMS>,
}

impl_default!(NMTVASYNCDRAW, 'a);

impl<'a> NMTVASYNCDRAW<'a> {
	pub_fn_ptr_get_set!('a, pimldp, set_pimldp, IMAGELISTDRAWPARAMS);
}
