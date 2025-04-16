use anyhow::{Context, Result};
use util::ResultExt;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::{KeyboardMapper, Keystroke, Modifiers};

pub(crate) struct WindowsKeyboardMapper;

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
                .log_err()
                .unwrap_or_else(|| {
                    self.map_for_char(&key, &mut modifiers)
                        .log_err()
                        .unwrap_or(key)
                });
        } else {
            key = self
                .map_for_char(&key, &mut modifiers)
                .log_err()
                .unwrap_or_else(|| {
                    self.map_virtual_key(&key, &mut modifiers)
                        .log_err()
                        .unwrap_or(key)
                });
        }
        Keystroke {
            modifiers,
            key,
            key_char,
        }
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
        let key = char::from_u32(unsafe { MapVirtualKeyW(virtual_key.0 as u32, MAPVK_VK_TO_CHAR) })
            .context(format!(
                "Failed to generate char given virtual key: {}, {:?}",
                key, virtual_key
            ))?
            .to_ascii_lowercase();
        if shift {
            if modifiers.shift {
                log::error!(
                    "Shift modifier already set, but shift is required for this key: {}",
                    key
                );
            }
            modifiers.shift = true;
        }
        Ok(key.to_string())
    }

    fn map_for_char(&self, key: &str, modifiers: &mut Modifiers) -> Result<String> {
        let key_char = key
            .encode_utf16()
            .next()
            .context("Empty key in keystorke")?;
        let result = unsafe { VkKeyScanW(key_char) };
        if result == -1 {
            return Err(anyhow::anyhow!("Unrecognized key to virtual key: {}", key));
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
        let virtual_key = low as u32;
        let key = char::from_u32(unsafe { MapVirtualKeyW(virtual_key, MAPVK_VK_TO_CHAR) })
            .context(format!(
                "Failed to generate char given virtual key: {}, {:?}",
                key, virtual_key
            ))?
            .to_ascii_lowercase();
        Ok(key.to_string())
    }
}

fn get_modifiers(high: i8) -> (bool, bool, bool) {
    let shift = high & 1;
    let ctrl = (high >> 1) & 1;
    let alt = (high >> 2) & 1;
    (shift != 0, ctrl != 0, alt != 0)
}
