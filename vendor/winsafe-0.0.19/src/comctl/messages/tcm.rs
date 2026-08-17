use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TCM_ADJUSTRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-adjustrect)
/// message parameters.
///
/// Return type: `()`.
pub struct AdjustRect<'a> {
	pub display_rect: bool,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for AdjustRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::ADJUSTRECT.into(),
			wparam: self.display_rect as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TCM_DELETEALLITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deleteallitems)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteAllItems {}

unsafe impl MsgSend for DeleteAllItems {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DELETEALLITEMS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteItem {
	pub index: u32,
}

unsafe impl MsgSend for DeleteItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_DESELECTALL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deselectall)
/// message parameters.
///
/// Return type: `()`.
pub struct DeselectAll {
	pub except_current: bool,
}

unsafe impl MsgSend for DeselectAll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DESELECTALL.into(),
			wparam: self.except_current as _,
			lparam: 0,
		}
	}
}

/// [`TCM_GETCURFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getcurfocus)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurFocus {}

unsafe impl MsgSend for GetCurFocus {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETCURFOCUS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

unsafe impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TCS_EX`.
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TCS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TCS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getimagelist)
/// message, which has no parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct GetImageList {}

unsafe impl MsgSend for GetImageList {
	type RetType = SysResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItem<'a, 'b> {
	pub index: u32,
	pub item: &'b mut TCITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *mut _ as _,
		}
	}
}

/// [`TCM_GETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetItemCount {}

unsafe impl MsgSend for GetItemCount {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|c| c as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TCM_GETROWCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getrowcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRowCount {}

unsafe impl MsgSend for GetRowCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETROWCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HWND::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetUnicodeFormat {}

unsafe impl MsgSend for GetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_HIGHLIGHTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-highlightitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HighlightItem {
	pub index: u32,
	pub highlight: bool,
}

unsafe impl MsgSend for HighlightItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::HIGHLIGHTITEM.into(),
			wparam: self.index as _,
			lparam: MAKEDWORD(self.highlight as _, 0) as _,
		}
	}
}

/// [`TCM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-hittest)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct HitTest<'a> {
	pub info: &'a mut TCHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::HITTEST.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TCM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-insertitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct InsertItem<'a, 'b> {
	pub index: u32,
	pub item: &'b TCITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::INSERTITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TCM_REMOVEIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-removeimage)
/// message parameters.
///
/// Return type: `()`.
pub struct RemoveImage {
	pub index: u32,
}

unsafe impl MsgSend for RemoveImage {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::REMOVEIMAGE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETCURFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setcurfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCurFocus {
	pub index: u32,
}

unsafe impl MsgSend for SetCurFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETCURFOCUS.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setcursel)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct SetCurSel {
	pub index: u32,
}

unsafe impl MsgSend for SetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETCURSEL.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setextendedstyle)
/// message parameters.
///
/// Return type: `co::TCS_EX`.
pub struct SetExtendedStyle {
	pub mask: co::TCS_EX,
	pub style: co::TCS_EX,
}

unsafe impl MsgSend for SetExtendedStyle {
	type RetType = co::TCS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TCS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETEXTENDEDSTYLE.into(),
			wparam: self.mask.raw() as _,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`TCM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList<'a> {
	pub himagelist: &'a HIMAGELIST,
}

unsafe impl<'a> MsgSend for SetImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.himagelist.ptr() as _,
		}
	}
}

/// [`TCM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItem<'a, 'b> {
	pub index: u32,
	pub item: &'b TCITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TCM_SETITEMEXTRA`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitemextra)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItemExtra {
	pub num_bytes: u32,
}

unsafe impl MsgSend for SetItemExtra {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETITEMEXTRA.into(),
			wparam: self.num_bytes as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETITEMSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitemsize)
/// message parameters.
///
/// Return type: `(u16, u16)`.
pub struct SetItemSize {
	pub width: u16,
	pub height: u16,
}

unsafe impl MsgSend for SetItemSize {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETITEMSIZE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.width, self.height) as _,
		}
	}
}

/// [`TCM_SETMINTABWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setmintabwidth)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetMinTabWidth {
	pub min_width: Option<u32>,
}

unsafe impl MsgSend for SetMinTabWidth {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETMINTABWIDTH.into(),
			wparam: 0,
			lparam: self.min_width.map_or(-1, |w| w as _),
		}
	}
}

/// [`TCM_SETPADDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setpadding)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPadding {
	pub horizontal: u16,
	pub vertical: u16,
}

unsafe impl MsgSend for SetPadding {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETPADDING.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.horizontal, self.vertical) as _,
		}
	}
}

/// [`TCM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-settooltips)
/// message parameters.
///
/// Return type: `()`.
pub struct SetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

unsafe impl<'a> MsgSend for SetTooltips<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TCM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetUnicodeFormat {
	pub use_unicode: bool,
}

unsafe impl MsgSend for SetUnicodeFormat {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
