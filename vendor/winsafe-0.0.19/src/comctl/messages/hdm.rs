use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`HDM_CLEARFILTER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-clearfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct ClearFilter {
	pub filter: Option<u32>,
}

unsafe impl MsgSend for ClearFilter {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::CLEARFILTER.into(),
			wparam: self.filter.map_or(-1, |f| f as i32) as _,
			lparam: 0,
		}
	}
}

/// [`HDM_CREATEDRAGIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-createdragimage)
/// message parameters.
///
/// Return type: `SysResult<HIMAGELIST>`.
pub struct CreateDragImage {
	pub index: u32,
}

unsafe impl MsgSend for CreateDragImage {
	type RetType = SysResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::CREATEDRAGIMAGE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-deleteitem)
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
			msg_id: co::HDM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`HDM_EDITFILTER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-editfilter)
/// message parameters.
///
/// Return type: `bool`.
pub struct EditFilter {
	pub index: u32,
	pub discard_changes: bool,
}

unsafe impl MsgSend for EditFilter {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::EDITFILTER.into(),
			wparam: self.index as _,
			lparam: self.discard_changes as _,
		}
	}
}

/// [`HDM_GETBITMAPMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getbitmapmargin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetBitmapMargin {}

unsafe impl MsgSend for GetBitmapMargin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETBITMAPMARGIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETFOCUSEDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getfocuseditem)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetFocusedItem {}

unsafe impl MsgSend for GetFocusedItem {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETFOCUSEDITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetImageList {
	pub which: co::HDSIL,
}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETIMAGELIST.into(),
			wparam: self.which.raw() as _,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b mut HDITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *mut _ as _,
		}
	}
}

/// [`HDM_GETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetItemCount {}

unsafe impl MsgSend for GetItemCount {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_GETITEMDROPDOWNRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemdropdownrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItemDropDownRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemDropDownRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETITEMDROPDOWNRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getitemrect)
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
			msg_id: co::HDM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getorderarray)
/// message parameters.
///
/// Return type `SysResult<()>`.
pub struct GetOrderArray<'a> {
	pub buffer: &'a mut [u32],
}

unsafe impl<'a> MsgSend for GetOrderArray<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

/// [`HDM_GETOVERFLOWRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getoverflowrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetOverflowRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetOverflowRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::GETOVERFLOWRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`HDM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-getunicodeformat)
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
			msg_id: co::HDM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`HDM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-hittest)
/// message parameters.
///
/// Return type: `u32`.
pub struct HitTest<'a> {
	pub test_info: &'a mut HDHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`HDM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-insertitem)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct InsertItem<'a, 'b> {
	pub index_after: u32,
	pub item: &'b HDITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for InsertItem<'a, 'b> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::INSERTITEM.into(),
			wparam: self.index_after as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`HDM_LAYOUT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-layout)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct Layout<'a, 'b, 'c> {
	pub hdlayout: &'c mut HDLAYOUT<'a, 'b>,
}

unsafe impl<'a, 'b, 'c> MsgSend for Layout<'a, 'b, 'c> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::LAYOUT.into(),
			wparam: 0,
			lparam: self.hdlayout as *mut _ as _,
		}
	}
}

/// [`HDM_ORDERTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-ordertoindex)
/// message parameters.
///
/// Return type: `u32`.
pub struct OrderToIndex {
	pub order: u32,
}

unsafe impl MsgSend for OrderToIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::ORDERTOINDEX.into(),
			wparam: self.order as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETBITMAPMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setbitmapmargin)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetBitmapMargin {
	pub width: u32,
}

unsafe impl MsgSend for SetBitmapMargin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETBITMAPMARGIN.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFILTERCHANGETIMEOUT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setfilterchangetimeout)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetFilterChangeTimeout {
	pub timeout_ms: u32,
}

unsafe impl MsgSend for SetFilterChangeTimeout {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: self.timeout_ms as _,
			lparam: 0,
		}
	}
}

/// [`HDM_SETFOCUSEDITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setfocuseditem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetFocusedItem {
	pub index: u32,
}

unsafe impl MsgSend for SetFocusedItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETFILTERCHANGETIMEOUT.into(),
			wparam: 0,
			lparam: self.index as _,
		}
	}
}

/// [`HDM_SETHOTDIVIDER`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-sethotdivider)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetHotDivider {
	pub value: PtIdx,
}

unsafe impl MsgSend for SetHotDivider {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETHOTDIVIDER.into(),
			wparam: match self.value {
				PtIdx::Pt(_) => true,
				PtIdx::Idx(_) => false,
			} as _,
			lparam: match self.value {
				PtIdx::Pt(pt) => pt.into(),
				PtIdx::Idx(idx) => idx,
			} as _,
		}
	}
}

/// [`HDM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList<'a> {
	pub which: co::HDSIL,
	pub himagelist: &'a HIMAGELIST,
}

unsafe impl<'a> MsgSend for SetImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETIMAGELIST.into(),
			wparam: self.which.raw() as _,
			lparam: self.himagelist.ptr() as _,
		}
	}
}

/// [`HDM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItem<'a, 'b> {
	pub index: u32,
	pub hditem: &'b HDITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETITEM.into(),
			wparam: self.index as _,
			lparam: self.hditem as *const _ as _,
		}
	}
}

/// [`HDM_SETORDERARRAY`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setorderarray)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetOrderArray<'a> {
	pub buffer: &'a [u32],
}

unsafe impl<'a> MsgSend for SetOrderArray<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::HDM::SETORDERARRAY.into(),
			wparam: self.buffer.len(),
			lparam: vec_ptr(self.buffer) as _,
		}
	}
}

/// [`HDM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/hdm-setunicodeformat)
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
			msg_id: co::HDM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
