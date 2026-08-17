// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::bthdef::{
    BTH_ADDR, MAX_L2CAP_INFO_DATA_LENGTH, MAX_L2CAP_PING_DATA_LENGTH, MAX_UUIDS_IN_QUERY,
};
use shared::bthsdpdef::{SdpAttributeRange, SdpQueryUuid};
use shared::guiddef::GUID;
use shared::minwindef::{DWORD, PULONG, UCHAR, ULONG, USHORT};
use shared::ws2def::IOC_VENDOR;
use um::winnt::HANDLE;
pub const BT_PORT_ANY: ULONG = -1i32 as ULONG;
pub const BT_PORT_MIN: ULONG = 0x1;
pub const BT_PORT_MAX: ULONG = 0xffff;
pub const BT_PORT_DYN_FIRST: ULONG = 0x1001;
pub const AF_BTH: USHORT = 32;
pub const PH_BTH: USHORT = AF_BTH;
pub const NS_BTH: USHORT = 16;
STRUCT!{#[repr(packed)] struct SOCKADDR_BTH {
    addressFamily: USHORT,
    btAddr: BTH_ADDR,
    serviceClassId: GUID,
    port: ULONG,
}}
pub type PSOCKADDR_BTH = *mut SOCKADDR_BTH;
DEFINE_GUID!{SVCID_BTH_PROVIDER,
    0x6aa63e0, 0x7d60, 0x41ff, 0xaf, 0xb2, 0x3e, 0xe6, 0xd2, 0xd9, 0x39, 0x2d}
