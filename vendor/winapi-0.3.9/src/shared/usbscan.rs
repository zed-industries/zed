// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::ntdef::{ULONG, USHORT};
use um::winioctl::{FILE_ANY_ACCESS, METHOD_BUFFERED};
//98
STRUCT!{struct DEVICE_DESCRIPTOR {
    usVendorId: USHORT,
    usProductId: USHORT,
    usBcdDevice: USHORT,
    usLanguageId: USHORT,
}}
pub type PDEVICE_DESCRIPTOR = *mut DEVICE_DESCRIPTOR;
//132
pub const FILE_DEVICE_USB_SCAN: ULONG = 0x8000;
pub const IOCTL_INDEX: ULONG = 0x0800;
//143
pub const IOCTL_GET_USB_DESCRIPTOR: ULONG
    = CTL_CODE!(FILE_DEVICE_USB_SCAN, IOCTL_INDEX + 8, METHOD_BUFFERED, FILE_ANY_ACCESS);
