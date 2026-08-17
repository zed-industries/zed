use crate::co;
use crate::comctl::privs::*;
use crate::decl::*;
use crate::msg::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TB_ADDBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addbitmap)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct AddBitmap<'a> {
	pub num_images: u32,
	pub info: &'a TBADDBITMAP,
}

unsafe impl<'a> MsgSend for AddBitmap<'a> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBITMAP.into(),
			wparam: self.num_images as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_ADDBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addbuttons)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct AddButtons<'a, 'b> {
	pub buttons: &'a mut [TBBUTTON<'b>],
}

unsafe impl<'a, 'b> MsgSend for AddButtons<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBUTTONS.into(),
			wparam: self.buttons.len() as _,
			lparam: self.buttons.as_mut_ptr() as _,
		}
	}
}

/// [`TB_ADDSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-addstring)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct AddString {
	pub texts: ResStrs,
}

unsafe impl MsgSend for AddString {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDSTRING.into(),
			wparam: match &self.texts {
				ResStrs::Res(_, hinst) => hinst.ptr() as _,
				ResStrs::Strs(_) => 0,
			},
			lparam: match &self.texts {
				ResStrs::Res(res, _) => res.as_ptr() as _,
				ResStrs::Strs(strs) => strs.as_ptr() as _,
			},
		}
	}
}

pub_struct_msg_empty! { AutoSize: co::TBM::AUTOSIZE.into();
	/// [`TB_AUTOSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-autosize)
}

/// [`TB_BUTTONCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-buttoncount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct ButtonCount {}

unsafe impl MsgSend for ButtonCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::BUTTONCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_BUTTONSTRUCTSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-buttonstructsize)
/// message parameters.
///
/// Return type: `()`.
pub struct ButtonStructSize {
	pub size: u32,
}

unsafe impl MsgSend for ButtonStructSize {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::BUTTONSTRUCTSIZE.into(),
			wparam: self.size as _,
			lparam: 0,
		}
	}
}

/// [`TB_CHANGEBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-changebitmap)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct ChangeBitmap {
	pub btn_cmd_id: u16,
	pub image: IdxCbNone,
}

unsafe impl MsgSend for ChangeBitmap {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHANGEBITMAP.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.image.into(),
		}
	}
}

/// [`TB_CHECKBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-checkbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct CheckButton {
	pub btn_cmd_id: u16,
	pub check: bool,
}

unsafe impl MsgSend for CheckButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHECKBUTTON.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.check as _,
		}
	}
}

/// [`TB_COMMANDTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-commandtoindex)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct CommandToIndex {
	pub btn_cmd_id: u16,
}

unsafe impl MsgSend for CommandToIndex {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::COMMANDTOINDEX.into(),
			wparam: self.btn_cmd_id as _,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { Customize: co::TBM::CUSTOMIZE.into();
	/// [`TB_CUSTOMIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-customize)
}

/// [`TB_DELETEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-deletebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteButton {
	pub btn_index: u32,
}

unsafe impl MsgSend for DeleteButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::CHECKBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: 0,
		}
	}
}

/// [`TB_ENABLEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-enablebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct EnableButton {
	pub btn_cmd_id: u16,
	pub enable: bool,
}

unsafe impl MsgSend for EnableButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ENABLEBUTTON.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.enable as _,
		}
	}
}

/// [`TB_GETANCHORHIGHLIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getanchorhighlight)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct GetAnchorHighlight {}

unsafe impl MsgSend for GetAnchorHighlight {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETANCHORHIGHLIGHT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbitmap)
/// message parameters.
///
/// Return type: `u32`.
pub struct GetBitmap {
	pub btn_cmd_id: u16,
}

unsafe impl MsgSend for GetBitmap {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBITMAP.into(),
			wparam: self.btn_cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_GETBITMAPFLAGS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbitmapflags)
/// message, which has no parameters.
///
/// Return type: `co::TBBF`.
pub struct GetBitmapFlags {}

