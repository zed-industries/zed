use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

pub_struct_msg_empty! { CloseMonthCal: co::DTM::CLOSEMONTHCAL.into();
	/// [`DTM_CLOSEMONTHCAL`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-closemonthcal)
}

/// [`DTM_GETDATETIMEPICKERINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getdatetimepickerinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetDateTimePickerInfo<'a> {
	pub info: &'a mut DATETIMEPICKERINFO,
}

unsafe impl<'a> MsgSend for GetDateTimePickerInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETDATETIMEPICKERINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`DTM_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getidealsize)
/// message parameters.
///
/// Return type: `()`.
pub struct GetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`DTM_GETMCCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct GetMcColor {
	pub color_index: co::MCSC,
}

unsafe impl MsgSend for GetMcColor {
	type RetType = SysResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCCOLOR.into(),
			wparam: self.color_index.raw() as _,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMCSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmcstyle)
/// message, which has no parameters.
///
/// Return type: `SysResult<co::MCS>`.
pub struct GetMcStyle {}

unsafe impl MsgSend for GetMcStyle {
	type RetType = SysResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { co::MCS::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMONTHCAL`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getmonthcal)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct GetMonthCal {}

unsafe impl MsgSend for GetMonthCal {
	type RetType = SysResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMONTHCAL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
pub struct GetRange<'a> {
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::GDTR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETRANGE.into(),
			wparam: 0,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_GETSYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-getsystemtime)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetSystemTime<'a> {
	pub system_time: &'a mut SYSTEMTIME,
}

unsafe impl<'a> MsgSend for GetSystemTime<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const GDT_NONE: i32 = co::GDT::NONE.raw() as _;
		match v as i32 {
			GDT_ERROR => Err(co::ERROR::BAD_ARGUMENTS),
			GDT_NONE => Err(co::ERROR::INVALID_DATA),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETSYSTEMTIME.into(),
			wparam: 0,
			lparam: self.system_time as *mut _ as _,
		}
	}
}

/// [`DTM_SETFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setformat)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetFormat {
	pub format_string: Option<WString>,
}

unsafe impl MsgSend for SetFormat {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETFORMAT.into(),
			wparam: 0,
			lparam: self.format_string.as_ref().map_or(0, |ws| ws.as_ptr() as _),
		}
	}
}

/// [`DTM_SETMCCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setmccolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct SetMcColor {
	pub color_index: co::MCSC,
	pub color: COLORREF,
}

unsafe impl MsgSend for SetMcColor {
	type RetType = SysResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCCOLOR.into(),
			wparam: self.color_index.raw() as _,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`DTM_SETMCSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setmcstyle)
/// message parameters.
///
/// Return type: `SysResult<co::MCS>`.
pub struct SetMcStyle {
	pub style: co::MCS,
}

unsafe impl MsgSend for SetMcStyle {
	type RetType = SysResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| unsafe { co::MCS::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCSTYLE.into(),
			wparam: 0,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`DTM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetRange<'a> {
	pub valid: co::GDTR,
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for SetRange<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETRANGE.into(),
			wparam: self.valid.raw() as _,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_SETSYSTEMTIME`](https://learn.microsoft.com/en-us/windows/win32/controls/dtm-setsystemtime)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetSystemTime<'a> {
	pub system_time: Option<&'a SYSTEMTIME>,
}

unsafe impl<'a> MsgSend for SetSystemTime<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETSYSTEMTIME.into(),
			wparam: self.system_time.as_ref().map_or(co::GDT::NONE.raw(), |_| co::GDT::VALID.raw()) as _,
			lparam: self.system_time.as_ref().map_or(0, |st| st as *const _ as _),
		}
	}
}
