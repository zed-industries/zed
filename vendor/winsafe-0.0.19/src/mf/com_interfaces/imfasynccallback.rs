#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFAsyncCallback`](crate::IMFAsyncCallback) virtual table.
#[repr(C)]
pub struct IMFAsyncCallbackVT {
	pub IUnknownVT: IUnknownVT,
	pub GetParameters: fn(COMPTR, *mut u32, *mut u32) -> HRES,
	pub Invoke: fn(COMPTR, COMPTR) -> HRES,
}

com_interface! { IMFAsyncCallback: "a27003cf-2354-4f2a-8d6a-ab7cff15437e";
	/// [`IMFAsyncCallback`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfasynccallback)
	/// COM interface over
	/// [`IMFAsyncCallbackVT`](crate::vt::IMFAsyncCallbackVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFAsyncCallback for IMFAsyncCallback {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFAsyncCallback`](crate::IMFAsyncCallback).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFAsyncCallback: ole_IUnknown {
	/// [`IMFAsyncCallback::GetParameters`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasynccallback-getparameters)
	/// method.
	///
	/// Returns the flag and the ID of the work queue.
	#[must_use]
	fn GetParameters(&self) -> HrResult<(co::MFASYNC, u32)> {
		let (mut flags, mut queue) = (co::MFASYNC::default(), u32::default());
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAsyncCallbackVT>(self).GetParameters)(
					self.ptr(),
					flags.as_mut(),
					&mut queue,
				)
			},
		).map(|_| (flags, queue))
	}

	/// [`IMFAsyncCallback::Invoke`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfasynccallback-invoke)
	/// method.
	fn Invoke(&self, async_result: &impl mf_IMFAsyncResult) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMFAsyncCallbackVT>(self).Invoke)(
					self.ptr(),
					async_result.ptr(),
				)
			},
		)
	}
}