unsafe impl MsgSend for GetBitmapFlags {
	type RetType = co::TBBF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TBBF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBITMAPFLAGS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetButton<'a, 'b> {
	pub btn_index: u32,
	pub info: &'a mut TBBUTTON<'b>,
}

unsafe impl<'a, 'b> MsgSend for GetButton<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETBUTTONINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttoninfo)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetButtonInfo<'a, 'b> {
	pub btn_cmd_id: u16,
	pub info: &'a mut TBBUTTONINFO<'b>,
}

unsafe impl<'a, 'b> MsgSend for GetButtonInfo<'a, 'b> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONINFO.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETBUTTONSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttonsize)
/// message, which has no parameters.
///
/// Return type: `SIZE`.
pub struct GetButtonSize {}

unsafe impl MsgSend for GetButtonSize {
	type RetType = SIZE;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		SIZE::from(v as u32)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONSIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETBUTTONTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getbuttontext)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct GetButtonText<'a> {
	pub btn_cmd_id: u16,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetButtonText<'a> {
	type RetType = SysResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETBUTTONTEXT.into(),
			wparam: self.btn_cmd_id as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`TB_GETCOLORSCHEME`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getcolorscheme)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetColorScheme<'a> {
	pub scheme: &'a mut COLORSCHEME,
}

unsafe impl<'a> MsgSend for GetColorScheme<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETCOLORSCHEME.into(),
			wparam: 0,
			lparam: self.scheme as *mut _ as _,
		}
	}
}

/// [`TB_GETDISABLEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getdisabledimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetDisabledImageList {}

unsafe impl MsgSend for GetDisabledImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETDISABLEDIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TBSTYLE_EX`.
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TBSTYLE_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TBSTYLE_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETHOTIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gethotimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetHotImageList {}

unsafe impl MsgSend for GetHotImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETHOTIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gethotitem)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetHotItem {}

unsafe impl MsgSend for GetHotItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETHOTITEM.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETIDEALSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getidealsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetIdealSize<'a> {
	pub get_height: bool,
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIDEALSIZE.into(),
			wparam: self.get_height as _,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`TB_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetImageList {}

unsafe impl MsgSend for GetImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETIMAGELISTCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getimagelistcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetImageListCount {}

unsafe impl MsgSend for GetImageListCount {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETIMAGELISTCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getinsertmark)
/// message parameters.
///
/// Return type: `()`.
pub struct GetInsertMark<'a> {
	pub info: &'a mut TBINSERTMARK,
}

unsafe impl<'a> MsgSend for GetInsertMark<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETINSERTMARK.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_GETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getinsertmarkcolor)
/// message, which has no parameters.
///
/// Return type: `COLORREF`.
pub struct GetInsertMarkColor {}

unsafe impl MsgSend for GetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETITEMDROPDOWNRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getitemdropdownrect)
/// message parameters.
///
/// Return type: `()`.
pub struct GetItemDropdownRect<'a> {
	pub item_index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemDropdownRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETITEMDROPDOWNRECT.into(),
			wparam: self.item_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItemRect<'a> {
	pub btn_index: u32,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETITEMRECT.into(),
			wparam: self.btn_index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETMAXSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getmaxsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetMaxSize<'a> {
	pub size: &'a mut SIZE,
}

unsafe impl<'a> MsgSend for GetMaxSize<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETMAXSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`TB_GETMETRICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getmetrics)
/// message parameters.
///
/// Return type: `()`.
pub struct GetMetrics<'a> {
	pub metrics: &'a mut TBMETRICS,
}

unsafe impl<'a> MsgSend for GetMetrics<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETMETRICS.into(),
			wparam: 0,
			lparam: self.metrics as *mut _ as _,
		}
	}
}

/// [`TB_GETOBJECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getobject)
/// message parameters.
///
/// Return type: `HrResult<()>`.
pub struct GetObject<'a, T>
	where T: ole_IDropTarget,
{
	pub obj: &'a mut T,
}

