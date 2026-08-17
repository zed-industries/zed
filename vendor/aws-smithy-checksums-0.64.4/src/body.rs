/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! HTTP body-wrappers that calculate and validate checksums.

pub mod cache;
pub mod calculate;
pub mod validate;

pub use cache::ChecksumCache;
