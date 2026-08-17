use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`CB_GETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getcuebanner)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetCueBanner<'a> {
	pub buffer: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetCueBanner<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 | 1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETCUEBANNER.into(),
			wparam: unsafe { self.buffer.as_mut_ptr() } as _,
			lparam: self.buffer.buf_len() as _,
		}
	}
}

/// [`CB_GETMINVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-getminvisible)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMinVisible {}

unsafe impl MsgSend for GetMinVisible {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::GETMINVISIBLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`CB_SETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setcuebanner)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCueBanner {
	pub text: WString,
}

unsafe impl MsgSend for SetCueBanner {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETCUEBANNER.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`CB_SETMINVISIBLE`](https://learn.microsoft.com/en-us/windows/win32/controls/cb-setminvisible)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetMinVisible {
	pub num_items: u32,
}

unsafe impl MsgSend for SetMinVisible {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::CB::SETMINVISIBLE.into(),
			wparam: self.num_items as _,
			lparam: 0,
		}
	}
}
