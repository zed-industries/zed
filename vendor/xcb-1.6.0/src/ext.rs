use crate::base::{Connection, Reply};
use crate::ffi::{
    xcb_connection_t, xcb_extension_t, xcb_get_extension_data, xcb_prefetch_extension_data,
};
use crate::x;

use std::fmt;
use std::mem;
use std::ptr;

/// Refers to a X protocol extension.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Extension {
    /// The `BIG-REQUESTS` extension.
    BigRequests,

    /// The `XCMISC` extension.
    XcMisc,

    #[cfg(feature = "composite")]
    /// The `Composite` extension.
    /// Available with the `composite` cargo feature.
    Composite,

    #[cfg(feature = "damage")]
    /// The `DAMAGE` extension.
    /// Available with the `damage` cargo feature.
    Damage,

    #[cfg(feature = "dpms")]
    /// The `DPMS` extension.
    /// Available with the `dpms` cargo feature.
    Dpms,

    #[cfg(feature = "dri2")]
    /// The `DRI2` extension.
    /// Available with the `dri2` cargo feature.
    Dri2,

    #[cfg(feature = "dri3")]
    /// The `DRI3` extension.
    /// Available with the `dri3` cargo feature.
    Dri3,

    #[cfg(feature = "ge")]
    /// The `Generic Event Extension` extension.
    /// Available with the `ge` cargo feature.
    GenericEvent,

    #[cfg(feature = "glx")]
    /// The `GLX` extension.
    /// Available with the `glx` cargo feature.
    Glx,

    #[cfg(feature = "present")]
    /// The `Present` extension.
    /// Available with the `present` cargo feature.
    Present,

    #[cfg(feature = "randr")]
    /// The `RANDR` extension.
    /// Available with the `randr` cargo feature.
    RandR,

    #[cfg(feature = "record")]
    /// The `RECORD` extension.
    /// Available with the `record` cargo feature.
    Record,

    #[cfg(feature = "render")]
    /// The `RENDER` extension.
    /// Available with the `render` cargo feature.
    Render,

    #[cfg(feature = "res")]
    /// The `X-Resource` extension.
    /// Available with the `res` cargo feature.
    Res,

    #[cfg(feature = "screensaver")]
    /// The `MIT-SCREEN-SAVER` extension.
    /// Available with the `screensaver` cargo feature.
    ScreenSaver,

    #[cfg(feature = "shape")]
    /// The `SHAPE` extension.
    /// Available with the `shape` cargo feature.
    Shape,

    #[cfg(feature = "shm")]
    /// The `MIT-SHM` extension.
    /// Available with the `shm` cargo feature.
    Shm,

    #[cfg(feature = "sync")]
    /// The `SYNC` extension.
    /// Available with the `sync` cargo feature.
    Sync,

    #[cfg(feature = "xevie")]
    /// The `XEVIE` extension.
    /// Available with the `xevie` cargo feature.
    Xevie,

    #[cfg(feature = "xf86dri")]
    /// The `XFree86-DRI` extension.
    /// Available with the `xf86dri` cargo feature.
    Xf86Dri,

    #[cfg(feature = "xf86vidmode")]
    /// The `XFree86-VidModeExtension` extension.
    /// Available with the `xf86vidmode` cargo feature.
    Xf86VidMode,

    #[cfg(feature = "xfixes")]
    /// The `XFIXES` extension.
    /// Available with the `xfixes` cargo feature.
    XFixes,

    #[cfg(feature = "xinerama")]
    /// The `XINERAMA` extension.
    /// Available with the `xinerama` cargo feature.
    Xinerama,

    #[cfg(feature = "xinput")]
    /// The `XInputExtension` extension.
    /// Available with the `xinput` cargo feature.
    Input,

    #[cfg(feature = "xkb")]
    /// The `XKEYBOARD` extension.
    /// Available with the `xkb` cargo feature.
    Xkb,

    #[cfg(feature = "xprint")]
    /// The `XpExtension` extension.
    /// Available with the `xprint` cargo feature.
    XPrint,

    #[cfg(feature = "xselinux")]
    /// The `SELinux` extension.
    /// Available with the `xselinux` cargo feature.
    SeLinux,

    #[cfg(feature = "xtest")]
    /// The `XTEST` extension.
    /// Available with the `xtest` cargo feature.
    Test,

    #[cfg(feature = "xv")]
    /// The `XVideo` extension.
    /// Available with the `xv` cargo feature.
    Xv,

    #[cfg(feature = "xvmc")]
    /// The `XVideo-MotionCompensation` extension.
    /// Available with the `xvmc` cargo feature.
    XvMc,
}