pub const BTH_ADDR_STRING_SIZE: DWORD = 12;
pub const BTHPROTO_RFCOMM: USHORT = 0x0003;
pub const BTHPROTO_L2CAP: USHORT = 0x0100;
pub const SOL_RFCOMM: USHORT = BTHPROTO_RFCOMM;
pub const SOL_L2CAP: USHORT = BTHPROTO_L2CAP;
pub const SOL_SDP: USHORT = 0x0101;
pub const SO_BTH_AUTHENTICATE: ULONG = 0x80000001;
pub const SO_BTH_ENCRYPT: ULONG = 0x00000002;
pub const SO_BTH_MTU: ULONG = 0x80000007;
pub const SO_BTH_MTU_MAX: ULONG = 0x80000008;
pub const SO_BTH_MTU_MIN: ULONG = 0x8000000a;
pub const RFCOMM_MAX_MTU: ULONG = 0x000003F3;
pub const RFCOMM_MIN_MTU: ULONG = 0x00000017;
pub const BTH_SDP_VERSION: ULONG = 1;
STRUCT!{#[repr(packed)] struct BTH_SET_SERVICE {
    pSdpVersion: PULONG,
    pRecordHandle: HANDLE,
    fCodService: ULONG,
    Reserved: [ULONG; 5],
    ulRecordLength: ULONG,
    pRecord: [UCHAR; 1],
}}
pub type PBTH_SET_SERVICE = *mut BTH_SET_SERVICE;
pub const SDP_DEFAULT_INQUIRY_SECONDS: UCHAR = 6;
pub const SDP_MAX_INQUIRY_SECONDS: UCHAR = 60;
pub const SDP_DEFAULT_INQUIRY_MAX_RESPONSES: UCHAR = 255;
pub const SDP_SERVICE_SEARCH_REQUEST: ULONG = 1;
pub const SDP_SERVICE_ATTRIBUTE_REQUEST: ULONG = 2;
pub const SDP_SERVICE_SEARCH_ATTRIBUTE_REQUEST: ULONG = 3;
STRUCT!{#[repr(packed)] struct BTH_QUERY_DEVICE {
    LAP: ULONG,
    length: UCHAR,
}}
pub type PBTH_QUERY_DEVICE = *mut BTH_QUERY_DEVICE;
STRUCT!{#[repr(packed)] struct BTH_QUERY_SERVICE {
    type_: ULONG,
    serviceHandle: ULONG,
    uuids: [SdpQueryUuid; MAX_UUIDS_IN_QUERY],
    numRange: ULONG,
    pRange: [SdpAttributeRange; 1],
}}
pub type PBTH_QUERY_SERVICE = *mut BTH_QUERY_SERVICE;
pub const BTHNS_RESULT_DEVICE_CONNECTED: DWORD = 0x00010000;
pub const BTHNS_RESULT_DEVICE_REMEMBERED: DWORD = 0x00020000;
pub const BTHNS_RESULT_DEVICE_AUTHENTICATED: DWORD = 0x00040000;
pub const SIO_RFCOMM_SEND_COMMAND: DWORD = _WSAIORW!(IOC_VENDOR, 101);
pub const SIO_RFCOMM_WAIT_COMMAND: DWORD = _WSAIORW!(IOC_VENDOR, 102);
pub const SIO_BTH_PING: DWORD = _WSAIORW!(IOC_VENDOR, 8);
pub const SIO_BTH_INFO: DWORD = _WSAIORW!(IOC_VENDOR, 9);
pub const SIO_RFCOMM_SESSION_FLOW_OFF: DWORD = _WSAIORW!(IOC_VENDOR, 103);
pub const SIO_RFCOMM_TEST: DWORD = _WSAIORW!(IOC_VENDOR, 104);
pub const SIO_RFCOMM_USECFC: DWORD = _WSAIORW!(IOC_VENDOR, 105);
macro_rules! BIT {
    ($b:expr) => {
        1 << $b
    };
}
STRUCT!{#[repr(packed)] struct RFCOMM_MSC_DATA {
    Signals: UCHAR,
    Break: UCHAR,
}}
pub type PRFCOMM_MSC_DATA = *mut RFCOMM_MSC_DATA;
pub const MSC_EA_BIT: UCHAR = BIT!(0);
pub const MSC_FC_BIT: UCHAR = BIT!(1);
pub const MSC_RTC_BIT: UCHAR = BIT!(2);
pub const MSC_RTR_BIT: UCHAR = BIT!(3);
pub const MSC_RESERVED: UCHAR = BIT!(4) | BIT!(5);
pub const MSC_IC_BIT: UCHAR = BIT!(6);
pub const MSC_DV_BIT: UCHAR = BIT!(7);
pub const MSC_BREAK_BIT: UCHAR = BIT!(1);
macro_rules! MSC_SET_BREAK_LENGTH {
    ($b: expr, $l: expr) => {
        ($b & 0x3) | (($l & 0xf) << 4)
    };
}
STRUCT!{#[repr(packed)] struct RFCOMM_RLS_DATA {
    LineStatus: UCHAR,
}}
pub type PRFCOMM_RLS_DATA = *mut RFCOMM_RLS_DATA;
pub const RLS_ERROR: UCHAR = 0x01;
pub const RLS_OVERRUN: UCHAR = 0x02;
pub const RLS_PARITY: UCHAR = 0x04;
pub const RLS_FRAMING: UCHAR = 0x08;
STRUCT!{#[repr(packed)] struct RFCOMM_RPN_DATA {
    Baud: UCHAR,
    Data: UCHAR,
    FlowControl: UCHAR,
    XonChar: UCHAR,
    XoffChar: UCHAR,
    ParameterMask1: UCHAR,
    ParameterMask2: UCHAR,
}}
pub type PRFCOMM_RPN_DATA = *mut RFCOMM_RPN_DATA;
pub const RPN_BAUD_2400: UCHAR = 0;
pub const RPN_BAUD_4800: UCHAR = 1;
pub const RPN_BAUD_7200: UCHAR = 2;
pub const RPN_BAUD_9600: UCHAR = 3;
pub const RPN_BAUD_19200: UCHAR = 4;
pub const RPN_BAUD_38400: UCHAR = 5;
pub const RPN_BAUD_57600: UCHAR = 6;
pub const RPN_BAUD_115200: UCHAR = 7;
pub const RPN_BAUD_230400: UCHAR = 8;
pub const RPN_DATA_5: UCHAR = 0x0;
pub const RPN_DATA_6: UCHAR = 0x1;
pub const RPN_DATA_7: UCHAR = 0x2;
pub const RPN_DATA_8: UCHAR = 0x3;
pub const RPN_STOP_1: UCHAR = 0x0;
pub const RPN_STOP_1_5: UCHAR = 0x4;
pub const RPN_PARITY_NONE: UCHAR = 0x00;
pub const RPN_PARITY_ODD: UCHAR = 0x08;
pub const RPN_PARITY_EVEN: UCHAR = 0x18;
pub const RPN_PARITY_MARK: UCHAR = 0x28;
pub const RPN_PARITY_SPACE: UCHAR = 0x38;
pub const RPN_FLOW_X_IN: UCHAR = 0x01;
pub const RPN_FLOW_X_OUT: UCHAR = 0x02;
pub const RPN_FLOW_RTR_IN: UCHAR = 0x04;
pub const RPN_FLOW_RTR_OUT: UCHAR = 0x08;
pub const RPN_FLOW_RTC_IN: UCHAR = 0x10;
pub const RPN_FLOW_RTC_OUT: UCHAR = 0x20;
pub const RPN_PARAM_BAUD: UCHAR = 0x01;
pub const RPN_PARAM_DATA: UCHAR = 0x02;
pub const RPN_PARAM_STOP: UCHAR = 0x04;
pub const RPN_PARAM_PARITY: UCHAR = 0x08;
pub const RPN_PARAM_P_TYPE: UCHAR = 0x10;
pub const RPN_PARAM_XON: UCHAR = 0x20;
pub const RPN_PARAM_XOFF: UCHAR = 0x40;
pub const RPN_PARAM_X_IN: UCHAR = 0x01;
pub const RPN_PARAM_X_OUT: UCHAR = 0x02;
pub const RPN_PARAM_RTR_IN: UCHAR = 0x04;
pub const RPN_PARAM_RTR_OUT: UCHAR = 0x08;
pub const RPN_PARAM_RTC_IN: UCHAR = 0x10;
pub const RPN_PARAM_RTC_OUT: UCHAR = 0x20;
pub const RFCOMM_CMD_NONE: UCHAR = 0;
pub const RFCOMM_CMD_MSC: UCHAR = 1;
pub const RFCOMM_CMD_RLS: UCHAR = 2;
pub const RFCOMM_CMD_RPN: UCHAR = 3;
pub const RFCOMM_CMD_RPN_REQUEST: UCHAR = 4;
pub const RFCOMM_CMD_RPN_RESPONSE: UCHAR = 5;
UNION!{#[repr(packed)] union RFCOMM_COMMAND_Data {
    [u8; 7],
    MSC MSC_mut: RFCOMM_MSC_DATA,
    RLS RLS_mut: RFCOMM_RLS_DATA,
    RPN RPN_mut: RFCOMM_RPN_DATA,
}}
STRUCT!{#[repr(packed)] struct RFCOMM_COMMAND {
    CmdType: ULONG,
    Data: RFCOMM_COMMAND_Data,
}}
pub type PRFCOMM_COMMAND = *mut RFCOMM_COMMAND;
STRUCT!{#[repr(packed)] struct BTH_PING_REQ {
    btAddr: BTH_ADDR,
    dataLen: UCHAR,
    data: [UCHAR; MAX_L2CAP_PING_DATA_LENGTH],
}}
pub type PBTH_PING_REQ = *mut BTH_PING_REQ;
STRUCT!{#[repr(packed)] struct BTH_PING_RSP {
    dataLen: UCHAR,
    data: [UCHAR; MAX_L2CAP_PING_DATA_LENGTH],
}}
pub type PBTH_PING_RSP = *mut BTH_PING_RSP;
STRUCT!{#[repr(packed)] struct BTH_INFO_REQ {
    btAddr: BTH_ADDR,
    infoType: USHORT,
}}
pub type PBTH_INFO_REQ = *mut BTH_INFO_REQ;
UNION!{#[repr(packed)] union BTH_INFO_RSP_u {
    [u8; MAX_L2CAP_INFO_DATA_LENGTH],
    connectionlessMTU connectionlessMTU_mut: USHORT,
    data data_mut: [UCHAR; MAX_L2CAP_INFO_DATA_LENGTH],
}}
STRUCT!{#[repr(packed)] struct BTH_INFO_RSP {
    result: USHORT,
    dataLen: UCHAR,
    u: BTH_INFO_RSP_u,
}}
pub type PBTH_INFO_RSP = *mut BTH_INFO_RSP;
pub type BTHNS_SETBLOB = BTH_SET_SERVICE;
pub type PBTHNS_SETBLOB = PBTH_SET_SERVICE;
pub type BTHNS_INQUIRYBLOB = BTH_QUERY_DEVICE;
pub type PBTHNS_INQUIRYBLOB = PBTH_QUERY_DEVICE;
pub type BTHNS_RESTRICTIONBLOB = BTH_QUERY_SERVICE;
pub type PBTHNS_RESTRICTIONBLOB = PBTH_QUERY_SERVICE;
