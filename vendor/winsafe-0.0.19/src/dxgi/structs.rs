#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;

/// [`DXGI_ADAPTER_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_adapter_desc)
/// struct.
#[repr(C)]
pub struct DXGI_ADAPTER_DESC {
	Description: [u16; 128],
	pub VendorId: u32,
	pub DeviceId: u32,
	pub SubSysId: u32,
	pub Revision: u32,
	pub DedicatedVideoMemory: usize,
	pub DedicatedSystemMemory: usize,
	pub SharedSystemMemory: usize,
	pub AdapterLuid: LUID,
}

impl_default!(DXGI_ADAPTER_DESC);

impl DXGI_ADAPTER_DESC {
	pub_fn_string_arr_get_set!(Description, set_Description);
}

/// [`DXGI_FRAME_STATISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_frame_statistics)
/// struct.
#[repr(C)]
#[derive(Default, PartialEq, Eq)]
pub struct DXGI_FRAME_STATISTICS {
	pub PresentCount: u32,
	pub PresentRefreshCount: u32,
	pub SyncRefreshCount: u32,
	pub SyncQPCTime: i64,
	pub SyncGPUTime: i64,
}

/// [`DXGI_GAMMA_CONTROL`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173061(v=vs.85))
/// struct.
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL {
	pub Scale: DXGI_RGB,
	pub Offset: DXGI_RGB,
	pub GammaCurve: [DXGI_RGB; 1025],
}

impl_default!(DXGI_GAMMA_CONTROL);

/// [`DXGI_GAMMA_CONTROL_CAPABILITIES`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/dxgitype/ns-dxgitype-dxgi_gamma_control_capabilities)
/// struct.
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
	ScaleAndOffsetSupported: BOOL,
	pub MaxConvertedValue: f32,
	pub MinConvertedValue: f32,
	pub NumGammaControlPoints: u32,
	pub ControlPointPositions: [f32; 1025],
}

impl_default!(DXGI_GAMMA_CONTROL_CAPABILITIES);

impl DXGI_GAMMA_CONTROL_CAPABILITIES {
	pub_fn_bool_get_set!(ScaleAndOffsetSupported, set_ScaleAndOffsetSupported);
}

/// [`DXGI_MAPPED_RECT`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_mapped_rect)
/// struct.
#[repr(C)]
pub struct DXGI_MAPPED_RECT {
	Pitch: i32,
	pBits: *mut u8,
}

impl_default!(DXGI_MAPPED_RECT);

impl DXGI_MAPPED_RECT {
	/// Returns a slice over the `pBits` buffer.
	pub fn pBits(&self) -> Option<&[u8]> {
		if self.pBits.is_null() {
			None
		} else {
			Some(unsafe { std::slice::from_raw_parts(self.pBits, self.Pitch as _) })
		}
	}
}

/// [`DXGI_MODE_DESC`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173064(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct DXGI_MODE_DESC {
	pub Width: u32,
	pub Height: u32,
	pub RefreshRate: DXGI_RATIONAL,
	pub Format: co::DXGI_FORMAT,
	pub ScanlineOrdering: co::DXGI_MODE_SCANLINE_ORDER,
	pub Scaling: co::DXGI_MODE_SCALING,
}

/// [`DXGI_OUTPUT_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_output_desc)
/// struct.
#[repr(C)]
pub struct DXGI_OUTPUT_DESC {
	DeviceName: [u16; 32],
	pub DesktopCoordinates: RECT,
	AttachedToDesktop: BOOL,
	pub Rotation: co::DXGI_MODE_ROTATION,
	pub Monitor: HMONITOR,
}

impl_default!(DXGI_OUTPUT_DESC);

impl DXGI_OUTPUT_DESC {
	pub_fn_string_arr_get_set!(DeviceName, set_DeviceName);
	pub_fn_bool_get_set!(AttachedToDesktop, set_AttachedToDesktop);

}

/// [`DXGI_RATIONAL`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_rational?redirectedfrom=MSDN)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DXGI_RATIONAL {
	pub Numerator: u32,
	pub Denominator: u32,
}

/// [`DXGI_RGB`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173071(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct DXGI_RGB {
	pub Red: f32,
	pub Green: f32,
	pub Blue: f32,
}

/// [`DXGI_SAMPLE_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_sample_desc)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DXGI_SAMPLE_DESC {
	pub Count: u32,
	pub Quality: u32,
}

/// [`DXGI_SHARED_RESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_shared_resource)
/// struct.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_SHARED_RESOURCE {
	pub Handle: *mut std::ffi::c_void,
}

impl_default!(DXGI_SHARED_RESOURCE);

/// [`DXGI_SURFACE_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_surface_desc)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DXGI_SURFACE_DESC {
	pub Width: u32,
	pub Height: u32,
	pub Format: co::DXGI_FORMAT,
	pub SampleDesc: DXGI_SAMPLE_DESC,
}

/// [`DXGI_SWAP_CHAIN_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_swap_chain_desc)
/// struct.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct DXGI_SWAP_CHAIN_DESC {
	pub BufferDesc: DXGI_MODE_DESC,
	pub SampleDesc: DXGI_SAMPLE_DESC,
	pub BufferUsage: co::DXGI_USAGE,
	pub BufferCount: u32,
	pub OutputWindow: HWND,
	Windowed: BOOL,
	pub SwapEffect: co::DXGI_SWAP_EFFECT,
	pub Flags: co::DXGI_SWAP_CHAIN_FLAG,
}

impl_default!(DXGI_SWAP_CHAIN_DESC);

impl DXGI_SWAP_CHAIN_DESC {
	pub_fn_bool_get_set!(Windowed, set_Windowed);
}