impl Extension {
    /// Returns the official X-Name of the extension,
    /// such as `"BIG-REQUESTS"`.
    fn xname(&self) -> &'static str {
        match self {
            Extension::BigRequests => crate::bigreq::XNAME,
            Extension::XcMisc => crate::xc_misc::XNAME,

            #[cfg(feature = "composite")]
            Extension::Composite => crate::composite::XNAME,

            #[cfg(feature = "damage")]
            Extension::Damage => crate::damage::XNAME,

            #[cfg(feature = "dpms")]
            Extension::Dpms => crate::dpms::XNAME,

            #[cfg(feature = "dri2")]
            Extension::Dri2 => crate::dri2::XNAME,

            #[cfg(feature = "dri3")]
            Extension::Dri3 => crate::dri3::XNAME,

            #[cfg(feature = "ge")]
            Extension::GenericEvent => crate::ge::XNAME,

            #[cfg(feature = "glx")]
            Extension::Glx => crate::glx::XNAME,

            #[cfg(feature = "present")]
            Extension::Present => crate::present::XNAME,

            #[cfg(feature = "randr")]
            Extension::RandR => crate::randr::XNAME,

            #[cfg(feature = "record")]
            Extension::Record => crate::record::XNAME,

            #[cfg(feature = "render")]
            Extension::Render => crate::render::XNAME,

            #[cfg(feature = "res")]
            Extension::Res => crate::res::XNAME,

            #[cfg(feature = "screensaver")]
            Extension::ScreenSaver => crate::screensaver::XNAME,

            #[cfg(feature = "shape")]
            Extension::Shape => crate::shape::XNAME,

            #[cfg(feature = "shm")]
            Extension::Shm => crate::shm::XNAME,

            #[cfg(feature = "sync")]
            Extension::Sync => crate::sync::XNAME,

            #[cfg(feature = "xevie")]
            Extension::Xevie => crate::xevie::XNAME,

            #[cfg(feature = "xf86dri")]
            Extension::Xf86Dri => crate::xf86dri::XNAME,

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => crate::xf86vidmode::XNAME,

            #[cfg(feature = "xfixes")]
            Extension::XFixes => crate::xfixes::XNAME,

            #[cfg(feature = "xinerama")]
            Extension::Xinerama => crate::xinerama::XNAME,

            #[cfg(feature = "xinput")]
            Extension::Input => crate::xinput::XNAME,

            #[cfg(feature = "xkb")]
            Extension::Xkb => crate::xkb::XNAME,

            #[cfg(feature = "xprint")]
            Extension::XPrint => crate::xprint::XNAME,

            #[cfg(feature = "xselinux")]
            Extension::SeLinux => crate::xselinux::XNAME,

            #[cfg(feature = "xtest")]
            Extension::Test => crate::xtest::XNAME,

            #[cfg(feature = "xv")]
            Extension::Xv => crate::xv::XNAME,

            #[cfg(feature = "xvmc")]
            Extension::XvMc => crate::xvmc::XNAME,
        }
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.xname().fmt(f)
    }
}

