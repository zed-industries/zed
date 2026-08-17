// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "apple-sandbox")]
pub(crate) unsafe fn get_cpu_frequency() -> u64 {
    0
}

#[cfg(not(feature = "apple-sandbox"))]
pub(crate) unsafe fn get_cpu_frequency() -> u64 {
    use crate::sys::ffi;
    use crate::sys::macos::utils::IOReleaser;
    use crate::sys::utils::CFReleaser;

    let matching = ffi::IOServiceMatching(b"AppleARMIODevice\0".as_ptr() as *const _);
    if matching.is_null() {
        sysinfo_debug!("IOServiceMatching call failed, `AppleARMIODevice` not found");
        return 0;
    }

    // Starting from mac M1, the above call returns nothing for the CPU frequency
    // so we try to get it from another source. This code comes from
    // <https://github.com/giampaolo/psutil/pull/2222>.
    let mut iterator: ffi::io_iterator_t = 0;
    let result =
        ffi::IOServiceGetMatchingServices(ffi::kIOMasterPortDefault, matching, &mut iterator);
    if result != ffi::KIO_RETURN_SUCCESS {
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

    let mut name: ffi::io_name = std::mem::zeroed();
    let entry = loop {
        let entry = match IOReleaser::new(ffi::IOIteratorNext(iterator.inner())) {
            Some(d) => d,
            None => {
                sysinfo_debug!("`pmgr` entry was not found in AppleARMIODevice service");
                return 0;
            }
        };
        let status = ffi::IORegistryEntryGetName(entry.inner(), name.as_mut_ptr());
        if status != libc::KERN_SUCCESS {
            continue;
        } else if libc::strcmp(name.as_ptr(), b"pmgr\0".as_ptr() as *const _) == 0 {
            break entry;
        }
    };

    let node_name = match CFReleaser::new(ffi::CFStringCreateWithCStringNoCopy(
        std::ptr::null(),
        b"voltage-states5-sram\0".as_ptr() as *const _,
        core_foundation_sys::string::kCFStringEncodingUTF8,
        core_foundation_sys::base::kCFAllocatorNull as *mut _,
    )) {
        Some(n) => n,
        None => {
            sysinfo_debug!("CFStringCreateWithCStringNoCopy failed");
            return 0;
        }
    };

    let core_ref = match CFReleaser::new(ffi::IORegistryEntryCreateCFProperty(
        entry.inner(),
        node_name.inner(),
        core_foundation_sys::base::kCFAllocatorDefault,
        0,
    )) {
        Some(c) => c,
        None => {
            sysinfo_debug!("`voltage-states5-sram` property not found");
            return 0;
        }
    };

    let core_length = core_foundation_sys::data::CFDataGetLength(core_ref.inner() as *const _);
    if core_length < 8 {
        sysinfo_debug!("expected `voltage-states5-sram` buffer to have at least size 8");
        return 0;
    }
    let mut max: u64 = 0;
    core_foundation_sys::data::CFDataGetBytes(
        core_ref.inner() as *const _,
        core_foundation_sys::base::CFRange::init(core_length - 8, 4),
        &mut max as *mut _ as *mut _,
    );
    max / 1_000_000
}
