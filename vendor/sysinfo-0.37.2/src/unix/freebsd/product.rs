// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::utils::get_kenv_var;

pub(crate) struct ProductInner;

impl ProductInner {
    pub(crate) fn family() -> Option<String> {
        get_kenv_var(b"smbios.system.family\0")
    }

    pub(crate) fn name() -> Option<String> {
        get_kenv_var(b"smbios.system.product\0")
    }

    pub(crate) fn serial_number() -> Option<String> {
        get_kenv_var(b"smbios.system.serial\0")
    }

    pub(crate) fn stock_keeping_unit() -> Option<String> {
        get_kenv_var(b"smbios.system.sku\0")
    }

    pub(crate) fn uuid() -> Option<String> {
        get_kenv_var(b"smbios.system.uuid\0")
    }

    pub(crate) fn version() -> Option<String> {
        get_kenv_var(b"smbios.system.version\0")
    }

    pub(crate) fn vendor_name() -> Option<String> {
        get_kenv_var(b"smbios.system.maker\0")
    }
}
