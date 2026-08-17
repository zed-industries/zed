#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "windows")]
mod win;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux;

/// Checks if process has permission to capture the screen
pub fn has_permission() -> bool {
    #[cfg(target_os = "macos")]
    return mac::has_permission();

    #[cfg(target_os = "windows")]
    return true;

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    return true;
}

/// Prompts user to grant screen capturing permission to current process
pub fn request_permission() -> bool {
    #[cfg(target_os = "macos")]
    return mac::request_permission();

    // assume windows to be true
    #[cfg(target_os = "windows")]
    return true;

    // TODO: check if linux has permission system
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    return true;
}

/// Checks if scap is supported on the current system
pub fn is_supported() -> bool {
    #[cfg(target_os = "macos")]
    return mac::is_supported();

    #[cfg(target_os = "windows")]
    return win::is_supported();

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    return linux::is_supported();
}
