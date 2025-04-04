use std::rc::Rc;

use collections::HashMap;

use crate::{chars_for_modified_key, KeyCode, KeyboardMapper, Modifiers};

/// TODO:
pub(crate) struct MacKeyboardMapperManager {
    mapper: HashMap<String, Rc<MacKeyboardMapper>>,
}

/// TODO:
pub(crate) struct MacKeyboardMapper {
    char_to_code: HashMap<String, (KeyCode, Modifiers)>,
    code_to_char: HashMap<KeyCode, String>,
}

impl MacKeyboardMapperManager {
    pub(crate) fn new() -> Self {
        let mut mapper = HashMap::default();
        let current_layout = keyboard_layout();
        mapper.insert(current_layout, Rc::new(MacKeyboardMapper::new()));

        Self { mapper }
    }

    pub(crate) fn update(&mut self, layout: &str) {
        if !self.mapper.contains_key(layout) {
            let info = MacKeyboardMapper::new();
            self.mapper.insert(layout.to_string(), Rc::new(info));
        }
    }

    pub(crate) fn get_mapper(&self, layout: &str) -> Rc<MacKeyboardMapper> {
        self.mapper.get(layout).unwrap().clone()
    }
}

impl MacKeyboardMapper {
    fn new() -> Self {
        let mut char_to_code = HashMap::default();
        let mut code_to_char = HashMap::default();

        for (scan_code, code) in OTHER_CODES {
            for (key, modifiers) in generate_keymap_info(scan_code) {
                if modifiers == Modifiers::none() {
                    code_to_char.insert(code, key.clone());
                }
                char_to_code.insert(key, (code, modifiers));
            }
        }

        Self {
            char_to_code,
            code_to_char,
        }
    }
}

impl KeyboardMapper for MacKeyboardMapper {
    fn parse(&self, input: &str, char_matching: bool) -> Option<(KeyCode, Modifiers)> {
        if let Some(code) = parse_letters(input) {
            return Some((code, Modifiers::none()));
        }
        if !char_matching {
            if let Some(code) = match input {
                "0" => Some(KeyCode::Digital0),
                "1" => Some(KeyCode::Digital1),
                "2" => Some(KeyCode::Digital2),
                "3" => Some(KeyCode::Digital3),
                "4" => Some(KeyCode::Digital4),
                "5" => Some(KeyCode::Digital5),
                "6" => Some(KeyCode::Digital6),
                "7" => Some(KeyCode::Digital7),
                "8" => Some(KeyCode::Digital8),
                "9" => Some(KeyCode::Digital9),
                ";" => Some(KeyCode::Semicolon),
                "=" => Some(KeyCode::Plus),
                "," => Some(KeyCode::Comma),
                "-" => Some(KeyCode::Minus),
                "." => Some(KeyCode::Period),
                "/" => Some(KeyCode::Slash),
                "`" => Some(KeyCode::Tilde),
                "[" => Some(KeyCode::LeftBracket),
                "\\" => Some(KeyCode::Backslash),
                "]" => Some(KeyCode::RightBracket),
                "'" => Some(KeyCode::Quote),
                _ => None,
            } {
                return Some((code, Modifiers::none()));
            }
        } else {
            if let Some((code, modifiers)) = self.char_to_code.get(input) {
                return Some((*code, *modifiers));
            }
        }
        None
    }

    fn keycode_to_face(&self, code: KeyCode) -> Option<String> {
        self.code_to_char.get(&code).cloned()
    }
}

fn generate_keymap_info(scan_code: u16) -> Vec<(String, Modifiers)> {
    let mut keymap = Vec::new();
    let no_mod = chars_for_modified_key(scan_code, NO_MOD);
    if !no_mod.is_empty() {
        keymap.push((no_mod, Modifiers::none()));
    }
    let shift_mod = chars_for_modified_key(scan_code, SHIFT_MOD);
    if !shift_mod.is_empty() {
        keymap.push((shift_mod, Modifiers::shift()));
    }
    let alt_mod = chars_for_modified_key(scan_code, OPTION_MOD);
    if !alt_mod.is_empty() {
        keymap.push((alt_mod, Modifiers::alt()));
    }
    let shift_alt_mod = chars_for_modified_key(scan_code, SHIFT_MOD | OPTION_MOD);
    if !shift_alt_mod.is_empty() {
        keymap.push((
            shift_alt_mod,
            Modifiers {
                shift: true,
                alt: true,
                ..Default::default()
            },
        ));
    }
    keymap
}

fn parse_letters(input: &str) -> Option<KeyCode> {
    match input {
        "a" => Some(KeyCode::A),
        "b" => Some(KeyCode::B),
        "c" => Some(KeyCode::C),
        "d" => Some(KeyCode::D),
        "e" => Some(KeyCode::E),
        "f" => Some(KeyCode::F),
        "g" => Some(KeyCode::G),
        "h" => Some(KeyCode::H),
        "i" => Some(KeyCode::I),
        "j" => Some(KeyCode::J),
        "k" => Some(KeyCode::K),
        "l" => Some(KeyCode::L),
        "m" => Some(KeyCode::M),
        "n" => Some(KeyCode::N),
        "o" => Some(KeyCode::O),
        "p" => Some(KeyCode::P),
        "q" => Some(KeyCode::Q),
        "r" => Some(KeyCode::R),
        "s" => Some(KeyCode::S),
        "t" => Some(KeyCode::T),
        "u" => Some(KeyCode::U),
        "v" => Some(KeyCode::V),
        "w" => Some(KeyCode::W),
        "x" => Some(KeyCode::X),
        "y" => Some(KeyCode::Y),
        "z" => Some(KeyCode::Z),
        _ => None,
    }
}

