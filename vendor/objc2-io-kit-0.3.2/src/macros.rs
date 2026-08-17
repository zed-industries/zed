#![allow(unused_macros, unused_imports)]

macro_rules! IOUSBBit {
    ($bit:expr) => {
        1 << $bit
    };
}

macro_rules! IOUSBBitRange {
    ($start:expr, $end:expr) => {
        !((1 << $start) - 1) & ((1 << $end) | ((1 << $end) - 1))
    };
}

macro_rules! IOUSBBitRangePhase {
    ($start:expr, $end:expr) => {
        $start
    };
}

macro_rules! EncodeRequest {
    ($request:expr, $direction:expr, $type:expr, $recipient:expr) => {
        (($request << 8)
            + ($recipient + ($type << kUSBRqTypeShift) + ($direction << kUSBRqDirnShift)))
    };
}

macro_rules! err_system {
    ($x:expr) => {
        ((($x as u32) & 0x3f) as i32) << 26
    };
}

macro_rules! err_sub {
    ($x:expr) => {
        ($x & 0xfff) << 14
    };
}

macro_rules! iokit_common_msg {
    ($message:expr) => {
        // (sys_iokit | sub_iokit_common | message)
        (err_system!(0x38) | err_sub!(0) | $message) as u32
    };
}

macro_rules! iokit_family_msg {
    ($sub:expr, $message:expr) => {
        // (sys_iokit | sub| message)
        (err_system!(0x38) | ($sub as i32) | $message) as u32
    };
}

macro_rules! iokit_family_err {
    ($sub:expr, $return:expr) => {
        // (sys_iokit | sub| return)
        (err_system!(0x38) | ($sub as i32) | $return) as i32
    };
}

macro_rules! iokit_usb_msg {
    ($message:expr) => {
        // (sys_iokit | sub_iokit_usb | message)
        (err_system!(0x38) | err_sub!(1) | $message) as u32
    };
}

macro_rules! iokit_vendor_specific_msg {
    ($message:expr) => {
        // (sys_iokit | sub_iokit_vendor_specific | message)
        (err_system!(0x38) | err_sub!(-2) | $message) as u32
    };
}

macro_rules! IO_FOUR_CHAR_CODE {
    ($code:expr) => {
        $code
    };
}

pub(crate) use err_sub;
pub(crate) use err_system;
pub(crate) use iokit_common_msg;
pub(crate) use iokit_common_msg as iokit_common_err;
pub(crate) use iokit_family_err;
pub(crate) use iokit_family_msg;
pub(crate) use iokit_usb_msg as iokit_usb_err;
pub(crate) use iokit_usb_msg;
pub(crate) use iokit_vendor_specific_msg;
pub(crate) use EncodeRequest;
pub(crate) use IOUSBBit;
pub(crate) use IOUSBBit as IOUSBHostFamilyBit;
pub(crate) use IOUSBBitRange;
pub(crate) use IOUSBBitRange as USBBitRange;
pub(crate) use IOUSBBitRange as IOUSBHostFamilyBitRange;
pub(crate) use IOUSBBitRangePhase;
pub(crate) use IOUSBBitRangePhase as USBBitRangePhase;
pub(crate) use IOUSBBitRangePhase as IOUSBHostFamilyBitRangePhase;
pub(crate) use IO_FOUR_CHAR_CODE;