unsafe impl<'a, T> MsgSend for GetObject<'a, T>
	where T: ole_IDropTarget,
{
	type RetType = HrResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		ok_to_hrresult(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETOBJECT.into(),
			wparam: &IDropTarget::IID as *const _ as _,
			lparam: &self.obj.ptr() as *const _ as _,
		}
	}
}

/// [`TB_GETPADDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getpadding)
/// message, which has no parameters.
///
/// Return type: `(u16, u16)`.
pub struct GetPadding {}

unsafe impl MsgSend for GetPadding {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPADDING.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETPRESSEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getpressedimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct GetPressedImageList {}

unsafe impl MsgSend for GetPressedImageList {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETPRESSEDIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetRect<'a> {
	pub cmd_id: u16,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for GetRect<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETRECT.into(),
			wparam: self.cmd_id as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TB_GETROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getrows)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetRows {}

unsafe impl MsgSend for GetRows {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETROWS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstate)
/// message parameters.
///
/// Return type: `SysResult<co::TBSTATE>`.
pub struct GetState {
	pub cmd_id: u16,
}

unsafe impl MsgSend for GetState {
	type RetType = SysResult<co::TBSTATE>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| unsafe { co::TBSTATE::from_raw(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTATE.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_GETSTRING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstring)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetString<'a> {
	pub index: u16,
	pub text: &'a mut WString,
}

unsafe impl<'a> MsgSend for GetString<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTRING.into(),
			wparam: MAKEDWORD(self.text.buf_len() as _, self.index) as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`TB_GETSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getstyle)
/// message, which has no parameters.
///
/// Return type: `co::BTNS`.
pub struct GetStyle {}

unsafe impl MsgSend for GetStyle {
	type RetType = co::BTNS;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::BTNS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETTEXTROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gettextrows)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetTextRows {}

unsafe impl MsgSend for GetTextRows {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTEXTROWS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct GetTooltips {}

unsafe impl MsgSend for GetTooltips {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HWND::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TB_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-getunicodeformat)
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

/// [`TB_HIDEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-hidebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct HideButton {
	pub cmd_id: u16,
	pub hide: bool,
}

unsafe impl MsgSend for HideButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::HIDEBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.hide as _, 0) as _,
		}
	}
}

/// [`TB_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-hittest)
/// message parameters.
///
/// Return type: `i32`.
pub struct HitTest<'a> {
	pub coords: &'a mut POINT,
}

unsafe impl<'a> MsgSend for HitTest<'a> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::HITTEST.into(),
			wparam: 0,
			lparam: self.coords as *mut _ as _,
		}
	}
}

/// [`TB_INDETERMINATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-indeterminate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct Indeterminate {
	pub cmd_id: u16,
	pub indeterminate: bool,
}

unsafe impl MsgSend for Indeterminate {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INDETERMINATE.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.indeterminate as _, 0) as _,
		}
	}
}

/// [`TB_INSERTBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-insertbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct InsertButton<'a, 'b> {
	pub index: u32,
	pub button: &'a TBBUTTON<'b>,
}

unsafe impl<'a, 'b> MsgSend for InsertButton<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INSERTBUTTON.into(),
			wparam: self.index as _,
			lparam: self.button as *const _ as _,
		}
	}
}

/// [`TB_INSERTMARKHITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-insertmarkhittest)
/// message parameters.
///
/// Return type: `bool`.
pub struct InsertMarkHitTest<'a, 'b> {
	pub coords: &'a POINT,
	pub info: &'b mut TBINSERTMARK,
}

unsafe impl<'a, 'b> MsgSend for InsertMarkHitTest<'a, 'b> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::INSERTMARKHITTEST.into(),
			wparam: self.coords as *const _ as _,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TB_ISBUTTONCHECKED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonchecked)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonChecked {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonChecked {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONCHECKED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONENABLED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonenabled)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonEnabled {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonEnabled {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONENABLED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONHIDDEN`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonhidden)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonHidden {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonHidden {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONHIDDEN.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONHIGHLIGHTED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonhighlighted)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonHighlighted {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonHighlighted {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONHIGHLIGHTED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONINDETERMINATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonindeterminate)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonIndeterminate {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonIndeterminate {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONINDETERMINATE.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_ISBUTTONPRESSED`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-isbuttonpressed)
/// message parameters.
///
/// Return type: `bool`.
pub struct IsButtonPressed {
	pub cmd_id: u16,
}

unsafe impl MsgSend for IsButtonPressed {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ISBUTTONPRESSED.into(),
			wparam: self.cmd_id as _,
			lparam: 0,
		}
	}
}

