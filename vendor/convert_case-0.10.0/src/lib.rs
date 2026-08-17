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
//! This propogates and the converted string will share the behavior.  **This can cause
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
//!           pattern
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
//! character or list of characters.  The [`delim_boundary`] macro builds
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
//! use convert_case::{Case, Casing, delim_boundary, Pattern};
//!
//! let dot_case = Case::Custom {
//!     boundaries: &[delim_boundary!(".")],
//!     pattern: Pattern::Lowercase,
//!     delim: ".",
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
//! use convert_case::{Case, Converter, delim_boundary};
//!
//! let modules_into_path = Converter::new()
//!     .set_boundaries(&[delim_boundary!("::")])
//!     .set_delim("/");
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
//! Some extra utilties for convert_case that don't need to be in the main library.
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
    /// `String` with the same pattern and delimeter as `case`.  It will split on boundaries
    /// defined at [`Boundary::defaults()`].
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "tetronimo-piece-border",
    ///     "Tetronimo piece border".to_case(Case::Kebab)
    /// );
    /// ```
    fn to_case(&self, case: Case) -> String;

    /// Start the case conversion by storing the boundaries associated with the given case.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert_eq!(
    ///     "2020-08-10_dannie_birthday",
    ///     "2020-08-10 Dannie Birthday"
    ///         .from_case(Case::Title)
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    #[allow(clippy::wrong_self_convention)]
    fn from_case(&self, case: Case) -> StateConverter<T>;

    /// Creates a `StateConverter` struct initialized with the boundaries provided.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "e1_m1_hangar",
    ///     "E1M1 Hangar"
    ///         .set_boundaries(&[Boundary::DigitUpper, Boundary::Space])
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    fn set_boundaries(&self, bs: &[Boundary]) -> StateConverter<T>;

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
    fn remove_boundaries(&self, bs: &[Boundary]) -> StateConverter<T>;

    /// Determines if `self` is of the given case.  This is done simply by applying
    /// the conversion and seeing if the result is the same.
    /// ```
    /// use convert_case::{Case, Casing};
    ///
    /// assert!( "kebab-case-string".is_case(Case::Kebab));
    /// assert!( "Train-Case-String".is_case(Case::Train));
    ///
    /// assert!(!"kebab-case-string".is_case(Case::Snake));
    /// assert!(!"kebab-case-string".is_case(Case::Train));
    /// ```
    fn is_case(&self, case: Case) -> bool;
}

impl<T: AsRef<str>> Casing<T> for T {
    fn to_case(&self, case: Case) -> String {
        StateConverter::new(self).to_case(case)
    }

    fn set_boundaries(&self, bs: &[Boundary]) -> StateConverter<T> {
        StateConverter::new(self).set_boundaries(bs)
    }

    fn remove_boundaries(&self, bs: &[Boundary]) -> StateConverter<T> {
        StateConverter::new(self).remove_boundaries(bs)
    }

    fn from_case(&self, case: Case) -> StateConverter<T> {
        StateConverter::new(self).from_case(case)
    }

    fn is_case(&self, case: Case) -> bool {
        self.as_ref() == self.to_case(case).as_str()
        /*
        let digitless = self
            .as_ref()
            .chars()
            .filter(|x| !x.is_ascii_digit())
            .collect::<String>();

        digitless == digitless.to_case(case)
        */
    }
}

