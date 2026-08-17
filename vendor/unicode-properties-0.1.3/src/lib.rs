// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Query character Unicode properties according to
//! [Unicode Standard Annex #44](https://www.unicode.org/reports/tr44/)
//! and [Unicode Technical Standard #51](https://www.unicode.org/reports/tr51/)
//! rules.
//!
//! Currently we support the `General_Category` property as well as `Emoji` and `Emoji_Component`.
//!
//! Future properties can be added as requested.
//!
//! ```rust
//! use unicode_properties::UnicodeEmoji;
//! use unicode_properties::UnicodeGeneralCategory;
//!
//! let ch = '🦀'; // U+1F980 CRAB
//! let is_emoji = ch.is_emoji_char();
//! let group = ch.general_category_group();
//! println!("{}({:?})", ch, group);
//! println!("The above char {} for use as emoji char.",
//!          if is_emoji { "is recommended" } else { "is not recommended" });
//! ```
//!
//! # Features
//!
//! ## `general-category`
//!
//! Provides the most general classification of a character,
//! based on its primary characteristic.
//!
//! ## `emoji`
//!
//! Provides the emoji character properties of a character.
//!
#![no_std]
#![deny(missing_docs)]

#[rustfmt::skip]
mod tables;

#[cfg(feature = "emoji")]
/// Query the emoji character properties of a character.
pub mod emoji {
    pub use crate::tables::emoji::EmojiStatus;

    /// Query the emoji character properties of a character.
    pub trait UnicodeEmoji: Sized {
        /// Returns the emoji character properties in a status enum.
        fn emoji_status(self) -> EmojiStatus;

        /// Checks whether this character is recommended for use as emoji, i.e. `Emoji=YES`.
        #[allow(clippy::wrong_self_convention)]
        fn is_emoji_char(self) -> bool {
            crate::tables::emoji::is_emoji_status_for_emoji_char(self.emoji_status())
        }

        /// Checks whether this character are used in emoji sequences where they're not
        /// intended for independent, direct input, i.e. `Emoji_Component=YES`.
        #[allow(clippy::wrong_self_convention)]
        fn is_emoji_component(self) -> bool {
            crate::tables::emoji::is_emoji_status_for_emoji_component(self.emoji_status())
        }

        /// Checks whether this character occurs in emoji sequences, i.e. `Emoji=YES | Emoji_Component=YES`
        #[allow(clippy::wrong_self_convention)]
        fn is_emoji_char_or_emoji_component(self) -> bool {
            crate::tables::emoji::is_emoji_status_for_emoji_char_or_emoji_component(
                self.emoji_status(),
            )
        }
    }

    impl UnicodeEmoji for char {
        fn emoji_status(self) -> EmojiStatus {
            crate::tables::emoji::emoji_status(self)
        }
    }

    #[inline]
    /// Checks whether this character is the U+200D ZERO WIDTH JOINER (ZWJ) character.
    ///
    /// It can be used between the elements of a sequence of characters to indicate that
    /// a single glyph should be presented if available.
    pub fn is_zwj(c: char) -> bool {
        c == '\u{200D}'
    }

    #[inline]
    /// Checks whether this character is the U+FE0F VARIATION SELECTOR-16 (VS16) character, used to
    /// request an emoji presentation for an emoji character.
    pub fn is_emoji_presentation_selector(c: char) -> bool {
        c == '\u{FE0F}'
    }

    #[inline]
    /// Checks whether this character is the U+FE0E VARIATION SELECTOR-15 (VS15) character, used to
    /// request a text presentation for an emoji character.
    pub fn is_text_presentation_selector(c: char) -> bool {
        c == '\u{FE0E}'
    }

    #[inline]
    /// Checks whether this character is one of the Regional Indicator characters.
    ///
    /// A pair of REGIONAL INDICATOR symbols is referred to as an emoji_flag_sequence.
    pub fn is_regional_indicator(c: char) -> bool {
        matches!(c, '\u{1F1E6}'..='\u{1F1FF}')
    }

    #[inline]
    /// Checks whether this character is one of the Tag Characters.
    ///
    /// These can be used in indicating variants or extensions of emoji characters.
    pub fn is_tag_character(c: char) -> bool {
        matches!(c, '\u{E0020}'..='\u{E007F}')
    }
}

#[cfg(feature = "general-category")]
/// Query the general category property of a character.
pub mod general_category {
    pub use crate::tables::general_category::{GeneralCategory, GeneralCategoryGroup};

    /// Query the general category property of a character.
    ///
    /// See [General Category Values](https://www.unicode.org/reports/tr44/#General_Category_Values) for more info.
    pub trait UnicodeGeneralCategory: Sized {
        /// Queries the most general classification of a character.
        fn general_category(self) -> GeneralCategory;

        /// Queries the grouping of the most general classification of a character.
        fn general_category_group(self) -> GeneralCategoryGroup {
            crate::tables::general_category::general_category_group(self.general_category())
        }

        /// Queries whether the most general classification of a character belongs to the `LetterCased` group
        ///
        /// The `LetterCased` group includes `LetterUppercase`, `LetterLowercase`, and `LetterTitlecase`
        /// categories, and is a subset of the `Letter` group.
        #[allow(clippy::wrong_self_convention)]
        fn is_letter_cased(self) -> bool {
            crate::tables::general_category::general_category_is_letter_cased(
                self.general_category(),
            )
        }
    }

    impl UnicodeGeneralCategory for char {
        fn general_category(self) -> GeneralCategory {
            crate::tables::general_category::general_category_of_char(self)
        }
    }
}

pub use tables::UNICODE_VERSION;

#[cfg(feature = "emoji")]
#[doc(inline)]
pub use emoji::UnicodeEmoji;

#[cfg(feature = "emoji")]
#[doc(inline)]
pub use emoji::EmojiStatus;

#[cfg(feature = "general-category")]
#[doc(inline)]
pub use general_category::GeneralCategory;

#[cfg(feature = "general-category")]
#[doc(inline)]
pub use general_category::GeneralCategoryGroup;

#[cfg(feature = "general-category")]
#[doc(inline)]
pub use general_category::UnicodeGeneralCategory;
