use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`SB_GETBORDERS`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-getborders)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetBorders<'a> {
	pub borders: &'a mut [u32; 3],
}

unsafe impl<'a> MsgSend for GetBorders<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETBORDERS.into(),
			wparam: 0,
			lparam: self.borders.as_mut_ptr() as _,
		}
	}
}

/// [`SB_GETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-geticon)
/// message parameters.
///
/// Return type: `SysResult<HICON>`.
pub struct GetIcon {
	pub part_index: u8,
}

unsafe impl MsgSend for GetIcon {
	type RetType = SysResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HICON::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETICON.into(),
			wparam: self.part_index as _,
			lparam: 0,
		}
	}
}

/// [`SB_GETPARTS`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-getparts)
/// message parameters.
///
/// Return type: `u8`.
pub struct GetParts<'a> {
	pub right_edges: Option<&'a mut [i32]>,
}

unsafe impl<'a> MsgSend for GetParts<'a> {
	type RetType = u8;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETPARTS.into(),
			wparam: self.right_edges.as_ref().map_or(0, |re| re.len()),
			lparam: self.right_edges.as_mut().map_or(0, |re| re.as_mut_ptr() as _),
		}
	}
}

/// [`SB_GETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-getrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetRect<'a> {
	pub part_index: u8,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETRECT.into(),
			wparam: self.part_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`SB_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-gettext)
/// message parameters.
///
/// Return type: `(u16, co::SBT)`.
pub struct GetText<'a> {
	pub part_index: u8,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetText<'a> {
	type RetType = (u16, co::SBT);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), unsafe { co::SBT::from_raw(HIWORD(v as _)) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTEXT.into(),
			wparam: self.part_index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`SB_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-gettextlength)
/// message parameters.
///
/// Return type: `(u16, co::SBT)`.
pub struct GetTextLength {
	pub part_index: u8,
}

unsafe impl MsgSend for GetTextLength {
	type RetType = (u16, co::SBT);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), unsafe { co::SBT::from_raw(HIWORD(v as _)) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTEXTLENGTH.into(),
			wparam: self.part_index as _,
			lparam: 0,
		}
	}
}

/// [`SB_GETTIPTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-gettiptext)
/// message parameters.
///
/// Return type: `()`.
pub struct GetTipText<'a> {
	pub part_index: u8,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetTipText<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::GETTIPTEXT.into(),
			wparam: MAKEDWORD(self.part_index as _, self.text.buf_len() as _) as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`SB_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-getunicodeformat)
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
			msg_id: co::SB::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`SB_ISSIMPLE`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-issimple)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct IsSimple {}

unsafe impl MsgSend for IsSimple {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::ISSIMPLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`SB_SETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-setbkcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct SetBkColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(unsafe { COLORREF::from_raw(v) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETBKCOLOR.into(),
			wparam: 0,
			lparam: self.color.map_or(CLR_DEFAULT, |color| color.into()) as _,
		}
	}
}

/// [`SB_SETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-seticon)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetIcon<'a> {
	pub part_index: u8,
	pub hicon: Option<&'a HICON>,
}

unsafe impl<'a> MsgSend for SetIcon<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETICON.into(),
			wparam: self.part_index as _,
			lparam: self.hicon.map_or(0, |p| p.ptr() as _),
		}
	}
}

/// [`SB_SETMINHEIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-setminheight)
/// message parameters.
///
/// Return value: `()`.
pub struct SetMinHeight {
	pub min_height: u32,
}

unsafe impl MsgSend for SetMinHeight {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETMINHEIGHT.into(),
			wparam: self.min_height as _,
			lparam: 0,
		}
	}
}

/// [`SB_SETPARTS`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-setparts)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetParts<'a> {
	pub right_edges: &'a [i32],
}

unsafe impl<'a> MsgSend for SetParts<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETPARTS.into(),
			wparam: self.right_edges.len(),
			lparam: vec_ptr(self.right_edges) as _,
		}
	}
}

/// [`SB_SETTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-settext)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetText {
	pub part_index: u8,
	pub draw_operation: co::SBT,
	pub text: WString,
}

unsafe impl MsgSend for SetText {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETTEXT.into(),
			wparam: MAKEDWORD(MAKEWORD(self.part_index, 0), self.draw_operation.raw()) as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`SB_SETTIPTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-settiptext)
/// message parameters.
pub struct SetTipText {
	pub part_index: u8,
	pub text: WString,
}

unsafe impl MsgSend for SetTipText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SETTIPTEXT.into(),
			wparam: self.part_index as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`SB_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-setunicodeformat)
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
			msg_id: co::SB::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`SB_SIMPLE`](https://learn.microsoft.com/en-us/windows/win32/controls/sb-simple)
/// message parameters.
pub struct Simple {
	pub display_simple: bool,
}

unsafe impl MsgSend for Simple {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::SB::SIMPLE.into(),
			wparam: self.display_simple as _,
			lparam: 0,
		}
	}
}
