use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`EN_CANUNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/em-canundo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct CanUndo {}

unsafe impl MsgSend for CanUndo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CANUNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_CHARFROMPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-charfrompos)
/// message parameters.
///
/// Return type: `(u16, u16)`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
pub struct CharFromPos {
	pub coords: POINT,
}

unsafe impl MsgSend for CharFromPos {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::CHARFROMPOS.into(),
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

pub_struct_msg_empty! { EmptyUndoBuffer: co::EM::EMPTYUNDOBUFFER.into();
	/// [`EM_EMPTYUNDOBUFFER`](https://learn.microsoft.com/en-us/windows/win32/controls/em-emptyundobuffer)
}

/// [`EM_FMTLINES`](https://learn.microsoft.com/en-us/windows/win32/controls/em-fmtlines)
/// message parameters.
///
/// Return type: `bool`.
pub struct FmtLines {
	pub insert_soft_line_breaks: bool,
}

unsafe impl MsgSend for FmtLines {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::FMTLINES.into(),
			wparam: self.insert_soft_line_breaks as _,
			lparam: 0,
		}
	}
}

/// [`EM_GETFIRSTVISIBLELINE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getfirstvisibleline)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetFirstVisibleLine {}

unsafe impl MsgSend for GetFirstVisibleLine {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETFIRSTVISIBLELINE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETHANDLE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-gethandle)
/// message, which has no parameters.
///
/// Return type: `SysResult<HLOCAL>`.
pub struct GetHandle {}

unsafe impl MsgSend for GetHandle {
	type RetType = SysResult<HLOCAL>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { HLOCAL::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETHANDLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETIMESTATUS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getimestatus)
/// message, which has no parameters.
///
/// Return type: `co::EIMES`.
pub struct GetImeStatus {}

unsafe impl MsgSend for GetImeStatus {
	type RetType = co::EIMES;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::EIMES::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: 0,
		}
	}
}

/// [`EM_GETLIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getlimittext)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLimitText {}

unsafe impl MsgSend for GetLimitText {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLIMITTEXT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETLINE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getline)
/// message parameters.
///
/// The message will retrieve at most `buffer.len() - 1` characters for the
/// line, because there must be room for a terminating null.
///
/// Returns the number of chars copied to `buffer`, not counting the terminating
/// null, or `None` if no chars were copied. There is no documented way to
/// differentiate between an error and an empty line.
///
/// Return type: `Option<u32>`.
pub struct GetLine<'a> {
	pub index: u16,
	pub buffer: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetLine<'a> {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|count| count as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		self.buffer.fill_with_zero();
		let buf_len = self.buffer.buf_len() - 1; // leave room for terminating null
		self.buffer.as_mut_slice()
			.iter_mut()
			.next()
			.map(|wchar| *wchar = buf_len as _); // leave room for terminating null

		WndMsg {
			msg_id: co::EM::GETLINE.into(),
			wparam: self.index as _,
			lparam: unsafe { self.buffer.as_mut_ptr() } as _,
		}
	}
}

/// [`EM_GETLINECOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getlinecount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLineCount {}

unsafe impl MsgSend for GetLineCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETLINECOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMARGINS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getmargins)
/// message, which has no parameters.
///
/// Return type: `SIZE`.
pub struct GetMargins {}

unsafe impl MsgSend for GetMargins {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETMARGINS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETMODIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getmodify)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetModify {}

unsafe impl MsgSend for GetModify {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETMODIFY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETPASSWORDCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getpasswordchar)
/// message, which has no parameters.
///
/// Return type: `Option<char>`.
pub struct GetPasswordChar {}

unsafe impl MsgSend for GetPasswordChar {
	type RetType = Option<char>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|c| unsafe { std::char::from_u32_unchecked(c as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETPASSWORDCHAR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`EM_GETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getsel)
/// message parameters.
///
/// Return type: `()`.
pub struct GetSel<'a, 'b> {
	pub first_index: Option<&'a mut u32>,
	pub past_last_index: Option<&'b mut u32>,
}

unsafe impl<'a, 'b> MsgSend for GetSel<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETSEL.into(),
			wparam: self.first_index.as_mut().map_or(0, |r| r as *mut _ as _),
			lparam: self.past_last_index.as_mut().map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`EM_GETTHUMB`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getthumb)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetThumb {}

unsafe impl MsgSend for GetThumb {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_GETWORDBREAKPROC`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getwordbreakproc)
/// message, which has no parameters.
///
/// Return type: `Option<EDITWORDBREAKPROC>`.
pub struct GetWordBreakProc {}

unsafe impl MsgSend for GetWordBreakProc {
	type RetType = Option<EDITWORDBREAKPROC>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { std::mem::transmute(p) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETTHUMB.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_LIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-limittext)
/// message parameters.
///
/// Return type: `()`.
pub struct LimitText {
	pub max: Option<u32>,
}

unsafe impl MsgSend for LimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LIMITTEXT.into(),
			wparam: self.max.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEFROMCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linefromchar)
/// message parameters.
///
/// Return type: `u32`.
pub struct LineFromChar {
	pub char_index: Option<u32>,
}

unsafe impl MsgSend for LineFromChar {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINEFROMCHAR.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINEINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/em-lineindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct LineIndex {
	pub line_index: Option<u32>,
}

unsafe impl MsgSend for LineIndex {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINEINDEX.into(),
			wparam: self.line_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINELENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linelength)
/// message parameters.
///
/// Return type: `u32`.
pub struct LineLength {
	pub char_index: Option<u32>,
}

unsafe impl MsgSend for LineLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINELENGTH.into(),
			wparam: self.char_index.unwrap_or(-1i32 as _) as _,
			lparam: 0,
		}
	}
}

