/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Deprecated metadata type.

/// Metadata added to the [`ConfigBag`](aws_smithy_types::config_bag::ConfigBag) that identifies the API being called.
#[deprecated(
    since = "0.60.2",
    note = "Use aws_smithy_runtime_api::client::orchestrator::Metadata instead."
)]
pub type Metadata = aws_smithy_runtime_api::client::orchestrator::Metadata;
