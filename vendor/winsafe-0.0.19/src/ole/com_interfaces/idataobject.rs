#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDataObject`](crate::IDataObject) virtual table.
#[repr(C)]
pub struct IDataObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub GetData: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub GetDataHere: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub QueryGetData: fn(COMPTR, PVOID) -> HRES,
	pub GetCanonicalFormatEtc: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub SetData: fn(COMPTR, PVOID, PVOID, BOOL) -> HRES,
	pub EnumFormatEtc: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub DAdvise: fn(COMPTR, PVOID, u32, COMPTR, *mut u32) -> HRES,
	pub DUnadvise: fn(COMPTR, u32) -> HRES,
	pub EnumDAdvise: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IDataObject: "0000010e-0000-0000-c000-000000000046";
	/// [`IDataObject`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-idataobject)
	/// COM interface over [`IDataObjectVT`](crate::vt::IDataObjectVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IDataObject for IDataObject {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IDataObject`](crate::IDataObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IDataObject: ole_IUnknown {
	/// [`IDataObject::DAdvise`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-idataobject-dadvise)
	/// method.
	fn DAdvise(&self,
		formatetc: &FORMATETC,
		advf: co::ADVF,
		adv_sink: &impl ole_IAdviseSink,
	) -> HrResult<u32>
	{
		let mut connection = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDataObjectVT>(self).DAdvise)(
					self.ptr(),
					formatetc as *const _ as _,
					advf.raw(),
					adv_sink.ptr(),
					&mut connection,
				)
			},
		).map(|_| connection)
	}

	/// [`IDataObject::DUnadvise`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-idataobject-dunadvise)
	/// method.
	fn DUnadvise(&self, connection: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDataObjectVT>(self).DUnadvise)(self.ptr(), connection)
			},
		)
	}

	/// [`IDataObject::QueryGetData`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-idataobject-querygetdata)
	/// method.
	fn QueryGetData(&self, formatetc: &FORMATETC) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDataObjectVT>(self).QueryGetData)(
					self.ptr(),
					formatetc as *const _ as _,
				)
			},
		)
	}
}
