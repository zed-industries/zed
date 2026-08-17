/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Providers that load configuration from environment variables

use std::error::Error;
use std::fmt;

/// Load credentials from the environment
pub mod credentials;
pub use credentials::EnvironmentVariableCredentialsProvider;

/// Load regions from the environment
pub mod region;
pub use region::EnvironmentVariableRegionProvider;

#[derive(Debug)]
pub(crate) struct InvalidBooleanValue {
    value: String,
}

impl fmt::Display for InvalidBooleanValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid boolean", self.value)
    }
}

impl Error for InvalidBooleanValue {}

pub(crate) fn parse_bool(value: &str) -> Result<bool, InvalidBooleanValue> {
    if value.eq_ignore_ascii_case("false") {
        Ok(false)
    } else if value.eq_ignore_ascii_case("true") {
        Ok(true)
    } else {
        Err(InvalidBooleanValue {
            value: value.to_string(),
        })
    }
}

#[derive(Debug)]
pub(crate) struct InvalidUintValue {
    value: String,
}

impl fmt::Display for InvalidUintValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid u32", self.value)
    }
}

impl Error for InvalidUintValue {}

pub(crate) fn parse_uint(value: &str) -> Result<u32, InvalidUintValue> {
    value.parse::<u32>().map_err(|_| InvalidUintValue {
        value: value.to_string(),
    })
}

#[derive(Debug)]
pub(crate) struct InvalidUrlValue {
    value: String,
}

impl fmt::Display for InvalidUrlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid URL", self.value)
    }
}

impl Error for InvalidUrlValue {}

pub(crate) fn parse_url(value: &str) -> Result<String, InvalidUrlValue> {
    match url::Url::parse(value) {
        // We discard the parse result because it includes a trailing slash
        Ok(_) => Ok(value.to_string()),
        Err(_) => Err(InvalidUrlValue {
            value: value.to_string(),
        }),
    }
}
