use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_TCFType,
    string::{CFString, CFStringRef},
};

use crate::keyboard_layouts::KeyboardLayout;

pub(crate) fn retrieve_current_keboard_layout() -> KeyboardLayout {
    let Some(name) = get_current_layout() else {
        log::error!("Cannot retrieve current keyboard layout");
        return KeyboardLayout::ABC;
    };
    let name = name.trim_start_matches("com.apple.keylayout.");
    match name {
        "ABC" => KeyboardLayout::ABC,
        "Czech-QWERTY" | "Czech" => KeyboardLayout::CzechQwerty,
        "German" | "German-DIN-2137" => KeyboardLayout::German,
        "Russian" | "RussianWin" => KeyboardLayout::Russian,
        _ => {
            log::error!("Unsupported keyboard layout found: {}", name);
            KeyboardLayout::ABC
        }
    }
}

fn get_current_layout() -> Option<String> {
    unsafe {
        let keyboard =
            TISInputSource::wrap_under_create_rule(TISCopyCurrentKeyboardLayoutInputSource());
        let layout_name_data =
            TISGetInputSourceProperty(keyboard.as_concrete_TypeRef(), kTISPropertyInputSourceID);
        if layout_name_data.is_null() {
            return None;
        }
        Some(CFString::wrap_under_get_rule(layout_name_data).to_string())
    }
}

#[repr(C)]
pub struct __TISInputSource {
    _private: i32,
}

pub type TISInputSourceRef = *const __TISInputSource;

declare_TCFType!(TISInputSource, TISInputSourceRef);
impl_TCFType!(TISInputSource, TISInputSourceRef, TISInputSourceGetTypeID);

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyInputSourceID: CFStringRef;

    fn TISInputSourceGetTypeID() -> CFTypeID;
    fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, key: CFStringRef) -> CFStringRef;
}
