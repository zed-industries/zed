//! **heck** is a case conversion library.
//!
//! This library exists to provide case conversion between common cases like
//! CamelCase and snake_case. It is intended to be unicode aware, internally
//! consistent, and reasonably well performing.
//!
//! ## Definition of a word boundary
//!
//! Word boundaries are defined as the "unicode words" defined in the
//! `unicode_segmentation` library, as well as within those words in this
//! manner:
//!
//! 1. All underscore characters are considered word boundaries.
//! 2. If an uppercase character is followed by lowercase letters, a word
//! boundary is considered to be just prior to that uppercase character.
//! 3. If multiple uppercase characters are consecutive, they are considered to
//! be within a single word, except that the last will be part of the next word
//! if it is followed by lowercase characters (see rule 2).
//!
//! That is, "HelloWorld" is segmented `Hello|World` whereas "XMLHttpRequest" is
//! segmented `XML|Http|Request`.
//!
//! Characters not within words (such as spaces, punctuations, and underscores)
//! are not included in the output string except as they are a part of the case
//! being converted to. Multiple adjacent word boundaries (such as a series of
//! underscores) are folded into one. ("hello__world" in snake case is therefore
//! "hello_world", not the exact same string). Leading or trailing word boundary
//! indicators are dropped, except insofar as CamelCase capitalizes the first
//! word.
//!
//! ### Cases contained in this library:
//!
//! 1. UpperCamelCase
//! 2. lowerCamelCase
//! 3. snake_case
//! 4. kebab-case
//! 5. SHOUTY_SNAKE_CASE
//! 6. Title Case
//! 7. SHOUTY-KEBAB-CASE
//! 8. Train-Case
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod kebab;
mod lower_camel;
mod shouty_kebab;
mod shouty_snake;
mod snake;
mod title;
mod train;
mod upper_camel;

pub use kebab::{AsKebabCase, ToKebabCase};
pub use lower_camel::{AsLowerCamelCase, ToLowerCamelCase};
pub use shouty_kebab::{AsShoutyKebabCase, ToShoutyKebabCase};
pub use shouty_snake::{
    AsShoutySnakeCase, AsShoutySnakeCase as AsShoutySnekCase, ToShoutySnakeCase, ToShoutySnekCase,
};
pub use snake::{AsSnakeCase, AsSnakeCase as AsSnekCase, ToSnakeCase, ToSnekCase};
pub use title::{AsTitleCase, ToTitleCase};
pub use train::{AsTrainCase, ToTrainCase};
pub use upper_camel::{
    AsUpperCamelCase, AsUpperCamelCase as AsPascalCase, ToPascalCase, ToUpperCamelCase,
};

use std::fmt;

#[cfg(feature = "unicode")]
fn get_iterator(s: &str) -> unicode_segmentation::UnicodeWords {
    use unicode_segmentation::UnicodeSegmentation;
    s.unicode_words()
}
#[cfg(not(feature = "unicode"))]
fn get_iterator(s: &str) -> impl Iterator<Item = &str> {
    s.split(|letter: char| !letter.is_ascii_alphanumeric())
}

fn transform<F, G>(
    s: &str,
    mut with_word: F,
    mut boundary: G,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
    F: FnMut(&str, &mut fmt::Formatter) -> fmt::Result,
    G: FnMut(&mut fmt::Formatter) -> fmt::Result,
{
    /// Tracks the current 'mode' of the transformation algorithm as it scans
    /// the input string.
    ///
    /// The mode is a tri-state which tracks the case of the last cased
    /// character of the current word. If there is no cased character
    /// (either lowercase or uppercase) since the previous word boundary,
    /// than the mode is `Boundary`. If the last cased character is lowercase,
    /// then the mode is `Lowercase`. Othertherwise, the mode is
    /// `Uppercase`.
    #[derive(Clone, Copy, PartialEq)]
    enum WordMode {
        /// There have been no lowercase or uppercase characters in the current
        /// word.
        Boundary,
        /// The previous cased character in the current word is lowercase.
        Lowercase,
        /// The previous cased character in the current word is uppercase.
        Uppercase,
    }

    let mut first_word = true;

    for word in get_iterator(s) {
        let mut char_indices = word.char_indices().peekable();
        let mut init = 0;
        let mut mode = WordMode::Boundary;

        while let Some((i, c)) = char_indices.next() {
            // Skip underscore characters
            if c == '_' {
                if init == i {
                    init += 1;
                }
                continue;
            }

            if let Some(&(next_i, next)) = char_indices.peek() {
                // The mode including the current character, assuming the
                // current character does not result in a word boundary.
                let next_mode = if c.is_lowercase() {
                    WordMode::Lowercase
                } else if c.is_uppercase() {
                    WordMode::Uppercase
                } else {
                    mode
                };

                // Word boundary after if next is underscore or current is
                // not uppercase and next is uppercase
                if next == '_' || (next_mode == WordMode::Lowercase && next.is_uppercase()) {
                    if !first_word {
                        boundary(f)?;
                    }
                    with_word(&word[init..next_i], f)?;
                    first_word = false;
                    init = next_i;
                    mode = WordMode::Boundary;

                // Otherwise if current and previous are uppercase and next
                // is lowercase, word boundary before
                } else if mode == WordMode::Uppercase && c.is_uppercase() && next.is_lowercase() {
                    if !first_word {
                        boundary(f)?;
                    } else {
                        first_word = false;
                    }
                    with_word(&word[init..i], f)?;
                    init = i;
                    mode = WordMode::Boundary;

                // Otherwise no word boundary, just update the mode
                } else {
                    mode = next_mode;
                }
            } else {
                // Collect trailing characters as a word
                if !first_word {
                    boundary(f)?;
                } else {
                    first_word = false;
                }
                with_word(&word[init..], f)?;
                break;
            }
        }
    }

    Ok(())
}

fn lowercase(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == 'Σ' && chars.peek().is_none() {
            write!(f, "ς")?;
        } else {
            write!(f, "{}", c.to_lowercase())?;
        }
    }

    Ok(())
}

fn uppercase(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    for c in s.chars() {
        write!(f, "{}", c.to_uppercase())?;
    }

    Ok(())
}

fn capitalize(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let mut char_indices = s.char_indices();
    if let Some((_, c)) = char_indices.next() {
        write!(f, "{}", c.to_uppercase())?;
        if let Some((i, _)) = char_indices.next() {
            lowercase(&s[i..], f)?;
        }
    }

    Ok(())
}
