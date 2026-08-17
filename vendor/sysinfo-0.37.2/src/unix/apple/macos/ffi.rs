// Take a look at the license at the top of the repository in the LICENSE file.

// Note: IOKit is only available on macOS up until very recent iOS versions: https://developer.apple.com/documentation/iokit

cfg_if! {
    // TODO(madsmtm): Expose this in `objc2-io-kit`.
    if #[cfg(feature = "disk")] {
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyDeviceCharacteristicsKey: &str = "Device Characteristics";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeKey: &str = "Medium Type";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeSolidStateKey: &str = "Solid State";
        #[allow(non_upper_case_globals)]
        pub const kIOPropertyMediumTypeRotationalKey: &str = "Rotational";
        #[allow(non_upper_case_globals)]
        pub const kIOBlockStorageDriverStatisticsKey: &str = "Statistics";
        #[allow(non_upper_case_globals)]
        pub const kIOBlockStorageDriverStatisticsBytesReadKey: &str = "Bytes (Read)";
        #[allow(non_upper_case_globals)]
        pub const kIOBlockStorageDriverStatisticsBytesWrittenKey: &str = "Bytes (Write)";
    }
}

#[cfg(all(
    not(feature = "apple-sandbox"),
    all(
        feature = "component",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
))]
mod keydata {
    #[cfg_attr(feature = "debug", derive(Eq, Hash, PartialEq))]
    #[derive(Clone)]
    #[repr(C)]
    pub struct Val_t {
        pub key: [i8; 5],
        pub data_size: u32,
        pub data_type: [i8; 5], // UInt32Char_t
        pub bytes: [i8; 32],    // SMCBytes_t
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

#[cfg(all(
    not(feature = "apple-sandbox"),
    all(
        feature = "component",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
))]
pub use keydata::*;

/// Private Apple APIs.
#[cfg(all(
    feature = "component",
    not(feature = "apple-sandbox"),
    target_arch = "aarch64"
))]
mod private {
    use std::ptr::NonNull;

    use objc2_core_foundation::{CFAllocator, CFDictionary};
    use objc2_io_kit::{IOHIDEventSystemClient, IOHIDServiceClient};

    #[repr(C)]
    pub struct IOHIDEvent(libc::c_void);

    objc2_core_foundation::cf_type!(
        unsafe impl IOHIDEvent {}
    );

    #[allow(non_upper_case_globals)]
    pub const kIOHIDEventTypeTemperature: i64 = 15;

    #[inline]
    #[allow(non_snake_case)]
    pub fn IOHIDEventFieldBase(event_type: i64) -> i64 {
        event_type << 16
    }

    #[cfg(not(feature = "apple-sandbox"))]
    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        pub fn IOHIDEventSystemClientCreate(
            allocator: Option<&CFAllocator>,
        ) -> Option<NonNull<IOHIDEventSystemClient>>;

        pub fn IOHIDEventSystemClientSetMatching(
            client: &IOHIDEventSystemClient,
            matches: &CFDictionary,
        ) -> i32;

        pub fn IOHIDServiceClientCopyEvent(
            service: &IOHIDServiceClient,
            v0: i64,
            v1: i32,
            v2: i64,
        ) -> Option<NonNull<IOHIDEvent>>;

        pub fn IOHIDEventGetFloatValue(event: &IOHIDEvent, field: i64) -> f64;
    }

    pub(crate) const HID_DEVICE_PROPERTY_PRODUCT: &str = "Product";

    #[allow(non_upper_case_globals)]
    pub(crate) const kIOHIDSerialNumberKey: &str = "SerialNumber";

    pub(crate) const HID_DEVICE_PROPERTY_PRIMARY_USAGE: &str = "PrimaryUsage";
    pub(crate) const HID_DEVICE_PROPERTY_PRIMARY_USAGE_PAGE: &str = "PrimaryUsagePage";

    #[allow(non_upper_case_globals)]
    pub(crate) const kHIDPage_AppleVendor: i32 = 0xff00;

    #[allow(non_upper_case_globals)]
    pub(crate) const kHIDUsage_AppleVendor_TemperatureSensor: i32 = 0x0005;
}

#[cfg(all(
    feature = "component",
    not(feature = "apple-sandbox"),
    target_arch = "aarch64"
))]
pub use private::*;
