#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFAttributes`](crate::IMFAttributes) virtual table.
#[repr(C)]
pub struct IMFAttributesVT {
	pub IUnknownVT: IUnknownVT,
	pub GetItem: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetItemType: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub CompareItem: fn(COMPTR, PCVOID, PCVOID, *mut BOOL) -> HRES,
	pub Compare: fn(COMPTR, COMPTR, u32, *mut BOOL) -> HRES,
	pub GetUINT32: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetUINT64: fn(COMPTR, PCVOID, *mut u64) -> HRES,
	pub GetDouble: fn(COMPTR, PCVOID, *mut f64) -> HRES,
	pub GetGUID: fn(COMPTR, COMPTR, PVOID) -> HRES,
	pub GetStringLength: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetString: fn(COMPTR, PCVOID, PSTR, u32, *mut u32) -> HRES,
	pub GetAllocatedString: fn(COMPTR, PCVOID, PSTR, *mut u32) -> HRES,
	pub GetBlobSize: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetBlob: fn(COMPTR, PCVOID, *mut u8, u32, *mut u32) -> HRES,
	pub GetAllocatedBlob: fn(COMPTR, PCVOID, *mut *mut u8, *mut u32) -> HRES,
	pub GetUnknown: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub SetItem: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub DeleteItem: fn(COMPTR, PCVOID) -> HRES,
	pub DeleteAllItems: fn(COMPTR) -> HRES,
	pub SetUINT32: fn(COMPTR, PCVOID, u32) -> HRES,
	pub SetUINT64: fn(COMPTR, PCVOID, u64) -> HRES,
	pub SetDouble: fn(COMPTR, PCVOID, f64) -> HRES,
	pub SetGUID: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub SetString: fn(COMPTR, PCVOID, PCSTR) -> HRES,
	pub SetBlob: fn(COMPTR, PCVOID, *const u8, u32) -> HRES,
	pub SetUnknown: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub LockStore: fn(COMPTR) -> HRES,
	pub UnlockStore: fn(COMPTR) -> HRES,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetItemByIndex: fn(COMPTR, u32, PVOID, PVOID) -> HRES,
	pub CopyAllItems: fn(COMPTR, COMPTR) -> HRES,
}

com_interface! { IMFAttributes: "2cd2d921-c447-44a7-a13c-4adabfc247e3";
	/// [`IMFAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfattributes)
	/// COM interface over
	/// [`IMFAttributesVT`](crate::vt::IMFAttributesVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAttributes for IMFAttributes {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFAttributes`](crate::IMFAttributes).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFAttributes: ole_IUnknown {
	/// [`IMFAttributes::CopyAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-copyallitems)
	/// method.
	fn CopyAllItems(&self, dest: &impl mf_IMFAttributes) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).CopyAllItems)(self.ptr(), dest.ptr())
			},
		)
	}

	fn_com_noparm! { DeleteAllItems: IMFAttributesVT;
		/// [`IMFAttributes::DeleteAllItems`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteallitems)
		/// method.
	}

	/// [`IMFAttributes::DeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-deleteitem)
	/// method.
	fn DeleteItem(&self, guid_key: &GUID) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).DeleteItem)(
					self.ptr(),
					guid_key as *const _ as _,
				)
			},
		)
	}

	/// [`IMFAttributes::GetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint32)
	/// method.
	fn GetUINT32(&self, guid_key: &GUID) -> HrResult<u32> {
		let mut value = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetUINT32)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut value,
				)
			},
		).map(|_| value)
	}

	/// [`IMFAttributes::GetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-getuint64)
	/// method.
	fn GetUINT64(&self, guid_key: &GUID) -> HrResult<u64> {
		let mut value = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).GetUINT64)(
					self.ptr(),
					guid_key as *const _ as _,
					&mut value,
				)
			},
		).map(|_| value)
	}

	/// [`IMFAttributes::SetUINT32`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint32)
	/// method.
	fn SetUINT32(&self, guid_key: &GUID, value: u32) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).SetUINT32)(
					self.ptr(),
					guid_key as *const _ as _,
					value,
				)
			},
		)
	}

	/// [`IMFAttributes::SetUINT64`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfattributes-setuint64)
	/// method.
	fn SetUINT64(&self, guid_key: &GUID, value: u64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAttributesVT>(self).SetUINT64)(
					self.ptr(),
					guid_key as *const _ as _,
					value,
				)
			},
		)
	}
}
