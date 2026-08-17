// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! USB Spec Definitions.
use shared::basetsd::ULONG64;
use shared::guiddef::GUID;
use shared::minwindef::{UCHAR, ULONG, USHORT};
use um::winnt::WCHAR;
ENUM!{enum USB_DEVICE_SPEED {
    UsbLowSpeed = 0,
    UsbFullSpeed,
    UsbHighSpeed,
    UsbSuperSpeed,
}}
ENUM!{enum USB_DEVICE_TYPE {
    Usb11Device = 0,
    Usb20Device,
}}
STRUCT!{#[repr(packed)] struct BM_REQUEST_TYPE {
    B: UCHAR,
}}
BITFIELD!{BM_REQUEST_TYPE B: UCHAR [
    Recipient set_Recipient[0..2],
    Reserved set_Reserved[2..5],
    Type set_Type[5..7],
    Dir set_Dir[7..8],
]}
pub type PBM_REQUEST_TYPE = *mut BM_REQUEST_TYPE;
STRUCT!{#[repr(packed)] struct USB_DEFAULT_PIPE_SETUP_PACKET_wValue_s {
    LowByte: UCHAR,
    HiByte: UCHAR,
}}
UNION!{#[repr(packed)] union USB_DEFAULT_PIPE_SETUP_PACKET_wValue {
    [u16; 1],
    s s_mut: USB_DEFAULT_PIPE_SETUP_PACKET_wValue_s,
    W W_mut: USHORT,
}}
STRUCT!{#[repr(packed)] struct USB_DEFAULT_PIPE_SETUP_PACKET_wIndex_s {
    LowByte: UCHAR,
    HiByte: UCHAR,
}}
UNION!{#[repr(packed)] union USB_DEFAULT_PIPE_SETUP_PACKET_wIndex {
    [u16; 1],
    s s_mut: USB_DEFAULT_PIPE_SETUP_PACKET_wIndex_s,
    W W_mut: USHORT,
}}
STRUCT!{#[repr(packed)] struct USB_DEFAULT_PIPE_SETUP_PACKET {
    bmRequestType: BM_REQUEST_TYPE,
    bRequest: UCHAR,
    wValue: USB_DEFAULT_PIPE_SETUP_PACKET_wValue,
    wIndex: USB_DEFAULT_PIPE_SETUP_PACKET_wIndex,
    wLength: USHORT,
}}
pub type PUSB_DEFAULT_PIPE_SETUP_PACKET = *mut USB_DEFAULT_PIPE_SETUP_PACKET;
pub const BMREQUEST_HOST_TO_DEVICE: UCHAR = 0;
pub const BMREQUEST_DEVICE_TO_HOST: UCHAR = 1;
pub const BMREQUEST_STANDARD: UCHAR = 0;
pub const BMREQUEST_CLASS: UCHAR = 1;
pub const BMREQUEST_VENDOR: UCHAR = 2;
pub const BMREQUEST_TO_DEVICE: UCHAR = 0;
pub const BMREQUEST_TO_INTERFACE: UCHAR = 1;
pub const BMREQUEST_TO_ENDPOINT: UCHAR = 2;
pub const BMREQUEST_TO_OTHER: UCHAR = 3;
#[inline]
pub fn USB_DESCRIPTOR_MAKE_TYPE_AND_INDEX(d: UCHAR, i: UCHAR) -> USHORT {
    (d as USHORT) << 8 | (i as USHORT)
}
pub const USB_REQUEST_GET_STATUS: UCHAR = 0x00;
pub const USB_REQUEST_CLEAR_FEATURE: UCHAR = 0x01;
pub const USB_REQUEST_SET_FEATURE: UCHAR = 0x03;
pub const USB_REQUEST_SET_ADDRESS: UCHAR = 0x05;
pub const USB_REQUEST_GET_DESCRIPTOR: UCHAR = 0x06;
pub const USB_REQUEST_SET_DESCRIPTOR: UCHAR = 0x07;
pub const USB_REQUEST_GET_CONFIGURATION: UCHAR = 0x08;
pub const USB_REQUEST_SET_CONFIGURATION: UCHAR = 0x09;
pub const USB_REQUEST_GET_INTERFACE: UCHAR = 0x0A;
pub const USB_REQUEST_SET_INTERFACE: UCHAR = 0x0B;
pub const USB_REQUEST_SYNC_FRAME: UCHAR = 0x0C;
pub const USB_REQUEST_SET_SEL: UCHAR = 0x30;
pub const USB_REQUEST_ISOCH_DELAY: UCHAR = 0x31;
pub const USB_DEVICE_DESCRIPTOR_TYPE: UCHAR = 0x01;
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: UCHAR = 0x02;
pub const USB_STRING_DESCRIPTOR_TYPE: UCHAR = 0x03;
pub const USB_INTERFACE_DESCRIPTOR_TYPE: UCHAR = 0x04;
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: UCHAR = 0x05;
pub const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: UCHAR = 0x06;
pub const USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE: UCHAR = 0x07;
pub const USB_INTERFACE_POWER_DESCRIPTOR_TYPE: UCHAR = 0x08;
pub const USB_OTG_DESCRIPTOR_TYPE: UCHAR = 0x09;
pub const USB_DEBUG_DESCRIPTOR_TYPE: UCHAR = 0x0A;
pub const USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE: UCHAR = 0x0B;
pub const USB_BOS_DESCRIPTOR_TYPE: UCHAR = 0x0F;
pub const USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE: UCHAR = 0x10;
pub const USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: UCHAR = 0x30;
pub const USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: UCHAR = 0x31;
pub const USB_RESERVED_DESCRIPTOR_TYPE: UCHAR = 0x06;
pub const USB_CONFIG_POWER_DESCRIPTOR_TYPE: UCHAR = 0x07;
pub const USB_FEATURE_ENDPOINT_STALL: UCHAR = 0x00;
pub const USB_FEATURE_REMOTE_WAKEUP: UCHAR = 0x01;
pub const USB_FEATURE_TEST_MODE: UCHAR = 0x02;
pub const USB_FEATURE_FUNCTION_SUSPEND: UCHAR = 0x00;
pub const USB_FEATURE_U1_ENABLE: UCHAR = 0x30;
pub const USB_FEATURE_U2_ENABLE: UCHAR = 0x31;
pub const USB_FEATURE_LTM_ENABLE: UCHAR = 0x32;
pub const USB_FEATURE_LDM_ENABLE: UCHAR = 0x35;
pub const USB_FEATURE_BATTERY_WAKE_MASK: UCHAR = 0x28;
pub const USB_FEATURE_OS_IS_PD_AWARE: UCHAR = 0x29;
pub const USB_FEATURE_POLICY_MODE: UCHAR = 0x2A;
pub const USB_FEATURE_CHARGING_POLICY: UCHAR = 0x36;
pub const USB_CHARGING_POLICY_DEFAULT: UCHAR = 0x00;
pub const USB_CHARGING_POLICY_ICCHPF: UCHAR = 0x01;
pub const USB_CHARGING_POLICY_ICCLPF: UCHAR = 0x02;
pub const USB_CHARGING_POLICY_NO_POWER: UCHAR = 0x03;
pub const USB_STATUS_PORT_STATUS: UCHAR = 0x00;
pub const USB_STATUS_PD_STATUS: UCHAR = 0x01;
pub const USB_STATUS_EXT_PORT_STATUS: UCHAR = 0x02;
pub const USB_GETSTATUS_SELF_POWERED: UCHAR = 0x01;
pub const USB_GETSTATUS_REMOTE_WAKEUP_ENABLED: UCHAR = 0x02;
pub const USB_GETSTATUS_U1_ENABLE: UCHAR = 0x04;
pub const USB_GETSTATUS_U2_ENABLE: UCHAR = 0x08;
pub const USB_GETSTATUS_LTM_ENABLE: UCHAR = 0x10;
STRUCT!{#[repr(packed)] struct USB_DEVICE_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_DEVICE_STATUS AsUshort16: USHORT [
    SelfPowered set_SelfPowered[0..1],
    RemoteWakeup set_RemoteWakeup[1..2],
    U1Enable set_U1Enable[2..3],
    U2Enable set_U2Enable[3..4],
    LtmEnable set_LtmEnable[4..5],
    Reserved set_Reserved[5..16],
]}
pub type PUSB_DEVICE_STATUS = *mut USB_DEVICE_STATUS;
STRUCT!{#[repr(packed)] struct USB_INTERFACE_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_INTERFACE_STATUS AsUshort16: USHORT [
    RemoteWakeupCapable set_RemoteWakeupCapable[0..1],
    RemoteWakeupEnabled set_RemoteWakeupEnabled[1..2],
    Reserved set_Reserved[2..16],
]}
pub type PUSB_INTERFACE_STATUS = *mut USB_INTERFACE_STATUS;
STRUCT!{#[repr(packed)] struct USB_ENDPOINT_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_ENDPOINT_STATUS AsUshort16: USHORT [
    Halt set_Halt[0..1],
    Reserved set_Reserved[1..16],
]}
pub type PUSB_ENDPOINT_STATUS = *mut USB_ENDPOINT_STATUS;
STRUCT!{#[repr(packed)] struct USB_COMMON_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
}}
pub type PUSB_COMMON_DESCRIPTOR = *mut USB_COMMON_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bcdUSB: USHORT,
    bDeviceClass: UCHAR,
    bDeviceSubClass: UCHAR,
    bDeviceProtocol: UCHAR,
    bMaxPacketSize0: UCHAR,
    idVendor: USHORT,
    idProduct: USHORT,
    bcdDevice: USHORT,
    iManufacturer: UCHAR,
    iProduct: UCHAR,
    iSerialNumber: UCHAR,
    bNumConfigurations: UCHAR,
}}
pub type PUSB_DEVICE_DESCRIPTOR = *mut USB_DEVICE_DESCRIPTOR;
pub const USB_DEVICE_CLASS_RESERVED: UCHAR = 0x00;
pub const USB_DEVICE_CLASS_AUDIO: UCHAR = 0x01;
pub const USB_DEVICE_CLASS_COMMUNICATIONS: UCHAR = 0x02;
pub const USB_DEVICE_CLASS_HUMAN_INTERFACE: UCHAR = 0x03;
pub const USB_DEVICE_CLASS_MONITOR: UCHAR = 0x04;
pub const USB_DEVICE_CLASS_PHYSICAL_INTERFACE: UCHAR = 0x05;
pub const USB_DEVICE_CLASS_POWER: UCHAR = 0x06;
pub const USB_DEVICE_CLASS_IMAGE: UCHAR = 0x06;
pub const USB_DEVICE_CLASS_PRINTER: UCHAR = 0x07;
pub const USB_DEVICE_CLASS_STORAGE: UCHAR = 0x08;
pub const USB_DEVICE_CLASS_HUB: UCHAR = 0x09;
pub const USB_DEVICE_CLASS_CDC_DATA: UCHAR = 0x0A;
pub const USB_DEVICE_CLASS_SMART_CARD: UCHAR = 0x0B;
pub const USB_DEVICE_CLASS_CONTENT_SECURITY: UCHAR = 0x0D;
pub const USB_DEVICE_CLASS_VIDEO: UCHAR = 0x0E;
pub const USB_DEVICE_CLASS_PERSONAL_HEALTHCARE: UCHAR = 0x0F;
pub const USB_DEVICE_CLASS_AUDIO_VIDEO: UCHAR = 0x10;
pub const USB_DEVICE_CLASS_BILLBOARD: UCHAR = 0x11;
pub const USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE: UCHAR = 0xDC;
pub const USB_DEVICE_CLASS_WIRELESS_CONTROLLER: UCHAR = 0xE0;
pub const USB_DEVICE_CLASS_MISCELLANEOUS: UCHAR = 0xEF;
pub const USB_DEVICE_CLASS_APPLICATION_SPECIFIC: UCHAR = 0xFE;
pub const USB_DEVICE_CLASS_VENDOR_SPECIFIC: UCHAR = 0xFF;
STRUCT!{#[repr(packed)] struct USB_DEVICE_QUALIFIER_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bcdUSB: USHORT,
    bDeviceClass: UCHAR,
    bDeviceSubClass: UCHAR,
    bDeviceProtocol: UCHAR,
    bMaxPacketSize0: UCHAR,
    bNumConfigurations: UCHAR,
    bReserved: UCHAR,
}}
pub type PUSB_DEVICE_QUALIFIER_DESCRIPTOR = *mut USB_DEVICE_QUALIFIER_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_BOS_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    wTotalLength: USHORT,
    bNumDeviceCaps: UCHAR,
}}
pub type PUSB_BOS_DESCRIPTOR = *mut USB_BOS_DESCRIPTOR;
pub const USB_DEVICE_CAPABILITY_WIRELESS_USB: UCHAR = 0x01;
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION: UCHAR = 0x02;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_USB: UCHAR = 0x03;
pub const USB_DEVICE_CAPABILITY_CONTAINER_ID: UCHAR = 0x04;
pub const USB_DEVICE_CAPABILITY_PLATFORM: UCHAR = 0x05;
pub const USB_DEVICE_CAPABILITY_POWER_DELIVERY: UCHAR = 0x06;
pub const USB_DEVICE_CAPABILITY_BATTERY_INFO: UCHAR = 0x07;
pub const USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT: UCHAR = 0x08;
pub const USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT: UCHAR = 0x09;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB: UCHAR = 0x0A;
pub const USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT: UCHAR = 0x0B;
pub const USB_DEVICE_CAPABILITY_BILLBOARD: UCHAR = 0x0D;
pub const USB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY: UCHAR = 0x10;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_bmAttributes {
    AsUlong: ULONG,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_bmAttributes AsUlong: ULONG [
    Reserved set_Reserved[0..1],
    LPMCapable set_LPMCapable[1..2],
    BESLAndAlternateHIRDSupported set_BESLAndAlternateHIRDSupported[2..3],
    BaselineBESLValid set_BaselineBESLValid[3..4],
    DeepBESLValid set_DeepBESLValid[4..5],
    Reserved1 set_Reserved1[5..8],
    BaselineBESL set_BaselineBESL[8..12],
    DeepBESL set_DeepBESL[12..16],
    Reserved2 set_Reserved2[16..32],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bmAttributes: USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_bmAttributes,
}}
pub type PUSB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR;
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION_BMATTRIBUTES_RESERVED_MASK: ULONG = 0xFFFF00E1;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_bmAttributes {
    AsUlong: ULONG,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_bmAttributes AsUlong: ULONG [
    Reserved set_Reserved[0..1],
    BatteryCharging set_BatteryCharging[1..2],
    USBPowerDelivery set_USBPowerDelivery[2..3],
    Provider set_Provider[3..4],
    Consumer set_Consumer[4..5],
    ChargingPolicy set_ChargingPolicy[5..6],
    TypeCCurrent set_TypeCCurrent[6..7],
    Reserved2 set_Reserved2[7..8],
    ACSupply set_ACSupply[8..9],
    Battery set_Battery[9..10],
    Other set_Other[10..11],
    NumBatteries set_NumBatteries[11..14],
    UsesVbus set_UsesVbus[14..15],
    Reserved3 set_Reserved3[15..32],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bReserved: UCHAR,
    bmAttributes: USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_bmAttributes,
    bmProviderPorts: USHORT,
    bmConsumerPorts: USHORT,
    bcdBCVersion: USHORT,
    bcdPDVersion: USHORT,
    bcdUSBTypeCVersion: USHORT,
}}
pub type PUSB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_bmCapabilities {
    AsUshort: USHORT,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_bmCapabilities AsUshort: USHORT [
    BatteryCharging set_BatteryCharging[0..1],
    USBPowerDelivery set_USBPowerDelivery[1..2],
    USBTypeCCurrent set_USBTypeCCurrent[2..3],
    Reserved set_Reserved[3..16],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bReserved: UCHAR,
    bmCapabilities: USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_bmCapabilities,
    wMinVoltage: USHORT,
    wMaxVoltage: USHORT,
    wReserved: USHORT,
    dwMaxOperatingPower: ULONG,
    dwMaxPeakPower: ULONG,
    dwMaxPeakPowerTime: ULONG,
}}
pub type PUSB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bmAttributes: UCHAR,
    wSpeedsSupported: USHORT,
    bFunctionalitySupport: UCHAR,
    bU1DevExitLat: UCHAR,
    wU2DevExitLat: USHORT,
}}
pub type PUSB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK: UCHAR = 0xFD;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE: UCHAR = 0x02;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK: USHORT = 0xFFF0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW: USHORT = 0x0001;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL: USHORT = 0x0002;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH: USHORT = 0x0004;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER: USHORT = 0x0008;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE: UCHAR = 0x0A;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE: USHORT = 0x07FF;
pub const USB_DEVICE_CAPABILITY_MAX_U1_LATENCY: UCHAR = 0x0A;
pub const USB_DEVICE_CAPABILITY_MAX_U2_LATENCY: USHORT = 0x07FF;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS: ULONG = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS: ULONG = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS: ULONG = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS: ULONG = 3;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC: ULONG = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC: ULONG = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX: ULONG = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX: ULONG = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS: ULONG = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP: ULONG = 1;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    AsUlong32: ULONG,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED AsUlong32: ULONG [
    SublinkSpeedAttrID set_SublinkSpeedAttrID[0..4],
    LaneSpeedExponent set_LaneSpeedExponent[4..6],
    SublinkTypeMode set_SublinkTypeMode[6..7],
    SublinkTypeDir set_SublinkTypeDir[7..8],
    Reserved set_Reserved[8..14],
    LinkProtocol set_LinkProtocol[14..16],
    LaneSpeedMantissa set_LaneSpeedMantissa[16..32],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_bmAttributes {
    AsUlong32: ULONG,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_bmAttributes AsUlong32: ULONG [
    SublinkSpeedAttrCount set_SublinkSpeedAttrCount[0..5],
    SublinkSpeedIDCount set_SublinkSpeedIDCount[5..9],
    Reserved set_Reserved[9..32],
]}
STRUCT!{#[repr(packed)]
    struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_wFunctionalitySupport {
    AsUshort: USHORT,
}}
BITFIELD!{
    USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_wFunctionalitySupport AsUshort: USHORT [
    SublinkSpeedAttrID set_SublinkSpeedAttrID[0..4],
    Reserved set_Reserved[4..8],
    MinRxLaneCount set_MinRxLaneCount[8..12],
    MinTxLaneCount set_MinTxLaneCount[12..16],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bReserved: UCHAR,
    bmAttributes: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_bmAttributes,
    wFunctionalitySupport:
        USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_wFunctionalitySupport,
    wReserved: USHORT,
    bmSublinkSpeedAttr: [USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED; 1],
}}
pub type PUSB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bReserved: UCHAR,
    ContainerID: [UCHAR; 16],
}}
pub type PUSB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY_DESCRIPTOR_Function {
    bClass: UCHAR,
    bSubClass: UCHAR,
    bProtocol: UCHAR,
}}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bcdVersion: USHORT,
    bConfigurationValue: UCHAR,
    bMaxPower: UCHAR,
    bNumFunctions: UCHAR,
    Function: [USB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY_DESCRIPTOR_Function; 1],
}}
pub type PUSB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_CONFIGURATION_SUMMARY_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    bReserved: UCHAR,
    PlatformCapabilityUuid: GUID,
    CapabililityData: [UCHAR; 1],
}}
pub type PUSB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_VconnPower {
    AsUshort: USHORT,
}}
BITFIELD!{USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_VconnPower AsUshort: USHORT [
    VConnPowerNeededForFullFunctionality set_VConnPowerNeededForFullFunctionality[0..3],
    Reserved set_Reserved[3..15],
    NoVconnPowerRequired set_NoVconnPowerRequired[15..16],
]}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_AlternateMode {
    wSVID: USHORT,
    bAlternateMode: UCHAR,
    iAlternateModeSetting: UCHAR,
}}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
    iAddtionalInfoURL: UCHAR,
    bNumberOfAlternateModes: UCHAR,
    bPreferredAlternateMode: UCHAR,
    VconnPower: USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_VconnPower,
    bmConfigured: [UCHAR; 32],
    bReserved: ULONG,
    AlternateMode: [USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_AlternateMode; 1],
}}
pub type PUSB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR
    = *mut USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR;
