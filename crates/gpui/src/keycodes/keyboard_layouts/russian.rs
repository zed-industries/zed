// use std::sync::LazyLock;

// use collections::FxHashMap;

// use crate::KeyCodes;

// pub(crate) static RUSSIAN_LAYOUT_ANSI: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x32, KeyCodes::Tilde);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x2C, KeyCodes::Slash);
//     map
// });

// pub(crate) static RUSSIAN_LAYOUT_ISO: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x0A, KeyCodes::Tilde);
//     map.insert(0x21, KeyCodes::LeftBracket);
//     map.insert(0x1E, KeyCodes::RightBracket);
//     map.insert(0x2A, KeyCodes::Backslash);
//     map.insert(0x29, KeyCodes::Semicolon);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x2C, KeyCodes::Slash);
//     map.insert(0x32, KeyCodes::OEM102);
//     map
// });
