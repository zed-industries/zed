// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dxgi1_6.h
use ctypes::c_void;
use shared::basetsd::SIZE_T;
use shared::dxgi1_2::{
    DXGI_COMPUTE_PREEMPTION_GRANULARITY, DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
};
use shared::dxgi1_4::{IDXGIAdapter3, IDXGIAdapter3Vtbl};
use shared::dxgi1_5::{IDXGIFactory5, IDXGIFactory5Vtbl, IDXGIOutput5, IDXGIOutput5Vtbl};
use shared::dxgitype::{DXGI_COLOR_SPACE_TYPE, DXGI_MODE_ROTATION};
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, FLOAT, UINT};
use shared::windef::{HMONITOR, RECT};
use um::winnt::{HRESULT, LUID, WCHAR};
ENUM!{enum DXGI_ADAPTER_FLAG3 {
    DXGI_ADAPTER_FLAG3_NONE = 0,
    DXGI_ADAPTER_FLAG3_REMOTE = 1,
    DXGI_ADAPTER_FLAG3_SOFTWARE = 2,
    DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE = 4,
    DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES = 8,
    DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES = 0x10,
    DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE = 0x20,
    DXGI_ADAPTER_FLAG3_FORCE_DWORD = 0xFFFFFFFF,
}}
STRUCT!{struct DXGI_ADAPTER_DESC3 {
    Description: [WCHAR; 128],
    VendorID: UINT,
    DeviceID: UINT,
    SubSysID: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
    Flags: DXGI_ADAPTER_FLAG3,
    GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}}
RIDL!{#[uuid(0x3c8d99d1, 0x4fbf, 0x4181, 0xa8, 0x2c, 0xaf, 0x66, 0xbf, 0x7b, 0xd2, 0x4e)]
interface IDXGIAdapter4(IDXGIAdapter4Vtbl): IDXGIAdapter3(IDXGIAdapter3Vtbl) {
    fn GetDesc3(
        pDesc: *mut DXGI_ADAPTER_DESC3,
    ) -> HRESULT,
}}
STRUCT!{struct DXGI_OUTPUT_DESC1 {
    DeviceName: [WCHAR; 32],
    DesktopCoordinates: RECT,
    AttachedToDesktop: BOOL,
    Rotation: DXGI_MODE_ROTATION,
    Monitor: HMONITOR,
    BitsPerColor: UINT,
    ColorSpace: DXGI_COLOR_SPACE_TYPE,
    RedPrimary: [FLOAT; 2],
    GreenPrimary: [FLOAT; 2],
    BluePrimary: [FLOAT; 2],
    WhitePoint: [FLOAT; 2],
    MinLuminance: FLOAT,
    MaxLuminance: FLOAT,
    MaxFullFrameLuminance: FLOAT,
}}
ENUM!{enum DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN = 1,
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED = 2,
    DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED = 4,
}}
RIDL!{#[uuid(0x068346e8, 0xaaec, 0x4b84, 0xad, 0xd7, 0x13, 0x7f, 0x51, 0x3f, 0x77, 0xa1)]
interface IDXGIOutput6(IDXGIOutput6Vtbl): IDXGIOutput5(IDXGIOutput5Vtbl) {
    fn GetDesc1(
        pDesc: *mut DXGI_OUTPUT_DESC1,
    ) -> HRESULT,
    fn CheckHardwareCompositionSupport(
        pFlags: *mut UINT,
    ) -> HRESULT,
}}
ENUM!{enum DXGI_GPU_PREFERENCE {
    DXGI_GPU_PREFERENCE_UNSPECIFIED = 0,
    DXGI_GPU_PREFERENCE_MINIMUM_POWER = 1,
    DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE = 2,
}}
RIDL!{#[uuid(0xc1b6694f, 0xff09, 0x44a9, 0xb0, 0x3c, 0x77, 0x90, 0x0a, 0x0a, 0x1d, 0x17)]
interface IDXGIFactory6(IDXGIFactory6Vtbl): IDXGIFactory5(IDXGIFactory5Vtbl) {
    fn EnumAdapterByGpuPreference(
        Adapter: UINT,
        GpuPreference: DXGI_GPU_PREFERENCE,
        riid: REFIID,
        ppvAdapter: *mut *mut c_void,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDXGIAdapter4,
    0x3c8d99d1, 0x4fbf, 0x4181, 0xa8, 0x2c, 0xaf, 0x66, 0xbf, 0x7b, 0xd2, 0x4e}
DEFINE_GUID!{IID_IDXGIOutput6,
    0x068346e8, 0xaaec, 0x4b84, 0xad, 0xd7, 0x13, 0x7f, 0x51, 0x3f, 0x77, 0xa1}
DEFINE_GUID!{IID_IDXGIFactory6,
    0xc1b6694f, 0xff09, 0x44a9, 0xb0, 0x3c, 0x77, 0x90, 0x0a, 0x0a, 0x1d, 0x17}
