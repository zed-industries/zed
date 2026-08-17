// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Constants and types for accessing SCSI port adapters.
use shared::basetsd::{ULONG32, ULONG_PTR};
use shared::minwindef::{UCHAR, ULONG, USHORT};
use shared::ntdef::{LARGE_INTEGER, LONG, LONGLONG, PVOID, ULONGLONG, VOID, WCHAR};
use um::winioctl::{
    DEVICE_TYPE, FILE_ANY_ACCESS, FILE_DEVICE_CONTROLLER, FILE_READ_ACCESS,
    FILE_WRITE_ACCESS, METHOD_BUFFERED
};
use um::winnt::{ANYSIZE_ARRAY, BOOLEAN, PBOOLEAN};
DEFINE_GUID!{ScsiRawInterfaceGuid,
    0x53f56309, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b}
DEFINE_GUID!{WmiScsiAddressGuid,
    0x53f5630f, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b}
pub const IOCTL_SCSI_BASE: DEVICE_TYPE = FILE_DEVICE_CONTROLLER;
pub const FILE_DEVICE_SCSI: ULONG = 0x0000001;
pub const DD_SCSI_DEVICE_NAME: &'static str = "\\Device\\ScsiPort";
pub const IOCTL_SCSI_PASS_THROUGH: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_MINIPORT: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0402, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_GET_INQUIRY_DATA: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0403, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SCSI_GET_CAPABILITIES: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0404, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0405, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_GET_ADDRESS: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0406, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SCSI_RESCAN_BUS: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0407, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SCSI_GET_DUMP_POINTERS: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0408, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0409, METHOD_BUFFERED, FILE_ANY_ACCESS);
pub const IOCTL_IDE_PASS_THROUGH: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040a, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_ATA_PASS_THROUGH: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040b, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040c, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_ATA_MINIPORT: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040d, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040e, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_MPIO_PASS_THROUGH_PATH: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x040f, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0410, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_PASS_THROUGH_EX: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0411, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0412, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0413, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: ULONG =
    CTL_CODE!(IOCTL_SCSI_BASE, 0x0414, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
pub const IOCTL_SCSI_MINIPORT_NVCACHE: ULONG = (FILE_DEVICE_SCSI << 16) + 0x0600;
pub const IOCTL_SCSI_MINIPORT_HYBRID: ULONG = (FILE_DEVICE_SCSI << 16) + 0x0620;
pub const IOCTL_SCSI_MINIPORT_FIRMWARE: ULONG = (FILE_DEVICE_SCSI << 16) + 0x0780;
STRUCT!{struct SCSI_PASS_THROUGH {
    Length: USHORT,
    ScsiStatus: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    CdbLength: UCHAR,
    SenseInfoLength: UCHAR,
    DataIn: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    DataBufferOffset: ULONG_PTR,
    SenseInfoOffset: ULONG,
    Cdb: [UCHAR; 16],
}}
pub type PSCSI_PASS_THROUGH = *mut SCSI_PASS_THROUGH;
STRUCT!{struct SCSI_PASS_THROUGH_DIRECT {
    Length: USHORT,
    ScsiStatus: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    CdbLength: UCHAR,
    SenseInfoLength: UCHAR,
    DataIn: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    DataBuffer: PVOID,
    SenseInfoOffset: ULONG,
    Cdb: [UCHAR; 16],
}}
pub type PSCSI_PASS_THROUGH_DIRECT = *mut SCSI_PASS_THROUGH_DIRECT;
STRUCT!{struct SCSI_PASS_THROUGH32 {
    Length: USHORT,
    ScsiStatus: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    CdbLength: UCHAR,
    SenseInfoLength: UCHAR,
    DataIn: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    DataBufferOffset: ULONG32,
    SenseInfoOffset: ULONG,
    Cdb: [UCHAR; 16],
}}
#[cfg(target_arch = "x86_64")]
IFDEF!{
pub type PSCSI_PASS_THROUGH32 = *mut SCSI_PASS_THROUGH32;
STRUCT!{struct SCSI_PASS_THROUGH_DIRECT32 {
    Length: USHORT,
    ScsiStatus: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    CdbLength: UCHAR,
    SenseInfoLength: UCHAR,
    DataIn: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    DataBuffer: ULONG32, // Rust doesn't have anything like __ptr32
    SenseInfoOffset: ULONG,
    Cdb: [UCHAR; 16],
}}
pub type PSCSI_PASS_THROUGH_DIRECT32 = *mut SCSI_PASS_THROUGH_DIRECT32;
}
STRUCT!{struct SCSI_PASS_THROUGH_EX {
    Version: ULONG,
    Length: ULONG,
    CdbLength: ULONG,
    StorAddressLength: ULONG,
    ScsiStatus: UCHAR,
    SenseInfolength: UCHAR,
    DataDirection: UCHAR,
    Reserved: UCHAR,
    TimeOutValue: ULONG,
    StorAddressOffset: ULONG,
    SenseInfoOffset: ULONG,
    DataOutTransferLength: ULONG,
    DataInTransferLength: ULONG,
    DataOutBufferOffset: ULONG_PTR,
    DataInBufferOffset: ULONG_PTR,
    Cdb: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSCSI_PASS_THROUGH_EX = *mut SCSI_PASS_THROUGH_EX;
STRUCT!{struct SCSI_PASS_THROUGH_DIRECT_EX {
    Version: ULONG,
    Length: ULONG,
    CdbLength: ULONG,
    StorAddressLength: ULONG,
    ScsiStatus: UCHAR,
    SenseInfolength: UCHAR,
    DataDirection: UCHAR,
    Reserved: UCHAR,
    TimeOutValue: ULONG,
    StorAddressOffset: ULONG,
    SenseInfoOffset: ULONG,
    DataOutTransferLength: ULONG,
    DataInTransferLength: ULONG,
    DataOutBuffer: *mut VOID,
    DataInBuffer: *mut VOID,
    Cdb: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSCSI_PASS_THROUGH_DIRECT_EX = *mut SCSI_PASS_THROUGH_DIRECT_EX;
#[cfg(target_arch = "x86_64")]
IFDEF!{
STRUCT!{struct SCSI_PASS_THROUGH32_EX {
    Version: ULONG,
    Length: ULONG,
    CdbLength: ULONG,
    StorAddressLength: ULONG,
    ScsiStatus: UCHAR,
    SenseInfolength: UCHAR,
    DataDirection: UCHAR,
    Reserved: UCHAR,
    TimeOutValue: ULONG,
    StorAddressOffset: ULONG,
    SenseInfoOffset: ULONG,
    DataOutTransferLength: ULONG,
    DataInTransferLength: ULONG,
    DataOutBufferOffset: ULONG32,
    DataInBufferOffset: ULONG32,
    Cdb: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSCSI_PASS_THROUGH32_EX = *mut SCSI_PASS_THROUGH32_EX;
STRUCT!{struct SCSI_PASS_THROUGH_DIRECT32_EX {
    Version: ULONG,
    Length: ULONG,
    CdbLength: ULONG,
    StorAddressLength: ULONG,
    ScsiStatus: UCHAR,
    SenseInfolength: UCHAR,
    DataDirection: UCHAR,
    Reserved: UCHAR,
    TimeOutValue: ULONG,
    StorAddressOffset: ULONG,
    SenseInfoOffset: ULONG,
    DataOutTransferLength: ULONG,
    DataInTransferLength: ULONG,
    DataOutBuffer: ULONG32,
    DataInBuffer: ULONG32,
    Cdb: [UCHAR; ANYSIZE_ARRAY],
}}
pub type PSCSI_PASS_THROUGH_DIRECT32_EX = *mut SCSI_PASS_THROUGH_DIRECT32_EX;
}
STRUCT!{struct ATA_PASS_THROUGH_EX {
    Length: USHORT,
    AtaFlags: USHORT,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    ReservedAsUchar: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    ReservedAsUlong: ULONG,
    DataBufferOffset: ULONG_PTR,
    PreviousTaskFile: [UCHAR; 8],
    CurrentTaskFile: [UCHAR; 8],
}}
pub type PATA_PASS_THROUGH_EX = *mut ATA_PASS_THROUGH_EX;
STRUCT!{struct ATA_PASS_THROUGH_DIRECT {
    Length: USHORT,
    AtaFlags: USHORT,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    ReservedAsUchar: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    ReservedAsUlong: ULONG,
    DataBuffer: PVOID,
    PreviousTaskFile: [UCHAR; 8],
    CurrentTaskFile: [UCHAR; 8],
}}
pub type PATA_PASS_THROUGH_DIRECT = *mut ATA_PASS_THROUGH_DIRECT;
#[cfg(target_arch = "x86_64")]
IFDEF!{
STRUCT!{struct ATA_PASS_THROUGH_EX32 {
    Length: USHORT,
    AtaFlags: USHORT,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    ReservedAsUchar: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    ReservedAsUlong: ULONG,
    DataBufferOffset: ULONG32,
    PreviousTaskFile: [UCHAR; 8],
    CurrentTaskFile: [UCHAR; 8],
}}
pub type PATA_PASS_THROUGH_EX32 = *mut ATA_PASS_THROUGH_EX32;
STRUCT!{struct ATA_PASS_THROUGH_DIRECT32 {
    Length: USHORT,
    AtaFlags: USHORT,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    ReservedAsUchar: UCHAR,
    DataTransferLength: ULONG,
    TimeOutValue: ULONG,
    ReservedAsUlong: ULONG,
    DataBuffer: ULONG32,
    PreviousTaskFile: [UCHAR; 8],
    CurrentTaskFile: [UCHAR; 8],
}}
pub type PATA_PASS_THROUGH_DIRECT32 = *mut ATA_PASS_THROUGH_DIRECT32;
}
pub const ATA_FLAGS_DRDY_REQUIRED: USHORT = 1 << 0;
pub const ATA_FLAGS_DATA_IN: USHORT = 1 << 1;
pub const ATA_FLAGS_DATA_OUT: USHORT = 1 << 2;
pub const ATA_FLAGS_48BIT_COMMAND: USHORT = 1 << 3;
pub const ATA_FLAGS_USE_DMA: USHORT = 1 << 4;
pub const ATA_FLAGS_NO_MULTIPLE: USHORT = 1 << 5;
STRUCT!{struct IDE_IO_CONTROL {
    HeaderLength: ULONG,
    Signature: [UCHAR; 8],
    Timeout: ULONG,
    ControlCode: ULONG,
    ReturnStatus: ULONG,
    DataLength: ULONG,
}}
pub type PIDE_IO_CONTROL = *mut IDE_IO_CONTROL;
STRUCT!{struct MPIO_PASS_THROUGH_PATH {
    PassThrough: SCSI_PASS_THROUGH,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH = *mut MPIO_PASS_THROUGH_PATH;
STRUCT!{struct MPIO_PASS_THROUGH_PATH_DIRECT {
    PassThrough: SCSI_PASS_THROUGH_DIRECT,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH_DIRECT = *mut MPIO_PASS_THROUGH_PATH_DIRECT;
STRUCT!{struct MPIO_PASS_THROUGH_PATH_EX {
    PassThroughOffset: ULONG,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH_EX = *mut MPIO_PASS_THROUGH_PATH_EX;
STRUCT!{struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    PassThroughOffset: ULONG,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH_DIRECT_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT_EX;
#[cfg(target_arch = "x86_64")]
IFDEF!{
STRUCT!{struct MPIO_PASS_THROUGH_PATH32 {
    PassThrough: SCSI_PASS_THROUGH32,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH32 = *mut MPIO_PASS_THROUGH_PATH32;
STRUCT!{struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH_DIRECT32 = *mut MPIO_PASS_THROUGH_PATH_DIRECT32;
STRUCT!{struct MPIO_PASS_THROUGH_PATH32_EX {
    PassThroughOffset: ULONG,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH32_EX = *mut MPIO_PASS_THROUGH_PATH32_EX;
STRUCT!{struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    PassThroughOffset: ULONG,
    Version: ULONG,
    Length: USHORT,
    Flags: UCHAR,
    PortNumber: UCHAR,
    MpioPathId: ULONGLONG,
}}
pub type PMPIO_PASS_THROUGH_PATH_DIRECT32_EX = *mut MPIO_PASS_THROUGH_PATH_DIRECT32_EX;
}
STRUCT!{struct SCSI_BUS_DATA {
    NumberOfLogicalUnits: UCHAR,
    InitiatorBusId: UCHAR,
    InquiryDataOffset: ULONG,
}}
pub type PSCSI_BUS_DATA = *mut SCSI_BUS_DATA;
STRUCT!{struct SCSI_ADAPTER_BUS_INFO {
    NumberOfBuses: UCHAR,
    BusData: [SCSI_BUS_DATA; 1],
}}
pub type PSCSI_ADAPTER_BUS_INFO = *mut SCSI_ADAPTER_BUS_INFO;
STRUCT!{struct SCSI_INQUIRY_DATA {
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
    DeviceClaimed: BOOLEAN,
    InquiryDataLength: ULONG,
    NextInquiryDataOffset: ULONG,
    InquiryData: [UCHAR; 1],
}}
pub type PSCSI_INQUIRY_DATA = *mut SCSI_INQUIRY_DATA;
pub const IOCTL_MINIPORT_SIGNATURE_SCSIDISK: &'static str = "SCSIDISK";
pub const IOCTL_MINIPORT_SIGNATURE_HYBRDISK: &'static str = "HYBRDISK";
pub const IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION: &'static str = "MPDSM   ";
pub const IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL: &'static str = "MPDSMGEN";
pub const IOCTL_MINIPORT_SIGNATURE_FIRMWARE: &'static str = "FIRMWARE";
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL: &'static str = "PROTOCOL";
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE: &'static str = "TEMPERAT";
pub const IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD: &'static str = "SETTEMPT";
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY: &'static str = "TOPOLOGY";
STRUCT!{struct SRB_IO_CONTROL {
    HeaderLength: ULONG,
    Signature: [UCHAR; 8],
    Timeout: ULONG,
    ControlCode: ULONG,
    ReturnCode: ULONG,
    Length: ULONG,
}}
pub type PSRB_IO_CONTROL = *mut SRB_IO_CONTROL;
STRUCT!{struct NVCACHE_REQUEST_BLOCK {
    NRBSize: ULONG,
    Function: USHORT,
    NRBFlags: ULONG,
    NRBStatus: ULONG,
    Count: ULONG,
    LBA: ULONGLONG,
    DataBufSize: ULONG,
    NVCacheStatus: ULONG,
    NVCacheSubStatus: ULONG,
}}
pub type PNVCACHE_REQUEST_BLOCK = *mut NVCACHE_REQUEST_BLOCK;
pub const NRB_FUNCTION_NVCACHE_INFO: USHORT = 0xEC;
pub const NRB_FUNCTION_SPINDLE_STATUS: USHORT = 0xE5;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: USHORT = 0x00;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: USHORT = 0x01;
pub const NRB_FUNCTION_FLUSH_NVCACHE: USHORT = 0x14;
pub const NRB_FUNCTION_QUERY_PINNED_SET: USHORT = 0x12;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: USHORT = 0x13;
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: USHORT = 0x10;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: USHORT = 0x11;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: USHORT = 0xD0;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: USHORT = 0xD1;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: USHORT = 0xE0;
pub const NRB_FUNCTION_NVSEPARATED_INFO: USHORT = 0xc0;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: USHORT = 0xc1;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: USHORT = 0xc2;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: USHORT = 0xc3;
pub const NRB_SUCCESS: ULONG = 0;
pub const NRB_ILLEGAL_REQUEST: ULONG = 1;
pub const NRB_INVALID_PARAMETER: ULONG = 2;
pub const NRB_INPUT_DATA_OVERRUN: ULONG = 3;
pub const NRB_INPUT_DATA_UNDERRUN: ULONG = 4;
pub const NRB_OUTPUT_DATA_OVERRUN: ULONG = 5;
pub const NRB_OUTPUT_DATA_UNDERRUN: ULONG = 6;
STRUCT!{struct NV_FEATURE_PARAMETER {
    NVPowerModeEnabled: USHORT,
    NVParameterReserv1: USHORT,
    NVCmdEnabled: USHORT,
    NVParameterReserv2: USHORT,
    NVPowerModeVer: USHORT,
    NVCmdVer: USHORT,
    NVSize: ULONG,
    NVReadSpeed: USHORT,
    NVWrtSpeed: USHORT,
    DeviceSpinUpTime: ULONG,
}}
pub type PNV_FEATURE_PARAMETER = *mut NV_FEATURE_PARAMETER;
STRUCT!{struct NVCACHE_HINT_PAYLOAD {
    Command: UCHAR,
    Feature7_0: UCHAR,
    Feature15_8: UCHAR,
    Count15_8: UCHAR,
    LBA7_0: UCHAR,
    LBA15_8: UCHAR,
    LBA23_16: UCHAR,
    LBA31_24: UCHAR,
    LBA39_32: UCHAR,
    LBA47_40: UCHAR,
    Auxiliary7_0: UCHAR,
    Auxiliary23_16: UCHAR,
    Reserved: [UCHAR; 4],
}}
pub type PNVCACHE_HINT_PAYLOAD = *mut NVCACHE_HINT_PAYLOAD;
STRUCT!{struct NV_SEP_CACHE_PARAMETER {
    Version: ULONG,
    Size: ULONG,
    Flags: NV_SEP_CACHE_PARAMETER_Flags,
    WriteCacheType: UCHAR,
    WriteCacheTypeEffective: UCHAR,
    ParameterReserve1: [UCHAR; 3],
}}
pub type PNV_SEP_CACHE_PARAMETER = *mut NV_SEP_CACHE_PARAMETER;
UNION!{union NV_SEP_CACHE_PARAMETER_Flags {
    [u8; 1],
    CacheFlags CacheFlags_mut: NV_SEP_CACHE_PARAMETER_Flags_CacheFlags,
    CacheFlagsSet CacheFlagsSet_mut: UCHAR,
}}
STRUCT!{struct NV_SEP_CACHE_PARAMETER_Flags_CacheFlags {
    Bitfield: UCHAR,
}}
BITFIELD!{NV_SEP_CACHE_PARAMETER_Flags_CacheFlags Bitfield: UCHAR [
    WriteCacheEnabled set_WriteCacheEnabled[0..1],
    WriteCacheChangeable set_WriteCacheChangeable[1..2],
    WriteThroughIOSupported set_WriteThroughIOSupported[2..3],
    FlushCacheSupported set_FlushCacheSupported[3..4],
    ReservedBits set_ReservedBits[4..8],
]}
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: ULONG = 1;
pub const NV_SEP_CACHE_PARAMETER_VERSION: ULONG = NV_SEP_CACHE_PARAMETER_VERSION_1;
ENUM!{enum NV_SEP_WRITE_CACHE_TYPE {
    NVSEPWriteCacheTypeUnknown = 0,
    NVSEPWriteCacheTypeNone = 1,
    NVSEPWriteCacheTypeWriteBack = 2,
    NVSEPWriteCacheTypeWriteThrough = 3,
}}
pub type PNV_SEP_WRITE_CACHE_TYPE = *mut NV_SEP_WRITE_CACHE_TYPE;
STRUCT!{struct MP_DEVICE_DATA_SET_RANGE {
    StartingOffset: LONGLONG,
    LengthInBytes: ULONGLONG,
}}
pub type PMP_DEVICE_DATA_SET_RANGE = *mut MP_DEVICE_DATA_SET_RANGE;
STRUCT!{struct DSM_NOTIFICATION_REQUEST_BLOCK {
    Size: ULONG,
    Version: ULONG,
    NotifyFlags: ULONG,
    DataSetProfile: ULONG,
    Reserved: [ULONG; 3],
    DataSetRangesCount: ULONG,
    DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; ANYSIZE_ARRAY],
}}
pub type PDSM_NOTIFICATION_REQUEST_BLOCK = *mut DSM_NOTIFICATION_REQUEST_BLOCK;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: ULONG = 1;
pub const MINIPORT_DSM_NOTIFICATION_VERSION: ULONG = MINIPORT_DSM_NOTIFICATION_VERSION_1;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: ULONG = 0;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: ULONG = 1;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: ULONG = 2;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: ULONG = 3;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: ULONG = 0x00000001;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: ULONG = 0x00000002;
pub const HYBRID_FUNCTION_GET_INFO: ULONG = 0x01;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: ULONG = 0x10;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: ULONG = 0x11;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: ULONG = 0x12;
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: ULONG = 0x13;
pub const HYBRID_STATUS_SUCCESS: ULONG = 0x0;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: ULONG = 0x1;
pub const HYBRID_STATUS_INVALID_PARAMETER: ULONG = 0x2;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: ULONG = 0x3;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: ULONG = 0x10;
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: ULONG = 0x1;
STRUCT!{struct HYBRID_REQUEST_BLOCK {
    Version: ULONG,
    Size: ULONG,
    Function: ULONG,
    Flags: ULONG,
    DataBufferOffset: ULONG,
    DataBufferLength: ULONG,
}}
pub type PHYBRID_REQUEST_BLOCK = *mut HYBRID_REQUEST_BLOCK;
ENUM!{enum NVCACHE_TYPE {
    NvCacheTypeUnknown = 0,
    NvCacheTypeNone = 1,
    NvCacheTypeWriteBack = 2,
    NvCacheTypeWriteThrough = 3,
}}
ENUM!{enum NVCACHE_STATUS {
    NvCacheStatusUnknown = 0,
    NvCacheStatusDisabling = 1,
    NvCacheStatusDisabled = 2,
    NvCacheStatusEnabled = 3,
}}
STRUCT!{struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    PriorityLevel: UCHAR,
    Reserved0: [UCHAR; 3],
    ConsumedNVMSizeFraction: ULONG,
    ConsumedMappingResourcesFraction: ULONG,
    ConsumedNVMSizeForDirtyDataFraction: ULONG,
    ConsumedMappingResourcesForDirtyDataFraction: ULONG,
    Reserved1: ULONG,
}}
pub type PNVCACHE_PRIORITY_LEVEL_DESCRIPTOR = *mut NVCACHE_PRIORITY_LEVEL_DESCRIPTOR;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: ULONG = 1;
STRUCT!{struct HYBRID_INFORMATION {
    Version: ULONG,
    Size: ULONG,
    HybridSupported: BOOLEAN,
    Status: NVCACHE_STATUS,
    CacheTypeEffective: NVCACHE_TYPE,
    CacheTypeDefault: NVCACHE_TYPE,
    FractionBase: ULONG,
    CacheSize: ULONGLONG,
    Attributes: HYBRID_INFORMATION_Attributes,
    Priorities: HYBRID_INFORMATION_Priorities,
}}
pub type PHYBRID_INFORMATION = *mut HYBRID_INFORMATION;
STRUCT!{struct HYBRID_INFORMATION_Attributes {
    Bitfield: ULONG,
}}
BITFIELD!{HYBRID_INFORMATION_Attributes Bitfield: ULONG [
    WriteCacheChangeable set_WriteCacheChangeable[0..1],
    WriteThroughIoSupported set_WriteThroughIoSupported[1..2],
    FlushCacheSupported set_FlushCacheSupported[2..3],
    Removable set_Removable[3..4],
    ReservedBits set_ReservedBits[4..32],
]}
STRUCT!{struct HYBRID_INFORMATION_Priorities {
    PriorityLevelCount: UCHAR,
    MaxPriorityBehavior: BOOLEAN,
    OptimalWriteGranularity: UCHAR,
    Reserved: UCHAR,
    DirtyThresholdLow: ULONG,
    DirtyThresholdHigh: ULONG,
    SupportedCommands: HYBRID_INFORMATION_Priorities_SupportedCommands,
    Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 0],
}}
STRUCT!{struct HYBRID_INFORMATION_Priorities_SupportedCommands {
    Bitfield: ULONG,
    MaxEvictCommands: ULONG,
    MaxLbaRangeCountForEvict: ULONG,
    MaxLbaRangeCountForChangeLba: ULONG,
}}
BITFIELD!{HYBRID_INFORMATION_Priorities_SupportedCommands Bitfield: ULONG [
    CacheDisable set_CacheDisable[0..1],
    SetDirtyThreshold set_SetDirtyThreshold[1..2],
    PriorityDemoteBySize set_PriorityDemoteBySize[2..3],
    PriorityChangeByLbaRange set_PriorityChangeByLbaRange[3..4],
    Evict set_Evict[4..5],
    ReservedBits set_ReservedBits[5..32],
]}
STRUCT!{struct HYBRID_DIRTY_THRESHOLDS {
    Version: ULONG,
    Size: ULONG,
    DirtyLowThreshold: ULONG,
    DirtyHighThreshold: ULONG,
}}
pub type PHYBRID_DIRTY_THRESHOLDS = *mut HYBRID_DIRTY_THRESHOLDS;
STRUCT!{struct HYBRID_DEMOTE_BY_SIZE {
    Version: ULONG,
    Size: ULONG,
    SourcePriority: UCHAR,
    TargetPriority: UCHAR,
    Reserved0: USHORT,
    Reserved1: ULONG,
    LbaCount: ULONGLONG,
}}
pub type PHYBRID_DEMOTE_BY_SIZE = *mut HYBRID_DEMOTE_BY_SIZE;
pub const FIRMWARE_FUNCTION_GET_INFO: ULONG = 0x01;
pub const FIRMWARE_FUNCTION_DOWNLOAD: ULONG = 0x02;
pub const FIRMWARE_FUNCTION_ACTIVATE: ULONG = 0x03;
pub const FIRMWARE_STATUS_SUCCESS: ULONG = 0x0;
pub const FIRMWARE_STATUS_ERROR: ULONG = 0x1;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: ULONG = 0x2;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: ULONG = 0x3;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: ULONG = 0x4;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: ULONG = 0x5;
pub const FIRMWARE_STATUS_INVALID_SLOT: ULONG = 0x6;
pub const FIRMWARE_STATUS_INVALID_IMAGE: ULONG = 0x7;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: ULONG = 0x10;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: ULONG = 0x20;
pub const FIRMWARE_STATUS_DEVICE_ERROR: ULONG = 0x40;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: ULONG = 0x80;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: ULONG = 0x81;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: ULONG = 0x82;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: ULONG = 0x83;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: ULONG = 0x84;
pub const FIRMWARE_STATUS_COMMAND_ABORT: ULONG = 0x85;
pub const FIRMWARE_STATUS_END_OF_MEDIA: ULONG = 0x86;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: ULONG = 0x87;
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: ULONG = 0x1;
STRUCT!{struct FIRMWARE_REQUEST_BLOCK {
    Version: ULONG,
    Size: ULONG,
    Function: ULONG,
    Flags: ULONG,
    DataBufferOffset: ULONG,
    DataBufferLength: ULONG,
}}
pub type PFIRMWARE_REQUEST_BLOCK = *mut FIRMWARE_REQUEST_BLOCK;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: ULONG = 0x00000001;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: ULONG = 0x00000002;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: ULONG = 0x80000000;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: ULONG = 0x1;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: ULONG = 0x2;
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: UCHAR = 0xFF;
STRUCT!{struct STORAGE_FIRMWARE_SLOT_INFO {
    SlotNumber: UCHAR,
    ReadOnly: BOOLEAN,
    Reserved: [UCHAR; 6],
    Revision: STORAGE_FIRMWARE_SLOT_INFO_Revision,
}}
pub type PSTORAGE_FIRMWARE_SLOT_INFO = *mut STORAGE_FIRMWARE_SLOT_INFO;
UNION!{union STORAGE_FIRMWARE_SLOT_INFO_Revision {
    [u64; 1],
    Info Info_mut: [UCHAR; 8],
    AsUlonglong AsUlonglong_mut: ULONGLONG,
}}
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: usize = 16;
STRUCT!{struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    SlotNumber: UCHAR,
    ReadOnly: BOOLEAN,
    Reserved: [UCHAR; 6],
    Revision: [UCHAR; STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH],
}}
pub type PSTORAGE_FIRMWARE_SLOT_INFO_V2 = *mut STORAGE_FIRMWARE_SLOT_INFO_V2;
STRUCT!{struct STORAGE_FIRMWARE_INFO {
    Version: ULONG,
    Size: ULONG,
    UpgradeSupport: BOOLEAN,
    SlotCount: UCHAR,
    ActiveSlot: UCHAR,
    PendingActivateSlot: UCHAR,
    Reserved: ULONG,
    Slot: [STORAGE_FIRMWARE_SLOT_INFO; 0],
}}
pub type PSTORAGE_FIRMWARE_INFO = *mut STORAGE_FIRMWARE_INFO;
STRUCT!{struct STORAGE_FIRMWARE_INFO_V2 {
    Version: ULONG,
    Size: ULONG,
    UpgradeSupport: BOOLEAN,
    SlotCount: UCHAR,
    ActiveSlot: UCHAR,
    PendingActivateSlot: UCHAR,
    FirmwareShared: BOOLEAN,
    Reserved: [UCHAR; 3],
    ImagePayloadAlignment: ULONG,
    ImagePayloadMaxSize: ULONG,
    Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 0],
}}
pub type PSTORAGE_FIRMWARE_INFO_V2 = *mut STORAGE_FIRMWARE_INFO_V2;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: ULONG = 0x1;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: ULONG = 0x2;
STRUCT!{struct STORAGE_FIRMWARE_DOWNLOAD {
    Version: ULONG,
    Size: ULONG,
    Offset: ULONGLONG,
    BufferSize: ULONGLONG,
    ImageBuffer: [UCHAR; 0],
}}
pub type PSTORAGE_FIRMWARE_DOWNLOAD = *mut STORAGE_FIRMWARE_DOWNLOAD;
STRUCT!{struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    Version: ULONG,
    Size: ULONG,
    Offset: ULONGLONG,
    BufferSize: ULONGLONG,
    Slot: UCHAR,
    Reserved: [UCHAR; 7],
    ImageBuffer: [UCHAR; 0],
}}
pub type PSTORAGE_FIRMWARE_DOWNLOAD_V2 = *mut STORAGE_FIRMWARE_DOWNLOAD_V2;
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: ULONG = 0x1;
STRUCT!{struct STORAGE_FIRMWARE_ACTIVATE {
    Version: ULONG,
    Size: ULONG,
    SlotToActivate: UCHAR,
    Reserved0: [UCHAR; 3],
}}
pub type PSTORAGE_FIRMWARE_ACTIVATE = *mut STORAGE_FIRMWARE_ACTIVATE;
STRUCT!{struct IO_SCSI_CAPABILITIES {
    Length: ULONG,
    MaximumTransferLength: ULONG,
    MaximumPhysicalPages: ULONG,
    SupportedAsynchronousEvents: ULONG,
    AlignmentMask: ULONG,
    TaggedQueuing: BOOLEAN,
    AdapterScansDown: BOOLEAN,
    AdapterUsesPio: BOOLEAN,
}}
pub type PIO_SCSI_CAPABILITIES = *mut IO_SCSI_CAPABILITIES;
STRUCT!{struct SCSI_ADDRESS {
    Length: ULONG,
    PortNumber: UCHAR,
    PathId: UCHAR,
    TargetId: UCHAR,
    Lun: UCHAR,
}}
pub type PSCSI_ADDRESS = *mut SCSI_ADDRESS;
pub const DUMP_POINTERS_VERSION_1: ULONG = 1;
pub const DUMP_POINTERS_VERSION_2: ULONG = 2;
pub const DUMP_POINTERS_VERSION_3: ULONG = 3;
pub const DUMP_POINTERS_VERSION_4: ULONG = 4;
pub const DUMP_DRIVER_NAME_LENGTH: usize = 15;
FN!{cdecl DUMP_DEVICE_POWERON_ROUTINE(
    Context: PVOID,
) -> LONG}
pub type PDUMP_DEVICE_POWERON_ROUTINE = *mut DUMP_DEVICE_POWERON_ROUTINE;
STRUCT!{struct DUMP_POINTERS_VERSION {
    Version: ULONG,
    Size: ULONG,
}}
pub type PDUMP_POINTERS_VERSION = *mut DUMP_POINTERS_VERSION;
STRUCT!{struct DUMP_POINTERS {
    AdapterObject: PVOID, // struct _ADAPTER_OBJECT *
    MappedRegisterBase: PVOID,
    DumpData: PVOID,
    CommonBufferVa: PVOID,
    CommonBufferPa: LARGE_INTEGER,
    CommonBufferSize: ULONG,
    AllocateCommonBuffers: BOOLEAN,
    UseDiskDump: BOOLEAN,
    Spare1: [UCHAR; 2],
    DeviceObject: PVOID,
}}
pub type PDUMP_POINTERS = *mut DUMP_POINTERS;
STRUCT!{struct DUMP_POINTERS_EX {
    Header: DUMP_POINTERS_VERSION,
    DumpData: PVOID,
    CommonBufferVa: PVOID,
    CommonBufferSize: ULONG,
    AllocateCommonBuffers: BOOLEAN,
    DeviceObject: PVOID,
    DriverList: PVOID,
    dwPortFlags: ULONG,
    MaxDeviceDumpSectionSize: ULONG,
    MaxDeviceDumpLevel: ULONG,
    MaxTransferSize: ULONG,
    AdapterObject: PVOID,
    MappedRegisterBase: PVOID,
    DeviceReady: PBOOLEAN,
    DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    DumpDevicePowerOnContext: PVOID,
}}
pub type PDUMP_POINTERS_EX = *mut DUMP_POINTERS_EX;
// TODO: Revisit these definitions when const size_of and offset_of! arrive.
#[cfg(target_pointer_width = "32")]
IFDEF!{
pub const DUMP_POINTERS_EX_V2_SIZE: ULONG = 32;
pub const DUMP_POINTERS_EX_V3_SIZE: ULONG = 60;
pub const DUMP_POINTERS_EX_V4_SIZE: ULONG = 68;
}
#[cfg(target_pointer_width = "64")]
IFDEF!{
pub const DUMP_POINTERS_EX_V2_SIZE: ULONG = 48;
pub const DUMP_POINTERS_EX_V3_SIZE: ULONG = 88;
pub const DUMP_POINTERS_EX_V4_SIZE: ULONG = 104;
}
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: ULONG = 0x00000001;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: ULONG = 0x00000002;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: ULONG = 0x00000004;
STRUCT!{struct DUMP_DRIVER {
    DumpDriverList: PVOID,
    DriverName: [WCHAR; DUMP_DRIVER_NAME_LENGTH],
    BaseName: [WCHAR; DUMP_DRIVER_NAME_LENGTH],
}}
pub type PDUMP_DRIVER = *mut DUMP_DRIVER;
pub const SCSI_IOCTL_DATA_OUT: UCHAR = 0;
pub const SCSI_IOCTL_DATA_IN: UCHAR = 1;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: UCHAR = 2;
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: UCHAR = 3;
pub const MPIO_IOCTL_FLAG_USE_PATHID: UCHAR = 1;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: UCHAR = 2;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: UCHAR = 4;