/// Holds information about parsing before converting into a case.
///
/// This struct is used when invoking the `from_case` and `with_boundaries` methods on
/// `Casing`.  For a more fine grained approach to case conversion, consider using the [`Converter`]
/// struct.
/// ```
/// use convert_case::{Case, Casing};
///
/// let title = "ninety-nine_problems".from_case(Case::Snake).to_case(Case::Title);
/// assert_eq!("Ninety-nine Problems", title);
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
    ///
    /// let name = "Chuck Schuldiner"
    ///     .from_case(Case::Snake) // from Casing trait
    ///     .from_case(Case::Title) // from StateConverter, overwrites previous
    ///     .to_case(Case::Kebab);
    /// assert_eq!("chuck-schuldiner", name);
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
    ///
    /// let song = "theHumbling river-puscifer"
    ///     .from_case(Case::Kebab) // from Casing trait
    ///     .set_boundaries(&[Boundary::Space, Boundary::LowerUpper]) // overwrites `from_case`
    ///     .to_case(Case::Pascal);
    /// assert_eq!("TheHumblingRiver-puscifer", song);  // doesn't split on hyphen `-`
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
    ///
    /// assert_eq!(
    ///     "2d_transformation",
    ///     "2dTransformation"
    ///         .from_case(Case::Camel)
    ///         .remove_boundaries(&Boundary::digits())
    ///         .to_case(Case::Snake)
    /// );
    /// ```
    pub fn remove_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.remove_boundaries(bs),
        }
    }

    /// Consumes the `StateConverter` and returns the converted string.
    /// ```
    /// use convert_case::{Boundary, Case, Casing};
    ///
    /// assert_eq!(
    ///     "ice-cream social",
    ///     "Ice-Cream Social".from_case(Case::Title).to_case(Case::Lower)
    /// );
    /// ```
    pub fn to_case(self, case: Case) -> String {
        self.conv.to_case(case).convert(self.s)
    }
}