pub(crate) fn keyboard_layout() -> String {
    use std::ffi::{c_void, CStr};

    use objc::{msg_send, runtime::Object, sel, sel_impl};

    use crate::platform::mac::{
        kTISPropertyInputSourceID, TISCopyCurrentKeyboardLayoutInputSource,
        TISGetInputSourceProperty,
    };

    unsafe {
        let current_keyboard = TISCopyCurrentKeyboardLayoutInputSource();

        let input_source_id: *mut Object =
            TISGetInputSourceProperty(current_keyboard, kTISPropertyInputSourceID as *const c_void);
        let input_source_id: *const std::os::raw::c_char = msg_send![input_source_id, UTF8String];
        let input_source_id = CStr::from_ptr(input_source_id).to_str().unwrap();

        input_source_id.to_string()
    }
}

const NO_MOD: u32 = 0;
const SHIFT_MOD: u32 = 2;
const OPTION_MOD: u32 = 8;

static OTHER_CODES: [(u16, KeyCode); 47] = [
    // 0x001d => KeyCode::Digital0,
    (0x001d, KeyCode::Digital0),
    // 0x0012 => KeyCode::Digital1,
    (0x0012, KeyCode::Digital1),
    // 0x0013 => KeyCode::Digital2,
    (0x0013, KeyCode::Digital2),
    // 0x0014 => KeyCode::Digital3,
    (0x0014, KeyCode::Digital3),
    // 0x0015 => KeyCode::Digital4,
    (0x0015, KeyCode::Digital4),
    // 0x0017 => KeyCode::Digital5,
    (0x0017, KeyCode::Digital5),
    // 0x0016 => KeyCode::Digital6,
    (0x0016, KeyCode::Digital6),
    // 0x001a => KeyCode::Digital7,
    (0x001a, KeyCode::Digital7),
    // 0x001c => KeyCode::Digital8,
    (0x001c, KeyCode::Digital8),
    // 0x0019 => KeyCode::Digital9,
    (0x0019, KeyCode::Digital9),
    // 0x0029 => KeyCode::Semicolon,
    (0x0029, KeyCode::Semicolon),
    // 0x0018 => KeyCode::Plus,
    (0x0018, KeyCode::Plus),
    // 0x002b => KeyCode::Comma,
    (0x002b, KeyCode::Comma),
    // 0x001b => KeyCode::Minus,
    (0x001b, KeyCode::Minus),
    // 0x002f => KeyCode::Period,
    (0x002f, KeyCode::Period),
    // 0x002c => KeyCode::Slash,
    (0x002c, KeyCode::Slash),
    // 0x0032 => KeyCode::Tilde,
    (0x0032, KeyCode::Tilde),
    // 0x0021 => KeyCode::LeftBracket,
    (0x0021, KeyCode::LeftBracket),
    // 0x002a => KeyCode::Backslash,
    (0x002a, KeyCode::Backslash),
    // 0x001e => KeyCode::RightBracket,
    (0x001e, KeyCode::RightBracket),
    // 0x0027 => KeyCode::Quote,
    (0x0027, KeyCode::Quote),
    // 0x0000 => KeyCode::A,
    (0x0000, KeyCode::A),
    // 0x000b => KeyCode::B,
    (0x000b, KeyCode::B),
    // 0x0008 => KeyCode::C,
    (0x0008, KeyCode::C),
    // 0x0002 => KeyCode::D,
    (0x0002, KeyCode::D),
    // 0x000e => KeyCode::E,
    (0x000e, KeyCode::E),
    // 0x0003 => KeyCode::F,
    (0x0003, KeyCode::F),
    // 0x0005 => KeyCode::G,
    (0x0005, KeyCode::G),
    // 0x0004 => KeyCode::H,
    (0x0004, KeyCode::H),
    // 0x0022 => KeyCode::I,
    (0x0022, KeyCode::I),
    // 0x0026 => KeyCode::J,
    (0x0026, KeyCode::J),
    // 0x0028 => KeyCode::K,
    (0x0028, KeyCode::K),
    // 0x0025 => KeyCode::L,
    (0x0025, KeyCode::L),
    // 0x002e => KeyCode::M,
    (0x002e, KeyCode::M),
    // 0x002d => KeyCode::N,
    (0x002d, KeyCode::N),
    // 0x001f => KeyCode::O,
    (0x001f, KeyCode::O),
    // 0x0023 => KeyCode::P,
    (0x0023, KeyCode::P),
    // 0x000c => KeyCode::Q,
    (0x000c, KeyCode::Q),
    // 0x000f => KeyCode::R,
    (0x000f, KeyCode::R),
    // 0x0001 => KeyCode::S,
    (0x0001, KeyCode::S),
    // 0x0011 => KeyCode::T,
    (0x0011, KeyCode::T),
    // 0x0020 => KeyCode::U,
    (0x0020, KeyCode::U),
    // 0x0009 => KeyCode::V,
    (0x0009, KeyCode::V),
    // 0x000d => KeyCode::W,
    (0x000d, KeyCode::W),
    // 0x0007 => KeyCode::X,
    (0x0007, KeyCode::X),
    // 0x0010 => KeyCode::Y,
    (0x0010, KeyCode::Y),
    // 0x0006 => KeyCode::Z,
    (0x0006, KeyCode::Z),
];
