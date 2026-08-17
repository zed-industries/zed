use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`WM_ACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
///
/// Return type: `()`.
pub struct Activate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

unsafe impl MsgSend for Activate {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.raw(), self.is_minimized as _) as _,
			lparam: self.hwnd.ptr() as _,
		}
	}
}

unsafe impl MsgSendRecv for Activate {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: unsafe { co::WA::from_raw(LOWORD(p.wparam as _)) },
			is_minimized: HIWORD(p.wparam as _) != 0,
			hwnd: unsafe { HWND::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_ACTIVATEAPP`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
pub struct ActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

unsafe impl MsgSend for ActivateApp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as _,
			lparam: self.thread_id as _,
		}
	}
}

unsafe impl MsgSendRecv for ActivateApp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as _,
		}
	}
}

/// [`WM_APPCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
/// message parameters.
///
/// Return type: `()`.
pub struct AppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

unsafe impl MsgSend for AppCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.ptr() as _,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.raw() | self.u_device.raw()) as _,
		}
	}
}

unsafe impl MsgSendRecv for AppCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				hwnd_owner: HWND::from_ptr(p.wparam as _),
				app_command: co::APPCOMMAND::from_raw(HIWORD(p.lparam as _) & !FAPPCOMMAND_MASK),
				u_device: co::FAPPCOMMAND::from_raw(HIWORD(p.lparam as _) & FAPPCOMMAND_MASK),
				keys: co::MK::from_raw(LOWORD(p.lparam as _)),
			}
		}
	}
}

pub_struct_msg_empty_handleable! { CancelMode: co::WM::CANCELMODE;
	/// [`WM_CANCELMODE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
}

/// [`WM_CAPTURECHANGED`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct CaptureChanged {
	pub hwnd_gaining_mouse: HWND,
}

unsafe impl MsgSend for CaptureChanged {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CAPTURECHANGED,
			wparam: self.hwnd_gaining_mouse.ptr() as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for CaptureChanged {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_gaining_mouse: unsafe { HWND::from_ptr(p.wparam as _) },
		}
	}
}

pub_struct_msg_char_code! { Char: co::WM::CHAR;
	/// [`WM_CHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-char)
}

pub_struct_msg_empty_handleable! { ChildActivate: co::WM::CHILDACTIVATE;
	/// [`WM_CHILDACTIVATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
}

pub_struct_msg_empty_handleable! { Close: co::WM::CLOSE;
	/// [`WM_CLOSE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close)
}

/// [`WM_COMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
pub struct Command {
	pub event: AccelMenuCtrl,
}

unsafe impl MsgSend for Command {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::COMMAND,
			wparam: match &self.event {
				AccelMenuCtrl::Accel(id) => MAKEDWORD(*id, 1) as _,
				AccelMenuCtrl::Menu(id) => MAKEDWORD(*id, 0) as _,
				AccelMenuCtrl::Ctrl(data) => MAKEDWORD(data.ctrl_id, data.notif_code.raw()) as _,
			},
			lparam: match &self.event {
				AccelMenuCtrl::Accel(_) => co::CMD::Accelerator.raw() as _,
				AccelMenuCtrl::Menu(_) => co::CMD::Menu.raw() as _,
				AccelMenuCtrl::Ctrl(data) => data.ctrl_hwnd.ptr() as _,
			},
		}
	}
}

unsafe impl MsgSendRecv for Command {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: match HIWORD(p.wparam as _) {
				1 => AccelMenuCtrl::Accel(LOWORD(p.wparam as _)),
				0 => AccelMenuCtrl::Menu(LOWORD(p.wparam as _)),
				code => AccelMenuCtrl::Ctrl(
					AccelMenuCtrlData {
						notif_code: unsafe { co::CMD::from_raw(code) },
						ctrl_id: LOWORD(p.wparam as _),
						ctrl_hwnd: unsafe { HWND::from_ptr(p.lparam as _) },
					},
				),
			},
		}
	}
}

/// [`WM_CONTEXTMENU`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
pub struct ContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

