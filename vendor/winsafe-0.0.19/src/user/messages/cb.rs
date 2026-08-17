use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`CB_ADDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-addstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct AddString {
	pub text: WString,
}

unsafe impl MsgSend for AddString {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::ADDSTRING.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_DELETESTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-deletestring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct DeleteString {
	pub index: u32,
}

unsafe impl MsgSend for DeleteString {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::DELETESTRING.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_DIR`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-dir)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct Dir {
	pub attributes: co::DDL,
	pub path: WString,
}

unsafe impl MsgSend for Dir {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::DIR.into(),
			wparam: self.attributes.raw() as _,
			lparam: self.path.as_ptr() as _,
		}
	}
}

/// [`CB_FINDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for FindString {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::FINDSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_FINDSTRINGEXACT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct FindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for FindStringExact {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::FINDSTRINGEXACT.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_GETCOMBOBOXINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcomboboxinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetComboBoxInfo<'a> {
	pub data: &'a mut COMBOBOXINFO,
}

unsafe impl<'a> MsgSend for GetComboBoxInfo<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCOMBOBOXINFO.into(),
			wparam: 0,
			lparam: self.data as *mut _ as _,
		}
	}
}

/// [`CB_GETCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetCount {}

unsafe impl MsgSend for GetCount {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

unsafe impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDCONTROLRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedcontrolrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetDroppedControlRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetDroppedControlRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDCONTROLRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`CB_GETDROPPEDSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedstate)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetDroppedState {}

unsafe impl MsgSend for GetDroppedState {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETDROPPEDWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getdroppedwidth)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetDroppedWidth {}

unsafe impl MsgSend for GetDroppedWidth {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETDROPPEDWIDTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEDITSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-geteditsel)
/// message, which has no parameters.
///
/// Return type: `(i32, i32)`.
pub struct GetEditSel {}

unsafe impl MsgSend for GetEditSel {
	type RetType = (i32, i32);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETEDITSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETEXTENDEDUI`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getextendedui)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetExtendedUi {}

unsafe impl MsgSend for GetExtendedUi {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETEXTENDEDUI.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetHorizontalExtent {}

unsafe impl MsgSend for GetHorizontalExtent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getitemdata)
/// message parameters.
///
/// Return type: `SysResult<isize>`.
pub struct GetItemData {
	pub index: u32,
}

unsafe impl MsgSend for GetItemData {
	type RetType = SysResult<isize>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(v),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETITEMDATA.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getitemheight)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetItemHeight {
	pub component: i32,
}

unsafe impl MsgSend for GetItemHeight {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cy => Ok(cy as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLBTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlbtext)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetLbText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetLbText<'a> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`CB_GETLBTEXTLEN`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlbtextlen)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetLbTextLen {
	pub index: u32,
}

unsafe impl MsgSend for GetLbTextLen {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLBTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_GETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getlocale)
/// message, which has no parameters.
///
/// Return type: `LANGID`.
pub struct GetLocale {}

unsafe impl MsgSend for GetLocale {
	type RetType = LANGID;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { LANGID::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETLOCALE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_GETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-gettopindex)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetTopIndex {}

unsafe impl MsgSend for GetTopIndex {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_INITSTORAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-initstorage)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct InitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

unsafe impl MsgSend for InitStorage {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			n => Ok(n as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

/// [`CB_INSERTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-insertstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct InsertString {
	pub index: Option<u32>,
	pub text: WString,
}

unsafe impl MsgSend for InsertString {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			CB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::INSERTSTRING.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_LIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-limittext)
/// message parameters.
///
/// Return type: `()`.
pub struct LimitText {
	pub max_chars: Option<u32>,
}

unsafe impl MsgSend for LimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::LIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or_default() as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { ResetContent: co::CB::RESETCONTENT.into();
	/// [`CB_RESETCONTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-resetcontent)
}

/// [`CB_SELECTSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-selectstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct SelectString {
	pub preceding_index: Option<u32>,
	pub search_text: WString,
}

unsafe impl MsgSend for SelectString {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SELECTSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.search_text.as_ptr() as _,
		}
	}
}

/// [`CB_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setcursel)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCurSel {
	pub index: Option<u32>,
}

unsafe impl MsgSend for SetCurSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETCURSEL.into(),
			wparam: match self.index {
				Some(index) => index as i32,
				None => -1,
			} as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETDROPPEDWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setdroppedwidth)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct SetDroppedWidth {
	pub min_width: u32,
}

unsafe impl MsgSend for SetDroppedWidth {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			cx => Ok(cx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETDROPPEDWIDTH.into(),
			wparam: self.min_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETEDITSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-seteditsel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetEditSel {
	pub start_pos: Option<u32>,
	pub end_pos: Option<u32>,
}

unsafe impl MsgSend for SetEditSel {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETEDITSEL.into(),
			wparam: 0,
			lparam: MAKEDWORD(
				self.start_pos.map_or(-1, |pos| pos as i16) as _,
				self.end_pos.map_or(-1, |pos| pos as i16) as _,
			) as _,
		}
	}
}

/// [`CB_SETEXTENDEDUI`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setextendedui)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetExtendedUi {
	pub use_extended_ui: bool,
}

unsafe impl MsgSend for SetExtendedUi {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETEXTENDEDUI.into(),
			wparam: self.use_extended_ui as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETHORIZONTALEXTENT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-sethorizontalextent)
/// message parameters.
///
/// Return type: `()`.
pub struct SetHorizontalExtent {
	pub scrollable_width: u32,
}

unsafe impl MsgSend for SetHorizontalExtent {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETHORIZONTALEXTENT.into(),
			wparam: self.scrollable_width as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETITEMDATA`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setitemdata)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItemData {
	pub index: u32,
	pub data: isize,
}

unsafe impl MsgSend for SetItemData {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETITEMDATA.into(),
			wparam: self.index as _,
			lparam: self.data,
		}
	}
}

/// [`CB_SETITEMHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setitemheight)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetItemHeight {
	pub component: i32,
	pub height: u32,
}

unsafe impl MsgSend for SetItemHeight {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETITEMHEIGHT.into(),
			wparam: self.component as _,
			lparam: self.height as _,
		}
	}
}

/// [`CB_SETLOCALE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setlocale)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetLocale {
	pub locale: LANGID,
}

unsafe impl MsgSend for SetLocale {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			CB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETLOCALE.into(),
			wparam: u16::from(self.locale) as _,
			lparam: 0,
		}
	}
}

/// [`CB_SETTOPINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-settopindex)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTopIndex {
	pub index: u32,
}

unsafe impl MsgSend for SetTopIndex {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`CB_SHOWDROPDOWN`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-showdropdown)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowDropDown {
	pub show: bool,
}

unsafe impl MsgSend for ShowDropDown {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SHOWDROPDOWN.into(),
			wparam: self.show as _,
			lparam: 0,
		}
	}
}
