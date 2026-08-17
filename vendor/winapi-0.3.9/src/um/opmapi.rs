// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::d3d9::IDirect3DDevice9;
use shared::d3d9types::D3DFORMAT;
use shared::guiddef::GUID;
use shared::minwindef::{BYTE, DWORD, ULONG};
use shared::windef::HMONITOR;
use um::dxva2api::DXVA2_SampleFormat;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LUID};
DEFINE_GUID!{OPM_GET_CURRENT_HDCP_SRM_VERSION,
    0x99c5ceff, 0x5f1d, 0x4879, 0x81, 0xc1, 0xc5, 0x24, 0x43, 0xc9, 0x48, 0x2b}
DEFINE_GUID!{OPM_GET_CONNECTED_HDCP_DEVICE_INFORMATION,
    0x0db59d74, 0xa992, 0x492e, 0xa0, 0xbd, 0xc2, 0x3f, 0xda, 0x56, 0x4e, 0x00}
DEFINE_GUID!{OPM_GET_ACP_AND_CGMSA_SIGNALING,
    0x6629a591, 0x3b79, 0x4cf3, 0x92, 0x4a, 0x11, 0xe8, 0xe7, 0x81, 0x16, 0x71}
DEFINE_GUID!{OPM_GET_CONNECTOR_TYPE,
    0x81d0bfd5, 0x6afe, 0x48c2, 0x99, 0xc0, 0x95, 0xa0, 0x8f, 0x97, 0xc5, 0xda}
DEFINE_GUID!{OPM_GET_SUPPORTED_PROTECTION_TYPES,
    0x38f2a801, 0x9a6c, 0x48bb, 0x91, 0x07, 0xb6, 0x69, 0x6e, 0x6f, 0x17, 0x97}
DEFINE_GUID!{OPM_GET_VIRTUAL_PROTECTION_LEVEL,
    0xb2075857, 0x3eda, 0x4d5d, 0x88, 0xdb, 0x74, 0x8f, 0x8c, 0x1a, 0x05, 0x49}
DEFINE_GUID!{OPM_GET_ACTUAL_PROTECTION_LEVEL,
    0x1957210a, 0x7766, 0x452a, 0xb9, 0x9a, 0xd2, 0x7a, 0xed, 0x54, 0xf0, 0x3a}
DEFINE_GUID!{OPM_GET_ACTUAL_OUTPUT_FORMAT,
    0xd7bf1ba3, 0xad13, 0x4f8e, 0xaf, 0x98, 0x0d, 0xcb, 0x3c, 0xa2, 0x04, 0xcc}
DEFINE_GUID!{OPM_GET_ADAPTER_BUS_TYPE,
    0xc6f4d673, 0x6174, 0x4184, 0x8e, 0x35, 0xf6, 0xdb, 0x52, 0x0, 0xbc, 0xba}
DEFINE_GUID!{OPM_GET_OUTPUT_ID,
    0x72cb6df3, 0x244f, 0x40ce, 0xb0, 0x9e, 0x20, 0x50, 0x6a, 0xf6, 0x30, 0x2f}
DEFINE_GUID!{OPM_GET_DVI_CHARACTERISTICS,
    0xa470b3bb, 0x5dd7, 0x4172, 0x83, 0x9c, 0x3d, 0x37, 0x76, 0xe0, 0xeb, 0xf5}
DEFINE_GUID!{OPM_GET_CODEC_INFO,
    0x4f374491, 0x8f5f, 0x4445, 0x9d, 0xba, 0x95, 0x58, 0x8f, 0x6b, 0x58, 0xb4}
DEFINE_GUID!{OPM_GET_OUTPUT_HARDWARE_PROTECTION_SUPPORT,
    0x3b129589, 0x2af8, 0x4ef0, 0x96, 0xa2, 0x70, 0x4a, 0x84, 0x5a, 0x21, 0x8e}
DEFINE_GUID!{OPM_SET_PROTECTION_LEVEL,
    0x9bb9327c, 0x4eb5, 0x4727, 0x9f, 0x00, 0xb4, 0x2b, 0x09, 0x19, 0xc0, 0xda}
DEFINE_GUID!{OPM_SET_ACP_AND_CGMSA_SIGNALING,
    0x09a631a5, 0xd684, 0x4c60, 0x8e, 0x4d, 0xd3, 0xbb, 0x0f, 0x0b, 0xe3, 0xee}
