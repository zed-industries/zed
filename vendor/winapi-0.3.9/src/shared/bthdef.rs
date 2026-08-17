// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::bthsdpdef::SDP_ERROR;
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, UCHAR, ULONG, USHORT};
use shared::ntdef::{CHAR, ULONGLONG};
pub const BTH_MAJORVERSION: DWORD = 2;
pub const BTH_MINORVERSION: DWORD = 1;
DEFINE_GUID!{GUID_BTHPORT_DEVICE_INTERFACE,
    0x850302a, 0xb344, 0x4fda, 0x9b, 0xe9, 0x90, 0x57, 0x6b, 0x8d, 0x46, 0xf0}
DEFINE_GUID!{GUID_BTH_RFCOMM_SERVICE_DEVICE_INTERFACE,
    0xb142fc3e, 0xfa4e, 0x460b, 0x8a, 0xbc, 0x07, 0x2b, 0x62, 0x8b, 0x3c, 0x70}
DEFINE_GUID!{GUID_BLUETOOTH_RADIO_IN_RANGE,
    0xea3b5b82, 0x26ee, 0x450e, 0xb0, 0xd8, 0xd2, 0x6f, 0xe3, 0x0a, 0x38, 0x69}
DEFINE_GUID!{GUID_BLUETOOTH_RADIO_OUT_OF_RANGE,
    0xe28867c9, 0xc2aa, 0x4ced, 0xb9, 0x69, 0x45, 0x70, 0x86, 0x60, 0x37, 0xc4}
DEFINE_GUID!{GUID_BLUETOOTH_L2CAP_EVENT,
    0x7eae4030, 0xb709, 0x4aa8, 0xac, 0x55, 0xe9, 0x53, 0x82, 0x9c, 0x9d, 0xaa}
DEFINE_GUID!{GUID_BLUETOOTH_HCI_EVENT,
    0xfc240062, 0x1541, 0x49be, 0xb4, 0x63, 0x84, 0xc4, 0xdc, 0xd7, 0xbf, 0x7f}
DEFINE_GUID!{GUID_BLUETOOTH_AUTHENTICATION_REQUEST,
    0x5DC9136D, 0x996C, 0x46DB, 0x84, 0xF5, 0x32, 0xC0, 0xA3, 0xF4, 0x73, 0x52}
DEFINE_GUID!{GUID_BLUETOOTH_KEYPRESS_EVENT,
    0xD668DFCD, 0x0F4E, 0x4EFC, 0xBF, 0xE0, 0x39, 0x2E, 0xEE, 0xC5, 0x10, 0x9C}
DEFINE_GUID!{GUID_BLUETOOTH_HCI_VENDOR_EVENT,
    0x547247e6, 0x45bb, 0x4c33, 0xaf, 0x8c, 0xc0, 0x0e, 0xfe, 0x15, 0xa7, 0x1d}
DEFINE_GUID!{Bluetooth_Base_UUID,
    0x00000000, 0x0000, 0x1000, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB}
