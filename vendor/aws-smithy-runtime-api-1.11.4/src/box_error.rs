/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// A boxed error that is `Send` and `Sync`.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