DEFINE_GUID!{OPM_SET_HDCP_SRM,
    0x8b5ef5d1, 0xc30d, 0x44ff, 0x84, 0xa5, 0xea, 0x71, 0xdc, 0xe7, 0x8f, 0x13}
DEFINE_GUID!{OPM_SET_PROTECTION_LEVEL_ACCORDING_TO_CSS_DVD,
    0x39ce333e, 0x4cc0, 0x44ae, 0xbf, 0xcc, 0xda, 0x50, 0xb5, 0xf8, 0x2e, 0x72}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0001 {
    OPM_OMAC_SIZE = 16,
    OPM_128_BIT_RANDOM_NUMBER_SIZE = 16,
    OPM_ENCRYPTED_INITIALIZATION_PARAMETERS_SIZE = 256,
    OPM_CONFIGURE_SETTING_DATA_SIZE = 4056,
    OPM_GET_INFORMATION_PARAMETERS_SIZE = 4056,
    OPM_REQUESTED_INFORMATION_SIZE = 4076,
    OPM_HDCP_KEY_SELECTION_VECTOR_SIZE = 5,
    OPM_PROTECTION_TYPE_SIZE = 4,
    OPM_BUS_TYPE_MASK = 0xffff,
    OPM_BUS_IMPLEMENTATION_MODIFIER_MASK = 0x7fff,
}}
ENUM!{enum OPM_VIDEO_OUTPUT_SEMANTICS {
    OPM_VOS_COPP_SEMANTICS = 0,
    OPM_VOS_OPM_SEMANTICS = 1,
    OPM_VOS_OPM_INDIRECT_DISPLAY = 2,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0002 {
    OPM_HDCP_FLAG_NONE = 0,
    OPM_HDCP_FLAG_REPEATER = 0x1,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0003 {
    OPM_STATUS_NORMAL = 0,
    OPM_STATUS_LINK_LOST = 0x1,
    OPM_STATUS_RENEGOTIATION_REQUIRED = 0x2,
    OPM_STATUS_TAMPERING_DETECTED = 0x4,
    OPM_STATUS_REVOKED_HDCP_DEVICE_ATTACHED = 0x8,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0004 {
    OPM_CONNECTOR_TYPE_OTHER = -1i32 as u32,
    OPM_CONNECTOR_TYPE_VGA = 0,
    OPM_CONNECTOR_TYPE_SVIDEO = 1,
    OPM_CONNECTOR_TYPE_COMPOSITE_VIDEO = 2,
    OPM_CONNECTOR_TYPE_COMPONENT_VIDEO = 3,
    OPM_CONNECTOR_TYPE_DVI = 4,
    OPM_CONNECTOR_TYPE_HDMI = 5,
    OPM_CONNECTOR_TYPE_LVDS = 6,
    OPM_CONNECTOR_TYPE_D_JPN = 8,
    OPM_CONNECTOR_TYPE_SDI = 9,
    OPM_CONNECTOR_TYPE_DISPLAYPORT_EXTERNAL = 10,
    OPM_CONNECTOR_TYPE_DISPLAYPORT_EMBEDDED = 11,
    OPM_CONNECTOR_TYPE_UDI_EXTERNAL = 12,
    OPM_CONNECTOR_TYPE_UDI_EMBEDDED = 13,
    OPM_CONNECTOR_TYPE_RESERVED = 14,
    OPM_CONNECTOR_TYPE_MIRACAST = 15,
    OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A = 16,
    OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B = 17,
    OPM_COPP_COMPATIBLE_CONNECTOR_TYPE_INTERNAL = 0x80000000,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0005 {
    OPM_DVI_CHARACTERISTIC_1_0 = 1,
    OPM_DVI_CHARACTERISTIC_1_1_OR_ABOVE = 2,
}}
ENUM!{enum OPM_OUTPUT_HARDWARE_PROTECTION {
    OPM_OUTPUT_HARDWARE_PROTECTION_NOT_SUPPORTED = 0,
    OPM_OUTPUT_HARDWARE_PROTECTION_SUPPORTED = 0x1,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0006 {
    OPM_BUS_TYPE_OTHER = 0,
    OPM_BUS_TYPE_PCI = 0x1,
    OPM_BUS_TYPE_PCIX = 0x2,
    OPM_BUS_TYPE_PCIEXPRESS = 0x3,
    OPM_BUS_TYPE_AGP = 0x4,
    OPM_BUS_IMPLEMENTATION_MODIFIER_INSIDE_OF_CHIPSET = 0x10000,
    OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP = 0x20000,
    OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET = 0x30000,
    OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR = 0x40000,
    OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE = 0x50000,
    OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD = 0x80000000,
    OPM_COPP_COMPATIBLE_BUS_TYPE_INTEGRATED = 0x80000000,
}}
ENUM!{enum OPM_DPCP_PROTECTION_LEVEL {
    OPM_DPCP_OFF = 0,
    OPM_DPCP_ON = 1,
    OPM_DPCP_FORCE_ULONG = 0x7fffffff,
}}
ENUM!{enum OPM_HDCP_PROTECTION_LEVEL {
    OPM_HDCP_OFF = 0,
    OPM_HDCP_ON = 1,
    OPM_HDCP_FORCE_ULONG = 0x7fffffff,
}}
ENUM!{enum OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL {
    OPM_TYPE_ENFORCEMENT_HDCP_OFF = OPM_HDCP_OFF,
    OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_NO_TYPE_RESTRICTION = OPM_HDCP_ON,
    OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_TYPE1_RESTRICTION = OPM_HDCP_ON + 1,
    OPM_TYPE_ENFORCEMENT_HDCP_FORCE_ULONG = 0x7fffffff,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0007 {
    OPM_CGMSA_OFF = 0,
    OPM_CGMSA_COPY_FREELY = 0x1,
    OPM_CGMSA_COPY_NO_MORE = 0x2,
    OPM_CGMSA_COPY_ONE_GENERATION = 0x3,
    OPM_CGMSA_COPY_NEVER = 0x4,
    OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED = 0x8,
}}
ENUM!{enum OPM_ACP_PROTECTION_LEVEL {
    OPM_ACP_OFF = 0,
    OPM_ACP_LEVEL_ONE = 1,
    OPM_ACP_LEVEL_TWO = 2,
    OPM_ACP_LEVEL_THREE = 3,
    OPM_ACP_FORCE_ULONG = 0x7fffffff,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0008 {
    OPM_PROTECTION_TYPE_OTHER = 0x80000000,
    OPM_PROTECTION_TYPE_NONE = 0,
    OPM_PROTECTION_TYPE_COPP_COMPATIBLE_HDCP = 0x1,
    OPM_PROTECTION_TYPE_ACP = 0x2,
    OPM_PROTECTION_TYPE_CGMSA = 0x4,
    OPM_PROTECTION_TYPE_HDCP = 0x8,
    OPM_PROTECTION_TYPE_DPCP = 0x10,
    OPM_PROTECTION_TYPE_TYPE_ENFORCEMENT_HDCP = 0x20,
}}
ENUM!{enum __MIDL___MIDL_itf_opmapi_0000_0000_0009 {
    OPM_PROTECTION_STANDARD_OTHER = 0x80000000,
    OPM_PROTECTION_STANDARD_NONE = 0,
    OPM_PROTECTION_STANDARD_IEC61880_525I = 0x1,
    OPM_PROTECTION_STANDARD_IEC61880_2_525I = 0x2,
    OPM_PROTECTION_STANDARD_IEC62375_625P = 0x4,
    OPM_PROTECTION_STANDARD_EIA608B_525 = 0x8,
    OPM_PROTECTION_STANDARD_EN300294_625I = 0x10,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEA_525P = 0x20,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEA_750P = 0x40,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEA_1125I = 0x80,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEB_525P = 0x100,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEB_750P = 0x200,
    OPM_PROTECTION_STANDARD_CEA805A_TYPEB_1125I = 0x400,
    OPM_PROTECTION_STANDARD_ARIBTRB15_525I = 0x800,
    OPM_PROTECTION_STANDARD_ARIBTRB15_525P = 0x1000,
    OPM_PROTECTION_STANDARD_ARIBTRB15_750P = 0x2000,
    OPM_PROTECTION_STANDARD_ARIBTRB15_1125I = 0x4000,
}}
ENUM!{enum OPM_IMAGE_ASPECT_RATIO_EN300294 {
    OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3 = 0,
    OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_CENTER = 1,
    OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_TOP = 2,
    OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_CENTER = 3,
    OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_TOP = 4,
    OPM_ASPECT_RATIO_EN300294_BOX_GT_16_BY_9_CENTER = 5,
    OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3_PROTECTED_CENTER = 6,
    OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_16_BY_9_ANAMORPHIC = 7,
    OPM_ASPECT_RATIO_FORCE_ULONG = 0x7fffffff,
}}
STRUCT!{#[repr(packed)] struct OPM_RANDOM_NUMBER {
    abRandomNumber: [BYTE; 16],
}}
STRUCT!{#[repr(packed)] struct OPM_OMAC {
    abOMAC: [BYTE; 16],
}}
STRUCT!{#[repr(packed)] struct OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    abEncryptedInitializationParameters: [BYTE; 256],
}}
STRUCT!{#[repr(packed)] struct OPM_GET_INFO_PARAMETERS {
    omac: OPM_OMAC,
    rnRandomNumber: OPM_RANDOM_NUMBER,
    guidInformation: GUID,
    ulSequenceNumber: ULONG,
    cbParametersSize: ULONG,
    abParameters: [BYTE; 4056],
}}
STRUCT!{#[repr(packed)] struct OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    guidInformation: GUID,
    ulSequenceNumber: ULONG,
    cbParametersSize: ULONG,
    abParameters: [BYTE; 4056],
}}
STRUCT!{#[repr(packed)] struct OPM_HDCP_KEY_SELECTION_VECTOR {
    abKeySelectionVector: [BYTE; 5],
}}
STRUCT!{#[repr(packed)] struct OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    ulStatusFlags: ULONG,
    ulHDCPFlags: ULONG,
    ksvB: OPM_HDCP_KEY_SELECTION_VECTOR,
    Reserved: [BYTE; 11],
    Reserved2: [BYTE; 16],
    Reserved3: [BYTE; 16],
}}
STRUCT!{#[repr(packed)] struct OPM_REQUESTED_INFORMATION {
    omac: OPM_OMAC,
    cbRequestedInformationSize: ULONG,
    abRequestedInformation: [BYTE; 4076],
}}
STRUCT!{#[repr(packed)] struct OPM_STANDARD_INFORMATION {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    ulStatusFlags: ULONG,
    ulInformation: ULONG,
    ulReserved: ULONG,
    ulReserved2: ULONG,
}}
STRUCT!{#[repr(packed)] struct OPM_ACTUAL_OUTPUT_FORMAT {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    ulStatusFlags: ULONG,
    ulDisplayWidth: ULONG,
    ulDisplayHeight: ULONG,
    dsfSampleInterleaveFormat: DXVA2_SampleFormat,
    d3dFormat: D3DFORMAT,
    ulFrequencyNumerator: ULONG,
    ulFrequencyDenominator: ULONG,
}}
STRUCT!{#[repr(packed)] struct OPM_ACP_AND_CGMSA_SIGNALING {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    ulStatusFlags: ULONG,
    ulAvailableTVProtectionStandards: ULONG,
    ulActiveTVProtectionStandard: ULONG,
    ulReserved: ULONG,
    ulAspectRatioValidMask1: ULONG,
    ulAspectRatioData1: ULONG,
    ulAspectRatioValidMask2: ULONG,
    ulAspectRatioData2: ULONG,
    ulAspectRatioValidMask3: ULONG,
    ulAspectRatioData3: ULONG,
    ulReserved2: [ULONG; 4],
    ulReserved3: [ULONG; 4],
}}
STRUCT!{#[repr(packed)] struct OPM_OUTPUT_ID_DATA {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    ulStatusFlags: ULONG,
    OutputId: UINT64,
}}
STRUCT!{#[repr(packed)] struct OPM_CONFIGURE_PARAMETERS {
    omac: OPM_OMAC,
    guidSetting: GUID,
    ulSequenceNumber: ULONG,
    cbParametersSize: ULONG,
    abParameters: [BYTE; 4056],
}}
STRUCT!{#[repr(packed)] struct OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    ulProtectionType: ULONG,
    ulProtectionLevel: ULONG,
    Reserved: ULONG,
    Reserved2: ULONG,
}}
STRUCT!{#[repr(packed)] struct OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    ulNewTVProtectionStandard: ULONG,
    ulAspectRatioChangeMask1: ULONG,
    ulAspectRatioData1: ULONG,
    ulAspectRatioChangeMask2: ULONG,
    ulAspectRatioData2: ULONG,
    ulAspectRatioChangeMask3: ULONG,
    ulAspectRatioData3: ULONG,
    ulReserved: [ULONG; 4],
    ulReserved2: [ULONG; 4],
    ulReserved3: ULONG,
}}
STRUCT!{#[repr(packed)] struct OPM_SET_HDCP_SRM_PARAMETERS {
    ulSRMVersion: ULONG,
}}
STRUCT!{#[repr(packed)] struct OPM_GET_CODEC_INFO_PARAMETERS {
    cbVerifier: DWORD,
    Verifier: [BYTE; 4052],
}}
STRUCT!{#[repr(packed)] struct OPM_GET_CODEC_INFO_INFORMATION {
    rnRandomNumber: OPM_RANDOM_NUMBER,
    Merit: DWORD,
}}
DEFINE_GUID!{IID_IOPMVideoOutput,
    0x0a15159d, 0x41c7, 0x4456, 0x93, 0xe1, 0x28, 0x4c, 0xd6, 0x1d, 0x4e, 0x8d}
