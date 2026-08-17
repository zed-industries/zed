#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

/// Variant parameter for:
///
/// * [`wm::Command`](crate::msg::wm::Command).
pub enum AccelMenuCtrl {
	/// Accelerator event. Contains the accelerator command ID.
	Accel(u16),
	/// Menu item click event. Contains the menu item command ID.
	Menu(u16),
	/// Some child control event. Contains
	/// [`AccelMenuCtrlData`](crate::AccelMenuCtrlData) data.
	Ctrl(AccelMenuCtrlData),
}

impl AccelMenuCtrl {
	/// Returns the notification code and the control ID pair.
	#[must_use]
	pub const fn code_id(&self) -> (co::CMD, u16) {
		match self {
			AccelMenuCtrl::Accel(id) => (co::CMD::Accelerator, *id),
			AccelMenuCtrl::Menu(id) => (co::CMD::Menu, *id),
			AccelMenuCtrl::Ctrl(data) => (data.notif_code, data.ctrl_id),
		}
	}
}

/// Variant parameter for:
///
/// * [`AccelMenuCtrl`](crate::AccelMenuCtrl).
pub struct AccelMenuCtrlData {
	pub notif_code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

/// Variant parameter for:
///
/// * [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx);
/// /// * [`HWND::FindWindow`](crate::prelude::user_Hwnd::FindWindow);
/// * [`HWND::FindWindowEx`](crate::prelude::user_Hwnd::FindWindowEx);
/// * [`UnregisterClass`](crate::UnregisterClass).
#[derive(Clone)]
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(WString),
}

impl AtomStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Atom(atom) => MAKEINTRESOURCE(u16::from(*atom) as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`bm::GetImage`](crate::msg::bm::GetImage);
/// * [`bm::SetImage`](crate::msg::bm::SetImage).
pub enum BmpIcon {
	/// A bitmap.
	Bmp(HBITMAP),
	/// An icon.
	Icon(HICON),
}

impl BmpIcon {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		unsafe {
			std::mem::transmute(match self {
				BmpIcon::Bmp(hbmp) => hbmp.ptr(),
				BmpIcon::Icon(hicon) => hicon.ptr(),
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu).
pub enum BmpPtrStr {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A pointer to anything.
	Ptr(*const std::ffi::c_void),
	/// A string.
	Str(WString),
	/// Nothing.
	None,
}

impl BmpPtrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Bmp(hbmp) => hbmp.ptr() as _,
			Self::Ptr(lp) => *lp as _,
			Self::Str(ws) => ws.as_ptr(),
			Self::None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`DEVMODE`](crate::DEVMODE).
#[derive(Clone, Copy)]
pub enum DispfNup {
	/// Used for displays.
	Dispf(co::DMDISPLAYFLAGS),
	/// Used for printers.
	Nup(co::DMNUP),
}

/// Variant parameter for:
///
/// * [`EnumDisplaySettings`](crate::EnumDisplaySettings).
#[derive(Clone, Copy)]
pub enum GmidxEnum {
	/// Graphics mode index.
	Gmidx(u32),
	/// Predefined enumeration.
	Enum(co::ENUM_SETTINGS),
}

impl From<GmidxEnum> for u32 {
	fn from(v: GmidxEnum) -> Self {
		match v {
			GmidxEnum::Gmidx(idx) => idx,
			GmidxEnum::Enum(es) => es.raw(),
		}
	}
}

/// Variant parameter for:
///
/// * [`INPUT`](crate::INPUT).
#[derive(Clone, Copy)]
pub enum HwKbMouse {
	/// Hardware event.
	Hw(HARDWAREINPUT),
	/// Keyboard event.
	Kb(KEYBDINPUT),
	/// Mouse event.
	Mouse(MOUSEINPUT),
}

/// Variant parameter for:
///
/// * [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl).
pub enum HwndFocus {
	/// Handle to the control to receive the focus.
	Hwnd(HWND),
	/// If `true`, the next control with [`WS::TABSTOP`](crate::co::WS::TABSTOP)
	/// receives the focus; if `false`, the previous.
	FocusNext(bool),
}

/// Variant parameter for:
///
/// * [`wm::EnterIdle`](crate::msg::wm::EnterIdle);
/// * [`HELPINFO`](crate::HELPINFO).
pub enum HwndHmenu {
	/// A window.
	Hwnd(HWND),
	/// A menu.
	Hmenu(HMENU),
}

impl HwndHmenu {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		match self {
			HwndHmenu::Hwnd(hwnd) => hwnd.ptr() as _,
			HwndHmenu::Hmenu(hmenu) => hmenu.ptr() as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos);
/// * [`WINDOWPOS`](crate::WINDOWPOS);
/// * [`WINDOWPOS`](crate::WINDOWPOS).
pub enum HwndPlace {
	/// A handle to the window to precede the positioned window in the Z order.
	Hwnd(HWND),
	/// A constant specifying where the window will be placed.
	Place(co::HWND_PLACE),
	/// Nothing.
	None,
}

impl HwndPlace {
	/// Returns a pointer to the raw data content, or null if no content.
	#[must_use]
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Hwnd(hwnd) => hwnd.ptr(),
			Self::Place(v) => v.raw() as _,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::ParentNotify`](crate::msg::wm::ParentNotify).
pub enum HwndPointId {
	/// Handle to the child window.
	Hwnd(HWND),
	/// Cursor coordinates.
	Point(POINT),
	/// Pointer identifier.
	Id(u16),
}

impl HwndPointId {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		match self {
			HwndPointId::Hwnd(hwnd) => hwnd.ptr() as _,
			HwndPointId::Point(pt) => u32::from(*pt) as _,
			HwndPointId::Id(id) => *id as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadCursor`](crate::prelude::user_Hinstance::LoadCursor).
#[derive(Clone)]
pub enum IdIdcStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdcStr {
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
			Self::Idc(idc) => MAKEINTRESOURCE(idc.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadIcon`](crate::prelude::user_Hinstance::LoadIcon).
#[derive(Clone)]
pub enum IdIdiStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDI`](crate::co::IDI) constant for a stock system icon.
	Idi(co::IDI),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdiStr {
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
			Self::Idi(idi) => MAKEINTRESOURCE(idi.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu);
/// * [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
pub enum IdMenu<'a> {
	/// A command ID.
	Id(u16),
	/// An [`HMENU`](crate::HMENU).
	Menu(&'a HMENU),
	/// Nothing.
	None,
}

impl<'a> IdMenu<'a> {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Id(id) => *id as _,
			Self::Menu(hMenu) => hMenu.ptr(),
			Self::None => std::ptr::null_mut(),
		}
	}

