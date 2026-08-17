//! Convert to and from different string cases.
//!
//! # Basic Usage
//!
//! The most common use of this crate is to just convert a string into a
//! particular case, like snake, camel, or kebab.  You can use the [`ccase`]
//! macro to convert most string types into the new case.
//! ```
//! use convert_case::ccase;
//!
//! let s = "myVarName";
//! assert_eq!(ccase!(snake, s),  "my_var_name");
//! assert_eq!(ccase!(kebab, s),  "my-var-name");
//! assert_eq!(ccase!(pascal, s), "MyVarName");
//! assert_eq!(ccase!(title, s),  "My Var Name");
//! ```
//!
//! For more explicit conversion, import the [`Casing`] trait which adds methods
//! to string types that perform the conversion based on a variant of the [`Case`] enum.
//! ```
//! use convert_case::{Case, Casing};
//!
//! let s = "myVarName";
//! assert_eq!(s.to_case(Case::Snake),  "my_var_name");
//! assert_eq!(s.to_case(Case::Kebab),  "my-var-name");
//! assert_eq!(s.to_case(Case::Pascal), "MyVarName");
//! assert_eq!(s.to_case(Case::Title),  "My Var Name");
//! ```
//!
//! For a full list of cases, see [`Case`].
//!
//! # Splitting Conditions
//!
//! Case conversion starts by splitting a single identifier into a list of words.  The
//! condition for when to split and how to perform the split is defined by a [`Boundary`].
//!
//! By default, [`ccase`] and [`Casing::to_case`] will split identifiers at all locations
//! based on a list of [default boundaries](Boundary::defaults).
//!
//! ```
//! use convert_case::ccase;
//!
//! assert_eq!(ccase!(pascal, "hyphens-and_underscores"), "HyphensAndUnderscores");
//! assert_eq!(ccase!(pascal, "lowerUpper space"), "LowerUpperSpace");
//! assert_eq!(ccase!(snake, "HTTPRequest"), "http_request");
//! assert_eq!(ccase!(snake, "vector4d"), "vector_4_d")
//! ```
//!
//! Associated with each case is a [list of boundaries](Case::boundaries) that can be
//! used to split identifiers instead of the defaults.  We can use the following notation
//! with the [`ccase`] macro.
//! ```
//! use convert_case::ccase;
//!
//! assert_eq!(
//!     ccase!(title, "1999-25-01_family_photo.png"),
//!     "1999 25 01 Family Photo.png",
//! );
//! assert_eq!(
//!     ccase!(snake -> title, "1999-25-01_family_photo.png"),
//!     "1999-25-01 Family Photo.png",
//! );
//! ```
//! Or we can use the [`from_case`](Casing::from_case) method on `Casing` before calling
//! `to_case`.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!(
//!     "John McCarthy".to_case(Case::Snake),
//!     "john_mc_carthy",
//! );
//! assert_eq!(
//!     "John McCarthy".from_case(Case::Title).to_case(Case::Snake),
//!     "john_mccarthy",
//! );
//! ```
//! You can remove boundaries from the list of defaults with [`Casing::remove_boundaries`].  See
//! the list of constants on [`Boundary`] for splitting conditions.
//! ```
//! use convert_case::{Boundary, Case, Casing};
//!
//! assert_eq!(
//!     "Vector4D".remove_boundaries(&[Boundary::DigitUpper]).to_case(Case::Snake),
//!     "vector_4d",
//! );
//! ```
//!
//! # Other Behavior
//!
//! ### Acronyms
//! Part of the default list of boundaries is [`acronym`](Boundary::Acronym) which
//! will detect two capital letters followed by a lowercase letter.  But there is no memory
//! that the word itself was parsed considered an acronym.
//! ```
//! # use convert_case::ccase;
//! assert_eq!(ccase!(snake, "HTTPRequest"), "http_request");
//! assert_eq!(ccase!(pascal, "HTTPRequest"), "HttpRequest");
//! ```
//!
//! ### Digits
//! The default list of boundaries includes splitting before and after digits.
//! ```
//! # use convert_case::ccase;
//! assert_eq!(ccase!(title, "word2vec"), "Word 2 Vec");
//! ```
//!
//! ### Unicode
//! Conversion works on _graphemes_ as defined by the
//! [`unicode_segmentation`](unicode_segmentation::UnicodeSegmentation::graphemes) library.
//! This means that transforming letters to lowercase or uppercase works on all unicode
//! characters, which also means that the number of characters isn't necessarily the
//! same after conversion.
//! ```
//! # use convert_case::ccase;
//! assert_eq!(ccase!(kebab, "GranatÄpfel"), "granat-äpfel");
//! assert_eq!(ccase!(title, "ПЕРСПЕКТИВА24"), "Перспектива 24");
//! assert_eq!(ccase!(lower, "ὈΔΥΣΣΕΎΣ"), "ὀδυσσεύς");
//! ```
//!
//! ### Symbols
//! All symbols that are not part of the default boundary conditions are ignored.  This
//! is any symbol that isn't an underscore, hyphen, or space.
//! ```
//! # use convert_case::ccase;
//! assert_eq!(ccase!(snake, "dots.arent.default"), "dots.arent.default");
//! assert_eq!(ccase!(pascal, "path/to/file_name"), "Path/to/fileName");
//! assert_eq!(ccase!(pascal, "list\nof\nwords"),   "List\nof\nwords");
//! ```
//!
//! ### Delimiters
//! Leading, trailing, and duplicate delimiters create empty words.
//! This propagates and the converted string will share the behavior.  **This can cause
//! unintuitive behavior for patterns that transform words based on index.**
//! ```
//! # use convert_case::ccase;
//! assert_eq!(ccase!(constant, "_leading_score"), "_LEADING_SCORE");
//! assert_eq!(ccase!(ada, "trailing-dash-"), "Trailing_Dash_");
//! assert_eq!(ccase!(train, "duplicate----hyphens"), "Duplicate----Hyphens");
//!
//! // not what you might expect!
//! assert_eq!(ccase!(camel, "_empty__first_word"), "EmptyFirstWord");
//! ```
//! To remove empty words before joining, you can call `remove_empty` from the
//! `Casing` trait before finishing the conversion.
//! ```
//! # use convert_case::{Casing, Case};
//! assert_eq!(
//!     "_empty__first_word".remove_empty().to_case(Case::Camel),
//!     "emptyFirstWord"
//! )
//! ```
//!
//! # Customizing Behavior
//!
//! Case conversion takes place in three steps:
//! 1. Splitting the identifier into a list of words
//! 2. Mutating the letter case of graphemes within each word
//! 3. Joining the words back into an identifier using a delimiter
//!
//! Those are defined by boundaries, patterns, and delimiters respectively.  Graphically:
//!
//! ```md
//! Identifier        Identifier'
//!     |                 ^
//!     | boundaries      | delimiter
//!     V                 |
//!   Words ----------> Words'
//!           patterns
//! ```
//!
//! ## Patterns
//!
//! How to change the case of letters across a list of words is called a _pattern_.
//! A pattern is a function that when passed a `&[&str]`, produces a
//! `Vec<String>`.  The [`Pattern`] enum encapsulates the common transformations
//! used across all cases.  Although custom functions can be supplied with the
//! [`Custom`](Pattern::Custom) variant.
//!
//! ## Boundaries
//!
//! The condition for splitting at part of an identifier, where to perform
//! the split, and if any characters are removed are defined by [boundaries](Boundary).
//! By default, identifiers are split based on [`Boundary::defaults`].  This list
//! contains word boundaries that you would likely see after creating a multi-word
//! identifier of typical cases.
//!
//! Custom boundary conditions can also be created.  Commonly, you might split based on some
//! character or list of characters.  The [`separator`] macro builds
//! a boundary that splits on the presence of a string, and then removes the string
//! while producing the list of words.
//!
//! You can also use [`Boundary::Custom`] to explicitly define boundary
//! conditions.  If you actually need to create a
//! boundary condition from scratch, you should file an issue to let the author know
//! how you used it.  I'm not certain what other boundary condition would be helpful.
//!
//! ## Cases
//!
//! A case is defined by a list of boundaries, a pattern, and a _delimiter_: the string to
//! intersperse between words before concatenation. [`Case::Custom`] is a struct enum variant with
//! exactly those three fields.  You could create your own case like so.
//! ```
//! use convert_case::{Case, Casing, separator, Pattern};
//!
//! let dot_case = Case::Custom {
//!     boundaries: &[separator!(".")],
//!     pattern: Pattern::Lowercase,
//!     delimiter: ".",
//! };
//!
//! assert_eq!("AnimalFactoryFactory".to_case(dot_case), "animal.factory.factory");
//!
//! assert_eq!(
//!     "pd.options.mode.copy_on_write"
//!         .from_case(dot_case)
//!         .to_case(Case::Title),
//!     "Pd Options Mode Copy_on_write",
//! )
//! ```
//!
//! ## Converter
//!
//! Case conversion with `convert_case` allows using attributes from two cases.  From
//! the first case is how you split the identifier (the _from_ case), and
//! from the second is how to mutate and join the words (the _to_ case.)  The
//! [`Converter`] is used to define the _conversion_ process, not a case directly.
//!
//! It has the same fields as case, but is exposed via a builder interface
//! and can be used to apply a conversion on a string directly, without
//! specifying all the parameters at the time of conversion.
//!
//! In the below example, we build a converter that maps the double colon
//! delimited module path in rust into a series of file directories.
//!
//! ```
//! use convert_case::{Case, Converter, separator};
//!
//! let modules_into_path = Converter::new()
//!     .set_boundaries(&[separator!("::")])
//!     .set_delimiter("/");
//!
//! assert_eq!(
//!     modules_into_path.convert("std::os::unix"),
//!     "std/os/unix",
//! );
//! ```
//!
//! # Associated Projects
//!
//! ## Rust library `convert_case_extras`
//!
//! Some extra utilities for convert_case that don't need to be in the main library.
//! You can read more here: [`convert_case_extras`](https://docs.rs/convert_case_extras).
//!
//! ## stringcase.org
//!
//! While developing `convert_case`, the author became fascinated in the naming conventions
//! used for cases as well as different implementations for conversion.  On [stringcase.org](https://stringcase.org)
//! is documentation of the history of naming conventions, a catalogue of case conversion tools,
//! and a more rigorous definition of what it means to "convert the case of an identifier."
//!
//! ## Command Line Utility `ccase`
//!
//! `convert_case` was originally developed for the purposes of a command line utility
//! for converting the case of strings and filenames.  You can check out
//! [`ccase` on Github](https://github.com/rutrum/ccase).
#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::string::String;