DEFINE_GUID!{GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID,
    0xd8dd60df, 0x4589, 0x4cc7, 0x9c, 0xd2, 0x65, 0x9d, 0x9e, 0x64, 0x8a, 0x9f}
STRUCT!{#[repr(packed)] struct USB_DEVICE_CAPABILITY_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bDevCapabilityType: UCHAR,
}}
pub type PUSB_DEVICE_CAPABILITY_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_CONFIGURATION_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    wTotalLength: USHORT,
    bNumInterfaces: UCHAR,
    bConfigurationValue: UCHAR,
    iConfiguration: UCHAR,
    bmAttributes: UCHAR,
    MaxPower: UCHAR,
}}
pub type PUSB_CONFIGURATION_DESCRIPTOR = *mut USB_CONFIGURATION_DESCRIPTOR;
pub const USB_CONFIG_POWERED_MASK: UCHAR = 0xC0;
pub const USB_CONFIG_BUS_POWERED: UCHAR = 0x80;
pub const USB_CONFIG_SELF_POWERED: UCHAR = 0x40;
pub const USB_CONFIG_REMOTE_WAKEUP: UCHAR = 0x20;
pub const USB_CONFIG_RESERVED: UCHAR = 0x1F;
STRUCT!{#[repr(packed)] struct USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bFirstInterface: UCHAR,
    bInterfaceCount: UCHAR,
    bFunctionClass: UCHAR,
    bFunctionSubClass: UCHAR,
    bFunctionProtocol: UCHAR,
    iFunction: UCHAR,
}}
pub type PUSB_INTERFACE_ASSOCIATION_DESCRIPTOR = *mut USB_INTERFACE_ASSOCIATION_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_INTERFACE_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bInterfaceNumber: UCHAR,
    bAlternateSetting: UCHAR,
    bNumEndpoints: UCHAR,
    bInterfaceClass: UCHAR,
    bInterfaceSubClass: UCHAR,
    bInterfaceProtocol: UCHAR,
    iInterface: UCHAR,
}}
pub type PUSB_INTERFACE_DESCRIPTOR = *mut USB_INTERFACE_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_ENDPOINT_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bEndpointAddress: UCHAR,
    bmAttributes: UCHAR,
    wMaxPacketSize: USHORT,
    bInterval: UCHAR,
}}
pub type PUSB_ENDPOINT_DESCRIPTOR = *mut USB_ENDPOINT_DESCRIPTOR;
pub const USB_ENDPOINT_DIRECTION_MASK: UCHAR = 0x80;
#[inline]
pub fn USB_ENDPOINT_DIRECTION_OUT(addr: UCHAR) -> UCHAR {
    !(addr & USB_ENDPOINT_DIRECTION_MASK)
}
#[inline]
pub fn USB_ENDPOINT_DIRECTION_IN(addr: UCHAR) -> UCHAR {
    addr & USB_ENDPOINT_DIRECTION_MASK
}
pub const USB_ENDPOINT_ADDRESS_MASK: UCHAR = 0x0F;
pub const USB_ENDPOINT_TYPE_MASK: UCHAR = 0x03;
pub const USB_ENDPOINT_TYPE_CONTROL: UCHAR = 0x00;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: UCHAR = 0x01;
pub const USB_ENDPOINT_TYPE_BULK: UCHAR = 0x02;
pub const USB_ENDPOINT_TYPE_INTERRUPT: UCHAR = 0x03;
pub const USB_ENDPOINT_TYPE_BULK_RESERVED_MASK: UCHAR = 0xFC;
pub const USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK: UCHAR = 0xFC;
pub const USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: UCHAR = 0xFC;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: UCHAR = 0xCC;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK: UCHAR = 0xC0;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK: UCHAR = 0x30;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC: UCHAR = 0x00;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION: UCHAR = 0x10;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10: UCHAR = 0x20;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11: UCHAR = 0x30;
#[inline]
pub fn USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE(bmAttr: UCHAR) -> UCHAR {
    bmAttr & USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK
}
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK: UCHAR = 0x0C;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION: UCHAR = 0x00;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS: UCHAR = 0x04;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE: UCHAR = 0x08;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS: UCHAR = 0x0C;
#[inline]
pub fn USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION(bmAttr: UCHAR) -> UCHAR {
    bmAttr & USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK
}
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK: UCHAR = 0x30;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT: UCHAR = 0x00;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT: UCHAR = 0x10;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT: UCHAR = 0x20;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED: UCHAR = 0x30;
#[inline]
pub fn USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE(bmAttr: UCHAR) -> UCHAR {
    bmAttr & USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK
}
STRUCT!{#[repr(packed)] struct USB_HIGH_SPEED_MAXPACKET {
    us: USHORT,
}}
BITFIELD!{USB_HIGH_SPEED_MAXPACKET us: USHORT [
    MaxPacket set_MaxPacket[0..11],
    HSmux set_HSmux[11..13],
    Reserved set_Reserved[13..16],
]}
pub type PUSB_HIGH_SPEED_MAXPACKET = *mut USB_HIGH_SPEED_MAXPACKET;
pub const USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE: USHORT = 1024;
pub const USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE: USHORT = 512;
pub const USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE: USHORT = 1024;
pub const USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE: USHORT = 1024;
STRUCT!{#[repr(packed)] struct USB_STRING_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bString: [WCHAR; 1],
}}
pub type PUSB_STRING_DESCRIPTOR = *mut USB_STRING_DESCRIPTOR;
pub const MAXIMUM_USB_STRING_LENGTH: UCHAR = 255;
STRUCT!{#[repr(packed)] struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Bulk {
    BitField: UCHAR,
}}
BITFIELD!{USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Bulk BitField: UCHAR [
    MaxStreams set_MaxStreams[0..5],
    Reserved1 set_Reserved1[5..8],
]}
STRUCT!{#[repr(packed)]
    struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Isochronous {
    BitField: UCHAR,
}}
BITFIELD!{USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Isochronous BitField: UCHAR [
    Mult set_Mult[0..2],
    Reserved2 set_Reserved2[2..7],
    SspCompanion set_SspCompanion[7..8],
]}
UNION!{#[repr(packed)] union USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes {
    [u8; 1],
    AsUchar AsUchar_mut: UCHAR,
    Bulk Bulk_mut: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Bulk,
    Isochronous Isochronous_mut:
        USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes_Isochronous,
}}
STRUCT!{#[repr(packed)] struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bMaxBurst: UCHAR,
    bmAttributes: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_bmAttributes,
    wBytesPerInterval: USHORT,
}}
pub type PUSB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR
    = *mut USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR;