pub const SDP_PROTOCOL_UUID16: USHORT = 0x0001;
pub const UDP_PROTOCOL_UUID16: USHORT = 0x0002;
pub const RFCOMM_PROTOCOL_UUID16: USHORT = 0x0003;
pub const TCP_PROTOCOL_UUID16: USHORT = 0x0004;
pub const TCSBIN_PROTOCOL_UUID16: USHORT = 0x0005;
pub const TCSAT_PROTOCOL_UUID16: USHORT = 0x0006;
pub const ATT_PROTOCOL_UUID16: USHORT = 0x0007;
pub const OBEX_PROTOCOL_UUID16: USHORT = 0x0008;
pub const IP_PROTOCOL_UUID16: USHORT = 0x0009;
pub const FTP_PROTOCOL_UUID16: USHORT = 0x000A;
pub const HTTP_PROTOCOL_UUID16: USHORT = 0x000C;
pub const WSP_PROTOCOL_UUID16: USHORT = 0x000E;
pub const BNEP_PROTOCOL_UUID16: USHORT = 0x000F;
pub const UPNP_PROTOCOL_UUID16: USHORT = 0x0010;
pub const HID_PROTOCOL_UUID16: USHORT = 0x0011;
pub const HCCC_PROTOCOL_UUID16: USHORT = 0x0012;
pub const HCDC_PROTOCOL_UUID16: USHORT = 0x0014;
pub const HCN_PROTOCOL_UUID16: USHORT = 0x0016;
pub const AVCTP_PROTOCOL_UUID16: USHORT = 0x0017;
pub const AVDTP_PROTOCOL_UUID16: USHORT = 0x0019;
pub const CMPT_PROTOCOL_UUID16: USHORT = 0x001B;
pub const UDI_C_PLANE_PROTOCOL_UUID16: USHORT = 0x001D;
pub const L2CAP_PROTOCOL_UUID16: USHORT = 0x0100;
DEFINE_BLUETOOTH_UUID128!{SDP_PROTOCOL_UUID, SDP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{UDP_PROTOCOL_UUID, UDP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{RFCOMM_PROTOCOL_UUID, RFCOMM_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{TCP_PROTOCOL_UUID, TCP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{TCSBIN_PROTOCOL_UUID, TCSBIN_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{TCSAT_PROTOCOL_UUID, TCSAT_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{ATT_PROTOCOL_UUID, ATT_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{OBEX_PROTOCOL_UUID, OBEX_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{IP_PROTOCOL_UUID, IP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{FTP_PROTOCOL_UUID, FTP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{HTTP_PROTOCOL_UUID, HTTP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{WSP_PROTOCOL_UUID, WSP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{BNEP_PROTOCOL_UUID, BNEP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{UPNP_PROTOCOL_UUID, UPNP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{HID_PROTOCOL_UUID, HID_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{HCCC_PROTOCOL_UUID, HCCC_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{HCDC_PROTOCOL_UUID, HCDC_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{HCN_PROTOCOL_UUID, HCN_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{AVCTP_PROTOCOL_UUID, AVCTP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{AVDTP_PROTOCOL_UUID, AVDTP_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{CMPT_PROTOCOL_UUID, CMPT_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{UDI_C_PLANE_PROTOCOL_UUID, UDI_C_PLANE_PROTOCOL_UUID16}
DEFINE_BLUETOOTH_UUID128!{L2CAP_PROTOCOL_UUID, L2CAP_PROTOCOL_UUID16}
pub const ServiceDiscoveryServerServiceClassID_UUID16: USHORT = 0x1000;
pub const BrowseGroupDescriptorServiceClassID_UUID16: USHORT = 0x1001;
pub const PublicBrowseGroupServiceClassID_UUID16: USHORT = 0x1002;
pub const SerialPortServiceClassID_UUID16: USHORT = 0x1101;
pub const LANAccessUsingPPPServiceClassID_UUID16: USHORT = 0x1102;
pub const DialupNetworkingServiceClassID_UUID16: USHORT = 0x1103;
pub const IrMCSyncServiceClassID_UUID16: USHORT = 0x1104;
pub const OBEXObjectPushServiceClassID_UUID16: USHORT = 0x1105;
pub const OBEXFileTransferServiceClassID_UUID16: USHORT = 0x1106;
pub const IrMcSyncCommandServiceClassID_UUID16: USHORT = 0x1107;
pub const HeadsetServiceClassID_UUID16: USHORT = 0x1108;
pub const CordlessTelephonyServiceClassID_UUID16: USHORT = 0x1109;
pub const AudioSourceServiceClassID_UUID16: USHORT = 0x110A;
pub const AudioSinkServiceClassID_UUID16: USHORT = 0x110B;
pub const AVRemoteControlTargetServiceClassID_UUID16: USHORT = 0x110C;
pub const AVRemoteControlServiceClassID_UUID16: USHORT = 0x110E;
pub const AVRemoteControlControllerServiceClass_UUID16: USHORT = 0x110F;
pub const IntercomServiceClassID_UUID16: USHORT = 0x1110;
pub const FaxServiceClassID_UUID16: USHORT = 0x1111;
pub const HeadsetAudioGatewayServiceClassID_UUID16: USHORT = 0x1112;
pub const WAPServiceClassID_UUID16: USHORT = 0x1113;
pub const WAPClientServiceClassID_UUID16: USHORT = 0x1114;
pub const PANUServiceClassID_UUID16: USHORT = 0x1115;
pub const NAPServiceClassID_UUID16: USHORT = 0x1116;
pub const GNServiceClassID_UUID16: USHORT = 0x1117;
pub const DirectPrintingServiceClassID_UUID16: USHORT = 0x1118;
pub const ReferencePrintingServiceClassID_UUID16: USHORT = 0x1119;
pub const ImagingResponderServiceClassID_UUID16: USHORT = 0x111B;
pub const ImagingAutomaticArchiveServiceClassID_UUID16: USHORT = 0x111C;
pub const ImagingReferenceObjectsServiceClassID_UUID16: USHORT = 0x111D;
pub const HandsfreeServiceClassID_UUID16: USHORT = 0x111E;
pub const HandsfreeAudioGatewayServiceClassID_UUID16: USHORT = 0x111F;
pub const DirectPrintingReferenceObjectsServiceClassID_UUID16: USHORT = 0x1120;
pub const ReflectsUIServiceClassID_UUID16: USHORT = 0x1121;
pub const PrintingStatusServiceClassID_UUID16: USHORT = 0x1123;
pub const HumanInterfaceDeviceServiceClassID_UUID16: USHORT = 0x1124;
pub const HCRPrintServiceClassID_UUID16: USHORT = 0x1126;
pub const HCRScanServiceClassID_UUID16: USHORT = 0x1127;
pub const CommonISDNAccessServiceClassID_UUID16: USHORT = 0x1128;
pub const VideoConferencingGWServiceClassID_UUID16: USHORT = 0x1129;
pub const UDIMTServiceClassID_UUID16: USHORT = 0x112A;
pub const UDITAServiceClassID_UUID16: USHORT = 0x112B;
pub const AudioVideoServiceClassID_UUID16: USHORT = 0x112C;
pub const SimAccessServiceClassID_UUID16: USHORT = 0x112D;
pub const PhonebookAccessPceServiceClassID_UUID16: USHORT = 0x112E;
pub const PhonebookAccessPseServiceClassID_UUID16: USHORT = 0x112F;
pub const HeadsetHSServiceClassID_UUID16: USHORT = 0x1131;
pub const MessageAccessServerServiceClassID_UUID16: USHORT = 0x1132;
pub const MessageNotificationServerServiceClassID_UUID16: USHORT = 0x1133;
pub const GNSSServerServiceClassID_UUID16: USHORT = 0x1136;
pub const ThreeDimensionalDisplayServiceClassID_UUID16: USHORT = 0x1137;
pub const ThreeDimensionalGlassesServiceClassID_UUID16: USHORT = 0x1138;
pub const MPSServiceClassID_UUID16: USHORT = 0x113B;
pub const CTNAccessServiceClassID_UUID16: USHORT = 0x113C;
pub const CTNNotificationServiceClassID_UUID16: USHORT = 0x113D;
pub const PnPInformationServiceClassID_UUID16: USHORT = 0x1200;
pub const GenericNetworkingServiceClassID_UUID16: USHORT = 0x1201;
pub const GenericFileTransferServiceClassID_UUID16: USHORT = 0x1202;
pub const GenericAudioServiceClassID_UUID16: USHORT = 0x1203;
pub const GenericTelephonyServiceClassID_UUID16: USHORT = 0x1204;
pub const UPnpServiceClassID_UUID16: USHORT = 0x1205;
pub const UPnpIpServiceClassID_UUID16: USHORT = 0x1206;
pub const ESdpUpnpIpPanServiceClassID_UUID16: USHORT = 0x1300;
pub const ESdpUpnpIpLapServiceClassID_UUID16: USHORT = 0x1301;
pub const ESdpUpnpL2capServiceClassID_UUID16: USHORT = 0x1302;
pub const VideoSourceServiceClassID_UUID16: USHORT = 0x1303;
pub const VideoSinkServiceClassID_UUID16: USHORT = 0x1304;
pub const HealthDeviceProfileSourceServiceClassID_UUID16: USHORT = 0x1401;
pub const HealthDeviceProfileSinkServiceClassID_UUID16: USHORT = 0x1402;
DEFINE_BLUETOOTH_UUID128!{ServiceDiscoveryServerServiceClassID_UUID,
    ServiceDiscoveryServerServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{BrowseGroupDescriptorServiceClassID_UUID,
    BrowseGroupDescriptorServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PublicBrowseGroupServiceClass_UUID,
    PublicBrowseGroupServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{SerialPortServiceClass_UUID,
    SerialPortServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{LANAccessUsingPPPServiceClass_UUID,
    LANAccessUsingPPPServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{DialupNetworkingServiceClass_UUID,
    DialupNetworkingServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{IrMCSyncServiceClass_UUID,
    IrMCSyncServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{OBEXObjectPushServiceClass_UUID,
    OBEXObjectPushServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{OBEXFileTransferServiceClass_UUID,
    OBEXFileTransferServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{IrMCSyncCommandServiceClass_UUID,
    IrMcSyncCommandServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HeadsetServiceClass_UUID,
    HeadsetServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{CordlessTelephonyServiceClass_UUID,
    CordlessTelephonyServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AudioSourceServiceClass_UUID,
    AudioSourceServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AudioSinkServiceClass_UUID,
    AudioSinkServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AVRemoteControlTargetServiceClass_UUID,
    AVRemoteControlTargetServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AVRemoteControlServiceClass_UUID,
    AVRemoteControlServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AVRemoteControlControllerServiceClass_UUID,
    AVRemoteControlControllerServiceClass_UUID16}
DEFINE_BLUETOOTH_UUID128!{IntercomServiceClass_UUID,
    IntercomServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{FaxServiceClass_UUID,
    FaxServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HeadsetAudioGatewayServiceClass_UUID,
    HeadsetAudioGatewayServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{WAPServiceClass_UUID,
    WAPServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{WAPClientServiceClass_UUID,
    WAPClientServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PANUServiceClass_UUID,
    PANUServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{NAPServiceClass_UUID,
    NAPServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GNServiceClass_UUID,
    GNServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{DirectPrintingServiceClass_UUID,
    DirectPrintingServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ReferencePrintingServiceClass_UUID,
    ReferencePrintingServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ImagingResponderServiceClass_UUID,
    ImagingResponderServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ImagingAutomaticArchiveServiceClass_UUID,
    ImagingAutomaticArchiveServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ImagingReferenceObjectsServiceClass_UUID,
    ImagingReferenceObjectsServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HandsfreeServiceClass_UUID,
    HandsfreeServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HandsfreeAudioGatewayServiceClass_UUID,
    HandsfreeAudioGatewayServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{DirectPrintingReferenceObjectsServiceClass_UUID,
    DirectPrintingReferenceObjectsServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ReflectedUIServiceClass_UUID,
    ReflectsUIServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PrintingStatusServiceClass_UUID,
    PrintingStatusServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HumanInterfaceDeviceServiceClass_UUID,
    HumanInterfaceDeviceServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HCRPrintServiceClass_UUID,
    HCRPrintServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HCRScanServiceClass_UUID,
    HCRScanServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{CommonISDNAccessServiceClass_UUID,
    CommonISDNAccessServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{VideoConferencingGWServiceClass_UUID,
    VideoConferencingGWServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{UDIMTServiceClass_UUID,
    UDIMTServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{UDITAServiceClass_UUID,
    UDITAServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{AudioVideoServiceClass_UUID,
    AudioVideoServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{SimAccessServiceClass_UUID,
    SimAccessServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PhonebookAccessPceServiceClass_UUID,
    PhonebookAccessPceServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PhonebookAccessPseServiceClass_UUID,
    PhonebookAccessPseServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HeadsetHSServiceClass_UUID,
    HeadsetHSServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{MessageAccessServerServiceClass_UUID,
    MessageAccessServerServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{MessageNotificationServerServiceClass_UUID,
    MessageNotificationServerServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GNSSServerServiceClass_UUID,
    GNSSServerServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ThreeDimensionalDisplayServiceClass_UUID,
    ThreeDimensionalDisplayServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ThreeDimensionalGlassesServiceClass_UUID,
    ThreeDimensionalGlassesServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{MPSServiceClass_UUID,
    MPSServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{CTNAccessServiceClass_UUID,
    CTNAccessServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{CTNNotificationServiceClass_UUID,
    CTNNotificationServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PnPInformationServiceClass_UUID,
    PnPInformationServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GenericNetworkingServiceClass_UUID,
    GenericNetworkingServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GenericFileTransferServiceClass_UUID,
    GenericFileTransferServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GenericAudioServiceClass_UUID,
    GenericAudioServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GenericTelephonyServiceClass_UUID,
    GenericTelephonyServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{UPnpServiceClass_UUID,
    UPnpServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{UPnpIpServiceClass_UUID,
    UPnpIpServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ESdpUpnpIpPanServiceClass_UUID,
    ESdpUpnpIpPanServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ESdpUpnpIpLapServiceClass_UUID,
    ESdpUpnpIpLapServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ESdpUpnpL2capServiceClass_UUID,
    ESdpUpnpL2capServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{VideoSourceServiceClass_UUID,
    VideoSourceServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{VideoSinkServiceClass_UUID,
    VideoSinkServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HealthDeviceProfileSourceServiceClass_UUID,
    HealthDeviceProfileSourceServiceClassID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HealthDeviceProfileSinkServiceClass_UUID,
    HealthDeviceProfileSinkServiceClassID_UUID16}
pub const AdvancedAudioDistributionProfileID_UUID16: USHORT = 0x110D;
pub const ImagingServiceProfileID_UUID16: USHORT = 0x111A;
pub const BasicPrintingProfileID_UUID16: USHORT = 0x1122;
pub const HardcopyCableReplacementProfileID_UUID16: USHORT = 0x1125;
pub const PhonebookAccessProfileID_UUID16: USHORT = 0x1130;
pub const MessageAccessProfileID_UUID16: USHORT = 0x1134;
pub const GNSSProfileID_UUID16: USHORT = 0x1135;
pub const ThreeDimensionalSynchronizationProfileID_UUID16: USHORT = 0x1139;
pub const MPSProfileID_UUID16: USHORT = 0x113A;
pub const CTNProfileID_UUID16: USHORT = 0x113E;
pub const VideoDistributionProfileID_UUID16: USHORT = 0x1305;
pub const HealthDeviceProfileID_UUID16: USHORT = 0x1400;
DEFINE_BLUETOOTH_UUID128!{AdvancedAudioDistributionProfile_UUID,
    AdvancedAudioDistributionProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ImagingServiceProfile_UUID,
    ImagingServiceProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{BasicPrintingProfile_UUID,
    BasicPrintingProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HardcopyCableReplacementProfile_UUID,
    HardcopyCableReplacementProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{PhonebookAccessProfile_UUID,
    PhonebookAccessProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{MessageAccessProfile_UUID,
    MessageAccessProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{GNSSProfile_UUID,
    GNSSProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{ThreeDimensionalSynchronizationProfile_UUID,
    ThreeDimensionalSynchronizationProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{MPSProfile_UUID,
    MPSProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{CTNProfile_UUID,
    CTNProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{VideoDistributionProfile_UUID,
    VideoDistributionProfileID_UUID16}
DEFINE_BLUETOOTH_UUID128!{HealthDeviceProfile_UUID,
    HealthDeviceProfileID_UUID16}
pub const VideoConferencingServiceClass_UUID: GUID = AVRemoteControlControllerServiceClass_UUID;
pub const VideoConferencingServiceClassID_UUID16: USHORT
    = AVRemoteControlControllerServiceClass_UUID16;
pub const HN_PROTOCOL_UUID: GUID = HCN_PROTOCOL_UUID;
pub const BasicPringingServiceClass_UUID: GUID = BasicPrintingProfile_UUID;
pub const CommonISDNAccessServiceClass_UUID16: USHORT = CommonISDNAccessServiceClassID_UUID16;
pub const VideoConferencingGWServiceClass_UUID16: USHORT
    = VideoConferencingGWServiceClassID_UUID16;
pub const UDIMTServiceClass_UUID16: USHORT = UDIMTServiceClassID_UUID16;
pub const UDITAServiceClass_UUID16: USHORT = UDITAServiceClassID_UUID16;
pub const AudioVideoServiceClass_UUID16: USHORT = AudioVideoServiceClassID_UUID16;
pub const CordlessServiceClassID_UUID16: USHORT = CordlessTelephonyServiceClassID_UUID16;
pub const AudioSinkSourceServiceClassID_UUID16: USHORT = AudioSinkServiceClassID_UUID16;
pub const AdvancedAudioDistributionServiceClassID_UUID16: USHORT
    = AdvancedAudioDistributionProfileID_UUID16;
pub const ImagingServiceClassID_UUID16: USHORT = ImagingServiceProfileID_UUID16;
pub const BasicPrintingServiceClassID_UUID16: USHORT = BasicPrintingProfileID_UUID16;
pub const HardcopyCableReplacementServiceClassID_UUID16: USHORT
    = HardcopyCableReplacementProfileID_UUID16;
pub const AdvancedAudioDistributionServiceClass_UUID: GUID = AdvancedAudioDistributionProfile_UUID;
pub const ImagingServiceClass_UUID: GUID = ImagingServiceProfile_UUID;
pub const BasicPrintingServiceClass_UUID: GUID = BasicPrintingProfile_UUID;
pub const HardcopyCableReplacementServiceClass_UUID: GUID = HardcopyCableReplacementProfile_UUID;
pub const VideoDistributionServiceClass_UUID: GUID = VideoDistributionProfile_UUID;
pub const BTH_MAX_NAME_SIZE: usize = 248;
pub const BTH_MAX_PIN_SIZE: usize = 16;
pub const BTH_LINK_KEY_LENGTH: usize = 16;
pub const BTH_MFG_ERICSSON: u16 = 0;
pub const BTH_MFG_NOKIA: u16 = 1;
pub const BTH_MFG_INTEL: u16 = 2;
pub const BTH_MFG_IBM: u16 = 3;
pub const BTH_MFG_TOSHIBA: u16 = 4;
pub const BTH_MFG_3COM: u16 = 5;
pub const BTH_MFG_MICROSOFT: u16 = 6;
pub const BTH_MFG_LUCENT: u16 = 7;
pub const BTH_MFG_MOTOROLA: u16 = 8;
pub const BTH_MFG_INFINEON: u16 = 9;
pub const BTH_MFG_CSR: u16 = 10;
pub const BTH_MFG_SILICONWAVE: u16 = 11;
pub const BTH_MFG_DIGIANSWER: u16 = 12;
pub const BTH_MFG_TI: u16 = 13;
pub const BTH_MFG_PARTHUS: u16 = 14;
pub const BTH_MFG_BROADCOM: u16 = 15;
pub const BTH_MFG_MITEL: u16 = 16;
pub const BTH_MFG_WIDCOMM: u16 = 17;
pub const BTH_MFG_ZEEVO: u16 = 18;
pub const BTH_MFG_ATMEL: u16 = 19;
pub const BTH_MFG_MITSIBUSHI: u16 = 20;
pub const BTH_MFG_RTX_TELECOM: u16 = 21;
pub const BTH_MFG_KC_TECHNOLOGY: u16 = 22;
pub const BTH_MFG_NEWLOGIC: u16 = 23;
pub const BTH_MFG_TRANSILICA: u16 = 24;
pub const BTH_MFG_ROHDE_SCHWARZ: u16 = 25;
pub const BTH_MFG_TTPCOM: u16 = 26;
pub const BTH_MFG_SIGNIA: u16 = 27;
pub const BTH_MFG_CONEXANT: u16 = 28;
pub const BTH_MFG_QUALCOMM: u16 = 29;
pub const BTH_MFG_INVENTEL: u16 = 30;
pub const BTH_MFG_AVM_BERLIN: u16 = 31;
pub const BTH_MFG_BANDSPEED: u16 = 32;
pub const BTH_MFG_MANSELLA: u16 = 33;
pub const BTH_MFG_NEC: u16 = 34;
pub const BTH_MFG_WAVEPLUS_TECHNOLOGY_CO: u16 = 35;
pub const BTH_MFG_ALCATEL: u16 = 36;
pub const BTH_MFG_PHILIPS_SEMICONDUCTOR: u16 = 37;
pub const BTH_MFG_C_TECHNOLOGIES: u16 = 38;
pub const BTH_MFG_OPEN_INTERFACE: u16 = 39;
pub const BTH_MFG_RF_MICRO_DEVICES: u16 = 40;
pub const BTH_MFG_HITACHI: u16 = 41;
pub const BTH_MFG_SYMBOL_TECHNOLOGIES: u16 = 42;
pub const BTH_MFG_TENOVIS: u16 = 43;
pub const BTH_MFG_MACRONIX_INTERNATIONAL: u16 = 44;
pub const BTH_MFG_APPLE: u16 = 76;
pub const BTH_MFG_NORDIC_SEMICONDUCTORS_ASA: u16 = 89;
pub const BTH_MFG_ARUBA_NETWORKS: u16 = 283;
pub const BTH_MFG_INTERNAL_USE: u16 = 65535;
pub type BTH_ADDR = ULONGLONG;
pub type PBTH_ADDR = *mut ULONGLONG;
pub type BTH_COD = ULONG;
pub type PBTH_COD = *mut ULONG;
pub type BTH_LAP = ULONG;
pub type PBTH_LAP = *mut ULONG;
pub const BTH_ADDR_NULL: BTH_ADDR = 0x0000000000000000;
pub const NAP_MASK: u64 = 0xFFFF00000000;
pub const SAP_MASK: u64 = 0x0000FFFFFFFF;
pub const NAP_BIT_OFFSET: u8 = 8 * 4;
pub const SAP_BIT_OFFSET: u8 = 0;
#[inline]
pub fn GET_NAP(addr: BTH_ADDR) -> u16 {
    ((addr & NAP_MASK) >> NAP_BIT_OFFSET) as u16
}
#[inline]
pub fn GET_SAP(addr: BTH_ADDR) -> u32 {
    ((addr & SAP_MASK) >> SAP_BIT_OFFSET) as u32
}
#[inline]
pub fn SET_NAP(nap: u16) -> BTH_ADDR {
    (nap as u64) << NAP_BIT_OFFSET
}
#[inline]
pub fn SET_SAP(sap: u32) -> BTH_ADDR {
    (sap as u64) << SAP_BIT_OFFSET
}
#[inline]
pub fn SET_NAP_SAP(nap: u16, sap: u32) -> BTH_ADDR {
    SET_NAP(nap) | SET_SAP(sap)
}
pub const COD_FORMAT_BIT_OFFSET: u8 = 0;
pub const COD_MINOR_BIT_OFFSET: u8 = 2;
pub const COD_MAJOR_BIT_OFFSET: u8 = 8 * 1;
pub const COD_SERVICE_BIT_OFFSET: u8 = 8 * 1 + 5;
pub const COD_FORMAT_MASK: u32 = 0x000003;
pub const COD_MINOR_MASK: u32 = 0x0000FC;
pub const COD_MAJOR_MASK: u32 = 0x001F00;
pub const COD_SERVICE_MASK: u32 = 0xFFE000;
#[inline]
pub fn GET_COD_FORMAT(cod: BTH_COD) -> u8 {
    ((cod & COD_FORMAT_MASK) >> COD_FORMAT_BIT_OFFSET) as u8
}
#[inline]
pub fn GET_COD_MINOR(cod: BTH_COD) -> u8 {
    ((cod & COD_MINOR_MASK) >> COD_MINOR_BIT_OFFSET) as u8
}
#[inline]
pub fn GET_COD_MAJOR(cod: BTH_COD) -> u8 {
    ((cod & COD_MAJOR_MASK) >> COD_MAJOR_BIT_OFFSET) as u8
}
#[inline]
pub fn GET_COD_SERVICE(cod: BTH_COD) -> u16 {
    ((cod & COD_SERVICE_MASK) >> COD_SERVICE_BIT_OFFSET) as u16
}
#[inline]
pub fn SET_COD_MINOR(cod: BTH_COD, minor: u8) -> BTH_COD {
    (cod & !COD_MINOR_MASK) | ((minor as u32) << COD_MINOR_BIT_OFFSET)
}
#[inline]
pub fn SET_COD_MAJOR(cod: BTH_COD, major: u8) -> BTH_COD {
    (cod & !COD_MAJOR_MASK) | ((major as u32) << COD_MAJOR_BIT_OFFSET)
}
#[inline]
pub fn SET_COD_SERVICE(cod: BTH_COD, service: u16) -> BTH_COD {
    (cod & !COD_SERVICE_MASK) | ((service as u32) << COD_SERVICE_BIT_OFFSET)
}
pub const COD_VERSION: u32 = 0x0;
pub const COD_SERVICE_LIMITED: u16 = 0x0001;
pub const COD_SERVICE_POSITIONING: u16 = 0x0008;
pub const COD_SERVICE_NETWORKING: u16 = 0x0010;
pub const COD_SERVICE_RENDERING: u16 = 0x0020;
pub const COD_SERVICE_CAPTURING: u16 = 0x0040;
pub const COD_SERVICE_OBJECT_XFER: u16 = 0x0080;
pub const COD_SERVICE_AUDIO: u16 = 0x0100;
pub const COD_SERVICE_TELEPHONY: u16 = 0x0200;
pub const COD_SERVICE_INFORMATION: u16 = 0x0400;
pub const COD_SERVICE_VALID_MASK: u16 = COD_SERVICE_LIMITED | COD_SERVICE_POSITIONING
    | COD_SERVICE_NETWORKING | COD_SERVICE_RENDERING | COD_SERVICE_CAPTURING
    | COD_SERVICE_OBJECT_XFER | COD_SERVICE_AUDIO | COD_SERVICE_TELEPHONY
    | COD_SERVICE_INFORMATION;
pub const COD_SERVICE_MAX_COUNT: usize = 9;
pub const COD_MAJOR_MISCELLANEOUS: u8 = 0x00;
pub const COD_MAJOR_COMPUTER: u8 = 0x01;
pub const COD_MAJOR_PHONE: u8 = 0x02;
pub const COD_MAJOR_LAN_ACCESS: u8 = 0x03;
pub const COD_MAJOR_AUDIO: u8 = 0x04;
pub const COD_MAJOR_PERIPHERAL: u8 = 0x05;
pub const COD_MAJOR_IMAGING: u8 = 0x06;
pub const COD_MAJOR_WEARABLE: u8 = 0x07;
pub const COD_MAJOR_TOY: u8 = 0x08;
pub const COD_MAJOR_HEALTH: u8 = 0x09;
pub const COD_MAJOR_UNCLASSIFIED: u8 = 0x1F;
pub const COD_COMPUTER_MINOR_UNCLASSIFIED: u8 = 0x00;
pub const COD_COMPUTER_MINOR_DESKTOP: u8 = 0x01;
pub const COD_COMPUTER_MINOR_SERVER: u8 = 0x02;
pub const COD_COMPUTER_MINOR_LAPTOP: u8 = 0x03;
pub const COD_COMPUTER_MINOR_HANDHELD: u8 = 0x04;
pub const COD_COMPUTER_MINOR_PALM: u8 = 0x05;
pub const COD_COMPUTER_MINOR_WEARABLE: u8 = 0x06;
pub const COD_PHONE_MINOR_UNCLASSIFIED: u8 = 0x00;
pub const COD_PHONE_MINOR_CELLULAR: u8 = 0x01;
pub const COD_PHONE_MINOR_CORDLESS: u8 = 0x02;
pub const COD_PHONE_MINOR_SMART: u8 = 0x03;
pub const COD_PHONE_MINOR_WIRED_MODEM: u8 = 0x04;
pub const COD_AUDIO_MINOR_UNCLASSIFIED: u8 = 0x00;
pub const COD_AUDIO_MINOR_HEADSET: u8 = 0x01;
pub const COD_AUDIO_MINOR_HANDS_FREE: u8 = 0x02;
pub const COD_AUDIO_MINOR_HEADSET_HANDS_FREE: u8 = 0x03;
pub const COD_AUDIO_MINOR_MICROPHONE: u8 = 0x04;
pub const COD_AUDIO_MINOR_LOUDSPEAKER: u8 = 0x05;
pub const COD_AUDIO_MINOR_HEADPHONES: u8 = 0x06;
pub const COD_AUDIO_MINOR_PORTABLE_AUDIO: u8 = 0x07;
pub const COD_AUDIO_MINOR_CAR_AUDIO: u8 = 0x08;
pub const COD_AUDIO_MINOR_SET_TOP_BOX: u8 = 0x09;
pub const COD_AUDIO_MINOR_HIFI_AUDIO: u8 = 0x0A;
pub const COD_AUDIO_MINOR_VCR: u8 = 0x0B;
pub const COD_AUDIO_MINOR_VIDEO_CAMERA: u8 = 0x0C;
pub const COD_AUDIO_MINOR_CAMCORDER: u8 = 0x0D;
pub const COD_AUDIO_MINOR_VIDEO_MONITOR: u8 = 0x0E;
pub const COD_AUDIO_MINOR_VIDEO_DISPLAY_LOUDSPEAKER: u8 = 0x0F;
pub const COD_AUDIO_MINOR_VIDEO_DISPLAY_CONFERENCING: u8 = 0x10;
pub const COD_AUDIO_MINOR_GAMING_TOY: u8 = 0x12;
pub const COD_PERIPHERAL_MINOR_KEYBOARD_MASK: u8 = 0x10;
pub const COD_PERIPHERAL_MINOR_POINTER_MASK: u8 = 0x20;
pub const COD_PERIPHERAL_MINOR_NO_CATEGORY: u8 = 0x00;
pub const COD_PERIPHERAL_MINOR_JOYSTICK: u8 = 0x01;
pub const COD_PERIPHERAL_MINOR_GAMEPAD: u8 = 0x02;
pub const COD_PERIPHERAL_MINOR_REMOTE_CONTROL: u8 = 0x03;
pub const COD_PERIPHERAL_MINOR_SENSING: u8 = 0x04;
pub const COD_IMAGING_MINOR_DISPLAY_MASK: u8 = 0x04;
pub const COD_IMAGING_MINOR_CAMERA_MASK: u8 = 0x08;
pub const COD_IMAGING_MINOR_SCANNER_MASK: u8 = 0x10;
pub const COD_IMAGING_MINOR_PRINTER_MASK: u8 = 0x20;
pub const COD_WEARABLE_MINOR_WRIST_WATCH: u8 = 0x01;
pub const COD_WEARABLE_MINOR_PAGER: u8 = 0x02;
pub const COD_WEARABLE_MINOR_JACKET: u8 = 0x03;
pub const COD_WEARABLE_MINOR_HELMET: u8 = 0x04;
pub const COD_WEARABLE_MINOR_GLASSES: u8 = 0x05;
pub const COD_TOY_MINOR_ROBOT: u8 = 0x01;
pub const COD_TOY_MINOR_VEHICLE: u8 = 0x02;
pub const COD_TOY_MINOR_DOLL_ACTION_FIGURE: u8 = 0x03;
pub const COD_TOY_MINOR_CONTROLLER: u8 = 0x04;
pub const COD_TOY_MINOR_GAME: u8 = 0x05;
pub const COD_HEALTH_MINOR_BLOOD_PRESSURE_MONITOR: u8 = 0x01;
pub const COD_HEALTH_MINOR_THERMOMETER: u8 = 0x02;
pub const COD_HEALTH_MINOR_WEIGHING_SCALE: u8 = 0x03;
pub const COD_HEALTH_MINOR_GLUCOSE_METER: u8 = 0x04;
pub const COD_HEALTH_MINOR_PULSE_OXIMETER: u8 = 0x05;
pub const COD_HEALTH_MINOR_HEART_PULSE_MONITOR: u8 = 0x06;
pub const COD_HEALTH_MINOR_HEALTH_DATA_DISPLAY: u8 = 0x07;
pub const COD_HEALTH_MINOR_STEP_COUNTER: u8 = 0x08;
pub const COD_LAN_ACCESS_BIT_OFFSET: u8 = 5;
pub const COD_LAN_MINOR_MASK: u32 = 0x00001C;
pub const COD_LAN_ACCESS_MASK: u32 = 0x0000E0;
#[inline]
pub fn GET_COD_LAN_MINOR(cod: BTH_COD) -> u8 {
    ((cod & COD_LAN_MINOR_MASK) >> COD_MINOR_BIT_OFFSET) as u8
}
#[inline]
pub fn GET_COD_LAN_ACCESS(cod: BTH_COD) -> u8 {
    ((cod & COD_LAN_ACCESS_MASK) >> COD_LAN_ACCESS_BIT_OFFSET) as u8
}
pub const COD_LAN_MINOR_UNCLASSIFIED: u8 = 0x00;
pub const COD_LAN_ACCESS_0_USED: u8 = 0x00;
pub const COD_LAN_ACCESS_17_USED: u8 = 0x01;
pub const COD_LAN_ACCESS_33_USED: u8 = 0x02;
pub const COD_LAN_ACCESS_50_USED: u8 = 0x03;
pub const COD_LAN_ACCESS_67_USED: u8 = 0x04;
pub const COD_LAN_ACCESS_83_USED: u8 = 0x05;
pub const COD_LAN_ACCESS_99_USED: u8 = 0x06;
pub const COD_LAN_ACCESS_FULL: u8 = 0x07;
pub const BTH_EIR_FLAGS_ID: u8 = 0x01;
pub const BTH_EIR_16_UUIDS_PARTIAL_ID: u8 = 0x02;
pub const BTH_EIR_16_UUIDS_COMPLETE_ID: u8 = 0x03;
pub const BTH_EIR_32_UUIDS_PARTIAL_ID: u8 = 0x04;
pub const BTH_EIR_32_UUIDS_COMPLETE_ID: u8 = 0x05;
pub const BTH_EIR_128_UUIDS_PARTIAL_ID: u8 = 0x06;
pub const BTH_EIR_128_UUIDS_COMPLETE_ID: u8 = 0x07;
pub const BTH_EIR_LOCAL_NAME_PARTIAL_ID: u8 = 0x08;
pub const BTH_EIR_LOCAL_NAME_COMPLETE_ID: u8 = 0x09;
pub const BTH_EIR_TX_POWER_LEVEL_ID: u8 = 0x0A;
pub const BTH_EIR_OOB_OPT_DATA_LEN_ID: u8 = 0x0B;
pub const BTH_EIR_OOB_BD_ADDR_ID: u8 = 0x0C;
pub const BTH_EIR_OOB_COD_ID: u8 = 0x0D;
pub const BTH_EIR_OOB_SP_HASH_ID: u8 = 0x0E;
pub const BTH_EIR_OOB_SP_RANDOMIZER_ID: u8 = 0x0F;
pub const BTH_EIR_MANUFACTURER_ID: u8 = 0xFF;
pub const BTH_EIR_SIZE: usize = 240;
// #define LAP_GIAC_INIT { 0x33, 0x8B, 0x9E }
// #define LAP_LIAC_INIT { 0x00, 0x8B, 0x9E }
pub const LAP_GIAC_VALUE: BTH_LAP = 0x009E8B33;
pub const LAP_LIAC_VALUE: BTH_LAP = 0x009E8B00;
pub const BTH_ADDR_IAC_FIRST: BTH_ADDR = 0x9E8B00;
pub const BTH_ADDR_IAC_LAST: BTH_ADDR = 0x9E8B3f;
pub const BTH_ADDR_LIAC: BTH_ADDR = 0x9E8B00;
pub const BTH_ADDR_GIAC: BTH_ADDR = 0x9E8B33;
pub type BTHSTATUS = UCHAR;
pub type PBTHSTATUS = *mut UCHAR;
#[inline]
pub fn BTH_ERROR(btStatus: BTHSTATUS) -> bool {
    btStatus != BTH_ERROR_SUCCESS
}
#[inline]
pub fn BTH_SUCCESS(btStatus: BTHSTATUS) -> bool {
    btStatus == BTH_ERROR_SUCCESS
}
pub const BTH_ERROR_SUCCESS: BTHSTATUS = 0x00;
pub const BTH_ERROR_UNKNOWN_HCI_COMMAND: BTHSTATUS = 0x01;
pub const BTH_ERROR_NO_CONNECTION: BTHSTATUS = 0x02;
pub const BTH_ERROR_HARDWARE_FAILURE: BTHSTATUS = 0x03;
pub const BTH_ERROR_PAGE_TIMEOUT: BTHSTATUS = 0x04;
pub const BTH_ERROR_AUTHENTICATION_FAILURE: BTHSTATUS = 0x05;
pub const BTH_ERROR_KEY_MISSING: BTHSTATUS = 0x06;
pub const BTH_ERROR_MEMORY_FULL: BTHSTATUS = 0x07;
pub const BTH_ERROR_CONNECTION_TIMEOUT: BTHSTATUS = 0x08;
pub const BTH_ERROR_MAX_NUMBER_OF_CONNECTIONS: BTHSTATUS = 0x09;
pub const BTH_ERROR_MAX_NUMBER_OF_SCO_CONNECTIONS: BTHSTATUS = 0x0a;
pub const BTH_ERROR_ACL_CONNECTION_ALREADY_EXISTS: BTHSTATUS = 0x0b;
pub const BTH_ERROR_COMMAND_DISALLOWED: BTHSTATUS = 0x0c;
pub const BTH_ERROR_HOST_REJECTED_LIMITED_RESOURCES: BTHSTATUS = 0x0d;
pub const BTH_ERROR_HOST_REJECTED_SECURITY_REASONS: BTHSTATUS = 0x0e;
pub const BTH_ERROR_HOST_REJECTED_PERSONAL_DEVICE: BTHSTATUS = 0x0f;
pub const BTH_ERROR_HOST_TIMEOUT: BTHSTATUS = 0x10;
pub const BTH_ERROR_UNSUPPORTED_FEATURE_OR_PARAMETER: BTHSTATUS = 0x11;
pub const BTH_ERROR_INVALID_HCI_PARAMETER: BTHSTATUS = 0x12;
pub const BTH_ERROR_REMOTE_USER_ENDED_CONNECTION: BTHSTATUS = 0x13;
pub const BTH_ERROR_REMOTE_LOW_RESOURCES: BTHSTATUS = 0x14;
pub const BTH_ERROR_REMOTE_POWERING_OFF: BTHSTATUS = 0x15;
pub const BTH_ERROR_LOCAL_HOST_TERMINATED_CONNECTION: BTHSTATUS = 0x16;
pub const BTH_ERROR_REPEATED_ATTEMPTS: BTHSTATUS = 0x17;
pub const BTH_ERROR_PAIRING_NOT_ALLOWED: BTHSTATUS = 0x18;
pub const BTH_ERROR_UKNOWN_LMP_PDU: BTHSTATUS = 0x19;
pub const BTH_ERROR_UNSUPPORTED_REMOTE_FEATURE: BTHSTATUS = 0x1a;
pub const BTH_ERROR_SCO_OFFSET_REJECTED: BTHSTATUS = 0x1b;
pub const BTH_ERROR_SCO_INTERVAL_REJECTED: BTHSTATUS = 0x1c;
pub const BTH_ERROR_SCO_AIRMODE_REJECTED: BTHSTATUS = 0x1d;
pub const BTH_ERROR_INVALID_LMP_PARAMETERS: BTHSTATUS = 0x1e;
pub const BTH_ERROR_UNSPECIFIED_ERROR: BTHSTATUS = 0x1f;
pub const BTH_ERROR_UNSUPPORTED_LMP_PARM_VALUE: BTHSTATUS = 0x20;
pub const BTH_ERROR_ROLE_CHANGE_NOT_ALLOWED: BTHSTATUS = 0x21;
pub const BTH_ERROR_LMP_RESPONSE_TIMEOUT: BTHSTATUS = 0x22;
pub const BTH_ERROR_LMP_TRANSACTION_COLLISION: BTHSTATUS = 0x23;
pub const BTH_ERROR_LMP_PDU_NOT_ALLOWED: BTHSTATUS = 0x24;
pub const BTH_ERROR_ENCRYPTION_MODE_NOT_ACCEPTABLE: BTHSTATUS = 0x25;
pub const BTH_ERROR_UNIT_KEY_NOT_USED: BTHSTATUS = 0x26;
pub const BTH_ERROR_QOS_IS_NOT_SUPPORTED: BTHSTATUS = 0x27;
pub const BTH_ERROR_INSTANT_PASSED: BTHSTATUS = 0x28;
pub const BTH_ERROR_PAIRING_WITH_UNIT_KEY_NOT_SUPPORTED: BTHSTATUS = 0x29;
pub const BTH_ERROR_DIFFERENT_TRANSACTION_COLLISION: BTHSTATUS = 0x2a;
pub const BTH_ERROR_QOS_UNACCEPTABLE_PARAMETER: BTHSTATUS = 0x2c;
pub const BTH_ERROR_QOS_REJECTED: BTHSTATUS = 0x2d;
pub const BTH_ERROR_CHANNEL_CLASSIFICATION_NOT_SUPPORTED: BTHSTATUS = 0x2e;
pub const BTH_ERROR_INSUFFICIENT_SECURITY: BTHSTATUS = 0x2f;
pub const BTH_ERROR_PARAMETER_OUT_OF_MANDATORY_RANGE: BTHSTATUS = 0x30;
pub const BTH_ERROR_ROLE_SWITCH_PENDING: BTHSTATUS = 0x32;
pub const BTH_ERROR_RESERVED_SLOT_VIOLATION: BTHSTATUS = 0x34;
pub const BTH_ERROR_ROLE_SWITCH_FAILED: BTHSTATUS = 0x35;
pub const BTH_ERROR_EXTENDED_INQUIRY_RESPONSE_TOO_LARGE: BTHSTATUS = 0x36;
pub const BTH_ERROR_SECURE_SIMPLE_PAIRING_NOT_SUPPORTED_BY_HOST: BTHSTATUS = 0x37;
pub const BTH_ERROR_HOST_BUSY_PAIRING: BTHSTATUS = 0x38;
pub const BTH_ERROR_CONNECTION_REJECTED_DUE_TO_NO_SUITABLE_CHANNEL_FOUND: BTHSTATUS = 0x39;
pub const BTH_ERROR_CONTROLLER_BUSY: BTHSTATUS = 0x3a;
pub const BTH_ERROR_UNACCEPTABLE_CONNECTION_INTERVAL: BTHSTATUS = 0x3b;
pub const BTH_ERROR_DIRECTED_ADVERTISING_TIMEOUT: BTHSTATUS = 0x3c;
pub const BTH_ERROR_CONNECTION_TERMINATED_DUE_TO_MIC_FAILURE: BTHSTATUS = 0x3d;
pub const BTH_ERROR_CONNECTION_FAILED_TO_BE_ESTABLISHED: BTHSTATUS = 0x3e;
pub const BTH_ERROR_MAC_CONNECTION_FAILED: BTHSTATUS = 0x3f;
pub const BTH_ERROR_UNSPECIFIED: BTHSTATUS = 0xFF;
pub const L2CAP_MIN_MTU: u16 = 48;
pub const L2CAP_MAX_MTU: u16 = 0xFFFF;
pub const L2CAP_DEFAULT_MTU: u16 = 672;
pub const MAX_L2CAP_PING_DATA_LENGTH: usize = 44;
pub const MAX_L2CAP_INFO_DATA_LENGTH: usize = 44;
pub const BDIF_ADDRESS: u32 = 0x00000001;
pub const BDIF_COD: u32 = 0x00000002;
pub const BDIF_NAME: u32 = 0x00000004;
pub const BDIF_PAIRED: u32 = 0x00000008;
pub const BDIF_PERSONAL: u32 = 0x00000010;
pub const BDIF_CONNECTED: u32 = 0x00000020;
pub const BDIF_SHORT_NAME: u32 = 0x00000040;
pub const BDIF_VISIBLE: u32 = 0x00000080;
pub const BDIF_SSP_SUPPORTED: u32 = 0x00000100;
pub const BDIF_SSP_PAIRED: u32 = 0x00000200;
pub const BDIF_SSP_MITM_PROTECTED: u32 = 0x00000400;
pub const BDIF_RSSI: u32 = 0x00001000;
pub const BDIF_EIR: u32 = 0x00002000;
pub const BDIF_BR: u32 = 0x00004000;
pub const BDIF_LE: u32 = 0x00008000;
pub const BDIF_LE_PAIRED: u32 = 0x00010000;
pub const BDIF_LE_PERSONAL: u32 = 0x00020000;
pub const BDIF_LE_MITM_PROTECTED: u32 = 0x00040000;
pub const BDIF_LE_PRIVACY_ENABLED: u32 = 0x00080000;
pub const BDIF_LE_RANDOM_ADDRESS_TYPE: u32 = 0x00100000;
pub const BDIF_LE_DISCOVERABLE: u32 = 0x00200000;
pub const BDIF_LE_NAME: u32 = 0x00400000;
pub const BDIF_LE_VISIBLE: u32 = 0x00800000;
pub const BDIF_LE_CONNECTED: u32 = 0x01000000;
pub const BDIF_LE_CONNECTABLE: u32 = 0x02000000;
pub const BDIF_CONNECTION_INBOUND: u32 = 0x04000000;
pub const BDIF_BR_SECURE_CONNECTION_PAIRED: u32 = 0x08000000;
pub const BDIF_LE_SECURE_CONNECTION_PAIRED: u32 = 0x10000000;
pub const BDIF_VALID_FLAGS: u32 =  BDIF_ADDRESS | BDIF_COD | BDIF_NAME | BDIF_PAIRED
    | BDIF_PERSONAL | BDIF_CONNECTED | BDIF_SHORT_NAME | BDIF_VISIBLE | BDIF_RSSI | BDIF_EIR
    | BDIF_SSP_PAIRED | BDIF_SSP_MITM_PROTECTED | BDIF_BR | BDIF_LE | BDIF_LE_PAIRED
    | BDIF_LE_PERSONAL | BDIF_LE_MITM_PROTECTED | BDIF_LE_PRIVACY_ENABLED
    | BDIF_LE_RANDOM_ADDRESS_TYPE | BDIF_LE_DISCOVERABLE | BDIF_LE_NAME | BDIF_LE_VISIBLE
    | BDIF_LE_CONNECTED | BDIF_LE_CONNECTABLE | BDIF_CONNECTION_INBOUND
    | BDIF_BR_SECURE_CONNECTION_PAIRED | BDIF_LE_SECURE_CONNECTION_PAIRED;
STRUCT!{struct BTH_DEVICE_INFO {
    flags: ULONG,
    address: BTH_ADDR,
    classOfDevice: BTH_COD,
    name: [CHAR; BTH_MAX_NAME_SIZE],
}}
pub type PBTH_DEVICE_INFO = *mut BTH_DEVICE_INFO;
STRUCT!{struct BTH_RADIO_IN_RANGE {
    deviceInfo: BTH_DEVICE_INFO,
    previousDeviceFlags: ULONG,
}}
pub type PBTH_RADIO_IN_RANGE = *mut BTH_RADIO_IN_RANGE;
STRUCT!{struct BTH_L2CAP_EVENT_INFO {
    bthAddress: BTH_ADDR,
    psm: USHORT,
    connected: UCHAR,
    initiated: UCHAR,
}}
pub type PBTH_L2CAP_EVENT_INFO = *mut BTH_L2CAP_EVENT_INFO;
pub const HCI_CONNECTION_TYPE_ACL: u8 = 1;
pub const HCI_CONNECTION_TYPE_SCO: u8 = 2;
pub const HCI_CONNECTION_TYPE_LE: u8 = 3;
pub const HCI_CONNNECTION_TYPE_ACL: u8 = HCI_CONNECTION_TYPE_ACL;
pub const HCI_CONNNECTION_TYPE_SCO: u8 = HCI_CONNECTION_TYPE_SCO;
STRUCT!{struct BTH_HCI_EVENT_INFO {
    bthAddress: BTH_ADDR,
    connectionType: UCHAR,
    connected: UCHAR,
}}
pub type PBTH_HCI_EVENT_INFO = *mut BTH_HCI_EVENT_INFO;
ENUM!{enum IO_CAPABILITY {
    IoCaps_DisplayOnly = 0x00,
    IoCaps_DisplayYesNo = 0x01,
    IoCaps_KeyboardOnly = 0x02,
    IoCaps_NoInputNoOutput = 0x03,
    IoCaps_Undefined = 0xff,
}}
ENUM!{enum AUTHENTICATION_REQUIREMENTS {
    MITMProtectionNotRequired = 0x00,
    MITMProtectionRequired = 0x01,
    MITMProtectionNotRequiredBonding = 0x02,
    MITMProtectionRequiredBonding = 0x03,
    MITMProtectionNotRequiredGeneralBonding = 0x04,
    MITMProtectionRequiredGeneralBonding = 0x05,
    MITMProtectionNotDefined = 0xff,
}}
#[inline]
pub fn IsMITMProtectionRequired(requirements: AUTHENTICATION_REQUIREMENTS) -> bool {
    MITMProtectionRequired == requirements || MITMProtectionRequiredBonding == requirements
        || MITMProtectionRequiredGeneralBonding == requirements
}
pub const BTH_MAX_SERVICE_NAME_SIZE: usize = 256;
pub const MAX_UUIDS_IN_QUERY: usize = 12;
pub const BTH_VID_DEFAULT_VALUE: u16 = 0xFFFF;
pub const SDP_ERROR_INVALID_SDP_VERSION: u16 = 0x0001;
pub const SDP_ERROR_INVALID_RECORD_HANDLE: u16 = 0x0002;
pub const SDP_ERROR_INVALID_REQUEST_SYNTAX: u16 = 0x0003;
pub const SDP_ERROR_INVALID_PDU_SIZE: u16 = 0x0004;
pub const SDP_ERROR_INVALID_CONTINUATION_STATE: u16 = 0x0005;
pub const SDP_ERROR_INSUFFICIENT_RESOURCES: u16 = 0x0006;
pub const SDP_ERROR_SUCCESS: SDP_ERROR = 0x0000;
pub const SDP_ERROR_SERVER_INVALID_RESPONSE: SDP_ERROR = 0x0100;
pub const SDP_ERROR_SERVER_RESPONSE_DID_NOT_PARSE: SDP_ERROR = 0x0200;
pub const SDP_ERROR_SERVER_BAD_FORMAT: SDP_ERROR = 0x0300;
pub const SDP_ERROR_COULD_NOT_SEND_CONTINUE: SDP_ERROR = 0x0400;
pub const SDP_ERROR_RESPONSE_TOO_LARGE: SDP_ERROR = 0x0500;
pub const SDP_ATTRIB_RECORD_HANDLE: u16 = 0x0000;
pub const SDP_ATTRIB_CLASS_ID_LIST: u16 = 0x0001;
pub const SDP_ATTRIB_RECORD_STATE: u16 = 0x0002;
pub const SDP_ATTRIB_SERVICE_ID: u16 = 0x0003;
pub const SDP_ATTRIB_PROTOCOL_DESCRIPTOR_LIST: u16 = 0x0004;
pub const SDP_ATTRIB_BROWSE_GROUP_LIST: u16 = 0x0005;
pub const SDP_ATTRIB_LANG_BASE_ATTRIB_ID_LIST: u16 = 0x0006;
pub const SDP_ATTRIB_INFO_TIME_TO_LIVE: u16 = 0x0007;
pub const SDP_ATTRIB_AVAILABILITY: u16 = 0x0008;
pub const SDP_ATTRIB_PROFILE_DESCRIPTOR_LIST: u16 = 0x0009;
pub const SDP_ATTRIB_DOCUMENTATION_URL: u16 = 0x000A;
pub const SDP_ATTRIB_CLIENT_EXECUTABLE_URL: u16 = 0x000B;
pub const SDP_ATTRIB_ICON_URL: u16 = 0x000C;
pub const SDP_ATTRIB_ADDITIONAL_PROTOCOL_DESCRIPTOR_LIST: u16 = 0x000D;
pub const SDP_ATTRIB_PROFILE_SPECIFIC: u16 = 0x0200;
pub const LANG_BASE_LANGUAGE_INDEX: u16 = 0x0000;
pub const LANG_BASE_ENCODING_INDEX: u16 = 0x0001;
pub const LANG_BASE_OFFSET_INDEX: u16 = 0x0002;
pub const LANG_DEFAULT_ID: u16 = 0x0100;
pub const LANGUAGE_EN_US: u16 = 0x656E;
pub const ENCODING_UTF_8: u16 = 0x006A;
pub const STRING_NAME_OFFSET: u16 = 0x0000;
pub const STRING_DESCRIPTION_OFFSET: u16 = 0x0001;
pub const STRING_PROVIDER_NAME_OFFSET: u16 = 0x0002;
pub const SDP_ATTRIB_SDP_VERSION_NUMBER_LIST: u16 = 0x0200;
pub const SDP_ATTRIB_SDP_DATABASE_STATE: u16 = 0x0201;
pub const SDP_ATTRIB_BROWSE_GROUP_ID: u16 = 0x0200;
pub const SDP_ATTRIB_CORDLESS_EXTERNAL_NETWORK: u16 = 0x0301;
pub const SDP_ATTRIB_FAX_CLASS_1_SUPPORT: u16 = 0x0302;
pub const SDP_ATTRIB_FAX_CLASS_2_0_SUPPORT: u16 = 0x0303;
pub const SDP_ATTRIB_FAX_CLASS_2_SUPPORT: u16 = 0x0304;
pub const SDP_ATTRIB_FAX_AUDIO_FEEDBACK_SUPPORT: u16 = 0x0305;
pub const SDP_ATTRIB_HEADSET_REMOTE_AUDIO_VOLUME_CONTROL: u16 = 0x0302;
pub const SDP_ATTRIB_LAN_LPSUBNET: u16 = 0x0200;
pub const SDP_ATTRIB_OBJECT_PUSH_SUPPORTED_FORMATS_LIST: u16 = 0x0303;
pub const SDP_ATTRIB_SYNCH_SUPPORTED_DATA_STORES_LIST: u16 = 0x0301;
pub const SDP_ATTRIB_SERVICE_VERSION: u16 = 0x0300;
pub const SDP_ATTRIB_PAN_NETWORK_ADDRESS: u16 = 0x0306;
pub const SDP_ATTRIB_PAN_WAP_GATEWAY: u16 = 0x0307;
pub const SDP_ATTRIB_PAN_HOME_PAGE_URL: u16 = 0x0308;
pub const SDP_ATTRIB_PAN_WAP_STACK_TYPE: u16 = 0x0309;
pub const SDP_ATTRIB_PAN_SECURITY_DESCRIPTION: u16 = 0x030A;
pub const SDP_ATTRIB_PAN_NET_ACCESS_TYPE: u16 = 0x030B;
pub const SDP_ATTRIB_PAN_MAX_NET_ACCESS_RATE: u16 = 0x030C;
pub const SDP_ATTRIB_IMAGING_SUPPORTED_CAPABILITIES: u16 = 0x0310;
pub const SDP_ATTRIB_IMAGING_SUPPORTED_FEATURES: u16 = 0x0311;
pub const SDP_ATTRIB_IMAGING_SUPPORTED_FUNCTIONS: u16 = 0x0312;
pub const SDP_ATTRIB_IMAGING_TOTAL_DATA_CAPACITY: u16 = 0x0313;
pub const SDP_ATTRIB_DI_SPECIFICATION_ID: u16 = 0x0200;
pub const SDP_ATTRIB_DI_VENDOR_ID: u16 = 0x0201;
pub const SDP_ATTRIB_DI_PRODUCT_ID: u16 = 0x0202;
pub const SDP_ATTRIB_DI_VERSION: u16 = 0x0203;
pub const SDP_ATTRIB_DI_PRIMARY_RECORD: u16 = 0x0204;
pub const SDP_ATTRIB_DI_VENDOR_ID_SOURCE: u16 = 0x0205;
pub const SDP_ATTRIB_HID_DEVICE_RELEASE_NUMBER: u16 = 0x0200;
pub const SDP_ATTRIB_HID_PARSER_VERSION: u16 = 0x0201;
pub const SDP_ATTRIB_HID_DEVICE_SUBCLASS: u16 = 0x0202;
pub const SDP_ATTRIB_HID_COUNTRY_CODE: u16 = 0x0203;
pub const SDP_ATTRIB_HID_VIRTUAL_CABLE: u16 = 0x0204;
pub const SDP_ATTRIB_HID_RECONNECT_INITIATE: u16 = 0x0205;
pub const SDP_ATTRIB_HID_DESCRIPTOR_LIST: u16 = 0x0206;
pub const SDP_ATTRIB_HID_LANG_ID_BASE_LIST: u16 = 0x0207;
pub const SDP_ATTRIB_HID_SDP_DISABLE: u16 = 0x0208;
pub const SDP_ATTRIB_HID_BATTERY_POWER: u16 = 0x0209;
pub const SDP_ATTRIB_HID_REMOTE_WAKE: u16 = 0x020A;
pub const SDP_ATTRIB_HID_PROFILE_VERSION: u16 = 0x020B;
pub const SDP_ATTRIB_HID_SUPERVISION_TIMEOUT: u16 = 0x020C;
pub const SDP_ATTRIB_HID_NORMALLY_CONNECTABLE: u16 = 0x020D;
pub const SDP_ATTRIB_HID_BOOT_DEVICE: u16 = 0x020E;
pub const SDP_ATTRIB_HID_SSR_HOST_MAX_LATENCY: u16 = 0x020F;
pub const SDP_ATTRIB_HID_SSR_HOST_MIN_TIMEOUT: u16 = 0x0210;
pub const CORDLESS_EXTERNAL_NETWORK_PSTN: u8 = 0x01;
pub const CORDLESS_EXTERNAL_NETWORK_ISDN: u8 = 0x02;
pub const CORDLESS_EXTERNAL_NETWORK_GSM: u8 = 0x03;
pub const CORDLESS_EXTERNAL_NETWORK_CDMA: u8 = 0x04;
pub const CORDLESS_EXTERNAL_NETWORK_ANALOG_CELLULAR: u8 = 0x05;
pub const CORDLESS_EXTERNAL_NETWORK_PACKET_SWITCHED: u8 = 0x06;
pub const CORDLESS_EXTERNAL_NETWORK_OTHER: u8 = 0x07;
pub const OBJECT_PUSH_FORMAT_VCARD_2_1: u8 = 0x01;
pub const OBJECT_PUSH_FORMAT_VCARD_3_0: u8 = 0x02;
pub const OBJECT_PUSH_FORMAT_VCAL_1_0: u8 = 0x03;
pub const OBJECT_PUSH_FORMAT_ICAL_2_0: u8 = 0x04;
pub const OBJECT_PUSH_FORMAT_VNOTE: u8 = 0x05;
pub const OBJECT_PUSH_FORMAT_VMESSAGE: u8 = 0x06;
pub const OBJECT_PUSH_FORMAT_ANY: u8 = 0xFF;
pub const SYNCH_DATA_STORE_PHONEBOOK: u8 = 0x01;
pub const SYNCH_DATA_STORE_CALENDAR: u8 = 0x03;
pub const SYNCH_DATA_STORE_NOTES: u8 = 0x05;
pub const SYNCH_DATA_STORE_MESSAGES: u8 = 0x06;
pub const DI_VENDOR_ID_SOURCE_BLUETOOTH_SIG: u16 = 0x0001;
pub const DI_VENDOR_ID_SOURCE_USB_IF: u16 = 0x0002;
pub const PSM_SDP: u16 = 0x0001;
pub const PSM_RFCOMM: u16 = 0x0003;
pub const PSM_TCS_BIN: u16 = 0x0005;
pub const PSM_TCS_BIN_CORDLESS: u16 = 0x0007;
pub const PSM_BNEP: u16 = 0x000F;
pub const PSM_HID_CONTROL: u16 = 0x0011;
pub const PSM_HID_INTERRUPT: u16 = 0x0013;
pub const PSM_UPNP: u16 = 0x0015;
pub const PSM_AVCTP: u16 = 0x0017;
pub const PSM_AVDTP: u16 = 0x0019;
pub const PSM_AVCTP_BROWSE: u16 = 0x001B;
pub const PSM_UDI_C_PLANE: u16 = 0x001D;
pub const PSM_ATT: u16 = 0x001F;
pub const PSM_3DSP: u16 = 0x0021;
pub const PSM_LE_IPSP: u16 = 0x0023;
pub const STR_ADDR_FMTA: &'static str = "(%02x:%02x:%02x:%02x:%02x:%02x)\0";
// #define STR_ADDR_FMTW L"(%02x:%02x:%02x:%02x:%02x:%02x)"
pub const STR_ADDR_SHORT_FMTA: &'static str = "%04x%08x\0";
// #define STR_ADDR_SHORT_FMTW L"%04x%08x"
pub const STR_USBHCI_CLASS_HARDWAREIDA: &'static str = "USB\\Class_E0&SubClass_01&Prot_01\0";
// #define STR_USBHCI_CLASS_HARDWAREIDW L"USB\\Class_E0&SubClass_01&Prot_01"
#[inline]
pub fn GET_BITS(field: u64, offset: u8, mask: u64) -> u64 {
    (field >> offset) & mask
}
#[inline]
pub fn GET_BIT(field: u64, offset: u8) -> u64 {
    GET_BITS(field, offset, 1)
}
#[inline]
pub fn LMP_3_SLOT_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 0)
}
#[inline]
pub fn LMP_5_SLOT_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 1)
}
#[inline]
pub fn LMP_ENCRYPTION(x: u64) -> u64 {
    GET_BIT(x, 2)
}
#[inline]
pub fn LMP_SLOT_OFFSET(x: u64) -> u64 {
    GET_BIT(x, 3)
}
#[inline]
pub fn LMP_TIMING_ACCURACY(x: u64) -> u64 {
    GET_BIT(x, 4)
}
#[inline]
pub fn LMP_SWITCH(x: u64) -> u64 {
    GET_BIT(x, 5)
}
#[inline]
pub fn LMP_HOLD_MODE(x: u64) -> u64 {
    GET_BIT(x, 6)
}
#[inline]
pub fn LMP_SNIFF_MODE(x: u64) -> u64 {
    GET_BIT(x, 7)
}
#[inline]
pub fn LMP_PARK_MODE(x: u64) -> u64 {
    GET_BIT(x, 8)
}
#[inline]
pub fn LMP_RSSI(x: u64) -> u64 {
    GET_BIT(x, 9)
}
#[inline]
pub fn LMP_CHANNEL_QUALITY_DRIVEN_MODE(x: u64) -> u64 {
    GET_BIT(x, 10)
}
#[inline]
pub fn LMP_SCO_LINK(x: u64) -> u64 {
    GET_BIT(x, 11)
}
#[inline]
pub fn LMP_HV2_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 12)
}
#[inline]
pub fn LMP_HV3_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 13)
}
#[inline]
pub fn LMP_MU_LAW_LOG(x: u64) -> u64 {
    GET_BIT(x, 14)
}
#[inline]
pub fn LMP_A_LAW_LOG(x: u64) -> u64 {
    GET_BIT(x, 15)
}
#[inline]
pub fn LMP_CVSD(x: u64) -> u64 {
    GET_BIT(x, 16)
}
#[inline]
pub fn LMP_PAGING_SCHEME(x: u64) -> u64 {
    GET_BIT(x, 17)
}
#[inline]
pub fn LMP_POWER_CONTROL(x: u64) -> u64 {
    GET_BIT(x, 18)
}
#[inline]
pub fn LMP_TRANSPARENT_SCO_DATA(x: u64) -> u64 {
    GET_BIT(x, 19)
}
#[inline]
pub fn LMP_FLOW_CONTROL_LAG(x: u64) -> u64 {
    GET_BITS(x, 20, 0x3)
}
#[inline]
pub fn LMP_BROADCAST_ENCRYPTION(x: u64) -> u64 {
    GET_BIT(x, 23)
}
#[inline]
pub fn LMP_ENHANCED_DATA_RATE_ACL_2MBPS_MODE(x: u64) -> u64 {
    GET_BIT(x, 25)
}
#[inline]
pub fn LMP_ENHANCED_DATA_RATE_ACL_3MBPS_MODE(x: u64) -> u64 {
    GET_BIT(x, 26)
}
#[inline]
pub fn LMP_ENHANCED_INQUIRY_SCAN(x: u64) -> u64 {
    GET_BIT(x, 27)
}
#[inline]
pub fn LMP_INTERLACED_INQUIRY_SCAN(x: u64) -> u64 {
    GET_BIT(x, 28)
}
#[inline]
pub fn LMP_INTERLACED_PAGE_SCAN(x: u64) -> u64 {
    GET_BIT(x, 29)
}
#[inline]
pub fn LMP_RSSI_WITH_INQUIRY_RESULTS(x: u64) -> u64 {
    GET_BIT(x, 30)
}
#[inline]
pub fn LMP_ESCO_LINK(x: u64) -> u64 {
    GET_BIT(x, 31)
}
#[inline]
pub fn LMP_EV4_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 32)
}
#[inline]
pub fn LMP_EV5_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 33)
}
#[inline]
pub fn LMP_AFH_CAPABLE_SLAVE(x: u64) -> u64 {
    GET_BIT(x, 35)
}
#[inline]
pub fn LMP_AFH_CLASSIFICATION_SLAVE(x: u64) -> u64 {
    GET_BIT(x, 36)
}
#[inline]
pub fn LMP_BR_EDR_NOT_SUPPORTED(x: u64) -> u64 {
    GET_BIT(x, 37)
}
#[inline]
pub fn LMP_LE_SUPPORTED(x: u64) -> u64 {
    GET_BIT(x, 38)
}
#[inline]
pub fn LMP_3SLOT_EDR_ACL_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 39)
}
#[inline]
pub fn LMP_5SLOT_EDR_ACL_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 40)
}
#[inline]
pub fn LMP_SNIFF_SUBRATING(x: u64) -> u64 {
    GET_BIT(x, 41)
}
#[inline]
pub fn LMP_PAUSE_ENCRYPTION(x: u64) -> u64 {
    GET_BIT(x, 42)
}
#[inline]
pub fn LMP_AFH_CAPABLE_MASTER(x: u64) -> u64 {
    GET_BIT(x, 43)
}
#[inline]
pub fn LMP_AFH_CLASSIFICATION_MASTER(x: u64) -> u64 {
    GET_BIT(x, 44)
}
#[inline]
pub fn LMP_EDR_ESCO_2MBPS_MODE(x: u64) -> u64 {
    GET_BIT(x, 45)
}
#[inline]
pub fn LMP_EDR_ESCO_3MBPS_MODE(x: u64) -> u64 {
    GET_BIT(x, 46)
}
#[inline]
pub fn LMP_3SLOT_EDR_ESCO_PACKETS(x: u64) -> u64 {
    GET_BIT(x, 47)
}
#[inline]
pub fn LMP_EXTENDED_INQUIRY_RESPONSE(x: u64) -> u64 {
    GET_BIT(x, 48)
}
#[inline]
pub fn LMP_SIMULT_LE_BR_TO_SAME_DEV(x: u64) -> u64 {
    GET_BIT(x, 49)
}
#[inline]
pub fn LMP_SECURE_SIMPLE_PAIRING(x: u64) -> u64 {
    GET_BIT(x, 51)
}
#[inline]
pub fn LMP_ENCAPSULATED_PDU(x: u64) -> u64 {
    GET_BIT(x, 52)
}
#[inline]
pub fn LMP_ERRONEOUS_DATA_REPORTING(x: u64) -> u64 {
    GET_BIT(x, 53)
}
#[inline]
pub fn LMP_NON_FLUSHABLE_PACKET_BOUNDARY_FLAG(x: u64) -> u64 {
    GET_BIT(x, 54)
}
#[inline]
pub fn LMP_LINK_SUPERVISION_TIMEOUT_CHANGED_EVENT(x: u64) -> u64 {
    GET_BIT(x, 56)
}
#[inline]
pub fn LMP_INQUIRY_RESPONSE_TX_POWER_LEVEL(x: u64) -> u64 {
    GET_BIT(x, 57)
}
#[inline]
pub fn LMP_EXTENDED_FEATURES(x: u64) -> u64 {
    GET_BIT(x, 63)
}
