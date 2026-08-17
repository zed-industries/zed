use unicode_segmentation::UnicodeSegmentation;

use alloc::vec::Vec;

fn grapheme_is_digit(c: &&str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

fn grapheme_is_uppercase(c: &&str) -> bool {
    c.to_uppercase() != c.to_lowercase() && *c == c.to_uppercase()
}

fn grapheme_is_lowercase(c: &&str) -> bool {
    c.to_uppercase() != c.to_lowercase() && *c == c.to_lowercase()
}

/// Conditions for splitting an identifier into words.
///
/// Some boundaries, [`Hyphen`](Boundary::Hyphen), [`Underscore`](Boundary::Underscore), and [`Space`](Boundary::Space),
/// consume the character they split on, whereas the other boundaries do not.
///
/// `Boundary` includes methods that return useful groups of boundaries.  It also
/// contains the [`defaults_from`](Boundary::defaults_from) method which will generate a subset
/// of default boundaries based on the boundaries present in a string.
///
/// You can also create custom delimiter boundaries using the [`delim_boundary`](crate::delim_boundary)
/// macro or directly instantiate `Boundary` for complex boundary conditions.
/// ```
/// use convert_case::{Boundary, Case, Casing, Converter};
///
/// assert_eq!(
///     "TransformationsIn3D"
///         .from_case(Case::Camel)
///         .remove_boundaries(&Boundary::digit_letter())
///         .to_case(Case::Snake),
///     "transformations_in_3d",
/// );
///
/// let conv = Converter::new()
///     .set_boundaries(&Boundary::defaults_from("aA "))
///     .to_case(Case::Title);
/// assert_eq!("7empest By Tool", conv.convert("7empest byTool"));
/// ```
///
/// ## Example
///
/// For more complex boundaries, such as splitting based on the first character being a certain
/// symbol and the second is lowercase, you can instantiate a boundary directly.
///
/// ```
/// # use convert_case::{Boundary, Case, Casing};
/// let at_then_letter = Boundary::Custom {
///     condition: |s| {
///         s.get(0).map(|c| *c == "@") == Some(true)
///             && s.get(1).map(|c| *c == c.to_lowercase()) == Some(true)
///     },
///     start: 1,
///     len: 0,
/// };
/// assert_eq!(
///     "name@domain"
///         .set_boundaries(&[at_then_letter])
///         .to_case(Case::Title),
///     "Name@ Domain",
/// )
/// ```

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Boundary {
    Custom {
        /// A function that determines if this boundary is present at the start
        /// of the string.  Second argument is the `arg` field.
        condition: fn(&[&str]) -> bool,
        /// Where the beginning of the boundary is.
        start: usize,
        /// The length of the boundary.  This is the number of graphemes that
        /// are removed when splitting.
        len: usize,
    },

    /// Splits on `-`, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("-"),
    ///     vec![Boundary::Hyphen],
    /// );
    /// ```
    Hyphen,

    /// Splits on `_`, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("_"),
    ///     vec![Boundary::Underscore],
    /// );
    /// ```
    Underscore,

    /// Splits on space, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from(" "),
    ///     vec![Boundary::Space],
    /// );
    /// ```
    Space,

    /// Splits where an uppercase letter is followed by a lowercase letter.  This is seldom used,
    /// and is **not** included in the [defaults](Boundary::defaults).
    /// ```
    /// # use convert_case::Boundary;
    /// assert!(
    ///     Boundary::defaults_from("Aa").len() == 0
    /// );
    UpperLower,

    /// Splits where a lowercase letter is followed by an uppercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("aA"),
    ///     vec![Boundary::LowerUpper],
    /// );
    /// ```
    LowerUpper,

    /// Splits where digit is followed by an uppercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("1A"),
    ///     vec![Boundary::DigitUpper],
    /// );
    /// ```
    DigitUpper,

    /// Splits where an uppercase letter is followed by a digit.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("A1"),
    ///     vec![Boundary::UpperDigit],
    /// );
    /// ```
    UpperDigit,

    /// Splits where digit is followed by a lowercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("1a"),
    ///     vec![Boundary::DigitLower],
    /// );
    /// ```
    DigitLower,

    /// Splits where a lowercase letter is followed by a digit.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("a1"),
    ///     vec![Boundary::LowerDigit],
    /// );
    /// ```
    LowerDigit,

    /// Acronyms are identified by two uppercase letters followed by a lowercase letter.
    /// The word boundary is between the two uppercase letters.  For example, "HTTPRequest"
    /// would have an acronym boundary identified at "PRe" and split into "HTTP" and "Request".
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("AAa"),
    ///     vec![Boundary::Acronym],
    /// );
    /// ```
    Acronym,
}

impl Boundary {
    pub fn matches(self, s: &[&str]) -> bool {
        use Boundary::*;
        match self {
            Underscore => s.first() == Some(&"_"),
            Hyphen => s.first() == Some(&"-"),
            Space => s.first() == Some(&" "),
            Acronym => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
                    && s.get(2).map(grapheme_is_lowercase) == Some(true)
            }
            LowerUpper => {
                s.first().map(grapheme_is_lowercase) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
            }
            UpperLower => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_lowercase) == Some(true)
            }
            LowerDigit => {
                s.first().map(grapheme_is_lowercase) == Some(true)
                    && s.get(1).map(grapheme_is_digit) == Some(true)
            }
            UpperDigit => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_digit) == Some(true)
            }
            DigitLower => {
                s.first().map(grapheme_is_digit) == Some(true)
                    && s.get(1).map(grapheme_is_lowercase) == Some(true)
            }
            DigitUpper => {
                s.first().map(grapheme_is_digit) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
            }
            Custom { condition, .. } => condition(s),
        }
    }

    /// The number of graphemes consumed when splitting at the boundary.
    pub fn len(self) -> usize {
        use Boundary::*;
        match self {
            Underscore | Hyphen | Space => 1,
            LowerUpper | UpperLower | LowerDigit | UpperDigit | DigitLower | DigitUpper
            | Acronym => 0,
            Custom { len, .. } => len,
        }
    }

    /// The index of the character to split at.
    pub fn start(self) -> usize {
        use Boundary::*;
        match self {
            Underscore | Hyphen | Space => 0,
            LowerUpper | UpperLower | LowerDigit | UpperDigit | DigitLower | DigitUpper
            | Acronym => 1,
            Custom { start, .. } => start,
        }
    }

    /// The default list of boundaries used when `Casing::to_case` is called directly
    /// and in a `Converter` generated from `Converter::new()`.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults(),
    ///     [
    ///         Boundary::Underscore,
    ///         Boundary::Hyphen,
    ///         Boundary::Space,
    ///         Boundary::LowerUpper,
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper,
    ///         Boundary::Acronym,
    ///     ],
    /// );
    /// ```
    pub const fn defaults() -> [Boundary; 9] {
        [
            Boundary::Underscore,
            Boundary::Hyphen,
            Boundary::Space,
            Boundary::LowerUpper,
            Boundary::LowerDigit,
            Boundary::UpperDigit,
            Boundary::DigitLower,
            Boundary::DigitUpper,
            Boundary::Acronym,
        ]
    }

    /// Returns the boundaries that involve digits.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::digits(),
    ///     [
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper,
    ///     ],
    /// );
    /// ```
    pub const fn digits() -> [Boundary; 4] {
        [
            Boundary::LowerDigit,
            Boundary::UpperDigit,
            Boundary::DigitLower,
            Boundary::DigitUpper,
        ]
    }

    /// Returns the boundaries that are letters followed by digits.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::letter_digit(),
    ///     [
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///     ],
    /// );
    /// ```
    pub const fn letter_digit() -> [Boundary; 2] {
        [Boundary::LowerDigit, Boundary::UpperDigit]
    }

    /// Returns the boundaries that are digits followed by letters.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::digit_letter(),
    ///     [
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper
    ///     ],
    /// );
    /// ```
    pub const fn digit_letter() -> [Boundary; 2] {
        [Boundary::DigitLower, Boundary::DigitUpper]
    }

    /// Returns a list of all boundaries that are identified within the given string.
    /// Could be a short of writing out all the boundaries in a list directly.  This will not
    /// identify boundary `UpperLower` if it also used as part of `Acronym`.
    ///
    /// If you want to be very explicit and not overlap boundaries, it is recommended to use a colon
    /// character.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("aA8a -"),
    ///     vec![
    ///         Boundary::Hyphen,
    ///         Boundary::Space,
    ///         Boundary::LowerUpper,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///     ],
    /// );
    /// assert_eq!(
    ///     Boundary::defaults_from("bD:0B:_:AAa"),
    ///     vec![
    ///         Boundary::Underscore,
    ///         Boundary::LowerUpper,
    ///         Boundary::DigitUpper,
    ///         Boundary::Acronym,
    ///     ],
    /// );
    /// ```
    pub fn defaults_from(pattern: &str) -> Vec<Boundary> {
        let mut boundaries = Vec::new();
        for boundary in Boundary::defaults() {
            let parts = split(&pattern, &[boundary]);
            if parts.len() > 1 || parts.is_empty() || parts[0] != pattern {
                boundaries.push(boundary);
            }
        }
        boundaries
    }
}