	/// Converts the raw data into an `usize`.
	#[must_use]
	pub fn as_usize(&self) -> usize {
		match self {
			IdMenu::Id(id) => *id as _,
			IdMenu::Menu(hMenu) => hMenu.ptr() as _,
			IdMenu::None => 0,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::CheckMenuItem`](crate::prelude::user_Hmenu::CheckMenuItem);
/// * [`HMENU::CheckMenuRadioItem`](crate::prelude::user_Hmenu::CheckMenuRadioItem);
/// * [`HMENU::DeleteMenu`](crate::prelude::user_Hmenu::DeleteMenu);
/// * [`HMENU::EnableMenuItem`](crate::prelude::user_Hmenu::EnableMenuItem);
/// * [`HMENU::GetMenuDefaultItem`](crate::prelude::user_Hmenu::GetMenuDefaultItem);
/// * [`HMENU::GetMenuItemInfo`](crate::prelude::user_Hmenu::GetMenuItemInfo);
/// * [`HMENU::GetMenuState`](crate::prelude::user_Hmenu::GetMenuState);
/// * [`HMENU::GetMenuString`](crate::prelude::user_Hmenu::GetMenuString);
/// * [`HMENU::InsertMenuItem`](crate::prelude::user_Hmenu::InsertMenuItem);
/// * [`HMENU::RemoveMenu`](crate::prelude::user_Hmenu::RemoveMenu);
/// * [`HMENU::SetMenuItemBitmaps`](crate::prelude::user_Hmenu::SetMenuItemBitmaps);
/// * [`HMENU::SetMenuItemInfo`](crate::prelude::user_Hmenu::SetMenuItemInfo);
/// * [`HWND::HiliteMenuItem`](crate::prelude::user_Hwnd::HiliteMenuItem).
#[derive(Clone, Copy)]
pub enum IdPos {
	/// A command ID.
	Id(u16),
	/// Zero-based position.
	Pos(u32),
}

impl IdPos {
	/// Returns whether value is `Pos`.
	#[must_use]
	pub const fn is_by_pos(self) -> bool {
		match self {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		}
	}

	/// Returns the ID or the position as a plain `u32`.
	#[must_use]
	pub const fn id_or_pos_u32(self) -> u32 {
		match self {
			IdPos::Id(id) => id as _,
			IdPos::Pos(pos) => pos,
		}
	}

	/// Returns [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) if value is `Id`, or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) if value is `Pos`.
	#[must_use]
	pub const fn mf_flag(self) -> co::MF {
		match self {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::append_item`](crate::prelude::user_Hmenu::append_item).
pub enum MenuItem<'a> {
	/// A selectable entry item, with command ID and text.
	Entry(u16, &'a str),
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu(&'a HMENU, &'a str),
}

/// Variant parameter for:
///
/// * [`HMENU::item_info`](crate::prelude::user_Hmenu::item_info);
/// * [`HMENU::iter_items`](crate::prelude::user_Hmenu::iter_items).
pub enum MenuItemInfo {
	/// A selectable entry item, with command ID and text.
	Entry(u16, String),
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu(HMENU, String),
}

/// Variant parameter for:
///
/// * [`wm::NcCalcSize`](crate::msg::wm::NcCalcSize).
pub enum NccspRect<'a, 'b> {
	/// Mutable reference to [`NCCALCSIZE_PARAMS`](crate::NCCALCSIZE_PARAMS).
	Nccsp(&'b mut NCCALCSIZE_PARAMS<'a>),
	/// Mutable reference to [`RECT`](crate::RECT).
	Rect(&'b mut RECT),
}

/// Variant parameter for:
///
/// * [`HWND::MapWindowPoints`](crate::prelude::user_Hwnd::MapWindowPoints) `points`.
pub enum PtsRc<'a> {
	/// A series of [`POINT`](crate::POINT) structs.
	Pts(&'a mut [POINT]),
	/// A single [`RECT`](crate::RECT) struct.
	Rc(&'a mut RECT),
}
