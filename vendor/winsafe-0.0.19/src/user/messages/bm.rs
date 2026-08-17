use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

pub_struct_msg_empty! { Click: co::BM::CLICK.into();
	/// [`BM_CLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-click)
}

/// [`BM_GETCHECK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getcheck)
/// message parameters.
///
/// Return type: `co::BST`.
pub struct GetCheck {}

unsafe impl MsgSend for GetCheck {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::BST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETCHECK.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_GETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIcon>`.
pub struct GetImage {
	pub img_type: co::IMAGE_TYPE,
}

unsafe impl MsgSend for GetImage {
	type RetType = SysResult<BmpIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe {
			match self.img_type {
				co::IMAGE_TYPE::BITMAP => Ok(BmpIcon::Bmp(HBITMAP::from_ptr(v as _))),
				co::IMAGE_TYPE::ICON => Ok(BmpIcon::Icon(HICON::from_ptr(v as _))),
				_ => Err(co::ERROR::BAD_ARGUMENTS),
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETIMAGE.into(),
			wparam: self.img_type.raw() as _,
			lparam: 0,
		}
	}
}

/// [`BM_GETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-getstate)
/// message, which has no parameters.
///
/// Return type: `co::BST`.
pub struct GetState {}

unsafe impl MsgSend for GetState {
	type RetType = co::BST;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::BST::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::GETSTATE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`BM_SETCHECK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setcheck)
/// message parameters.
///
/// Return type: `()`.
pub struct SetCheck {
	pub state: co::BST,
}

unsafe impl MsgSend for SetCheck {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETCHECK.into(),
			wparam: self.state.raw() as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETDONTCLICK`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setdontclick)
/// message parameters.
///
/// Return type: `()`.
pub struct SetDontClick {
	pub dont_click: bool,
}

unsafe impl MsgSend for SetDontClick {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETDONTCLICK.into(),
			wparam: self.dont_click as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIcon>`.
pub struct SetImage {
	pub image: BmpIcon,
}

unsafe impl MsgSend for SetImage {
	type RetType = SysResult<BmpIcon>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		if v == 0 {
			Err(co::ERROR::BAD_ARGUMENTS)
		} else {
			unsafe {
				match self.image {
					BmpIcon::Bmp(_) => Ok(BmpIcon::Bmp(HBITMAP::from_ptr(v as _))),
					BmpIcon::Icon(_) => Ok(BmpIcon::Icon(HICON::from_ptr(v as _))),
				}
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETIMAGE.into(),
			wparam: match self.image {
				BmpIcon::Bmp(_) => co::IMAGE_TYPE::BITMAP,
				BmpIcon::Icon(_) => co::IMAGE_TYPE::ICON,
			}.raw() as _,
			lparam: self.image.as_isize(),
		}
	}
}

/// [`BM_SETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setstate)
/// message parameters.
///
/// Return type: `()`.
pub struct SetState {
	pub highlight: bool,
}

unsafe impl MsgSend for SetState {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSTATE.into(),
			wparam: self.highlight as _,
			lparam: 0,
		}
	}
}

/// [`BM_SETSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/bm-setstyle)
/// message parameters.
///
/// Return type: `()`.
pub struct SetStyle {
	pub style: co::BS,
	pub redraw: bool,
}

unsafe impl MsgSend for SetStyle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::BM::SETSTYLE.into(),
			wparam: self.style.raw() as _,
			lparam: self.redraw as _,
		}
	}
}
