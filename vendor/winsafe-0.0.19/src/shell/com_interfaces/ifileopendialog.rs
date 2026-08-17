#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFileOpenDialog`](crate::IFileOpenDialog) virtual table.
#[repr(C)]
pub struct IFileOpenDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub GetResults: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetSelectedItems: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IFileOpenDialog: "d57c7288-d4ad-4768-be02-9d969532d960";
	/// [`IFileOpenDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
	/// COM interface over [`IFileOpenDialogVT`](crate::vt::IFileOpenDialogVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Choosing a single existing TXT file:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hparent: w::HWND; // initialized somewhere
	/// # let hparent = w::HWND::NULL;
	///
	/// let file_open = w::CoCreateInstance::<w::IFileOpenDialog>(
	///     &co::CLSID::FileOpenDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	///
	/// file_open.SetOptions(
	///     file_open.GetOptions()?
	///     | co::FOS::FORCEFILESYSTEM
	///     | co::FOS::FILEMUSTEXIST,
	/// )?;
	///
	/// file_open.SetFileTypes(&[
	///     ("Text files", "*.txt"),
	///     ("All files", "*.*"),
	/// ])?;
	/// file_open.SetFileTypeIndex(1)?;
	///
	/// if file_open.Show(&hparent)? {
	///     let chosen_file = file_open.GetResult()?
	///         .GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///     println!("{}", chosen_file);
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_IModalWindow for IFileOpenDialog {}
impl shell_IFileDialog for IFileOpenDialog {}
impl shell_IFileOpenDialog for IFileOpenDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileOpenDialog`](crate::IFileOpenDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileOpenDialog: shell_IFileDialog {
	fn_com_interface_get! { GetResults: IFileOpenDialogVT, IShellItemArray;
		/// [`IFileOpenDialog::GetResults`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
		/// method.
		///
		/// If you chose multiple files, this is the method to retrieve the
		/// paths.
		///
		/// # Examples
		///
		/// Collecting the file paths into a [`Vec`](std::vec::Vec):
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co};
		///
		/// let fo: w::IFileOpenDialog; // initialized somewhere
		/// # let fo = unsafe { w::IFileOpenDialog::null() };
		///
		/// let paths = fo.GetResults()?.iter()?
		///     .map(|shi| {
		///         let shi = shi?;
		///         let name = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
		///         Ok(name)
		///     })
		///     .collect::<w::HrResult<Vec<_>>>()?;
		/// # Ok::<_, co::HRESULT>(())
		/// ```
	}

	fn_com_interface_get! { GetSelectedItems: IFileOpenDialogVT, IShellItemArray;
		/// [`IFileOpenDialog::GetSelectedItems`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
		/// method.
	}
}
