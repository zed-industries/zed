use std::{
    fmt::Display,
    ops::Deref,
    str::FromStr,
};

pub const MAX_IDENTIFIER_LEN: usize = 64;

pub const IDENTIFIER_REQUIREMENTS: &str =
    "Identifiers must start with a letter and can only contain letters, digits, and underscores.";

/// Check that a string can be used for a table name or field name in a
/// document.
///
/// We use a simplified ASCII version of Rust's syntax[^1] which also overlaps
/// with JavaScript's syntax[^2].
///
/// ```text
/// ident: start continue*
/// start: a-zA-z_
/// continue: a-zA-Z0-9_
/// ```
/// [^3] [^4]
/// To be conservative, let's also ban identifiers of entirely `_` too.
///
/// [^1]: <https://doc.rust-lang.org/reference/identifiers.html>
/// [^2]: <https://developer.mozilla.org/en-US/docs/Glossary/Identifier>
/// [^3]: <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_START%3A%5D&g=&i=>
/// [^4]: <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_CONTINUE%3A%5D&g=&i=>
pub fn check_valid_identifier(s: &str) -> anyhow::Result<()> {
    check_valid_identifier_inner(s).map_err(|e| anyhow::anyhow!(e))
}

pub fn is_valid_identifier(s: &str) -> bool {
    check_valid_identifier_inner(s).is_ok()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn min() -> Self {
        Identifier(MIN_IDENTIFIER.to_string())
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        check_valid_identifier(s)?;
        Ok(Identifier(s.to_string()))
    }
}

impl From<Identifier> for String {
    fn from(id: Identifier) -> String {
        id.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

fn check_valid_identifier_inner(s: &str) -> Result<(), String> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => (),
        Some('_') => (),
        Some(c) => {
            return Err(format!(
                "Invalid first character '{c}' in {s}: Identifiers must start with an alphabetic \
                 character or underscore"
            ))
        },
        None => return Err(format!("Identifier cannot be empty")),
    };
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return Err(format!(
                "Identifier {s} has invalid character '{c}': Identifiers can only contain \
                 alphanumeric characters or underscores"
            ));
        }
    }
    if s.len() > MAX_IDENTIFIER_LEN {
        return Err(format!(
            "Identifier is too long ({} > maximum {})",
            s.len(),
            MAX_IDENTIFIER_LEN
        ));
    }
    if s.chars().all(|c| c == '_') {
        return Err(format!(
            "Identifier {s} cannot have exclusively underscores"
        ));
    }
    Ok(())
}

pub const MIN_IDENTIFIER: &str = "A";

pub const MAX_FIELD_NAME_LENGTH: usize = 1024;

/// Check that a string can be used as field in a Convex object.
///
/// Field names cannot start with '$', must contain only non-control ASCII
/// characters, and must be at most 1024 characters long.
pub fn check_valid_field_name(s: &str) -> anyhow::Result<()> {
    check_valid_field_name_inner(s).map_err(|e| anyhow::anyhow!(e))
}

pub fn is_valid_field_name(s: &str) -> bool {
    check_valid_field_name_inner(s).is_ok()
}

fn check_valid_field_name_inner(s: &str) -> Result<(), String> {
    if s.starts_with('$') {
        return Err(format!(
            "Field name {s} starts with '$', which is reserved."
        ));
    }
    for c in s.chars() {
        if !c.is_ascii() || c.is_ascii_control() {
            return Err(format!(
                "Field name {s} has invalid character '{c}': Field names can only contain \
                 non-control ASCII characters"
            ));
        }
    }
    if s.len() > MAX_FIELD_NAME_LENGTH {
        return Err(format!(
            "Field name is too long ({} > maximum {})",
            s.len(),
            MAX_FIELD_NAME_LENGTH
        ));
    }
    Ok(())
}

#[cfg(any(test, feature = "testing"))]
pub mod arbitrary_regexes {
    use proptest::prelude::*;

    use super::Identifier;

    pub const IDENTIFIER_REGEX: &str = "[a-zA-Z_][a-zA-Z][a-zA-Z0-9_]{0,62}";
    pub const USER_IDENTIFIER_REGEX: &str = "[a-zA-Z][a-zA-Z0-9_]{0,63}";
    pub const SYSTEM_IDENTIFIER_REGEX: &str = "_[a-zA-Z][a-zA-Z0-9_]{0,62}";
    // ' ' through ~ is all non-control ASCII. First character cannot be `$` or
    // `_`. These can be longer, but keep them shorter for the sake of tests.
    pub const USER_FIELD_NAME_REGEX: &str = "[ -#%-^`-~][ -~]{0,63}";
    // Technically this can be broader, but system fields are usually valid
    // identifiers
    pub const SYSTEM_FIELD_NAME_REGEX: &str = "_[a-zA-Z][a-zA-Z0-9_]{0,62}";

    impl Arbitrary for Identifier {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![USER_IDENTIFIER_REGEX, SYSTEM_IDENTIFIER_REGEX]
                .prop_map(Identifier)
                .boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{
        arbitrary_regexes::IDENTIFIER_REGEX,
        MIN_IDENTIFIER,
    };

    proptest! {
        #![proptest_config(
            ProptestConfig { failure_persistence: None, ..ProptestConfig::default() }
        )]

        #[test]
        fn test_min_identifier(ident in IDENTIFIER_REGEX) {
            assert!(MIN_IDENTIFIER <= &ident[..]);
        }
    }
}
