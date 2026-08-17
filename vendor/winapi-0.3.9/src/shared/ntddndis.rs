// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::ifdef::IF_MAX_STRING_SIZE;
use shared::minwindef::{UCHAR, USHORT};
//290
STRUCT!{struct NDIS_OBJECT_HEADER {
    Type: UCHAR,
    Revision: UCHAR,
    Size: USHORT,
}}
pub type PNDIS_OBJECT_HEADER = *mut NDIS_OBJECT_HEADER;
//2274
ENUM!{enum NDIS_MEDIUM {
    NdisMedium802_3 = 0,
    NdisMedium802_5 = 1,
    NdisMediumFddi = 2,
    NdisMediumWan = 3,
    NdisMediumLocalTalk = 4,
    NdisMediumDix = 5,
    NdisMediumArcnetRaw = 6,
    NdisMediumArcnet878_2 = 7,
    NdisMediumAtm = 8,
    NdisMediumWirelessWan = 9,
    NdisMediumIrda = 10,
    NdisMediumBpc = 11,
    NdisMediumCoWan = 12,
    NdisMedium1394 = 13,
    NdisMediumInfiniBand = 14,
    NdisMediumTunnel = 15,
    NdisMediumNative802_11 = 16,
    NdisMediumLoopback = 17,
    NdisMediumWiMAX = 18,
    NdisMediumIP = 19,
    NdisMediumMax = 20,
}}
pub type PNDIS_MEDIUM = *mut NDIS_MEDIUM;
ENUM!{enum NDIS_PHYSICAL_MEDIUM {
    NdisPhysicalMediumUnspecified = 0,
    NdisPhysicalMediumWirelessLan = 1,
    NdisPhysicalMediumCableModem = 2,
    NdisPhysicalMediumPhoneLine = 3,
    NdisPhysicalMediumPowerLine = 4,
    NdisPhysicalMediumDSL = 5,
    NdisPhysicalMediumFibreChannel = 6,
    NdisPhysicalMedium1394 = 7,
    NdisPhysicalMediumWirelessWan = 8,
    NdisPhysicalMediumNative802_11 = 9,
    NdisPhysicalMediumBluetooth = 10,
    NdisPhysicalMediumInfiniband = 11,
    NdisPhysicalMediumWiMax = 12,
    NdisPhysicalMediumUWB = 13,
    NdisPhysicalMedium802_3 = 14,
    NdisPhysicalMedium802_5 = 15,
    NdisPhysicalMediumIrda = 16,
    NdisPhysicalMediumWiredWAN = 17,
    NdisPhysicalMediumWiredCoWan = 18,
    NdisPhysicalMediumOther = 19,
    NdisPhysicalMediumMax = 20,
}}
pub type PNDIS_PHYSICAL_MEDIUM = *mut NDIS_PHYSICAL_MEDIUM;
//2691
pub type NDIS_STATUS = c_int;
pub type PNDIS_STATUS = *mut c_int;
//2736
pub const NDIS_PACKET_TYPE_DIRECTED: u32 = 0x00000001;
pub const NDIS_PACKET_TYPE_MULTICAST: u32 = 0x00000002;
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: u32 = 0x00000004;
pub const NDIS_PACKET_TYPE_BROADCAST: u32 = 0x00000008;
pub const NDIS_PACKET_TYPE_PROMISCUOUS: u32 = 0x00000020;
//2835
pub const NDIS_IF_MAX_STRING_SIZE: usize = IF_MAX_STRING_SIZE;