RIDL!{#[uuid(0x0a15159d, 0x41c7, 0x4456, 0x93, 0xe1, 0x28, 0x4c, 0xd6, 0x1d, 0x4e, 0x8d)]
interface IOPMVideoOutput(IOPMVideoOutputVtbl): IUnknown(IUnknownVtbl) {
    fn StartInitialization(
        prnRandomNumber: *mut OPM_RANDOM_NUMBER,
        ppbCertificate: *mut *mut BYTE,
        pulCertificateLength: *mut ULONG,
    ) -> HRESULT,
    fn FinishInitialization(
        pParameters: *const OPM_ENCRYPTED_INITIALIZATION_PARAMETERS,
    ) -> HRESULT,
    fn GetInformation(
        pParameters: *const OPM_GET_INFO_PARAMETERS,
        pRequestedInformation: *mut OPM_REQUESTED_INFORMATION,
    ) -> HRESULT,
    fn COPPCompatibleGetInformation(
        pParameters: *const OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS,
        pRequestedInformation: *mut OPM_REQUESTED_INFORMATION,
    ) -> HRESULT,
    fn Configure(
        pParameters: *const OPM_CONFIGURE_PARAMETERS,
        ulAdditionalParametersSize: ULONG,
        pbAdditionalParameters: *const BYTE,
    ) -> HRESULT,
}}
#[inline]
pub fn GetBusType(ulBusTypeAndImplementation: ULONG) -> ULONG {
    ulBusTypeAndImplementation & OPM_BUS_TYPE_MASK
}
#[inline]
pub fn GetBusImplementation(ulBusTypeAndImplementation: ULONG) -> ULONG {
    (ulBusTypeAndImplementation & OPM_BUS_IMPLEMENTATION_MODIFIER_MASK) >> 16
}
#[inline]
pub fn IsNonStandardBusImplementation(ulBusTypeAndImplementation: ULONG) -> ULONG {
    ulBusTypeAndImplementation & OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD
}
extern "system" {
    pub fn OPMGetVideoOutputsFromHMONITOR(
        hMonitor: HMONITOR,
        vos: OPM_VIDEO_OUTPUT_SEMANTICS,
        pulNumVideoOutputs: *mut ULONG,
        pppOPMVideoOutputArray: *mut *mut *mut IOPMVideoOutput,
    ) -> HRESULT;
    pub fn OPMGetVideoOutputForTarget(
        pAdapterLuid: *mut LUID,
        VidPnTarget: ULONG,
        vos: OPM_VIDEO_OUTPUT_SEMANTICS,
        ppOPMVideoOutput: *mut *mut IOPMVideoOutput,
    ) -> HRESULT;
    pub fn OPMGetVideoOutputsFromIDirect3DDevice9Object(
        pDirect3DDevice9: *mut IDirect3DDevice9,
        vos: OPM_VIDEO_OUTPUT_SEMANTICS,
        pulNumVideoOutputs: *mut ULONG,
        pppOPMVideoOutputArray: *mut *mut *mut IOPMVideoOutput,
    ) -> HRESULT;
}