pub const USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER: UCHAR = 2;
STRUCT!{#[repr(packed)] struct USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    wReserved: USHORT,
    dwBytesPerInterval: ULONG,
}}
pub type PUSB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR
    = *mut USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL: ULONG = 0xC001;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL: ULONG = 0xFFFFFF;
STRUCT!{#[repr(packed)] struct USB_HUB_DESCRIPTOR {
    bDescriptorLength: UCHAR,
    bDescriptorType: UCHAR,
    bNumberOfPorts: UCHAR,
    wHubCharacteristics: USHORT,
    bPowerOnToPowerGood: UCHAR,
    bHubControlCurrent: UCHAR,
    bRemoveAndPowerMask: [UCHAR; 64],
}}
pub type PUSB_HUB_DESCRIPTOR = *mut USB_HUB_DESCRIPTOR;
pub const USB_20_HUB_DESCRIPTOR_TYPE: UCHAR = 0x29;
STRUCT!{#[repr(packed)] struct USB_30_HUB_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bNumberOfPorts: UCHAR,
    wHubCharacteristics: USHORT,
    bPowerOnToPowerGood: UCHAR,
    bHubControlCurrent: UCHAR,
    bHubHdrDecLat: UCHAR,
    wHubDelay: USHORT,
    DeviceRemovable: USHORT,
}}
pub type PUSB_30_HUB_DESCRIPTOR = *mut USB_30_HUB_DESCRIPTOR;
pub const USB_30_HUB_DESCRIPTOR_TYPE: UCHAR = 0x2A;
pub const USB_REQUEST_GET_STATE: UCHAR = 0x02;
pub const USB_REQUEST_CLEAR_TT_BUFFER: UCHAR = 0x08;
pub const USB_REQUEST_RESET_TT: UCHAR = 0x09;
pub const USB_REQUEST_GET_TT_STATE: UCHAR = 0x0A;
pub const USB_REQUEST_STOP_TT: UCHAR = 0x0B;
pub const USB_REQUEST_SET_HUB_DEPTH: UCHAR = 0x0C;
pub const USB_REQUEST_GET_PORT_ERR_COUNT: UCHAR = 0x0D;
STRUCT!{#[repr(packed)] struct USB_HUB_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_HUB_STATUS AsUshort16: USHORT [
    LocalPowerLost set_LocalPowerLost[0..1],
    OverCurrent set_OverCurrent[1..2],
    Reserved set_Reserved[2..16],
]}
pub type PUSB_HUB_STATUS = *mut USB_HUB_STATUS;
STRUCT!{#[repr(packed)] struct USB_HUB_CHANGE {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_HUB_CHANGE AsUshort16: USHORT [
    LocalPowerChange set_LocalPowerChange[0..1],
    OverCurrentChange set_OverCurrentChange[1..2],
    Reserved set_Reserved[2..16],
]}
pub type PUSB_HUB_CHANGE = *mut USB_HUB_CHANGE;
STRUCT!{#[repr(packed)] struct USB_HUB_STATUS_AND_CHANGE_s {
    HubStatus: USB_HUB_STATUS,
    HubChange: USB_HUB_CHANGE,
}}
UNION!{#[repr(packed)] union USB_HUB_STATUS_AND_CHANGE {
    [u32; 1],
    AsUlong32 AsUlong32_mut: ULONG,
    s s_mut: USB_HUB_STATUS_AND_CHANGE_s,
}}
pub type PUSB_HUB_STATUS_AND_CHANGE = *mut USB_HUB_STATUS_AND_CHANGE;
STRUCT!{#[repr(packed)] struct USB_20_PORT_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_20_PORT_STATUS AsUshort16: USHORT [
    CurrentConnectStatus set_CurrentConnectStatus[0..1],
    PortEnabledDisabled set_PortEnabledDisabled[1..2],
    Suspend set_Suspend[2..3],
    OverCurrent set_OverCurrent[3..4],
    Reset set_Reset[4..5],
    L1 set_L1[5..6],
    Reserved0 set_Reserved0[6..8],
    PortPower set_PortPower[8..9],
    LowSpeedDeviceAttached set_LowSpeedDeviceAttached[9..10],
    HighSpeedDeviceAttached set_HighSpeedDeviceAttached[10..11],
    PortTestMode set_PortTestMode[11..12],
    PortIndicatorControl set_PortIndicatorControl[12..13],
    Reserved1 set_Reserved1[13..16],
]}
pub type PUSB_20_PORT_STATUS = *mut USB_20_PORT_STATUS;
pub const USB_PORT_STATUS_CONNECT: USHORT = 0x0001;
pub const USB_PORT_STATUS_ENABLE: USHORT = 0x0002;
pub const USB_PORT_STATUS_SUSPEND: USHORT = 0x0004;
pub const USB_PORT_STATUS_OVER_CURRENT: USHORT = 0x0008;
pub const USB_PORT_STATUS_RESET: USHORT = 0x0010;
pub const USB_PORT_STATUS_POWER: USHORT = 0x0100;
pub const USB_PORT_STATUS_LOW_SPEED: USHORT = 0x0200;
pub const USB_PORT_STATUS_HIGH_SPEED: USHORT = 0x0400;
STRUCT!{#[repr(packed)] struct USB_20_PORT_CHANGE {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_20_PORT_CHANGE AsUshort16: USHORT [
    ConnectStatusChange set_ConnectStatusChange[0..1],
    PortEnableDisableChange set_PortEnableDisableChange[1..2],
    SuspendChange set_SuspendChange[2..3],
    OverCurrentIndicatorChange set_OverCurrentIndicatorChange[3..4],
    ResetChange set_ResetChange[4..5],
    Reserved2 set_Reserved2[5..16],
]}
pub type PUSB_20_PORT_CHANGE = *mut USB_20_PORT_CHANGE;
STRUCT!{#[repr(packed)] struct USB_30_PORT_STATUS {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_30_PORT_STATUS AsUshort16: USHORT [
    CurrentConnectStatus set_CurrentConnectStatus[0..1],
    PortEnabledDisabled set_PortEnabledDisabled[1..2],
    Reserved0 set_Reserved0[2..3],
    OverCurrent set_OverCurrent[3..4],
    Reset set_Reset[4..5],
    PortLinkState set_PortLinkState[5..9],
    PortPower set_PortPower[9..10],
    NegotiatedDeviceSpeed set_NegotiatedDeviceSpeed[10..13],
    Reserved1 set_Reserved1[13..16],
]}
pub type PUSB_30_PORT_STATUS = *mut USB_30_PORT_STATUS;
pub const PORT_LINK_STATE_U0: USHORT = 0;
pub const PORT_LINK_STATE_U1: USHORT = 1;
pub const PORT_LINK_STATE_U2: USHORT = 2;
pub const PORT_LINK_STATE_U3: USHORT = 3;
pub const PORT_LINK_STATE_DISABLED: USHORT = 4;
pub const PORT_LINK_STATE_RX_DETECT: USHORT = 5;
pub const PORT_LINK_STATE_INACTIVE: USHORT = 6;
pub const PORT_LINK_STATE_POLLING: USHORT = 7;
pub const PORT_LINK_STATE_RECOVERY: USHORT = 8;
pub const PORT_LINK_STATE_HOT_RESET: USHORT = 9;
pub const PORT_LINK_STATE_COMPLIANCE_MODE: USHORT = 10;
pub const PORT_LINK_STATE_LOOPBACK: USHORT = 11;
pub const PORT_LINK_STATE_TEST_MODE: USHORT = 11;
STRUCT!{#[repr(packed)] struct USB_30_PORT_CHANGE {
    AsUshort16: USHORT,
}}
BITFIELD!{USB_30_PORT_CHANGE AsUshort16: USHORT [
    ConnectStatusChange set_ConnectStatusChange[0..1],
    Reserved2 set_Reserved2[1..3],
    OverCurrentIndicatorChange set_OverCurrentIndicatorChange[3..4],
    ResetChange set_ResetChange[4..5],
    BHResetChange set_BHResetChange[5..6],
    PortLinkStateChange set_PortLinkStateChange[6..7],
    PortConfigErrorChange set_PortConfigErrorChange[7..8],
    Reserved3 set_Reserved3[8..16],
]}
pub type PUSB_30_PORT_CHANGE = *mut USB_30_PORT_CHANGE;
UNION!{#[repr(packed)] union USB_PORT_STATUS {
    [u16; 1],
    AsUshort16 AsUshort16_mut: USHORT,
    Usb20PortStatus Usb20PortStatus_mut: USB_20_PORT_STATUS,
    Usb30PortStatus Usb30PortStatus_mut: USB_30_PORT_STATUS,
}}
pub type PUSB_PORT_STATUS = *mut USB_PORT_STATUS;
UNION!{#[repr(packed)] union USB_PORT_CHANGE {
    [u16; 1],
    AsUshort16 AsUshort16_mut: USHORT,
    Usb20PortChange Usb20PortChange_mut: USB_20_PORT_CHANGE,
    Usb30PortChange Usb30PortChange_mut: USB_30_PORT_CHANGE,
}}
pub type PUSB_PORT_CHANGE = *mut USB_PORT_CHANGE;
STRUCT!{#[repr(packed)] struct USB_PORT_EXT_STATUS {
    AsUlong32: ULONG,
}}
BITFIELD!{USB_PORT_EXT_STATUS AsUlong32: ULONG [
    RxSublinkSpeedID set_RxSublinkSpeedID[0..4],
    TxSublinkSpeedID set_TxSublinkSpeedID[4..8],
    RxLaneCount set_RxLaneCount[8..12],
    TxLaneCount set_TxLaneCount[12..16],
    Reserved set_Reserved[16..32],
]}
pub type PUSB_PORT_EXT_STATUS = *mut USB_PORT_EXT_STATUS;
STRUCT!{#[repr(packed)] struct USB_PORT_STATUS_AND_CHANGE_s {
    PortStatus: USB_PORT_STATUS,
    PortChange: USB_PORT_CHANGE,
}}
UNION!{#[repr(packed)] union USB_PORT_STATUS_AND_CHANGE {
    [u32; 1],
    AsUlong32 AsUlong32_mut: ULONG,
    s s_mut: USB_PORT_STATUS_AND_CHANGE_s,
}}
pub type PUSB_PORT_STATUS_AND_CHANGE = *mut USB_PORT_STATUS_AND_CHANGE;
STRUCT!{#[repr(packed)] struct USB_PORT_EXT_STATUS_AND_CHANGE_s {
    PortStatusChange: USB_PORT_STATUS_AND_CHANGE,
    PortExtStatus: USB_PORT_EXT_STATUS,
}}
UNION!{#[repr(packed)] union USB_PORT_EXT_STATUS_AND_CHANGE {
    [u64; 1],
    AsUlong64 AsUlong64_mut: ULONG64,
    s s_mut: USB_PORT_EXT_STATUS_AND_CHANGE_s,
}}
pub type PUSB_PORT_EXT_STATUS_AND_CHANGE = *mut USB_PORT_EXT_STATUS_AND_CHANGE;
STRUCT!{#[repr(packed)] struct USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    AsUchar8: UCHAR,
}}
BITFIELD!{USB_HUB_30_PORT_REMOTE_WAKE_MASK AsUchar8: UCHAR [
    ConnectRemoteWakeEnable set_ConnectRemoteWakeEnable[0..1],
    DisconnectRemoteWakeEnable set_DisconnectRemoteWakeEnable[1..2],
    OverCurrentRemoteWakeEnable set_OverCurrentRemoteWakeEnable[2..3],
    Reserved0 set_Reserved0[3..8],
]}
pub type PUSB_HUB_30_PORT_REMOTE_WAKE_MASK = *mut USB_HUB_30_PORT_REMOTE_WAKE_MASK;
STRUCT!{#[repr(packed)] struct USB_FUNCTION_SUSPEND_OPTIONS {
    AsUchar: UCHAR,
}}
BITFIELD!{USB_FUNCTION_SUSPEND_OPTIONS AsUchar: UCHAR [
    PowerState set_PowerState[0..1],
    RemoteWakeEnabled set_RemoteWakeEnabled[1..2],
    Reserved0 set_Reserved0[2..8],
]}
pub type PUSB_FUNCTION_SUSPEND_OPTIONS = *mut USB_FUNCTION_SUSPEND_OPTIONS;
pub const USB_FEATURE_INTERFACE_POWER_D0: USHORT = 0x0002;
pub const USB_FEATURE_INTERFACE_POWER_D1: USHORT = 0x0003;
pub const USB_FEATURE_INTERFACE_POWER_D2: USHORT = 0x0004;
pub const USB_FEATURE_INTERFACE_POWER_D3: USHORT = 0x0005;
pub const USB_SUPPORT_D0_COMMAND: UCHAR = 0x01;
pub const USB_SUPPORT_D1_COMMAND: UCHAR = 0x02;
pub const USB_SUPPORT_D2_COMMAND: UCHAR = 0x04;
pub const USB_SUPPORT_D3_COMMAND: UCHAR = 0x08;
pub const USB_SUPPORT_D1_WAKEUP: UCHAR = 0x10;
pub const USB_SUPPORT_D2_WAKEUP: UCHAR = 0x20;
STRUCT!{#[repr(packed)] struct USB_CONFIGURATION_POWER_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    SelfPowerConsumedD0: [UCHAR; 3],
    bPowerSummaryId: UCHAR,
    bBusPowerSavingD1: UCHAR,
    bSelfPowerSavingD1: UCHAR,
    bBusPowerSavingD2: UCHAR,
    bSelfPowerSavingD2: UCHAR,
    bBusPowerSavingD3: UCHAR,
    bSelfPowerSavingD3: UCHAR,
    TransitionTimeFromD1: USHORT,
    TransitionTimeFromD2: USHORT,
    TransitionTimeFromD3: USHORT,
}}
pub type PUSB_CONFIGURATION_POWER_DESCRIPTOR = *mut USB_CONFIGURATION_POWER_DESCRIPTOR;
STRUCT!{#[repr(packed)] struct USB_INTERFACE_POWER_DESCRIPTOR {
    bLength: UCHAR,
    bDescriptorType: UCHAR,
    bmCapabilitiesFlags: UCHAR,
    bBusPowerSavingD1: UCHAR,
    bSelfPowerSavingD1: UCHAR,
    bBusPowerSavingD2: UCHAR,
    bSelfPowerSavingD2: UCHAR,
    bBusPowerSavingD3: UCHAR,
    bSelfPowerSavingD3: UCHAR,
    TransitionTimeFromD1: USHORT,
    TransitionTimeFromD2: USHORT,
    TransitionTimeFromD3: USHORT,
}}
pub type PUSB_INTERFACE_POWER_DESCRIPTOR = *mut USB_INTERFACE_POWER_DESCRIPTOR;
