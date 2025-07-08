use anyhow::Result;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetKeyboardLayoutNameW, MAPVK_VK_TO_CHAR, MapVirtualKeyW, ToUnicode, VIRTUAL_KEY, VK_0,
        VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_ABNT_C1, VK_CONTROL, VK_MENU,
        VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_102,
        VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_SHIFT,
    },
    WindowsAndMessaging::KL_NAMELENGTH,
};
use windows_core::HSTRING;

use crate::{Modifiers, PlatformKeyboardLayout};

pub(crate) struct WindowsKeyboardLayout {
    id: String,
    name: String,
}

impl PlatformKeyboardLayout for WindowsKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl WindowsKeyboardLayout {
    pub(crate) fn new() -> Result<Self> {
        let mut buffer = [0u16; KL_NAMELENGTH as usize];
        unsafe { GetKeyboardLayoutNameW(&mut buffer)? };
        let id = HSTRING::from_wide(&buffer).to_string();
        let entry = windows_registry::LOCAL_MACHINE.open(format!(
            "System\\CurrentControlSet\\Control\\Keyboard Layouts\\{}",
            id
        ))?;
        let name = entry.get_hstring("Layout Text")?.to_string();
        Ok(Self { id, name })
    }

    pub(crate) fn unknown() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "unknown".to_string(),
        }
    }
}

pub(crate) fn get_keystroke_key(
    vkey: VIRTUAL_KEY,
    scan_code: u32,
    modifiers: &mut Modifiers,
) -> Option<String> {
    if modifiers.shift && need_to_convert_to_shifted_key(vkey) {
        get_shifted_key(vkey, scan_code).inspect(|_| {
            modifiers.shift = false;
        })
    } else {
        get_key_from_vkey(vkey)
    }
}

fn get_key_from_vkey(vkey: VIRTUAL_KEY) -> Option<String> {
    let key_data = unsafe { MapVirtualKeyW(vkey.0 as u32, MAPVK_VK_TO_CHAR) };
    if key_data == 0 {
        return None;
    }

    // The high word contains dead key flag, the low word contains the character
    let key = char::from_u32(key_data & 0xFFFF)?;

    Some(key.to_ascii_lowercase().to_string())
}

#[inline]
fn need_to_convert_to_shifted_key(vkey: VIRTUAL_KEY) -> bool {
    matches!(
        vkey,
        VK_OEM_3
            | VK_OEM_MINUS
            | VK_OEM_PLUS
            | VK_OEM_4
            | VK_OEM_5
            | VK_OEM_6
            | VK_OEM_1
            | VK_OEM_7
            | VK_OEM_COMMA
            | VK_OEM_PERIOD
            | VK_OEM_2
            | VK_OEM_102
            | VK_OEM_8
            | VK_ABNT_C1
            | VK_0
            | VK_1
            | VK_2
            | VK_3
            | VK_4
            | VK_5
            | VK_6
            | VK_7
            | VK_8
            | VK_9
    )
}

fn get_shifted_key(vkey: VIRTUAL_KEY, scan_code: u32) -> Option<String> {
    generate_key_char(vkey, scan_code, false, true, false)
}

pub(crate) fn generate_key_char(
    vkey: VIRTUAL_KEY,
    scan_code: u32,
    control: bool,
    shift: bool,
    alt: bool,
) -> Option<String> {
    let mut state = [0; 256];
    if control {
        state[VK_CONTROL.0 as usize] = 0x80;
    }
    if shift {
        state[VK_SHIFT.0 as usize] = 0x80;
    }
    if alt {
        state[VK_MENU.0 as usize] = 0x80;
    }

    let mut buffer = [0; 8];
    let len = unsafe { ToUnicode(vkey.0 as u32, scan_code, Some(&state), &mut buffer, 1 << 2) };

    if len > 0 {
        let candidate = String::from_utf16_lossy(&buffer[..len as usize]);
        if !candidate.is_empty() && !candidate.chars().next().unwrap().is_control() {
            return Some(candidate);
        }
    }
    None
}

pub(crate) fn keycode_to_key(keycode: u32) -> Option<String> {
    let c = match keycode {
        // Top row: QWERTYUIOP[]
        0x51 => 'q', // VK_Q
        0x57 => 'w', // VK_W
        0x45 => 'e', // VK_E
        0x52 => 'r', // VK_R
        0x54 => 't', // VK_T
        0x59 => 'y', // VK_Y
        0x55 => 'u', // VK_U
        0x49 => 'i', // VK_I
        0x4F => 'o', // VK_O
        0x50 => 'p', // VK_P
        0xDB => '[', // VK_OEM_4 (US: [{ )
        0xDD => ']', // VK_OEM_6 (US: ]} )

        // Home row: ASDFGHJKL;'
        0x41 => 'a',  // VK_A
        0x53 => 's',  // VK_S
        0x44 => 'd',  // VK_D
        0x46 => 'f',  // VK_F
        0x47 => 'g',  // VK_G
        0x48 => 'h',  // VK_H
        0x4A => 'j',  // VK_J
        0x4B => 'k',  // VK_K
        0x4C => 'l',  // VK_L
        0xBA => ';',  // VK_OEM_1 (US: ;: )
        0xDE => '\'', // VK_OEM_7 (US: '" )

        // Bottom row: ZXCVBNM,./
        0x5A => 'z', // VK_Z
        0x58 => 'x', // VK_X
        0x43 => 'c', // VK_C
        0x56 => 'v', // VK_V
        0x42 => 'b', // VK_B
        0x4E => 'n', // VK_N
        0x4D => 'm', // VK_M
        0xBC => ',', // VK_OEM_COMMA (US: ,< )
        0xBE => '.', // VK_OEM_PERIOD (US: .> )
        0xBF => '/', // VK_OEM_2 (US: /? )

        _ => return None,
    };
    Some(String::from(c))
}