/// The variant of `case` from a token.
///
/// The token associated with each variant is the variant written in snake case.
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
/// The macro can be used as follows.
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
    use alloc::vec::Vec;

    fn possible_cases(s: &str) -> Vec<Case> {
        Case::all_cases()
            .iter()
            .filter(|&case| s.from_case(*case).to_case(*case) == s)
            .map(|c| *c)
            .collect()
    }

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
                assert_eq!(*str_a, str_b.from_case(*case_b).to_case(*case_a))
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
            assert_eq!("super_mario_64_game", example.to_case(Case::Snake));
        }
    }

    #[test]
    fn multiline_strings() {
        assert_eq!("One\ntwo\nthree", "one\ntwo\nthree".to_case(Case::Title));
    }

    #[test]
    fn camel_case_acroynms() {
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest".from_case(Case::Camel).to_case(Case::Snake)
        );
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest"
                .from_case(Case::UpperCamel)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest"
                .from_case(Case::Pascal)
                .to_case(Case::Snake)
        );
    }

    #[test]
    fn leading_tailing_delimeters() {
        assert_eq!(
            "_leading_underscore",
            "_leading_underscore"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailing_underscore_",
            "tailing_underscore_"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "_leading_hyphen",
            "-leading-hyphen"
                .from_case(Case::Kebab)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailing_hyphen_",
            "tailing-hyphen-"
                .from_case(Case::Kebab)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailing_hyphens_____",
            "tailing-hyphens-----"
                .from_case(Case::Kebab)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailingHyphens",
            "tailing-hyphens-----"
                .from_case(Case::Kebab)
                .to_case(Case::Camel)
        );
    }

    #[test]
    fn double_delimeters() {
        assert_eq!(
            "many___underscores",
            "many___underscores"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "many---underscores",
            "many---underscores"
                .from_case(Case::Kebab)
                .to_case(Case::Kebab)
        );
    }

    #[test]
    fn early_word_boundaries() {
        assert_eq!(
            "a_bagel",
            "aBagel".from_case(Case::Camel).to_case(Case::Snake)
        );
    }

    #[test]
    fn late_word_boundaries() {
        assert_eq!(
            "team_a",
            "teamA".from_case(Case::Camel).to_case(Case::Snake)
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
            "abc_abc_abc_abc_abc_abc",
            "ABC-abc_abcAbc ABCAbc".to_case(Case::Snake)
        );
        assert_eq!("8_a_8_a_8", "8a8A8".to_case(Case::Snake));
    }

    mod is_case {
        use super::*;

        #[test]
        fn snake() {
            assert!("im_snake_case".is_case(Case::Snake));
            assert!(!"im_NOTsnake_case".is_case(Case::Snake));
        }

        #[test]
        fn kebab() {
            assert!("im-kebab-case".is_case(Case::Kebab));
            assert!(!"im_not_kebab".is_case(Case::Kebab));
        }

        #[test]
        fn lowercase_word() {
            for lower_case in [
                Case::Snake,
                Case::Kebab,
                Case::Flat,
                Case::Lower,
                Case::Camel,
            ] {
                assert!("lowercase".is_case(lower_case));
            }
        }

        #[test]
        fn uppercase_word() {
            for upper_case in [Case::Constant, Case::Cobol, Case::UpperFlat, Case::Upper] {
                assert!("UPPERCASE".is_case(upper_case));
            }
        }

        #[test]
        fn capital_word() {
            for capital_case in [
                Case::Ada,
                Case::Train,
                Case::Pascal,
                Case::Title,
                Case::Sentence,
            ] {
                assert!("Capitalcase".is_case(capital_case));
            }
        }

        #[test]
        fn underscores_not_kebab() {
            assert!(!"kebab-case".is_case(Case::Snake));
        }

        #[test]
        fn multiple_delimiters() {
            assert!(!"kebab-snake_case".is_case(Case::Snake));
            assert!(!"kebab-snake_case".is_case(Case::Kebab));
            assert!(!"kebab-snake_case".is_case(Case::Lower));
        }

        /*
        #[test]
        fn digits_ignored() {
            assert!("UPPER_CASE_WITH_DIGIT1".is_case(Case::Constant));

            assert!("transformation_2d".is_case(Case::Snake));

            assert!("Transformation2d".is_case(Case::Pascal));
            assert!("Transformation2D".is_case(Case::Pascal));

            assert!("transformation2D".is_case(Case::Camel));

            assert!(!"5isntPascal".is_case(Case::Pascal))
        }
        */

        #[test]
        fn not_a_case() {
            for c in Case::all_cases() {
                assert!(!"hyphen-and_underscore".is_case(*c));
                assert!(!"Sentence-with-hyphens".is_case(*c));
                assert!(!"Sentence_with_underscores".is_case(*c));
            }
        }
    }

    #[test]
    fn remove_boundaries() {
        assert_eq!(
            "m02_s05_binary_trees.pdf",
            "M02S05BinaryTrees.pdf"
                .from_case(Case::Pascal)
                .remove_boundaries(&[Boundary::UpperDigit])
                .to_case(Case::Snake)
        );
    }

    #[test]
    fn with_boundaries() {
        assert_eq!(
            "my-dumb-file-name",
            "my_dumbFileName"
                .set_boundaries(&[Boundary::Underscore, Boundary::LowerUpper])
                .to_case(Case::Kebab)
        );
    }

    #[test]
    fn multiple_from_case() {
        assert_eq!(
            "longtime_nosee",
            "LongTime NoSee"
                .from_case(Case::Camel)
                .from_case(Case::Title)
                .to_case(Case::Snake),
        )
    }

    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn detect_many_cases() {
        let lower_cases_vec = possible_cases(&"asef");
        let lower_cases_set = HashSet::from_iter(lower_cases_vec.into_iter());
        let mut actual = HashSet::new();
        actual.insert(Case::Lower);
        actual.insert(Case::Camel);
        actual.insert(Case::Snake);
        actual.insert(Case::Kebab);
        actual.insert(Case::Flat);
        assert_eq!(lower_cases_set, actual);

        let lower_cases_vec = possible_cases(&"asefCase");
        let lower_cases_set = HashSet::from_iter(lower_cases_vec.into_iter());
        let mut actual = HashSet::new();
        actual.insert(Case::Camel);
        assert_eq!(lower_cases_set, actual);
    }

    #[test]
    fn detect_each_case() {
        let s = "My String Identifier".to_string();
        for &case in Case::all_cases() {
            let new_s = s.from_case(case).to_case(case);
            let possible = possible_cases(&new_s);
            assert!(possible.iter().any(|c| c == &case));
        }
    }

    // From issue https://github.com/rutrum/convert-case/issues/8
    #[test]
    fn accent_mark() {
        let s = "música moderna".to_string();
        assert_eq!("MúsicaModerna", s.to_case(Case::Pascal));
    }

    // From issue https://github.com/rutrum/convert-case/issues/4
    #[test]
    fn russian() {
        let s = "ПЕРСПЕКТИВА24".to_string();
        let _n = s.to_case(Case::Title);
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
                        &case.pattern().mutate(words).join(case.delim()),
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