/// [`TB_LOADIMAGES`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-loadimages)
/// message parameters.
///
/// Return type: `u32`.
pub struct LoadImages {
	pub img_list: co::IDB,
}

unsafe impl MsgSend for LoadImages {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::LOADIMAGES.into(),
			wparam: self.img_list.raw() as _,
			lparam: HINST_COMMCTRL,
		}
	}
}

/// [`TB_MAPACCELERATOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-mapaccelerator)
/// message parameters.
///
/// Return type: `bool`.
pub struct MapAccelerator<'a> {
	pub character: char,
	pub cmd_id: &'a mut u16,
}

unsafe impl<'a> MsgSend for MapAccelerator<'a> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MAPACCELERATOR.into(),
			wparam: self.character as _,
			lparam: self.cmd_id as *mut _ as _,
		}
	}
}

/// [`TB_MARKBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-markbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct MarkButton {
	pub cmd_id: u16,
	pub highlight: bool,
}

unsafe impl MsgSend for MarkButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MARKBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.highlight as _, 0) as _,
		}
	}
}

/// [`TB_MOVEBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-movebutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct MoveButton {
	pub btn_index: u32,
	pub dest_index: u32,
}

unsafe impl MsgSend for MoveButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::MOVEBUTTON.into(),
			wparam: self.btn_index as _,
			lparam: self.dest_index as _,
		}
	}
}

/// [`TB_PRESSBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-pressbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PressButton {
	pub cmd_id: u16,
	pub press: bool,
}

unsafe impl MsgSend for PressButton {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::PRESSBUTTON.into(),
			wparam: self.cmd_id as _,
			lparam: MAKEDWORD(self.press as _, 0) as _,
		}
	}
}

/// [`TB_REPLACEBITMAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-replacebitmap)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct ReplaceBitmap<'a> {
	pub info: &'a TBREPLACEBITMAP,
}

unsafe impl<'a> MsgSend for ReplaceBitmap<'a> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::REPLACEBITMAP.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_SAVERESTORE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-saverestore)
/// message parameters.
///
/// Return type: `()`.
pub struct SaveRestore<'a, 'b> {
	pub save: bool,
	pub info: &'b mut TBSAVEPARAMS<'a>,
}

unsafe impl<'a, 'b> MsgSend for SaveRestore<'a, 'b> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SAVERESTORE.into(),
			wparam: self.save as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_SETANCHORHIGHLIGHT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setanchorhighlight)
/// message parameters.
///
/// Return type: `bool`.
pub struct SetAnchorHighlight {
	pub enable: bool,
}

unsafe impl MsgSend for SetAnchorHighlight {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETANCHORHIGHLIGHT.into(),
			wparam: self.enable as _,
			lparam: 0,
		}
	}
}

/// [`TB_SETBITMAPSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setbitmapsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetBitmapSize {
	pub width: u16,
	pub height: u16,
}

unsafe impl MsgSend for SetBitmapSize {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETBITMAPSIZE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.width, self.height) as _,
		}
	}
}

/// [`TB_SETBUTTONINFO`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setbuttoninfo)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetButtonInfo<'a, 'b> {
	pub btn_cmd_id: u16,
	pub info: &'b TBBUTTONINFO<'a>,
}

unsafe impl<'a, 'b> MsgSend for SetButtonInfo<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETBUTTONINFO.into(),
			wparam: self.btn_cmd_id as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_SETBUTTONSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setbuttonsize)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetButtonSize {
	pub width: u16,
	pub height: u16,
}

unsafe impl MsgSend for SetButtonSize {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETBUTTONSIZE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.width, self.height) as _,
		}
	}
}

