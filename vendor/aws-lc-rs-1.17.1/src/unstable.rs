// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#![cfg(feature = "unstable")]

//! This module contains unstable/experimental APIs.
//!
//! # ⚠️ Warning
//! The APIs under this module are not stable and may change in the future.
//! They are not covered by semver guarantees.
//!
#[cfg(not(feature = "fips"))]
pub mod signature;
