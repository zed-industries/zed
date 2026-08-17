#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`ITypeInfo`](crate::ITypeInfo) virtual table.
#[repr(C)]
pub struct ITypeInfoVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeAttr: fn(COMPTR, *mut PVOID) -> HRES,
	pub GetTypeComp: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetFuncDesc: fn(COMPTR, u32, *mut PVOID) -> HRES,
	pub GetVarDesc: fn(COMPTR, u32, *mut PVOID) -> HRES,
	pub GetNames: fn(COMPTR, i32, *mut PSTR, u32, *mut u32) -> HRES,
	pub GetRefTypeOfImplType: fn(COMPTR, u32, *mut u32) -> HRES,
	pub GetImplTypeFlags: fn(COMPTR, u32, *mut i32) -> HRES,
	pub GetIDsOfNames: fn(COMPTR, *mut PSTR, u32, *mut i32) -> HRES,
	pub Invoke: fn(COMPTR, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
	pub GetDocumentation: fn(COMPTR, i32, *mut PSTR, *mut PSTR, *mut u32, PSTR) -> HRES,
	pub GetDllEntry: fn(COMPTR, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRES,
	pub GetRefTypeInfo: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub AddressOfMember: fn(COMPTR, i32, u32, *mut PVOID) -> HRES,
	pub CreateInstance: fn(COMPTR, *mut COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub GetMops: fn(COMPTR, i32, *mut PSTR) -> HRES,
	pub GetContainingTypeLib: fn(COMPTR, *mut COMPTR, *mut u32) -> HRES,
	pub ReleaseTypeAttr: fn(COMPTR, PVOID) -> HRES,
	pub ReleaseFuncDesc: fn(COMPTR, PVOID) -> HRES,
	pub ReleaseVarDesc: fn(COMPTR, PVOID) -> HRES,
}

com_interface! { ITypeInfo: "00020401-0000-0000-c000-000000000046";
	/// [`ITypeInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
	/// COM interface over [`ITypeInfoVT`](crate::vt::ITypeInfoVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_ITypeInfo for ITypeInfo {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`ITypeInfo`](crate::ITypeInfo).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_ITypeInfo: ole_IUnknown {
	/// [`ITypeInfo::CreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
	/// method.
	#[must_use]
	fn CreateInstance<T>(&self, iunk_outer: Option<&mut IUnknown>) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let (mut queried, mut queried_outer) = unsafe {(
			T::null(),
			IUnknown::null(),
		)};

		ok_to_hrresult(
			unsafe {
				(vt::<ITypeInfoVT>(self).CreateInstance)(
					self.ptr(),
					iunk_outer.as_ref()
						.map_or(std::ptr::null_mut(), |_| queried_outer.as_mut()),
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
