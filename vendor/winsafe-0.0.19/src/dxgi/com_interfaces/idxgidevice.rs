#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGIDevice`](crate::IDXGIDevice) virtual table.
#[repr(C)]
pub struct IDXGIDeviceVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetAdapter: fn(COMPTR, *mut COMPTR) -> HRES,
	pub CreateSurface: fn(COMPTR, *const u32, u32, u32, PCVOID, *mut COMPTR) -> HRES,
	pub QueryResourceResidency: fn(COMPTR, COMPTR, *mut u32, u32) -> HRES,
	pub SetGPUThreadPriority: fn(COMPTR, i32) -> HRES,
	pub GetGPUThreadPriority: fn(COMPTR, *mut i32) -> HRES,
}

com_interface! { IDXGIDevice: "54ec77fa-1377-44e6-8c32-88fd5f44c84c";
	/// [`IDXGIDevice`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgidevice)
	/// COM interface over [`IDXGIDeviceVT`](crate::vt::IDXGIDeviceVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGIDevice {}
impl dxgi_IDXGIDevice for IDXGIDevice {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGIDevice`](crate::IDXGIDevice).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGIDevice: dxgi_IDXGIObject {
	/// [`IDXGIDevice::CreateSurface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-createsurface)
	/// method.
	#[must_use]
	fn CreateSurface(&self,
		desc: &DXGI_SURFACE_DESC,
		num_surfaces: u32,
		usage: co::DXGI_USAGE,
		shared_resource: Option<&DXGI_SHARED_RESOURCE>,
	) -> HrResult<IDXGISurface>
	{
		let mut queried = unsafe { IDXGISurface::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIDeviceVT>(self).CreateSurface)(
					self.ptr(),
					desc as *const _ as _,
					num_surfaces,
					usage.raw(),
					shared_resource.map_or(std::ptr::null(), |s| s as *const _ as _),
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	fn_com_interface_get! { GetAdapter: IDXGIDeviceVT,  IDXGIAdapter;
		/// [`IDXGIDevice::GetAdapter`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getadapter)
		/// method.
	}

	/// [`IDXGIDevice::GetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-getgputhreadpriority)
	/// method.
	#[must_use]
	fn GetGPUThreadPriority(&self) -> HrResult<i8> {
		let mut priority = i32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIDeviceVT>(self).GetGPUThreadPriority)(
					self.ptr(),
					&mut priority,
				)
			},
		).map(|_| priority as _)
	}

	/// [`IDXGIDevice::SetGPUThreadPriority`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice-setgputhreadpriority)
	/// method.
	fn SetGPUThreadPriority(&self, priority: i8) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGIDeviceVT>(self).SetGPUThreadPriority)(
					self.ptr(),
					priority as _,
				)
			},
		)
	}
}
