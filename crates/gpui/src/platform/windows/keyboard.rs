use anyhow::Result;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetKeyboardLayoutNameW, MAPVK_VK_TO_CHAR, MapVirtualKeyW, ToUnicode, VIRTUAL_KEY,
        VK_ABNT_C1, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7,
        VK_OEM_102, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_SHIFT,
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
) -> Option<(String, bool)> {
    if need_to_convert_to_shifted_key(vkey) && modifiers.shift {
        get_shifted_key(vkey, scan_code).map(|key| {
            modifiers.shift = false;
            (key, true)
        })
    } else {
        get_key_from_vkey(vkey)
    }
}

fn get_key_from_vkey(vkey: VIRTUAL_KEY) -> Option<(String, bool)> {
    let key_data = unsafe { MapVirtualKeyW(vkey.0 as u32, MAPVK_VK_TO_CHAR) };
    if key_data == 0 {
        return None;
    }

    // The high word contains dead key flag, the low word contains the character
    let is_dead_key = (key_data >> 16) > 0;
    let key = char::from_u32(key_data & 0xFFFF)?;

    Some((key.to_ascii_lowercase().to_string(), is_dead_key))
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
            | VK_ABNT_C1
    )
}

fn get_shifted_key(vkey: VIRTUAL_KEY, scan_code: u32) -> Option<String> {
    let mut state = [0; 256];
    state[VK_SHIFT.0 as usize] = 0x80;

    let mut buffer = [0; 4];
    let len = unsafe { ToUnicode(vkey.0 as u32, scan_code, Some(&state), &mut buffer, 0) };

    if len > 0 {
        let candidate = String::from_utf16_lossy(&buffer[..len as usize]);
        if !candidate.is_empty() && !candidate.chars().next().unwrap().is_control() {
            return Some(candidate);
        }
    }
    None
}