unsafe impl MsgSend for ContextMenu {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.ptr() as _,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

unsafe impl MsgSendRecv for ContextMenu {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: unsafe { HWND::from_ptr(p.wparam as _) },
			cursor_pos: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_CREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
pub struct Create<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

unsafe impl<'a, 'b, 'c> MsgSend for Create<'a, 'b, 'c> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

unsafe impl<'a, 'b, 'c> MsgSendRecv for Create<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
/// message parameters.
///
/// Return type: `()`.
pub struct DeleteItem<'a> {
	pub control_id: u16,
	pub deleteitemstruct: &'a DELETEITEMSTRUCT,
}

unsafe impl<'a> MsgSend for DeleteItem<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DELETEITEM,
			wparam: self.control_id as _,
			lparam: self.deleteitemstruct as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for DeleteItem<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			control_id: p.wparam as _,
			deleteitemstruct: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

pub_struct_msg_empty_handleable! { Destroy: co::WM::DESTROY;
	/// [`WM_DESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
}

/// [`WM_DEVICECHANGE`](https://learn.microsoft.com/en-us/windows/win32/devio/wm-devicechange)
/// message parameters.
///
/// Return type: `()`.
pub struct DeviceChange<'a> {
	pub event: co::DBT,
	pub data: Option<&'a DEV_BROADCAST_HDR>,
}

unsafe impl<'a> MsgSend for DeviceChange<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DEVICECHANGE,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

unsafe impl<'a> MsgSendRecv for DeviceChange<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: unsafe { co::DBT::from_raw(p.wparam as _) },
			data: if p.lparam == 0 {
				None
			} else {
				Some(unsafe { &*(p.lparam as *const _) })
			}
		}
	}
}

/// [`WM_ENABLE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
pub struct Enable {
	pub has_been_enabled: bool,
}

unsafe impl MsgSend for Enable {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for Enable {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			has_been_enabled: p.wparam != 0,
		}
	}
}

/// [`WM_ENDSESSION`](https://learn.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
pub struct EndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

unsafe impl MsgSend for EndSession {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as _,
			lparam: self.event.raw() as _,
		}
	}
}

unsafe impl MsgSendRecv for EndSession {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: unsafe { co::ENDSESSION::from_raw(p.lparam as _) },
		}
	}
}

/// [`WM_ENTERIDLE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

unsafe impl MsgSend for EnterIdle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.raw() as _,
			lparam: self.handle.as_isize(),
		}
	}
}

unsafe impl MsgSendRecv for EnterIdle {
	fn from_generic_wm(p: WndMsg) -> Self {
		let reason = unsafe { co::MSGF::from_raw(p.wparam as _) };
		Self {
			reason,
			handle: match reason {
				co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(unsafe { HWND::from_ptr(p.lparam as _) }),
				_ => HwndHmenu::Hmenu(unsafe { HMENU::from_ptr(p.lparam as _) }),
			},
		}
	}
}

/// [`WM_ENTERMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct EnterMenuLoop {
	pub with_trackpopupmenu: bool,
}

unsafe impl MsgSend for EnterMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERMENULOOP,
			wparam: self.with_trackpopupmenu as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for EnterMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			with_trackpopupmenu: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { EnterSizeMove: co::WM::ENTERSIZEMOVE;
	/// [`WM_ENTERSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
}

/// [`WM_ERASEBKGND`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
pub struct EraseBkgnd {
	pub hdc: HDC,
}

unsafe impl MsgSend for EraseBkgnd {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.ptr() as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for EraseBkgnd {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdc: unsafe { HDC::from_ptr(p.wparam as _) },
		}
	}
}

/// [`WM_EXITMENULOOP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
/// message parameters.
///
/// Return type: `()`.
pub struct ExitMenuLoop {
	pub is_shortcut: bool,
}

unsafe impl MsgSend for ExitMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::EXITMENULOOP,
			wparam: self.is_shortcut as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for ExitMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_shortcut: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { ExitSizeMove: co::WM::EXITSIZEMOVE;
	/// [`WM_EXITSIZEMOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
}

/// [`WM_GETDLGCODE`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
/// message parameters.
///
/// Return type: `co::DLGC`.
pub struct GetDlgCode<'a> {
	pub vkey_code: co::VK,
	pub msg: Option<&'a mut MSG>,
	pub is_query: bool,
}

