// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::d3d9::{IDirect3DDevice9Ex, IDirect3DSurface9};
use shared::d3d9types::{D3DCOLOR, D3DFORMAT, D3DPOOL};
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, FLOAT, INT, UINT};
use shared::windef::{RECT, SIZE};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, ULONGLONG};
DEFINE_GUID!{IID_IDXVAHD_Device,
    0x95f12dfd, 0xd77e, 0x49be, 0x81, 0x5f, 0x57, 0xd5, 0x79, 0x63, 0x4d, 0x6d}
DEFINE_GUID!{IID_IDXVAHD_VideoProcessor,
    0x95f4edf4, 0x6e03, 0x4cd7, 0xbe, 0x1b, 0x30, 0x75, 0xd6, 0x65, 0xaa, 0x52}
ENUM!{enum DXVAHD_FRAME_FORMAT {
    DXVAHD_FRAME_FORMAT_PROGRESSIVE = 0,
    DXVAHD_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST = 1,
    DXVAHD_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST = 2,
}}
ENUM!{enum DXVAHD_DEVICE_USAGE {
    DXVAHD_DEVICE_USAGE_PLAYBACK_NORMAL = 0,
    DXVAHD_DEVICE_USAGE_OPTIMAL_SPEED = 1,
    DXVAHD_DEVICE_USAGE_OPTIMAL_QUALITY = 2,
}}
ENUM!{enum DXVAHD_SURFACE_TYPE {
    DXVAHD_SURFACE_TYPE_VIDEO_INPUT = 0,
    DXVAHD_SURFACE_TYPE_VIDEO_INPUT_PRIVATE = 1,
    DXVAHD_SURFACE_TYPE_VIDEO_OUTPUT = 2,
}}
ENUM!{enum DXVAHD_DEVICE_TYPE {
    DXVAHD_DEVICE_TYPE_HARDWARE = 0,
    DXVAHD_DEVICE_TYPE_SOFTWARE = 1,
    DXVAHD_DEVICE_TYPE_REFERENCE = 2,
    DXVAHD_DEVICE_TYPE_OTHER = 3,
}}
ENUM!{enum DXVAHD_DEVICE_CAPS {
    DXVAHD_DEVICE_CAPS_LINEAR_SPACE = 0x1,
    DXVAHD_DEVICE_CAPS_xvYCC = 0x2,
    DXVAHD_DEVICE_CAPS_RGB_RANGE_CONVERSION = 0x4,
    DXVAHD_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION = 0x8,
}}
ENUM!{enum DXVAHD_FEATURE_CAPS {
    DXVAHD_FEATURE_CAPS_ALPHA_FILL = 0x1,
    DXVAHD_FEATURE_CAPS_CONSTRICTION = 0x2,
    DXVAHD_FEATURE_CAPS_LUMA_KEY = 0x4,
    DXVAHD_FEATURE_CAPS_ALPHA_PALETTE = 0x8,
}}
ENUM!{enum DXVAHD_FILTER_CAPS {
    DXVAHD_FILTER_CAPS_BRIGHTNESS = 0x1,
    DXVAHD_FILTER_CAPS_CONTRAST = 0x2,
    DXVAHD_FILTER_CAPS_HUE = 0x4,
    DXVAHD_FILTER_CAPS_SATURATION = 0x8,
    DXVAHD_FILTER_CAPS_NOISE_REDUCTION = 0x10,
    DXVAHD_FILTER_CAPS_EDGE_ENHANCEMENT = 0x20,
    DXVAHD_FILTER_CAPS_ANAMORPHIC_SCALING = 0x40,
}}
ENUM!{enum DXVAHD_INPUT_FORMAT_CAPS {
    DXVAHD_INPUT_FORMAT_CAPS_RGB_INTERLACED = 0x1,
    DXVAHD_INPUT_FORMAT_CAPS_RGB_PROCAMP = 0x2,
    DXVAHD_INPUT_FORMAT_CAPS_RGB_LUMA_KEY = 0x4,
    DXVAHD_INPUT_FORMAT_CAPS_PALETTE_INTERLACED = 0x8,
}}
ENUM!{enum DXVAHD_PROCESSOR_CAPS {
    DXVAHD_PROCESSOR_CAPS_DEINTERLACE_BLEND = 0x1,
    DXVAHD_PROCESSOR_CAPS_DEINTERLACE_BOB = 0x2,
    DXVAHD_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE = 0x4,
    DXVAHD_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION = 0x8,
    DXVAHD_PROCESSOR_CAPS_INVERSE_TELECINE = 0x10,
    DXVAHD_PROCESSOR_CAPS_FRAME_RATE_CONVERSION = 0x20,
}}
ENUM!{enum DXVAHD_ITELECINE_CAPS {
    DXVAHD_ITELECINE_CAPS_32 = 0x1,
    DXVAHD_ITELECINE_CAPS_22 = 0x2,
    DXVAHD_ITELECINE_CAPS_2224 = 0x4,
    DXVAHD_ITELECINE_CAPS_2332 = 0x8,
    DXVAHD_ITELECINE_CAPS_32322 = 0x10,
    DXVAHD_ITELECINE_CAPS_55 = 0x20,
    DXVAHD_ITELECINE_CAPS_64 = 0x40,
    DXVAHD_ITELECINE_CAPS_87 = 0x80,
    DXVAHD_ITELECINE_CAPS_222222222223 = 0x100,
    DXVAHD_ITELECINE_CAPS_OTHER = 0x80000000,
}}
ENUM!{enum DXVAHD_FILTER {
    DXVAHD_FILTER_BRIGHTNESS = 0,
    DXVAHD_FILTER_CONTRAST = 1,
    DXVAHD_FILTER_HUE = 2,
    DXVAHD_FILTER_SATURATION = 3,
    DXVAHD_FILTER_NOISE_REDUCTION = 4,
    DXVAHD_FILTER_EDGE_ENHANCEMENT = 5,
    DXVAHD_FILTER_ANAMORPHIC_SCALING = 6,
}}
ENUM!{enum DXVAHD_BLT_STATE {
    DXVAHD_BLT_STATE_TARGET_RECT = 0,
    DXVAHD_BLT_STATE_BACKGROUND_COLOR = 1,
    DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE = 2,
    DXVAHD_BLT_STATE_ALPHA_FILL = 3,
    DXVAHD_BLT_STATE_CONSTRICTION = 4,
    DXVAHD_BLT_STATE_PRIVATE = 1000,
}}
ENUM!{enum DXVAHD_ALPHA_FILL_MODE {
    DXVAHD_ALPHA_FILL_MODE_OPAQUE = 0,
    DXVAHD_ALPHA_FILL_MODE_BACKGROUND = 1,
    DXVAHD_ALPHA_FILL_MODE_DESTINATION = 2,
    DXVAHD_ALPHA_FILL_MODE_SOURCE_STREAM = 3,
}}
ENUM!{enum DXVAHD_STREAM_STATE {
    DXVAHD_STREAM_STATE_D3DFORMAT = 0,
    DXVAHD_STREAM_STATE_FRAME_FORMAT = 1,
    DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE = 2,
    DXVAHD_STREAM_STATE_OUTPUT_RATE = 3,
    DXVAHD_STREAM_STATE_SOURCE_RECT = 4,
    DXVAHD_STREAM_STATE_DESTINATION_RECT = 5,
    DXVAHD_STREAM_STATE_ALPHA = 6,
    DXVAHD_STREAM_STATE_PALETTE = 7,
    DXVAHD_STREAM_STATE_LUMA_KEY = 8,
    DXVAHD_STREAM_STATE_ASPECT_RATIO = 9,
    DXVAHD_STREAM_STATE_FILTER_BRIGHTNESS = 100,
    DXVAHD_STREAM_STATE_FILTER_CONTRAST = 101,
    DXVAHD_STREAM_STATE_FILTER_HUE = 102,
    DXVAHD_STREAM_STATE_FILTER_SATURATION = 103,
    DXVAHD_STREAM_STATE_FILTER_NOISE_REDUCTION = 104,
    DXVAHD_STREAM_STATE_FILTER_EDGE_ENHANCEMENT = 105,
    DXVAHD_STREAM_STATE_FILTER_ANAMORPHIC_SCALING = 106,
    DXVAHD_STREAM_STATE_PRIVATE = 1000,
}}
ENUM!{enum DXVAHD_OUTPUT_RATE {
    DXVAHD_OUTPUT_RATE_NORMAL = 0,
    DXVAHD_OUTPUT_RATE_HALF = 1,
    DXVAHD_OUTPUT_RATE_CUSTOM = 2,
}}
STRUCT!{struct DXVAHD_RATIONAL {
    Numerator: UINT,
    Denominator: UINT,
}}
STRUCT!{struct DXVAHD_COLOR_RGBA {
    R: FLOAT,
    G: FLOAT,
    B: FLOAT,
    A: FLOAT,
}}
STRUCT!{struct DXVAHD_COLOR_YCbCrA {
    Y: FLOAT,
    Cb: FLOAT,
    Cr: FLOAT,
    A: FLOAT,
}}
UNION!{union DXVAHD_COLOR {
    [u32; 4],
    RGB RGB_mut: DXVAHD_COLOR_RGBA,
    YCbCr YCbCr_mut: DXVAHD_COLOR_YCbCrA,
}}
STRUCT!{struct DXVAHD_CONTENT_DESC {
    InputFrameFormat: DXVAHD_FRAME_FORMAT,
    InputFrameRate: DXVAHD_RATIONAL,
    InputWidth: UINT,
    InputHeight: UINT,
    OutputFrameRate: DXVAHD_RATIONAL,
    OutputWidth: UINT,
    OutputHeight: UINT,
}}
STRUCT!{struct DXVAHD_VPDEVCAPS {
    DeviceType: DXVAHD_DEVICE_TYPE,
    DeviceCaps: UINT,
    FeatureCaps: UINT,
    FilterCaps: UINT,
    InputFormatCaps: UINT,
    InputPool: D3DPOOL,
    OutputFormatCount: UINT,
    InputFormatCount: UINT,
    VideoProcessorCount: UINT,
    MaxInputStreams: UINT,
    MaxStreamStates: UINT,
}}
STRUCT!{struct DXVAHD_VPCAPS {
    VPGuid: GUID,
    PastFrames: UINT,
    FutureFrames: UINT,
    ProcessorCaps: UINT,
    ITelecineCaps: UINT,
    CustomRateCount: UINT,
}}
STRUCT!{struct DXVAHD_CUSTOM_RATE_DATA {
    CustomRate: DXVAHD_RATIONAL,
    OutputFrames: UINT,
    InputInterlaced: BOOL,
    InputFramesOrFields: UINT,
}}
STRUCT!{struct DXVAHD_FILTER_RANGE_DATA {
    Minimum: INT,
    Maximum: INT,
    Default: INT,
    Multiplier: FLOAT,
}}
STRUCT!{struct DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    Enable: BOOL,
    TargetRect: RECT,
}}
STRUCT!{struct DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA {
    YCbCr: BOOL,
    BackgroundColor: DXVAHD_COLOR,
}}
STRUCT!{struct DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA {
    Value: UINT,
}}
BITFIELD!{DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA Value: UINT [
    Usage set_Usage[0..1],
    RGB_Range set_RGB_Range[1..2],
    YCbCr_Matrix set_YCbCr_Matrix[2..3],
    YCbCr_xvYCC set_YCbCr_xvYCC[3..4],
    Reserved set_Reserved[4..32],
]}
STRUCT!{struct DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    Mode: DXVAHD_ALPHA_FILL_MODE,
    StreamNumber: UINT,
}}
STRUCT!{struct DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    Enable: BOOL,
    Size: SIZE,
}}
STRUCT!{struct DXVAHD_BLT_STATE_PRIVATE_DATA {
    Guid: GUID,
    DataSize: UINT,
    pData: *mut c_void,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    Format: D3DFORMAT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    FrameFormat: DXVAHD_FRAME_FORMAT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA {
    Value: UINT,
}}
BITFIELD!{DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA Value: UINT [
    Type set_Type[0..1],
    RGB_Range set_RGB_Range[1..2],
    YCbCr_Matrix set_YCbCr_Matrix[2..3],
    YCbCr_xvYCC set_YCbCr_xvYCC[3..4],
    Reserved set_Reserved[4..32],
]}
STRUCT!{struct DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    RepeatFrame: BOOL,
    OutputRate: DXVAHD_OUTPUT_RATE,
    CustomRate: DXVAHD_RATIONAL,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    Enable: BOOL,
    SourceRect: RECT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    Enable: BOOL,
    DestinationRect: RECT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_ALPHA_DATA {
    Enable: BOOL,
    Alpha: FLOAT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_PALETTE_DATA {
    Count: UINT,
    pEntries: *mut D3DCOLOR,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    Enable: BOOL,
    Lower: FLOAT,
    Upper: FLOAT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    Enable: BOOL,
    SourceAspectRatio: DXVAHD_RATIONAL,
    DestinationAspectRatio: DXVAHD_RATIONAL,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_FILTER_DATA {
    Enable: BOOL,
    Level: INT,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_PRIVATE_DATA {
    Guid: GUID,
    DataSize: UINT,
    pData: *mut c_void,
}}
STRUCT!{struct DXVAHD_STREAM_DATA {
    Enable: BOOL,
    OutputIndex: UINT,
    InputFrameOrField: UINT,
    PastFrames: UINT,
    FutureFrames: UINT,
    ppPastSurfaces: *mut *mut IDirect3DSurface9,
    pInputSurface: *mut IDirect3DSurface9,
    ppFutureSurfaces: *mut *mut IDirect3DSurface9,
}}
STRUCT!{struct DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    Enable: BOOL,
    ITelecineFlags: UINT,
    Frames: UINT,
    InputField: UINT,
}}
RIDL!{#[uuid(0x95f12dfd, 0xd77e, 0x49be, 0x81, 0x5f, 0x57, 0xd5, 0x79, 0x63, 0x4d, 0x6d)]
interface IDXVAHD_Device(IDXVAHD_DeviceVtbl): IUnknown(IUnknownVtbl) {
    fn CreateVideoSurface(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        Usage: DWORD,
        Type: DXVAHD_SURFACE_TYPE,
        NumSurfaces: UINT,
        ppSurfaces: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn GetVideoProcessorDeviceCaps(
        pCaps: *mut DXVAHD_VPDEVCAPS,
    ) -> HRESULT,
    fn GetVideoProcessorOutputFormats(
        Count: UINT,
        pFormats: *mut D3DFORMAT,
    ) -> HRESULT,
    fn GetVideoProcessorInputFormats(
        Count: UINT,
        pFormats: *mut D3DFORMAT,
    ) -> HRESULT,
    fn GetVideoProcessorCaps(
        Count: UINT,
        pCaps: *mut DXVAHD_VPCAPS,
    ) -> HRESULT,
    fn GetVideoProcessorCustomRates(
        pVPGuid: *const GUID,
        Count: UINT,
        pRates: *mut DXVAHD_CUSTOM_RATE_DATA,
    ) -> HRESULT,
    fn GetVideoProcessorFilterRange(
        Filter: DXVAHD_FILTER,
        pRange: *mut DXVAHD_FILTER_RANGE_DATA,
    ) -> HRESULT,
    fn CreateVideoProcessor(
        pVPGuid: *const GUID,
        ppVideoProcessor: *mut *mut IDXVAHD_VideoProcessor,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x95f4edf4, 0x6e03, 0x4cd7, 0xbe, 0x1b, 0x30, 0x75, 0xd6, 0x65, 0xaa, 0x52)]
interface IDXVAHD_VideoProcessor(IDXVAHD_VideoProcessorVtbl): IUnknown(IUnknownVtbl) {
    fn SetVideoProcessBltState(
        State: DXVAHD_BLT_STATE,
        DataSize: UINT,
        pData: *const c_void,
    ) -> HRESULT,
    fn GetVideoProcessBltState(
        State: DXVAHD_BLT_STATE,
        DataSize: UINT,
        pData: *mut c_void,
    ) -> HRESULT,
    fn SetVideoProcessStreamState(
        StreamNumber: UINT,
        State: DXVAHD_STREAM_STATE,
        DataSize: UINT,
        pData: *const c_void,
    ) -> HRESULT,
    fn GetVideoProcessStreamState(
        StreamNumber: UINT,
        State: DXVAHD_STREAM_STATE,
        DataSize: UINT,
        pData: *mut c_void,
    ) -> HRESULT,
    fn VideoProcessBltHD(
        pOutputSurface: *mut IDirect3DSurface9,
        OutputFrame: UINT,
        StreamCount: UINT,
        pStreams: *const DXVAHD_STREAM_DATA,
    ) -> HRESULT,
}}
FN!{stdcall PDXVAHDSW_CreateDevice(
    pD3DDevice: *mut IDirect3DDevice9Ex,
    phDevice: *mut HANDLE,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_ProposeVideoPrivateFormat(
    hDevice: HANDLE,
    pFormat: *mut D3DFORMAT,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorDeviceCaps(
    hDevice: HANDLE,
    pContentDesc: *const DXVAHD_CONTENT_DESC,
    Usage: DXVAHD_DEVICE_USAGE,
    pCaps: *mut DXVAHD_VPDEVCAPS,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorOutputFormats(
    hDevice: HANDLE,
    pContentDesc: *const DXVAHD_CONTENT_DESC,
    Usage: DXVAHD_DEVICE_USAGE,
    Count: UINT,
    pFormats: *mut D3DFORMAT,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorInputFormats(
    hDevice: HANDLE,
    pContentDesc: *const DXVAHD_CONTENT_DESC,
    Usage: DXVAHD_DEVICE_USAGE,
    Count: UINT,
    pFormats: *mut D3DFORMAT,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorCaps(
    hDevice: HANDLE,
    pContentDesc: *const DXVAHD_CONTENT_DESC,
    Usage: DXVAHD_DEVICE_USAGE,
    Count: UINT,
    pCaps: *mut DXVAHD_VPCAPS,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorCustomRates(
    hDevice: HANDLE,
    pVPGuid: *const GUID,
    Count: UINT,
    pRates: *mut DXVAHD_CUSTOM_RATE_DATA,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessorFilterRange(
    hDevice: HANDLE,
    Filter: DXVAHD_FILTER,
    pRange: *mut DXVAHD_FILTER_RANGE_DATA,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_DestroyDevice(
    hDevice: HANDLE,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_CreateVideoProcessor(
    hDevice: HANDLE,
    pVPGuid: *const GUID,
    phVideoProcessor: *mut HANDLE,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_SetVideoProcessBltState(
    hVideoProcessor: HANDLE,
    State: DXVAHD_BLT_STATE,
    DataSize: UINT,
    pData: *const c_void,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessBltStatePrivate(
    hVideoProcessor: HANDLE,
    pData: *mut DXVAHD_BLT_STATE_PRIVATE_DATA,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_SetVideoProcessStreamState(
    hVideoProcessor: HANDLE,
    StreamNumber: UINT,
    State: DXVAHD_STREAM_STATE,
    DataSize: UINT,
    pData: *const c_void,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_GetVideoProcessStreamStatePrivate(
    hVideoProcessor: HANDLE,
    StreamNumber: UINT,
    pData: *mut DXVAHD_STREAM_STATE_PRIVATE_DATA,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_VideoProcessBltHD(
    hVideoProcessor: HANDLE,
    pOutputSurface: *mut IDirect3DSurface9,
    OutputFrame: UINT,
    StreamCount: UINT,
    pStreams: *const DXVAHD_STREAM_DATA,
) -> HRESULT}
FN!{stdcall PDXVAHDSW_DestroyVideoProcessor(
    hVideoProcessor: HANDLE,
) -> HRESULT}
STRUCT!{struct DXVAHDSW_CALLBACKS {
    CreateDevice: PDXVAHDSW_CreateDevice,
    ProposeVideoPrivateFormat: PDXVAHDSW_ProposeVideoPrivateFormat,
    GetVideoProcessorDeviceCaps: PDXVAHDSW_GetVideoProcessorDeviceCaps,
    GetVideoProcessorOutputFormats: PDXVAHDSW_GetVideoProcessorOutputFormats,
    GetVideoProcessorInputFormats: PDXVAHDSW_GetVideoProcessorInputFormats,
    GetVideoProcessorCaps: PDXVAHDSW_GetVideoProcessorCaps,
    GetVideoProcessorCustomRates: PDXVAHDSW_GetVideoProcessorCustomRates,
    GetVideoProcessorFilterRange: PDXVAHDSW_GetVideoProcessorFilterRange,
    DestroyDevice: PDXVAHDSW_DestroyDevice,
    CreateVideoProcessor: PDXVAHDSW_CreateVideoProcessor,
    SetVideoProcessBltState: PDXVAHDSW_SetVideoProcessBltState,
    GetVideoProcessBltStatePrivate: PDXVAHDSW_GetVideoProcessBltStatePrivate,
    SetVideoProcessStreamState: PDXVAHDSW_SetVideoProcessStreamState,
    GetVideoProcessStreamStatePrivate: PDXVAHDSW_GetVideoProcessStreamStatePrivate,
    VideoProcessBltHD: PDXVAHDSW_VideoProcessBltHD,
    DestroyVideoProcessor: PDXVAHDSW_DestroyVideoProcessor,
}}
FN!{stdcall PDXVAHDSW_Plugin(
    Size: UINT,
    pCallbacks: *mut c_void,
) -> HRESULT}
DEFINE_GUID!{DXVAHDControlGuid,
    0xa0386e75, 0xf70c, 0x464c, 0xa9, 0xce, 0x33, 0xc4, 0x4e, 0x09, 0x16, 0x23}
DEFINE_GUID!{DXVAHDETWGUID_CREATEVIDEOPROCESSOR,
    0x681e3d1e, 0x5674, 0x4fb3, 0xa5, 0x03, 0x2f, 0x20, 0x55, 0xe9, 0x1f, 0x60}
DEFINE_GUID!{DXVAHDETWGUID_VIDEOPROCESSBLTSTATE,
    0x76c94b5a, 0x193f, 0x4692, 0x94, 0x84, 0xa4, 0xd9, 0x99, 0xda, 0x81, 0xa8}
DEFINE_GUID!{DXVAHDETWGUID_VIDEOPROCESSSTREAMSTATE,
    0x262c0b02, 0x209d, 0x47ed, 0x94, 0xd8, 0x82, 0xae, 0x02, 0xb8, 0x4a, 0xa7}
DEFINE_GUID!{DXVAHDETWGUID_VIDEOPROCESSBLTHD,
    0xbef3d435, 0x78c7, 0x4de3, 0x97, 0x07, 0xcd, 0x1b, 0x08, 0x3b, 0x16, 0x0a}
DEFINE_GUID!{DXVAHDETWGUID_VIDEOPROCESSBLTHD_STREAM,
    0x27ae473e, 0xa5fc, 0x4be5, 0xb4, 0xe3, 0xf2, 0x49, 0x94, 0xd3, 0xc4, 0x95}
DEFINE_GUID!{DXVAHDETWGUID_DESTROYVIDEOPROCESSOR,
    0xf943f0a0, 0x3f16, 0x43e0, 0x80, 0x93, 0x10, 0x5a, 0x98, 0x6a, 0xa5, 0xf1}
STRUCT!{struct DXVAHDETW_CREATEVIDEOPROCESSOR {
    pObject: ULONGLONG,
    pD3D9Ex: ULONGLONG,
    VPGuid: GUID,
}}
STRUCT!{struct DXVAHDETW_VIDEOPROCESSBLTSTATE {
    pObject: ULONGLONG,
    State: DXVAHD_BLT_STATE,
    DataSize: UINT,
    SetState: BOOL,
}}
STRUCT!{struct DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    pObject: ULONGLONG,
    StreamNumber: UINT,
    State: DXVAHD_STREAM_STATE,
    DataSize: UINT,
    SetState: BOOL,
}}
STRUCT!{struct DXVAHDETW_VIDEOPROCESSBLTHD {
    pObject: ULONGLONG,
    pOutputSurface: ULONGLONG,
    TargetRect: RECT,
    OutputFormat: D3DFORMAT,
    ColorSpace: UINT,
    OutputFrame: UINT,
    StreamCount: UINT,
    Enter: BOOL,
}}
STRUCT!{struct DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    pObject: ULONGLONG,
    pInputSurface: ULONGLONG,
    SourceRect: RECT,
    DestinationRect: RECT,
    InputFormat: D3DFORMAT,
    FrameFormat: DXVAHD_FRAME_FORMAT,
    ColorSpace: UINT,
    StreamNumber: UINT,
    OutputIndex: UINT,
    InputFrameOrField: UINT,
    PastFrames: UINT,
    FutureFrames: UINT,
}}
STRUCT!{struct DXVAHDETW_DESTROYVIDEOPROCESSOR {
    pObject: ULONGLONG,
}}
extern "system" {
    pub fn DXVAHD_CreateDevice(
        pD3DDevice: *mut IDirect3DDevice9Ex,
        pContentDesc: *const DXVAHD_CONTENT_DESC,
        Usage: DXVAHD_DEVICE_USAGE,
        pPlugin: PDXVAHDSW_Plugin,
        ppDevice: *mut *mut IDXVAHD_Device,
    ) -> HRESULT;
}
FN!{stdcall PDXVAHD_CreateDevice(
    pD3DDevice: *mut IDirect3DDevice9Ex,
    pContentDesc: *const DXVAHD_CONTENT_DESC,
    Usage: DXVAHD_DEVICE_USAGE,
    pPlugin: PDXVAHDSW_Plugin,
    ppDevice: *mut *mut IDXVAHD_Device,
) -> HRESULT}
