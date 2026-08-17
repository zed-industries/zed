use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`MCM_GETCALENDARBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarborder)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCalendarBorder {}

unsafe impl MsgSend for GetCalendarBorder {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARBORDER.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendarcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCalendarCount {}

unsafe impl MsgSend for GetCalendarCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCALENDARGRIDINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalendargridinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetCalendarGridInfo<'a, 'b> {
	pub info: &'b mut MCGRIDINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetCalendarGridInfo<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALENDARGRIDINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETCALID`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalid)
/// message, which has no parameters.
///
/// Return type: `co::CAL`.
pub struct GetCalId {}

unsafe impl MsgSend for GetCalId {
	type RetType = co::CAL;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::CAL::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCALID.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcolor)
/// message parameters.
///
/// Return type: `SysResult<COLORREF>`.
pub struct GetColor {
	pub which: co::MCSC,
}

unsafe impl MsgSend for GetColor {
	type RetType = SysResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCOLOR.into(),
			wparam: self.which.raw() as _,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCURRENTVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcurrentview)
/// message, which has no parameters.
///
/// Return type: `co::MCMV`.
pub struct GetCurrentView {}

unsafe impl MsgSend for GetCurrentView {
	type RetType = co::MCMV;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::MCMV::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCURRENTVIEW.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetCurSel<'a> {
	pub info: &'a mut SYSTEMTIME,
}

unsafe impl<'a> MsgSend for GetCurSel<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETFIRSTDAYOFWEEK`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getfirstdayofweek)
/// message, which has no parameters.
///
/// Return type: `(bool, u16)`.
pub struct GetFirstDayOfWeek {}

unsafe impl MsgSend for GetFirstDayOfWeek {
	type RetType = (bool, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(HIWORD(v as _) != 0, LOWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETFIRSTDAYOFWEEK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXSELCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmaxselcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMaxSelCount {}

unsafe impl MsgSend for GetMaxSelCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMAXSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMAXTODAYWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmaxtodaywidth)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMaxTodayWidth {}

unsafe impl MsgSend for GetMaxTodayWidth {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMAXTODAYWIDTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMINREQRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getminreqrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetMinReqRect<'a> {
	pub bounds_rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetMinReqRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMINREQRECT.into(),
			wparam: 0,
			lparam: self.bounds_rect as *mut _ as _,
		}
	}
}

/// [`MCM_GETMONTHDELTA`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmonthdelta)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetMonthDelta {}

unsafe impl MsgSend for GetMonthDelta {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMONTHDELTA.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_GETMONTHRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getmonthrange)
/// message parameters.
///
/// Return value: `u32`.
pub struct GetMonthRange<'a> {
	pub scope: co::GMR,
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetMonthRange<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETMONTHRANGE.into(),
			wparam: self.scope.raw() as _,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
pub struct GetRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::GDTR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETSELRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getselrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetSelRange<'a> {
	pub limits: &'a mut [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for GetSelRange<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETSELRANGE.into(),
			wparam: 0,
			lparam: self.limits.as_mut_ptr() as _,
		}
	}
}

/// [`MCM_GETTODAY`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-gettoday)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetToday<'a> {
	pub info: &'a mut SYSTEMTIME,
}