unsafe impl<'a> MsgSend for GetDlgCode<'a> {
	type RetType = co::DLGC;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::DLGC::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETDLGCODE,
			wparam: self.vkey_code.raw() as _,
			lparam: self.msg.as_mut().map_or(0, |m| m as *mut _ as _),
		}
	}
}

unsafe impl<'a> MsgSendRecv for GetDlgCode<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			vkey_code: unsafe { co::VK::from_raw(p.wparam as _) },
			msg: match p.lparam {
				0 => None,
				ptr => Some(unsafe { &mut *(ptr as *mut _) })
			},
			is_query: p.lparam == 0,
		}
	}
}

/// [`WM_GETHMENU`](https://learn.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
/// message, which has no parameters. Originally has `MN` prefix.
///
/// Return type: `Option<HMENU>`.
pub struct GetHMenu {}

unsafe impl MsgSend for GetHMenu {
	type RetType = Option<HMENU>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HMENU::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MN_GETHMENU,
			wparam: 0,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for GetHMenu {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETMINMAXINFO`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
pub struct GetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

unsafe impl<'a> MsgSend for GetMinMaxInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for GetMinMaxInfo<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_GETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettext)
/// message parameters.
///
/// Return type: `u32`.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, msg};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let needed_len = hwnd.SendMessage(msg::wm::GetTextLength {});
/// let mut buf = w::WString::new_alloc_buf(needed_len as _);
///
/// hwnd.SendMessage(
///     msg::wm::GetText {
///         buffer: buf.as_mut_slice(),
///     },
/// );
///
/// println!("Text: {}", buf.to_string());
/// ```
pub struct GetText<'a> {
	pub buffer: &'a mut [u16], // can't be WString because this message can be received
}

unsafe impl<'a> MsgSend for GetText<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXT,
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for GetText<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			buffer: unsafe { std::slice::from_raw_parts_mut(p.lparam as _, p.wparam) },
		}
	}
}

/// [`WM_GETTEXTLENGTH`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetTextLength {}

unsafe impl MsgSend for GetTextLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXTLENGTH,
			wparam: 0,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for GetTextLength {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETTITLEBARINFOEX`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
/// message parameters.
///
/// Return type: `()`.
pub struct GetTitleBarInfoEx<'a> {
	pub info: &'a mut TITLEBARINFOEX,
}

unsafe impl<'a> MsgSend for GetTitleBarInfoEx<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTITLEBARINFOEX,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for GetTitleBarInfoEx<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HELP`](https://learn.microsoft.com/en-us/windows/win32/shell/wm-help)
/// message parameters.
///
/// Return type: `()`.
pub struct Help<'a> {
	pub helpinfo: &'a HELPINFO,
}

unsafe impl<'a> MsgSend for Help<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HELP,
			wparam: 0,
			lparam: self.helpinfo as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for Help<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			helpinfo: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct HScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

unsafe impl MsgSend for HScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

unsafe impl MsgSendRecv for HScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: unsafe { co::SB_REQ::from_raw(LOWORD(p.wparam as _)) },
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

/// [`WM_INITDIALOG`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
pub struct InitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

unsafe impl MsgSend for InitDialog {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.ptr() as _,
			lparam: self.additional_data,
		}
	}
}

unsafe impl MsgSendRecv for InitDialog {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: unsafe { HWND::from_ptr(p.wparam as _) },
			additional_data: p.lparam,
		}
	}
}

/// [`WM_INITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct InitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

unsafe impl MsgSend for InitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as _) as _,
		}
	}
}

unsafe impl MsgSendRecv for InitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: unsafe { HMENU::from_ptr(p.wparam as _) },
			item_pos: LOWORD(p.lparam as _),
			is_window_menu: HIWORD(p.lparam as _) != 0,
		}
	}
}

pub_struct_msg_char_key! { KeyDown: co::WM::KEYDOWN;
	/// [`WM_KEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
}

pub_struct_msg_char_key! { KeyUp: co::WM::KEYUP;
	/// [`WM_KEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
}

