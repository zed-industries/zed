/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Credential provider augmentation through the AWS Security Token Service (STS).

pub use assume_role::{AssumeRoleProvider, AssumeRoleProviderBuilder};

mod assume_role;
pub(crate) mod util;
