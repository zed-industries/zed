use crate::{transform, uppercase};

/// This trait defines a shouty kebab case conversion.
///
/// In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
/// words are in uppercase.
///
/// ## Example:
///
/// ```rust
/// use heck::ShoutyKebabCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(sentence.to_shouty_kebab_case(), "WE-ARE-GOING-TO-INHERIT-THE-EARTH");
/// ```
pub trait ShoutyKebabCase: ToOwned {
    /// Convert this type to shouty kebab case.
    fn to_shouty_kebab_case(&self) -> Self::Owned;
}

impl ShoutyKebabCase for str {
    fn to_shouty_kebab_case(&self) -> Self::Owned {
        transform(self, uppercase, |s| s.push('-'))
    }
}

#[cfg(test)]
mod tests {
    use super::ShoutyKebabCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_shouty_kebab_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "CAMEL-CASE");
    t!(test2: "This is Human case." => "THIS-IS-HUMAN-CASE");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "MIXED-UP-CAMEL-CASE-WITH-SOME-SPACES");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "MIXED-UP-SNAKE-CASE-WITH-SOME-SPACES");
    t!(test5: "kebab-case" => "KEBAB-CASE");
    t!(test6: "SHOUTY_SNAKE_CASE" => "SHOUTY-SNAKE-CASE");
    t!(test7: "snake_case" => "SNAKE-CASE");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "THIS-CONTAINS-ALL-KINDS-OF-WORD-BOUNDARIES");
    t!(test9: "XΣXΣ baﬄe" => "XΣXΣ-BAFFLE");
    t!(test10: "XMLHttpRequest" => "XML-HTTP-REQUEST");
    t!(test11: "SHOUTY-KEBAB-CASE" => "SHOUTY-KEBAB-CASE");
}
