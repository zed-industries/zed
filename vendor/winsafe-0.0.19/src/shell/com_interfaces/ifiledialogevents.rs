#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFileDialogEvents`](crate::IFileDialogEvents) virtual table.
#[repr(C)]
pub struct IFileDialogEventsVT {
	pub IUnknownVT: IUnknownVT,
	pub OnFileOk: fn(COMPTR, COMPTR) -> HRES,
	pub OnFolderChanging: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub OnFolderChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnSelectionChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnShareViolation: fn(COMPTR, COMPTR, *mut u32) -> HRES,
	pub OnTypeChange: fn(COMPTR, COMPTR) -> HRES,
	pub OnOverwrite: fn(COMPTR, COMPTR, *mut u32) -> HRES,
}

com_interface! { IFileDialogEvents: "973510db-7d7f-452b-8975-74a85828d354";
	/// [`IFileDialogEvents`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialogevents)
	/// COM interface over [`IFileDialogEventsVT`](crate::vt::IFileDialogEventsVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IFileDialogEvents for IFileDialogEvents {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileDialogEvents`](crate::IFileDialogEvents).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileDialogEvents: ole_IUnknown {

}
