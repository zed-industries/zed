use objc::{class, msg_send, sel, sel_impl};

use crate::{PermissionKind, PermissionStatus, PlatformPermissionsHandler};

pub struct MacPermissionsHandler;

impl PlatformPermissionsHandler for MacPermissionsHandler {
    fn status(&self, kind: PermissionKind) -> PermissionStatus {
        match kind {
            PermissionKind::ScreenCapture => Self::screen_capture_status(),
            PermissionKind::Microphone => Self::microphone_status(),
        }
    }

    fn request(&self, kind: PermissionKind) {
        match kind {
            PermissionKind::ScreenCapture => Self::request_screen_capture(),
            PermissionKind::Microphone => Self::request_microphone(),
        }
    }

    fn open_settings(&self, _kind: PermissionKind) {}
}

impl MacPermissionsHandler {
    fn screen_capture_status() -> PermissionStatus {
        if unsafe { CGPreflightScreenCaptureAccess() } {
            PermissionStatus::Granted
        } else {
            // CGPreflightScreenCaptureAccess returns false both when denied
            // and when not yet determined - we cannot distinguish between them
            PermissionStatus::NotDetermined
        }
    }

    fn request_screen_capture() {
        unsafe {
            CGRequestScreenCaptureAccess();
        }
    }

    fn microphone_status() -> PermissionStatus {
        unsafe {
            let status: isize = msg_send![class!(AVCaptureDevice), authorizationStatusForMediaType: AVMediaTypeAudio];
            match status {
                0 => PermissionStatus::NotDetermined,
                1 => PermissionStatus::Restricted,
                2 => PermissionStatus::Denied,
                3 => PermissionStatus::Granted,
                _ => PermissionStatus::Unsupported,
            }
        }
    }

    fn request_microphone() {
        todo!()
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVMediaTypeAudio: *const objc::runtime::Object;
}
