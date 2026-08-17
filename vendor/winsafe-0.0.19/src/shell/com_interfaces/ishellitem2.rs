#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IShellItem2`](crate::IShellItem2) virtual table.
#[repr(C)]
pub struct IShellItem2VT {
	pub IShellItemVT: IShellItemVT,
	pub GetPropertyStore: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreWithCreateObject: fn(COMPTR, u32, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyStoreForKeys: fn(COMPTR, PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES,
	pub GetPropertyDescriptionList: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub Update: fn(COMPTR, COMPTR) -> HRES,
	pub GetProperty: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetCLSID: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetFileTime: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetInt32: fn(COMPTR, PCVOID, *mut i32) -> HRES,
	pub GetString: fn(COMPTR, PCVOID, *mut PSTR) -> HRES,
	pub GetUInt32: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetUInt64: fn(COMPTR, PCVOID, *mut u64) -> HRES,
	pub GetBool: fn(COMPTR, PCVOID, *mut BOOL) -> HRES,
}

com_interface! { IShellItem2: "7e9fb0d3-919f-4307-ab2e-9b1860310c93";
	/// [`IShellItem2`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem2)
	/// COM interface over [`IShellItem2VT`](crate::vt::IShellItem2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem2>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl shell_IShellItem for IShellItem2 {}
impl shell_IShellItem2 for IShellItem2 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem2`](crate::IShellItem2).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem2: shell_IShellItem {
	/// [`IShellItem2::GetBool`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getbool)
	/// method.
	#[must_use]
	fn GetBool(&self, key: &PROPERTYKEY) -> HrResult<bool> {
		let mut f: BOOL = 0;
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetBool)(
					self.ptr(),
					key as *const _ as _,
					&mut f,
				)
			},
		).map(|_| f != 0)
	}

	/// [`IShellItem2::GetFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getfiletime)
	/// method.
	#[must_use]
	fn GetFileTime(&self, key: &PROPERTYKEY) -> HrResult<FILETIME> {
		let mut ft = FILETIME::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetFileTime)(
					self.ptr(),
					key as *const _ as _,
					&mut ft as *mut _ as _,
				)
			},
		).map(|_| ft)
	}

	/// [`IShellItem2::GetInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getint32)
	/// method.
	#[must_use]
	fn GetInt32(&self, key: &PROPERTYKEY) -> HrResult<i32> {
		let mut i = i32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetInt32)(
					self.ptr(),
					key as *const _ as _,
					&mut i,
				)
			},
		).map(|_| i)
	}

	/// [`IShellItem2::GetPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getpropertystore)
	/// method.
	#[must_use]
	fn GetPropertyStore(&self, flags: co::GPS) -> HrResult<IPropertyStore> {
		let mut queried = unsafe { IPropertyStore::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetPropertyStore)(
					self.ptr(),
					flags.raw(),
					&IPropertyStore::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IShellItem2::GetUInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint32)
	/// method.
	#[must_use]
	fn GetUInt32(&self, key: &PROPERTYKEY) -> HrResult<u32> {
		let mut ui = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetUInt32)(
					self.ptr(),
					key as *const _ as _,
					&mut ui,
				)
			},
		).map(|_| ui)
	}

	/// [`IShellItem2::GetUInt64`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint64)
	/// method.
	#[must_use]
	fn GetUInt64(&self, key: &PROPERTYKEY) -> HrResult<u64> {
		let mut ull = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetUInt64)(
					self.ptr(),
					key as *const _ as _,
					&mut ull,
				)
			},
		).map(|_| ull)
	}

	/// [`IShellItem2::Update`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-update)
	/// method.
	fn Update(&self, pbc: &impl ole_IBindCtx) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IShellItem2VT>(self).Update)(self.ptr(), pbc.ptr()) },
		)
	}
}
