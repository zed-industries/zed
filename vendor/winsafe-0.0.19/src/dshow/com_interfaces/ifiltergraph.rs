#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFilterGraph`](crate::IFilterGraph) virtual table.
#[repr(C)]
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(COMPTR, COMPTR, PCSTR) -> HRES,
	pub RemoveFilter: fn(COMPTR, COMPTR) -> HRES,
	pub EnumFilters: fn(COMPTR, *mut COMPTR) -> HRES,
	pub FindFilterByName: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub ConnectDirect: fn(COMPTR, COMPTR, COMPTR, PCVOID) -> HRES,
	pub Reconnect: fn(COMPTR, COMPTR) -> HRES,
	pub Disconnect: fn(COMPTR, COMPTR) -> HRES,
	pub SetDefaultSyncSource: fn(COMPTR) -> HRES,
}

com_interface! { IFilterGraph: "56a8689f-0ad4-11ce-b03a-0020af0ba770";
	/// [`IFilterGraph`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph)
	/// COM interface over [`IFilterGraphVT`](crate::vt::IFilterGraphVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFilterGraph for IFilterGraph {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFilterGraph`](crate::IFilterGraph).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFilterGraph: ole_IUnknown {
	/// [`IFilterGraph::AddFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-addfilter)
	/// method.
	fn AddFilter(&self,
		filter: &impl dshow_IBaseFilter,
		name: &str,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).AddFilter)(
					self.ptr(),
					filter.ptr(),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IFilterGraph::ConnectDirect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-connectdirect)
	/// method.
	fn ConnectDirect(&self,
		pin_out: &impl dshow_IPin,
		pin_in: &impl dshow_IPin,
		mt: Option<&AM_MEDIA_TYPE>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).ConnectDirect)(
					self.ptr(),
					pin_out.ptr(),
					pin_in.ptr(),
					mt.map_or(std::ptr::null_mut(), |amt| amt as *const _ as _),
				)
			},
		)
	}

	/// [`IFilterGraph::Disconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-disconnect)
	/// method.
	fn Disconnect(&self, pin: &impl dshow_IPin) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).Disconnect)(self.ptr(), pin.ptr())
			},
		)
	}

	fn_com_interface_get! { EnumFilters: IFilterGraphVT, IEnumFilters;
		/// [`IFilterGraph::EnumFilters`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-enumfilters)
		/// method.
	}

	/// [`IFilterGraph::FindFilterByName`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-findfilterbyname)
	/// method.
	#[must_use]
	fn FindFilterByName(&self, name: &str) -> HrResult<IBaseFilter> {
		let mut queried = unsafe { IBaseFilter::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).FindFilterByName)(
					self.ptr(),
					WString::from_str(name).as_ptr(),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IFilterGraph::Reconnect`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-reconnect)
	/// method.
	fn Reconnect(&self, pin: &impl dshow_IPin) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).Reconnect)(self.ptr(), pin.ptr())
			},
		)
	}

	/// [`IFilterGraph::RemoveFilter`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-removefilter)
	/// method.
	fn RemoveFilter(&self, filter: &impl dshow_IBaseFilter) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraphVT>(self).RemoveFilter)(
					self.ptr(),
					filter.ptr(),
				)
			},
		)
	}

	fn_com_noparm! { SetDefaultSyncSource: IFilterGraphVT;
		/// [`IFilterGraph::SetDefaultSyncSource`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph-setdefaultsyncsource)
		/// method.
	}
}
