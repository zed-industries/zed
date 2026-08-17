// Take a look at the license at the top of the repository in the LICENSE file.

pub(crate) struct ProductInner;

impl ProductInner {
    pub(crate) fn family() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_family")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn name() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .ok()
            .or_else(|| {
                std::fs::read_to_string("/sys/firmware/devicetree/base/model")
                    .ok()
                    .or_else(|| {
                        std::fs::read_to_string("/sys/firmware/devicetree/base/banner-name").ok()
                    })
                    .or_else(|| std::fs::read_to_string("/tmp/sysinfo/model").ok())
                    .map(|s| s.trim_end_matches('\0').to_owned())
            })
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn serial_number() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_serial")
            .ok()
            .or_else(|| {
                std::fs::read_to_string("/sys/firmware/devicetree/base/serial-number")
                    .ok()
                    .map(|s| s.trim_end_matches('\0').to_owned())
            })
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn stock_keeping_unit() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_sku")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn uuid() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_uuid")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn version() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
            .ok()
            .map(|s| s.trim().to_owned())
    }

    pub(crate) fn vendor_name() -> Option<String> {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor")
            .ok()
            .map(|s| s.trim().to_owned())
    }
}
