// use std::sync::LazyLock;

// use collections::FxHashMap;

// use crate::KeyCodes;

// // http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ANSI104
// pub(crate) static CZECH_QWERTY_ANSI: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x32, KeyCodes::Tilde);
//     map.insert(0x1B, KeyCodes::Minus);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x2C, KeyCodes::Slash);
//     map
// });

// // http://kbdlayout.info/kbdcz1/virtualkeys?arrangement=ISO105
// pub(crate) static CZECH_QWERTY_ISO: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x0A, KeyCodes::Tilde);
//     map.insert(0x1B, KeyCodes::Minus);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x2C, KeyCodes::Slash);
//     map.insert(0x32, KeyCodes::OEM102);
//     map
// });

// // http://kbdlayout.info/kbdcz/virtualkeys?arrangement=ANSI104
// pub(crate) static CZECH_LAYOUT_ANSI: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x32, KeyCodes::Tilde);
//     map.insert(0x18, KeyCodes::Slash);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map
// });

// // http://kbdlayout.info/kbdcz/virtualkeys?arrangement=ISO105
// pub(crate) static CZECH_LAYOUT_ISO: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x0A, KeyCodes::Tilde);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x2C, KeyCodes::Slash);
//     map.insert(0x32, KeyCodes::OEM102);
//     map
// });
