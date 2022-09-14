use std::ffi::CStr;

use cocoa::{
    appkit::{NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight},
    base::id,
    foundation::NSString,
};
use objc::{msg_send, sel, sel_impl};

use crate::Appearance;

impl Appearance {
    pub unsafe fn from_native(appearance: id) -> Self {
        let name: id = msg_send![appearance, name];
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

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSAppearanceNameAqua: id;
    pub static NSAppearanceNameDarkAqua: id;
}