/// [`WM_KILLFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct KillFocus {
	pub hwnd: Option<HWND>,
}

unsafe impl MsgSend for KillFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::KILLFOCUS,
			wparam: self.hwnd.as_ref().map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for KillFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: match p.wparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

pub_struct_msg_button! { LButtonDblClk: co::WM::LBUTTONDBLCLK;
	/// [`WM_LBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
}

pub_struct_msg_button! { LButtonDown: co::WM::LBUTTONDOWN;
	/// [`WM_LBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
}

pub_struct_msg_button! { LButtonUp: co::WM::LBUTTONUP;
	/// [`WM_LBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
}

pub_struct_msg_button! { MButtonDblClk: co::WM::MBUTTONDBLCLK;
	/// [`WM_MBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
}

pub_struct_msg_button! { MButtonDown: co::WM::MBUTTONDOWN;
	/// [`WM_MBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
}

pub_struct_msg_button! { MButtonUp: co::WM::MBUTTONUP;
	/// [`WM_MBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
}

/// [`WM_MENUCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuCommand {
	pub item_index: u32,
	pub hmenu: HMENU,
}

unsafe impl MsgSend for MenuCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUCOMMAND,
			wparam: self.item_index as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

unsafe impl MsgSendRecv for MenuCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			item_index: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_MENUDRAG`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
/// message parameters.
///
/// Return type: `co::MND`.
pub struct MenuDrag {
	pub position: u32,
	pub hmenu: HMENU,
}

unsafe impl MsgSend for MenuDrag {
	type RetType = co::MND;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::MND::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUDRAG,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

unsafe impl MsgSendRecv for MenuDrag {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

/// [`WM_MENURBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
/// message parameters.
///
/// Return type: `()`.
pub struct MenuRButtonUp {
	pub position: u32,
	pub hmenu: HMENU,
}

unsafe impl MsgSend for MenuRButtonUp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENURBUTTONUP,
			wparam: self.position as _,
			lparam: self.hmenu.ptr() as _,
		}
	}
}

unsafe impl MsgSendRecv for MenuRButtonUp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: unsafe { HMENU::from_ptr(p.lparam as _) },
		}
	}
}

pub_struct_msg_button! { MouseHover: co::WM::MOUSEHOVER;
	/// [`WM_MOUSEHOVER`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
}

pub_struct_msg_empty_handleable! { MouseLeave: co::WM::MOUSELEAVE;
	/// [`WM_MOUSELEAVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
}

pub_struct_msg_button! { MouseMove: co::WM::MOUSEMOVE;
	/// [`WM_MOUSEMOVE`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
}

/// [`WM_MOVE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
pub struct Move {
	pub coords: POINT,
}

unsafe impl MsgSend for Move {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: u32::from(self.coords) as _,
		}
	}
}

unsafe impl MsgSendRecv for Move {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			coords: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_MOVING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
pub struct Moving<'a> {
	pub window_pos: &'a mut RECT,
}

unsafe impl<'a> MsgSend for Moving<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *mut _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for Moving<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_pos: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_NCCALCSIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `co::WVR`.
pub struct NcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

unsafe impl<'a, 'b> MsgSend for NcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::WVR::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCALCSIZE,
			wparam: match &self.data {
				NccspRect::Nccsp(_) => true as _,
				NccspRect::Rect(_) => false as _,
			},
			lparam: match &self.data {
				NccspRect::Nccsp(nccalc) => *nccalc as *const _ as _,
				NccspRect::Rect(rc) => *rc as *const _ as _,
			},
		}
	}
}

unsafe impl<'a, 'b> MsgSendRecv for NcCalcSize<'a, 'b> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			data: match p.wparam {
				0 => NccspRect::Rect(unsafe { &mut *(p.lparam as *mut _) }),
				_ => NccspRect::Nccsp(unsafe { &mut *(p.lparam as *mut _) }),
			},
		}
	}
}

/// [`WM_NCCREATE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
pub struct NcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

unsafe impl<'a, 'b, 'c> MsgSend for NcCreate<'a, 'b, 'c> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

