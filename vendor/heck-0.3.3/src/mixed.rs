use crate::{capitalize, lowercase, transform};

/// This trait defines a mixed case conversion.
///
/// In mixedCase, word boundaries are indicated by capital letters, excepting
/// the first word.
///
/// ## Example:
///
/// ```rust
/// use heck::MixedCase;
///
/// let sentence = "It is we who built these palaces and cities.";
/// assert_eq!(sentence.to_mixed_case(), "itIsWeWhoBuiltThesePalacesAndCities");
/// ```
pub trait MixedCase: ToOwned {
    /// Convert this type to mixed case.
    fn to_mixed_case(&self) -> Self::Owned;
}

impl MixedCase for str {
    fn to_mixed_case(&self) -> String {
        transform(
            self,
            |s, out| {
                if out.is_empty() {
                    lowercase(s, out);
                } else {
                    capitalize(s, out)
                }
            },
            |_| {},
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MixedCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_mixed_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camelCase");
    t!(test2: "This is Human case." => "thisIsHumanCase");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixedUpCamelCaseWithSomeSpaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "mixedUpSnakeCaseWithSomeSpaces");
    t!(test5: "kebab-case" => "kebabCase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shoutySnakeCase");
    t!(test7: "snake_case" => "snakeCase");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "thisContainsAllKindsOfWordBoundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxςBaﬄe");
    t!(test10: "XMLHttpRequest" => "xmlHttpRequest");
    // TODO unicode tests
}
