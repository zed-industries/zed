#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IFilterGraph2`](crate::IFilterGraph2) virtual table.
#[repr(C)]
pub struct IFilterGraph2VT {
	pub IGraphBuilderVT: IGraphBuilderVT,
	pub AddSourceFilterForMoniker: fn(COMPTR, COMPTR, COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub ReconnectEx: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub RenderEx: fn(COMPTR, COMPTR, u32, *mut u32) -> HRES,
}

com_interface! { IFilterGraph2: "36b73882-c2c8-11cf-8b46-00805f6cef60";
	/// [`IFilterGraph2`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nn-strmif-ifiltergraph2)
	/// COM interface over [`IFilterGraph2VT`](crate::vt::IFilterGraph2VT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dshow_IFilterGraph for IFilterGraph2 {}
impl dshow_IGraphBuilder for IFilterGraph2 {}
impl dshow_IFilterGraph2 for IFilterGraph2 {}

/// This trait is enabled with the `dshow` feature, and provides methods for
/// [`IFilterGraph2`](crate::IFilterGraph2).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dshow_IFilterGraph2: dshow_IGraphBuilder {
	/// [`IFilterGraph2::ReconnectEx`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-ifiltergraph2-reconnectex)
	/// method.
	fn ReconnectEx(&self,
		pin: &impl dshow_IPin,
		mt: Option<&AM_MEDIA_TYPE>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IFilterGraph2VT>(self).ReconnectEx)(
					self.ptr(),
					pin.ptr(),
					mt.map_or(std::ptr::null_mut(), |mt| mt as *const _ as _),
				)
			},
		)
	}
}
