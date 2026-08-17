#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IMediaSeeking`](crate::IMediaSeeking) virtual table.
#[repr(C)]
pub struct IMediaSeekingVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub CheckCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub IsFormatSupported: fn(COMPTR, PCVOID) -> HRES,
	pub QueryPreferredFormat: fn(COMPTR, PVOID) -> HRES,
	pub GetTimeFormat: fn(COMPTR, PVOID) -> HRES,
	pub IsUsingTimeFormat: fn(COMPTR, PCVOID) -> HRES,
	pub SetTimeFormat: fn(COMPTR, PCVOID) -> HRES,
	pub GetDuration: fn(COMPTR, *mut i64) -> HRES,
	pub GetStopPosition: fn(COMPTR, *mut i64) -> HRES,
	pub GetCurrentPosition: fn(COMPTR, *mut i64) -> HRES,
	pub ConvertTimeFormat: fn(COMPTR, *mut i64, PCVOID, i64, PCVOID) -> HRES,
	pub SetPositions: fn(COMPTR, *mut i64, u32, *mut i64, u32) -> HRES,
	pub GetPositions: fn(COMPTR, *mut i64, *mut i64) -> HRES,
	pub GetAvailable: fn(COMPTR, *mut i64, *mut i64) -> HRES,
	pub SetRate: fn(COMPTR, f64) -> HRES,
	pub GetRate: fn(COMPTR, *mut f64) -> HRES,
	pub GetPreroll: fn(COMPTR, *mut i64) -> HRES,
}

com_interface! { IMediaSeeking: "36b73880-c2c8-11cf-8b46-00805f6cef60";
	/// [`IMediaSeeking`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-imediaseeking)
	/// COM interface over [`IMediaSeekingVT`](crate::vt::IMediaSeekingVT). Inherits
	/// from [`IUnknown`](crate::IUnknown).
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
	/// let graph_builder: w::IGraphBuilder; // initialized somewhere
	/// # let graph_builder = unsafe { w::IGraphBuilder::null() };
	///
	/// let media_seeking = graph_builder
	///     .QueryInterface::<w::IMediaSeeking>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl dshow_IMediaSeeking for IMediaSeeking {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IMediaSeeking`](crate::IMediaSeeking).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IMediaSeeking: ole_IUnknown {
	/// [`IMediaSeeking::ConvertTimeFormat`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-converttimeformat)
	/// method.
	#[must_use]
	fn ConvertTimeFormat(&self,
		target_format: &co::TIME_FORMAT,
		source: i64,
		source_format: &co::TIME_FORMAT,
	) -> HrResult<i64>
	{
		let mut target = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).ConvertTimeFormat)(
					self.ptr(),
					&mut target,
					target_format as *const _ as _,
					source,
					source_format as *const _ as _,
				)
			},
		).map(|_| target)
	}

	/// [`IMediaSeeking::GetAvailable`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getavailable)
	/// method.
	///
	/// Returns earliest and latest times for efficient seeking.
	#[must_use]
	fn GetAvailable(&self) -> HrResult<(i64, i64)> {
		let (mut early, mut late) = (i64::default(), i64::default());
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetPositions)(
					self.ptr(),
					&mut early,
					&mut late,
				)
			},
		).map(|_| (early, late))
	}

	/// [`IMediaSeeking::GetCurrentPosition method`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getcurrentposition)
	/// method.
	#[must_use]
	fn GetCurrentPosition(&self) -> HrResult<i64> {
		let mut pos = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetCurrentPosition)(
					self.ptr(),
					&mut pos,
				)
			},
		).map(|_| pos)
	}

	/// [`IMediaSeeking::GetDuration`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getduration)
	/// method.
	#[must_use]
	fn GetDuration(&self) -> HrResult<i64> {
		let mut duration = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetDuration)(self.ptr(), &mut duration)
			},
		).map(|_| duration)
	}

	/// [`IMediaSeeking::GetPositions`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpositions)
	/// method.
	///
	/// Returns current and stop positions.
	#[must_use]
	fn GetPositions(&self) -> HrResult<(i64, i64)> {
		let (mut current, mut stop) = (i64::default(), i64::default());
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetPositions)(
					self.ptr(),
					&mut current,
					&mut stop,
				)
			},
		).map(|_| (current, stop))
	}

	/// [`IMediaSeeking::GetPreroll`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getpreroll)
	/// method.
	#[must_use]
	fn GetPreroll(&self) -> HrResult<i64> {
		let mut preroll = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetPreroll)(self.ptr(), &mut preroll)
			},
		).map(|_| preroll)
	}

	/// [`IMediaSeeking::GetRate`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getrate)
	/// method.
	#[must_use]
	fn GetRate(&self) -> HrResult<f64> {
		let mut rate = f64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetRate)(self.ptr(), &mut rate)
			},
		).map(|_| rate)
	}

	/// [`IMediaSeeking::GetStopPosition`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-getstopposition)
	/// method.
	#[must_use]
	fn GetStopPosition(&self) -> HrResult<i64> {
		let mut pos = i64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetStopPosition)(self.ptr(), &mut pos)
			},
		).map(|_| pos)
	}

	/// [`IMediaSeeking::GetTimeFormat`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-gettimeformat)
	/// method.
	#[must_use]
	fn GetTimeFormat(&self) -> HrResult<co::TIME_FORMAT> {
		let mut time_guid = co::TIME_FORMAT::NONE;
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).GetStopPosition)(
					self.ptr(),
					&mut time_guid as *mut _ as _,
				)
			},
		).map(|_| time_guid)
	}

	/// [`IMediaSeeking::SetPositions`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
	/// method.
	fn SetPositions(&self,
		current: i64,
		current_flags: co::SEEKING_FLAGS,
		stop: i64,
		stop_flags: co::SEEKING_FLAGS,
	) -> HrResult<()>
	{
		let (mut current, mut stop) = (current, stop);
		match unsafe {
			co::HRESULT::from_raw(
				(vt::<IMediaSeekingVT>(self).SetPositions)(
					self.ptr(),
					&mut current,
					current_flags.raw(),
					&mut stop,
					stop_flags.raw(),
				) as _,
			)
		} {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(()),
			hr => Err(hr),
		}
	}

	/// [`IMediaSeeking::SetRate`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setrate)
	/// method.
	fn SetRate(&self, rate: f64) -> HrResult<()> {
		ok_to_hrresult(
			unsafe { (vt::<IMediaSeekingVT>(self).SetRate)(self.ptr(), rate) },
		)
	}

	/// [`IMediaSeeking::SetTimeFormat`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-settimeformat)
	/// method.
	fn SetTimeFormat(&self, format: &co::TIME_FORMAT) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IMediaSeekingVT>(self).SetTimeFormat)(
					self.ptr(),
					format as *const _ as _,
				)
			},
		)
	}
}
