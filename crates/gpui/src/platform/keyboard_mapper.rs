use std::sync::LazyLock;

use collections::HashMap;

use super::{
    always_use_command_layout, chars_for_modified_key, keyboard_layout, KeyCode, Modifiers,
};

/// TODO:
pub struct KeyboardMapperManager {
    mapper: HashMap<String, KeyboardMapper>,
}

/// TODO:
pub struct KeyboardMapper {
    letter: HashMap<String, KeyCode>,
    other: HashMap<String, (KeyCode, Modifiers)>,
    code_to_char: HashMap<KeyCode, String>,
}

impl KeyboardMapperManager {
    pub(crate) fn new() -> Self {
        let mut mapper = HashMap::default();
        let current_layout = keyboard_layout();
        mapper.insert(current_layout, KeyboardMapper::new());

        Self { mapper }
    }

    pub(crate) fn update(&mut self, layout: &str) {
        if !self.mapper.contains_key(layout) {
            let info = KeyboardMapper::new();
            self.mapper.insert(layout.to_string(), info);
        }
    }

    pub(crate) fn get_mapper(&self, layout: &str) -> &KeyboardMapper {
        self.mapper.get(layout).unwrap()
    }
}

impl KeyboardMapper {
    fn new() -> Self {
        let mut letter = HashMap::default();
        let mut other = HashMap::default();
        let mut code_to_char = HashMap::default();

        if always_use_command_layout() {
            letter.insert("a".to_string(), KeyCode::A);
            letter.insert("b".to_string(), KeyCode::B);
            letter.insert("c".to_string(), KeyCode::C);
            letter.insert("d".to_string(), KeyCode::D);
            letter.insert("e".to_string(), KeyCode::E);
            letter.insert("f".to_string(), KeyCode::F);
            letter.insert("g".to_string(), KeyCode::G);
            letter.insert("h".to_string(), KeyCode::H);
            letter.insert("i".to_string(), KeyCode::I);
            letter.insert("j".to_string(), KeyCode::J);
            letter.insert("k".to_string(), KeyCode::K);
            letter.insert("l".to_string(), KeyCode::L);
            letter.insert("m".to_string(), KeyCode::M);
            letter.insert("n".to_string(), KeyCode::N);
            letter.insert("o".to_string(), KeyCode::O);
            letter.insert("p".to_string(), KeyCode::P);
            letter.insert("q".to_string(), KeyCode::Q);
            letter.insert("r".to_string(), KeyCode::R);
            letter.insert("s".to_string(), KeyCode::S);
            letter.insert("t".to_string(), KeyCode::T);
            letter.insert("u".to_string(), KeyCode::U);
            letter.insert("v".to_string(), KeyCode::V);
            letter.insert("w".to_string(), KeyCode::W);
            letter.insert("x".to_string(), KeyCode::X);
            letter.insert("y".to_string(), KeyCode::Y);
            letter.insert("z".to_string(), KeyCode::Z);
        }

        // 0x001d => KeyCode::Digital0,
        let chars = generate_keymap_info(0x001d);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital0, c.clone());
            }
            other.insert(c, (KeyCode::Digital0, modifiers));
        }
        // 0x0012 => KeyCode::Digital1,
        let chars = generate_keymap_info(0x0012);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital1, c.clone());
            }
            other.insert(c, (KeyCode::Digital1, modifiers));
        }
        // 0x0013 => KeyCode::Digital2,
        let chars = generate_keymap_info(0x0013);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital2, c.clone());
            }
            other.insert(c, (KeyCode::Digital2, modifiers));
        }
        // 0x0014 => KeyCode::Digital3,
        let chars = generate_keymap_info(0x0014);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital3, c.clone());
            }
            other.insert(c, (KeyCode::Digital3, modifiers));
        }
        // 0x0015 => KeyCode::Digital4,
        let chars = generate_keymap_info(0x0015);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital4, c.clone());
            }
            other.insert(c, (KeyCode::Digital4, modifiers));
        }
        // 0x0017 => KeyCode::Digital5,
        let chars = generate_keymap_info(0x0017);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital5, c.clone());
            }
            other.insert(c, (KeyCode::Digital5, modifiers));
        }
        // 0x0016 => KeyCode::Digital6,
        let chars = generate_keymap_info(0x0016);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital6, c.clone());
            }
            other.insert(c, (KeyCode::Digital6, modifiers));
        }
        // 0x001a => KeyCode::Digital7,
        let chars = generate_keymap_info(0x001a);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital7, c.clone());
            }
            other.insert(c, (KeyCode::Digital7, modifiers));
        }
        // 0x001c => KeyCode::Digital8,
        let chars = generate_keymap_info(0x001c);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital8, c.clone());
            }
            other.insert(c, (KeyCode::Digital8, modifiers));
        }
        // 0x0019 => KeyCode::Digital9,
        let chars = generate_keymap_info(0x0019);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Digital9, c.clone());
            }
            other.insert(c, (KeyCode::Digital9, modifiers));
        }
        // 0x0029 => KeyCode::Semicolon,
        let chars = generate_keymap_info(0x0029);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Semicolon, c.clone());
            }
            other.insert(c, (KeyCode::Semicolon, modifiers));
        }
        // 0x0018 => KeyCode::Plus,
        let chars = generate_keymap_info(0x0018);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Plus, c.clone());
            }
            other.insert(c, (KeyCode::Plus, modifiers));
        }
        // 0x002b => KeyCode::Comma,
        let chars = generate_keymap_info(0x002b);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Comma, c.clone());
            }
            other.insert(c, (KeyCode::Comma, modifiers));
        }
        // 0x001b => KeyCode::Minus,
        let chars = generate_keymap_info(0x001b);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Minus, c.clone());
            }
            other.insert(c, (KeyCode::Minus, modifiers));
        }
        // 0x002f => KeyCode::Period,
        let chars = generate_keymap_info(0x002f);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Period, c.clone());
            }
            other.insert(c, (KeyCode::Period, modifiers));
        }
        // 0x002c => KeyCode::Slash,
        let chars = generate_keymap_info(0x002c);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Slash, c.clone());
            }
            other.insert(c, (KeyCode::Slash, modifiers));
        }
        // 0x0032 => KeyCode::Tilde,
        let chars = generate_keymap_info(0x0032);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Tilde, c.clone());
            }
            other.insert(c, (KeyCode::Tilde, modifiers));
        }
        // 0x0021 => KeyCode::LeftBracket,
        let chars = generate_keymap_info(0x0021);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::LeftBracket, c.clone());
            }
            other.insert(c, (KeyCode::LeftBracket, modifiers));
        }
        // 0x002a => KeyCode::Backslash,
        let chars = generate_keymap_info(0x002a);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Backslash, c.clone());
            }
            other.insert(c, (KeyCode::Backslash, modifiers));
        }
        // 0x001e => KeyCode::RightBracket,
        let chars = generate_keymap_info(0x001e);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::RightBracket, c.clone());
            }
            other.insert(c, (KeyCode::RightBracket, modifiers));
        }
        // 0x0027 => KeyCode::Quote,
        let chars = generate_keymap_info(0x0027);
        for (c, modifiers) in chars {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Quote, c.clone());
            }
            other.insert(c, (KeyCode::Quote, modifiers));
        }
        // 0x0000 => KeyCode::A,
        for (c, modifiers) in generate_keymap_info(0x0000) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::A, c.clone());
            }
            other.insert(c, (KeyCode::A, modifiers));
        }
        // 0x000b => KeyCode::B,
        for (c, modifiers) in generate_keymap_info(0x000b) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::B, c.clone());
            }
            other.insert(c, (KeyCode::B, modifiers));
        }
        // 0x0008 => KeyCode::C,
        for (c, modifiers) in generate_keymap_info(0x0008) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::C, c.clone());
            }
            other.insert(c, (KeyCode::C, modifiers));
        }
        // 0x0002 => KeyCode::D,
        for (c, modifiers) in generate_keymap_info(0x0002) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::D, c.clone());
            }
            other.insert(c, (KeyCode::D, modifiers));
        }
        // 0x000e => KeyCode::E,
        for (c, modifiers) in generate_keymap_info(0x000e) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::E, c.clone());
            }
            other.insert(c, (KeyCode::E, modifiers));
        }
        // 0x0003 => KeyCode::F,
        for (c, modifiers) in generate_keymap_info(0x0003) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::F, c.clone());
            }
            other.insert(c, (KeyCode::F, modifiers));
        }
        // 0x0005 => KeyCode::G,
        for (c, modifiers) in generate_keymap_info(0x0005) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::G, c.clone());
            }
            other.insert(c, (KeyCode::G, modifiers));
        }
        // 0x0004 => KeyCode::H,
        for (c, modifiers) in generate_keymap_info(0x0004) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::H, c.clone());
            }
            other.insert(c, (KeyCode::H, modifiers));
        }
        // 0x0022 => KeyCode::I,
        for (c, modifiers) in generate_keymap_info(0x0022) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::I, c.clone());
            }
            other.insert(c, (KeyCode::I, modifiers));
        }
        // 0x0026 => KeyCode::J,
        for (c, modifiers) in generate_keymap_info(0x0026) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::J, c.clone());
            }
            other.insert(c, (KeyCode::J, modifiers));
        }
        // 0x0028 => KeyCode::K,
        for (c, modifiers) in generate_keymap_info(0x0028) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::K, c.clone());
            }
            other.insert(c, (KeyCode::K, modifiers));
        }
        // 0x0025 => KeyCode::L,
        for (c, modifiers) in generate_keymap_info(0x0025) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::L, c.clone());
            }
            other.insert(c, (KeyCode::L, modifiers));
        }
        // 0x002e => KeyCode::M,
        for (c, modifiers) in generate_keymap_info(0x002e) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::M, c.clone());
            }
            other.insert(c, (KeyCode::M, modifiers));
        }
        // 0x002d => KeyCode::N,
        for (c, modifiers) in generate_keymap_info(0x002d) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::N, c.clone());
            }
            other.insert(c, (KeyCode::N, modifiers));
        }
        // 0x001f => KeyCode::O,
        for (c, modifiers) in generate_keymap_info(0x001f) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::O, c.clone());
            }
            other.insert(c, (KeyCode::O, modifiers));
        }
        // 0x0023 => KeyCode::P,
        for (c, modifiers) in generate_keymap_info(0x0023) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::P, c.clone());
            }
            other.insert(c, (KeyCode::P, modifiers));
        }
        // 0x000c => KeyCode::Q,
        for (c, modifiers) in generate_keymap_info(0x000c) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Q, c.clone());
            }
            other.insert(c, (KeyCode::Q, modifiers));
        }
        // 0x000f => KeyCode::R,
        for (c, modifiers) in generate_keymap_info(0x000f) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::R, c.clone());
            }
            other.insert(c, (KeyCode::R, modifiers));
        }
        // 0x0001 => KeyCode::S,
        for (c, modifiers) in generate_keymap_info(0x0001) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::S, c.clone());
            }
            other.insert(c, (KeyCode::S, modifiers));
        }
        // 0x0011 => KeyCode::T,
        for (c, modifiers) in generate_keymap_info(0x0011) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::T, c.clone());
            }
            other.insert(c, (KeyCode::T, modifiers));
        }
        // 0x0020 => KeyCode::U,
        for (c, modifiers) in generate_keymap_info(0x0020) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::U, c.clone());
            }
            other.insert(c, (KeyCode::U, modifiers));
        }
        // 0x0009 => KeyCode::V,
        for (c, modifiers) in generate_keymap_info(0x0009) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::V, c.clone());
            }
            other.insert(c, (KeyCode::V, modifiers));
        }
        // 0x000d => KeyCode::W,
        for (c, modifiers) in generate_keymap_info(0x000d) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::W, c.clone());
            }
            other.insert(c, (KeyCode::W, modifiers));
        }
        // 0x0007 => KeyCode::X,
        for (c, modifiers) in generate_keymap_info(0x0007) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::X, c.clone());
            }
            other.insert(c, (KeyCode::X, modifiers));
        }
        // 0x0010 => KeyCode::Y,
        for (c, modifiers) in generate_keymap_info(0x0010) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Y, c.clone());
            }
            other.insert(c, (KeyCode::Y, modifiers));
        }
        // 0x0006 => KeyCode::Z,
        for (c, modifiers) in generate_keymap_info(0x0006) {
            if modifiers == Modifiers::none() {
                code_to_char.insert(KeyCode::Z, c.clone());
            }
            other.insert(c, (KeyCode::Z, modifiers));
        }

        Self {
            letter,
            other,
            code_to_char,
        }
    }

    pub(crate) fn parse(&self, input: &str, char_matching: bool) -> Option<(KeyCode, Modifiers)> {
        if !char_matching {
            if let Some(code) = self.letter.get(input) {
                return Some((*code, Modifiers::none()));
            }
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
            if let Some((code, modifiers)) = self.other.get(input) {
                return Some((*code, *modifiers));
            }
            if let Some(code) = self.letter.get(input) {
                return Some((*code, Modifiers::none()));
            }
        }
        None
    }

    pub(crate) fn code_to_char(&self, code: KeyCode) -> Option<String> {
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

const NO_MOD: u32 = 0;
const CMD_MOD: u32 = 1;
const SHIFT_MOD: u32 = 2;
const OPTION_MOD: u32 = 8;
