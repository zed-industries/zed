use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`UDM_GETACCEL`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getaccel)
/// message parameters.
///
/// Return type: `u32`.
pub struct GetAccel<'a> {
	pub info: &'a mut [UDACCEL],
}

unsafe impl<'a> MsgSend for GetAccel<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETACCEL.into(),
			wparam: self.info.len(),
			lparam: self.info.as_mut_ptr() as _,
		}
	}
}

/// [`UDM_GETBASE`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getbase)
/// message, which has no parameters.
///
/// Return type: `u8`.
pub struct GetBase {}

unsafe impl MsgSend for GetBase {
	type RetType = u8;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETBASE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`UDM_GETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getbuddy)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetBuddy {}

unsafe impl MsgSend for GetBuddy {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETBUDDY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`UDM_GETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getpos)
/// message, which has no parameters.
///
/// Return type: `SysResult<i16>`.
pub struct GetPos {}

unsafe impl MsgSend for GetPos {
	type RetType = SysResult<i16>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match HIWORD(v as _) {
			0 => Ok(LOWORD(v as _) as _),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`UDM_GETPOS32`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getpos32)
/// message parameters.
///
/// Return type: `i32`.
pub struct GetPos32<'a> {
	pub success_flag: Option<&'a mut i32>,
}

unsafe impl<'a> MsgSend for GetPos32<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETPOS32.into(),
			wparam: 0,
			lparam: self.success_flag.as_mut().map_or(0, |f| f as *mut _ as _),
		}
	}
}

/// [`UDM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getrange)
/// message, which has no parameters.
///
/// Return type: `(i16, i16)`.
pub struct GetRange {}

unsafe impl MsgSend for GetRange {
	type RetType = (i16, i16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETRANGE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`UDM_GETRANGE32`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getrange32)
/// message parameters.
///
/// Return type: `()`.
pub struct GetRange32<'a, 'b> {
	pub min: &'a mut i32,
	pub max: &'b mut i32,
}

unsafe impl<'a, 'b> MsgSend for GetRange32<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::GETRANGE32.into(),
			wparam: self.min as *mut _ as  _,
			lparam: self.max as *mut _ as  _,
		}
	}
}

/// [`UDM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-getunicodeformat)
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
			msg_id: co::UDM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`UDM_SETACCEL`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setaccel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetAccel<'a> {
	pub info: &'a [UDACCEL],
}

unsafe impl<'a> MsgSend for SetAccel<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETACCEL.into(),
			wparam: self.info.len(),
			lparam: vec_ptr(self.info) as _,
		}
	}
}

/// [`UDM_SETBASE`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setbase)
/// message parameters.
///
/// Return type: `SysResult<u8>`.
pub struct SetBase {
	pub base: u8,
}

unsafe impl MsgSend for SetBase {
	type RetType = SysResult<u8>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETBASE.into(),
			wparam: self.base as _,
			lparam: 0,
		}
	}
}

/// [`UDM_SETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct SetBuddy<'a> {
	pub hbuddy: &'a HWND,
}

unsafe impl<'a> MsgSend for SetBuddy<'a> {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETBUDDY.into(),
			wparam: self.hbuddy.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`UDM_SETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setpos)
/// message parameters.
///
/// Return type: `i16`.
pub struct SetPos {
	pub pos: i16,
}

unsafe impl MsgSend for SetPos {
	type RetType = i16;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETPOS.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`UDM_SETPOS32`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setpos32)
/// message parameters.
///
/// Return type: `i32`.
pub struct SetPos32 {
	pub pos: i32,
}

unsafe impl MsgSend for SetPos32 {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETPOS32.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`UDM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setrange)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRange {
	pub min: i16,
	pub max: i16,
}

unsafe impl MsgSend for SetRange {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETRANGE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.max as _, self.min as _) as _,
		}
	}
}

/// [`UDM_SETRANGE32`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setrange32)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRange32 {
	pub min: i32,
	pub max: i32,
}

unsafe impl MsgSend for SetRange32 {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::UDM::SETRANGE32.into(),
			wparam: self.min as _,
			lparam: self.max as _,
		}
	}
}

/// [`UDM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/udm-setunicodeformat)
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
			msg_id: co::UDM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
