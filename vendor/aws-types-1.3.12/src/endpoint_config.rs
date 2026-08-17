/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Newtypes for endpoint-related parameters
//!
//! Parameters require newtypes so they have distinct types when stored in layers in config bag.

use std::fmt;
use std::str::FromStr;

use aws_smithy_types::config_bag::{Storable, StoreReplace};

/// Newtype for `use_fips`
#[derive(Clone, Debug)]
pub struct UseFips(pub bool);
impl Storable for UseFips {
    type Storer = StoreReplace<UseFips>;
}

/// Newtype for `use_dual_stack`
#[derive(Clone, Debug)]
pub struct UseDualStack(pub bool);
impl Storable for UseDualStack {
    type Storer = StoreReplace<UseDualStack>;
}

/// Newtype for `endpoint_url`
#[derive(Clone, Debug)]
pub struct EndpointUrl(pub String);
impl Storable for EndpointUrl {
    type Storer = StoreReplace<EndpointUrl>;
}

const PREFERRED: &str = "preferred";
const DISABLED: &str = "disabled";
const REQUIRED: &str = "required";

/// Setting to control the account ID-based routing behavior.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AccountIdEndpointMode {
    /// The endpoint should include account ID if available.
    #[default]
    Preferred,
    /// A resolved endpoint does not include account ID.
    Disabled,
    /// The endpoint must include account ID. If the account ID isn't available, the SDK throws an error.
    Required,
}

impl AccountIdEndpointMode {
    fn all_variants() -> [AccountIdEndpointMode; 3] {
        use AccountIdEndpointMode::*;
        [Preferred, Disabled, Required]
    }
}

impl Storable for AccountIdEndpointMode {
    type Storer = StoreReplace<Self>;
}

impl fmt::Display for AccountIdEndpointMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        use AccountIdEndpointMode::*;
        write!(
            f,
            "{}",
            match self {
                Preferred => "preferred",
                Disabled => "disabled",
                Required => "required",
            }
        )
    }
}

impl FromStr for AccountIdEndpointMode {
    type Err = AccountIdEndpointModeParseError;

    fn from_str(mode_str: &str) -> Result<Self, Self::Err> {
        if mode_str.eq_ignore_ascii_case(PREFERRED) {
            Ok(Self::Preferred)
        } else if mode_str.eq_ignore_ascii_case(DISABLED) {
            Ok(Self::Disabled)
        } else if mode_str.eq_ignore_ascii_case(REQUIRED) {
            Ok(Self::Required)
        } else {
            Err(AccountIdEndpointModeParseError::new(mode_str))
        }
    }
}

/// Error encountered when failing to parse a string into [`AccountIdEndpointMode`].
#[derive(Debug)]
pub struct AccountIdEndpointModeParseError {
    mode_string: String,
}

impl AccountIdEndpointModeParseError {
    fn new(mode_string: impl Into<String>) -> Self {
        Self {
            mode_string: mode_string.into(),
        }
    }
}

impl fmt::Display for AccountIdEndpointModeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error parsing string `{}` as `AccountIdEndpointMode`, valid options are: {:#?}",
            self.mode_string,
            AccountIdEndpointMode::all_variants().map(|mode| mode.to_string())
        )
    }
}

impl std::error::Error for AccountIdEndpointModeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok_account_id_endpoint_mode() {
        assert_eq!(
            AccountIdEndpointMode::Preferred,
            AccountIdEndpointMode::from_str("preferred").unwrap()
        );
        assert_eq!(
            AccountIdEndpointMode::Disabled,
            AccountIdEndpointMode::from_str("disabled").unwrap()
        );
        assert_eq!(
            AccountIdEndpointMode::Required,
            AccountIdEndpointMode::from_str("required").unwrap()
        );
    }

    #[test]
    fn parse_err_account_id_endpoint_mode() {
        let err = AccountIdEndpointMode::from_str("invalid").err().unwrap();
        assert_eq!(
            r#"error parsing string `invalid` as `AccountIdEndpointMode`, valid options are: [
    "preferred",
    "disabled",
    "required",
]"#,
            format!("{err}")
        );
    }
}
