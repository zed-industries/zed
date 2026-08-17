#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFileDialog`](crate::IFileDialog) virtual table.
#[repr(C)]
pub struct IFileDialogVT {
	pub IModalWindowVT: IModalWindowVT,
	pub SetFileTypes: fn(COMPTR, u32, PCVOID) -> HRES,
	pub SetFileTypeIndex: fn(COMPTR, u32) -> HRES,
	pub GetFileTypeIndex: fn(COMPTR, *mut u32) -> HRES,
	pub Advise: fn(COMPTR, PVOID, *mut u32) -> HRES,
	pub Unadvise: fn(COMPTR, u32) -> HRES,
	pub SetOptions: fn(COMPTR, u32) -> HRES,
	pub GetOptions: fn(COMPTR, *mut u32) -> HRES,
	pub SetDefaultFolder: fn(COMPTR, COMPTR) -> HRES,
	pub SetFolder: fn(COMPTR, COMPTR) -> HRES,
	pub GetFolder: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetCurrentSelection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub SetFileName: fn(COMPTR, PCSTR) -> HRES,
	pub GetFileName: fn(COMPTR, *mut PSTR) -> HRES,
	pub SetTitle: fn(COMPTR, PCSTR) -> HRES,
	pub SetOkButtonLabel: fn(COMPTR, PCSTR) -> HRES,
	pub SetFileNameLabel: fn(COMPTR, PCSTR) -> HRES,
	pub GetResult: fn(COMPTR, *mut COMPTR) -> HRES,
	pub AddPlace: fn(COMPTR, COMPTR, u32) -> HRES,
	pub SetDefaultExtension: fn(COMPTR, PCSTR) -> HRES,
	pub Close: fn(COMPTR, HRES) -> HRES,
	pub SetClientGuid: fn(COMPTR, PCVOID) -> HRES,
	pub ClearClientData: fn(COMPTR) -> HRES,
	pub SetFilter: fn(COMPTR, PVOID) -> HRES,
}

