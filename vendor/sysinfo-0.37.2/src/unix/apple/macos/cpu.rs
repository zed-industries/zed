// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "apple-sandbox")]
pub(crate) unsafe fn get_cpu_frequency(_brand: &str) -> u64 {
    0
}

#[cfg(not(feature = "apple-sandbox"))]
pub(crate) unsafe fn get_cpu_frequency(brand: &str) -> u64 {
    use crate::sys::macos::utils::IOReleaser;
    use objc2_core_foundation::{
        CFData, CFDictionary, CFRange, CFRetained, CFString, kCFAllocatorDefault,
    };
    use objc2_io_kit::{
        IOIteratorNext, IORegistryEntryCreateCFProperty, IORegistryEntryGetName,
        IOServiceGetMatchingServices, IOServiceMatching, io_iterator_t, kIOMasterPortDefault,
        kIOReturnSuccess,
    };

    unsafe {
        let Some(matching) = IOServiceMatching(c"AppleARMIODevice".as_ptr().cast()) else {
            sysinfo_debug!("IOServiceMatching call failed, `AppleARMIODevice` not found");
            return 0;
        };
        let matching = CFRetained::<CFDictionary>::from(&matching);

        // Starting from mac M1, the above call returns nothing for the CPU frequency
        // so we try to get it from another source. This code comes from
        // <https://github.com/giampaolo/psutil/pull/2222>.
        let mut iterator: io_iterator_t = 0;
        let result =
            IOServiceGetMatchingServices(kIOMasterPortDefault, Some(matching), &mut iterator);
        if result != kIOReturnSuccess {
            sysinfo_debug!("Error: IOServiceGetMatchingServices() = {}", result);
            return 0;
        }
        let iterator = match IOReleaser::new(iterator) {
            Some(i) => i,
            None => {
                sysinfo_debug!(
                    "Error: IOServiceGetMatchingServices() succeeded but returned invalid descriptor"
                );
                return 0;
            }
        };

        let mut name = [0; 128];
        let entry = loop {
            let entry = match IOReleaser::new(IOIteratorNext(iterator.inner())) {
                Some(d) => d,
                None => {
                    sysinfo_debug!("`pmgr` entry was not found in AppleARMIODevice service");
                    return 0;
                }
            };
            let status = IORegistryEntryGetName(entry.inner(), &mut name);
            if status != libc::KERN_SUCCESS {
                continue;
            } else if libc::strcmp(name.as_ptr(), c"pmgr".as_ptr() as *const _) == 0 {
                break entry;
            }
        };

        let node_name = CFString::from_static_str("voltage-states5-sram");

        let core_ref = match IORegistryEntryCreateCFProperty(
            entry.inner(),
            Some(&node_name),
            kCFAllocatorDefault,
            0,
        ) {
            Some(c) => c,
            None => {
                sysinfo_debug!("`voltage-states5-sram` property not found");
                return 0;
            }
        };

        let Ok(core_ref) = core_ref.downcast::<CFData>() else {
            sysinfo_debug!("`voltage-states5-sram` property was not CFData");
            return 0;
        };

        let core_length = core_ref.length();
        if core_length < 8 {
            sysinfo_debug!("expected `voltage-states5-sram` buffer to have at least size 8");
            return 0;
        }
        let mut max: u64 = 0;
        core_ref.bytes(
            CFRange {
                location: core_length - 8,
                length: 4,
            },
            &mut max as *mut _ as *mut _,
        );

        // Check taken from https://github.com/vladkens/macmon/commit/9e05a6f6e9aee01c4cd6e01e0639ac23f5820f18.
        // Not sure if there is a better way to differentiate this.
        if brand.contains("M1") || brand.contains("M2") | brand.contains("M3") {
            max / 1_000_000
        } else {
            max / 1_000
        }
    }
}
