/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types that allow users to indicate their preferences for checksum calculation and validation

// Note: These types would likely make more sense in `aws-smithy-checksums` and were originally
// added there. But we have lints protecting against exporting non-stable types from stable crates
// and the checksums crate is not yet 1.0, so these types cannot live there for now. In the future
// if we do decide to 1.0 the checksums crate we can move these types there and re-export them here
// to maintain the current behavior.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::config_bag::{Storable, StoreReplace};

// Valid names for RequestChecksumCalculation and ResponseChecksumValidation
const WHEN_SUPPORTED: &str = "when_supported";
const WHEN_REQUIRED: &str = "when_required";

/// Determines when a checksum will be calculated for request payloads. Values are:
/// * [RequestChecksumCalculation::WhenSupported] - (default) When set, a checksum will be
///   calculated for all request payloads of operations modeled with the
///   `httpChecksum` trait where `requestChecksumRequired` is `true` and/or a
///   `requestAlgorithmMember` is modeled.
/// * [RequestChecksumCalculation::WhenRequired] - When set, a checksum will only be calculated for
///   request payloads of operations modeled with the  `httpChecksum` trait where
///   `requestChecksumRequired` is `true` or where a requestAlgorithmMember
///   is modeled and supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum RequestChecksumCalculation {
    /// Calculate request checksums when they are supported.
    #[default]
    WhenSupported,
    /// Caulculate request checksums only when they are required.
    WhenRequired,
}

impl Storable for RequestChecksumCalculation {
    type Storer = StoreReplace<Self>;
}

impl FromStr for RequestChecksumCalculation {
    type Err = UnknownRequestChecksumCalculationError;

    fn from_str(request_checksum_calculation: &str) -> Result<Self, Self::Err> {
        if request_checksum_calculation.eq_ignore_ascii_case(WHEN_SUPPORTED) {
            Ok(Self::WhenSupported)
        } else if request_checksum_calculation.eq_ignore_ascii_case(WHEN_REQUIRED) {
            Ok(Self::WhenRequired)
        } else {
            Err(UnknownRequestChecksumCalculationError::new(
                request_checksum_calculation,
            ))
        }
    }
}

/// Determines when checksum validation will be performed on response payloads. Values are:
/// * [ResponseChecksumValidation::WhenSupported] - (default) When set, checksum validation is performed on all
///   response payloads of operations modeled with the `httpChecksum` trait where
///   `responseAlgorithms` is modeled, except when no modeled checksum algorithms
///   are supported.
/// * [ResponseChecksumValidation::WhenRequired] - When set, checksum validation is not performed on
///   response payloads of operations unless the checksum algorithm is supported and
///   the `requestValidationModeMember` member is set to `ENABLED`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ResponseChecksumValidation {
    /// Validate response checksums when they are supported.
    #[default]
    WhenSupported,
    /// Validate response checksums only when they are required.
    WhenRequired,
}

impl Storable for ResponseChecksumValidation {
    type Storer = StoreReplace<Self>;
}

impl FromStr for ResponseChecksumValidation {
    type Err = UnknownResponseChecksumValidationError;

    fn from_str(response_checksum_validation: &str) -> Result<Self, Self::Err> {
        if response_checksum_validation.eq_ignore_ascii_case(WHEN_SUPPORTED) {
            Ok(Self::WhenSupported)
        } else if response_checksum_validation.eq_ignore_ascii_case(WHEN_REQUIRED) {
            Ok(Self::WhenRequired)
        } else {
            Err(UnknownResponseChecksumValidationError::new(
                response_checksum_validation,
            ))
        }
    }
}

/// Unknown setting for `request_checksum_calculation`
#[derive(Debug)]
#[non_exhaustive]
pub struct UnknownRequestChecksumCalculationError {
    request_checksum_calculation: String,
}

impl UnknownRequestChecksumCalculationError {
    pub(crate) fn new(request_checksum_calculation: impl Into<String>) -> Self {
        Self {
            request_checksum_calculation: request_checksum_calculation.into(),
        }
    }

    /// The unknown value
    pub fn request_checksum_calculation(&self) -> &str {
        &self.request_checksum_calculation
    }
}

impl fmt::Display for UnknownRequestChecksumCalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"unknown request_checksum_calculation value "{}", please pass a known name ("when_supported", "when_required")"#,
            self.request_checksum_calculation
        )
    }
}

impl Error for UnknownRequestChecksumCalculationError {}

/// Unknown setting for `response_checksum_validation`
#[derive(Debug)]
#[non_exhaustive]
pub struct UnknownResponseChecksumValidationError {
    response_checksum_validation: String,
}

impl UnknownResponseChecksumValidationError {
    pub(crate) fn new(response_checksum_validation: impl Into<String>) -> Self {
        Self {
            response_checksum_validation: response_checksum_validation.into(),
        }
    }

    /// The unknown value
    pub fn response_checksum_validation(&self) -> &str {
        &self.response_checksum_validation
    }
}

impl fmt::Display for UnknownResponseChecksumValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"unknown response_checksum_validation value "{}", please pass a known name ("when_supported", "when_required")"#,
            self.response_checksum_validation
        )
    }
}

impl Error for UnknownResponseChecksumValidationError {}
