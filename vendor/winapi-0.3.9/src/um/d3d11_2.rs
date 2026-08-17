// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::{UINT16, UINT64, UINT8};
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::{BOOL, INT, UINT};
use um::d3d11::{ID3D11Buffer, ID3D11DeviceChild, ID3D11Resource};
use um::d3d11_1::{
    ID3D11Device1, ID3D11Device1Vtbl, ID3D11DeviceContext1, ID3D11DeviceContext1Vtbl,
};
use um::winnt::{HRESULT, LPCWSTR};
DEFINE_GUID!{IID_ID3D11DeviceContext2,
    0x420d5b32, 0xb90c, 0x4da4, 0xbe, 0xf0, 0x35, 0x9f, 0x6a, 0x24, 0xa8, 0x3a}
DEFINE_GUID!{IID_ID3D11Device2,
    0x9d06dffa, 0xd1e5, 0x4d07, 0x83, 0xa8, 0x1b, 0xb1, 0x23, 0xf2, 0xf8, 0x41}
STRUCT!{struct D3D11_TILED_RESOURCE_COORDINATE {
    X: UINT,
    Y: UINT,
    Z: UINT,
    Subresource: UINT,
}}
STRUCT!{struct D3D11_TILE_REGION_SIZE {
    NumTiles: UINT,
    bUseBox: BOOL,
    Width: UINT,
    Height: UINT16,
    Depth: UINT16,
}}
ENUM!{enum D3D11_TILE_MAPPING_FLAG {
    D3D11_TILE_MAPPING_NO_OVERWRITE = 0x00000001,
}}
ENUM!{enum D3D11_TILE_RANGE_FLAG {
    D3D11_TILE_RANGE_NULL = 0x00000001,
    D3D11_TILE_RANGE_SKIP = 0x00000002,
    D3D11_TILE_RANGE_REUSE_SINGLE_TILE = 0x00000004,
}}
STRUCT!{struct D3D11_SUBRESOURCE_TILING {
    WidthInTiles: UINT,
    HeightInTiles: UINT16,
    DepthInTiles: UINT16,
    StartTileIndexInOverallResource: UINT,
}}
STRUCT!{struct D3D11_TILE_SHAPE {
    WidthInTexels: UINT,
    HeightInTexels: UINT,
    DepthInTexels: UINT,
}}
STRUCT!{struct D3D11_PACKED_MIP_DESC {
    NumStandardMips: UINT8,
    NumPackedMips: UINT8,
    NumTilesForPackedMips: UINT,
    StartTileIndexInOverallResource: UINT,
}}
ENUM!{enum D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
    D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE = 0x00000001,
}}
ENUM!{enum D3D11_TILE_COPY_FLAG {
    D3D11_TILE_COPY_NO_OVERWRITE = 0x00000001,
    D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x00000002,
    D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x00000004,
}}
RIDL!{#[uuid(0x420d5b32, 0xb90c, 0x4da4, 0xbe, 0xf0, 0x35, 0x9f, 0x6a, 0x24, 0xa8, 0x3a)]
interface ID3D11DeviceContext2(ID3D11DeviceContext2Vtbl):
    ID3D11DeviceContext1(ID3D11DeviceContext1Vtbl) {
    fn UpdateTileMappings(
        pTiledResource: *mut ID3D11Resource,
        NumTiledResourceRegions: UINT,
        pTiledResourceRegionStartCoordinates: *const D3D11_TILED_RESOURCE_COORDINATE,
        pTiledResourceRegionSizes: *const D3D11_TILE_REGION_SIZE,
        pTilePool: *mut ID3D11Buffer,
        NumRanges: UINT,
        pRangeFlags: *const UINT,
        pTilePoolStartOffsets: *const UINT,
        pRangeTileCounts: *const UINT,
        Flags: UINT,
    ) -> HRESULT,
    fn CopyTileMappings(
        pDestTiledResource: *mut ID3D11Resource,
        pDestRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE,
        pSourceTiledResource: *mut ID3D11Resource,
        pSourceRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE,
        pTileRegionSize: *const D3D11_TILE_REGION_SIZE,
        Flags: UINT,
    ) -> HRESULT,
    fn CopyTiles(
        pTiledResource: *mut ID3D11Resource,
        pTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE,
        pTileRegionSize: *const D3D11_TILE_REGION_SIZE,
        pBuffer: *mut ID3D11Buffer,
        BufferStartOffsetInBytes: UINT64,
        Flags: UINT,
    ) -> (),
    fn UpdateTiles(
        pDestTiledResource: *mut ID3D11Resource,
        pDestTileRegionStartCoordinate: *const D3D11_TILED_RESOURCE_COORDINATE,
        pDestTileRegionSize: *const D3D11_TILE_REGION_SIZE,
        pSourceTileData: *const c_void,
        Flags: UINT,
    ) -> (),
    fn ResizeTilePool(
        pTilePool: *mut ID3D11Buffer,
        NewSizeInBytes: UINT64,
    ) -> HRESULT,
    fn TiledResourceBarrier(
        pTiledResourceOrViewAccessBeforeBarrier: *mut ID3D11DeviceChild,
        pTiledResourceOrViewAccessAfterBarrier: *mut ID3D11DeviceChild,
    ) -> (),
    fn IsAnnotationEnabled() -> BOOL,
    fn SetMarkerInt(
        pLabel: LPCWSTR,
        Data: INT,
    ) -> (),
    fn BeginEventInt(
        pLabel: LPCWSTR,
        Data: INT,
    ) -> (),
    fn EndEvent() -> (),
}}
RIDL!{#[uuid(0x9d06dffa, 0xd1e5, 0x4d07, 0x83, 0xa8, 0x1b, 0xb1, 0x23, 0xf2, 0xf8, 0x41)]
interface ID3D11Device2(ID3D11Device2Vtbl): ID3D11Device1(ID3D11Device1Vtbl) {
    fn GetImmediateContext2(
        ppImmediateContext: *mut *mut ID3D11DeviceContext2,
    ) -> (),
    fn CreateDeferredContext2(
        ContextFlags: UINT,
        ppDeferredContext: *mut *mut ID3D11DeviceContext2,
    ) -> HRESULT,
    fn GetResourceTiling(
        pTiledResource: *mut ID3D11Resource,
        pNumTilesForEntireResource: *mut UINT,
        pPackedMipDesc: *mut D3D11_PACKED_MIP_DESC,
        pStandardTileShapeForNonPackedMips: *mut D3D11_TILE_SHAPE,
        pNumSubresourceTilings: *mut UINT,
        FirstSubresourceTilingToGet: UINT,
        pSubresourceTilingsForNonPackedMips: *mut D3D11_SUBRESOURCE_TILING,
    ) -> (),
    fn CheckMultisampleQualityLevels1(
        Format: DXGI_FORMAT,
        SampleCount: UINT,
        Flags: UINT,
        pNumQualityLevels: *mut UINT,
    ) -> HRESULT,
}}
