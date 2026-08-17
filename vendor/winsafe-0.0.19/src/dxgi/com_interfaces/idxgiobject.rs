#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGIObject`](crate::IDXGIObject) virtual table.
#[repr(C)]
pub struct IDXGIObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub SetPrivateData: fn(COMPTR, PCVOID, u32, PCVOID) -> HRES,
	pub SetPrivateDataInterface: fn(COMPTR, PCVOID, COMPTR) -> HRES,
	pub GetPrivateData: fn(COMPTR, PCVOID, *mut u32, PVOID) -> HRES,
	pub GetParent: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
}

com_interface! { IDXGIObject: "aec22fb8-76f3-4639-9be0-28eb43a67a2e";
	/// [`IDXGIObject`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiobject)
	/// COM interface over [`IDXGIObjectVT`](crate::vt::IDXGIObjectVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIObject {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIObject`](crate::IDXGIObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIObject: ole_IUnknown {
	/// [`IDXGIObject::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-getparent)
	/// method.
	#[must_use]
	fn GetParent<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIObjectVT>(self).GetParent)(
					self.ptr(),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IDXGIObject::SetPrivateData`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedata)
	/// method.
	///
	/// Note: a copy of the data is made.
	fn SetPrivateData<T>(&self, name: &GUID, data: &T) -> HrResult<()>
		where T: Sized,
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIObjectVT>(self).SetPrivateData)(
					self.ptr(),
					name as *const _ as _,
					std::mem::size_of::<T>() as _,
					data as *const _ as _,
				)
			},
		)
	}

	/// [`IDXGIObject::SetPrivateDataInterface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedatainterface)
	/// method.
	fn SetPrivateDataInterface<T>(&self, obj: &T) -> HrResult<()>
		where T: ole_IUnknown,
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIObjectVT>(self).SetPrivateDataInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					obj.ptr(),
				)
			},
		)
	}
}
