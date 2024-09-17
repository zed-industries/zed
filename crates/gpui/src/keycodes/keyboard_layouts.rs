#![allow(dead_code)]

pub(crate) mod czech;
pub(crate) mod german;
pub(crate) mod russian;

use std::sync::LazyLock;

use collections::FxHashMap;
// https://stackoverflow.com/questions/3202629/where-can-i-find-a-list-of-mac-virtual-key-codes
pub(crate) use czech::*;
pub(crate) use german::*;
pub(crate) use russian::*;

use super::KeyCodes;

pub(crate) type KeyboardLayoutMapping = &'static LazyLock<FxHashMap<u16, KeyCodes>>;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum KeyboardLayout {
    #[default]
    ABC,
    Czech,
    CzechQwerty,
    German,
    Russian,
}

impl KeyboardLayout {
    pub(crate) fn layout_data(&self, keyboard_iso: bool) -> Option<KeyboardLayoutMapping> {
        if keyboard_iso {
            println!("ISO keyboard detected.");
        } else {
            println!("ANSI keyboard detected.");
        }
        match self {
            KeyboardLayout::ABC => None,
            KeyboardLayout::Czech => Some(if keyboard_iso {
                &CZECH_LAYOUT_ISO
            } else {
                &CZECH_LAYOUT_ANSI
            }),
            KeyboardLayout::CzechQwerty => Some(if keyboard_iso {
                &CZECH_QWERTY_ISO
            } else {
                &CZECH_QWERTY_ANSI
            }),
            KeyboardLayout::German => Some(if keyboard_iso {
                &GERMAN_LAYOUT_ISO
            } else {
                &GERMAN_LAYOUT_ANSI
            }),
            KeyboardLayout::Russian => Some(if keyboard_iso {
                &RUSSIAN_LAYOUT_ISO
            } else {
                &RUSSIAN_LAYOUT_ANSI
            }),
        }
    }
}
