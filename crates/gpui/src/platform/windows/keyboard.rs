use anyhow::Result;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        GetKeyboardLayoutNameW, MAPVK_VK_TO_CHAR, MAPVK_VK_TO_VSC, MapVirtualKeyW, ToUnicode,
        VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_ABNT_C1,
        VK_CONTROL, VK_MENU, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7,
        VK_OEM_8, VK_OEM_102, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_SHIFT,
    },
    WindowsAndMessaging::KL_NAMELENGTH,
};
use windows_core::HSTRING;

use crate::{
    KeybindingKeystroke, Keystroke, Modifiers, PlatformKeyboardLayout, PlatformKeyboardMapper,
};

pub(crate) struct WindowsKeyboardLayout {
    id: String,
    name: String,
}

pub(crate) struct WindowsKeyboardMapper;

impl PlatformKeyboardLayout for WindowsKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl PlatformKeyboardMapper for WindowsKeyboardMapper {
    fn map_key_equivalent(&self, mut keystroke: Keystroke) -> KeybindingKeystroke {
        let Some((vkey, shifted_key)) = key_needs_processing(&keystroke.key) else {
            return KeybindingKeystroke::new(keystroke);
        };
        if shifted_key && keystroke.modifiers.shift {
            log::warn!(
                "Keystroke '{}' has both shift and a shifted key, this is likely a bug",
                keystroke.key
            );
        }

        let shift = shifted_key || keystroke.modifiers.shift;
        keystroke.modifiers.shift = false;

        let Some(key) = get_key_from_vkey(vkey) else {
            log::error!(
                "Failed to map key equivalent '{:?}' to a valid key",
                keystroke
            );
            return KeybindingKeystroke::new(keystroke);
        };

        keystroke.key = if shift {
            let scan_code = unsafe { MapVirtualKeyW(vkey.0 as u32, MAPVK_VK_TO_VSC) };
            if scan_code == 0 {
                log::error!(
                    "Failed to map keystroke {:?} with virtual key '{:?}' to a scan code",
                    keystroke,
                    vkey
                );
                return KeybindingKeystroke::new(keystroke);
            }
            let Some(shifted_key) = get_shifted_key(vkey, scan_code) else {
                log::error!(
                    "Failed to map keystroke {:?} with virtual key '{:?}' to a shifted key",
                    keystroke,
                    vkey
                );
                return KeybindingKeystroke::new(keystroke);
            };
            shifted_key
        } else {
            key.clone()
        };

        let modifiers = Modifiers {
            shift,
            ..keystroke.modifiers
        };

        KeybindingKeystroke {
            inner: keystroke,
            modifiers,
            key,
        }
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

    match len {
        len if len > 0 => String::from_utf16(&buffer[..len as usize])
            .ok()
            .filter(|candidate| {
                !candidate.is_empty() && !candidate.chars().next().unwrap().is_control()
            }),
        len if len < 0 => String::from_utf16(&buffer[..(-len as usize)]).ok(),
        _ => None,
    }
}

fn key_needs_processing(key: &str) -> Option<(VIRTUAL_KEY, bool)> {
    match key {
        "`" => Some((VK_OEM_3, false)),
        "~" => Some((VK_OEM_3, true)),
        "1" => Some((VK_1, false)),
        "!" => Some((VK_1, true)),
        "2" => Some((VK_2, false)),
        "@" => Some((VK_2, true)),
        "3" => Some((VK_3, false)),
        "#" => Some((VK_3, true)),
        "4" => Some((VK_4, false)),
        "$" => Some((VK_4, true)),
        "5" => Some((VK_5, false)),
        "%" => Some((VK_5, true)),
        "6" => Some((VK_6, false)),
        "^" => Some((VK_6, true)),
        "7" => Some((VK_7, false)),
        "&" => Some((VK_7, true)),
        "8" => Some((VK_8, false)),
        "*" => Some((VK_8, true)),
        "9" => Some((VK_9, false)),
        "(" => Some((VK_9, true)),
        "0" => Some((VK_0, false)),
        ")" => Some((VK_0, true)),
        "-" => Some((VK_OEM_MINUS, false)),
        "_" => Some((VK_OEM_MINUS, true)),
        "=" => Some((VK_OEM_PLUS, false)),
        "+" => Some((VK_OEM_PLUS, true)),
        "[" => Some((VK_OEM_4, false)),
        "{" => Some((VK_OEM_4, true)),
        "]" => Some((VK_OEM_6, false)),
        "}" => Some((VK_OEM_6, true)),
        "\\" => Some((VK_OEM_5, false)),
        "|" => Some((VK_OEM_5, true)),
        ";" => Some((VK_OEM_1, false)),
        ":" => Some((VK_OEM_1, true)),
        "'" => Some((VK_OEM_7, false)),
        "\"" => Some((VK_OEM_7, true)),
        "," => Some((VK_OEM_COMMA, false)),
        "<" => Some((VK_OEM_COMMA, true)),
        "." => Some((VK_OEM_PERIOD, false)),
        ">" => Some((VK_OEM_PERIOD, true)),
        "/" => Some((VK_OEM_2, false)),
        "?" => Some((VK_OEM_2, true)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Keystroke, Modifiers, PlatformKeyboardMapper, WindowsKeyboardMapper};

    #[test]
    fn test_keyboard_mapper() {
        let mapper = WindowsKeyboardMapper::new();

        // Normal case
        let keystroke = Keystroke {
            modifiers: Modifiers::control(),
            key: "a".to_string(),
            key_char: None,
        };
        let mapped = mapper.map_key_equivalent(keystroke.clone());
        assert_eq!(mapped.inner, keystroke);
        assert_eq!(mapped.key, "a");
        assert_eq!(mapped.modifiers, Modifiers::control());

        // Shifted case, ctrl-$
        let keystroke = Keystroke {
            modifiers: Modifiers::control(),
            key: "$".to_string(),
            key_char: None,
        };
        let mapped = mapper.map_key_equivalent(keystroke.clone());
        assert_eq!(mapped.inner, keystroke);
        assert_eq!(mapped.key, "4");
        assert_eq!(mapped.modifiers, Modifiers::control_shift());

        // Shifted case, but shift is true
        let keystroke = Keystroke {
            modifiers: Modifiers::control_shift(),
            key: "$".to_string(),
            key_char: None,
        };
        let mapped = mapper.map_key_equivalent(keystroke.clone());
        assert_eq!(mapped.inner.modifiers, Modifiers::control());
        assert_eq!(mapped.key, "4");
        assert_eq!(mapped.modifiers, Modifiers::control_shift());

        // Windows style
        let keystroke = Keystroke {
            modifiers: Modifiers::control_shift(),
            key: "4".to_string(),
            key_char: None,
        };
        let mapped = mapper.map_key_equivalent(keystroke.clone());
        assert_eq!(mapped.inner.modifiers, Modifiers::control());
        assert_eq!(mapped.inner.key, "$");
        assert_eq!(mapped.key, "4");
        assert_eq!(mapped.modifiers, Modifiers::control_shift());
    }
}
