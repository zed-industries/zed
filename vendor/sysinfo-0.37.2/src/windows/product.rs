// Take a look at the license at the top of the repository in the LICENSE file.

use super::ffi::SMBIOSSystemInformation;
use super::utils::{get_smbios_table, parse_smbios};

pub(crate) struct ProductInner;

impl ProductInner {
    pub(crate) fn family() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.family == 0 {
            return None;
        }
        strings
            .get(info.family as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn name() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.product_name == 0 {
            return None;
        }
        strings
            .get(info.product_name as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn serial_number() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.serial_number == 0 {
            return None;
        }
        strings
            .get(info.serial_number as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn stock_keeping_unit() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.sku_number == 0 {
            return None;
        }
        strings
            .get(info.sku_number as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn uuid() -> Option<String> {
        let table = get_smbios_table()?;
        Some(
            parse_smbios::<SMBIOSSystemInformation>(&table, 1)?
                .0
                .uuid
                .to_string(),
        )
    }

    pub(crate) fn version() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.version == 0 {
            return None;
        }
        strings
            .get(info.version as usize - 1)
            .copied()
            .map(str::to_string)
    }

    pub(crate) fn vendor_name() -> Option<String> {
        let table = get_smbios_table()?;
        let (info, strings) = parse_smbios::<SMBIOSSystemInformation>(&table, 1)?;
        if info.manufacturer == 0 {
            return None;
        }
        strings
            .get(info.manufacturer as usize - 1)
            .copied()
            .map(str::to_string)
    }
}
