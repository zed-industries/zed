// Take a look at the license at the top of the repository in the LICENSE file.

use std::fs::{read, read_to_string};

pub(crate) struct MotherboardInner;

impl MotherboardInner {
    pub(crate) fn new() -> Option<Self> {
        Some(Self)
    }

    pub(crate) fn asset_tag(&self) -> Option<String> {
        read_to_string("/sys/devices/virtual/dmi/id/board_asset_tag")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn name(&self) -> Option<String> {
        read_to_string("/sys/devices/virtual/dmi/id/board_name")
            .ok()
            .or_else(|| {
                read_to_string("/proc/device-tree/board")
                    .ok()
                    .or_else(|| Some(parse_device_tree_compatible()?.1))
            })
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn vendor_name(&self) -> Option<String> {
        read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
            .ok()
            .or_else(|| Some(parse_device_tree_compatible()?.0))
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn version(&self) -> Option<String> {
        read_to_string("/sys/devices/virtual/dmi/id/board_version")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn serial_number(&self) -> Option<String> {
        read_to_string("/sys/devices/virtual/dmi/id/board_serial")
            .ok()
            .map(|s| s.trim().to_owned())
    }
}

// Parses the first entry of the file `/proc/device-tree/compatible`, to extract the vendor and
// motherboard name. This file contains several `\0` separated strings; the first one include the
// vendor and the motherboard name, separated by a comma.
//
// According to the specification: https://github.com/devicetree-org/devicetree-specification
// a compatible string must contain only one comma.
fn parse_device_tree_compatible() -> Option<(String, String)> {
    let bytes = read("/proc/device-tree/compatible").ok()?;
    let first_line = bytes.split(|&b| b == 0).next()?;
    std::str::from_utf8(first_line)
        .ok()?
        .split_once(',')
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
}
