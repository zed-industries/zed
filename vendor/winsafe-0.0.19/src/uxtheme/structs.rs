#![allow(non_snake_case)]

/// [`MARGINS`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/ns-uxtheme-margins)
/// struct.
#[repr(C)]
#[derive(Default, PartialEq, Eq)]
pub struct MARGINS {
	pub cxLeftWidth: i32,
	pub cxRightWidth: i32,
	pub cyTopHeight: i32,
	pub cyBottomHeight: i32,
}
