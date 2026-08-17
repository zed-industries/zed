// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))]
use crate::sys::macos::system::get_io_platform_property;

pub(crate) struct MotherboardInner;

impl MotherboardInner {
    pub(crate) fn new() -> Option<Self> {
        Some(Self)
    }

    pub(crate) fn name(&self) -> Option<String> {
        cfg_if! {
            if #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))] {
                get_io_platform_property("board-id")
            } else {
                None
            }
        }
    }

    pub(crate) fn vendor_name(&self) -> Option<String> {
        cfg_if! {
            if #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))] {
                get_io_platform_property("manufacturer")
            } else {
                None
            }
        }
    }

    pub(crate) fn version(&self) -> Option<String> {
        cfg_if! {
            if #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))] {
                get_io_platform_property("version")
            } else {
                None
            }
        }
    }

    pub(crate) fn serial_number(&self) -> Option<String> {
        cfg_if! {
            if #[cfg(all(target_os = "macos", not(feature = "apple-sandbox")))] {
                get_io_platform_property("IOPlatformSerialNumber")
            } else {
                None
            }
        }
    }

    pub(crate) fn asset_tag(&self) -> Option<String> {
        None
    }
}
