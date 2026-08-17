/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Config structs to programmatically customize the profile files that get loaded

use std::fmt;
use std::path::PathBuf;

/// Provides the ability to programmatically override the profile files that get loaded by the SDK.
///
/// The [`Default`] for `EnvConfigFiles` includes the default SDK config and credential files located in
/// `~/.aws/config` and `~/.aws/credentials` respectively.
///
/// Any number of config and credential files may be added to the `EnvConfigFiles` file set, with the
/// only requirement being that there is at least one of them. Custom file locations that are added
/// will produce errors if they don't exist, while the default config/credentials files paths are
/// allowed to not exist even if they're included.
///
/// # Example: Using a custom profile file path
///
/// ```no_run,ignore
/// use aws_runtime::env_config::file::{EnvConfigFiles, SharedConfigFileKind};
/// use std::sync::Arc;
///
/// # async fn example() {
/// let profile_files = EnvConfigFiles::builder()
///     .with_file(SharedConfigFileKind::Credentials, "some/path/to/credentials-file")
///     .build();
/// let sdk_config = aws_config::from_env()
///     .profile_files(profile_files)
///     .load()
///     .await;
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct EnvConfigFiles {
    pub(crate) files: Vec<EnvConfigFile>,
}

impl EnvConfigFiles {
    /// Returns a builder to create `EnvConfigFiles`
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Default for EnvConfigFiles {
    fn default() -> Self {
        Self {
            files: vec![
                EnvConfigFile::Default(EnvConfigFileKind::Config),
                EnvConfigFile::Default(EnvConfigFileKind::Credentials),
            ],
        }
    }
}

/// Profile file type (config or credentials)
#[derive(Copy, Clone, Debug)]
pub enum EnvConfigFileKind {
    /// The SDK config file that typically resides in `~/.aws/config`
    Config,
    /// The SDK credentials file that typically resides in `~/.aws/credentials`
    Credentials,
}

impl EnvConfigFileKind {
    pub(crate) fn default_path(&self) -> &'static str {
        match &self {
            EnvConfigFileKind::Credentials => "~/.aws/credentials",
            EnvConfigFileKind::Config => "~/.aws/config",
        }
    }

    pub(crate) fn override_environment_variable(&self) -> &'static str {
        match &self {
            EnvConfigFileKind::Config => "AWS_CONFIG_FILE",
            EnvConfigFileKind::Credentials => "AWS_SHARED_CREDENTIALS_FILE",
        }
    }
}

/// A single config file within a [`EnvConfigFiles`] file set.
#[derive(Clone)]
pub(crate) enum EnvConfigFile {
    /// One of the default profile files (config or credentials in their default locations)
    Default(EnvConfigFileKind),
    /// A profile file at a custom location
    FilePath {
        kind: EnvConfigFileKind,
        path: PathBuf,
    },
    /// The direct contents of a profile file
    FileContents {
        kind: EnvConfigFileKind,
        contents: String,
    },
}

impl fmt::Debug for EnvConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(kind) => f.debug_tuple("Default").field(kind).finish(),
            Self::FilePath { kind, path } => f
                .debug_struct("FilePath")
                .field("kind", kind)
                .field("path", path)
                .finish(),
            // Security: Redact the file contents since they may have credentials in them
            Self::FileContents { kind, contents: _ } => f
                .debug_struct("FileContents")
                .field("kind", kind)
                .field("contents", &"** redacted **")
                .finish(),
        }
    }
}

/// Builder for [`EnvConfigFiles`].
#[derive(Clone, Default, Debug)]
pub struct Builder {
    with_config: bool,
    with_credentials: bool,
    custom_sources: Vec<EnvConfigFile>,
}