/// Split an identifier into a list of words using the list of boundaries.
///
/// This is used internally for splitting an identifier before mutating by
/// a pattern and joining again with a delimiter.
/// ```
/// use convert_case::{Boundary, split};
/// assert_eq!(
///     split(&"one_two-three.four", &[Boundary::Underscore, Boundary::Hyphen]),
///     vec!["one", "two", "three.four"],
/// )
/// ```
pub fn split<'s, T>(s: &'s T, boundaries: &[Boundary]) -> Vec<&'s str>
where
    T: AsRef<str>,
{
    let s = s.as_ref();

    if s.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut last_boundary_end = 0;

    let (indices, graphemes): (Vec<_>, Vec<_>) = s.grapheme_indices(true).unzip();
    let grapheme_length = indices[graphemes.len() - 1] + graphemes[graphemes.len() - 1].len();

    // TODO:
    // swapping the order of these would be faster
    // end the loop sooner if any boundary condition is met
    // could also hit a bitvector and do the splitting at the end?  May or may not be faster
    for i in 0..graphemes.len() {
        for boundary in boundaries {
            //let byte_index = indices[i];

            if boundary.matches(&graphemes[i..]) {
                // What if we find a condition at the end of the array?
                // Maybe we can stop early based on length
                // To do this, need to switch the loops
                // TODO
                let boundary_byte_start: usize = *indices
                    .get(i + boundary.start())
                    .unwrap_or(&grapheme_length);
                let boundary_byte_end: usize = *indices
                    .get(i + boundary.start() + boundary.len())
                    .unwrap_or(&grapheme_length);

                // todo clean this up a bit
                words.push(&s[last_boundary_end..boundary_byte_start]);
                last_boundary_end = boundary_byte_end;
                break;
            }
        }
    }
    words.push(&s[last_boundary_end..]);
    //words.into_iter().filter(|s| !s.is_empty()).collect()
    words.into_iter().collect()
}