mod boundary;
mod case;
mod converter;
mod pattern;

pub use boundary::{split, Boundary};
pub use case::Case;
pub use converter::Converter;
pub use pattern::Pattern;

/// Describes items that can be converted into a case.  This trait is used
/// in conjunction with the [`StateConverter`] struct which is returned from a couple
/// methods on `Casing`.
pub trait Casing<T: AsRef<str>> {
    /// Convert the string into the given case.  It will reference `self` and create a new
    /// `String` with the same pattern and delimiter as `case`.  It will split on boundaries
    /// defined at [`Boundary::defaults()`].
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "Tetronimo piece border".to_case(Case::Kebab),
    ///     "tetronimo-piece-border",
    /// );
    /// ```
    fn to_case(&self, case: Case) -> String;

    /// Start the case conversion by storing the boundaries associated with the given case.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "2020-08-10 Dannie Birthday"
    ///         .from_case(Case::Title)
    ///         .to_case(Case::Snake),
    ///     "2020-08-10_dannie_birthday",
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn from_case(&self, case: Case) -> StateConverter<'_, T>;

    /// Creates a `StateConverter` struct initialized with the boundaries provided.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "E1M1 Hangar"
    ///         .set_boundaries(&[Boundary::DigitUpper, Boundary::Space])
    ///         .to_case(Case::Snake),
    ///     "e1_m1_hangar",
    /// );
    /// ```
    fn set_boundaries(&self, bs: &[Boundary]) -> StateConverter<'_, T>;