/// [`TB_SETBUTTONWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setbuttonwidth)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetButtonWidth {
	pub min_width: u16,
	pub max_width: u16,
}

unsafe impl MsgSend for SetButtonWidth {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETBUTTONWIDTH.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.min_width, self.max_width) as _,
		}
	}
}

/// [`TB_SETCMDID`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setcmdid)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetCmdId {
	pub btn_index: u32,
	pub cmd_id: u16,
}

unsafe impl MsgSend for SetCmdId {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETCMDID.into(),
			wparam: self.btn_index as _,
			lparam: self.cmd_id as _,
		}
	}
}

/// [`TB_SETCOLORSCHEME`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setcolorscheme)
/// message parameters.
///
/// Return type: `()`.
pub struct SetColorScheme<'a> {
	pub scheme: &'a COLORSCHEME,
}

unsafe impl<'a> MsgSend for SetColorScheme<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETCOLORSCHEME.into(),
			wparam: 0,
			lparam: self.scheme as *const _ as _,
		}
	}
}

/// [`TB_SETDISABLEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setdisabledimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetDisabledImageList<'a> {
	pub himagelist: &'a HIMAGELIST,
}

unsafe impl<'a> MsgSend for SetDisabledImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETDISABLEDIMAGELIST.into(),
			wparam: 0,
			lparam: self.himagelist.ptr() as _,
		}
	}
}

/// [`TB_SETDRAWTEXTFLAGS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setdrawtextflags)
/// message parameters.
///
/// Return type: `co::DT`.
pub struct SetDrawTextFlags {
	pub mask: co::DT,
	pub draw: co::DT,
}

unsafe impl MsgSend for SetDrawTextFlags {
	type RetType = co::DT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::DT::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETDRAWTEXTFLAGS.into(),
			wparam: self.mask.raw() as _,
			lparam: self.draw.raw() as _,
		}
	}
}

/// [`TB_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setextendedstyle)
/// message parameters.
///
/// Return type: `co::TBSTYLE_EX`.
pub struct SetExtendedStyle {
	pub style: co::TBSTYLE_EX,
}

unsafe impl MsgSend for SetExtendedStyle {
	type RetType = co::TBSTYLE_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TBSTYLE_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`TB_SETHOTIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-sethotimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetHotImageList<'a> {
	pub himagelist: &'a HIMAGELIST,
}

unsafe impl<'a> MsgSend for SetHotImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETHOTIMAGELIST.into(),
			wparam: 0,
			lparam: self.himagelist.ptr() as _,
		}
	}
}

/// [`TB_SETHOTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-sethotitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct SetHotItem {
	pub index: Option<u32>,
}

unsafe impl MsgSend for SetHotItem {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETHOTITEM.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: 0,
		}
	}
}

/// [`TB_SETHOTITEM2`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-sethotitem2)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct SetHotItem2 {
	pub index: Option<u32>,
	pub flags: co::HICF,
}

unsafe impl MsgSend for SetHotItem2 {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|idx| idx as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETHOTITEM2.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: self.flags.raw() as _,
		}
	}
}

/// [`TB_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetImageList<'a> {
	pub himagelist: &'a HIMAGELIST,
}

unsafe impl<'a> MsgSend for SetImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.himagelist.ptr() as _,
		}
	}
}

/// [`TB_SETINDENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setindent)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetIndent {
	pub pixels: u32,
}

unsafe impl MsgSend for SetIndent {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETINDENT.into(),
			wparam: self.pixels as _,
			lparam: 0,
		}
	}
}

/// [`TB_SETINSERTMARK`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setinsertmark)
/// message parameters.
///
/// Return type: `()`.
pub struct SetInsertMark<'a> {
	pub info: &'a TBINSERTMARK,
}

unsafe impl<'a> MsgSend for SetInsertMark<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETINSERTMARK.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_SETINSERTMARKCOLOR`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setinsertmarkcolor)
/// message parameters.
///
/// Return type: `COLORREF`.
pub struct SetInsertMarkColor {
	pub color: COLORREF,
}

