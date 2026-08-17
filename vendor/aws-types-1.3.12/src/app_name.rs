/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! New-type for a configurable app name.

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

static APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED: AtomicBool = AtomicBool::new(false);

/// App name that can be configured with an AWS SDK client to become part of the user agent string.
///
/// This name is used to identify the application in the user agent that gets sent along with requests.
///
/// The name may only have alphanumeric characters and any of these characters:
/// ```text
/// !#$%&'*+-.^_`|~
/// ```
/// Spaces are not allowed.
///
/// App names are recommended to be no more than 50 characters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppName(Cow<'static, str>);

impl AsRef<str> for AppName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AppName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Storable for AppName {
    type Storer = StoreReplace<AppName>;
}

impl AppName {
    /// Creates a new app name.
    ///
    /// This will return an `InvalidAppName` error if the given name doesn't meet the
    /// character requirements. See [`AppName`] for details on these requirements.
    pub fn new(app_name: impl Into<Cow<'static, str>>) -> Result<Self, InvalidAppName> {
        let app_name = app_name.into();

        if app_name.is_empty() {
            return Err(InvalidAppName);
        }
        fn valid_character(c: char) -> bool {
            match c {
                _ if c.is_ascii_alphanumeric() => true,
                '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '.' | '^' | '_' | '`'
                | '|' | '~' => true,
                _ => false,
            }
        }
        if !app_name.chars().all(valid_character) {
            return Err(InvalidAppName);
        }
        if app_name.len() > 50 {
            if let Ok(false) = APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED.compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                tracing::warn!(
                    "The `app_name` set when configuring the SDK client is recommended \
                     to have no more than 50 characters."
                )
            }
        }
        Ok(Self(app_name))
    }
}

/// Error for when an app name doesn't meet character requirements.
///
/// See [`AppName`] for details on these requirements.
#[derive(Debug)]
#[non_exhaustive]
pub struct InvalidAppName;

impl Error for InvalidAppName {}

impl fmt::Display for InvalidAppName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The app name can only have alphanumeric characters, or any of \
             '!' |  '#' |  '$' |  '%' |  '&' |  '\\'' |  '*' |  '+' |  '-' | \
             '.' |  '^' |  '_' |  '`' |  '|' |  '~'"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppName;
    use crate::app_name::APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED;
    use std::sync::atomic::Ordering;

    #[test]
    fn validation() {
        assert!(AppName::new("asdf1234ASDF!#$%&'*+-.^_`|~").is_ok());
        assert!(AppName::new("foo bar").is_err());
        assert!(AppName::new("🚀").is_err());
        assert!(AppName::new("").is_err());
    }

    #[tracing_test::traced_test]
    #[test]
    fn log_warn_once() {
        // Pre-condition: make sure we start in the expected state of having never logged this
        assert!(!APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED.load(Ordering::Relaxed));

        // Verify a short app name doesn't log
        AppName::new("not-long").unwrap();
        assert!(!logs_contain(
            "is recommended to have no more than 50 characters"
        ));
        assert!(!APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED.load(Ordering::Relaxed));

        // Verify a long app name logs
        AppName::new("greaterthanfiftycharactersgreaterthanfiftycharacters").unwrap();
        assert!(logs_contain(
            "is recommended to have no more than 50 characters"
        ));
        assert!(APP_NAME_LEN_RECOMMENDATION_WARN_EMITTED.load(Ordering::Relaxed));

        // Now verify it only logs once ever

        // HACK: there's no way to reset tracing-test, so just
        // reach into its internals and clear it manually
        tracing_test::internal::global_buf().lock().unwrap().clear();

        AppName::new("greaterthanfiftycharactersgreaterthanfiftycharacters").unwrap();
        assert!(!logs_contain(
            "is recommended to have no more than 50 characters"
        ));
    }
}
