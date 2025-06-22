use crate::WindowAppearance;
use cocoa::{
    appkit::{NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight},
    base::id,
    foundation::NSString,
};
use objc::{msg_send, sel, sel_impl};
use std::ffi::CStr;

impl WindowAppearance {
    pub(crate) unsafe fn from_native(appearance: id) -> Self {
        let name: id = msg_send![appearance, name];
        unsafe {
            if name == NSAppearanceNameVibrantLight {
                Self::VibrantLight
            } else if name == NSAppearanceNameVibrantDark {
                Self::VibrantDark
            } else if name == NSAppearanceNameAqua {
                Self::Light
            } else if name == NSAppearanceNameDarkAqua {
                Self::Dark
            } else {
                println!(
                    "unknown appearance: {:?}",
                    CStr::from_ptr(name.UTF8String())
                );
                Self::Light
            }
        }
    }

    pub(crate) unsafe fn into_native(self) -> id {
        unsafe {
            match self {
                Self::VibrantLight => NSAppearanceNameVibrantLight,
                Self::VibrantDark => NSAppearanceNameVibrantDark,
                Self::Light => NSAppearanceNameAqua,
                Self::Dark => NSAppearanceNameDarkAqua,
            }
        }
    }
}

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    pub static NSAppearanceNameAqua: id;
    pub static NSAppearanceNameDarkAqua: id;
}
