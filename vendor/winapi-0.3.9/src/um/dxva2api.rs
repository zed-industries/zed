// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_float, c_void};
use shared::basetsd::UINT64;
use shared::d3d9::{IDirect3DDevice9, IDirect3DSurface9};
use shared::d3d9types::{D3DFORMAT, D3DPOOL};
use shared::guiddef::{GUID, REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, FLOAT, HIWORD, LOWORD, UCHAR, UINT, USHORT};
use shared::windef::{RECT, SIZE};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LONG, LONGLONG, PVOID, SHORT};
DEFINE_GUID!{DXVA2_ModeMPEG2_MoComp,
    0xe6a9f44b, 0x61b0, 0x4563, 0x9e, 0xa4, 0x63, 0xd2, 0xa3, 0xc6, 0xfe, 0x66}
DEFINE_GUID!{DXVA2_ModeMPEG2_IDCT,
    0xbf22ad00, 0x03ea, 0x4690, 0x80, 0x77, 0x47, 0x33, 0x46, 0x20, 0x9b, 0x7e}
DEFINE_GUID!{DXVA2_ModeMPEG2_VLD,
    0xee27417f, 0x5e28, 0x4e65, 0xbe, 0xea, 0x1d, 0x26, 0xb5, 0x08, 0xad, 0xc9}
DEFINE_GUID!{DXVA2_ModeMPEG1_VLD,
    0x6f3ec719, 0x3735, 0x42cc, 0x80, 0x63, 0x65, 0xcc, 0x3c, 0xb3, 0x66, 0x16}
DEFINE_GUID!{DXVA2_ModeMPEG2and1_VLD,
    0x86695f12, 0x340e, 0x4f04, 0x9f, 0xd3, 0x92, 0x53, 0xdd, 0x32, 0x74, 0x60}
DEFINE_GUID!{DXVA2_ModeH264_A,
    0x1b81be64, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_B,
    0x1b81be65, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_C,
    0x1b81be66, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_D,
    0x1b81be67, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_E,
    0x1b81be68, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_F,
    0x1b81be69, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeH264_VLD_WithFMOASO_NoFGT,
    0xd5f04ff9, 0x3418, 0x45d8, 0x95, 0x61, 0x32, 0xa7, 0x6a, 0xae, 0x2d, 0xdd}
DEFINE_GUID!{DXVA2_ModeH264_VLD_Stereo_Progressive_NoFGT,
    0xd79be8da, 0x0cf1, 0x4c81, 0xb8, 0x2a, 0x69, 0xa4, 0xe2, 0x36, 0xf4, 0x3d}
DEFINE_GUID!{DXVA2_ModeH264_VLD_Stereo_NoFGT,
    0xf9aaccbb, 0xc2b6, 0x4cfc, 0x87, 0x79, 0x57, 0x07, 0xb1, 0x76, 0x05, 0x52}
DEFINE_GUID!{DXVA2_ModeH264_VLD_Multiview_NoFGT,
    0x705b9d82, 0x76cf, 0x49d6, 0xb7, 0xe6, 0xac, 0x88, 0x72, 0xdb, 0x01, 0x3c}
DEFINE_GUID!{DXVA2_ModeWMV8_A,
    0x1b81be80, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeWMV8_B,
    0x1b81be81, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeWMV9_A,
    0x1b81be90, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeWMV9_B,
    0x1b81be91, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeWMV9_C,
    0x1b81be94, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeVC1_A,
    0x1b81bea0, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeVC1_B,
    0x1b81bea1, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeVC1_C,
    0x1b81bea2, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeVC1_D,
    0x1b81bea3, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_ModeVC1_D2010,
    0x1b81bea4, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_NoEncrypt,
    0x1b81bed0, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5}
DEFINE_GUID!{DXVA2_VideoProcProgressiveDevice,
    0x5a54a0c9, 0xc7ec, 0x4bd9, 0x8e, 0xde, 0xf3, 0xc7, 0x5d, 0xc4, 0x39, 0x3b}
DEFINE_GUID!{DXVA2_VideoProcBobDevice,
    0x335aa36e, 0x7884, 0x43a4, 0x9c, 0x91, 0x7f, 0x87, 0xfa, 0xf3, 0xe3, 0x7e}
DEFINE_GUID!{DXVA2_VideoProcSoftwareDevice,
    0x4553d47f, 0xee7e, 0x4e3f, 0x94, 0x75, 0xdb, 0xf1, 0x37, 0x6c, 0x48, 0x10}
DEFINE_GUID!{DXVA2_ModeMPEG4pt2_VLD_Simple,
    0xefd64d74, 0xc9e8, 0x41d7, 0xa5, 0xe9, 0xe9, 0xb0, 0xe3, 0x9f, 0xa3, 0x19}
