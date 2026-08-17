#![allow(non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::ole::privs::*;

/// [`InitCommonControls`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrols)
/// function.
pub fn InitCommonControls() {
	unsafe { ffi::InitCommonControls() }
}

/// [`InitCommonControlsEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initcommoncontrolsex)
/// function.
pub fn InitCommonControlsEx(icce: &INITCOMMONCONTROLSEX) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { ffi::InitCommonControlsEx(icce as *const _ as  _) }
	)
}

/// [`InitMUILanguage`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initmuilanguage)
/// function.
pub fn InitMUILanguage(ui_lang: LANGID) {
	unsafe { ffi::InitMUILanguage(ui_lang.into()) }
}

/// [`TaskDialogIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialogindirect)
/// function.
///
/// Returns:
/// * the selected `co::DLGID` button;
/// * if `pRadioButtons` of [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) struct
/// was set, the `u16` control ID of one of the specified radio buttons;
/// otherwise zero.
///
/// If you don't need all customizations, consider the
/// [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog) method.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let mut tdc = w::TASKDIALOGCONFIG::default();
/// tdc.hwndParent = hwnd;
/// tdc.dwCommonButtons = co::TDCBF::YES | co::TDCBF::NO;
/// tdc.set_pszMainIcon(w::IconIdTdicon::Tdicon(co::TD_ICON::INFORMATION));
///
/// let mut title = w::WString::from_str("Title");
/// tdc.set_pszWindowTitle(Some(&mut title));
///
/// let mut header = w::WString::from_str("Header");
/// tdc.set_pszMainInstruction(Some(&mut header));
///
/// let mut body = w::WString::from_str("Body");
/// tdc.set_pszContent(Some(&mut body));
///
/// // A custom button to appear before Yes and No.
/// let mut btn1 = w::TASKDIALOG_BUTTON::default();
/// let mut btn1_text = w::WString::from_str("Hello");
/// btn1.set_pszButtonText(Some(&mut btn1_text));
/// btn1.set_nButtonID(333); // this ID is returned if user clicks this button
/// let btns_slice = &mut [btn1];
/// tdc.set_pButtons(Some(btns_slice));
///
/// w::TaskDialogIndirect(&tdc, None)?;
/// # Ok::<_, co::HRESULT>(())
/// ```
pub fn TaskDialogIndirect(
	task_config: &TASKDIALOGCONFIG,
	verification_flag_checked: Option<&mut bool>,
) -> HrResult<(co::DLGID, u16)>
{
	let mut pn_button = i32::default();
	let mut pn_radio_button = i32::default();
	let mut pf_bool: BOOL = 0;

	ok_to_hrresult(
		unsafe {
			ffi::TaskDialogIndirect(
				task_config as *const _ as _,
				&mut pn_button,
				&mut pn_radio_button,
				verification_flag_checked.as_ref()
					.map_or(std::ptr::null_mut(), |_| &mut pf_bool),
			)
		},
	)?;

	if let Some(pf) = verification_flag_checked {
		*pf = pf_bool != 0;
	}
	Ok((unsafe { co::DLGID::from_raw(pn_button as _) }, pn_radio_button as _))
}
