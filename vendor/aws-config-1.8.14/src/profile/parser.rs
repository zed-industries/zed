/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Code for parsing AWS profile config

use aws_runtime::env_config::file::EnvConfigFiles as ProfileFiles;
use aws_runtime::env_config::source;
use aws_types::os_shim_internal::{Env, Fs};
use std::borrow::Cow;

pub use aws_runtime::env_config::error::EnvConfigFileLoadError as ProfileFileLoadError;
pub use aws_runtime::env_config::parse::EnvConfigParseError as ProfileParseError;
pub use aws_runtime::env_config::property::Property;
pub use aws_runtime::env_config::section::{EnvConfigSections as ProfileSet, Profile};

/// Read & parse AWS config files
///
/// Loads AWS config file from the filesystem, parses them, and converts them into a [`ProfileSet`](ProfileSet).
///
/// Although the basic behavior is straightforward, there are number of nuances to maintain backwards
/// compatibility with other SDKs enumerated below.
///
#[doc = include_str!("location_of_profile_files.md")]
///
/// ## Profile file syntax
///
/// Profile files have a form similar to `.ini` but with a several edge cases. These behaviors exist
/// to match existing parser implementations, ensuring consistent behavior across AWS SDKs. These
/// cases fully enumerated in `test-data/profile-parser-tests.json`.
///
/// ### The config file `~/.aws/config`
/// ```ini
/// # ~/.aws/config
/// [profile default]
/// key = value
///
/// # profiles must begin with `profile`
/// [profile other]
/// key = value2
/// ```
///
/// ### The credentials file `~/.aws/credentials`
/// The main difference is that in ~/.aws/credentials, profiles MUST NOT be prefixed with profile:
/// ```ini
/// [default]
/// aws_access_key_id = 123
///
/// [other]
/// aws_access_key_id = 456
/// ```
pub async fn load(
    fs: &Fs,
    env: &Env,
    profile_files: &ProfileFiles,
    selected_profile_override: Option<Cow<'static, str>>,
) -> Result<ProfileSet, ProfileFileLoadError> {
    let mut source = source::load(env, fs, profile_files).await?;
    if let Some(profile) = selected_profile_override {
        source.profile = profile;
    }

    Ok(ProfileSet::parse(source)?)
}