    /// Creates a `StateConverter` struct initialized without the boundaries
    /// provided.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "2d_transformation",
    ///     "2dTransformation"
    ///         .remove_boundaries(&Boundary::digits())
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    fn remove_boundaries(&self, bs: &[Boundary]) -> StateConverter<'_, T>;

    /// Creates a `StateConverter` with the `RemoveEmpty` pattern prepended.
    /// This filters out empty words before conversion, useful when splitting
    /// produces empty words from leading, trailing, and duplicate delimiters.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "--leading-delims"
    ///         .from_case(Case::Kebab)
    ///         .remove_empty()
    ///         .to_case(Case::Camel),
    ///     "leadingDelims",
    /// );
    /// ```
    fn remove_empty(&self) -> StateConverter<'_, T>;
}

impl<T: AsRef<str>> Casing<T> for T {
    fn to_case(&self, case: Case) -> String {
        StateConverter::new(self).to_case(case)
    }

    fn set_boundaries(&self, bs: &[Boundary]) -> StateConverter<'_, T> {
        StateConverter::new(self).set_boundaries(bs)
    }

    fn remove_boundaries(&self, bs: &[Boundary]) -> StateConverter<'_, T> {
        StateConverter::new(self).remove_boundaries(bs)
    }

    fn from_case(&self, case: Case) -> StateConverter<'_, T> {
        StateConverter::new(self).from_case(case)
    }

    fn remove_empty(&self) -> StateConverter<'_, T> {
        StateConverter::new(self).remove_empty()
    }
}

