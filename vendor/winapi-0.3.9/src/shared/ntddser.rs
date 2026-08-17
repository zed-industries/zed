// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This is the include file that defines all constants and types for accessing the Serial device.
use shared::devpropdef::DEVPROPKEY;
DEFINE_GUID!{GUID_DEVINTERFACE_COMPORT,
    0x86E0D1E0, 0x8089, 0x11D0, 0x9C, 0xE4, 0x08, 0x00, 0x3E, 0x30, 0x1F, 0x73}
DEFINE_GUID!{GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR,
    0x4D36E978, 0xE325, 0x11CE, 0xBF, 0xC1, 0x08, 0x00, 0x2B, 0xE1, 0x03, 0x18}
DEFINE_DEVPROPKEY!{DEVPKEY_DeviceInterface_Serial_UsbVendorId,
    0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 2}
DEFINE_DEVPROPKEY!{DEVPKEY_DeviceInterface_Serial_UsbProductId,
    0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 3}
DEFINE_DEVPROPKEY!{DEVPKEY_DeviceInterface_Serial_PortName,
    0x4C6BF15C, 0x4C03, 0x4AAC, 0x91, 0xF5, 0x64, 0xC0, 0xF8, 0x52, 0xBC, 0xF4, 4}