DEFINE_GUID!{DXVA2_ModeMPEG4pt2_VLD_AdvSimple_NoGMC,
    0xed418a9f, 0x010d, 0x4eda, 0x9a, 0xe3, 0x9a, 0x65, 0x35, 0x8d, 0x8d, 0x2e}
DEFINE_GUID!{DXVA2_ModeMPEG4pt2_VLD_AdvSimple_GMC,
    0xab998b5b, 0x4258, 0x44a9, 0x9f, 0xeb, 0x94, 0xe5, 0x97, 0xa6, 0xba, 0xae}
DEFINE_GUID!{DXVA2_ModeHEVC_VLD_Main,
    0x5b11d51b, 0x2f4c, 0x4452, 0xbc, 0xc3, 0x09, 0xf2, 0xa1, 0x16, 0x0c, 0xc0}
DEFINE_GUID!{DXVA2_ModeHEVC_VLD_Main10,
    0x107af0e0, 0xef1a, 0x4d19, 0xab, 0xa8, 0x67, 0xa1, 0x63, 0x07, 0x3d, 0x13}
DEFINE_GUID!{DXVA2_ModeVP9_VLD_Profile0,
    0x463707f8, 0xa1d0, 0x4585, 0x87, 0x6d, 0x83, 0xaa, 0x6d, 0x60, 0xb8, 0x9e}
DEFINE_GUID!{DXVA2_ModeVP9_VLD_10bit_Profile2,
    0xa4c749ef, 0x6ecf, 0x48aa, 0x84, 0x48, 0x50, 0xa7, 0xa1, 0x16, 0x5f, 0xf7}
DEFINE_GUID!{DXVA2_ModeVP8_VLD,
    0x90b899ea, 0x3a62, 0x4705, 0x88, 0xb3, 0x8d, 0xf0, 0x4b, 0x27, 0x44, 0xe7}
pub const DXVA2_ModeMPEG2_MOCOMP: GUID = DXVA2_ModeMPEG2_MoComp;
pub const DXVA2_ModeWMV8_PostProc: GUID = DXVA2_ModeWMV8_A;
pub const DXVA2_ModeWMV8_MoComp: GUID = DXVA2_ModeWMV8_B;
pub const DXVA2_ModeWMV9_PostProc: GUID = DXVA2_ModeWMV9_A;
pub const DXVA2_ModeWMV9_MoComp: GUID = DXVA2_ModeWMV9_B;
pub const DXVA2_ModeWMV9_IDCT: GUID = DXVA2_ModeWMV9_C;
pub const DXVA2_ModeVC1_PostProc: GUID = DXVA2_ModeVC1_A;
pub const DXVA2_ModeVC1_MoComp: GUID = DXVA2_ModeVC1_B;
pub const DXVA2_ModeVC1_IDCT: GUID = DXVA2_ModeVC1_C;
pub const DXVA2_ModeVC1_VLD: GUID = DXVA2_ModeVC1_D;
pub const DXVA2_ModeH264_MoComp_NoFGT: GUID = DXVA2_ModeH264_A;
pub const DXVA2_ModeH264_MoComp_FGT: GUID = DXVA2_ModeH264_B;
pub const DXVA2_ModeH264_IDCT_NoFGT: GUID = DXVA2_ModeH264_C;
pub const DXVA2_ModeH264_IDCT_FGT: GUID = DXVA2_ModeH264_D;
pub const DXVA2_ModeH264_VLD_NoFGT: GUID = DXVA2_ModeH264_E;
pub const DXVA2_ModeH264_VLD_FGT: GUID = DXVA2_ModeH264_F;
pub const DXVA2_E_NOT_INITIALIZED: HRESULT = 0x80041000;
pub const DXVA2_E_NEW_VIDEO_DEVICE: HRESULT = 0x80041001;
pub const DXVA2_E_VIDEO_DEVICE_LOCKED: HRESULT = 0x80041002;
pub const DXVA2_E_NOT_AVAILABLE: HRESULT = 0x80041003;
DEFINE_GUID!{IID_IDirect3DDeviceManager9,
    0xa0cade0f, 0x06d5, 0x4cf4, 0xa1, 0xc7, 0xf3, 0xcd, 0xd7, 0x25, 0xaa, 0x75}
DEFINE_GUID!{IID_IDirectXVideoAccelerationService,
    0xfc51a550, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02}
DEFINE_GUID!{IID_IDirectXVideoDecoderService,
    0xfc51a551, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02}
DEFINE_GUID!{IID_IDirectXVideoProcessorService,
    0xfc51a552, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02}
