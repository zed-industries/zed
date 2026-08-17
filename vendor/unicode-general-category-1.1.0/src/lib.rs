//! Look up the general category for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_general_category::{get_general_category, GeneralCategory};
//!
//! assert_eq!(get_general_category('A'), GeneralCategory::UppercaseLetter);
//! ```

#![no_std]

mod category;
mod tables;
pub use category::get_general_category;
pub use tables::GeneralCategory;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-general-category was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (16, 0, 0);

impl GeneralCategory {
    /// Returns the general category abbreviation.
    /// See [Unicode General Category Values](https://www.unicode.org/reports/tr44/tr44-30.html#General_Category_Values)
    /// for the list of all general categories.
    pub fn abbreviation(&self) -> &'static str {
        match self {
            GeneralCategory::ClosePunctuation => "Pe",
            GeneralCategory::ConnectorPunctuation => "Pc",
            GeneralCategory::Control => "Cc",
            GeneralCategory::CurrencySymbol => "Sc",
            GeneralCategory::DashPunctuation => "Pd",
            GeneralCategory::DecimalNumber => "Nd",
            GeneralCategory::EnclosingMark => "Me",
            GeneralCategory::FinalPunctuation => "Pf",
            GeneralCategory::Format => "Cf",
            GeneralCategory::InitialPunctuation => "Pi",
            GeneralCategory::LetterNumber => "Nl",
            GeneralCategory::LineSeparator => "Zl",
            GeneralCategory::LowercaseLetter => "Ll",
            GeneralCategory::MathSymbol => "Sm",
            GeneralCategory::ModifierLetter => "Lm",
            GeneralCategory::ModifierSymbol => "Sk",
            GeneralCategory::NonspacingMark => "Mn",
            GeneralCategory::OpenPunctuation => "Ps",
            GeneralCategory::OtherLetter => "Lo",
            GeneralCategory::OtherNumber => "No",
            GeneralCategory::OtherPunctuation => "Po",
            GeneralCategory::OtherSymbol => "So",
            GeneralCategory::ParagraphSeparator => "Zp",
            GeneralCategory::PrivateUse => "Co",
            GeneralCategory::SpaceSeparator => "Zs",
            GeneralCategory::SpacingMark => "Mc",
            GeneralCategory::Surrogate => "Cs",
            GeneralCategory::TitlecaseLetter => "Lt",
            GeneralCategory::Unassigned => "Cn",
            GeneralCategory::UppercaseLetter => "Lu",
        }
    }
}

#[cfg(test)]
mod test {
    use super::{get_general_category, GeneralCategory};

    #[test]
    fn test_get_category() {
        assert_eq!(get_general_category('a'), GeneralCategory::LowercaseLetter);
        assert_eq!(get_general_category('.'), GeneralCategory::OtherPunctuation);
        assert_eq!(get_general_category('カ'), GeneralCategory::OtherLetter);
        assert_eq!(get_general_category('🦳'), GeneralCategory::OtherSymbol);
    }

    #[test]
    fn test_abbreviation() {
        let cat = get_general_category('a');
        assert_eq!(cat.abbreviation(), "Ll");
    }
}
