use std::{
    ops::Deref,
    str::FromStr,
};

use crate::identifier::check_valid_identifier;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, derive_more::Display)]
pub struct FunctionName(Box<str>);

impl FromStr for FunctionName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_valid_identifier(s)?;
        Ok(FunctionName(s.into()))
    }
}

impl Deref for FunctionName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0[..]
    }
}

impl From<FunctionName> for String {
    fn from(function_name: FunctionName) -> Self {
        function_name.0.into_string()
    }
}

impl FunctionName {
    pub fn default_export() -> Self {
        Self("default".into())
    }

    pub fn is_default_export(&self) -> bool {
        *self.0 == *"default"
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for FunctionName {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;

        use crate::identifier::arbitrary_regexes::IDENTIFIER_REGEX;
        IDENTIFIER_REGEX
            .prop_filter_map("Invalid IdentifierFieldName", |s| {
                FunctionName::from_str(&s).ok()
            })
            .boxed()
    }
}
