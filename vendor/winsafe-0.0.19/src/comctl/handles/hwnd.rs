#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::ffi;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;

impl comctl_Hwnd for HWND {}

/// This trait is enabled with the `comctl` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_Hwnd: user_Hwnd {
	/// [`DefSubclassProc`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	fn DefSubclassProc<M>(&self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				ffi::DefSubclassProc(
					self.ptr(), wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`InitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-initializeflatsb)
	/// function.
	fn InitializeFlatSB(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ffi::InitializeFlatSB(self.ptr()) })
	}

	/// [`RemoveWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// function.
	fn RemoveWindowSubclass(&self,
		subclass_func: SUBCLASSPROC,
		subclass_id: usize,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::RemoveWindowSubclass(
					self.ptr(),
					subclass_func as _,
					subclass_id,
				)
			},
		)
	}

	/// [`SetWindowSubclass`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// function.
	///
	/// # Safety
	///
	/// You must provide a subclass procedure.
	unsafe fn SetWindowSubclass(&self,
		subclass_proc: SUBCLASSPROC,
		subclass_id: usize,
		ref_data: usize,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::SetWindowSubclass(
					self.ptr(),
					subclass_proc as _,
					subclass_id,
					ref_data,
				)
			},
		)
	}

	/// [`TaskDialog`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialog)
	/// function.
	///
	/// If you need more customization, see the
	/// [`TaskDialogIndirect`](crate::TaskDialogIndirect) function.
	///
	/// # Examples
	///
	/// An information message with just an OK button:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.TaskDialog(
	///     None,
	///     Some("Operation successful"),
	///     None,
	///     Some("The operation completed successfully."),
	///     co::TDCBF::OK,
	///     w::IconRes::Info,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	///
	/// Prompt the user to click OK or Cancel upon a question:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let answer = hwnd.TaskDialog(
	///     None,
	///     Some("My app name"),
	///     Some("File modified"),
	///     Some("The file has been modified.\nProceed closing the application?"),
	///     co::TDCBF::OK | co::TDCBF::CANCEL,
	///     w::IconRes::Warn,
	/// )?;
	///
	/// if answer == co::DLGID::OK {
	///     println!("User clicked OK.");
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	fn TaskDialog(&self,
		hinstance: Option<&HINSTANCE>,
		window_title: Option<&str>,
		main_instruction: Option<&str>,
		content: Option<&str>,
		common_buttons: co::TDCBF,
		icon: IconRes,
	) -> HrResult<co::DLGID>
	{
		// https://weblogs.asp.net/kennykerr/Windows-Vista-for-Developers-_1320_-Part-2-_1320_-Task-Dialogs-in-Depth
		let mut pn_button = i32::default();
		ok_to_hrresult(
			unsafe {
				ffi::TaskDialog(
					self.ptr(),
					hinstance.map_or(std::ptr::null_mut(), |h| h.ptr()),
					WString::from_opt_str(window_title).as_ptr(),
					WString::from_opt_str(main_instruction).as_ptr(),
					WString::from_opt_str(content).as_ptr(),
					common_buttons.raw(),
					icon.as_ptr(),
					&mut pn_button,
				)
			},
		).map(|_| unsafe { co::DLGID::from_raw(pn_button as _) })
	}

	/// [`UninitializeFlatSB`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-uninitializeflatsb)
	/// function.
	fn UninitializeFlatSB(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { ffi::UninitializeFlatSB(self.ptr()) })
	}
}
