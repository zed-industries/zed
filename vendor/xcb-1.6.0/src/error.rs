use crate::base::ResolveWireError;
use crate::ext::{Extension, ExtensionData};
use crate::x;
use crate::{ffi::*, BaseError, Raw};
use std::mem;

#[cfg(feature = "damage")]
use crate::damage;

#[cfg(feature = "glx")]
use crate::glx;

#[cfg(feature = "randr")]
use crate::randr;

#[cfg(feature = "render")]
use crate::render;

#[cfg(feature = "shm")]
use crate::shm;

#[cfg(feature = "sync")]
use crate::sync;

#[cfg(feature = "xf86vidmode")]
use crate::xf86vidmode;

#[cfg(feature = "xfixes")]
use crate::xfixes;

#[cfg(feature = "xinput")]
use crate::xinput;

#[cfg(feature = "xkb")]
use crate::xkb;

#[cfg(feature = "xprint")]
use crate::xprint;

#[cfg(feature = "xv")]
use crate::xv;

/// A protocol error issued from the X server
///
/// The second member is the name of the request that emitted the error (if any).
#[derive(Debug)]
pub enum ProtocolError {
    /// The error is from the core X protocol.
    X(x::Error, Option<&'static str>),

    #[cfg(feature = "damage")]
    /// The error is issued from the `DAMAGE` extension.
    Damage(damage::Error, Option<&'static str>),

    #[cfg(feature = "glx")]
    /// The error is issued from the `GLX` extension.
    Glx(glx::Error, Option<&'static str>),

    #[cfg(feature = "randr")]
    /// The error is issued from the `RANDR` extension.
    RandR(randr::Error, Option<&'static str>),

    #[cfg(feature = "render")]
    /// The error is issued from the `RENDER` extension.
    Render(render::Error, Option<&'static str>),

    #[cfg(feature = "shm")]
    /// The error is issued from the `MIT-SHM` extension.
    Shm(shm::Error, Option<&'static str>),

    #[cfg(feature = "sync")]
    /// The error is issued from the `SYNC` extension.
    Sync(sync::Error, Option<&'static str>),

    #[cfg(feature = "xf86vidmode")]
    /// The error is issued from the `XFree86-VidModeExtension` extension.
    Xf86VidMode(xf86vidmode::Error, Option<&'static str>),

    #[cfg(feature = "xfixes")]
    /// The error is issued from the `XFIXES` extension.
    XFixes(xfixes::Error, Option<&'static str>),

    #[cfg(feature = "xinput")]
    /// The error is issued from the `XInputExtension` extension.
    Input(xinput::Error, Option<&'static str>),

    #[cfg(feature = "xkb")]
    /// The error is issued from the `XKEYBOARD` extension.
    Xkb(xkb::Error, Option<&'static str>),

    #[cfg(feature = "xprint")]
    /// The error is issued from the `XpExtension` extension.
    XPrint(xprint::Error, Option<&'static str>),

    #[cfg(feature = "xv")]
    /// The error is issued from the `XVideo` extension.
    Xv(xv::Error, Option<&'static str>),

    /// The error is unknown.
    Unknown(UnknownError, Option<&'static str>),
}

impl ProtocolError {
    pub fn as_raw(&self) -> *mut xcb_generic_error_t {
        match self {
            ProtocolError::X(e, _) => e.as_raw(),
            #[cfg(feature = "damage")]
            ProtocolError::Damage(e, _) => e.as_raw(),
            #[cfg(feature = "glx")]
            ProtocolError::Glx(e, _) => e.as_raw(),
            #[cfg(feature = "randr")]
            ProtocolError::RandR(e, _) => e.as_raw(),
            #[cfg(feature = "render")]
            ProtocolError::Render(e, _) => e.as_raw(),
            #[cfg(feature = "shm")]
            ProtocolError::Shm(e, _) => e.as_raw(),
            #[cfg(feature = "sync")]
            ProtocolError::Sync(e, _) => e.as_raw(),
            #[cfg(feature = "xf86vidmode")]
            ProtocolError::Xf86VidMode(e, _) => e.as_raw(),
            #[cfg(feature = "xfixes")]
            ProtocolError::XFixes(e, _) => e.as_raw(),
            #[cfg(feature = "xinput")]
            ProtocolError::Input(e, _) => e.as_raw(),
            #[cfg(feature = "xkb")]
            ProtocolError::Xkb(e, _) => e.as_raw(),
            #[cfg(feature = "xprint")]
            ProtocolError::XPrint(e, _) => e.as_raw(),
            #[cfg(feature = "xv")]
            ProtocolError::Xv(e, _) => e.as_raw(),
            ProtocolError::Unknown(e, _) => e.as_raw(),
        }
    }
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ProtocolError {}

/// an error that was not recognized as part of the core protocol or any enabled extension
pub struct UnknownError {
    raw: *mut xcb_generic_error_t,
}

impl Raw<xcb_generic_error_t> for UnknownError {
    unsafe fn from_raw(raw: *mut xcb_generic_error_t) -> Self {
        UnknownError { raw }
    }

    fn as_raw(&self) -> *mut xcb_generic_error_t {
        self.raw
    }
}

impl BaseError for UnknownError {
    const EXTENSION: Option<Extension> = None;
    const NUMBER: u32 = u32::MAX;
}

impl UnknownError {
    pub fn response_type(&self) -> u8 {
        unsafe { (*self.raw).response_type }
    }
    pub fn sequence(&self) -> u16 {
        unsafe { (*self.raw).sequence }
    }
    pub fn resource_id(&self) -> u32 {
        unsafe { (*self.raw).resource_id }
    }
    pub fn minor_code(&self) -> u16 {
        unsafe { (*self.raw).minor_code }
    }
    pub fn major_code(&self) -> u8 {
        unsafe { (*self.raw).major_code }
    }
    pub fn full_sequence(&self) -> u32 {
        unsafe { (*self.raw).full_sequence }
    }
}

unsafe impl Send for UnknownError {}
unsafe impl Sync for UnknownError {}

impl std::fmt::Debug for UnknownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnknownError").finish()
    }
}

impl Drop for UnknownError {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _) }
    }
}

/// Resolve an error from the X server
///
/// If the error originates from an extension, the `extension_data` parameter must contain the
/// data for the extension of origin.
/// If the resolution fails, an `ProtocolError::Unknown` is returned.
///
/// # Safety
/// The caller must ensure that `error` is a valid pointer to an `xcb_generic_error_t`.
pub unsafe fn resolve_error(
    error: *mut xcb_generic_error_t,
    extension_data: &[ExtensionData],
) -> ProtocolError {
    debug_assert!(!error.is_null());

    let error_code = (*error).error_code;
    let major_code = (*error).major_code;
    let minor_code = (*error).minor_code;

    let (best, emitting_ext) = {
        let mut best: Option<&ExtensionData> = None;
        let mut emitting_ext = None;
        for data in extension_data {
            if data.major_opcode == major_code {
                emitting_ext = Some(data.ext);
            }
            if data.first_error > 0 && error_code >= data.first_error {
                if let Some(ed) = best {
                    if data.first_error > ed.first_error {
                        best = Some(data);
                    }
                } else {
                    best = Some(data);
                }
            }
        }
        (best, emitting_ext)
    };

    let emitted_by = if let Some(ext) = emitting_ext {
        match ext {
            Extension::BigRequests => crate::bigreq::request_name(minor_code),
            Extension::XcMisc => crate::xc_misc::request_name(minor_code),

            #[cfg(feature = "composite")]
            Extension::Composite => crate::composite::request_name(minor_code),

            #[cfg(feature = "damage")]
            Extension::Damage => crate::damage::request_name(minor_code),

            #[cfg(feature = "dpms")]
            Extension::Dpms => crate::dpms::request_name(minor_code),

            #[cfg(feature = "dri2")]
            Extension::Dri2 => crate::dri2::request_name(minor_code),

            #[cfg(feature = "dri3")]
            Extension::Dri3 => crate::dri3::request_name(minor_code),

            #[cfg(feature = "ge")]
            Extension::GenericEvent => crate::ge::request_name(minor_code),

            #[cfg(feature = "glx")]
            Extension::Glx => crate::glx::request_name(minor_code),

            #[cfg(feature = "present")]
            Extension::Present => crate::present::request_name(minor_code),

            #[cfg(feature = "randr")]
            Extension::RandR => crate::randr::request_name(minor_code),

            #[cfg(feature = "record")]
            Extension::Record => crate::record::request_name(minor_code),

            #[cfg(feature = "render")]
            Extension::Render => crate::render::request_name(minor_code),

            #[cfg(feature = "res")]
            Extension::Res => crate::res::request_name(minor_code),

            #[cfg(feature = "screensaver")]
            Extension::ScreenSaver => crate::screensaver::request_name(minor_code),

            #[cfg(feature = "shape")]
            Extension::Shape => crate::shape::request_name(minor_code),

            #[cfg(feature = "shm")]
            Extension::Shm => crate::shm::request_name(minor_code),

            #[cfg(feature = "sync")]
            Extension::Sync => crate::sync::request_name(minor_code),

            #[cfg(feature = "xevie")]
            Extension::Xevie => crate::xevie::request_name(minor_code),

            #[cfg(feature = "xf86dri")]
            Extension::Xf86Dri => crate::xf86dri::request_name(minor_code),

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => crate::xf86vidmode::request_name(minor_code),

            #[cfg(feature = "xfixes")]
            Extension::XFixes => crate::xfixes::request_name(minor_code),

            #[cfg(feature = "xinerama")]
            Extension::Xinerama => crate::xinerama::request_name(minor_code),

            #[cfg(feature = "xinput")]
            Extension::Input => crate::xinput::request_name(minor_code),

            #[cfg(feature = "xkb")]
            Extension::Xkb => crate::xkb::request_name(minor_code),

            #[cfg(feature = "xprint")]
            Extension::XPrint => crate::xprint::request_name(minor_code),

            #[cfg(feature = "xselinux")]
            Extension::SeLinux => crate::xselinux::request_name(minor_code),

            #[cfg(feature = "xtest")]
            Extension::Test => crate::xtest::request_name(minor_code),

            #[cfg(feature = "xv")]
            Extension::Xv => crate::xv::request_name(minor_code),

            #[cfg(feature = "xvmc")]
            Extension::XvMc => crate::xvmc::request_name(minor_code),
        }
    } else {
        crate::x::request_name(major_code as u16)
    };

    let resolved: Option<ProtocolError> = if let Some(ext_data) = best {
        match ext_data.ext {
            #[cfg(feature = "damage")]
            Extension::Damage => damage::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Damage(e, emitted_by)),

            #[cfg(feature = "glx")]
            Extension::Glx => glx::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Glx(e, emitted_by)),

            #[cfg(feature = "randr")]
            Extension::RandR => randr::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::RandR(e, emitted_by)),

            #[cfg(feature = "render")]
            Extension::Render => render::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Render(e, emitted_by)),

            #[cfg(feature = "shm")]
            Extension::Shm => shm::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Shm(e, emitted_by)),

            #[cfg(feature = "sync")]
            Extension::Sync => sync::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Sync(e, emitted_by)),

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => {
                xf86vidmode::Error::resolve_wire_error(ext_data.first_error, error)
                    .map(|e| ProtocolError::Xf86VidMode(e, emitted_by))
            }

            #[cfg(feature = "xfixes")]
            Extension::XFixes => xfixes::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::XFixes(e, emitted_by)),

            #[cfg(feature = "xinput")]
            Extension::Input => xinput::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Input(e, emitted_by)),

            #[cfg(feature = "xkb")]
            Extension::Xkb => xkb::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Xkb(e, emitted_by)),

            #[cfg(feature = "xprint")]
            Extension::XPrint => xprint::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::XPrint(e, emitted_by)),

            #[cfg(feature = "xv")]
            Extension::Xv => xv::Error::resolve_wire_error(ext_data.first_error, error)
                .map(|e| ProtocolError::Xv(e, emitted_by)),

            _ => None,
        }
    } else {
        x::Error::resolve_wire_error(0, error).map(|e| ProtocolError::X(e, emitted_by))
    };
    resolved.unwrap_or_else(|| ProtocolError::Unknown(UnknownError::from_raw(error), emitted_by))
}

#[cfg(all(feature = "xinput", feature = "xkb", feature = "screensaver"))]
#[test]
fn test_resolve_error() {
    // resolve a core error with core request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 2,
        sequence: 12000,
        resource_id: 12,
        minor_code: 0,
        major_code: 4,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::X(x::Error::Value(_), Some(req_name)) if req_name == "x::DestroyWindow")
    );
    mem::forget(err);

    // resolve a core error with xinput request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 2,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::X(x::Error::Value(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);

    // resolve a xinput error with xinput request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 22,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::Input(xinput::Error::Mode(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);

    // same as previous, but reverse order of extension data
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 22,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::Input(xinput::Error::Mode(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);

    // mimic a regular X error (value) with extension that do not have errors (screensaver)
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 2,
        sequence: 12000,
        resource_id: 12,
        minor_code: 0,
        major_code: 1,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [ExtensionData {
        ext: Extension::ScreenSaver,
        major_opcode: 144,
        first_event: 92,
        first_error: 0,
    }];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::X(x::Error::Value(_), Some(req_name)) if req_name == "x::CreateWindow")
    );
    mem::forget(err);
}
