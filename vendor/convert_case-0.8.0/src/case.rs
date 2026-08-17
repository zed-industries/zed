use crate::boundary::{self, Boundary};
use crate::pattern;

use alloc::string::String;
use alloc::vec::Vec;

/// Defines the case of an identifier.
///
/// ```
/// use convert_case::{Case, Casing};
///
/// let super_mario_title: String = "super_mario_64".to_case(Case::Title);
/// assert_eq!("Super Mario 64", super_mario_title);
/// ```
///
/// A case is the pair of a [pattern](pattern::Pattern) and a delimeter (a string).  Given
/// a list of words, a pattern describes how to mutate the words and a delimeter is how the mutated
/// words are joined together.  These inherantly are the properties of what makes a "multiword
/// identifier case", or simply "case".
///
/// | pattern | underscore `_` | hyphen `-` | empty string | space |
/// | ---: | --- | --- | --- | --- |
/// | [lowercase](pattern::lowercase) | [snake_case](Case::Snake) | [kebab-case](Case::Kebab) | [flatcase](Case::Flat) | [lower case](Case::Lower) |
/// | [uppercase](pattern::uppercase) | [CONSTANT_CASE](Case::Constant) | [COBOL-CASE](Case::Cobol) | [UPPERFLATCASE](Case::UpperFlat) | [UPPER CASE](Case::Upper) |
/// | [capital](pattern::capital) | [Ada_Case](Case::Ada) | [Train-Case](Case::Train) | [PascalCase](Case::Pascal) | [Title Case](Case::Title) |
/// | [camel](pattern::camel) | | | [camelCase](Case::Camel) |
///
/// There are other less common cases, such as [`Case::Sentence`], [`Case::Alternating`], and [`Case::Toggle`].
///
/// Then there are two random cases [`Case::Random`] and [`Case::PseudoRandom`] from the `random` feature.
///
/// This crate provides the ability to convert "from" a case.  This introduces a different feature
/// of cases which are the [word boundaries](Boundary) that segment the identifier into words.  For example, a
/// snake case identifier `my_var_name` can be split on underscores `_` to segment into words.  A
/// camel case identifier `myVarName` is split where a lowercase letter is followed by an
/// uppercase letter.  Each case is also associated with a list of boundaries that are used when
/// converting "from" a particular case.
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Case<'a> {
    /// Custom cases can be delimited by any static string slice and mutate words
    /// using any pattern.  Further, they can use any list of boundaries for
    /// splitting identifiers into words.
    ///
    /// This flexibility can create cases not present as another variant of the
    /// Case enum.  For instance, you could create a "dot case" like so.
    /// ```
    /// use convert_case::{Case, Casing, Boundary, pattern};
    /// let dot_case = Case::Custom {
    ///     boundaries: &[Boundary::from_delim(".")],
    ///     pattern: pattern::lowercase,
    ///     delim: ".",
    /// };
    ///
    /// assert_eq!(
    ///     "my.new.case",
    ///     "myNewCase".to_case(dot_case),
    /// );
    /// assert_eq!(
    ///     "My New Case",
    ///     "my.new.case".from_case(dot_case).to_case(Case::Title),
    /// );
    /// ```
    Custom {
        boundaries: &'a [Boundary],
        pattern: fn(&[&str]) -> Vec<String>,
        delim: &'static str,
    },

    /// Snake case strings are delimited by underscores `_` and are all lowercase.
    /// * Boundaries: [Underscore](Boundary::UNDERSCORE)
    /// * Pattern: [lowercase](pattern::lowercase)
    /// * Delimeter: Underscore `"_"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my_variable_name", "My variable NAME".to_case(Case::Snake))
    /// ```
    Snake,

    /// Constant case strings are delimited by underscores `_` and are all uppercase.
    /// * Boundaries: [Underscore](Boundary::UNDERSCORE)
    /// * Pattern: [uppercase](pattern::uppercase)
    /// * Delimeter: Underscore `"_"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY_VARIABLE_NAME", "My variable NAME".to_case(Case::Constant))
    /// ```
    Constant,

    /// Upper snake case is an alternative name for [constant case](Case::Constant).
    UpperSnake,

    /// Ada case strings are delimited by underscores `_`.  The leading letter of
    /// each word is uppercase, while the rest is lowercase.
    /// * Boundaries: [Underscore](Boundary::UNDERSCORE)
    /// * Pattern: [capital](pattern::capital)
    /// * Delimeter: Underscore `"_"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My_Variable_Name", "My variable NAME".to_case(Case::Ada))
    /// ```
    Ada,

    /// Kebab case strings are delimited by hyphens `-` and are all lowercase.
    /// * Boundaries: [Hyphen](Boundary::HYPHEN)
    /// * Pattern: [lowercase](pattern::lowercase)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my-variable-name", "My variable NAME".to_case(Case::Kebab))
    /// ```
    Kebab,

    /// Cobol case strings are delimited by hyphens `-` and are all uppercase.
    /// * Boundaries: [Hyphen](Boundary::HYPHEN)
    /// * Pattern: [uppercase](pattern::uppercase)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY-VARIABLE-NAME", "My variable NAME".to_case(Case::Cobol))
    /// ```
    Cobol,

    /// Upper kebab case is an alternative name for [Cobol case](Case::Cobol).
    UpperKebab,

    /// Train case strings are delimited by hyphens `-`.  All characters are lowercase
    /// except for the leading character of each word.
    /// * Boundaries: [Hyphen](Boundary::HYPHEN)
    /// * Pattern: [capital](pattern::capital)
    /// * Delimeter: Hyphen `"-"`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My-Variable-Name", "My variable NAME".to_case(Case::Train))
    /// ```
    Train,

    /// Flat case strings are all lowercase, with no delimiter. Note that word boundaries are lost.
    /// * Boundaries: No boundaries
    /// * Pattern: [lowercase](pattern::lowercase)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("myvariablename", "My variable NAME".to_case(Case::Flat))
    /// ```
    Flat,

    /// Upper flat case strings are all uppercase, with no delimiter. Note that word boundaries are lost.
    /// * Boundaries: No boundaries
    /// * Pattern: [uppercase](pattern::uppercase)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MYVARIABLENAME", "My variable NAME".to_case(Case::UpperFlat))
    /// ```
    UpperFlat,

    /// Pascal case strings are lowercase, but for every word the
    /// first letter is capitalized.
    /// * Boundaries: [LowerUpper](Boundary::LOWER_UPPER), [DigitUpper](Boundary::DIGIT_UPPER),
    ///   [UpperDigit](Boundary::UPPER_DIGIT), [DigitLower](Boundary::DIGIT_LOWER),
    ///   [LowerDigit](Boundary::LOWER_DIGIT), [Acronym](Boundary::ACRONYM)
    /// * Pattern: [capital](`pattern::capital`)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MyVariableName", "My variable NAME".to_case(Case::Pascal))
    /// ```
    Pascal,

    /// Upper camel case is an alternative name for [Pascal case](Case::Pascal).
    UpperCamel,

    /// Camel case strings are lowercase, but for every word _except the first_ the
    /// first letter is capitalized.
    /// * Boundaries: [LowerUpper](Boundary::LOWER_UPPER), [DigitUpper](Boundary::DIGIT_UPPER),
    ///   [UpperDigit](Boundary::UPPER_DIGIT), [DigitLower](Boundary::DIGIT_LOWER),
    ///   [LowerDigit](Boundary::LOWER_DIGIT), [Acronym](Boundary::ACRONYM)
    /// * Pattern: [camel](`pattern::camel`)
    /// * Delimeter: Empty string `""`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("myVariableName", "My variable NAME".to_case(Case::Camel))
    /// ```
    Camel,

    /// Lowercase strings are delimited by spaces and all characters are lowercase.
    /// * Boundaries: [Space](`Boundary::SPACE`)
    /// * Pattern: [lowercase](`pattern::lowercase`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my variable name", "My variable NAME".to_case(Case::Lower))
    /// ```
    Lower,

    /// Lowercase strings are delimited by spaces and all characters are lowercase.
    /// * Boundaries: [Space](`Boundary::SPACE`)
    /// * Pattern: [uppercase](`pattern::uppercase`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY VARIABLE NAME", "My variable NAME".to_case(Case::Upper))
    /// ```
    Upper,

    /// Title case strings are delimited by spaces. Only the leading character of
    /// each word is uppercase.  No inferences are made about language, so words
    /// like "as", "to", and "for" will still be capitalized.
    /// * Boundaries: [Space](`Boundary::SPACE`)
    /// * Pattern: [capital](`pattern::capital`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My Variable Name", "My variable NAME".to_case(Case::Title))
    /// ```
    Title,

    /// Sentence case strings are delimited by spaces. Only the leading character of
    /// the first word is uppercase.
    /// * Boundaries: [Space](`Boundary::SPACE`)
    /// * Pattern: [sentence](`pattern::sentence`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My variable name", "My variable NAME".to_case(Case::Sentence))
    /// ```
    Sentence,

    /// Alternating case strings are delimited by spaces.  Characters alternate between uppercase
    /// and lowercase.
    /// * Boundaries: [Space](Boundary::SPACE)
    /// * Pattern: [alternating](pattern::alternating)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("mY vArIaBlE nAmE", "My variable NAME".to_case(Case::Alternating));
    /// ```
    Alternating,

    /// Toggle case strings are delimited by spaces.  All characters are uppercase except
    /// for the leading character of each word, which is lowercase.
    /// * Boundaries: [Space](`Boundary::SPACE`)
    /// * Pattern: [toggle](`pattern::toggle`)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("mY vARIABLE nAME", "My variable NAME".to_case(Case::Toggle))
    /// ```
    Toggle,

    /// Random case strings are delimited by spaces and characters are
    /// randomly upper case or lower case.  
    ///
    /// This uses the `rand` crate
    /// and is only available with the "random" feature.
    /// * Boundaries: [Space](Boundary::SPACE)
    /// * Pattern: [random](pattern::random)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// # #[cfg(any(doc, feature = "random"))]
    /// let new = "My variable NAME".to_case(Case::Random);
    /// ```
    /// String `new` could be "My vaRIAbLE nAme" for example.
    #[cfg(any(doc, feature = "random"))]
    #[cfg(feature = "random")]
    Random,

    /// Pseudo-random case strings are delimited by spaces and characters are randomly
    /// upper case or lower case, but there will never more than two consecutive lower
    /// case or upper case letters in a row.  
    ///
    /// This uses the `rand` crate and is
    /// only available with the "random" feature.
    /// * Boundaries: [Space](Boundary::SPACE)
    /// * Pattern: [pseudo_random](pattern::pseudo_random)
    /// * Delimeter: Space `" "`
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// # #[cfg(any(doc, feature = "random"))]
    /// let new = "My variable NAME".to_case(Case::Random);
    /// ```
    /// String `new` could be "mY vArIAblE NamE" for example.
    #[cfg(any(doc, feature = "random"))]
    #[cfg(feature = "random")]
    PseudoRandom,
}