DEFINE_GUID!{IID_IDirectXVideoDecoder,
    0xf2b0810a, 0xfd00, 0x43c9, 0x91, 0x8c, 0xdf, 0x94, 0xe2, 0xd8, 0xef, 0x7d}
DEFINE_GUID!{IID_IDirectXVideoProcessor,
    0x8c3a39f0, 0x916e, 0x4690, 0x80, 0x4f, 0x4c, 0x80, 0x01, 0x35, 0x5d, 0x25}
DEFINE_GUID!{IID_IDirectXVideoMemoryConfiguration,
    0xb7f916dd, 0xdb3b, 0x49c1, 0x84, 0xd7, 0xe4, 0x5e, 0xf9, 0x9e, 0xc7, 0x26}
pub const MAX_DEINTERLACE_SURFACES: usize = 32;
pub const MAX_SUBSTREAMS: usize = 15;
STRUCT!{struct DXVA2_ExtendedFormat {
    value: UINT,
}}
BITFIELD!{DXVA2_ExtendedFormat value: UINT [
    SampleFormat set_SampleFormat[0..8],
    VideoChromaSubsampling set_VideoChromaSubsampling[8..12],
    NominalRange set_NominalRange[12..15],
    VideoTransferMatrix set_VideoTransferMatrix[15..18],
    VideoLighting set_VideoLighting[18..22],
    VideoPrimaries set_VideoPrimaries[22..27],
    VideoTransferFunction set_VideoTransferFunction[27..32],
]}
ENUM!{enum DXVA2_SampleFormat {
    DXVA2_SampleFormatMask = 0xff,
    DXVA2_SampleUnknown = 0,
    DXVA2_SampleProgressiveFrame = 2,
    DXVA2_SampleFieldInterleavedEvenFirst = 3,
    DXVA2_SampleFieldInterleavedOddFirst = 4,
    DXVA2_SampleFieldSingleEven = 5,
    DXVA2_SampleFieldSingleOdd = 6,
    DXVA2_SampleSubStream = 7,
}}
ENUM!{enum DXVA2_VideoChromaSubSampling {
    DXVA2_VideoChromaSubsamplingMask = 0xf,
    DXVA2_VideoChromaSubsampling_Unknown = 0,
    DXVA2_VideoChromaSubsampling_ProgressiveChroma = 0x8,
    DXVA2_VideoChromaSubsampling_Horizontally_Cosited = 0x4,
    DXVA2_VideoChromaSubsampling_Vertically_Cosited = 0x2,
    DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes = 0x1,
    DXVA2_VideoChromaSubsampling_MPEG2 = DXVA2_VideoChromaSubsampling_Horizontally_Cosited |
        DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    DXVA2_VideoChromaSubsampling_MPEG1 =
        DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes,
    DXVA2_VideoChromaSubsampling_DV_PAL = DXVA2_VideoChromaSubsampling_Horizontally_Cosited |
        DXVA2_VideoChromaSubsampling_Vertically_Cosited,
    DXVA2_VideoChromaSubsampling_Cosited = DXVA2_VideoChromaSubsampling_Horizontally_Cosited |
        DXVA2_VideoChromaSubsampling_Vertically_Cosited |
        DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes,
}}
ENUM!{enum DXVA2_NominalRange {
    DXVA2_NominalRangeMask = 0x7,
    DXVA2_NominalRange_Unknown = 0,
    DXVA2_NominalRange_Normal = 1,
    DXVA2_NominalRange_Wide = 2,
    DXVA2_NominalRange_0_255 = 1,
    DXVA2_NominalRange_16_235 = 2,
    DXVA2_NominalRange_48_208 = 3,
}}
ENUM!{enum DXVA2_VideoTransferMatrix {
    DXVA2_VideoTransferMatrixMask = 0x7,
    DXVA2_VideoTransferMatrix_Unknown = 0,
    DXVA2_VideoTransferMatrix_BT709 = 1,
    DXVA2_VideoTransferMatrix_BT601 = 2,
    DXVA2_VideoTransferMatrix_SMPTE240M = 3,
}}
ENUM!{enum DXVA2_VideoLighting {
    DXVA2_VideoLightingMask = 0xf,
    DXVA2_VideoLighting_Unknown = 0,
    DXVA2_VideoLighting_bright = 1,
    DXVA2_VideoLighting_office = 2,
    DXVA2_VideoLighting_dim = 3,
    DXVA2_VideoLighting_dark = 4,
}}
ENUM!{enum DXVA2_VideoPrimaries {
    DXVA2_VideoPrimariesMask = 0x1f,
    DXVA2_VideoPrimaries_Unknown = 0,
    DXVA2_VideoPrimaries_reserved = 1,
    DXVA2_VideoPrimaries_BT709 = 2,
    DXVA2_VideoPrimaries_BT470_2_SysM = 3,
    DXVA2_VideoPrimaries_BT470_2_SysBG = 4,
    DXVA2_VideoPrimaries_SMPTE170M = 5,
    DXVA2_VideoPrimaries_SMPTE240M = 6,
    DXVA2_VideoPrimaries_EBU3213 = 7,
    DXVA2_VideoPrimaries_SMPTE_C = 8,
}}
ENUM!{enum DXVA2_VideoTransferFunction {
    DXVA2_VideoTransFuncMask = 0x1f,
    DXVA2_VideoTransFunc_Unknown = 0,
    DXVA2_VideoTransFunc_10 = 1,
    DXVA2_VideoTransFunc_18 = 2,
    DXVA2_VideoTransFunc_20 = 3,
    DXVA2_VideoTransFunc_22 = 4,
    DXVA2_VideoTransFunc_709 = 5,
    DXVA2_VideoTransFunc_240M = 6,
    DXVA2_VideoTransFunc_sRGB = 7,
    DXVA2_VideoTransFunc_28 = 8,
}}
pub const DXVA2_VideoTransFunc_22_709: DWORD = DXVA2_VideoTransFunc_709;
pub const DXVA2_VideoTransFunc_22_240M: DWORD = DXVA2_VideoTransFunc_240M;
pub const DXVA2_VideoTransFunc_22_8bit_sRGB: DWORD = DXVA2_VideoTransFunc_sRGB;
STRUCT!{struct DXVA2_Frequency {
    Numerator: UINT,
    Denominator: UINT,
}}
STRUCT!{struct DXVA2_VideoDesc {
    SampleWidth: UINT,
    SampleHeight: UINT,
    SampleFormat: DXVA2_ExtendedFormat,
    Format: D3DFORMAT,
    InputSampleFreq: DXVA2_Frequency,
    OutputFrameFreq: DXVA2_Frequency,
    UABProtectionLevel: UINT,
    Reserved: UINT,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0003 {
    DXVA2_DeinterlaceTech_Unknown = 0,
    DXVA2_DeinterlaceTech_BOBLineReplicate = 0x1,
    DXVA2_DeinterlaceTech_BOBVerticalStretch = 0x2,
    DXVA2_DeinterlaceTech_BOBVerticalStretch4Tap = 0x4,
    DXVA2_DeinterlaceTech_MedianFiltering = 0x8,
    DXVA2_DeinterlaceTech_EdgeFiltering = 0x10,
    DXVA2_DeinterlaceTech_FieldAdaptive = 0x20,
    DXVA2_DeinterlaceTech_PixelAdaptive = 0x40,
    DXVA2_DeinterlaceTech_MotionVectorSteered = 0x80,
    DXVA2_DeinterlaceTech_InverseTelecine = 0x100,
    DXVA2_DeinterlaceTech_Mask = 0x1ff,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0004 {
    DXVA2_NoiseFilterLumaLevel = 1,
    DXVA2_NoiseFilterLumaThreshold = 2,
    DXVA2_NoiseFilterLumaRadius = 3,
    DXVA2_NoiseFilterChromaLevel = 4,
    DXVA2_NoiseFilterChromaThreshold = 5,
    DXVA2_NoiseFilterChromaRadius = 6,
    DXVA2_DetailFilterLumaLevel = 7,
    DXVA2_DetailFilterLumaThreshold = 8,
    DXVA2_DetailFilterLumaRadius = 9,
    DXVA2_DetailFilterChromaLevel = 10,
    DXVA2_DetailFilterChromaThreshold = 11,
    DXVA2_DetailFilterChromaRadius = 12,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0005 {
    DXVA2_NoiseFilterTech_Unsupported = 0,
    DXVA2_NoiseFilterTech_Unknown = 0x1,
    DXVA2_NoiseFilterTech_Median = 0x2,
    DXVA2_NoiseFilterTech_Temporal = 0x4,
    DXVA2_NoiseFilterTech_BlockNoise = 0x8,
    DXVA2_NoiseFilterTech_MosquitoNoise = 0x10,
    DXVA2_NoiseFilterTech_Mask = 0x1f,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0006 {
    DXVA2_DetailFilterTech_Unsupported = 0,
    DXVA2_DetailFilterTech_Unknown = 0x1,
    DXVA2_DetailFilterTech_Edge = 0x2,
    DXVA2_DetailFilterTech_Sharpening = 0x4,
    DXVA2_DetailFilterTech_Mask = 0x7,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0007 {
    DXVA2_ProcAmp_None = 0,
    DXVA2_ProcAmp_Brightness = 0x1,
    DXVA2_ProcAmp_Contrast = 0x2,
    DXVA2_ProcAmp_Hue = 0x4,
    DXVA2_ProcAmp_Saturation = 0x8,
    DXVA2_ProcAmp_Mask = 0xf,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0008 {
    DXVA2_VideoProcess_None = 0,
    DXVA2_VideoProcess_YUV2RGB = 0x1,
    DXVA2_VideoProcess_StretchX = 0x2,
    DXVA2_VideoProcess_StretchY = 0x4,
    DXVA2_VideoProcess_AlphaBlend = 0x8,
    DXVA2_VideoProcess_SubRects = 0x10,
    DXVA2_VideoProcess_SubStreams = 0x20,
    DXVA2_VideoProcess_SubStreamsExtended = 0x40,
    DXVA2_VideoProcess_YUV2RGBExtended = 0x80,
    DXVA2_VideoProcess_AlphaBlendExtended = 0x100,
    DXVA2_VideoProcess_Constriction = 0x200,
    DXVA2_VideoProcess_NoiseFilter = 0x400,
    DXVA2_VideoProcess_DetailFilter = 0x800,
    DXVA2_VideoProcess_PlanarAlpha = 0x1000,
    DXVA2_VideoProcess_LinearScaling = 0x2000,
    DXVA2_VideoProcess_GammaCompensated = 0x4000,
    DXVA2_VideoProcess_MaintainsOriginalFieldData = 0x8000,
    DXVA2_VideoProcess_Mask = 0xffff,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0009 {
    DXVA2_VPDev_HardwareDevice = 0x1,
    DXVA2_VPDev_EmulatedDXVA1 = 0x2,
    DXVA2_VPDev_SoftwareDevice = 0x4,
    DXVA2_VPDev_Mask = 0x7,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0010 {
    DXVA2_SampleData_RFF = 0x1,
    DXVA2_SampleData_TFF = 0x2,
    DXVA2_SampleData_RFF_TFF_Present = 0x4,
    DXVA2_SampleData_Mask = 0xffff,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0011 {
    DXVA2_DestData_RFF = 0x1,
    DXVA2_DestData_TFF = 0x2,
    DXVA2_DestData_RFF_TFF_Present = 0x4,
    DXVA2_DestData_Mask = 0xffff,
}}
STRUCT!{struct DXVA2_VideoProcessorCaps {
    DeviceCaps: UINT,
    InputPool: D3DPOOL,
    NumForwardRefSamples: UINT,
    NumBackwardRefSamples: UINT,
    Reserved: UINT,
    DeinterlaceTechnology: UINT,
    ProcAmpControlCaps: UINT,
    VideoProcessorOperations: UINT,
    NoiseFilterTechnology: UINT,
    DetailFilterTechnology: UINT,
}}
STRUCT!{struct DXVA2_Fixed32_s {
    Fraction: USHORT,
    Value: SHORT,
}}
UNION!{union DXVA2_Fixed32 {
    [u32; 1],
    s s_mut: DXVA2_Fixed32_s,
    ll s_ll: LONG,
}}
STRUCT!{struct DXVA2_AYUVSample8 {
    Cr: UCHAR,
    Cb: UCHAR,
    Y: UCHAR,
    Alpha: UCHAR,
}}
STRUCT!{struct DXVA2_AYUVSample16 {
    Cr: USHORT,
    Cb: USHORT,
    Y: USHORT,
    Alpha: USHORT,
}}
pub type REFERENCE_TIME = LONGLONG;
STRUCT!{struct DXVA2_VideoSample {
    Start: REFERENCE_TIME,
    End: REFERENCE_TIME,
    SampleFormat: DXVA2_ExtendedFormat,
    SrcSurface: *mut IDirect3DSurface9,
    SrcRect: RECT,
    DstRect: RECT,
    Pal: [DXVA2_AYUVSample8; 16],
    PlanarAlpha: DXVA2_Fixed32,
    SampleData: DWORD,
}}
STRUCT!{struct DXVA2_ValueRange {
    MinValue: DXVA2_Fixed32,
    MaxValue: DXVA2_Fixed32,
    DefaultValue: DXVA2_Fixed32,
    StepSize: DXVA2_Fixed32,
}}
STRUCT!{struct DXVA2_ProcAmpValues {
    Brightness: DXVA2_Fixed32,
    Contrast: DXVA2_Fixed32,
    Hue: DXVA2_Fixed32,
    Saturation: DXVA2_Fixed32,
}}
STRUCT!{struct DXVA2_FilterValues {
    Level: DXVA2_Fixed32,
    Threshold: DXVA2_Fixed32,
    Radius: DXVA2_Fixed32,
}}
STRUCT!{struct DXVA2_VideoProcessBltParams {
    TargetFrame: REFERENCE_TIME,
    TargetRect: RECT,
    ConstrictionSize: SIZE,
    StreamingFlags: UINT,
    BackgroundColor: DXVA2_AYUVSample16,
    DestFormat: DXVA2_ExtendedFormat,
    ProcAmpValues: DXVA2_ProcAmpValues,
    Alpha: DXVA2_Fixed32,
    NoiseFilterLuma: DXVA2_FilterValues,
    NoiseFilterChroma: DXVA2_FilterValues,
    DetailFilterLuma: DXVA2_FilterValues,
    DetailFilterChroma: DXVA2_FilterValues,
    DestData: DWORD,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0012 {
    DXVA2_PictureParametersBufferType = 0,
    DXVA2_MacroBlockControlBufferType = 1,
    DXVA2_ResidualDifferenceBufferType = 2,
    DXVA2_DeblockingControlBufferType = 3,
    DXVA2_InverseQuantizationMatrixBufferType = 4,
    DXVA2_SliceControlBufferType = 5,
    DXVA2_BitStreamDateBufferType = 6,
    DXVA2_MotionVectorBuffer = 7,
    DXVA2_FilmGrainBuffer = 8,
}}
ENUM!{enum __MIDL___MIDL_itf_dxva2api_0000_0000_0013 {
    DXVA2_VideoDecoderRenderTarget = 0,
    DXVA2_VideoProcessorRenderTarget = 1,
    DXVA2_VideoSoftwareRenderTarget = 2,
}}
STRUCT!{struct DXVA2_ConfigPictureDecode {
    guidConfigBitstreamEncryption: GUID,
    guidConfigMBcontrolEncryption: GUID,
    guidConfigResidDiffEncryption: GUID,
    ConfigBitstreamRaw: UINT,
    ConfigMBcontrolRasterOrder: UINT,
    ConfigResidDiffHost: UINT,
    ConfigSpatialResid8: UINT,
    ConfigResid8Subtraction: UINT,
    ConfigSpatialHost8or9Clipping: UINT,
    ConfigSpatialResidInterleaved: UINT,
    ConfigIntraResidUnsigned: UINT,
    ConfigResidDiffAccelerator: UINT,
    ConfigHostInverseScan: UINT,
    ConfigSpecificIDCT: UINT,
    Config4GroupedCoefs: UINT,
    ConfigMinRenderTargetBuffCount: USHORT,
    ConfigDecoderSpecific: USHORT,
}}
STRUCT!{struct DXVA2_DecodeBufferDesc {
    CompressedBufferType: DWORD,
    BufferIndex: UINT,
    DataOffset: UINT,
    DataSize: UINT,
    FirstMBaddress: UINT,
    NumMBsInBuffer: UINT,
    Width: UINT,
    Height: UINT,
    Stride: UINT,
    ReservedBits: UINT,
    pvPVPState: PVOID,
}}
STRUCT!{struct DXVA2_AES_CTR_IV {
    IV: UINT64,
    Count: UINT64,
}}
STRUCT!{struct DXVA2_DecodeExtensionData {
    Function: UINT,
    pPrivateInputData: PVOID,
    PrivateInputDataSize: UINT,
    pPrivateOutputData: PVOID,
    PrivateOutputDataSize: UINT,
}}
pub const DXVA2_DECODE_GET_DRIVER_HANDLE: UINT = 0x725;
pub const DXVA2_DECODE_SPECIFY_ENCRYPTED_BLOCKS: UINT = 0x724;
STRUCT!{struct DXVA2_DecodeExecuteParams {
    NumCompBuffers: UINT,
    pCompressedBuffers: *mut DXVA2_DecodeBufferDesc,
    pExtensionData: *mut DXVA2_DecodeExtensionData,
}}
RIDL!{#[uuid(0xa0cade0f, 0x06d5, 0x4cf4, 0xa1, 0xc7, 0xf3, 0xcd, 0xd7, 0x25, 0xaa, 0x75)]
interface IDirect3DDeviceManager9(IDirect3DDeviceManager9Vtbl): IUnknown(IUnknownVtbl) {
    fn ResetDevice(
        pDevice: *mut IDirect3DDevice9,
        resetToken: UINT,
    ) -> HRESULT,
    fn OpenDeviceHandle(
        phDevice: *mut HANDLE,
    ) -> HRESULT,
    fn CloseDeviceHandle(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn TestDevice(
        hDevice: HANDLE,
    ) -> HRESULT,
    fn LockDevice(
        hDevice: HANDLE,
        ppDevice: *mut *mut IDirect3DDevice9,
        fBloc: BOOL,
    ) -> HRESULT,
    fn UnlockDevice(
        hDevice: HANDLE,
        fSaveState: BOOL,
    ) -> HRESULT,
    fn GetVideoService(
        hDevice: HANDLE,
        riid: REFIID,
        ppService: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfc51a550, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02)]
interface IDirectXVideoAccelerationService(IDirectXVideoAccelerationServiceVtbl):
    IUnknown(IUnknownVtbl) {
    fn CreateSurface(
        Width: UINT,
        Height: UINT,
        BackBuffers: UINT,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        Usage: DWORD,
        DxvaType: DWORD,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfc51a551, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02)]
interface IDirectXVideoDecoderService(IDirectXVideoDecoderServiceVtbl):
    IDirectXVideoAccelerationService(IDirectXVideoAccelerationServiceVtbl) {
    fn GetDecoderDeviceGuids(
        pCount: *mut UINT,
        pGuids: *mut *mut GUID,
    ) -> HRESULT,
    fn GetDecoderRenderTargets(
        Guid: REFGUID,
        pCount: *mut UINT,
        pFormats: *mut *mut D3DFORMAT,
    ) -> HRESULT,
    fn GetDecoderConfigurations(
        Guid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        pReserved: *mut c_void,
        pCount: *mut UINT,
        ppConfigs: *mut *mut DXVA2_ConfigPictureDecode,
    ) -> HRESULT,
    fn CreateVideoDecoder(
        Guid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        pConfig: *const DXVA2_ConfigPictureDecode,
        ppDecoderRenderTargets: *mut *mut IDirect3DSurface9,
        NumRenderTargets: UINT,
        ppDecode: *mut *mut IDirectXVideoDecoder,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfc51a552, 0xd5e7, 0x11d9, 0xaf, 0x55, 0x00, 0x05, 0x4e, 0x43, 0xff, 0x02)]
interface IDirectXVideoProcessorService(IDirectXVideoProcessorServiceVtbl):
    IDirectXVideoAccelerationService(IDirectXVideoAccelerationServiceVtbl) {
    fn RegisterVideoProcessorSoftwareDevice(
        pCallbacks: *mut c_void,
    ) -> HRESULT,
    fn GetVideoProcessorDeviceGuids(
        pVideoDesc: *mut DXVA2_VideoDesc,
        pCount: *mut UINT,
        pGuids: *mut *mut GUID,
    ) -> HRESULT,
    fn GetVideoProcessorRenderTargets(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        pCount: *mut UINT,
        pFormats: *mut *mut D3DFORMAT,
    ) -> HRESULT,
    fn GetVideoProcessorSubStreamFormats(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        RenderTargetFormat: D3DFORMAT,
        pCount: *mut UINT,
        pFormats: *mut *mut D3DFORMAT,
    ) -> HRESULT,
    fn GetVideoProcessorCaps(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        RenderTargetFormat: D3DFORMAT,
        pCaps: *mut DXVA2_VideoProcessorCaps,
    ) -> HRESULT,
    fn GetProcAmpRange(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        RenderTargetFormat: D3DFORMAT,
        ProcAmpCap: UINT,
        pRange: *mut DXVA2_ValueRange,
    ) -> HRESULT,
    fn GetFilterPropertyRange(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        RenderTargetFormat: D3DFORMAT,
        FilterSetting: UINT,
        pRange: *mut DXVA2_ValueRange,
    ) -> HRESULT,
    fn CreateVideoProcessor(
        VideoProcDeviceGuid: REFGUID,
        pVideoDesc: *const DXVA2_VideoDesc,
        RenderTargetFormat: D3DFORMAT,
        MaxNumSubStreams: UINT,
        ppVidProcess: *mut *mut IDirectXVideoProcessor,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf2b0810a, 0xfd00, 0x43c9, 0x91, 0x8c, 0xdf, 0x94, 0xe2, 0xd8, 0xef, 0x7d)]
interface IDirectXVideoDecoder(IDirectXVideoDecoderVtbl): IUnknown(IUnknownVtbl) {
    fn GetVideoDecoderService(
        ppService: *mut *mut IDirectXVideoDecoderService,
    ) -> HRESULT,
    fn GetCreationParameters(
        pDeviceGuid: *mut GUID,
        pVideoDesc: *mut DXVA2_VideoDesc,
        pConfig: *mut DXVA2_ConfigPictureDecode,
        pDecoderRenderTargets: *mut *mut *mut IDirect3DSurface9,
        pNumSurfaces: *mut UINT,
    ) -> HRESULT,
    fn GetBuffer(
        BufferType: UINT,
        ppBuffer: *mut *mut c_void,
        pBufferSize: *mut UINT,
    ) -> HRESULT,
    fn ReleaseBuffer(
        BufferType: UINT,
    ) -> HRESULT,
    fn BeginFrame(
        pRenderTarget: *mut IDirect3DSurface9,
        pvPVPData: *mut c_void,
    ) -> HRESULT,
    fn EndFrame(
        pHandleComplete: *mut HANDLE,
    ) -> HRESULT,
    fn Execute(
        pExecuteParams: *const DXVA2_DecodeExecuteParams,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x8c3a39f0, 0x916e, 0x4690, 0x80, 0x4f, 0x4c, 0x80, 0x01, 0x35, 0x5d, 0x25)]
interface IDirectXVideoProcessor(IDirectXVideoProcessorVtbl): IUnknown(IUnknownVtbl) {
    fn GetVideoProcessorService(
        ppService: *mut *mut IDirectXVideoProcessorService,
    ) -> HRESULT,
    fn GetCreationParameters(
        pDeviceGuid: *mut GUID,
        pVideoDesc: *mut DXVA2_VideoDesc,
        pRenderTargetFormat: *mut D3DFORMAT,
        pMaxNumSubStreams: *mut UINT,
    ) -> HRESULT,
    fn GetVideoProcessorCaps(
        pCaps: *mut DXVA2_VideoProcessorCaps,
    ) -> HRESULT,
    fn GetProcAmpRange(
        ProcAmpCap: UINT,
        pRange: *mut DXVA2_ValueRange,
    ) -> HRESULT,
    fn GetFilterPropertyRange(
        FilterSetting: UINT,
        pRange: *mut DXVA2_ValueRange,
    ) -> HRESULT,
    fn VideoProcessBlt(
        pRenderTarget: *mut IDirect3DSurface9,
        pBltParams: *const DXVA2_VideoProcessBltParams,
        pSamples: *const DXVA2_VideoSample,
        NumSamples: UINT,
        pHandleComplete: *mut HANDLE,
    ) -> HRESULT,
}}
ENUM!{enum DXVA2_SurfaceType {
    DXVA2_SurfaceType_DecoderRenderTarget = 0,
    DXVA2_SurfaceType_ProcessorRenderTarget = 1,
    DXVA2_SurfaceType_D3DRenderTargetTexture = 2,
}}
RIDL!{#[uuid(0xb7f916dd, 0xdb3b, 0x49c1, 0x84, 0xd7, 0xe4, 0x5e, 0xf9, 0x9e, 0xc7, 0x26)]
interface IDirectXVideoMemoryConfiguration(IDirectXVideoMemoryConfigurationVtbl):
    IUnknown(IUnknownVtbl) {
    fn GetAvailableSurfaceTypeByIndex(
        dwTypeIndex: DWORD,
        pdwType: *mut DXVA2_SurfaceType,
    ) -> HRESULT,
    fn SetSurfaceType(
        dwType: DXVA2_SurfaceType,
    ) -> HRESULT,
}}
extern "system" {
    pub fn DXVA2CreateDirect3DDeviceManager9(
        pResetToken: *mut UINT,
        ppDeviceManager: *mut *mut IDirect3DDeviceManager9,
    ) -> HRESULT;
    pub fn DXVA2CreateVideoService(
        pDD: *mut IDirect3DDevice9,
        riid: REFIID,
        ppService: *mut *mut c_void,
    ) -> HRESULT;
}
#[inline]
pub fn DXVA2FloatToFixed(_float_: c_float) -> DXVA2_Fixed32 {
    unsafe {
        let mut _fixed_: DXVA2_Fixed32 = ::core::mem::uninitialized();
        _fixed_.s_mut().Fraction = LOWORD((_float_ * 0x10000 as c_float) as DWORD);
        _fixed_.s_mut().Value = HIWORD((_float_ * 0x10000 as c_float) as DWORD) as SHORT;
        _fixed_
    }
}
#[inline]
pub fn DXVA2FixedToFloat(_fixed_: DXVA2_Fixed32) -> c_float {
    unsafe {
        _fixed_.s().Value as FLOAT + _fixed_.s().Fraction as FLOAT / 0x10000 as FLOAT
    }
}
#[inline]
pub fn DXVA2_Fixed32TransparentAlpha() -> DXVA2_Fixed32 {
    unsafe {
        let mut _fixed_: DXVA2_Fixed32 = ::core::mem::uninitialized();
        _fixed_.s_mut().Fraction = 0;
        _fixed_.s_mut().Value = 0;
        _fixed_
    }
}
#[inline]
pub fn DXVA2_Fixed32OpaqueAlpha() -> DXVA2_Fixed32 {
    unsafe {
        let mut _fixed_: DXVA2_Fixed32 = ::core::mem::uninitialized();
        _fixed_.s_mut().Fraction = 0;
        _fixed_.s_mut().Value = 1;
        _fixed_
    }
}
