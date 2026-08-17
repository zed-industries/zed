use crate::boundary::{self, Boundary};
use crate::pattern::Pattern;

use alloc::string::String;
use alloc::vec::Vec;

/// Defines the case of an identifier.
/// ```
/// use convert_case::ccase;
/// assert_eq!(ccase!(title, "super_mario_64"), "Super Mario 64");
///
/// use convert_case::{Case, Casing};
/// assert_eq!("super_mario_64".to_case(Case::Title), "Super Mario 64");
/// ```
///
/// A case is the pair of a [pattern](Pattern) and a delimeter (a string).  Given
/// a list of words, a pattern describes how to mutate the words and a delimeter is how the mutated
/// words are joined together.
///
/// | pattern | underscore `_` | hyphen `-` | empty string | space |
/// | ---: | --- | --- | --- | --- |
/// | [lowercase](Pattern::Lowercase) | [snake_case](Case::Snake) | [kebab-case](Case::Kebab) | [flatcase](Case::Flat) | [lower case](Case::Lower) |
/// | [uppercase](Pattern::Uppercase) | [CONSTANT_CASE](Case::Constant) | [COBOL-CASE](Case::Cobol) | [UPPERFLATCASE](Case::UpperFlat) | [UPPER CASE](Case::Upper) |
/// | [capital](Pattern::Capital) | [Ada_Case](Case::Ada) | [Train-Case](Case::Train) | [PascalCase](Case::Pascal) | [Title Case](Case::Title) |
/// | [camel](Pattern::Camel) | | | [camelCase](Case::Camel) |
///
/// There are additionally [`Case::Sentence`].
///
/// This crate provides the ability to convert "from" a case.  This introduces a different feature
/// of cases which are the [word boundaries](Boundary) that segment the identifier into words.  For example, a
/// snake case identifier `my_var_name` can be split on underscores `_` to segment into words.  A
/// camel case identifier `myVarName` is split where a lowercase letter is followed by an
/// uppercase letter.  Each case is also associated with a list of boundaries that are used when
/// converting "from" a particular case.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Case<'b> {
    /// Custom cases can be delimited by any static string slice and mutate words
    /// using any pattern.  Further, they can use any list of boundaries for
    /// splitting identifiers into words.
    ///
    /// This flexibility can create cases not present as another variant of the
    /// Case enum.  For instance, you could create a "dot case" like so.
    /// ```
    /// use convert_case::{Case, Casing, delim_boundary, Pattern};
    /// let dot_case = Case::Custom {
    ///     boundaries: &[delim_boundary!(".")],
    ///     pattern: Pattern::Lowercase,
    ///     delim: ".",
    /// };
    ///
    /// assert_eq!(
    ///     "myNewCase".to_case(dot_case),
    ///     "my.new.case",
    /// );
    /// assert_eq!(
    ///     "my.new.case".from_case(dot_case).to_case(Case::Title),
    ///     "My New Case",
    /// );
    /// ```
    Custom {
        boundaries: &'b [Boundary],
        pattern: Pattern,
        delim: &'static str,
    },

    /// Snake case strings are delimited by underscores `_` and are all lowercase.
    ///
    /// * Boundaries : [Underscore](Boundary::Underscore)
    /// * Pattern : [Lowercase](Pattern::Lowercase)
    /// * Delimeter : Underscore `"_"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(snake, "My variable NAME"), "my_variable_name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Snake), "my_variable_name");
    /// ```
    Snake,

    /// Constant case strings are delimited by underscores `_` and are all uppercase.
    /// * Boundaries: [Underscore](Boundary::Underscore)
    /// * Pattern: [Uppercase](Pattern::Uppercase)
    /// * Delimeter: Underscore `"_"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(constant, "My variable NAME"), "MY_VARIABLE_NAME");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Constant), "MY_VARIABLE_NAME");
    /// ```
    Constant,

    /// Upper snake case is an alternative name for [constant case](Case::Constant).
    UpperSnake,

    /// Ada case strings are delimited by underscores `_`.  The leading letter of
    /// each word is uppercase, while the rest is lowercase.
    /// * Boundaries: [Underscore](Boundary::Underscore)
    /// * Pattern: [Capital](Pattern::Capital)
    /// * Delimeter: Underscore `"_"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(ada, "My variable NAME"), "My_Variable_Name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Ada), "My_Variable_Name");
    /// ```
    Ada,

    /// Kebab case strings are delimited by hyphens `-` and are all lowercase.
    /// * Boundaries: [Hyphen](Boundary::Hyphen)
    /// * Pattern: [Lowercase](Pattern::Lowercase)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(kebab, "My variable NAME"), "my-variable-name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Kebab), "my-variable-name");
    /// ```
    Kebab,

    /// Cobol case strings are delimited by hyphens `-` and are all uppercase.
    /// * Boundaries: [Hyphen](Boundary::Hyphen)
    /// * Pattern: [Uppercase](Pattern::Uppercase)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(cobol, "My variable NAME"), "MY-VARIABLE-NAME");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Cobol), "MY-VARIABLE-NAME");
    /// ```
    Cobol,

    /// Upper kebab case is an alternative name for [Cobol case](Case::Cobol).
    UpperKebab,

    /// Train case strings are delimited by hyphens `-`.  The leading letter of
    /// each word is uppercase, while the rest is lowercase.
    /// * Boundaries: [Hyphen](Boundary::Hyphen)
    /// * Pattern: [Capital](Pattern::Capital)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(train, "My variable NAME"), "My-Variable-Name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Train), "My-Variable-Name");
    /// ```
    Train,

    /// Flat case strings are all lowercase, with no delimiter. Note that word boundaries are lost.
    /// * Boundaries: No boundaries
    /// * Pattern: [Lowercase](Pattern::Lowercase)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(flat, "My variable NAME"), "myvariablename");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Flat), "myvariablename");
    /// ```
    Flat,

    /// Upper flat case strings are all uppercase, with no delimiter. Note that word boundaries are lost.
    /// * Boundaries: No boundaries
    /// * Pattern: [Uppercase](Pattern::Uppercase)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(upper_flat, "My variable NAME"), "MYVARIABLENAME");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::UpperFlat), "MYVARIABLENAME");
    /// ```
    UpperFlat,

    /// Pascal case strings are lowercase, but for every word the
    /// first letter is capitalized.
    /// * Boundaries: [LowerUpper](Boundary::LowerUpper), [DigitUpper](Boundary::DigitUpper),
    ///   [UpperDigit](Boundary::UpperDigit), [DigitLower](Boundary::DigitLower),
    ///   [LowerDigit](Boundary::LowerDigit), [Acronym](Boundary::Acronym)
    /// * Pattern: [Capital](`Pattern::Capital`)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(pascal, "My variable NAME"), "MyVariableName");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Pascal), "MyVariableName");
    /// ```
    Pascal,

    /// Upper camel case is an alternative name for [Pascal case](Case::Pascal).
    UpperCamel,

    /// Camel case strings are lowercase, but for every word _except the first_ the
    /// first letter is capitalized.
    /// * Boundaries: [LowerUpper](Boundary::LowerUpper), [DigitUpper](Boundary::DigitUpper),
    ///   [UpperDigit](Boundary::UpperDigit), [DigitLower](Boundary::DigitLower),
    ///   [LowerDigit](Boundary::LowerDigit), [Acronym](Boundary::Acronym)
    /// * Pattern: [Camel](`Pattern::Camel`)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(camel, "My variable NAME"), "myVariableName");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Camel), "myVariableName");
    /// ```
    Camel,

    /// Lowercase strings are delimited by spaces and all characters are lowercase.
    /// * Boundaries: [Space](`Boundary::Space`)
    /// * Pattern: [Lowercase](`Pattern::Lowercase`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(lower, "My variable NAME"), "my variable name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Lower), "my variable name");
    /// ```
    Lower,

    /// Uppercase strings are delimited by spaces and all characters are uppercase.
    /// * Boundaries: [Space](`Boundary::Space`)
    /// * Pattern: [Uppercase](`Pattern::Uppercase`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(upper, "My variable NAME"), "MY VARIABLE NAME");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Upper), "MY VARIABLE NAME");
    /// ```
    Upper,

    /// Title case strings are delimited by spaces. Only the leading character of
    /// each word is uppercase.  No inferences are made about language, so words
    /// like "as", "to", and "for" will still be capitalized.
    /// * Boundaries: [Space](`Boundary::Space`)
    /// * Pattern: [Capital](`Pattern::Capital`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(title, "My variable NAME"), "My Variable Name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Title), "My Variable Name");
    /// ```
    Title,

    /// Sentence case strings are delimited by spaces. Only the leading character of
    /// the first word is uppercase.
    /// * Boundaries: [Space](`Boundary::Space`)
    /// * Pattern: [Sentence](`Pattern::Sentence`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::ccase;
    /// assert_eq!(ccase!(sentence, "My variable NAME"), "My variable name");
    ///
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable NAME".to_case(Case::Sentence), "My variable name");
    /// ```
    Sentence,
}

