//! Heuristic text width estimation helpers.

pub(crate) fn estimate_line_width_px(line: &str, font_size: f64) -> f64 {
    let mut em = 0.0;
    for ch in line.chars() {
        em += estimate_char_width_em(ch);
    }
    em * font_size
}

pub(crate) fn estimate_char_width_em(ch: char) -> f64 {
    if ch == ' ' {
        return 0.33;
    }
    if ch == '\t' {
        return 0.66;
    }
    if ch == '_' || ch == '-' {
        return 0.33;
    }
    if matches!(ch, '.' | ',' | ':' | ';') {
        return 0.28;
    }
    if matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | '/') {
        return 0.33;
    }
    if matches!(ch, '+' | '*' | '=' | '\\' | '^' | '|' | '~') {
        return 0.45;
    }
    if ch.is_ascii_digit() {
        return 0.56;
    }
    if ch.is_ascii_uppercase() {
        return match ch {
            'I' => 0.30,
            'W' => 0.85,
            _ => 0.60,
        };
    }
    if ch.is_ascii_lowercase() {
        return match ch {
            'i' | 'l' => 0.28,
            'm' | 'w' => 0.78,
            'k' | 'y' => 0.55,
            _ => 0.43,
        };
    }
    // Punctuation/symbols/unicode: approximate.
    0.60
}
