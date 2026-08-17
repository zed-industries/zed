use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

/// Variant parameter for:
///
/// * [`stm::GetImage`](crate::msg::stm::GetImage);
/// * [`stm::SetImage`](crate::msg::stm::SetImage).
pub enum BmpIconCurMeta {
	/// Bitmap.
	Bmp(HBITMAP),
	/// Icon.
	Icon(HICON),
	/// Cursor.
	Cur(HCURSOR),
	/// Enhanced metafile.
	Meta(HDC),
}

impl BmpIconCurMeta {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		unsafe {
			std::mem::transmute(match self {
				BmpIconCurMeta::Bmp(hbmp) => hbmp.ptr(),
				BmpIconCurMeta::Icon(hicon) => hicon.ptr(),
				BmpIconCurMeta::Cur(hcur) => hcur.ptr(),
				BmpIconCurMeta::Meta(hdc) => hdc.ptr(),
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`TBADDBITMAP`](crate::TBADDBITMAP).
pub enum BmpIdbRes {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A system-defined [`co::IDB`](crate::co::IDB) bitmap.
	Idb(co::IDB),
	/// A bitmap resource.
	Res(IdStr, HINSTANCE),
}

/// Variant parameter for:
///
/// * [`TBREPLACEBITMAP`](crate::TBREPLACEBITMAP).
pub enum BmpInstId {
	/// Bitmap handle.
	Bmp(HBITMAP),
	/// Module handle and resource ID.
	InstId(HINSTANCE, u16),
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG).
pub enum IconId {
	/// No icon.
	None,
	/// An icon handle.
	Icon(HICON),
	/// A resource ID.
	Id(u16),
}

/// Variant parameter for:
///
/// * [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG).
pub enum IconIdTdicon {
	/// No icon.
	None,
	/// An icon handle.
	Icon(HICON),
	/// A resource ID.
	Id(u16),
	/// A predefined icon.
	Tdicon(co::TD_ICON),
}

/// Variant parameter for:
///
/// * [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog).
#[derive(Clone)]
pub enum IconRes {
	/// No icon.
	None,
	/// An icon resource.
	Res(IdStr),
	/// The [`co::TD_ICON::WARNING`](crate::co::TD_ICON::WARNING) constant.
	Warn,
	/// The [`co::TD_ICON::ERROR`](crate::co::TD_ICON::ERROR) constant.
	Error,
	/// The [`co::TD_ICON::INFORMATION`](crate::co::TD_ICON::INFORMATION)
	/// constant.
	Info,
	/// The [`co::TD_ICON::SHIELD`](crate::co::TD_ICON::SHIELD) constant.
	Shield,
}

impl IconRes {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::None => std::ptr::null(),
			Self::Res(id_str) => id_str.as_ptr(),
			Self::Warn => MAKEINTRESOURCE(co::TD_ICON::WARNING.raw() as _),
			Self::Error => MAKEINTRESOURCE(co::TD_ICON::ERROR.raw() as _),
			Self::Info => MAKEINTRESOURCE(co::TD_ICON::INFORMATION.raw() as _),
			Self::Shield => MAKEINTRESOURCE(co::TD_ICON::SHIELD.raw() as _),
		}
	}
}

/// Variant type for:
///
/// * [`tbm::ChangeBitmap`](crate::msg::tbm::ChangeBitmap).
#[derive(Clone, Copy)]
pub enum IdxCbNone {
	/// Index of an image in the toolbar's image list.
	Idx(u32),
	/// Toolbar will send `TBN_GETDISPINFO` notifications.
	Cb,
	/// Button doesn't have an image.
	None,
}

impl From<IdxCbNone> for isize {
	fn from(v: IdxCbNone) -> Self {
		match v {
			IdxCbNone::Idx(idx) => idx as _,
			IdxCbNone::Cb => I_IMAGECALLBACK,
			IdxCbNone::None => I_IMAGENONE,
		}
	}
}

/// Variant parameter for:
///
/// * [`TBBUTTON`](crate::TBBUTTON).
#[derive(Clone)]
pub enum IdxStr {
	/// Index of button string.
	Idx(u16),
	/// A string buffer.
	Str(WString),
}

/// Variant parameter for:
///
/// * [`hdm::SetHotDivider`](crate::msg::hdm::SetHotDivider).
pub enum PtIdx {
	/// X and Y coordinates of the pointer
	Pt(POINT),
	/// Index of the divider.
	Idx(u32),
}

/// Variant parameter for:
///
/// * [`tbm::AddString`](crate::msg::tbm::AddString).
pub enum ResStrs {
	/// A resource string resource.
	Res(IdStr, HINSTANCE),
	/// A multi-string composed of null-separated strings. To use this field,
	/// prefer the [`ResStrs::from_strs`](crate::ResStrs::from_strs) static
	/// method.
	Strs(WString),
}

impl ResStrs {
	/// Constructs the enum from a list of strings.
	#[must_use]
	pub fn from_strs(texts: &[impl AsRef<str>]) -> ResStrs {
		Self::Strs(WString::from_str_vec(texts))
	}
}

/// Variant parameter for:
///
/// * [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT).
pub enum TreeitemTvi {
	/// Handle to a tree view item.
	Treeitem(HTREEITEM),
	/// One of the predefined values.
	Tvi(co::TVI),
}

impl From<TreeitemTvi> for isize {
	fn from(v: TreeitemTvi) -> Self {
		match v {
			TreeitemTvi::Treeitem(htreeitem) => htreeitem.ptr() as _,
			TreeitemTvi::Tvi(tvi) => tvi.raw(),
		}
	}
}

impl TreeitemTvi {
	/// Constructs the enum from an `isize`.
	#[must_use]
	pub fn from_isize(val: isize) -> TreeitemTvi {
		match unsafe { co::TVI::from_raw(val) } {
			co::TVI::FIRST => Self::Tvi(co::TVI::FIRST),
			co::TVI::LAST => Self::Tvi(co::TVI::LAST),
			co::TVI::ROOT => Self::Tvi(co::TVI::ROOT),
			co::TVI::SORT => Self::Tvi(co::TVI::SORT),
			val => Self::Treeitem(unsafe { HTREEITEM::from_ptr(val.raw() as _) }),
		}
	}
}
