/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Re-exports for types since moved to the aws-runtime crate.

/// Use aws_runtime::env_config::file::EnvConfigFiles instead.
#[deprecated(
    since = "1.1.11",
    note = "Use aws_runtime::env_config::file::EnvConfigFiles instead."
)]
pub type ProfileFiles = aws_runtime::env_config::file::EnvConfigFiles;

/// Use aws_runtime::env_config::file::Builder instead.
#[deprecated(since = "1.1.11", note = "Use aws_runtime::env_config::file::Builder.")]
pub type Builder = aws_runtime::env_config::file::Builder;

/// Use aws_runtime::env_config::file::EnvConfigFileKind instead.
#[deprecated(
    since = "1.1.11",
    note = "Use aws_runtime::env_config::file::EnvConfigFileKind."
)]
pub type ProfileFileKind = aws_runtime::env_config::file::EnvConfigFileKind;
