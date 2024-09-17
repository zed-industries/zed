use std::sync::LazyLock;

use collections::FxHashMap;

use crate::Keys;

pub(crate) static GERMAN_LAYOUT_ANSI: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x32, Keys::Backslash);
    map.insert(0x1B, Keys::LeftBracket);
    map.insert(0x18, Keys::RightBracket);
    map.insert(0x21, Keys::Semicolon);
    map.insert(0x2A, Keys::Slash);
    map.insert(0x29, Keys::Tilde);
    map.insert(0x27, Keys::Quote);
    map
});

pub(crate) static GERMAN_LAYOUT_ISO: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x0A, Keys::Backslash);
    map.insert(0x1B, Keys::LeftBracket);
    map.insert(0x18, Keys::RightBracket);
    map.insert(0x21, Keys::Semicolon);
    map.insert(0x2A, Keys::Slash);
    map.insert(0x29, Keys::Tilde);
    map.insert(0x27, Keys::Quote);
    map.insert(0x32, Keys::OEM102);
    map
});