unsafe impl<'a, 'b, 'c> MsgSendRecv for NcCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_empty_handleable! { NcDestroy: co::WM::NCDESTROY;
	/// [`WM_NCDESTROY`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
}

/// [`WM_NCHITTEST`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
/// message parameters.
///
/// Return type: `co::HT`.
pub struct NcHitTest{
	pub cursor_pos: POINT,
}

unsafe impl MsgSend for NcHitTest {
	type RetType = co::HT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::HT::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCHITTEST,
			wparam: 0,
			lparam: u32::from(self.cursor_pos) as _,
		}
	}
}

unsafe impl MsgSendRecv for NcHitTest {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			cursor_pos: POINT::from(p.lparam as u32),
		}
	}
}

/// [`WM_NEXTDLGCTL`](https://learn.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
/// message parameters.
///
/// Return type: `()`.
pub struct NextDlgCtl {
	pub hwnd_focus: HwndFocus,
}

unsafe impl MsgSend for NextDlgCtl {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NEXTDLGCTL,
			wparam: match &self.hwnd_focus {
				HwndFocus::Hwnd(hctl) => hctl.ptr() as _,
				HwndFocus::FocusNext(next) => if *next { 0 } else { 1 },
			},
			lparam: MAKEDWORD(match &self.hwnd_focus {
				HwndFocus::Hwnd(_) => 1,
				HwndFocus::FocusNext(_) => 0,
			}, 0) as _,
		}
	}
}

unsafe impl MsgSendRecv for NextDlgCtl {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: match p.wparam {
				1 => HwndFocus::Hwnd(unsafe { HWND::from_ptr(p.wparam as _) }),
				_ => HwndFocus::FocusNext(p.wparam == 0),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { Null: co::WM::NULL;
	/// [`WM_NULL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-null)
}

/// [`WM_PARENTNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct ParentNotify {
	pub event: co::WMPN,
	pub child_id: u16,
	pub data: HwndPointId,
}

unsafe impl MsgSend for ParentNotify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::PARENTNOTIFY,
			wparam: MAKEDWORD(self.event.raw(), self.child_id) as _,
			lparam: self.data.as_isize(),
		}
	}
}

unsafe impl MsgSendRecv for ParentNotify {
	fn from_generic_wm(p: WndMsg) -> Self {
		let event = unsafe { co::WMPN::from_raw(LOWORD(p.wparam as _)) };
		Self {
			event,
			child_id: HIWORD(p.wparam as _),
			data: match event {
				co::WMPN::CREATE
				| co::WMPN::DESTROY => HwndPointId::Hwnd(unsafe { HWND::from_ptr(p.lparam as _) }),
				co::WMPN::POINTERDOWN => HwndPointId::Id(p.lparam as _),
				_ => HwndPointId::Point(POINT::from(p.lparam as u32)),
			},
		}
	}
}

/// [`WM_POWERBROADCAST`](https://learn.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast)
/// message parameters.
///
/// Return type: `()`.
pub struct PowerBroadcast<'a> {
	pub event: co::PBT,
	pub data: Option<&'a POWERBROADCAST_SETTING>,
}

unsafe impl<'a> MsgSend for PowerBroadcast<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::POWERBROADCAST,
			wparam: self.event.raw() as _,
			lparam: self.data.map_or(0, |d| d as *const _ as _),
		}
	}
}

unsafe impl<'a> MsgSendRecv for PowerBroadcast<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: unsafe { co::PBT::from_raw(p.wparam as _) },
			data: if p.lparam == 0 {
				None
			} else {
				Some(unsafe { &*(p.lparam as *const _) })
			},
		}
	}
}

/// [`WM_QUERYOPEN`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct QueryOpen {}

unsafe impl MsgSend for QueryOpen {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for QueryOpen {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

pub_struct_msg_button! { RButtonDblClk: co::WM::RBUTTONDBLCLK;
	/// [`WM_RBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
}

pub_struct_msg_button! { RButtonDown: co::WM::RBUTTONDOWN;
	/// [`WM_RBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
}

pub_struct_msg_button! { RButtonUp: co::WM::RBUTTONUP;
	/// [`WM_RBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
}

/// [`WM_SETCURSOR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetCursor {
	pub hwnd: HWND,
	pub hit_test: co::HT,
	pub mouse_msg: u16,
}

unsafe impl MsgSend for SetCursor {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETCURSOR,
			wparam: self.hwnd.ptr() as _,
			lparam: MAKEDWORD(self.hit_test.raw(), self.mouse_msg) as _,
		}
	}
}