/// Create a new boundary based on a delimiter.
/// ```
/// # use convert_case::{Case, Converter, delim_boundary};
/// let conv = Converter::new()
///     .set_boundaries(&[delim_boundary!("::")])
///     .to_case(Case::Camel);
///
/// assert_eq!(
///     conv.convert("my::var::name"),
///     "myVarName",
/// )
/// ```
#[macro_export]
macro_rules! delim_boundary {
    ($delim:expr) => {
        convert_case::Boundary::Custom {
            condition: |s| s.join("").starts_with($delim),
            start: 0,
            len: $delim.len(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_equality() {
        let a = Boundary::Custom {
            condition: |_| true,
            start: 0,
            len: 0,
        };
        let b = a;

        assert_eq!(a, b)
    }

    #[test]
    fn hyphen() {
        let s = "a-b-c";
        let v = split(&s, &[Boundary::Hyphen]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn underscore() {
        let s = "a_b_c";
        let v = split(&s, &[Boundary::Underscore]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn space() {
        let s = "a b c";
        let v = split(&s, &[Boundary::Space]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn delimiters() {
        let s = "aaa-bbb_ccc ddd ddd-eee";
        let v = split(
            &s,
            &[Boundary::Space, Boundary::Underscore, Boundary::Hyphen],
        );
        assert_eq!(v, vec!["aaa", "bbb", "ccc", "ddd", "ddd", "eee"]);
    }

    #[test]
    fn lower_upper() {
        let s = "lowerUpperUpper";
        let v = split(&s, &[Boundary::LowerUpper]);
        assert_eq!(v, vec!["lower", "Upper", "Upper"]);
    }

    #[test]
    fn acronym() {
        let s = "XMLRequest";
        let v = split(&s, &[Boundary::Acronym]);
        assert_eq!(v, vec!["XML", "Request"]);
    }

    // TODO: add tests for other boundaries

    #[test]
    fn boundaries_found_in_string() {
        // upper lower is not longer a default
        assert_eq!(Vec::<Boundary>::new(), Boundary::defaults_from(".Aaaa"));
        assert_eq!(
            vec![Boundary::LowerUpper, Boundary::LowerDigit],
            Boundary::defaults_from("a8.Aa.aA")
        );
        assert_eq!(
            Boundary::digits().to_vec(),
            Boundary::defaults_from("b1B1b")
        );
        assert_eq!(
            vec![
                Boundary::Underscore,
                Boundary::Hyphen,
                Boundary::Space,
                Boundary::Acronym,
            ],
            Boundary::defaults_from("AAa -_")
        );
    }

    #[test]
    fn boundary_consts_same() {
        assert_eq!(Boundary::Space, Boundary::Space);
    }
}
