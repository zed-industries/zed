use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`STM_GETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-geticon)
/// message, which has no parameters.
///
/// Return type: `SysResult<HICON>`.
pub struct GetIcon {}

unsafe impl MsgSend for GetIcon {
	type RetType = SysResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HICON::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETICON.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`STM_GETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-getimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIconCurMeta>`.
pub struct GetImage {
	pub img_type: co::IMAGE_TYPE,
}

unsafe impl MsgSend for GetImage {
	type RetType = SysResult<BmpIconCurMeta>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe {
			match self.img_type {
				co::IMAGE_TYPE::BITMAP => Ok(BmpIconCurMeta::Bmp(HBITMAP::from_ptr(v as _))),
				co::IMAGE_TYPE::ICON => Ok(BmpIconCurMeta::Icon(HICON::from_ptr(v as _))),
				co::IMAGE_TYPE::CURSOR => Ok(BmpIconCurMeta::Cur(HCURSOR::from_ptr(v as _))),
				co::IMAGE_TYPE::ENHMETAFILE => Ok(BmpIconCurMeta::Meta(HDC::from_ptr(v as _))),
				_ => Err(co::ERROR::BAD_ARGUMENTS),
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::GETIMAGE.into(),
			wparam: self.img_type.raw() as _,
			lparam: 0,
		}
	}
}

/// [`STM_SETICON`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-seticon)
/// message parameters.
///
/// Return type: `SysResult<HICON>`.
pub struct SetIcon<'a> {
	pub icon: &'a HICON,
}

unsafe impl<'a> MsgSend for SetIcon<'a> {
	type RetType = SysResult<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HICON::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETICON.into(),
			wparam: self.icon.ptr() as _,
			lparam: 0,
		}
	}
}

/// [`STM_SETIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/stm-setimage)
/// message parameters.
///
/// Return type: `SysResult<BmpIconCurMeta>`.
pub struct SetImage {
	pub image: BmpIconCurMeta,
}

unsafe impl MsgSend for SetImage {
	type RetType = SysResult<BmpIconCurMeta>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		if v == 0 {
			Err(co::ERROR::BAD_ARGUMENTS)
		} else {
			unsafe {
				match self.image {
					BmpIconCurMeta::Bmp(_) => Ok(BmpIconCurMeta::Bmp(HBITMAP::from_ptr(v as _))),
					BmpIconCurMeta::Icon(_) => Ok(BmpIconCurMeta::Icon(HICON::from_ptr(v as _))),
					BmpIconCurMeta::Cur(_) => Ok(BmpIconCurMeta::Cur(HCURSOR::from_ptr(v as _))),
					BmpIconCurMeta::Meta(_) => Ok(BmpIconCurMeta::Meta(HDC::from_ptr(v as _))),
				}
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::STM::SETIMAGE.into(),
			wparam: match self.image {
				BmpIconCurMeta::Bmp(_) => co::IMAGE_TYPE::BITMAP,
				BmpIconCurMeta::Icon(_) => co::IMAGE_TYPE::ICON,
				BmpIconCurMeta::Cur(_) => co::IMAGE_TYPE::CURSOR,
				BmpIconCurMeta::Meta(_) => co::IMAGE_TYPE::ENHMETAFILE,
			}.raw() as _,
			lparam: self.image.as_isize(),
		}
	}
}
