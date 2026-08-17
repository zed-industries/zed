#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFGetService`](crate::IMFGetService) virtual table.
#[repr(C)]
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
}

com_interface! { IMFGetService: "fa993888-4383-415a-a930-dd472a8cf6f7";
	/// [`IMFGetService`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
	/// COM interface over [`IMFGetServiceVT`](crate::vt::IMFGetServiceVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let vmr: w::IBaseFilter; // initialized somewhere
	/// # let vmr = unsafe { w::IBaseFilter::null() };
	///
	/// let get_svc = vmr.QueryInterface::<w::IMFGetService>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFGetService for IMFGetService {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFGetService`](crate::IMFGetService).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFGetService: ole_IUnknown {
	/// [`IMFGetService::GetService`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let get_svc: w::IMFGetService; // initialized somewhere
	/// # let get_svc = unsafe { w::IMFGetService::null() };
	///
	/// let controller_evr = get_svc
	///     .GetService::<w::IMFVideoDisplayControl>(
	///         &co::MF_SERVICE::MR_VIDEO_RENDER_SERVICE,
	///     )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn GetService<T>(&self, service_id: &co::MF_SERVICE) -> HrResult<T>
		where T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFGetServiceVT>(self).GetService)(
					self.ptr(),
					service_id as *const _ as _,
					&T::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}
}
