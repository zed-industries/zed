#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMFMediaSource`](crate::IMFMediaSource) virtual table.
#[repr(C)]
pub struct IMFMediaSourceVT {
	pub IMFMediaEventGeneratorVT: IMFMediaEventGeneratorVT,
	pub GetCharacteristics: fn(COMPTR, *mut u32) -> HRES,
	pub CreatePresentationDescriptor: fn(COMPTR, *mut COMPTR) -> HRES,
	pub Start: fn(COMPTR, COMPTR, PCVOID, PCVOID) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Shutdown: fn(COMPTR) -> HRES,
}

com_interface! { IMFMediaSource: "279a808d-aec7-40c8-9c6b-a6b492c78a66";
	/// [`IMFMediaSource`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfmediasource)
	/// COM interface over [`IMFMediaSourceVT`](crate::vt::IMFMediaSourceVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl mf_IMFMediaEventGenerator for IMFMediaSource {}
impl mf_IMFMediaSource for IMFMediaSource {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFMediaSource`](crate::IMFMediaSource).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFMediaSource: mf_IMFMediaEventGenerator {
	/// [`IMFMediaSource::GetCharacteristics`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-getcharacteristics)
	/// method.
	#[must_use]
	fn GetCharacteristics(&self)-> HrResult<co::MFMEDIASOURCE> {
		let mut characteristics = co::MFMEDIASOURCE::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSourceVT>(self).GetCharacteristics)(
					self.ptr(),
					characteristics.as_mut(),
				)
			},
		).map(|_| characteristics)
	}

	fn_com_interface_get! { CreatePresentationDescriptor: IMFMediaSourceVT, IMFPresentationDescriptor;
		/// [`IMFMediaSource::CreatePresentationDescriptor`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-createpresentationdescriptor)
		/// method.
	}

	fn_com_noparm! { Pause: IMFMediaSourceVT;
		/// [`IMFMediaSource::Pause`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-pause)
		/// method.
	}

	fn_com_noparm! { Shutdown: IMFMediaSourceVT;
		/// [`IMFMediaSource::Shutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-shutdown)
		/// method.
	}

	/// [`IMFMediaSource::Start`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-start)
	/// method.
	fn Start(&self,
		presentation_descriptor: IMFPresentationDescriptor,
		time_format: Option<&GUID>,
		start_position: Option<&PROPVARIANT>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IMFMediaSourceVT>(self).Start)(
					self.ptr(),
					presentation_descriptor.ptr(),
					time_format.unwrap_or(&GUID::default()) as *const _ as _,
					start_position.unwrap_or(&PROPVARIANT::default()) as *const _ as _,
				)
			},
		)
	}

	fn_com_noparm! { Stop: IMFMediaSourceVT;
		/// [`IMFMediaSource::Stop`](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-stop)
		/// method.
	}
}
