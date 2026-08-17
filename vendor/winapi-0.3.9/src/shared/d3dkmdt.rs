// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Longhorn Display Driver Model (LDDM) kernel mode data type definitions
use shared::basetsd::UINT32;
use shared::minwindef::UINT;
use shared::ntdef::{BOOLEAN, WCHAR};
//1932
pub const DXGK_MAX_METADATA_NAME_LENGTH: usize = 32;
ENUM!{enum DXGK_ENGINE_TYPE {
    DXGK_ENGINE_TYPE_OTHER,
    DXGK_ENGINE_TYPE_3D,
    DXGK_ENGINE_TYPE_VIDEO_DECODE,
    DXGK_ENGINE_TYPE_VIDEO_ENCODE,
    DXGK_ENGINE_TYPE_VIDEO_PROCESSING,
    DXGK_ENGINE_TYPE_SCENE_ASSEMBLY,
    DXGK_ENGINE_TYPE_COPY,
    DXGK_ENGINE_TYPE_OVERLAY,
    DXGK_ENGINE_TYPE_CRYPTO,
    DXGK_ENGINE_TYPE_MAX,
}}
STRUCT!{#[repr(packed)] struct DXGK_NODEMETADATA_FLAGS {
    Value: UINT32,
}}
BITFIELD!{DXGK_NODEMETADATA_FLAGS Value: UINT32 [
    ContextSchedulingSupported set_ContextSchedulingSupported[0..1],
    RingBufferFenceRelease set_RingBufferFenceRelease[1..2],
    SupportTrackedWorkload set_SupportTrackedWorkload[2..3],
    Reserved set_Reserved[3..16],
    MaxInFlightHwQueueBuffers set_MaxInFlightHwQueueBuffers[16..32],
]}
STRUCT!{#[repr(packed)] struct DXGK_NODEMETADATA {
    EngineType: DXGK_ENGINE_TYPE,
    FriendlyName: [WCHAR; DXGK_MAX_METADATA_NAME_LENGTH],
    Flags: DXGK_NODEMETADATA_FLAGS,
    GpuMmuSupported: BOOLEAN,
    IoMmuSupported: BOOLEAN,
}}
//2100
STRUCT!{#[repr(packed)] struct D3DKMT_NODEMETADATA {
    NodeOrdinalAndAdapterIndex: UINT,
    NodeData: DXGK_NODEMETADATA,
}}