/// [`EM_LINESCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-linescroll)
/// message parameters.
///
/// Return type: `bool`.
pub struct LineScroll {
	pub num_chars: u32,
	pub num_lines: u32,
}

unsafe impl MsgSend for LineScroll {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::LINESCROLL.into(),
			wparam: self.num_chars as _,
			lparam: self.num_lines as _,
		}
	}
}

/// [`EM_POSFROMCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-posfromchar)
/// message parameters.
///
/// Return type: `POINT`.
///
/// This message is implemented for ordinary edit controls, not for rich edit.
pub struct PosFromChar {
	pub char_index: u32,
}

unsafe impl MsgSend for PosFromChar {
	type RetType = POINT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		POINT::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::POSFROMCHAR.into(),
			wparam: self.char_index as _,
			lparam: 0,
		}
	}
}

/// [`EM_REPLACESEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-replacesel)
/// message parameters.
///
/// Return type: `()`.
pub struct ReplaceSel {
	pub can_be_undone: bool,
	pub replacement_text: WString,
}

unsafe impl MsgSend for ReplaceSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::REPLACESEL.into(),
			wparam: self.can_be_undone as _,
			lparam: self.replacement_text.as_ptr() as _,
		}
	}
}

/// [`EM_SCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-scroll)
/// message parameters.
///
/// Return type: `SysResult<u16>`.
pub struct Scroll {
	pub action: co::SB_EM,
}

unsafe impl MsgSend for Scroll {
	type RetType = SysResult<u16>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|num_lines| num_lines as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SCROLL.into(),
			wparam: self.action.raw() as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { ScrollCaret: co::EM::SCROLLCARET.into();
	/// [`EM_SCROLLCARET`](https://learn.microsoft.com/en-us/windows/win32/controls/em-scrollcaret)
}

/// [`EM_SETHANDLE`](https://learn.microsoft.com/en-us/windows/win32/controls/em-sethandle)
/// message parameters.
///
/// Return type: `()`.
pub struct SetHandle<'a> {
	pub handle: &'a HLOCAL,
}

unsafe impl<'a> MsgSend for SetHandle<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETHANDLE.into(),
			wparam: self.handle.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETIMESTATUS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setimestatus)
/// message parameters.
///
/// Return type: `co::EIMES`.
pub struct SetImeStatus {
	pub status: co::EIMES,
}

unsafe impl MsgSend for SetImeStatus {
	type RetType = co::EIMES;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::EIMES::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETIMESTATUS.into(),
			wparam: 0x0001, // EMSIS_COMPOSITIONSTRING
			lparam: self.status.raw() as _,
		}
	}
}

/// [`EM_SETLIMITTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setlimittext)
/// message parameters.
///
/// Return type: `()`.
pub struct SetLimitText {
	pub max_chars: Option<u32>,
}

unsafe impl MsgSend for SetLimitText {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETLIMITTEXT.into(),
			wparam: self.max_chars.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETMARGINS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setmargins)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMargins {
	pub margins: co::EC,
	pub size: SIZE,
}

unsafe impl MsgSend for SetMargins {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETMARGINS.into(),
			wparam: self.margins.raw() as _,
			lparam: u32::from(self.size) as _,
		}
	}
}

/// [`EM_SETMODIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setmodify)
/// message parameters.
///
/// Return type: `()`.
pub struct SetModify {
	pub flag: bool,
}

unsafe impl MsgSend for SetModify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETMODIFY.into(),
			wparam: self.flag as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETPASSWORDCHAR`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setpasswordchar)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPasswordChar {
	pub character: Option<char>,
}

unsafe impl MsgSend for SetPasswordChar {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETPASSWORDCHAR.into(),
			wparam: self.character.map(|ch| ch as u32).unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETREADONLY`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setreadonly)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetReadOnly {
	pub read_only: bool,
}

unsafe impl MsgSend for SetReadOnly {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETREADONLY.into(),
			wparam: self.read_only as _,
			lparam: 0,
		}
	}
}

/// [`EM_SETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setrect)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRect<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

unsafe impl<'a> MsgSend for SetRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETRECT.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETRECTNP`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setrectnp)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRectNp<'a> {
	pub is_absolute_coords: bool,
	pub rect: Option<&'a RECT>,
}

unsafe impl<'a> MsgSend for SetRectNp<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETRECTNP.into(),
			wparam: self.is_absolute_coords as _,
			lparam: self.rect.map_or(0, |rect| rect as *const _ as _),
		}
	}
}

/// [`EM_SETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setsel)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSel {
	pub start: i32,
	pub end: i32,
}

unsafe impl MsgSend for SetSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETSEL.into(),
			wparam: self.start as _,
			lparam: self.end as _,
		}
	}
}

/// [`EM_SETTABSTOPS`](https://learn.microsoft.com/en-us/windows/win32/controls/em-settabstops)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTabStops<'a> {
	pub tab_stops: Option<&'a [i32]>,
}

unsafe impl<'a> MsgSend for SetTabStops<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETTABSTOPS.into(),
			wparam: self.tab_stops.map_or(0, |ts| ts.len() as _),
			lparam: self.tab_stops.map_or(0, |ts| vec_ptr(ts) as _),
		}
	}
}

/// [`EM_SETWORDBREAKPROC`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setwordbreakproc)
/// message parameters.
///
/// Return type: `()`.
pub struct SetWordBreakProc {
	pub proc: Option<EDITWORDBREAKPROC>,
}

unsafe impl MsgSend for SetWordBreakProc {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETWORDBREAKPROC.into(),
			wparam: 0,
			lparam: self.proc.map_or(0, |proc| proc as _),
		}
	}
}

/// [`EM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/em-undo)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct Undo {}

unsafe impl MsgSend for Undo {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::UNDO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
