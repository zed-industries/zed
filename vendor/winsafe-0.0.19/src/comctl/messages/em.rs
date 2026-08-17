use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`EM_GETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/em-getcuebanner)
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
			msg_id: co::EM::GETCUEBANNER.into(),
			wparam: unsafe { self.buffer.as_mut_ptr() } as _,
			lparam: self.buffer.buf_len() as _,
		}
	}
}

/// [`EM_HIDEBALLOONTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/em-hideballoontip)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct HideBalloonTip {}

unsafe impl MsgSend for HideBalloonTip {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::HIDEBALLOONTIP.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_SETCUEBANNER`](https://learn.microsoft.com/en-us/windows/win32/controls/em-setcuebanner)
/// message parameters..
///
/// Return type: `SysResult<()>`.
pub struct SetCueBanner {
	pub show_even_with_focus: bool,
	pub text: WString,
}

unsafe impl MsgSend for SetCueBanner {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETCUEBANNER.into(),
			wparam: self.show_even_with_focus as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`EM_SHOWBALLOONTIP`](https://learn.microsoft.com/en-us/windows/win32/controls/em-showballoontip)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct ShowBalloonTip<'a, 'b, 'c> {
	pub info: &'c EDITBALLOONTIP<'a, 'b>,
}

unsafe impl<'a, 'b, 'c> MsgSend for ShowBalloonTip<'a, 'b, 'c> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SHOWBALLOONTIP.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}
