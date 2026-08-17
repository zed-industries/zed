/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Errors related to AWS profile config files

use crate::env_config::parse::EnvConfigParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;

/// Failed to read or parse the profile file(s)
#[derive(Debug, Clone)]
pub enum EnvConfigFileLoadError {
    /// The profile could not be parsed
    #[non_exhaustive]
    ParseError(EnvConfigParseError),

    /// Attempt to read the AWS config file (`~/.aws/config` by default) failed with a filesystem error.
    #[non_exhaustive]
    CouldNotReadFile(CouldNotReadConfigFile),
}

impl Display for EnvConfigFileLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvConfigFileLoadError::ParseError(_err) => {
                write!(f, "could not parse profile file")
            }
            EnvConfigFileLoadError::CouldNotReadFile(err) => {
                write!(f, "could not read file `{}`", err.path.display())
            }
        }
    }
}

impl Error for EnvConfigFileLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EnvConfigFileLoadError::ParseError(err) => Some(err),
            EnvConfigFileLoadError::CouldNotReadFile(details) => Some(&details.cause),
        }
    }
}

impl From<EnvConfigParseError> for EnvConfigFileLoadError {
    fn from(err: EnvConfigParseError) -> Self {
        EnvConfigFileLoadError::ParseError(err)
    }
}

/// An error encountered while reading the AWS config file
#[derive(Debug, Clone)]
pub struct CouldNotReadConfigFile {
    pub(crate) path: PathBuf,
    pub(crate) cause: Arc<std::io::Error>,
}
