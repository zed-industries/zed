use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`BCM_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getidealsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`BCM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getimagelist)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetImageList<'a> {
	pub info: &'a mut BUTTON_IMAGELIST,
}

unsafe impl<'a> MsgSend for GetImageList<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`BCM_GETNOTE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getnote)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetNote<'a> {
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetNote<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETNOTE.into(),
			wparam: self.text.buf_len(),
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`BCM_GETNOTELENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getnotelength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetNoteLength {}

unsafe impl MsgSend for GetNoteLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETNOTELENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BCM_GETSPLITINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-getsplitinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetSplitInfo<'a> {
	pub splitinfo: &'a mut BUTTON_SPLITINFO,
}

unsafe impl<'a> MsgSend for GetSplitInfo<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *mut _ as _,
		}
	}
}

/// [`BCM_GETTEXTMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-gettextmargin)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetTextMargin<'a> {
	pub margins: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetTextMargin<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::GETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *mut _ as _,
		}
	}
}

/// [`BCM_SETDROPDOWNSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setdropdownstate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetDropDownState {
	pub is_pushed: bool,
}

unsafe impl MsgSend for SetDropDownState {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETDROPDOWNSTATE.into(),
			wparam: self.is_pushed as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setimagelist)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetImageList<'a> {
	pub info: &'a BUTTON_IMAGELIST,
}

unsafe impl<'a> MsgSend for SetImageList<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`BCM_SETNOTE`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setnote)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetNote {
	pub text: WString,
}

unsafe impl MsgSend for SetNote {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETNOTE.into(),
			wparam: self.text.buf_len(),
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`BCM_SETSHIELD`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setshield)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetShield {
	pub has_elevated_icon: bool,
}

unsafe impl MsgSend for SetShield {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETSHIELD.into(),
			wparam: self.has_elevated_icon as _,
			lparam: 0,
		}
	}
}

/// [`BCM_SETSPLITINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-setsplitinfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetSplitInfo<'a> {
	pub splitinfo: &'a BUTTON_SPLITINFO,
}

unsafe impl<'a> MsgSend for SetSplitInfo<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETSPLITINFO.into(),
			wparam: 0,
			lparam: self.splitinfo as *const _ as _,
		}
	}
}

/// [`BCM_SETTEXTMARGIN`](https://learn.microsoft.com/en-us/windows/win32/controls/bcm-settextmargin)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetTextMargin<'a> {
	pub margins: &'a RECT,
}

unsafe impl<'a> MsgSend for SetTextMargin<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BCM::SETTEXTMARGIN.into(),
			wparam: 0,
			lparam: self.margins as *const _ as _,
		}
	}
}
