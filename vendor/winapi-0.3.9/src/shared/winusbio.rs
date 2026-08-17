// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public header for WINUSB
use shared::minwindef::{UCHAR, ULONG, USHORT};
use shared::usb::USBD_PIPE_TYPE;
pub const SHORT_PACKET_TERMINATE: ULONG = 0x01;
pub const AUTO_CLEAR_STALL: ULONG = 0x02;
pub const PIPE_TRANSFER_TIMEOUT: ULONG = 0x03;
pub const IGNORE_SHORT_PACKETS: ULONG = 0x04;
pub const ALLOW_PARTIAL_READS: ULONG = 0x05;
pub const AUTO_FLUSH: ULONG = 0x06;
pub const RAW_IO: ULONG = 0x07;
pub const MAXIMUM_TRANSFER_SIZE: ULONG = 0x08;
pub const RESET_PIPE_ON_RESUME: ULONG = 0x09;
pub const DEVICE_SPEED: ULONG = 0x01;
pub const LowSpeed: ULONG = 0x01;
pub const FullSpeed: ULONG = 0x02;
pub const HighSpeed: ULONG = 0x03;
DEFINE_GUID!{WinUSB_TestGuid,
    0xda812bff, 0x12c3, 0x46a2, 0x8e, 0x2b, 0xdb, 0xd3, 0xb7, 0x83, 0x4c, 0x43}
STRUCT!{struct WINUSB_PIPE_INFORMATION {
    PipeType: USBD_PIPE_TYPE,
    PipeId: UCHAR,
    MaximumPacketSize: USHORT,
    Interval: UCHAR,
}}
pub type PWINUSB_PIPE_INFORMATION = *mut WINUSB_PIPE_INFORMATION;
STRUCT!{struct WINUSB_PIPE_INFORMATION_EX {
    PipeType: USBD_PIPE_TYPE,
    PipeId: UCHAR,
    MaximumPacketSize: USHORT,
    Interval: UCHAR,
    MaximumBytesPerInterval: ULONG,
}}
pub type PWINUSB_PIPE_INFORMATION_EX = *mut WINUSB_PIPE_INFORMATION_EX;
