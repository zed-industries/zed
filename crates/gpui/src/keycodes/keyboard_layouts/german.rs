// use std::sync::LazyLock;

// use collections::FxHashMap;

// use crate::KeyCodes;

// pub(crate) static GERMAN_LAYOUT_ANSI: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x32, KeyCodes::Backslash);
//     map.insert(0x1B, KeyCodes::LeftBracket);
//     map.insert(0x18, KeyCodes::RightBracket);
//     map.insert(0x21, KeyCodes::Semicolon);
//     map.insert(0x2A, KeyCodes::Slash);
//     map.insert(0x29, KeyCodes::Tilde);
//     map.insert(0x27, KeyCodes::Quote);
//     map
// });

// pub(crate) static GERMAN_LAYOUT_ISO: LazyLock<FxHashMap<u16, KeyCodes>> = LazyLock::new(|| {
//     let mut map = FxHashMap::default();
//     map.insert(0x0A, KeyCodes::Backslash);
//     map.insert(0x1B, KeyCodes::LeftBracket);
//     map.insert(0x18, KeyCodes::RightBracket);
//     map.insert(0x21, KeyCodes::Semicolon);
//     map.insert(0x2A, KeyCodes::Slash);
//     map.insert(0x29, KeyCodes::Tilde);
//     map.insert(0x27, KeyCodes::Quote);
//     map.insert(0x32, KeyCodes::OEM102);
//     map
// });
