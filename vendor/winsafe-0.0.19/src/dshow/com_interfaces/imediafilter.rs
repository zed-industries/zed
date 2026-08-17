#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi_types::*, privs::*};
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMediaFilter`](crate::IMediaFilter) virtual table.
#[repr(C)]
pub struct IMediaFilterVT {
	pub IPersistVT: IPersistVT,
	pub Stop: fn(COMPTR) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
   pub Run: fn(COMPTR, i64) -> HRES,
	pub GetState: fn(COMPTR, u32, *mut u32) -> HRES,
	pub SetSyncSource: fn(COMPTR, COMPTR) -> HRES,
	pub GetSyncSource: fn(COMPTR, *mut COMPTR) -> HRES,
}

com_interface! { IMediaFilter: "56a86899-0ad4-11ce-b03a-0020af0ba770";
	/// [`IMediaFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediafilter)
	/// COM interface over [`IMediaFilterVT`](crate::vt::IMediaFilterVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IMediaFilter {}
impl dshow_IMediaFilter for IMediaFilter {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaFilter`](crate::IMediaFilter).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IMediaFilter: ole_IPersist {
	/// [`IMediaFilter::GetState`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-getstate)
	/// method.
	#[must_use]
	fn GetState(&self, ms_timeout: Option<u32>) -> HrResult<co::FILTER_STATE> {
		let mut fs = co::FILTER_STATE::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaFilterVT>(self).GetState)(
					self.ptr(),
					ms_timeout.unwrap_or(INFINITE),
					fs.as_mut(),
				)
			},
		).map(|_| fs)
	}

	/// [`IMediaFilter::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-pause)
	/// method.
	fn Pause(&self) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IMediaFilterVT>(self).Pause)(self.ptr()) },
		)
	}

	/// [`IMediaFilter::Run`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-run)
	/// method.
	fn Run(&self, start: i64) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IMediaFilterVT>(self).Run)(self.ptr(), start) },
		)
	}

	/// [`IMediaFilter::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediafilter-stop)
	/// method.
	fn Stop(&self) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe { (vt::<IMediaFilterVT>(self).Stop)(self.ptr()) },
		)
	}
}