impl Case<'_> {
    /// Returns the boundaries used in the corresponding case.  That is, where can word boundaries
    /// be distinguished in a string of the given case.  The table outlines which cases use which
    /// set of boundaries.
    ///
    /// | Cases | Boundaries |
    /// | --- | --- |
    /// | Snake, Constant, UpperSnake, Ada | [Underscore](Boundary::Underscore)  |
    /// | Kebab, Cobol, UpperKebab, Train | [Hyphen](Boundary::Hyphen) |
    /// | Lower, Upper, Title | [Space](Boundary::Space) |
    /// | Pascal, UpperCamel, Camel | [LowerUpper](Boundary::LowerUpper), [LowerDigit](Boundary::LowerDigit), [UpperDigit](Boundary::UpperDigit), [DigitLower](Boundary::DigitLower), [DigitUpper](Boundary::DigitUpper), [Acronym](Boundary::Acronym) |
    /// | Flat, UpperFlat | No boundaries |
    pub fn boundaries(&self) -> &[Boundary] {
        use Case::*;
        match self {
            Snake | Constant | UpperSnake | Ada => &[Boundary::Underscore],
            Kebab | Cobol | UpperKebab | Train => &[Boundary::Hyphen],
            Upper | Lower | Title | Sentence => &[Boundary::Space],
            Camel | UpperCamel | Pascal => &[
                Boundary::LowerUpper,
                Boundary::Acronym,
                Boundary::LowerDigit,
                Boundary::UpperDigit,
                Boundary::DigitLower,
                Boundary::DigitUpper,
            ],
            UpperFlat | Flat => &[],
            Custom { boundaries, .. } => boundaries,
        }
    }

    /// Returns the delimiter used in the corresponding case.  The following
    /// table outlines which cases use which delimeter.
    ///
    /// | Cases | Delimeter |
    /// | --- | --- |
    /// | Snake, Constant, UpperSnake, Ada | Underscore `"_"` |
    /// | Kebab, Cobol, UpperKebab, Train | Hyphen `"-"` |
    /// | Upper, Lower, Title, Sentence | Space `" "` |
    /// | Flat, UpperFlat, Pascal, UpperCamel, Camel | Empty string `""` |
    pub const fn delim(&self) -> &'static str {
        use Case::*;
        match self {
            Snake | Constant | UpperSnake | Ada => "_",
            Kebab | Cobol | UpperKebab | Train => "-",
            Upper | Lower | Title | Sentence => " ",
            Flat | UpperFlat | Pascal | UpperCamel | Camel => "",
            Custom { delim, .. } => delim,
        }
    }

    /// Returns the pattern used in the corresponding case.  The following
    /// table outlines which cases use which pattern.
    ///
    /// | Cases | Pattern |
    /// | --- | --- |
    /// | Constant, UpperSnake, Cobol, UpperKebab, UpperFlat, Upper | [Uppercase](Pattern::Uppercase) |
    /// | Snake, Kebab, Flat, Lower | [Lowercase](Pattern::Lowercase) |
    /// | Ada, Train, Pascal, UpperCamel, Title | [Capital](Pattern::Capital) |
    /// | Camel | [Camel](Pattern::Camel) |
    pub const fn pattern(&self) -> Pattern {
        use Case::*;
        match self {
            Constant | UpperSnake | Cobol | UpperKebab | UpperFlat | Upper => Pattern::Uppercase,
            Snake | Kebab | Flat | Lower => Pattern::Lowercase,
            Ada | Train | Pascal | UpperCamel | Title => Pattern::Capital,
            Camel => Pattern::Camel,
            Sentence => Pattern::Sentence,
            Custom { pattern, .. } => *pattern,
        }
    }

    /// Split an identifier into words based on the boundaries of this case.
    /// ```
    /// use convert_case::Case;
    /// assert_eq!(
    ///     Case::Pascal.split(&"getTotalLength"),
    ///     vec!["get", "Total", "Length"],
    /// );
    /// ```
    pub fn split<T>(self, s: &T) -> Vec<&str>
    where
        T: AsRef<str>,
    {
        boundary::split(s, self.boundaries())
    }

    /// Mutate a list of words based on the pattern of this case.
    /// ```
    /// use convert_case::Case;
    /// assert_eq!(
    ///     Case::Snake.mutate(&["get", "Total", "Length"]),
    ///     vec!["get", "total", "length"],
    /// );
    /// ```
    pub fn mutate(self, words: &[&str]) -> Vec<String> {
        self.pattern().mutate(words)
    }

    /// Join a list of words into a single identifier using the delimiter of this case.
    /// ```
    /// use convert_case::Case;
    /// assert_eq!(
    ///     Case::Snake.join(&[
    ///         String::from("get"),
    ///         String::from("total"),
    ///         String::from("length")
    ///     ]),
    ///     String::from("get_total_length"),
    /// );
    /// ```
    pub fn join(self, words: &[String]) -> String {
        words.join(self.delim())
    }

    /// Array of all non-custom case enum variants.  Does not include aliases.
    pub fn all_cases() -> &'static [Case<'static>] {
        use Case::*;
        &[
            Snake, Constant, Ada, Kebab, Cobol, Train, Flat, UpperFlat, Pascal, Camel, Upper,
            Lower, Title, Sentence,
        ]
    }
}