impl Builder {
    /// Creates a new builder instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Include the default SDK config file in the list of profile files to be loaded.
    ///
    /// The default SDK config typically resides in `~/.aws/config`. When this flag is enabled,
    /// this config file will be included in the profile files that get loaded in the built
    /// [`EnvConfigFiles`] file set.
    ///
    /// This flag defaults to `false` when using the builder to construct [`EnvConfigFiles`].
    pub fn include_default_config_file(mut self, include_default_config_file: bool) -> Self {
        self.with_config = include_default_config_file;
        self
    }

    /// Include the default SDK credentials file in the list of profile files to be loaded.
    ///
    /// The default SDK credentials typically reside in `~/.aws/credentials`. When this flag is enabled,
    /// this credentials file will be included in the profile files that get loaded in the built
    /// [`EnvConfigFiles`] file set.
    ///
    /// This flag defaults to `false` when using the builder to construct [`EnvConfigFiles`].
    pub fn include_default_credentials_file(
        mut self,
        include_default_credentials_file: bool,
    ) -> Self {
        self.with_credentials = include_default_credentials_file;
        self
    }

    /// Include a custom `file` in the list of profile files to be loaded.
    ///
    /// The `kind` informs the parser how to treat the file. If it's intended to be like
    /// the SDK credentials file typically in `~/.aws/config`, then use [`EnvConfigFileKind::Config`].
    /// Otherwise, use [`EnvConfigFileKind::Credentials`].
    pub fn with_file(mut self, kind: EnvConfigFileKind, file: impl Into<PathBuf>) -> Self {
        self.custom_sources.push(EnvConfigFile::FilePath {
            kind,
            path: file.into(),
        });
        self
    }

    /// Include custom file `contents` in the list of profile files to be loaded.
    ///
    /// The `kind` informs the parser how to treat the file. If it's intended to be like
    /// the SDK credentials file typically in `~/.aws/config`, then use [`EnvConfigFileKind::Config`].
    /// Otherwise, use [`EnvConfigFileKind::Credentials`].
    pub fn with_contents(mut self, kind: EnvConfigFileKind, contents: impl Into<String>) -> Self {
        self.custom_sources.push(EnvConfigFile::FileContents {
            kind,
            contents: contents.into(),
        });
        self
    }

    /// Build the [`EnvConfigFiles`] file set.
    pub fn build(self) -> EnvConfigFiles {
        let mut files = self.custom_sources;
        if self.with_credentials {
            files.insert(0, EnvConfigFile::Default(EnvConfigFileKind::Credentials));
        }
        if self.with_config {
            files.insert(0, EnvConfigFile::Default(EnvConfigFileKind::Config));
        }
        if files.is_empty() {
            panic!("At least one profile file must be included in the `EnvConfigFiles` file set.");
        }
        EnvConfigFiles { files }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_file_contents_in_profile_file_debug() {
        let shared_config_file = EnvConfigFile::FileContents {
            kind: EnvConfigFileKind::Config,
            contents: "sensitive_contents".into(),
        };
        let debug = format!("{shared_config_file:?}");
        assert!(!debug.contains("sensitive_contents"));
        assert!(debug.contains("** redacted **"));
    }

    #[test]
    fn build_correctly_orders_default_config_credentials() {
        let shared_config_files = EnvConfigFiles::builder()
            .with_file(EnvConfigFileKind::Config, "foo")
            .include_default_credentials_file(true)
            .include_default_config_file(true)
            .build();
        assert_eq!(3, shared_config_files.files.len());
        assert!(matches!(
            shared_config_files.files[0],
            EnvConfigFile::Default(EnvConfigFileKind::Config)
        ));
        assert!(matches!(
            shared_config_files.files[1],
            EnvConfigFile::Default(EnvConfigFileKind::Credentials)
        ));
        assert!(matches!(
            shared_config_files.files[2],
            EnvConfigFile::FilePath {
                kind: EnvConfigFileKind::Config,
                path: _
            }
        ));
    }

    #[test]
    #[should_panic]
    fn empty_builder_panics() {
        EnvConfigFiles::builder().build();
    }
}