/// Extension data as returned by each extensions `get_extension_data`.
///
/// See [crate::bigreq::get_extension_data] as example.
#[derive(Debug)]
pub struct ExtensionData {
    pub ext: Extension,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

/// Returns the extension data for the given extensions.
/// This function may block as the data will be queried from the server
/// if not already cached.
///
/// #Panics
/// This function will panic if a mandatory extension is not present on the server.
pub fn cache_extensions_data(
    conn: *mut xcb_connection_t,
    mandatory: &[Extension],
    optional: &[Extension],
) -> Vec<ExtensionData> {
    unsafe {
        for ext in mandatory {
            let ext_id = get_extension_id(*ext);
            xcb_prefetch_extension_data(conn, ext_id);
        }
        for ext in optional {
            let ext_id = get_extension_id(*ext);
            xcb_prefetch_extension_data(conn, ext_id);
        }

        let mut ext_data = Vec::new();

        for ext in mandatory {
            let ext_id = get_extension_id(*ext);
            let raw = xcb_get_extension_data(conn, ext_id);
            let reply = x::QueryExtensionReply::from_raw(raw);

            assert!(
                reply.present(),
                "mandatory extension {} is not present on this system",
                ext
            );
            ext_data.push(ExtensionData {
                ext: *ext,
                major_opcode: reply.major_opcode(),
                first_event: reply.first_event(),
                first_error: reply.first_error(),
            });
            mem::forget(reply);
        }

        for ext in optional {
            let ext_id = get_extension_id(*ext);
            let raw = xcb_get_extension_data(conn, ext_id);
            let reply = x::QueryExtensionReply::from_raw(raw);

            if !reply.present() {
                mem::forget(reply);
                continue;
            }

            ext_data.push(ExtensionData {
                ext: *ext,
                major_opcode: reply.major_opcode(),
                first_event: reply.first_event(),
                first_error: reply.first_error(),
            });
            mem::forget(reply);
        }

        // we sort by event in reverse order to optimize the event algo
        ext_data.sort_by(|a, b| b.first_event.cmp(&a.first_event));

        ext_data
    }
}

unsafe fn get_extension_id(ext: Extension) -> *mut xcb_extension_t {
    match ext {
        Extension::BigRequests => ptr::addr_of_mut!(crate::bigreq::FFI_EXT),
        Extension::XcMisc => ptr::addr_of_mut!(crate::xc_misc::FFI_EXT),

        #[cfg(feature = "composite")]
        Extension::Composite => ptr::addr_of_mut!(crate::composite::FFI_EXT),

        #[cfg(feature = "damage")]
        Extension::Damage => ptr::addr_of_mut!(crate::damage::FFI_EXT),

        #[cfg(feature = "dpms")]
        Extension::Dpms => ptr::addr_of_mut!(crate::dpms::FFI_EXT),

        #[cfg(feature = "dri2")]
        Extension::Dri2 => ptr::addr_of_mut!(crate::dri2::FFI_EXT),

        #[cfg(feature = "dri3")]
        Extension::Dri3 => ptr::addr_of_mut!(crate::dri3::FFI_EXT),

        #[cfg(feature = "ge")]
        Extension::GenericEvent => ptr::addr_of_mut!(crate::ge::FFI_EXT),

        #[cfg(feature = "glx")]
        Extension::Glx => ptr::addr_of_mut!(crate::glx::FFI_EXT),

        #[cfg(feature = "present")]
        Extension::Present => ptr::addr_of_mut!(crate::present::FFI_EXT),

        #[cfg(feature = "randr")]
        Extension::RandR => ptr::addr_of_mut!(crate::randr::FFI_EXT),

        #[cfg(feature = "record")]
        Extension::Record => ptr::addr_of_mut!(crate::record::FFI_EXT),

        #[cfg(feature = "render")]
        Extension::Render => ptr::addr_of_mut!(crate::render::FFI_EXT),

        #[cfg(feature = "res")]
        Extension::Res => ptr::addr_of_mut!(crate::res::FFI_EXT),

        #[cfg(feature = "screensaver")]
        Extension::ScreenSaver => ptr::addr_of_mut!(crate::screensaver::FFI_EXT),

        #[cfg(feature = "shape")]
        Extension::Shape => ptr::addr_of_mut!(crate::shape::FFI_EXT),

        #[cfg(feature = "shm")]
        Extension::Shm => ptr::addr_of_mut!(crate::shm::FFI_EXT),

        #[cfg(feature = "sync")]
        Extension::Sync => ptr::addr_of_mut!(crate::sync::FFI_EXT),

        #[cfg(feature = "xevie")]
        Extension::Xevie => ptr::addr_of_mut!(crate::xevie::FFI_EXT),

        #[cfg(feature = "xf86dri")]
        Extension::Xf86Dri => ptr::addr_of_mut!(crate::xf86dri::FFI_EXT),

        #[cfg(feature = "xf86vidmode")]
        Extension::Xf86VidMode => ptr::addr_of_mut!(crate::xf86vidmode::FFI_EXT),

        #[cfg(feature = "xfixes")]
        Extension::XFixes => ptr::addr_of_mut!(crate::xfixes::FFI_EXT),

        #[cfg(feature = "xinerama")]
        Extension::Xinerama => ptr::addr_of_mut!(crate::xinerama::FFI_EXT),

        #[cfg(feature = "xinput")]
        Extension::Input => ptr::addr_of_mut!(crate::xinput::FFI_EXT),

        #[cfg(feature = "xkb")]
        Extension::Xkb => ptr::addr_of_mut!(crate::xkb::FFI_EXT),

        #[cfg(feature = "xprint")]
        Extension::XPrint => ptr::addr_of_mut!(crate::xprint::FFI_EXT),

        #[cfg(feature = "xselinux")]
        Extension::SeLinux => ptr::addr_of_mut!(crate::xselinux::FFI_EXT),

        #[cfg(feature = "xtest")]
        Extension::Test => ptr::addr_of_mut!(crate::xtest::FFI_EXT),

        #[cfg(feature = "xv")]
        Extension::Xv => ptr::addr_of_mut!(crate::xv::FFI_EXT),

        #[cfg(feature = "xvmc")]
        Extension::XvMc => ptr::addr_of_mut!(crate::xvmc::FFI_EXT),
    }
}
