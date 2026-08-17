#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFMediaSession`](crate::IMFMediaSession) virtual table.
#[repr(C)]
pub struct IMFMediaSessionVT {
	pub IMFMediaEventGeneratorVT: IMFMediaEventGeneratorVT,
	pub SetTopology: fn(COMPTR, u32, COMPTR) -> HRES,
	pub ClearTopologies: fn(COMPTR) -> HRES,
	pub Start: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub Close: fn(COMPTR) -> HRES,
	pub Shutdown: fn(COMPTR) -> HRES,
	pub GetClock: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetSessionCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub GetFullTopology: fn(COMPTR, u32, u64, *mut COMPTR) -> HRES,
}

com_interface! { IMFMediaSession: "90377834-21d0-4dee-8214-ba2e3e6c1127";
	/// [`IMFMediaSession`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfmediasession)
	/// COM interface over [`IMFMediaSessionVT`](crate::vt::IMFMediaSessionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let media_session = w::MFCreateMediaSession(None::<&w::IMFAttributes>)?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl mf_IMFMediaEventGenerator for IMFMediaSession {}
impl mf_IMFMediaSession for IMFMediaSession {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaSession`](crate::IMFMediaSession).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaSession: mf_IMFMediaEventGenerator {
	fn_com_noparm! { ClearTopologies: IMFMediaSessionVT;
		/// [`IMFMediaSession::ClearTopologies`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-cleartopologies)
		/// method.
	}

	fn_com_noparm! { Close: IMFMediaSessionVT;
		/// [`IMFMediaSession::Close`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-close)
		/// method.
	}

	fn_com_interface_get! { GetClock: IMFMediaSessionVT, IMFClock;
		/// [`IMFMediaSession::GetClock`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-getclock)
		/// method.
	}

	/// [`IMFMediaSession::GetFullTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-getfulltopology)
	/// method.
	#[must_use]
	fn GetFullTopology(&self,
		flags: co::MFSESSION_GETFULLTOPOLOGY,
		topo_id: u64,
	) -> HrResult<IMFTopology>
	{
		let mut queried = unsafe { IMFTopology::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).GetFullTopology)(
					self.ptr(),
					flags.raw(),
					topo_id,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFMediaSession::GetSessionCapabilities`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-getsessioncapabilities)
	/// method.
	#[must_use]
	fn GetSessionCapabilities(&self) -> HrResult<co::MFSESSIONCAP> {
		let mut caps = co::MFSESSIONCAP::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).GetSessionCapabilities)(
					self.ptr(),
					caps.as_mut(),
				)
			},
		).map(|_| caps)
	}

	fn_com_noparm! { Pause: IMFMediaSessionVT;
		/// [`IMFMediaSession::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-pause)
		/// method.
	}

	/// [`IMFMediaSession::SetTopology`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-settopology)
	/// method.
	fn SetTopology(&self,
		flags: co::MFSESSION_SETTOPOLOGY,
		topology: &impl mf_IMFTopology,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).SetTopology)(
					self.ptr(),
					flags.raw(),
					topology.ptr(),
				)
			},
		)
	}

	fn_com_noparm! { Shutdown: IMFMediaSessionVT;
		/// [`IMFMediaSession::Shutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-shutdown)
		/// method.
	}

	/// [`IMFMediaSession::Start`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-start)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let session: w::IMFMediaSession; // initialized somewhere
	/// # let session = unsafe { w::IMFMediaSession::null() };
	///
	/// session.Start(
	///     co::MF_TIME_FORMAT::NULL,
	///     &w::PROPVARIANT::default(),
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	fn Start(&self,
		time_format: co::MF_TIME_FORMAT,
		start_position: &PROPVARIANT,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSessionVT>(self).Start)(
					self.ptr(),
					&time_format as *const _ as _,
					start_position as *const _ as _,
				)
			},
		)
	}

	fn_com_noparm! { Stop: IMFMediaSessionVT;
		/// [`IMFMediaSession::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasession-stop)
		/// method.
	}
}
