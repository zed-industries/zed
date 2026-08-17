#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::privs::*;
use crate::vt::*;

/// [`IShellLink`](crate::IShellLink) virtual table.
#[repr(C)]
pub struct IShellLinkVT {
	pub IUnknownVT: IUnknownVT,
	pub GetPath: fn(COMPTR, PCSTR, i32, PVOID, u32) -> HRES,
	pub GetIDList: fn(COMPTR, PVOID) -> HRES,
	pub SetIDList: fn(COMPTR, PVOID) -> HRES,
	pub GetDescription: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetDescription: fn(COMPTR, PCSTR) -> HRES,
	pub GetWorkingDirectory: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetWorkingDirectory: fn(COMPTR, PCSTR) -> HRES,
	pub GetArguments: fn(COMPTR, PSTR, i32) -> HRES,
	pub SetArguments: fn(COMPTR, PCSTR) -> HRES,
	pub GetHotkey: fn(COMPTR, *mut u16) -> HRES,
	pub SetHotkey: fn(COMPTR, u16) -> HRES,
	pub GetShowCmd: fn(COMPTR, *mut i32) -> HRES,
	pub SetShowCmd: fn(COMPTR, i32) -> HRES,
	pub GetIconLocation: fn(COMPTR, PSTR, i32, *mut i32) -> HRES,
	pub SetIconLocation: fn(COMPTR, PCSTR, i32) -> HRES,
	pub SetRelativePath: fn(COMPTR, PCSTR, u32) -> HRES,
	pub Resolve: fn(COMPTR, HANDLE, u32) -> HRES,
	pub SetPath: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IShellLink: "000214f9-0000-0000-c000-000000000046";
	/// [`IShellLink`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw)
	/// COM interface over [`IShellLinkVT`](crate::vt::IShellLinkVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let obj = w::CoCreateInstance::<w::IShellLink>(
	///     &co::CLSID::ShellLink,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
}

impl shell_IShellLink for IShellLink {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellLink`](crate::IShellLink).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellLink: ole_IUnknown {
	/// [`IShellLink::GetArguments`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getarguments)
	/// method.
	#[must_use]
	fn GetArguments(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(INFOTIPSIZE + 1); // arbitrary
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetArguments)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`IShellLink::GetDescription`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getdescription)
	/// method.
	#[must_use]
	fn GetDescription(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(INFOTIPSIZE + 1);
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetDescription)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`IShellLink::GetIconLocation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-geticonlocation)
	/// method.
	///
	/// Returns the path of the icon and its index within the file.
	#[must_use]
	fn GetIconLocation(&self) -> HrResult<(String, i32)> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut index: i32 = 0;

		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetIconLocation)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					&mut index,
				)
			},
		).map(|_| (buf.to_string(), index))
	}

	/// [`IShellLink::GetPath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getpath)
	/// method.
	#[must_use]
	fn GetPath(&self,
		fd: Option<&mut WIN32_FIND_DATA>,
		flags: co::SLGP,
	) -> HrResult<String>
	{
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetPath)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
					fd.map_or(std::ptr::null_mut(), |fd| fd as *mut _ as _),
					flags.raw(),
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`IShellLink::GetShowCmd`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getshowcmd)
	/// method.
	#[must_use]
	fn GetShowCmd(&self) -> HrResult<co::SW> {
		let mut show_cmd = co::SW::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetShowCmd)(
					self.ptr(),
					show_cmd.as_mut(),
				)
			},
		).map(|_| show_cmd)
	}

	/// [`IShellLink::GetWorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-getworkingdirectory)
	/// method.
	#[must_use]
	fn GetWorkingDirectory(&self) -> HrResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).GetWorkingDirectory)(
					self.ptr(),
					buf.as_mut_ptr(),
					buf.buf_len() as _,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`IShellLink::Resolve`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-resolve)
	/// method.
	fn Resolve(&self, hwnd: &HWND, flags: co::SLR) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).Resolve)(
					self.ptr(),
					hwnd.ptr(),
					flags.raw(),
				)
			},
		)
	}

	/// [`IShellLink::SetArguments`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setarguments)
	/// method.
	fn SetArguments(&self, args: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetArguments)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				)
			},
		)
	}

	/// [`IShellLink::SetDescription`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setdescription)
	/// method.
	fn SetDescription(&self, args: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetDescription)(
					self.ptr(),
					WString::from_str(args).as_ptr(),
				)
			},
		)
	}

	/// [`IShellLink::SetIconLocation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-seticonlocation)
	/// method.
	fn SetIconLocation(&self, path: &str, index: i32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetIconLocation)(
					self.ptr(),
					WString::from_str(path).as_ptr(),
					index,
				)
			},
		)
	}

	/// [`IShellLink::SetPath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setpath)
	/// method.
	fn SetPath(&self, file: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetPath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
				)
			},
		)
	}

	/// [`IShellLink::SetRelativePath`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setrelativepath)
	/// method.
	fn SetRelativePath(&self, file: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetRelativePath)(
					self.ptr(),
					WString::from_str(file).as_ptr(),
					0,
				)
			},
		)
	}

	/// [`IShellLink::SetShowCmd`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setshowcmd)
	/// method.
	fn SetShowCmd(&self, show_cmd: co::SW) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetShowCmd)(self.ptr(), show_cmd.raw())
			},
		)
	}

	/// [`IShellLink::SetWorkingDirectory`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishelllinkw-setworkingdirectory)
	/// method.
	fn SetWorkingDirectory(&self, dir: &str) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IShellLinkVT>(self).SetWorkingDirectory)(
					self.ptr(),
					WString::from_str(dir).as_ptr(),
				)
			},
		)
	}
}