unsafe impl MsgSend for SetInsertMarkColor {
	type RetType = COLORREF;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		unsafe { COLORREF::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETINSERTMARKCOLOR.into(),
			wparam: 0,
			lparam: u32::from(self.color) as _,
		}
	}
}

/// [`TB_SETLISTGAP`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setlistgap)
/// message parameters.
///
/// Return type: `()`.
pub struct SetListGap {
	pub gap: u32,
}

unsafe impl MsgSend for SetListGap {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETLISTGAP.into(),
			wparam: self.gap as _,
			lparam: 0,
		}
	}
}

/// [`TB_SETMAXTEXTROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setmaxtextrows)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetMaxTextRows {
	pub max_rows: u32,
}

unsafe impl MsgSend for SetMaxTextRows {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETMAXTEXTROWS.into(),
			wparam: self.max_rows as _,
			lparam: 0,
		}
	}
}

/// [`TB_SETMETRICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setmetrics)
/// message parameters.
///
/// Return type: `()`.
pub struct SetMetrics<'a> {
	pub metrics: &'a TBMETRICS,
}

unsafe impl<'a> MsgSend for SetMetrics<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETMETRICS.into(),
			wparam: 0,
			lparam: self.metrics as *const _ as _,
		}
	}
}

/// [`TB_SETPADDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setpadding)
/// message parameters.
///
/// Return type: `(u16, u16)`.
pub struct SetPadding {
	pub horizontal: u16,
	pub vertical: u16,
}

unsafe impl MsgSend for SetPadding {
	type RetType = (u16, u16);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETPADDING.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.horizontal, self.vertical) as _,
		}
	}
}

/// [`TB_SETPARENT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setparent)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct SetParent<'a> {
	pub hparent: &'a HWND
}

unsafe impl<'a> MsgSend for SetParent<'a> {
	type RetType = Option<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HWND::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETPARENT.into(),
			wparam: 0,
			lparam: self.hparent.ptr() as _,
		}
	}
}

/// [`TB_SETPRESSEDIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setpressedimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct SetPressedImageList<'a> {
	pub index: u32,
	pub himagelist: Option<&'a HIMAGELIST>,
}

unsafe impl<'a> MsgSend for SetPressedImageList<'a> {
	type RetType = Option<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HIMAGELIST::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETPRESSEDIMAGELIST.into(),
			wparam: self.index as _,
			lparam: self.himagelist.map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TB_SETROWS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setrows)
/// message parameters.
///
/// Return type: `()`.
pub struct SetRows<'a> {
	pub num_rows: u16,
	pub create_more: bool,
	pub bounds: &'a mut RECT,
}

unsafe impl<'a> MsgSend for SetRows<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETROWS.into(),
			wparam: MAKEDWORD(self.num_rows, self.create_more as _) as _,
			lparam: self.bounds as *mut _ as _,
		}
	}
}

/// [`TB_SETSTATE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setstate)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct SetState {
	pub btn_cmd_id: u16,
	pub state: co::BTNS,
}

unsafe impl MsgSend for SetState {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETSTATE.into(),
			wparam: self.btn_cmd_id as _,
			lparam: MAKEDWORD(self.state.raw() as _, 0) as _,
		}
	}
}

/// [`TB_SETSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setstyle)
/// message parameters.
///
/// Return type: `()`.
pub struct SetStyle {
	pub style: co::BTNS,
}

unsafe impl MsgSend for SetStyle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETSTYLE.into(),
			wparam: 0,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`TB_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-settooltips)
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
			msg_id: co::TBM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TB_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setunicodeformat)
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

/// [`TB_SETWINDOWTHEME`](https://learn.microsoft.com/en-us/windows/win32/controls/tb-setwindowtheme)
/// message parameters.
///
/// Return type: `()`.
pub struct SetWindowTheme {
	pub visual_style: WString,
}

unsafe impl MsgSend for SetWindowTheme {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::SETWINDOWTHEME.into(),
			wparam: 0,
			lparam: self.visual_style.as_ptr() as _,
		}
	}
}
