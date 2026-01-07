use crate::{PermissionKind, PermissionStatus, PlatformPermissionsHandler};

pub struct MacPermissionsHandler;

impl PlatformPermissionsHandler for MacPermissionsHandler {
    fn status(&self, kind: PermissionKind) -> PermissionStatus {
        match kind {
            PermissionKind::ScreenCapture => Self::screen_capture_status(),
        }
    }

    fn request(&self, kind: PermissionKind) {
        match kind {
            PermissionKind::ScreenCapture => Self::request_screen_capture(),
        }
    }

    fn open_settings(&self, _kind: PermissionKind) {}
}

impl MacPermissionsHandler {
    fn screen_capture_status() -> PermissionStatus {
        if unsafe { CGPreflightScreenCaptureAccess() } {
            PermissionStatus::Granted
        } else {
            // Returns false even if the user has never been asked
            // so we cannot distinguish between denied and not determined
            PermissionStatus::NotDetermined
        }
    }

    fn request_screen_capture() {
        unsafe {
            CGRequestScreenCaptureAccess();
        }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}
