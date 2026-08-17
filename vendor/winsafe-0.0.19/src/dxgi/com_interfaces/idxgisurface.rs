#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGISurface`](crate::IDXGISurface) virtual table.
#[repr(C)]
pub struct IDXGISurfaceVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub Map: fn(COMPTR, PVOID, u32) -> HRES,
	pub Unmap: fn(COMPTR) -> HRES,
}

com_interface! { IDXGISurface: "cafcb56c-6ac3-4889-bf47-9e23bbd260ec";
	/// [`IDXGISurface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgisurface)
	/// COM interface over [`IDXGISurfaceVT`](crate::vt::IDXGISurfaceVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGISurface for IDXGISurface {}
impl dxgi_IDXGIDeviceSubObject for IDXGISurface {}
impl dxgi_IDXGIObject for IDXGISurface {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGISurface`](crate::IDXGISurface).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGISurface: dxgi_IDXGIDeviceSubObject {
	/// [`IDXGISurface::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgisurface-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_SURFACE_DESC> {
		let mut desc = DXGI_SURFACE_DESC::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISurfaceVT>(self).GetDesc)(
					self.ptr(),
					&mut desc as *mut _ as _,
				)
			},
		).map(|_| desc)
	}

	/// [`IDXGISurface::Map`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgisurface-map)
	/// method.
	#[must_use]
	fn Map(&self, map_flags: co::DXGI_MAP) -> HrResult<DXGI_MAPPED_RECT> {
		let mut mr = DXGI_MAPPED_RECT::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISurfaceVT>(self).Map)(
					self.ptr(),
					&mut mr as *mut _ as _,
					map_flags.raw(),
				)
			},
		).map(|_| mr)
	}

	/// [`IDXGISurface::Unmap`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgisurface-unmap)
	/// method.
	fn Unmap(&self) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IDXGISurfaceVT>(self).Unmap)(self.ptr()) })
	}
}
