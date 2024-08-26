use macos::translate_key_macos;

mod keyboard_layout;
mod macos;
mod scancode;

pub fn translate_key(keystroke: &str, keyboard_layout: KeyboardLayout) -> String {
    match keyboard_layout {
        KeyboardLayout::EnUs => keystroke.to_string(),
        KeyboardLayout::Czech => keyboard_layout::czech::map_keystroke(keystroke),
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum KeyboardLayout {
    #[default]
    EnUs,
    Czech,
}
