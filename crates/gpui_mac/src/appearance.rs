use std::ffi::CStr;

use cocoa::{
    appkit::{NSAppearanceNameVibrantDark, NSAppearanceNameVibrantLight},
    base::id,
    foundation::NSString,
};
use gpui::platform::Appearance;
use objc::{msg_send, sel, sel_impl};

pub trait AppearanceFromNative {
    unsafe fn from_native(appearance: id) -> Self;
}
impl AppearanceFromNative for Appearance {
    unsafe fn from_native(appearance: id) -> Appearance {
        let name: id = msg_send![appearance, name];
        if name == NSAppearanceNameVibrantLight {
            Appearance::VibrantLight
        } else if name == NSAppearanceNameVibrantDark {
            Appearance::VibrantDark
        } else if name == NSAppearanceNameAqua {
            Appearance::Light
        } else if name == NSAppearanceNameDarkAqua {
            Appearance::Dark
        } else {
            println!(
                "unknown appearance: {:?}",
                CStr::from_ptr(name.UTF8String())
            );
            Appearance::Light
        }
    }
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSAppearanceNameAqua: id;
    pub static NSAppearanceNameDarkAqua: id;
}
