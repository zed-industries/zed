#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::prelude::*;
use crate::shell::{ffi, iterators::*};

impl_handle! { HDROP;
	/// Handle to an
	/// [internal drop structure](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
}

impl shell_Hdrop for HDROP {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`HDROP`](crate::HDROP).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_Hdrop: Handle {
	/// [`DragQueryFile`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// function.
	///
	/// Returns an iterator over the dropped files. After the iterator is
	/// consumed, calls
	/// [`DragFinish`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish)
	/// and invalidates the `HDROP` handle, so further calls will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// # Examples
	///
	/// Iterating over the strings:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let mut hdrop: w::HDROP; // initialized somewhere
	/// # let mut hdrop = w::HDROP::NULL;
	///
	/// for file_path in hdrop.DragQueryFile()? {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Collecting the strings into a [`Vec`](std::vec::Vec):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let mut hdrop: w::HDROP; // initialized somewhere
	/// # let mut hdrop = w::HDROP::NULL;
	///
	/// let file_paths = hdrop.DragQueryFile()?
	///     .collect::<w::SysResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn DragQueryFile(&mut self,
	) -> SysResult<Box<dyn Iterator<Item = SysResult<String>> + '_>> {
		Ok(Box::new(HdropIter::new(self)?))
	}

	/// [`DragQueryPoint`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// function.
	///
	/// Returns the coordinates and whether the drop occurred in the client area
	/// of the window.
	///
	/// Note that you must call this method before
	/// [`DragQueryFile`](crate::prelude::shell_Hdrop::DragQueryFile), because
	/// it invalidates the `HDROP` handle after the iterator is consumed.
	#[must_use]
	fn DragQueryPoint(&self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let client_area = unsafe {
			ffi::DragQueryPoint(self.ptr(), &mut pt as *mut _ as _)
		};
		(pt, client_area != 0)
	}
}
