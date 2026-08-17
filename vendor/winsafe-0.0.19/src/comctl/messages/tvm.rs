use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TVM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct CreateDragImage<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for CreateDragImage<'a> {
	type RetType = SysResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::CREATEDRAGIMAGE.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteItem<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for DeleteItem<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::DELETEITEM.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_EDITLABEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-editlabel)
/// message parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct EditLabel<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for EditLabel<'a> {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::EDITLABEL.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_ENDEDITLABELNOW`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-endeditlabelnow)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct EndEditLabelNow {
	pub save: bool,
}

unsafe impl MsgSend for EndEditLabelNow {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::ENDEDITLABELNOW.into(),
			wparam: !self.save as _,
			lparam: 0,
		}
	}
}

/// [`TVM_ENSUREVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-ensurevisible)
/// message parameters.
///
/// Return type: `u32`.
pub struct EnsureVisible<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for EnsureVisible<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::ENSUREVISIBLE.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_EXPAND`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-expand)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct Expand<'a> {
	pub action: co::TVE,
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for Expand<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::EXPAND.into(),
			wparam: self.action.raw() as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_GETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct GetBkColor {}

unsafe impl MsgSend for GetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCount {}

unsafe impl MsgSend for GetCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEDITCONTROL`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-geteditcontrol)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct GetEditControl {}

unsafe impl MsgSend for GetEditControl {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETEDITCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TVS_EX`.
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TVS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TVS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetImageList {
	pub kind: co::TVSIL,
}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINDENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getindent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetIndent {}

unsafe impl MsgSend for GetIndent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETINDENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getinsertmarkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
pub struct GetInsertMarkColor {}

unsafe impl MsgSend for GetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETISEARCHSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getisearchstring)
/// message parameters.
///
/// Return type: `u32`.
pub struct GetISearchString<'a> {
	pub buf: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetISearchString<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETISEARCHSTRING.into(),
			wparam: 0,
			lparam: unsafe { self.buf.as_mut_ptr() } as _,
		}
	}
}

/// [`TVM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItem<'a, 'b> {
	pub tvitem: &'b mut TVITEMEX<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemheight)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetItemHeight {}

unsafe impl MsgSend for GetItemHeight {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMHEIGHT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItemRect<'a> {
	pub text_only: bool,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMRECT.into(),
			wparam: self.text_only as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TVM_GETITEMSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getitemstate)
/// message parameters.
///
/// Return type: `co::TVIS`.
pub struct GetItemState<'a> {
	pub hitem: &'a HTREEITEM,
	pub mask: co::TVIS,
}

unsafe impl<'a> MsgSend for GetItemState<'a> {
	type RetType = co::TVIS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TVIS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETITEMSTATE.into(),
			wparam: self.hitem.ptr() as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`TVM_GETLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getlinecolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct GetLineColor {}

unsafe impl MsgSend for GetLineColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			c => Some(unsafe { COLORREF::from_raw(c) })
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETLINECOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETNEXTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getnextitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct GetNextItem<'a> {
	pub relationship: co::TVGN,
	pub hitem: Option<&'a HTREEITEM>,
}

unsafe impl<'a> MsgSend for GetNextItem<'a> {
	type RetType = Option<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETNEXTITEM.into(),
			wparam: self.relationship.raw() as _,
			lparam: self.hitem.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_GETSCROLLTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getscrolltime)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetScrollTime {}

unsafe impl MsgSend for GetScrollTime {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETSCROLLTIME.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-gettextcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct GetTextColor {}

unsafe impl MsgSend for GetTextColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETTEXTCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getunicodeformat)
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
			msg_id: co::TVM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_GETVISIBLECOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-getvisiblecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetVisibleCount {}

unsafe impl MsgSend for GetVisibleCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::GETVISIBLECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TVM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-hittest)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct HitTest<'a> {
	pub info: &'a TVHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = Option<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::HITTEST.into(),
			wparam: 0,
			lparam: &mut self.info as *mut _ as _,
		}
	}
}

/// [`TVM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<HTREEITEM>`.
pub struct InsertItem<'a, 'b> {
	pub item: &'b TVINSERTSTRUCT<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = SysResult<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::INSERTITEM.into(),
			wparam: 0,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TVM_MAPACCIDTOHTREEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-mapaccidtohtreeitem)
/// message parameters.
///
/// Return type: `Option<HTREEITEM>`.
pub struct MapAccIdToHtreeitem {
	pub acc_id: u32,
}

unsafe impl MsgSend for MapAccIdToHtreeitem {
	type RetType = SysResult<HTREEITEM>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HTREEITEM::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::MAPACCIDTOHTREEITEM.into(),
			wparam: self.acc_id as _,
			lparam: 0,
		}
	}
}

/// [`TVM_MAPHTREEITEMTOACCID`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-maphtreeitemtoaccid)
/// message parameters.
///
/// Return type: `u32`.
pub struct MapHtreeitemToAccId<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for MapHtreeitemToAccId<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::MAPHTREEITEMTOACCID.into(),
			wparam: self.hitem.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SELECTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-selectitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SelectItem<'a> {
	pub action: co::TVGN,
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for SelectItem<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SELECTITEM.into(),
			wparam: self.action.raw() as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SETAUTOSCROLLINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setautoscrollinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct SetAutoScrollInfo {
	pub pixels_per_second: u32,
	pub redraw_interval: u32,
}

unsafe impl MsgSend for SetAutoScrollInfo {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETAUTOSCROLLINFO.into(),
			wparam: self.pixels_per_second as _,
			lparam: self.redraw_interval as _,
		}
	}
}

/// [`TVM_SETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setbkcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct SetBkColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(unsafe { COLORREF::from_raw(v as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(-1, |color| u32::from(color) as _),
		}
	}
}

/// [`TVM_SETBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setborder)
/// message parameters.
///
/// Return type: `(u16, u16)`.
pub struct SetBorder {
	pub action: co::TVSBF,
	pub left: u16,
	pub top: u16,
}

unsafe impl MsgSend for SetBorder {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETBORDER.into(),
			wparam: self.action.raw() as _,
			lparam: MAKEDWORD(self.left, self.top) as _,
		}
	}
}

