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

use super::VirtualKeyCode;

pub(crate) type KeyboardLayoutMapping = &'static LazyLock<FxHashMap<u16, VirtualKeyCode>>;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum KeyboardLayout {
    #[default]
    ABC,
    // Czech,
    CzechQwerty,
    German,
    Russian,
}

impl KeyboardLayout {
    pub(crate) fn layout_data(&self) -> Option<KeyboardLayoutMapping> {
        match self {
            KeyboardLayout::ABC => None,
            KeyboardLayout::CzechQwerty => Some(&CZECH_QWERTY_ANSI),
            KeyboardLayout::German => Some(&GERMAN_LAYOUT_ANSI),
            KeyboardLayout::Russian => Some(&RUSSIAN_LAYOUT_ANSI),
        }
    }
}