impl Case<'_> {
    /// Returns the boundaries used in the corresponding case.  That is, where can word boundaries
    /// be distinguished in a string of the given case.  The table outlines which cases use which
    /// set of boundaries.
    ///
    /// | Cases | Boundaries |
    /// | --- | --- |
    /// | Snake, Constant, UpperSnake, Ada | [UNDERSCORE](Boundary::UNDERSCORE)  |
    /// | Kebab, Cobol, UpperKebab, Train | [HYPHEN](Boundary::HYPHEN) |
    /// | Lower, Upper, Title, Alternating, Toggle, Random, PseudoRandom | [SPACE](Boundary::Space) |
    /// | Pascal, UpperCamel, Camel | [LOWER_UPPER](Boundary::LOWER_UPPER), [LOWER_DIGIT](Boundary::LOWER_DIGIT), [UPPER_DIGIT](Boundary::UPPER_DIGIT), [DIGIT_LOWER](Boundary::DIGIT_LOWER), [DIGIT_UPPER](Boundary::DIGIT_UPPER), [ACRONYM](Boundary::ACRONYM) |
    /// | Flat, UpperFlat | No boundaries |
    pub fn boundaries(&self) -> &[Boundary] {
        use Case::*;
        match self {
            Snake | Constant | UpperSnake | Ada => &[Boundary::UNDERSCORE],
            Kebab | Cobol | UpperKebab | Train => &[Boundary::HYPHEN],
            Upper | Lower | Title | Sentence | Toggle | Alternating => &[Boundary::SPACE],
            Camel | UpperCamel | Pascal => &[
                Boundary::LOWER_UPPER,
                Boundary::ACRONYM,
                Boundary::LOWER_DIGIT,
                Boundary::UPPER_DIGIT,
                Boundary::DIGIT_LOWER,
                Boundary::DIGIT_UPPER,
            ],
            UpperFlat | Flat => &[],
            Custom { boundaries, .. } => boundaries,

            #[cfg(feature = "random")]
            Random | PseudoRandom => &[Boundary::SPACE],
        }
    }

    /// Returns the delimiter used in the corresponding case.  The following
    /// table outlines which cases use which delimeter.
    ///
    /// | Cases | Delimeter |
    /// | --- | --- |
    /// | Snake, Constant, UpperSnake, Ada | Underscore `"_"` |
    /// | Kebab, Cobol, UpperKebab, Train | Hyphen `"-"` |
    /// | Upper, Lower, Title, Sentence, Alternating, Toggle, Random, PseudoRandom | Space `" "` |
    /// | Flat, UpperFlat, Pascal, UpperCamel, Camel | Empty string `""` |
    pub const fn delim(&self) -> &'static str {
        use Case::*;
        match self {
            Snake | Constant | UpperSnake | Ada => "_",
            Kebab | Cobol | UpperKebab | Train => "-",
            Upper | Lower | Title | Sentence | Alternating | Toggle => " ",
            Flat | UpperFlat | Pascal | UpperCamel | Camel => "",
            Custom { delim, .. } => delim,

            #[cfg(feature = "random")]
            Random | PseudoRandom => " ",
        }
    }

    /// Returns the pattern used in the corresponding case.  The following
    /// table outlines which cases use which pattern.
    ///
    /// | Cases | Pattern |
    /// | --- | --- |
    /// | Constant, UpperSnake, Cobol, UpperKebab, UpperFlat, Upper | [uppercase](pattern::uppercase) |
    /// | Snake, Kebab, Flat, Lower | [lowercase](pattern::lowercase) |
    /// | Ada, Train, Pascal, UpperCamel, Title | [capital](pattern::capital) |
    /// | Camel | [camel](pattern::camel) |
    /// | Alternating | [alternating](pattern::alternating) |
    /// | Random | [random](pattern::random) |
    /// | PseudoRandom | [pseudo_random](pattern::pseudo_random) |
    pub const fn pattern(&self) -> pattern::Pattern {
        use Case::*;
        match self {
            Constant | UpperSnake | Cobol | UpperKebab | UpperFlat | Upper => pattern::uppercase,
            Snake | Kebab | Flat | Lower => pattern::lowercase,
            Ada | Train | Pascal | UpperCamel | Title => pattern::capital,
            Camel => pattern::camel,
            Toggle => pattern::toggle,
            Alternating => pattern::alternating,
            Sentence => pattern::sentence,
            Custom { pattern, .. } => *pattern,

            #[cfg(feature = "random")]
            Random => pattern::random,
            #[cfg(feature = "random")]
            PseudoRandom => pattern::pseudo_random,
        }
    }

    /// Split an identifier into words based on the boundaries of this case.
    /// ```
    /// use convert_case::Case;
    /// assert_eq!(
    ///     vec!["get", "Total", "Length"],
    ///     Case::Pascal.split(&"getTotalLength"),
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
    ///     vec!["get", "total", "length"],
    ///     Case::Snake.mutate(&["get", "Total", "Length"]),
    /// );
    /// ```
    pub fn mutate(self, words: &[&str]) -> Vec<String> {
        (self.pattern())(words)
    }

    /// Join a list of words into a single identifier using the delimiter of this case.
    /// ```
    /// use convert_case::Case;
    /// assert_eq!(
    ///     String::from("get_total_length"),
    ///     Case::Snake.join(&[
    ///         String::from("get"),
    ///         String::from("total"),
    ///         String::from("length")
    ///     ]),
    /// );
    /// ```
    pub fn join(self, words: &[String]) -> String {
        words.join(self.delim())
    }

    /// Array of all non-custom case enum variants.  Does not include aliases.
    pub fn all_cases() -> &'static [Case<'static>] {
        use Case::*;
        &[
            Snake,
            Constant,
            Ada,
            Kebab,
            Cobol,
            Train,
            Flat,
            UpperFlat,
            Pascal,
            Camel,
            Upper,
            Lower,
            Title,
            Sentence,
            Alternating,
            Toggle,
            #[cfg(feature = "random")]
            Random,
            #[cfg(feature = "random")]
            PseudoRandom,
        ]
    }

    /// Array with the two "random" feature cases `Random` and `PseudoRandom`.  Only
    /// defined in the "random" feature.
    #[cfg(feature = "random")]
    pub fn random_cases() -> &'static [Case<'static>] {
        use Case::*;
        &[Random, PseudoRandom]
    }

    /// Array of all the cases that do not depend on randomness.  This is all
    /// the cases not in the "random" feature.  Does not include aliases.
    pub fn deterministic_cases() -> &'static [Case<'static>] {
        use Case::*;
        &[
            Snake,
            Constant,
            Ada,
            Kebab,
            Cobol,
            Train,
            Flat,
            UpperFlat,
            Pascal,
            Camel,
            Upper,
            Lower,
            Title,
            Sentence,
            Alternating,
            Toggle,
        ]
    }
}