unsafe impl MsgSendRecv for SetCursor {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: unsafe { HWND::from_ptr(p.wparam as _) },
			hit_test: unsafe { co::HT::from_raw(LOWORD(p.lparam as _)) },
			mouse_msg: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_SETFOCUS`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct SetFocus {
	pub hwnd_losing_focus: HWND,
}

unsafe impl MsgSend for SetFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.ptr() as _,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for SetFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_losing_focus: unsafe { HWND::from_ptr(p.wparam as _) },
		}
	}
}

/// [`WM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
pub struct SetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

unsafe impl MsgSend for SetIcon {
	type RetType = Option<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HICON::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETICON,
			wparam: self.size.raw() as _,
			lparam: self.hicon.ptr() as _,
		}
	}
}

unsafe impl MsgSendRecv for SetIcon {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				size: co::ICON_SZ::from_raw(p.wparam as _),
				hicon: HICON::from_ptr(p.lparam as _),
			}
		}
	}
}

/// [`WM_SETTEXT`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settext)
/// message parameters.
///
/// Return type: `bool`.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, msg};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let new_text = w::WString::from_str("some text");
///
/// hwnd.SendMessage(
///     msg::wm::SetText {
///         text: unsafe { new_text.as_ptr() },
///     },
/// );
/// ```
pub struct SetText {
	pub text: *const u16, // can't be WString because this message can be received
}

unsafe impl MsgSend for SetText {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			0 | LB_ERRSPACE | CB_ERR => false, // CB_ERRSPACE is equal to LB_ERRSPACE
			_ => true,
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETTEXT,
			wparam: 0,
			lparam: self.text as _,
		}
	}
}

unsafe impl MsgSendRecv for SetText {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			text: p.lparam as _,
		}
	}
}

/// [`WM_SHOWWINDOW`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
pub struct ShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

unsafe impl MsgSend for ShowWindow {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as _,
			lparam: self.status.raw() as _,
		}
	}
}

unsafe impl MsgSendRecv for ShowWindow {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: unsafe { co::SW_S::from_raw(p.lparam as _) },
		}
	}
}

/// [`WM_SIZE`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
pub struct Size {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

unsafe impl MsgSend for Size {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZE,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.client_area) as _,
		}
	}
}

unsafe impl MsgSendRecv for Size {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: unsafe { co::SIZE_R::from_raw(p.wparam as _) },
			client_area: SIZE::new(
				LOWORD(p.lparam as _) as _,
				HIWORD(p.lparam as _) as _,
			),
		}
	}
}

/// [`WM_SIZING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
pub struct Sizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

unsafe impl<'a> MsgSend for Sizing<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.raw() as _,
			lparam: self.coords as *mut _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for Sizing<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				window_edge: co::WMSZ::from_raw(p.wparam as _),
				coords: &mut *(p.lparam as *mut _),
			}
		}
	}
}

/// [`WM_STYLECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

unsafe impl<'a> MsgSend for StyleChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for StyleChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				change: co::GWL_C::from_raw(p.wparam as _),
				stylestruct: &*(p.lparam as *const _),
			}
		}
	}
}

/// [`WM_STYLECHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
pub struct StyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

unsafe impl<'a> MsgSend for StyleChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.raw() as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for StyleChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				change: co::GWL_C::from_raw(p.wparam as _),
				stylestruct: &*(p.lparam as *const _),
			}
		}
	}
}

pub_struct_msg_char_code! { SysChar: co::WM::SYSCHAR;
	/// [`WM_SYSCHAR`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
}

/// [`WM_SYSCOMMAND`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
/// message parameters.
///
/// Return type: `()`.
pub struct SysCommand {
	pub request: co::SC,
	pub position: POINT,
}

