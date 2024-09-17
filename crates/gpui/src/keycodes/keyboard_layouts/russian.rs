use std::sync::LazyLock;

use collections::FxHashMap;

use crate::Keys;

pub(crate) static RUSSIAN_LAYOUT_ANSI: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x32, Keys::Tilde);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map.insert(0x2C, Keys::Slash);
    map
});

pub(crate) static RUSSIAN_LAYOUT_ISO: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x0A, Keys::Tilde);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map.insert(0x2C, Keys::Slash);
    map.insert(0x32, Keys::OEM102);
    map
});
