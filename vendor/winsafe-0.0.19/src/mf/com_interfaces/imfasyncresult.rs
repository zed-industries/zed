#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFAsyncResult`](crate::IMFAsyncResult) virtual table.
#[repr(C)]
pub struct IMFAsyncResultVT {
	pub IUnknownVT: IUnknownVT,
	pub GetState: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetStatus: fn(COMPTR) -> HRES,
	pub SetStatus: fn(COMPTR, HRES) -> HRES,
	pub GetObject: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetStateNoAddRef: fn(COMPTR) -> COMPTR,
}

com_interface! { IMFAsyncResult: "ac6b7889-0740-4d51-8619-905994a55cc6";
	/// [`IMFAsyncResult`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfasyncresult)
	/// COM interface over
	/// [`IMFAsyncResultVT`](crate::vt::IMFAsyncResultVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAsyncResult for IMFAsyncResult {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFAsyncResult`](crate::IMFAsyncResult).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFAsyncResult: ole_IUnknown {
	/// [`IMFAsyncResult::GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasyncresult-getobject)
	/// method.
	#[must_use]
	fn GetObject<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAsyncResultVT>(self).GetObject)(
					self.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFAsyncResult::GetState`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasyncresult-getstate)
	/// method.
	#[must_use]
	fn GetState<T>(&self) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAsyncResultVT>(self).GetState)(
					self.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFAsyncResult::GetStateNoAddRef`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasyncresult-getstatenoaddref)
	/// method.
	#[must_use]
	fn GetStateNoAddRef<T>(&self) -> Option<ManuallyDrop<T>>
		where T: ole_IUnknown,
	{
		let ptr = unsafe {
			(vt::<IMFAsyncResultVT>(self).GetStateNoAddRef)(self.ptr())
		};

		if ptr.is_null() {
			None
		} else {
			Some(ManuallyDrop::new(unsafe { T::from_ptr(ptr) }))
		}
	}

	/// [`IMFAsyncResult::GetStatus`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasyncresult-getstatus)
	/// method.
	#[must_use]
	fn GetStatus(&self) -> co::HRESULT {
		unsafe {
			co::HRESULT::from_raw(
				(vt::<IMFAsyncResultVT>(self).GetStatus)(self.ptr()),
			)
		}
	}

	/// [`IMFAsyncResult::SetStatus`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasyncresult-setstatus)
	/// method.
	fn SetStatus(&self, status: co::HRESULT) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAsyncResultVT>(self).SetStatus)(self.ptr(), status.raw())
			},
		)
	}
}
