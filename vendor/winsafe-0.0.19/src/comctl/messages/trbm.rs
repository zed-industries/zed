use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TBM_CLEARSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-clearsel)
/// message parameters.
///
/// Return type: `()`.
pub struct ClearSel {
	pub redraw: bool,
}

unsafe impl MsgSend for ClearSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::CLEARSEL.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_CLEARTICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-cleartics)
/// message parameters.
///
/// Return type: `()`.
pub struct ClearTics {
	pub redraw: bool,
}

unsafe impl MsgSend for ClearTics {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::CLEARTICS.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetBuddy {
	pub left_above: bool,
}

unsafe impl MsgSend for GetBuddy {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETCHANNELRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getchannelrect)
/// message parameters.
///
/// Return type: `()`.
pub struct GetChannelRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetChannelRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETCHANNELRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TBM_GETLINESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getlinesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetLineSize {}

unsafe impl MsgSend for GetLineSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETLINESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETNUMTICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getnumtics)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetNumTics {}

unsafe impl MsgSend for GetNumTics {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETNUMTICS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPAGESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getpagesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetPageSize {}

unsafe impl MsgSend for GetPageSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETPAGESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getpos)
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
			msg_id: co::TRBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMAX`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getrangemax)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRangeMax {}

unsafe impl MsgSend for GetRangeMax {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETRANGEMAX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getrangemin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRangeMin {}

unsafe impl MsgSend for GetRangeMin {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETRANGEMIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELEND`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getselend)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetSelEnd {}

unsafe impl MsgSend for GetSelEnd {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETSELEND.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELSTART`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getselstart)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetSelStart {}

unsafe impl MsgSend for GetSelStart {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETSELSTART.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBLENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getthumblength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetThumbLength {}

unsafe impl MsgSend for GetThumbLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTHUMBLENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getthumbrect)
/// message parameters.
///
/// Return type: `()`.
pub struct GetThumbRect<'a> {
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetThumbRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTHUMBRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TBM_GETTIC`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-gettic)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetTic {
	pub index: u32,
}

unsafe impl MsgSend for GetTic {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTIC.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTICPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getticpos)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetTicPos {
	pub index: u32,
}

unsafe impl MsgSend for GetTicPos {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTICPOS.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getunicodeformat)
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
			msg_id: co::TBM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_SETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct SetBuddy<'a> {
	pub left_above: bool,
	pub hwnd_buddy: &'a HWND,
}

unsafe impl<'a> MsgSend for SetBuddy<'a> {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: self.hwnd_buddy.ptr() as _,
		}
	}
}

/// [`TBM_SETLINESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setlinesize)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetLineSize {
	pub size: u32,
}

unsafe impl MsgSend for SetLineSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETLINESIZE.into(),
			wparam: 0,
			lparam: self.size as _,
		}
	}
}

/// [`TBM_SETPAGSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setpagesize)
/// message parameters.
///
/// Return type: `u32`.
pub struct SetPageSize {
	pub page_size: u32,
}

unsafe impl MsgSend for SetPageSize {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPAGESIZE.into(),
			wparam: 0,
			lparam: self.page_size as _,
		}
	}
}

/// [`TBM_SETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setpos)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPos {
	pub redraw: bool,
	pub pos: u32,
}

unsafe impl MsgSend for SetPos {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPOS.into(),
			wparam: self.redraw as _,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETPOSNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setposnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct SetPosNotify {
	pub pos: u32,
}

unsafe impl MsgSend for SetPosNotify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETPOSNOTIFY.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrange)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRange {
	pub redraw: bool,
	pub min_pos: u16,
	pub max_pos: u16,
}

unsafe impl MsgSend for SetRange {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETRANGE.into(),
			wparam: self.redraw as _,
			lparam: MAKEDWORD(self.min_pos, self.max_pos) as _,
		}
	}
}

/// [`TBM_SETRANGEMAX`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrangemax)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRangeMax {
	pub redraw: bool,
	pub max: u32,
}

unsafe impl MsgSend for SetRangeMax {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETRANGEMAX.into(),
			wparam: self.redraw as _,
			lparam: self.max as _,
		}
	}
}

/// [`TBM_SETRANGEMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrangemin)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRangeMin {
	pub redraw: bool,
	pub min: u32,
}

unsafe impl MsgSend for SetRangeMin {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETRANGEMIN.into(),
			wparam: self.redraw as _,
			lparam: self.min as _,
		}
	}
}

/// [`TBM_SETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setsel)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSel {
	pub redraw: bool,
	pub start_pos: u16,
	pub end_pos: u16,
}

unsafe impl MsgSend for SetSel {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETSEL.into(),
			wparam: self.redraw as _,
			lparam: MAKEDWORD(self.start_pos, self.end_pos) as _,
		}
	}
}

/// [`TBM_SETSELEND`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setselend)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSelEnd {
	pub redraw: bool,
	pub end: u32,
}

unsafe impl MsgSend for SetSelEnd {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETSELEND.into(),
			wparam: self.redraw as _,
			lparam: self.end as _,
		}
	}
}

/// [`TBM_SETSELSTART`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setselstart)
/// message parameters.
///
/// Return type: `()`.
pub struct SetSelStart {
	pub redraw: bool,
	pub start: u32,
}

unsafe impl MsgSend for SetSelStart {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETSELSTART.into(),
			wparam: self.redraw as _,
			lparam: self.start as _,
		}
	}
}

/// [`TBM_SETTHUMBLENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setthumblength)
/// message parameters.
///
/// Return type: `()`.
pub struct SetThumbLength {
	pub length: u32,
}

unsafe impl MsgSend for SetThumbLength {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTHUMBLENGTH.into(),
			wparam: self.length as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTIC`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settic)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTic {
	pub pos: u32,
}

unsafe impl MsgSend for SetTic {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTIC.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETTICFREQ`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setticfreq)
/// message parameters.
///
/// Return type: `()`.
pub struct SetTicFreq {
	pub freq: u32,
}

unsafe impl MsgSend for SetTicFreq {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTICFREQ.into(),
			wparam: self.freq as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTIPSIDE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settipside)
/// message parameters.
///
/// Return type: `co::TBTS`.
pub struct SetTipSide {
	pub location: co::TBTS,
}

unsafe impl MsgSend for SetTipSide {
	type RetType = co::TBTS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TBTS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTIPSIDE.into(),
			wparam: self.location.raw() as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settooltips)
/// message parameters.
///
/// Return type: `()`.
pub struct SetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

unsafe impl<'a> MsgSend for SetTooltips<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TRBM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TBM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setunicodeformat)
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
			msg_id: co::TBM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
