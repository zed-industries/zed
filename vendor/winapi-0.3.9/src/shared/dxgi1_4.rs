// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dxgi1_4.h
use ctypes::c_void;
use shared::basetsd::UINT64;
use shared::dxgi1_2::{IDXGIAdapter2, IDXGIAdapter2Vtbl};
use shared::dxgi1_3::{
    IDXGIFactory3, IDXGIFactory3Vtbl, IDXGIOutput3, IDXGIOutput3Vtbl, IDXGISwapChain2,
    IDXGISwapChain2Vtbl,
};
use shared::dxgiformat::DXGI_FORMAT;
use shared::dxgitype::DXGI_COLOR_SPACE_TYPE;
use shared::guiddef::REFGUID;
use shared::minwindef::{DWORD, UINT};
use um::unknwnbase::IUnknown;
use um::winnt::{HANDLE, HRESULT, LUID};
ENUM!{enum DXGI_MEMORY_SEGMENT_GROUP {
    DXGI_MEMORY_SEGMENT_GROUP_LOCAL = 0,
    DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL = 1,
}}
ENUM!{enum DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
}}
ENUM!{enum DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1,
    DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT = 0x2,
}}
STRUCT!{struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    Budget: UINT64,
    CurrentUsage: UINT64,
    AvailableForReservation: UINT64,
    CurrentReservation: UINT64,
}}
RIDL!{#[uuid(0x645967a4, 0x1392, 0x4310, 0xa7, 0x98, 0x80, 0x53, 0xce, 0x3e, 0x93, 0xfd)]
interface IDXGIAdapter3(IDXGIAdapter3Vtbl): IDXGIAdapter2(IDXGIAdapter2Vtbl) {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(
        hEvent: HANDLE,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterHardwareContentProtectionTeardownStatus(
        dwCookie: DWORD,
    ) -> (),
    fn QueryVideoMemoryInfo(
        NodeIndex: UINT,
        MemorySegmentGroup: DXGI_MEMORY_SEGMENT_GROUP,
        pVideoMemoryInfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO,
    ) -> HRESULT,
    fn SetVideoMemoryReservation(
        NodeIndex: UINT,
        MemorySegmentGroup: DXGI_MEMORY_SEGMENT_GROUP,
        Reservation: UINT64,
    ) -> HRESULT,
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(
        hEvent: HANDLE,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterVideoMemoryBudgetChangeNotification(
        dwCookie: DWORD,
    ) -> (),
}}
RIDL!{#[uuid(0x1bc6ea02, 0xef36, 0x464f, 0xbf, 0x0c, 0x21, 0xca, 0x39, 0xe5, 0x16, 0x8a)]
interface IDXGIFactory4(IDXGIFactory4Vtbl): IDXGIFactory3(IDXGIFactory3Vtbl) {
    fn EnumAdapterByLuid(
        AdapterLuid: LUID,
        riid: REFGUID,
        ppvAdapter: *mut *mut c_void,
    ) -> HRESULT,
    fn EnumWarpAdapter(
        riid: REFGUID,
        ppvAdapter: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdc7dca35, 0x2196, 0x414d, 0x9f, 0x53, 0x61, 0x78, 0x84, 0x03, 0x2a, 0x60)]
interface IDXGIOutput4(IDXGIOutput4Vtbl): IDXGIOutput3(IDXGIOutput3Vtbl) {
    fn CheckOverlayColorSpaceSupport(
        Format: DXGI_FORMAT,
        ColorSpace: DXGI_COLOR_SPACE_TYPE,
        pConcernedDevice: *mut IUnknown,
        pFlags: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x94d99bdb, 0xf1f8, 0x4ab0, 0xb2, 0x36, 0x7d, 0xa0, 0x17, 0x0e, 0xda, 0xb1)]
interface IDXGISwapChain3(IDXGISwapChain3Vtbl): IDXGISwapChain2(IDXGISwapChain2Vtbl) {
    fn GetCurrentBackBufferIndex() -> UINT,
    fn CheckColorSpaceSupport(
        ColorSpace: DXGI_COLOR_SPACE_TYPE,
        pColorSpaceSupport: *mut UINT,
    ) -> HRESULT,
    fn SetColorSpace1(
        ColorSpace: DXGI_COLOR_SPACE_TYPE,
    ) -> HRESULT,
    fn ResizeBuffers1(
        BufferCount: UINT,
        Width: UINT,
        Height: UINT,
        Format: DXGI_FORMAT,
        SwapChainFlags: UINT,
        pCreationNodeMask: *const UINT,
        ppPresentQueue: *mut *mut IUnknown,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDXGISwapChain3,
    0x94d99bdb, 0xf1f8, 0x4ab0, 0xb2, 0x36, 0x7d, 0xa0, 0x17, 0x0e, 0xda, 0xb1}
DEFINE_GUID!{IID_IDXGIOutput4,
    0xdc7dca35, 0x2196, 0x414d, 0x9f, 0x53, 0x61, 0x78, 0x84, 0x03, 0x2a, 0x60}
DEFINE_GUID!{IID_IDXGIFactory4,
    0x1bc6ea02, 0xef36, 0x464f, 0xbf, 0x0c, 0x21, 0xca, 0x39, 0xe5, 0x16, 0x8a}
DEFINE_GUID!{IID_IDXGIAdapter3,
    0x645967a4, 0x1392, 0x4310, 0xa7, 0x98, 0x80, 0x53, 0xce, 0x3e, 0x93, 0xfd}