/// Holds information about parsing before converting into a case.
///
/// This struct is used when invoking the `from_case` and `with_boundaries` methods on
/// `Casing`.  For a more fine grained approach to case conversion, consider using the [`Converter`]
/// struct.
/// ```
/// # use convert_case::{Case, Casing};
/// assert_eq!(
///     "By-Tor And The Snow Dog".from_case(Case::Title).to_case(Case::Snake),
///     "by-tor_and_the_snow_dog",
/// );
/// ```
pub struct StateConverter<'a, T: AsRef<str>> {
    s: &'a T,
    conv: Converter,
}

impl<'a, T: AsRef<str>> StateConverter<'a, T> {
    /// Only called by Casing function to_case()
    fn new(s: &'a T) -> Self {
        Self {
            s,
            conv: Converter::new(),
        }
    }

    /// Uses the boundaries associated with `case` for word segmentation.  This
    /// will overwrite any boundary information initialized before.  This method is
    /// likely not useful, but provided anyway.
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!(
    ///     "Alan Turing"
    ///         .from_case(Case::Snake) // from Casing trait
    ///         .from_case(Case::Title) // from StateConverter, overwrites previous
    ///         .to_case(Case::Kebab),
    ///     "alan-turing"
    /// );
    /// ```
    pub fn from_case(self, case: Case) -> Self {
        Self {
            conv: self.conv.from_case(case),
            ..self
        }
    }

    /// Overwrites boundaries for word segmentation with those provided.  This will overwrite
    /// any boundary information initialized before.  This method is likely not useful, but
    /// provided anyway.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    /// assert_eq!(
    ///     "Vector5d Transformation"
    ///         .from_case(Case::Title) // from Casing trait
    ///         .set_boundaries(&[Boundary::Space, Boundary::LowerDigit]) // overwrites `from_case`
    ///         .to_case(Case::Kebab),
    ///     "vector-5d-transformation"
    /// );
    /// ```
    pub fn set_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.set_boundaries(bs),
        }
    }

    /// Removes any boundaries that were already initialized.  This is particularly useful when a
    /// case like `Case::Camel` has a lot of associated word boundaries, but you want to exclude
    /// some.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    /// assert_eq!(
    ///     "2dTransformation"
    ///         .from_case(Case::Camel)
    ///         .remove_boundaries(&Boundary::digits())
    ///         .to_case(Case::Snake),
    ///     "2d_transformation"
    /// );
    /// ```
    pub fn remove_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.remove_boundaries(bs),
        }
    }

    /// Prepends the `RemoveEmpty` pattern to filter out empty words before conversion.
    /// This is useful when splitting produces empty words from leading, trailing, and
    /// duplicate delimiters.
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!(
    ///     "_leading_underscore"
    ///         .from_case(Case::Snake)
    ///         .remove_empty()
    ///         .to_case(Case::Camel),
    ///     "leadingUnderscore"
    /// );
    /// ```
    pub fn remove_empty(self) -> Self {
        Self {
            s: self.s,
            conv: self.conv.add_pattern(pattern::Pattern::RemoveEmpty),
        }
    }

    /// Consumes the `StateConverter` and returns the converted string.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    /// assert_eq!(
    ///     "Ice-Cream Social".from_case(Case::Title).to_case(Case::Lower),
    ///     "ice-cream social",
    /// );
    /// ```
    pub fn to_case(self, case: Case) -> String {
        self.conv.to_case(case).convert(self.s)
    }
}

