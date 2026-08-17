#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IModalWindow`](crate::IModalWindow) virtual table.
#[repr(C)]
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(COMPTR, HANDLE) -> u32,
}

com_interface! { IModalWindow: "b4db1657-70d7-485e-8e3e-6fcb5a5c1802";
	/// [`IModalWindow`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
	/// COM interface over [`IModalWindowVT`](crate::vt::IModalWindowVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IModalWindow for IModalWindow {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IModalWindow`](crate::IModalWindow).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IModalWindow: ole_IUnknown {
	/// [`IModalWindow::Show`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
	/// method.
	///
	/// Returns false if user clicked Cancel.
	fn Show(&self, hwnd_owner: &HWND) -> HrResult<bool> {
		const CANCELLED: co::HRESULT = co::ERROR::CANCELLED.to_hresult();
		match unsafe {
			co::HRESULT::from_raw(
				(vt::<IModalWindowVT>(self).Show)(
					self.ptr(),
					hwnd_owner.ptr(),
				),
			)
		} {
			co::HRESULT::S_OK => Ok(true),
			CANCELLED => Ok(false),
			e => Err(e),
		}
	}
}
