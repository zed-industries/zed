// Take a look at the license at the top of the repository in the LICENSE file.

use core_foundation_sys::base::CFAllocatorRef;
#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
use core_foundation_sys::base::mach_port_t;
#[cfg(any(feature = "system", feature = "disk"))]
use core_foundation_sys::dictionary::CFDictionaryRef;
#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
use core_foundation_sys::dictionary::CFMutableDictionaryRef;
use core_foundation_sys::string::CFStringRef;

use libc::c_char;
#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
use libc::kern_return_t;

// Note: IOKit is only available on MacOS up until very recent iOS versions: https://developer.apple.com/documentation/iokit

#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
#[allow(non_camel_case_types)]
pub type io_object_t = mach_port_t;

#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
#[allow(non_camel_case_types)]
pub type io_iterator_t = io_object_t;
#[cfg(any(feature = "system", feature = "disk"))]
#[allow(non_camel_case_types)]
pub type io_registry_entry_t = io_object_t;
// This is a hack, `io_name_t` should normally be `[c_char; 128]` but Rust makes it very annoying
// to deal with that so we go around it a bit.
#[allow(non_camel_case_types, dead_code)]
pub type io_name = [c_char; 128];
#[cfg(any(feature = "system", feature = "disk"))]
#[allow(non_camel_case_types)]
pub type io_name_t = *const c_char;

#[cfg(any(feature = "system", feature = "disk"))]
pub type IOOptionBits = u32;

cfg_if! {
    if #[cfg(feature = "disk")] {
        #[allow(non_upper_case_globals)]
        pub const kIOServicePlane: &[u8] = b"IOService\0";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyDeviceCharacteristicsKey: &str = "Device Characteristics";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeKey: &str = "Medium Type";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeSolidStateKey: &str = "Solid State";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeRotationalKey: &str = "Rotational";
    }
}

// Based on https://github.com/libusb/libusb/blob/bed8d3034eac74a6e1ba123b5c270ea63cb6cf1a/libusb/os/darwin_usb.c#L54-L55,
// we can simply set it to 0 (and is the same value as its replacement `kIOMainPortDefault`).
#[allow(non_upper_case_globals)]
#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
pub const kIOMasterPortDefault: mach_port_t = 0;

// Note: Obtaining information about disks using IOKIt is allowed inside the default macOS App Sandbox.
#[cfg(any(
    feature = "system",
    feature = "disk",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    pub fn IOServiceGetMatchingServices(
        mainPort: mach_port_t,
        matching: CFMutableDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;
    #[cfg(any(
        feature = "system",
        all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
    ))]
    pub fn IOServiceMatching(a: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    pub fn IOObjectRelease(obj: io_object_t) -> kern_return_t;

    #[cfg(any(feature = "system", feature = "disk"))]
    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFDictionaryRef;
    #[cfg(feature = "disk")]
    pub fn IORegistryEntryGetParentEntry(
        entry: io_registry_entry_t,
        plane: io_name_t,
        parent: *mut io_registry_entry_t,
    ) -> kern_return_t;
    #[cfg(feature = "disk")]
    pub fn IOBSDNameMatching(
        mainPort: mach_port_t,
        options: u32,
        bsdName: *const c_char,
    ) -> CFMutableDictionaryRef;
    #[cfg(feature = "system")]
    pub fn IORegistryEntryGetName(entry: io_registry_entry_t, name: io_name_t) -> kern_return_t;
}

#[cfg(any(
    feature = "system",
    all(feature = "component", any(target_arch = "x86", target_arch = "x86_64")),
))]
pub const KIO_RETURN_SUCCESS: i32 = 0;

extern "C" {
    // FIXME: to be removed once higher version than core_foundation_sys 0.8.4 is released.
    #[allow(dead_code)]
    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        cStr: *const c_char,
        encoding: core_foundation_sys::string::CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

#[cfg(all(
    not(feature = "apple-sandbox"),
    all(
        feature = "component",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
))]
mod io_service {
    use super::{io_object_t, mach_port_t};
    use libc::{kern_return_t, size_t, task_t};

    #[allow(non_camel_case_types)]
    pub type io_connect_t = io_object_t;

    #[allow(non_camel_case_types)]
    pub type io_service_t = io_object_t;

    #[allow(non_camel_case_types)]
    pub type task_port_t = task_t;

    extern "C" {
        pub fn IOServiceOpen(
            device: io_service_t,
            owning_task: task_port_t,
            type_: u32,
            connect: *mut io_connect_t,
        ) -> kern_return_t;

        pub fn IOServiceClose(a: io_connect_t) -> kern_return_t;

        #[allow(dead_code)]
        pub fn IOConnectCallStructMethod(
            connection: mach_port_t,
            selector: u32,
            inputStruct: *const KeyData_t,
            inputStructCnt: size_t,
            outputStruct: *mut KeyData_t,
            outputStructCnt: *mut size_t,
        ) -> kern_return_t;
    }

    #[cfg_attr(feature = "debug", derive(Debug, Eq, Hash, PartialEq))]
    #[repr(C)]
    pub struct KeyData_vers_t {
        pub major: u8,
        pub minor: u8,
        pub build: u8,
        pub reserved: [u8; 1],
        pub release: u16,
    }

    #[cfg_attr(feature = "debug", derive(Debug, Eq, Hash, PartialEq))]
    #[repr(C)]
    pub struct KeyData_pLimitData_t {
        pub version: u16,
        pub length: u16,
        pub cpu_plimit: u32,
        pub gpu_plimit: u32,
        pub mem_plimit: u32,
    }