unsafe impl<'a> MsgSend for GetToday<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::GETTODAY.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`MCM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getunicodeformat)
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
			msg_id: co::MCM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`MCM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-hittest)
/// message parameters.
///
/// Return type: `()`.
pub struct HitTest<'a> {
	pub test_info: &'a mut MCHITTESTINFO,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::HITTEST.into(),
			wparam: 0,
			lparam: self.test_info as *mut _ as _,
		}
	}
}

/// [`MCM_SETCALENDARBORDER`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcalendarborder)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCalendarBorder {
	pub border: bool,
	pub pixels: u32,
}

unsafe impl MsgSend for SetCalendarBorder {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCALENDARBORDER.into(),
			wparam: self.border as _,
			lparam: self.pixels as _,
		}
	}
}

/// [`MCM_SETCALID`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcalid)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCalId {
	pub id: co::CAL,
}

unsafe impl MsgSend for SetCalId {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCALID.into(),
			wparam: self.id.raw() as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct SetColor {
	pub which: co::MCSC,
	pub color: COLORREF,
}

unsafe impl MsgSend for SetColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| unsafe { COLORREF::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCOLOR.into(),
			wparam: self.which.raw() as _,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`MCM_SETCURRENTVIEW`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcurrentview)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCurrentView {
	pub view: co::MCMV,
}

unsafe impl MsgSend for SetCurrentView {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCURRENTVIEW.into(),
			wparam: 0,
			lparam: self.view.raw() as _,
		}
	}
}

/// [`MCM_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCurSel<'a> {
	pub info: &'a SYSTEMTIME,
}

unsafe impl<'a> MsgSend for SetCurSel<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETCURSEL.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`MCM_SETDAYSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setdaystate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetDayState<'a> {
	pub months: &'a [MONTHDAYSTATE],
}

unsafe impl<'a> MsgSend for SetDayState<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETDAYSTATE.into(),
			wparam: self.months.len(),
			lparam: vec_ptr(self.months) as _,
		}
	}
}

/// [`MCM_SETFIRSTDAYOFWEEK`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setfirstdayofweek)
/// message parameters.
///
/// Return type: `(bool, u16)`.
pub struct SetFirstDayOfWeek {
	pub first_day: u8,
}

unsafe impl MsgSend for SetFirstDayOfWeek {
	type RetType = (bool, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(HIWORD(v as _) != 0, LOWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETFIRSTDAYOFWEEK.into(),
			wparam: 0,
			lparam: self.first_day as _,
		}
	}
}

/// [`MCM_SETMAXSELCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setmaxselcount)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetMaxSelCount {
	pub max_days: u8,
}

unsafe impl MsgSend for SetMaxSelCount {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETMAXSELCOUNT.into(),
			wparam: self.max_days as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETMONTHDELTA`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setmonthdelta)
/// message parameters.
///
/// Return type: `u8`.
pub struct SetMonthDelta {
	pub num_months: u8,
}

unsafe impl MsgSend for SetMonthDelta {
	type RetType = u8;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETMONTHDELTA.into(),
			wparam: self.num_months as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetRange<'a> {
	pub which: co::GDTR,
	pub limits: &'a [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for SetRange<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETRANGE.into(),
			wparam: self.which.raw() as _,
			lparam: vec_ptr(self.limits) as _,
		}
	}
}

/// [`MCM_SETSELRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setselrange)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetSelRange<'a> {
	pub limits: &'a [SYSTEMTIME; 2],
}

unsafe impl<'a> MsgSend for SetSelRange<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETSELRANGE.into(),
			wparam: 0,
			lparam: vec_ptr(self.limits) as _,
		}
	}
}

/// [`MCM_SETTODAY`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-settoday)
/// message parameters.
///
/// Return type: `()`.
pub struct SetToday<'a> {
	pub today: Option<&'a SYSTEMTIME>,
}

unsafe impl<'a> MsgSend for SetToday<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SETTODAY.into(),
			wparam: 0,
			lparam: self.today.map_or(0, |today| today as *const _ as _),
		}
	}
}

/// [`MCM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-setunicodeformat)
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
			msg_id: co::MCM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}

/// [`MCM_SIZERECTTOMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-sizerecttomin)
/// message parameters.
///
/// Return type: `()`.
pub struct SizeRectToMin<'a> {
	pub region: &'a mut RECT,
}

unsafe impl<'a> MsgSend for SizeRectToMin<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::MCM::SIZERECTTOMIN.into(),
			wparam: 0,
			lparam: self.region as *mut _ as _,
		}
	}
}
