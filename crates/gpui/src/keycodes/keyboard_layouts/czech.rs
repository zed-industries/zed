use std::sync::LazyLock;

use collections::FxHashMap;

use crate::Keys;

// http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ANSI104
pub(crate) static CZECH_QWERTY_ANSI: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x32, Keys::Tilde);
    map.insert(0x1B, Keys::Minus);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map.insert(0x2C, Keys::Slash);
    map
});

// http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ISO105
pub(crate) static CZECH_QWERTY_ISO: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x0A, Keys::Tilde);
    map.insert(0x1B, Keys::Minus);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x2C, Keys::Slash);
    map.insert(0x32, Keys::OEM102);
    map
});

// http://kbdlayout.info/kbdcz/virtualkeys?arrangement=ANSI104
pub(crate) static CZECH_LAYOUT_ANSI: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x32, Keys::Tilde);
    map.insert(0x18, Keys::Slash);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map
});

// http://kbdlayout.info/kbdcz/virtualkeys?arrangement=ISO105
pub(crate) static CZECH_LAYOUT_ISO: LazyLock<FxHashMap<u16, Keys>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    map.insert(0x0A, Keys::Tilde);
    map.insert(0x21, Keys::LeftBracket);
    map.insert(0x1E, Keys::RightBracket);
    map.insert(0x29, Keys::Semicolon);
    map.insert(0x27, Keys::Quote);
    map.insert(0x2A, Keys::Backslash);
    map.insert(0x2C, Keys::Slash);
    map.insert(0x32, Keys::OEM102);
    map
});
