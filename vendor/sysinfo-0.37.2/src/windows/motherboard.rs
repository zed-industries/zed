// Take a look at the license at the top of the repository in the LICENSE file.

use super::ffi::SMBIOSBaseboardInformation;
use super::utils::{get_smbios_table, parse_smbios};

pub(crate) struct MotherboardInner {
    smbios_table: Vec<u8>,
}

impl MotherboardInner {
    pub(crate) fn new() -> Option<Self> {
        Some(Self {
            smbios_table: get_smbios_table()?,
        })
    }

    pub(crate) fn asset_tag(&self) -> Option<String> {
        let (info, strings) = parse_smbios::<SMBIOSBaseboardInformation>(&self.smbios_table, 2)?;
        if info.asset_tag == 0 {
            return None;
        }
        strings
            .get(info.asset_tag as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn name(&self) -> Option<String> {
        let (info, strings) = parse_smbios::<SMBIOSBaseboardInformation>(&self.smbios_table, 2)?;
        if info.product_name == 0 {
            return None;
        }
        strings
            .get(info.product_name as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn vendor_name(&self) -> Option<String> {
        let (info, strings) = parse_smbios::<SMBIOSBaseboardInformation>(&self.smbios_table, 2)?;
        if info.manufacturer == 0 {
            return None;
        }
        strings
            .get(info.manufacturer as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn version(&self) -> Option<String> {
        let (info, strings) = parse_smbios::<SMBIOSBaseboardInformation>(&self.smbios_table, 2)?;
        if info.version == 0 {
            return None;
        }
        strings
            .get(info.version as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn serial_number(&self) -> Option<String> {
        let (info, strings) = parse_smbios::<SMBIOSBaseboardInformation>(&self.smbios_table, 2)?;
        if info.serial_number == 0 {
            return None;
        }
        strings
            .get(info.serial_number as usize - 1)
            .copied()
            .map(str::to_string)
    }
}
