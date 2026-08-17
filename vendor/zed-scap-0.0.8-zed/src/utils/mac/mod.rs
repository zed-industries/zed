use core_graphics_helmer_fork::access::ScreenCaptureAccess;
use sysinfo::System;

pub fn has_permission() -> bool {
    ScreenCaptureAccess.preflight()
}

pub fn request_permission() -> bool {
    ScreenCaptureAccess.request()
}

pub fn is_supported() -> bool {
    let os_version = System::os_version()
        .expect("Failed to get macOS version")
        .as_bytes()
        .to_vec();

    let min_version: Vec<u8> = "12.3\n".as_bytes().to_vec();

    os_version >= min_version
}