/// The variant of `case` from a token.
///
/// The token associated with each variant is the variant written in snake case.
/// To do conversion with a macro, see [`ccase`].
#[macro_export]
macro_rules! case {
    (snake) => {
        convert_case::Case::Snake
    };
    (constant) => {
        convert_case::Case::Constant
    };
    (upper_snake) => {
        convert_case::Case::UpperSnake
    };
    (ada) => {
        convert_case::Case::Ada;
    };
    (kebab) => {
        convert_case::Case::Kebab
    };
    (cobol) => {
        convert_case::Case::Cobol
    };
    (upper_kebab) => {
        convert_case::Case::UpperKebab
    };
    (train) => {
        convert_case::Case::Train
    };
    (flat) => {
        convert_case::Case::Flat
    };
    (upper_flat) => {
        convert_case::Case::UpperFlat
    };
    (pascal) => {
        convert_case::Case::Pascal
    };
    (upper_camel) => {
        convert_case::Case::UpperCamel
    };
    (camel) => {
        convert_case::Case::Camel
    };
    (lower) => {
        convert_case::Case::Lower
    };
    (upper) => {
        convert_case::Case::Upper
    };
    (title) => {
        convert_case::Case::Title
    };
    (sentence) => {
        convert_case::Case::Sentence
    };
}