/// [`TVM_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setextendedstyle)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct SetExtendedStyle {
	pub style: co::TVS_EX,
	pub mask: co::TVS_EX,
}

unsafe impl MsgSend for SetExtendedStyle {
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		ok_to_hrresult(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETEXTENDEDSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.mask.raw() as _,
		}
	}
}

/// [`TVM_SETHOT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sethot)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetHot<'a> {
	pub hitem: Option<&'a HTREEITEM>,
}

unsafe impl<'a> MsgSend for SetHot<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETHOT.into(),
			wparam: 0,
			lparam: self.hitem.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList<'a> {
	pub kind: co::TVSIL,
	pub himagelist: Option<&'a HIMAGELIST>,
}

unsafe impl<'a> MsgSend for SetImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETIMAGELIST.into(),
			wparam: self.kind.raw() as _,
			lparam: self.himagelist.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TVM_SETINDENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setindent)
/// message parameters.
///
/// Return type: `()`.
pub struct SetIndent {
	pub width: u32,
}

unsafe impl MsgSend for SetIndent {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETINDENT.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setinsertmark)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetInsertMark<'a> {
	pub insert_after: bool,
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for SetInsertMark<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETINSERTMARK.into(),
			wparam: self.insert_after as _,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setinsertmarkcolor)
/// message parameters.
///
/// Return type: `COLORREF`.
pub struct SetInsertMarkColor {
	pub color: COLORREF,
}

unsafe impl MsgSend for SetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`TVM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItem<'a, 'b> {
	pub tvitem: &'b TVITEMEX<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETITEM.into(),
			wparam: 0,
			lparam: self.tvitem as *const _ as _,
		}
	}
}

/// [`TVM_SETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setitemheight)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetItemHeight {
	pub height: Option<u32>,
}

unsafe impl MsgSend for SetItemHeight {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETITEMHEIGHT.into(),
			wparam: self.height.map_or(-1, |h| h as _) as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETLINECOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setlinecolor)
/// message parameters.
///
/// Return type: `COLORREF`.
pub struct SetLineColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetLineColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETLINECOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(CLR_DEFAULT, |c| c.into()) as _,
		}
	}
}

/// [`TVM_SETSCROLLTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setscrolltime)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetScrollTime {
	pub time_ms: u32,
}

unsafe impl MsgSend for SetScrollTime {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETSCROLLTIME.into(),
			wparam: self.time_ms as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SETTEXTCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-settextcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct SetTextColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetTextColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => None,
			v => Some(unsafe { COLORREF::from_raw(v as _) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETTEXTCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(-1, |color| u32::from(color) as _),
		}
	}
}

/// [`TVM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-settooltips)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct SetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

unsafe impl<'a> MsgSend for SetTooltips<'a> {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TVM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-setunicodeformat)
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
			msg_id: co::TVM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SHOWINFOTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-showinfotip)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowInfoTip<'a> {
	pub hitem: &'a HTREEITEM,
}

unsafe impl<'a> MsgSend for ShowInfoTip<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SHOWINFOTIP.into(),
			wparam: 0,
			lparam: self.hitem.ptr() as _,
		}
	}
}

/// [`TVM_SORTCHILDREN`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sortchildren)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SortChildren {
	pub recursive: bool,
}

unsafe impl MsgSend for SortChildren {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SORTCHILDREN.into(),
			wparam: self.recursive as _,
			lparam: 0,
		}
	}
}

/// [`TVM_SORTCHILDRENCB`](https://learn.microsoft.com/en-us/windows/win32/controls/tvm-sortchildrencb)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SortChildrenCb<'a> {
	pub info: &'a TVSORTCB,
}

unsafe impl<'a> MsgSend for SortChildrenCb<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TVM::SORTCHILDRENCB.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}
