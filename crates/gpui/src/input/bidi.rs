use unicode_bidi::{BidiClass, bidi_class};

/// Text direction for bidirectional text support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// Left-to-right text direction (default for Latin, Greek, Cyrillic, etc.)
    #[default]
    Ltr,
    /// Right-to-left text direction (for Arabic, Hebrew, etc.)
    Rtl,
}

impl TextDirection {
    /// Returns true if this is left-to-right direction.
    pub fn is_ltr(self) -> bool {
        matches!(self, TextDirection::Ltr)
    }

    /// Returns true if this is right-to-left direction.
    pub fn is_rtl(self) -> bool {
        matches!(self, TextDirection::Rtl)
    }
}

/// Detects the base direction of text using the first strong directional character.
///
/// This follows the Unicode Bidirectional Algorithm (UBA) rule P2/P3:
/// - Find the first character with a strong directional type (L, R, or AL)
/// - If it's L, the paragraph direction is LTR
/// - If it's R or AL, the paragraph direction is RTL
/// - If no strong character is found, defaults to LTR
///
/// # Examples
///
/// ```ignore
/// use gpui::input::bidi::detect_base_direction;
///
/// // English text is LTR
/// assert!(detect_base_direction("Hello world").is_ltr());
///
/// // Arabic text is RTL
/// assert!(detect_base_direction("مرحبا").is_rtl());
///
/// // Hebrew text is RTL
/// assert!(detect_base_direction("שלום").is_rtl());
///
/// // Mixed text uses first strong character
/// assert!(detect_base_direction("Hello مرحبا").is_ltr());
/// assert!(detect_base_direction("مرحبا Hello").is_rtl());
///
/// // Empty or neutral-only text defaults to LTR
/// assert!(detect_base_direction("").is_ltr());
/// assert!(detect_base_direction("123").is_ltr());
/// ```
pub fn detect_base_direction(text: &str) -> TextDirection {
    for c in text.chars() {
        match bidi_class(c) {
            BidiClass::L => return TextDirection::Ltr,
            BidiClass::R | BidiClass::AL => return TextDirection::Rtl,
            _ => continue,
        }
    }
    TextDirection::Ltr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(detect_base_direction(""), TextDirection::Ltr);
    }

    #[test]
    fn test_whitespace_only() {
        assert_eq!(detect_base_direction("   "), TextDirection::Ltr);
    }

    #[test]
    fn test_numbers_only() {
        assert_eq!(detect_base_direction("12345"), TextDirection::Ltr);
    }

    #[test]
    fn test_punctuation_only() {
        assert_eq!(detect_base_direction("!@#$%"), TextDirection::Ltr);
    }

    #[test]
    fn test_latin_text() {
        assert_eq!(detect_base_direction("Hello world"), TextDirection::Ltr);
    }

    #[test]
    fn test_arabic_text() {
        assert_eq!(detect_base_direction("مرحبا بالعالم"), TextDirection::Rtl);
    }

    #[test]
    fn test_hebrew_text() {
        assert_eq!(detect_base_direction("שלום עולם"), TextDirection::Rtl);
    }

    #[test]
    fn test_mixed_ltr_first() {
        assert_eq!(detect_base_direction("Hello مرحبا"), TextDirection::Ltr);
    }

    #[test]
    fn test_mixed_rtl_first() {
        assert_eq!(detect_base_direction("مرحبا Hello"), TextDirection::Rtl);
    }

    #[test]
    fn test_numbers_before_arabic() {
        assert_eq!(detect_base_direction("123 مرحبا"), TextDirection::Rtl);
    }

    #[test]
    fn test_numbers_before_latin() {
        assert_eq!(detect_base_direction("123 Hello"), TextDirection::Ltr);
    }

    #[test]
    fn test_punctuation_before_hebrew() {
        assert_eq!(detect_base_direction("... שלום"), TextDirection::Rtl);
    }

    #[test]
    fn test_greek_text() {
        assert_eq!(detect_base_direction("Γειά σου κόσμε"), TextDirection::Ltr);
    }

    #[test]
    fn test_cyrillic_text() {
        assert_eq!(detect_base_direction("Привет мир"), TextDirection::Ltr);
    }

    #[test]
    fn test_chinese_text() {
        assert_eq!(detect_base_direction("你好世界"), TextDirection::Ltr);
    }

    #[test]
    fn test_japanese_text() {
        assert_eq!(detect_base_direction("こんにちは"), TextDirection::Ltr);
    }

    #[test]
    fn test_direction_is_ltr() {
        assert!(TextDirection::Ltr.is_ltr());
        assert!(!TextDirection::Rtl.is_ltr());
    }

    #[test]
    fn test_direction_is_rtl() {
        assert!(TextDirection::Rtl.is_rtl());
        assert!(!TextDirection::Ltr.is_rtl());
    }

    #[test]
    fn test_direction_default() {
        assert_eq!(TextDirection::default(), TextDirection::Ltr);
    }
}
