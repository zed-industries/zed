use std::{
    ops::Deref,
    str::FromStr,
};

use crate::{
    identifier::MAX_IDENTIFIER_LEN,
    FunctionName,
};

pub fn check_valid_path_component(s: &str) -> anyhow::Result<()> {
    if s.len() > MAX_IDENTIFIER_LEN {
        anyhow::bail!(
            "Path component is too long ({} > maximum {}): {}...",
            s.len(),
            MAX_IDENTIFIER_LEN,
            &s[..s.len().min(MAX_IDENTIFIER_LEN)]
        );
    }
    if !s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
    {
        anyhow::bail!(
            "Path component {s} can only contain alphanumeric characters, underscores, or periods."
        );
    }
    if !s.chars().any(|c| c.is_ascii_alphanumeric()) {
        anyhow::bail!("Path component {s} must have at least one alphanumeric character.");
    }
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PathComponent(String);

impl FromStr for PathComponent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_valid_path_component(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl Deref for PathComponent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PathComponent> for String {
    fn from(p: PathComponent) -> Self {
        p.0
    }
}

impl From<FunctionName> for PathComponent {
    fn from(function_name: FunctionName) -> Self {
        function_name
            .parse()
            .expect("FunctionName isn't a valid PathComponent")
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for PathComponent {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        "_?[a-zA-Z0-9_]{1,60}(\\.js)?"
            .prop_filter_map("Invalid path component", |s| s.parse().ok())
            .boxed()
    }
}