com_interface! { IFileDialog: "42f85136-db7e-439c-85f1-e4075d135fc8";
	/// [`IFileDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
	/// COM interface over [`IFileDialogVT`](crate::vt::IFileDialogVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IModalWindow for IFileDialog {}
impl shell_IFileDialog for IFileDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileDialog`](crate::IFileDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileDialog: shell_IModalWindow {
	/// [`IFileDialog::AddPlace`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-addplace)
	/// method.
	fn AddPlace(&self,
		si: &impl shell_IShellItem,
		fdap: co::FDAP,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).AddPlace)(
					self.ptr(),
					si.ptr(),
					fdap.raw(),
				)
			},
		)
	}

	/// [`IFileDialog::Advise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-advise)
	/// method.
	fn Advise(&self, fde: &impl shell_IFileDialogEvents) -> HrResult<u32> {
		let mut cookie = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).Advise)(
					self.ptr(),
					fde.ptr(),
					&mut cookie,
				)
			},
		).map(|_| cookie)
	}

	fn_com_noparm! { ClearClientData: IFileDialogVT;
		/// [`IFileDialog::ClearClientData`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
		/// method.
	}

	/// [`IFileDialog::Close`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	fn Close(&self, hr: co::ERROR) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).Close)(self.ptr(), hr.raw() as _)
			},
		)
	}

	fn_com_interface_get! { GetCurrentSelection: IFileDialogVT, IShellItem;
		/// [`IFileDialog::GetCurrentSelection`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
		/// method.
	}

	/// [`IFileDialog::GetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
	/// method.
	#[must_use]
	fn GetFileName(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).GetFileName)(self.ptr(), &mut pstr)
			},
		).map(|_| {
			let name = unsafe { WString::from_wchars_nullt(pstr) };
			let _ = unsafe { CoTaskMemFreeGuard::new(pstr as _, 0) };
			name.to_string()
		})
	}

	/// [`IFileDialog::GetFileTypeIndex`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
	/// method.
	#[must_use]
	fn GetFileTypeIndex(&self) -> HrResult<u32> {
		let mut index = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).GetFileTypeIndex)(
					self.ptr(),
					&mut index,
				)
			},
		).map(|_| index)
	}

	fn_com_interface_get! { GetFolder: IFileDialogVT, IShellItem;
		/// [`IFileDialog::GetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
		/// method.
	}

	/// [`IFileDialog::GetOptions`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
	/// method.
	#[must_use]
	fn GetOptions(&self) -> HrResult<co::FOS> {
		let mut opts = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).GetOptions)(self.ptr(), &mut opts)
			},
		).map(|_| unsafe { co::FOS::from_raw(opts) })
	}

	fn_com_interface_get! { GetResult: IFileDialogVT, IShellItem;
		/// [`IFileDialog::GetResult`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
		/// method.
		///
		/// If you chose a single file, this is the method to retrieve its path.
	}

	/// [`IFileDialog::SetClientGuid`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
	/// method.
	fn SetClientGuid(&self, guid: &GUID) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetClientGuid)(
					self.ptr(),
					guid as *const _ as _,
				)
			},
		)
	}

	/// [`IFileDialog::SetDefaultExtension`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
	/// method.
	fn SetDefaultExtension(&self, default_extension: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetDefaultExtension)(
					self.ptr(),
					WString::from_str(default_extension).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetDefaultFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
	/// method.
	fn SetDefaultFolder(&self, si: &impl shell_IShellItem) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetDefaultFolder)(
					self.ptr(),
					si.ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
	/// method.
	fn SetFileName(&self, name: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetFileName)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetFileNameLabel`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
	/// method.
	fn SetFileNameLabel(&self, label: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetFileNameLabel)(
					self.ptr(),
					WString::from_str(label).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetFileTypeIndex`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
	/// method.
	///
	/// **Note:** The index is one-based.
	fn SetFileTypeIndex(&self, index: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetFileTypeIndex)(self.ptr(), index)
			},
		)
	}

	/// [`IFileDialog::SetFileTypes`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let file_dlg: w::IFileDialog; // initialized somewhere
	/// # let file_dlg = unsafe { w::IFileDialog::null() };
	///
	/// file_dlg.SetFileTypes(&[
	///     ("Documents", "*.docx;*.txt"),
	///     ("Images", "*.jpg;*.png;*.bmp"),
	///     ("All files", "*.*"),
	/// ])?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn SetFileTypes<S: AsRef<str>>(&self,
		filter_spec: &[(S, S)],
	) -> HrResult<()>
	{
		let mut names_buf = Vec::with_capacity(filter_spec.len());
		let mut specs_buf = Vec::with_capacity(filter_spec.len());
		let mut com_dlgs = Vec::with_capacity(filter_spec.len());

		for (name, spec) in filter_spec.iter() {
			names_buf.push(WString::from_str(name.as_ref()));
			specs_buf.push(WString::from_str(spec.as_ref()));
			com_dlgs.push(COMDLG_FILTERSPEC::default());
		}

		names_buf.iter_mut().enumerate()
			.for_each(|(i, el)| com_dlgs[i].set_pszName(Some(el)));

		specs_buf.iter_mut().enumerate()
			.for_each(|(i, el)| com_dlgs[i].set_pszSpec(Some(el)));

		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetFileTypes)(
					self.ptr(),
					filter_spec.len() as _,
					com_dlgs.as_ptr() as _,
				)
			},
		)
	}

	/// [`IFileDialog::SetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
	/// method.
	fn SetFolder(&self, si: &impl shell_IShellItem) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IFileDialogVT>(self).SetFolder)(self.ptr(), si.ptr()) },
		)
	}

	/// [`IFileDialog::SetOkButtonLabel`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
	/// method.
	fn SetOkButtonLabel(&self, text: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetOkButtonLabel)(
					self.ptr(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetOptions`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
	/// method.
	fn SetOptions(&self, opts: co::FOS) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetOptions)(self.ptr(), opts.raw())
			},
		)
	}

	/// [`IFileDialog::SetTitle`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
	/// method.
	fn SetTitle(&self, text: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFileDialogVT>(self).SetTitle)(
					self.ptr(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::Unadvise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-unadvise)
	/// method.
	fn Unadvise(&self, cookie: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IFileDialogVT>(self).Unadvise)(self.ptr(), cookie) },
		)
	}
}
