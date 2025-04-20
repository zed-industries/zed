use std::borrow::Cow;

use anyhow::{Context, Result};
use util::ResultExt;
use windows::Win32::UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::KL_NAMELENGTH};
use windows_core::HSTRING;

use crate::{KeyboardMapper, Keystroke, Modifiers, PlatformKeyboardLayout};

pub(crate) struct WindowsKeyboardMapper;

pub(crate) struct KeyboardLayout {
    id: String,
    name: String,
}

impl KeyboardMapper for WindowsKeyboardMapper {
    fn map_keystroke(&self, keystroke: Keystroke, use_key_equivalents: bool) -> Keystroke {
        let Keystroke {
            mut modifiers,
            mut key,
            key_char,
        } = keystroke;
        if use_key_equivalents {
            key = self
                .map_virtual_key(&key, &mut modifiers)
                .or_else(|_| self.map_for_char(&key, &mut modifiers))
                .context("Failed to map keystroke with use_key_equivalents = true")
                .log_err()
                .unwrap_or(key);
        } else {
            key = self
                .map_for_char(&key, &mut modifiers)
                .or_else(|_| self.map_virtual_key(&key, &mut modifiers))
                .context("Failed to map keystroke with use_key_equivalents = false")
                .log_err()
                .unwrap_or(key);
        }
        Keystroke {
            modifiers,
            key,
            key_char,
        }
    }

    fn to_vim_keystroke<'a>(&self, keystroke: &'a Keystroke) -> Cow<'a, Keystroke> {
        if keystroke.is_immutable_key() {
            return Cow::Borrowed(keystroke);
        }
        let mut modifiers = keystroke.modifiers;
        let vkey = match keystroke.key.as_str() {
            "a" => VK_A,
            "b" => VK_B,
            "c" => VK_C,
            "d" => VK_D,
            "e" => VK_E,
            "f" => VK_F,
            "g" => VK_G,
            "h" => VK_H,
            "i" => VK_I,
            "j" => VK_J,
            "k" => VK_K,
            "l" => VK_L,
            "m" => VK_M,
            "n" => VK_N,
            "o" => VK_O,
            "p" => VK_P,
            "q" => VK_Q,
            "r" => VK_R,
            "s" => VK_S,
            "t" => VK_T,
            "u" => VK_U,
            "v" => VK_V,
            "w" => VK_W,
            "x" => VK_X,
            "y" => VK_Y,
            "z" => VK_Z,
            key => {
                if key.len() != 1 {
                    log::error!(
                        "Failed to convert keystroke to vim keystroke: {}",
                        keystroke
                    );
                    return Cow::Borrowed(keystroke);
                }
                let Some(key) = self.get_vkey_from_char(key, &mut modifiers).log_err() else {
                    log::error!(
                        "Failed to convert keystroke to vim keystroke: {}",
                        keystroke
                    );
                    return Cow::Borrowed(keystroke);
                };
                key
            }
        };
        let new_key = {
            let mut state = [0; 256];
            if modifiers.shift {
                state[VK_SHIFT.0 as usize] = 0x80;
            }
            let scan_code = unsafe { MapVirtualKeyW(vkey.0 as u32, MAPVK_VK_TO_VSC) };
            let mut buffer = [0; 8];
            let len =
                unsafe { ToUnicode(vkey.0 as u32, scan_code, Some(&state), &mut buffer, 1 << 2) };
            if len > 0 {
                let candidate = String::from_utf16_lossy(&buffer[..len as usize]);
                if candidate.is_empty() {
                    keystroke.key.clone()
                } else {
                    if candidate.chars().next().unwrap().is_control() {
                        keystroke.key.clone()
                    } else {
                        candidate
                    }
                }
            } else {
                keystroke.key.clone()
            }
        };
        Cow::Owned(Keystroke {
            modifiers,
            key: new_key,
            key_char: keystroke.key_char.clone(),
        })
    }
}

impl WindowsKeyboardMapper {
    pub fn new() -> Self {
        Self
    }

    fn map_virtual_key(&self, key: &str, modifiers: &mut Modifiers) -> Result<String> {
        let (virtual_key, shift) = match key {
            // letters
            "a" => (VK_A, false),
            "b" => (VK_B, false),
            "c" => (VK_C, false),
            "d" => (VK_D, false),
            "e" => (VK_E, false),
            "f" => (VK_F, false),
            "g" => (VK_G, false),
            "h" => (VK_H, false),
            "i" => (VK_I, false),
            "j" => (VK_J, false),
            "k" => (VK_K, false),
            "l" => (VK_L, false),
            "m" => (VK_M, false),
            "n" => (VK_N, false),
            "o" => (VK_O, false),
            "p" => (VK_P, false),
            "q" => (VK_Q, false),
            "r" => (VK_R, false),
            "s" => (VK_S, false),
            "t" => (VK_T, false),
            "u" => (VK_U, false),
            "v" => (VK_V, false),
            "w" => (VK_W, false),
            "x" => (VK_X, false),
            "y" => (VK_Y, false),
            "z" => (VK_Z, false),
            // other keys
            "`" => (VK_OEM_3, false),
            "~" => (VK_OEM_3, true),
            "1" => (VK_1, false),
            "!" => (VK_1, true),
            "2" => (VK_2, false),
            "@" => (VK_2, true),
            "3" => (VK_3, false),
            "#" => (VK_3, true),
            "4" => (VK_4, false),
            "$" => (VK_4, true),
            "5" => (VK_5, false),
            "%" => (VK_5, true),
            "6" => (VK_6, false),
            "^" => (VK_6, true),
            "7" => (VK_7, false),
            "&" => (VK_7, true),
            "8" => (VK_8, false),
            "*" => (VK_8, true),
            "9" => (VK_9, false),
            "(" => (VK_9, true),
            "0" => (VK_0, false),
            ")" => (VK_0, true),
            "-" => (VK_OEM_MINUS, false),
            "_" => (VK_OEM_MINUS, true),
            "=" => (VK_OEM_PLUS, false),
            "+" => (VK_OEM_PLUS, true),
            "[" => (VK_OEM_4, false),
            "{" => (VK_OEM_4, true),
            "]" => (VK_OEM_6, false),
            "}" => (VK_OEM_6, true),
            "\\" => (VK_OEM_5, false),
            "|" => (VK_OEM_5, true),
            ";" => (VK_OEM_1, false),
            ":" => (VK_OEM_1, true),
            "'" => (VK_OEM_7, false),
            "\"" => (VK_OEM_7, true),
            "," => (VK_OEM_COMMA, false),
            "<" => (VK_OEM_COMMA, true),
            "." => (VK_OEM_PERIOD, false),
            ">" => (VK_OEM_PERIOD, true),
            "/" => (VK_OEM_2, false),
            "?" => (VK_OEM_2, true),
            _ => return Err(anyhow::anyhow!("Unrecognized key to virtual key: {}", key)),
        };
        let (key, _) = get_key_from_vkey(virtual_key).context(format!(
            "Failed to generate char given virtual key: {}, {:?}",
            key, virtual_key
        ))?;
        if shift {
            if modifiers.shift {
                log::error!(
                    "Shift modifier already set, but shift is required for this key: {}",
                    key
                );
            }
            modifiers.shift = true;
        }
        Ok(key)
    }

