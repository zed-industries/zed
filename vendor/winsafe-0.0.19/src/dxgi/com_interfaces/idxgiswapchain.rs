#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IDXGISwapChain`](crate::IDXGISwapChain) virtual table.
#[repr(C)]
pub struct IDXGISwapChainVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub Present: fn(COMPTR, u32, u32) -> HRES,
	pub GetBuffer: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub SetFullscreenState: fn(COMPTR, BOOL, COMPTR) -> HRES,
	pub GetFullscreenState: fn(COMPTR, *mut BOOL, *mut COMPTR) -> HRES,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub ResizeBuffers: fn(COMPTR, u32, u32, u32, u32, u32) -> HRES,
	pub ResizeTarget: fn(COMPTR, PCVOID) -> HRES,
	pub GetContainingOutput: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetFrameStatistics: fn(COMPTR, PVOID) -> HRES,
	pub GetLastPresentCount: fn(COMPTR, *mut u32) -> HRES,
}

com_interface! { IDXGISwapChain: "310d36a0-d2e7-4c0a-aa04-6a9d23b8886a";
	/// [`IDXGISwapChain`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiswapchain)
	/// COM interface over [`IDXGISwapChainVT`](crate::vt::IDXGISwapChainVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl dxgi_IDXGIObject for IDXGISwapChain {}
impl dxgi_IDXGIDeviceSubObject for IDXGISwapChain {}
impl dxgi_IDXGISwapChain for IDXGISwapChain {}

/// This trait is enabled with the `dxgi` feature, and provides methods for
/// [`IDXGISwapChain`](crate::IDXGISwapChain).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait dxgi_IDXGISwapChain: dxgi_IDXGIDeviceSubObject {
	fn_com_interface_get! { GetContainingOutput: IDXGISwapChainVT, IDXGIOutput;
		/// [`IDXGISwapChain::GetContainingOutput`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getcontainingoutput)
		/// method.
	}

	/// [`IDXGISwapChain::GetDesc`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getdesc)
	/// method.
	#[must_use]
	fn GetDesc(&self) -> HrResult<DXGI_SWAP_CHAIN_DESC> {
		let mut desc = DXGI_SWAP_CHAIN_DESC::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).GetDesc)(
					self.ptr(),
					&mut desc as *mut _ as _,
				)
			},
		).map(|_| desc)
	}

	/// [`IDXGISwapChain::GetFrameStatistics`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getframestatistics)
	/// method.
	fn GetFrameStatistics(&self) -> HrResult<DXGI_FRAME_STATISTICS> {
		let mut stats = DXGI_FRAME_STATISTICS::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).GetDesc)(
					self.ptr(),
					&mut stats as *mut _ as _,
				)
			},
		).map(|_| stats)
	}

	/// [`IDXGISwapChain::GetFullscreenState`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getfullscreenstate)
	/// method.
	#[must_use]
	fn GetFullscreenState(&self) -> HrResult<(bool, Option<IDXGIOutput>)> {
		let mut fullscreen: BOOL = 0;
		let mut queried = unsafe { IDXGIOutput::null() };

		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).GetFullscreenState)(
					self.ptr(),
					&mut fullscreen,
					queried.as_mut(),
				)
			},
		).map(|_| (
			fullscreen != 0,
			if queried.ptr().is_null() { None } else { Some(queried) },
		))
	}

	/// [`IDXGISwapChain::Present`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present)
	/// method.
	fn Present(&self,
		sync_interval: u32,
		flags: co::DXGI_PRESENT,
	) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).Present)(
					self.ptr(),
					sync_interval,
					flags.raw(),
				)
			},
		)
	}

	/// [`IDXGISwapChain::ResizeTarget`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-resizetarget)
	/// method.
	fn ResizeTarget(&self,
		new_target_parameters: &DXGI_MODE_DESC,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).ResizeTarget)(
					self.ptr(),
					new_target_parameters as *const _ as _,
				)
			},
		)
	}

	/// [`IDXGISwapChain::SetFullscreenState`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-setfullscreenstate)
	/// method.
	fn SetFullscreenState(&self,
		fullscreen: bool,
		target: Option<&impl dxgi_IDXGIOutput>,
	) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				(vt::<IDXGISwapChainVT>(self).SetFullscreenState)(
					self.ptr(),
					fullscreen as _,
					target.map_or(std::ptr::null_mut(), |t| t.ptr()),
				)
			},
		)
	}
}
