use anyhow::{Context, Result};
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetKeyboardLayoutNameW, MAPVK_VK_TO_CHAR, MAPVK_VSC_TO_VK, MapVirtualKeyW, ToUnicode,
        VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_ABNT_C1,
        VK_CONTROL, VK_MENU, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7,
        VK_OEM_8, VK_OEM_102, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_SHIFT,
    },
    WindowsAndMessaging::KL_NAMELENGTH,
};
use windows_core::HSTRING;

use crate::{Modifiers, PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode};

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

pub(crate) struct WindowsKeyboardMapper;

impl PlatformKeyboardMapper for WindowsKeyboardMapper {
    fn scan_code_to_key(&self, scan_code: ScanCode, modifiers: &mut Modifiers) -> Result<String> {
        if let Some(key) = scan_code.try_to_key() {
            return Ok(key);
        }
        let (win_scan_code, vkey) = get_virtual_key_from_scan_code(scan_code)?;
        get_keystroke_key(vkey, win_scan_code, modifiers).with_context(|| {
            format!(
                "Failed to get key from scan code: {:?}, vkey: {:?}",
                scan_code, vkey
            )
        })
    }
}

impl WindowsKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
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
            | VK_OEM_6
            | VK_OEM_5
            | VK_OEM_1
            | VK_OEM_7
            | VK_OEM_COMMA
            | VK_OEM_PERIOD
            | VK_OEM_2
            | VK_OEM_102
            | VK_OEM_8 // Same as VK_OEM_2
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

fn get_virtual_key_from_scan_code(gpui_scan_code: ScanCode) -> Result<(u32, VIRTUAL_KEY)> {
    // https://github.com/microsoft/node-native-keymap/blob/main/deps/chromium/dom_code_data.inc
    let scan_code = match gpui_scan_code {
        ScanCode::A => 0x001e,
        ScanCode::B => 0x0030,
        ScanCode::C => 0x002e,
        ScanCode::D => 0x0020,
        ScanCode::E => 0x0012,
        ScanCode::F => 0x0021,
        ScanCode::G => 0x0022,
        ScanCode::H => 0x0023,
        ScanCode::I => 0x0017,
        ScanCode::J => 0x0024,
        ScanCode::K => 0x0025,
        ScanCode::L => 0x0026,
        ScanCode::M => 0x0032,
        ScanCode::N => 0x0031,
        ScanCode::O => 0x0018,
        ScanCode::P => 0x0019,
        ScanCode::Q => 0x0010,
        ScanCode::R => 0x0013,
        ScanCode::S => 0x001f,
        ScanCode::T => 0x0014,
        ScanCode::U => 0x0016,
        ScanCode::V => 0x002f,
        ScanCode::W => 0x0011,
        ScanCode::X => 0x002d,
        ScanCode::Y => 0x0015,
        ScanCode::Z => 0x002c,
        ScanCode::Digit0 => 0x000b,
        ScanCode::Digit1 => 0x0002,
        ScanCode::Digit2 => 0x0003,
        ScanCode::Digit3 => 0x0004,
        ScanCode::Digit4 => 0x0005,
        ScanCode::Digit5 => 0x0006,
        ScanCode::Digit6 => 0x0007,
        ScanCode::Digit7 => 0x0008,
        ScanCode::Digit8 => 0x0009,
        ScanCode::Digit9 => 0x000a,
        ScanCode::Backquote => 0x0029,
        ScanCode::Minus => 0x000c,
        ScanCode::Equal => 0x000d,
        ScanCode::BracketLeft => 0x001a,
        ScanCode::BracketRight => 0x001b,
        ScanCode::Backslash => 0x002b,
        ScanCode::Semicolon => 0x0027,
        ScanCode::Quote => 0x0028,
        ScanCode::Comma => 0x0033,
        ScanCode::Period => 0x0034,
        ScanCode::Slash => 0x0035,
        ScanCode::IntlBackslash => 0x0056,
        ScanCode::IntlRo => 0x0073,
        _ => anyhow::bail!("Unsupported scan code: {:?}", gpui_scan_code),
    };
    let virtual_key = unsafe { MapVirtualKeyW(scan_code, MAPVK_VSC_TO_VK) };
    if virtual_key == 0 {
        anyhow::bail!(
            "Failed to get virtual key from scan code: {:?}, {}",
            gpui_scan_code,
            scan_code
        );
    }
    Ok((scan_code, VIRTUAL_KEY(virtual_key as u16)))
}