/// Convert an identifier into a case.
///
/// You can convert a string by writing the case name as a token.
/// ```
/// use convert_case::ccase;
///
/// assert_eq!(ccase!(snake, "myVarName"), "my_var_name");
/// // equivalent to
/// // "myVarName".to_case(Case::Snake)
/// ```
/// You can also specify a _from_ case, or the case that determines how the input
/// string is split into words.
/// ```
/// use convert_case::ccase;
///
/// assert_eq!(ccase!(sentence -> snake, "Ice-cream sales"), "ice-cream_sales");
/// // equivalent to
/// // "Ice-cream sales".from_case(Case::Sentence).to_case(Case::Snake)
/// ```
#[macro_export]
macro_rules! ccase {
    ($case:ident, $e:expr) => {
        convert_case::Converter::new()
            .to_case(convert_case::case!($case))
            .convert($e)
    };
    ($from:ident -> $to:ident, $e:expr) => {
        convert_case::Converter::new()
            .from_case(convert_case::case!($from))
            .to_case(convert_case::case!($to))
            .convert($e)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    use alloc::vec;

    #[test]
    fn lossless_against_lossless() {
        let examples = vec![
            (Case::Snake, "my_variable_22_name"),
            (Case::Constant, "MY_VARIABLE_22_NAME"),
            (Case::Ada, "My_Variable_22_Name"),
            (Case::Kebab, "my-variable-22-name"),
            (Case::Cobol, "MY-VARIABLE-22-NAME"),
            (Case::Train, "My-Variable-22-Name"),
            (Case::Pascal, "MyVariable22Name"),
            (Case::Camel, "myVariable22Name"),
            (Case::Lower, "my variable 22 name"),
            (Case::Upper, "MY VARIABLE 22 NAME"),
            (Case::Title, "My Variable 22 Name"),
            (Case::Sentence, "My variable 22 name"),
        ];

        for (case_a, str_a) in &examples {
            for (case_b, str_b) in &examples {
                assert_eq!(str_b.to_case(*case_a), *str_a);
                assert_eq!(str_b.from_case(*case_b).to_case(*case_a), *str_a);
            }
        }
    }

    #[test]
    fn obvious_default_parsing() {
        let examples = vec![
            "SuperMario64Game",
            "super-mario64-game",
            "superMario64 game",
            "Super Mario 64_game",
            "SUPERMario 64-game",
            "super_mario-64 game",
        ];

        for example in examples {
            assert_eq!(example.to_case(Case::Snake), "super_mario_64_game");
        }
    }

    #[test]
    fn multiline_strings() {
        assert_eq!("one\ntwo\nthree".to_case(Case::Title), "One\ntwo\nthree");
    }

    #[test]
    fn camel_case_acronyms() {
        assert_eq!(
            "XMLHttpRequest".from_case(Case::Camel).to_case(Case::Snake),
            "xml_http_request"
        );
        assert_eq!(
            "XMLHttpRequest"
                .from_case(Case::UpperCamel)
                .to_case(Case::Snake),
            "xml_http_request"
        );
        assert_eq!(
            "XMLHttpRequest"
                .from_case(Case::Pascal)
                .to_case(Case::Snake),
            "xml_http_request"
        );
    }

    #[test]
    fn leading_tailing_double_delimiters() {
        let words = ["first", "second"];
        let delimited_cases = &[
            Case::Snake,
            Case::Kebab,
            Case::Lower,
            Case::Custom {
                boundaries: &[Boundary::Custom {
                    condition: |s| *s.get(0).unwrap() == ".",
                    start: 0,
                    len: 1,
                }],
                pattern: Pattern::Lowercase,
                delimiter: ".",
            },
        ];

        for &case in delimited_cases {
            let delim = case.delimiter();
            let double = format!("{delim}{delim}");

            let identifiers = [
                format!("{delim}{}", words.join(delim)),
                format!("{}{delim}", words.join(delim)),
                format!("{delim}{}{delim}", words.join(delim)),
                format!("{}", words.join(&double)),
                format!("{delim}{}", words.join(&double)),
                format!("{}{delim}", words.join(&double)),
                format!("{delim}{}{delim}", words.join(&double)),
            ];

            for identifier in identifiers {
                assert_eq!(identifier.to_case(case), identifier);
                assert_eq!(identifier.from_case(case).to_case(case), identifier);
            }
        }
    }

    #[test]
    fn early_word_boundaries() {
        assert_eq!(
            "aBagel".from_case(Case::Camel).to_case(Case::Snake),
            "a_bagel"
        );
    }

    #[test]
    fn late_word_boundaries() {
        assert_eq!(
            "teamA".from_case(Case::Camel).to_case(Case::Snake),
            "team_a"
        );
    }

    #[test]
    fn empty_string() {
        for (case_a, case_b) in Case::all_cases()
            .into_iter()
            .zip(Case::all_cases().into_iter())
        {
            assert_eq!("", "".from_case(*case_a).to_case(*case_b));
        }
    }

    #[test]
    fn default_all_boundaries() {
        assert_eq!(
            "ABC-abc_abcAbc ABCAbc".to_case(Case::Snake),
            "abc_abc_abc_abc_abc_abc"
        );
        assert_eq!("8a8A8".to_case(Case::Snake), "8_a_8_a_8");
    }

    #[test]
    fn remove_boundaries() {
        assert_eq!(
            "M02S05BinaryTrees.pdf"
                .from_case(Case::Pascal)
                .remove_boundaries(&[Boundary::UpperDigit])
                .to_case(Case::Snake),
            "m02_s05_binary_trees.pdf"
        );
    }

    #[test]
    fn with_boundaries() {
        assert_eq!(
            "my_dumbFileName"
                .set_boundaries(&[Boundary::Underscore, Boundary::LowerUpper])
                .to_case(Case::Kebab),
            "my-dumb-file-name"
        );
    }

    // From issue https://github.com/rutrum/convert-case/issues/4
    // From issue https://github.com/rutrum/convert-case/issues/8
    #[test]
    fn unicode_words() {
        let strings = &["ПЕРСПЕКТИВА24", "música moderna"];
        for s in strings {
            for &case in Case::all_cases() {
                assert!(!s.to_case(case).is_empty());
            }
            for &from in Case::all_cases() {
                for &to in Case::all_cases() {
                    assert!(!s.from_case(from).to_case(to).is_empty());
                }
            }
        }
    }

    // idea for asserting the associated boundaries are correct
    #[test]
    fn appropriate_associated_boundaries() {
        let word_groups = &[
            vec!["my", "var", "name"],
            vec!["MY", "var", "Name"],
            vec!["another", "vAR"],
            vec!["XML", "HTTP", "Request"],
        ];

        for words in word_groups {
            for case in Case::all_cases() {
                if case == &Case::Flat || case == &Case::UpperFlat {
                    continue;
                }
                assert_eq!(
                    case.pattern().mutate(&split(
                        &case.pattern().mutate(words).join(case.delimiter()),
                        case.boundaries()
                    )),
                    case.pattern().mutate(words),
                    "Test boundaries on Case::{:?} with {:?}",
                    case,
                    words,
                );
            }
        }
    }
}
