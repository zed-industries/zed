#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFMediaEventGenerator`](crate::IMFMediaEventGenerator) virtual table.
#[repr(C)]
pub struct IMFMediaEventGeneratorVT {
	pub IUnknownVT: IUnknownVT,
	pub GetEvent: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub BeginGetEvent: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndGetEvent: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub QueueEvent: fn(COMPTR, u32, PCVOID, HRES, PCVOID) -> HRES,
}

com_interface! { IMFMediaEventGenerator: "2cd0bd52-bcd5-4b89-b62c-eadc0c031e7d";
	/// [`IMFMediaEventGenerator`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfmediaeventgenerator)
	/// COM interface over
	/// [`IMFMediaEventGeneratorVT`](crate::vt::IMFMediaEventGeneratorVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFMediaEventGenerator for IMFMediaEventGenerator {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaEventGenerator`](crate::IMFMediaEventGenerator).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaEventGenerator: ole_IUnknown {
	/// [`IMFMediaEventGenerator::BeginGetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-begingetevent)
	/// method.
	fn BeginGetEvent(&self,
		callback: &impl mf_IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaEventGeneratorVT>(self).BeginGetEvent)(
					self.ptr(),
					callback.ptr(),
					state.map_or(std::ptr::null_mut(), |s| s.ptr()),
				)
			},
		)
	}

	/// [`IMFMediaEventGenerator::EndGetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-endgetevent)
	/// method.
	#[must_use]
	fn EndGetEvent(&self,
		result: &impl mf_IMFAsyncResult,
	) -> HrResult<IMFMediaEvent>
	{
		let mut queried = unsafe { IMFMediaEvent::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaEventGeneratorVT>(self).EndGetEvent)(
					self.ptr(),
					result.ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFMediaEventGenerator::GetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-getevent)
	/// method.
	#[must_use]
	fn GetEvent(&self,
		flags: Option<co::MF_EVENT_FLAG>,
	) -> HrResult<IMFMediaEvent>
	{
		let mut queried = unsafe { IMFMediaEvent::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaEventGeneratorVT>(self).GetEvent)(
					self.ptr(),
					flags.unwrap_or_default().raw(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IMFMediaEventGenerator::QueueEvent`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfmediaeventgenerator-queueevent)
	/// method.
	fn QueueEvent(&self,
		met: co::ME,
		extended_type: &GUID,
		status: co::HRESULT,
		value: Option<&PROPVARIANT>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaEventGeneratorVT>(self).QueueEvent)(
					self.ptr(),
					met.raw(),
					extended_type as *const _ as _,
					status.raw(),
					value.map_or(std::ptr::null(), |v| v as *const _ as _),
				)
			},
		)
	}
}
