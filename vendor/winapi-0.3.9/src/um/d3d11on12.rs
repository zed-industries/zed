// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the content of d3d11on12.h
use ctypes::c_void;
use shared::guiddef::IID;
use shared::minwindef::UINT;
use um::d3d11::{ID3D11Device, ID3D11DeviceContext, ID3D11Resource};
use um::d3d12::D3D12_RESOURCE_STATES;
use um::d3dcommon::D3D_FEATURE_LEVEL;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::HRESULT;
FN!{stdcall PFN_D3D11ON12_CREATE_DEVICE(
    *mut IUnknown,
    UINT,
    *const D3D_FEATURE_LEVEL,
    UINT,
    *mut *mut IUnknown,
    UINT,
    UINT,
    *mut *mut ID3D11Device,
    *mut *mut ID3D11DeviceContext,
    *mut D3D_FEATURE_LEVEL,
) -> HRESULT}
extern "system" {
    pub fn D3D11On12CreateDevice(
        pDevice: *mut IUnknown,
        Flags: UINT,
        pFeatureLevels: *const D3D_FEATURE_LEVEL,
        FeatureLevels: UINT,
        ppCommandQueues: *mut *mut IUnknown,
        NumQueues: UINT,
        NodeMask: UINT,
        ppDevice: *mut *mut ID3D11Device,
        ppImmediateContext: *mut *mut ID3D11DeviceContext,
        pChosenFeatureLevel: *mut D3D_FEATURE_LEVEL,
    ) -> HRESULT;
}
STRUCT!{struct D3D11_RESOURCE_FLAGS {
    BindFlags: UINT,
    MiscFlags: UINT,
    CPUAccessFlags: UINT,
    StructureByteStride: UINT,
}}
RIDL!{#[uuid(0x85611e73, 0x70a9, 0x490e, 0x96, 0x14, 0xa9, 0xe3, 0x02, 0x77, 0x79, 0x04)]
interface ID3D11On12Device(ID3D11On12DeviceVtbl): IUnknown(IUnknownVtbl) {
    fn CreateWrappedResource(
        pResource12: *mut IUnknown,
        pFlags11: *const D3D11_RESOURCE_FLAGS,
        InState: D3D12_RESOURCE_STATES,
        OutState: D3D12_RESOURCE_STATES,
        riid: *const IID,
        ppResource11: *mut *mut c_void,
    ) -> HRESULT,
    fn ReleaseWrappedResources(
        ppResources: *mut *mut ID3D11Resource,
        NumResources: UINT,
    ) -> (),
    fn AcquireWrappedResources(
        ppResources: *mut *mut ID3D11Resource,
        NumResources: UINT,
    ) -> (),
}}
DEFINE_GUID!{IID_ID3D11On12Device,
    0x85611e73, 0x70a9, 0x490e, 0x96, 0x14, 0xa9, 0xe3, 0x02, 0x77, 0x79, 0x04}