unsafe impl MsgSend for SysCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SYSCOMMAND,
			wparam: self.request.raw() as _,
			lparam: u32::from(self.position) as _,
		}
	}
}

unsafe impl MsgSendRecv for SysCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: unsafe { co::SC::from_raw(p.wparam as _) },
			position: POINT::from(p.lparam as u32),
		}
	}
}

pub_struct_msg_char_code! { SysDeadChar: co::WM::SYSDEADCHAR;
	/// [`WM_SYSDEADCHAR`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
}

pub_struct_msg_char_key! { SysKeyDown: co::WM::SYSKEYDOWN;
	/// [`WM_SYSKEYDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
}

pub_struct_msg_char_key! { SysKeyUp: co::WM::SYSKEYUP;
	/// [`WM_SYSKEYUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
}

pub_struct_msg_empty_handleable! { ThemeChanged: co::WM::THEMECHANGED;
	/// [`WM_THEMECHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
}

/// [`WM_TIMER`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
pub struct Timer {
	pub timer_id: usize,
	pub timer_proc: Option<TIMERPROC>,
}

unsafe impl MsgSend for Timer {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id,
			lparam: self.timer_proc.map_or(0, |proc| proc as _),
		}
	}
}

unsafe impl MsgSendRecv for Timer {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			timer_id: p.wparam as _,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			},
		}
	}
}

/// [`WM_UNINITMENUPOPUP`](https://learn.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
/// message parameters.
///
/// Return type: `()`.
pub struct UninitMenuPopup {
	pub hmenu: HMENU,
	pub which: co::MF,
}

unsafe impl MsgSend for UninitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNINITMENUPOPUP,
			wparam: self.hmenu.ptr() as _,
			lparam: MAKEDWORD(0, self.which.raw() as _) as _,
		}
	}
}

unsafe impl MsgSendRecv for UninitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		unsafe {
			Self {
				hmenu: HMENU::from_ptr(p.wparam as _),
				which: co::MF::from_raw(LOWORD(p.lparam as _) as _),
			}
		}
	}
}

/// [`WM_UNDO`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-undo)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct Undo {}

unsafe impl MsgSend for Undo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNDO,
			wparam: 0,
			lparam: 0,
		}
	}
}

unsafe impl MsgSendRecv for Undo {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_VSCROLL`](https://learn.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
/// message parameters.
///
/// Return type: `()`.
pub struct VScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

unsafe impl MsgSend for VScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::VSCROLL,
			wparam: MAKEDWORD(self.request.raw(), self.scroll_box_pos) as _,
			lparam: self.hcontrol.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

unsafe impl MsgSendRecv for VScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: unsafe { co::SB_REQ::from_raw(LOWORD(p.wparam as _)) },
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(unsafe { HWND::from_ptr(ptr as _) }),
			},
		}
	}
}

/// [`WM_WINDOWPOSCHANGED`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

unsafe impl<'a> MsgSend for WindowPosChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for WindowPosChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WINDOWPOSCHANGING`](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
pub struct WindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

unsafe impl<'a> MsgSend for WindowPosChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

unsafe impl<'a> MsgSendRecv for WindowPosChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WTSSESSION_CHANGE`](https://learn.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change)
/// message parameters.
///
/// Return type: `()`.
pub struct WtsSessionChange {
	pub state: co::WTS,
	pub session_id: u32,
}

unsafe impl MsgSend for WtsSessionChange {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WTSSESSION_CHANGE,
			wparam: self.state.raw() as _,
			lparam: self.session_id as _,
		}
	}
}

unsafe impl MsgSendRecv for WtsSessionChange {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			state: unsafe { co::WTS::from_raw(p.wparam as _) },
			session_id: p.lparam as _,
		}
	}
}

pub_struct_msg_button! { XButtonDblClk: co::WM::XBUTTONDBLCLK;
	/// [`WM_XBUTTONDBLCLK`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
}

pub_struct_msg_button! { XButtonDown: co::WM::XBUTTONDOWN;
	/// [`WM_XBUTTONDOWN`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
}

pub_struct_msg_button! { XButtonUp: co::WM::XBUTTONUP;
	/// [`WM_XBUTTONUP`](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
}