    fn map_for_char(&self, key: &str, modifiers: &mut Modifiers) -> Result<String> {
        let virtual_key = self.get_vkey_from_char(key, modifiers)?;
        let (key, _) = get_key_from_vkey(virtual_key).context(format!(
            "Failed to generate char given virtual key: {}, {:?}",
            key, virtual_key
        ))?;
        Ok(key)
    }

    fn get_vkey_from_char(&self, key: &str, modifiers: &mut Modifiers) -> Result<VIRTUAL_KEY> {
        if key.len() != 1 {
            return Err(anyhow::anyhow!(
                "Key must be a single character, but got: {}",
                key
            ));
        }
        let key_char = key
            .encode_utf16()
            .next()
            .context("Empty key in keystorke")?;
        let result = unsafe { VkKeyScanW(key_char) };
        if result == -1 {
            return Err(anyhow::anyhow!("Failed to get vkey from char: {}", key));
        }
        let high = (result >> 8) as i8;
        let low = result as u8;
        let (shift, ctrl, alt) = get_modifiers(high);
        if shift {
            if modifiers.shift {
                log::error!(
                    "Shift modifier already set, but shift is required for this key: {}",
                    key
                );
            }
            modifiers.shift = true;
        }
        if ctrl {
            if modifiers.control {
                log::error!(
                    "Ctrl modifier already set, but ctrl is required for this key: {}",
                    key
                );
            }
            modifiers.control = true;
        }
        if alt {
            if modifiers.alt {
                log::error!(
                    "Alt modifier already set, but alt is required for this key: {}",
                    key
                );
            }
            modifiers.alt = true;
        }
        Ok(VIRTUAL_KEY(low as u16))
    }
}

fn get_modifiers(high: i8) -> (bool, bool, bool) {
    let shift = high & 1;
    let ctrl = (high >> 1) & 1;
    let alt = (high >> 2) & 1;
    (shift != 0, ctrl != 0, alt != 0)
}

/// Converts a Windows virtual key code to its corresponding character and dead key status.
///
/// # Parameters
/// * `vkey` - The virtual key code to convert
///
/// # Returns
/// * `Some((String, bool))` - The character as a string and a boolean indicating if it's a dead key.
///   A dead key is a key that doesn't produce a character by itself but modifies the next key pressed
///   (e.g., accent keys like ^ or `).
/// * `None` - If the virtual key code doesn't map to a character
pub(crate) fn get_key_from_vkey(vkey: VIRTUAL_KEY) -> Option<(String, bool)> {
    let key_data = unsafe { MapVirtualKeyW(vkey.0 as u32, MAPVK_VK_TO_CHAR) };
    if key_data == 0 {
        return None;
    }

    // The high word contains dead key flag, the low word contains the character
    let is_dead_key = (key_data >> 16) > 0;
    let key = char::from_u32(key_data & 0xFFFF)?;

    Some((key.to_ascii_lowercase().to_string(), is_dead_key))
}

impl PlatformKeyboardLayout for KeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl KeyboardLayout {
    pub(crate) fn new() -> Result<Self> {
        let id = get_keyboard_layout_id()?;
        let name = get_keyboard_layout_name(&id).unwrap_or("unknown".to_string());
        Ok(Self { id, name })
    }

    pub(crate) fn unknown() -> Self {
        Self {
            id: "unknown".to_string(),
            name: "unknown".to_string(),
        }
    }
}

pub(crate) fn get_keyboard_layout_id() -> Result<String> {
    let mut buffer = [0u16; KL_NAMELENGTH as usize];
    unsafe { GetKeyboardLayoutNameW(&mut buffer) }?;
    let kbd_layout_name = HSTRING::from_wide(&buffer);
    Ok(kbd_layout_name.to_string())
}

pub(crate) fn get_keyboard_layout_name(id: &str) -> Result<String> {
    let entry = format!(
        "System\\CurrentControlSet\\Control\\Keyboard Layouts\\{}",
        id
    );
    let key = windows_registry::LOCAL_MACHINE.open(entry)?;
    Ok(key.get_hstring("Layout Text")?.to_string())
}
