/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

mod cache;
pub use cache::{IdentityCache, LazyCacheBuilder};

/// Identity resolver implementation for "no auth".
pub mod no_auth;