    #[cfg_attr(feature = "debug", derive(Debug, Eq, Hash, PartialEq))]
    #[repr(C)]
    pub struct KeyData_keyInfo_t {
        pub data_size: u32,
        pub data_type: u32,
        pub data_attributes: u8,
    }

    #[cfg_attr(feature = "debug", derive(Debug, Eq, Hash, PartialEq))]
    #[repr(C)]
    pub struct KeyData_t {
        pub key: u32,
        pub vers: KeyData_vers_t,
        pub p_limit_data: KeyData_pLimitData_t,
        pub key_info: KeyData_keyInfo_t,
        pub result: u8,
        pub status: u8,
        pub data8: u8,
        pub data32: u32,
        pub bytes: [i8; 32], // SMCBytes_t
    }

    #[allow(dead_code)]
    pub const KERNEL_INDEX_SMC: i32 = 2;

    #[allow(dead_code)]
    pub const SMC_CMD_READ_KEYINFO: u8 = 9;

    #[allow(dead_code)]
    pub const SMC_CMD_READ_BYTES: u8 = 5;
}

#[cfg(feature = "apple-sandbox")]
mod io_service {}

#[cfg(all(
    feature = "component",
    not(feature = "apple-sandbox"),
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub use io_service::*;

#[cfg(all(feature = "component", not(feature = "apple-sandbox"), target_arch = "aarch64"))]
mod io_service {
    use std::ptr::null;

    use super::CFStringCreateWithCStringNoCopy;
    use core_foundation_sys::array::CFArrayRef;
    use core_foundation_sys::base::{CFAllocatorRef, CFRelease};
    use core_foundation_sys::dictionary::{
        kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionaryCreate,
        CFDictionaryRef,
    };
    use core_foundation_sys::number::{kCFNumberSInt32Type, CFNumberCreate};
    use core_foundation_sys::string::CFStringRef;

    #[repr(C)]
    pub struct __IOHIDServiceClient(libc::c_void);

    pub type IOHIDServiceClientRef = *const __IOHIDServiceClient;

    #[repr(C)]
    pub struct __IOHIDEventSystemClient(libc::c_void);

    pub type IOHIDEventSystemClientRef = *const __IOHIDEventSystemClient;

    #[repr(C)]
    pub struct __IOHIDEvent(libc::c_void);

    pub type IOHIDEventRef = *const __IOHIDEvent;

    #[allow(non_upper_case_globals)]
    pub const kIOHIDEventTypeTemperature: i64 = 15;

    #[inline]
    #[allow(non_snake_case)]
    pub fn IOHIDEventFieldBase(event_type: i64) -> i64 {
        event_type << 16
    }

    #[cfg(not(feature = "apple-sandbox"))]
    extern "C" {
        pub fn IOHIDEventSystemClientCreate(allocator: CFAllocatorRef)
            -> IOHIDEventSystemClientRef;

        pub fn IOHIDEventSystemClientSetMatching(
            client: IOHIDEventSystemClientRef,
            matches: CFDictionaryRef,
        ) -> i32;

        pub fn IOHIDEventSystemClientCopyServices(client: IOHIDEventSystemClientRef) -> CFArrayRef;

        pub fn IOHIDServiceClientCopyProperty(
            service: IOHIDServiceClientRef,
            key: CFStringRef,
        ) -> CFStringRef;

        pub fn IOHIDServiceClientCopyEvent(
            service: IOHIDServiceClientRef,
            v0: i64,
            v1: i32,
            v2: i64,
        ) -> IOHIDEventRef;

        pub fn IOHIDEventGetFloatValue(event: IOHIDEventRef, field: i64) -> f64;
    }

    pub(crate) const HID_DEVICE_PROPERTY_PRODUCT: &[u8] = b"Product\0";

    pub(crate) const HID_DEVICE_PROPERTY_PRIMARY_USAGE: &[u8] = b"PrimaryUsage\0";
    pub(crate) const HID_DEVICE_PROPERTY_PRIMARY_USAGE_PAGE: &[u8] = b"PrimaryUsagePage\0";

    #[allow(non_upper_case_globals)]
    pub(crate) const kHIDPage_AppleVendor: i32 = 0xff00;

    #[allow(non_upper_case_globals)]
    pub(crate) const kHIDUsage_AppleVendor_TemperatureSensor: i32 = 0x0005;

    pub(crate) fn matching(page: i32, usage: i32) -> CFDictionaryRef {
        unsafe {
            let keys = [
                CFStringCreateWithCStringNoCopy(
                    null() as *const _,
                    HID_DEVICE_PROPERTY_PRIMARY_USAGE_PAGE.as_ptr() as *const _,
                    core_foundation_sys::string::kCFStringEncodingUTF8,
                    core_foundation_sys::base::kCFAllocatorNull as *mut _,
                ),
                CFStringCreateWithCStringNoCopy(
                    null() as *const _,
                    HID_DEVICE_PROPERTY_PRIMARY_USAGE.as_ptr() as *const _,
                    core_foundation_sys::string::kCFStringEncodingUTF8,
                    core_foundation_sys::base::kCFAllocatorNull as *mut _,
                ),
            ];

            let nums = [
                CFNumberCreate(null(), kCFNumberSInt32Type, &page as *const _ as *const _),
                CFNumberCreate(null(), kCFNumberSInt32Type, &usage as *const _ as *const _),
            ];

            let dict = CFDictionaryCreate(
                null(),
                &keys as *const _ as *const _,
                &nums as *const _ as *const _,
                2,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );

            for key in keys {
                CFRelease(key as _);
            }

            for num in nums {
                CFRelease(num as _);
            }

            dict
        }
    }
}

#[cfg(all(feature = "component", not(feature = "apple-sandbox"), target_arch = "aarch64"))]
pub use io_service::*;
