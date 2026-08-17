use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`PBM_DELTAPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-deltapos)
/// message parameters.
///
/// Return type: `u32`.
pub struct DeltaPos {
	pub advance_amount: u32,
}

unsafe impl MsgSend for DeltaPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::DELTAPOS.into(),
			wparam: self.advance_amount as _,
			lparam: 0,
		}
	}
}

/// [`PBM_GETBARCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getbarcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct GetBarColor {}

unsafe impl MsgSend for GetBarColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(unsafe { COLORREF::from_raw(v) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETBARCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getbkcolor)
/// message, which has no parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct GetBkColor {}

unsafe impl MsgSend for GetBkColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(unsafe { COLORREF::from_raw(v) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETBKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getpos)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetPos {}

unsafe impl MsgSend for GetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getrange)
/// message parameters.
///
/// Return type: `i32`.
pub struct GetRange<'a> {
	pub return_low: bool,
	pub ranges: Option<&'a mut PBRANGE>,
}

unsafe impl<'a> MsgSend for GetRange<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETRANGE.into(),
			wparam: self.return_low as _,
			lparam: self.ranges.as_mut().map_or(0, |r| r as *mut _ as _),
		}
	}
}

/// [`PBM_GETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getstate)
/// message, which has no parameters.
///
/// Return type: `co::PBST`.
pub struct GetState {}

unsafe impl MsgSend for GetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::PBST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_GETSTEP`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-getstep)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetStep {}

unsafe impl MsgSend for GetStep {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTEP.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PBM_SETBARCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setbarcolor)
/// message parameters.
///
/// Return type: `Option<COLORREF>`.
pub struct SetBarColor {
	pub color: Option<COLORREF>,
}

unsafe impl MsgSend for SetBarColor {
	type RetType = Option<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as u32 {
			CLR_DEFAULT => None,
			v => Some(unsafe { COLORREF::from_raw(v) }),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETBARCOLOR.into(),
			wparam: self.color.map_or(CLR_DEFAULT, |color| color.into()) as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETBKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setbkcolor)
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
			msg_id: co::PBM::SETBKCOLOR.into(),
			wparam: self.color.map_or(CLR_DEFAULT, |color| color.into()) as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETMARQUEE`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setmarquee)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMarquee {
	pub turn_on: bool,
	pub time_ms: Option<u32>,
}

unsafe impl MsgSend for SetMarquee {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETMARQUEE.into(),
			wparam: self.turn_on as _,
			lparam: self.time_ms.unwrap_or(0) as _,
		}
	}
}

/// [`PBM_SETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setpos)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetPos {
	pub position: u32,
}

unsafe impl MsgSend for SetPos {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETPOS.into(),
			wparam: self.position as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setrange)
/// message parameters.
///
/// Return type: `SysResult<(u16, u16)>`.
pub struct SetRange {
	pub min: u16,
	pub max: u16,
}

unsafe impl MsgSend for SetRange {
	type RetType = SysResult<(u16, u16)>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| (LOWORD(v as _), HIWORD(v as _)))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETRANGE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.min, self.max) as _,
		}
	}
}

/// [`PBM_SETRANGE32`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setrange32)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRange32 {
	pub min: u32,
	pub max: u32,
}

unsafe impl MsgSend for SetRange32 {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETRANGE32.into(),
			wparam: self.min as _,
			lparam: self.max as _,
		}
	}
}

/// [`PBM_SETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setstate)
/// message parameters.
///
/// Return type: `co::PBST`.
pub struct SetState {
	pub state: co::PBST,
}

unsafe impl MsgSend for SetState {
	type RetType = co::PBST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::PBST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTATE.into(),
			wparam: self.state.raw() as _,
			lparam: 0,
		}
	}
}

/// [`PBM_SETSTEP`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setstep)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetStep {
	pub step: u32,
}

unsafe impl MsgSend for SetStep {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::SETSTEP.into(),
			wparam: self.step as _,
			lparam: 0,
		}
	}
}

/// [`PBM_STEPIT`](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-stepit)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct StepIt {}

unsafe impl MsgSend for StepIt {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PBM::STEPIT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
