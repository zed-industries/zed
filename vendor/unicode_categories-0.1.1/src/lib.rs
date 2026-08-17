//! `unicode_categories` is a crate that adds extensions to the
//! `char` primitive type that allow for a char to be queried
//! about whether or not it belongs to a particular Unicode category.
//!
//! These extensions exist on the `UnicodeCategories` trait, so
//! by importing it the extensions will be active on all chars:
//!
//! ```
//! use unicode_categories::UnicodeCategories;
//!
//! assert!('a'.is_letter_lowercase());
//! assert!('A'.is_letter_uppercase());
//! assert!('\n'.is_other_control());
//! ```
//!
//! `UnicodeCategories` is the only item contained exported
//! by this crate and contains all of methods that allow
//! for category queries.

mod tables;

pub trait UnicodeCategories : Sized + Copy {

    /// Returns `true` if this value is a member
    /// of the "Other, Control" (Cc) category.
    fn is_other_control(self) -> bool;

    /// Returns `true` if this value is a member
    /// of the "Other, Format" (Cf) category.
    fn is_other_format(self) -> bool;

    /// Returns true if this value is a member
    /// of the "Other, Private Use" (Co) category.
    fn is_other_private_use(self) -> bool;

    /// Returns true if this value is a member
    /// of the "Letter, Lowercase" (Ll) category.
    fn is_letter_lowercase(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Letter, Modifier" (Lm) category.
    fn is_letter_modifier(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Letter, Other" (Lo) category.
    fn is_letter_other(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Letter, Titlecase" (Lt) category.
    fn is_letter_titlecase(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Letter, Uppercase" (Lu) category.
    fn is_letter_uppercase(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Mark, Spacing Combining" (Mc) category.
    fn is_mark_spacing_combining(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Mark, Enclosing" (Me) category.
    fn is_mark_enclosing(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Mark, Nonspacing" (Mn) category.
    fn is_mark_nonspacing(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Number, Decimal Digit" (Nd) category.
    fn is_number_decimal_digit(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Number, Letter" (Nl) category.
    fn is_number_letter(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Number, Other" (No) category.
    fn is_number_other(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Connector" (Pc) category.
    fn is_punctuation_connector(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Dash" (Pd) category.
    fn is_punctuation_dash(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Close" (Pe) category.
    fn is_punctuation_close(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Final Quote" (Pf) category.
    fn is_punctuation_final_quote(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Initial Quote" (Pi) category.
    fn is_punctuation_initial_quote(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Other" (Po) category.
    fn is_punctuation_other(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Punctuation, Open" (Ps) category.
    fn is_punctuation_open(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Symbol, Currency" (Sc) category.
    fn is_symbol_currency(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Symbol, Modifier" (Sk) category.
    fn is_symbol_modifier(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Symbol, Math" (Sm) category.
    fn is_symbol_math(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Symbol, Other" (So) category.
    fn is_symbol_other(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Separator, Line" (Zl) category.
    fn is_separator_line(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Separator, Paragraph" (Zp) category.
    fn is_separator_paragraph(self) -> bool;

    /// Returns true if this value is a member of
    /// the "Separator, Space" (Zs) category.
    fn is_separator_space(self) -> bool;

    /// Returns true if this value is a member of
    /// a "Other" category: Cc, Cf, Cn, or Co.
    /// Surrogates cannot be `chars` in Rust, so
    /// they are not included.
    #[inline]
    fn is_other(self) -> bool {
        self.is_other_control()
            || self.is_other_format()
            || self.is_other_private_use()
    }

    /// Returns true if this value is a member of
    /// a "Letter" category: Lc, Ll, Lm, Lo, Lt, or Lu.
    #[inline]
    fn is_letter(self) -> bool {
        self.is_letter_lowercase()
            || self.is_letter_modifier()
            || self.is_letter_other()
            || self.is_letter_titlecase()
            || self.is_letter_uppercase()
    }

    /// Returns true if this value is a member of a
    /// "Mark" category: Mc, Me, or Mn.
    #[inline]
    fn is_mark(self) -> bool {
        self.is_mark_spacing_combining()
            || self.is_mark_enclosing()
            || self.is_mark_nonspacing()
    }

    /// Returns true if this value is a member of a
    /// "Number" category: Nd, Nl, or No.
    #[inline]
    fn is_number(self) -> bool {
        self.is_number_decimal_digit()
            || self.is_number_letter()
            || self.is_number_other()
    }

    /// Returns true if this value is a member of a
    /// "Punctuation" category: Pc, Pd, Pe, Pf, Pi, Po, or Ps.
    #[inline]
    fn is_punctuation(self) -> bool {
        self.is_punctuation_connector()
            || self.is_punctuation_dash()
            || self.is_punctuation_close()
            || self.is_punctuation_close()
            || self.is_punctuation_final_quote()
            || self.is_punctuation_initial_quote()
            || self.is_punctuation_other()
            || self.is_punctuation_open()
    }

    /// Returns true if this value is a member of a
    /// "Symbol" category: Sc, Sk, Sm, or So.
    #[inline]
    fn is_symbol(self) -> bool {
        self.is_symbol_currency()
            || self.is_symbol_modifier()
            || self.is_symbol_math()
            || self.is_symbol_other()
    }

    /// Returns true if this value is a member of a
    /// "Separator" category: Zl, Zp, or Zs.
    #[inline]
    fn is_separator(self) -> bool {
        self.is_separator_line()
            || self.is_separator_paragraph()
            || self.is_separator_space()
    }
}

fn table_binary_search(target: char, table: &'static [char]) -> bool {
    table.binary_search(&target).is_ok()
}

impl UnicodeCategories for char {
    #[inline]
    fn is_other_control(self) -> bool {
        table_binary_search(self, tables::OTHER_CONTROL)
    }

    #[inline]
    fn is_other_format(self) -> bool {
        table_binary_search(self, tables::OTHER_FORMAT)
    }

    #[inline]
    fn is_other_private_use(self) -> bool {
        match self {
            // Private Use
            '\u{E000}'...'\u{F8FF}' => true,
            // Plane 15, Private Use
            '\u{F0000}'...'\u{FFFFD}' => true,
            // Plane 16, private Use
            '\u{100000}'...'\u{10FFFD}' => true,
            _ => table_binary_search(self, tables::OTHER_PRIVATE_USE)
        }
    }

    #[inline]
    fn is_letter_lowercase(self) -> bool {
        table_binary_search(self, tables::LETTER_LOWERCASED)
    }

    #[inline]
    fn is_letter_modifier(self) -> bool {
        table_binary_search(self, tables::LETTER_MODIFIER)
    }

    #[inline]
    fn is_letter_other(self) -> bool {
        match self {
            // CJK Ideograph Extension A
            '\u{3400}'...'\u{4DB5}' => true,
            // CJK Ideograph
            '\u{4E00}'...'\u{9FD5}' => true,
            // Hangul Syllable
            '\u{AC00}'...'\u{D7A3}' => true,
            // Tangut Ideograph
            '\u{17000}'...'\u{187EC}' => true,
            // CJK Ideograph Extension B
            '\u{20000}'...'\u{2A6D6}' => true,
            // CJK Ideograph Extension C
            '\u{2A700}'...'\u{2B734}' => true,
            // CJK Ideograph Extension D
            '\u{2B740}'...'\u{2B81D}' => true,
            // CJK Ideograph Extension E
            '\u{2B820}'...'\u{2CEA1}' => true,
            _ => table_binary_search(self, tables::LETTER_OTHER)
        }
    }

    #[inline]
    fn is_letter_titlecase(self) -> bool {
        table_binary_search(self, tables::LETTER_TITLECASE)
    }

    #[inline]
    fn is_letter_uppercase(self) -> bool {
        table_binary_search(self, tables::LETTER_UPPERCASE)
    }

    #[inline]
    fn is_mark_spacing_combining(self) -> bool {
        table_binary_search(self, tables::MARK_SPACE_COMBINING)
    }

    #[inline]
    fn is_mark_enclosing(self) -> bool {
        table_binary_search(self, tables::MARK_ENCLOSING)
    }

    #[inline]
    fn is_mark_nonspacing(self) -> bool {
        table_binary_search(self, tables::MARK_NONSPACING)
    }

    #[inline]
    fn is_number_decimal_digit(self) -> bool {
        table_binary_search(self, tables::NUMBER_DECIMAL_DIGIT)
    }

    #[inline]
    fn is_number_letter(self) -> bool {
        table_binary_search(self, tables::NUMBER_LETTER)
    }

    #[inline]
    fn is_number_other(self) -> bool {
        table_binary_search(self, tables::NUMBER_OTHER)
    }

    #[inline]
    fn is_punctuation_connector(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_CONNECTOR)
    }

    #[inline]
    fn is_punctuation_dash(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_DASH)
    }

    #[inline]
    fn is_punctuation_close(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_CLOSE)
    }

    #[inline]
    fn is_punctuation_final_quote(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_FINAL_QUOTE)
    }

    #[inline]
    fn is_punctuation_initial_quote(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_INITIAL_QUOTE)
    }

    #[inline]
    fn is_punctuation_other(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_OTHER)
    }

    #[inline]
    fn is_punctuation_open(self) -> bool {
        table_binary_search(self, tables::PUNCTUATION_OPEN)
    }

    #[inline]
    fn is_symbol_currency(self) -> bool {
        table_binary_search(self, tables::SYMBOL_CURRENCY)
    }

    #[inline]
    fn is_symbol_modifier(self) -> bool {
        table_binary_search(self, tables::SYMBOL_MODIFIER)
    }

    #[inline]
    fn is_symbol_math(self) -> bool {
        table_binary_search(self, tables::SYMBOL_MATH)
    }

    #[inline]
    fn is_symbol_other(self) -> bool {
        table_binary_search(self, tables::SYMBOL_OTHER)
    }

    #[inline]
    fn is_separator_line(self) -> bool {
        table_binary_search(self, tables::SEPARATOR_LINE)
    }

    #[inline]
    fn is_separator_paragraph(self) -> bool {
        table_binary_search(self, tables::SEPARATOR_PARAGRAPH)
    }

    #[inline]
    fn is_separator_space(self) -> bool {
        table_binary_search(self, tables::SEPARATOR_SPACE)
    }
}

#[cfg(test)]
mod tests {
    use super::UnicodeCategories;

    #[test]
    fn is_other_control() {
        assert!('\0'.is_other_control());
        assert!('\u{007F}'.is_other_control());
        assert!(!'f'.is_other_control());
    }

    #[test]
    fn is_other_format() {
        assert!('؁'.is_other_format());
        assert!(!'0'.is_other_format());
    }

    #[test]
    fn is_other_private_use() {
        assert!('\u{F8FF}'.is_other_private_use());
        assert!(!'n'.is_other_private_use())
    }

    #[test]
    fn is_letter_lowercase() {
        assert!('q'.is_letter_lowercase());
        assert!(!'N'.is_letter_lowercase());
    }

    #[test]
    fn is_letter_modifier() {
        assert!('ˢ'.is_letter_modifier());
        assert!(!'m'.is_letter_modifier());
    }

    #[test]
    fn is_letter_range() {
        assert!('界'.is_letter_other());
    }
}
