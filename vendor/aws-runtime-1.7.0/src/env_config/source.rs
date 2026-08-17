/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Code for handling in-memory sources of profile data

use super::error::{CouldNotReadConfigFile, EnvConfigFileLoadError};
use crate::env_config::file::{EnvConfigFile, EnvConfigFileKind, EnvConfigFiles};
use crate::fs_util::{home_dir, Os};
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_types::os_shim_internal;
use std::borrow::Cow;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use tracing::{warn, Instrument};
const HOME_EXPANSION_FAILURE_WARNING: &str =
    "home directory expansion was requested (via `~` character) for the profile \
     config file path, but no home directory could be determined";

#[derive(Debug)]
/// In-memory source of profile data
pub struct Source {
    /// Profile file sources
    pub(crate) files: Vec<File>,

    /// Profile to use
    ///
    /// Overridden via `$AWS_PROFILE`, defaults to `default`
    pub profile: Cow<'static, str>,
}

#[derive(Debug)]
/// In-memory configuration file
pub struct File {
    pub(crate) kind: EnvConfigFileKind,
    pub(crate) path: Option<String>,
    pub(crate) contents: String,
}

/// Load a [`Source`] from a given environment and filesystem.
pub async fn load(
    proc_env: &os_shim_internal::Env,
    fs: &os_shim_internal::Fs,
    profile_files: &EnvConfigFiles,
) -> Result<Source, EnvConfigFileLoadError> {
    let home = home_dir(proc_env, Os::real());

    let mut files = Vec::new();
    for file in &profile_files.files {
        let file = load_config_file(file, &home, fs, proc_env)
            .instrument(tracing::debug_span!("load_config_file", file = ?file))
            .await?;
        files.push(file);
    }

    Ok(Source {
        files,
        profile: proc_env
            .get("AWS_PROFILE")
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed("default")),
    })
}

fn file_contents_to_string(path: &Path, contents: Vec<u8>) -> String {
    // if the file is not valid utf-8, log a warning and use an empty file instead
    match String::from_utf8(contents) {
        Ok(contents) => contents,
        Err(e) => {
            tracing::warn!(path = ?path, error = %DisplayErrorContext(&e), "config file did not contain utf-8 encoded data");
            Default::default()
        }
    }
}

/// Loads an AWS Config file
///
/// Both the default & the overriding patterns may contain `~/` which MUST be expanded to the users
/// home directory in a platform-aware way (see [`expand_home`]).
///
/// Arguments:
/// * `kind`: The type of config file to load
/// * `home_directory`: Home directory to use during home directory expansion
/// * `fs`: Filesystem abstraction
/// * `environment`: Process environment abstraction
async fn load_config_file(
    source: &EnvConfigFile,
    home_directory: &Option<String>,
    fs: &os_shim_internal::Fs,
    environment: &os_shim_internal::Env,
) -> Result<File, EnvConfigFileLoadError> {
    let (path, kind, contents) = match source {
        EnvConfigFile::Default(kind) => {
            let (path_is_default, path) = environment
                .get(kind.override_environment_variable())
                .map(|p| (false, Cow::Owned(p)))
                .ok()
                .unwrap_or_else(|| (true, kind.default_path().into()));
            let expanded = expand_home(path.as_ref(), path_is_default, home_directory);
            if path != expanded.to_string_lossy() {
                tracing::debug!(before = ?path, after = ?expanded, "home directory expanded");
            }
            // read the data at the specified path
            // if the path does not exist, log a warning but pretend it was actually an empty file
            let data = match fs.read_to_end(&expanded).await {
                Ok(data) => data,
                Err(e) => {
                    // Important: The default config/credentials files MUST NOT return an error
                    match e.kind() {
                        ErrorKind::NotFound if path == kind.default_path() => {
                            tracing::debug!(path = %path, "config file not found")
                        }
                        ErrorKind::NotFound if path != kind.default_path() => {
                            // in the case where the user overrode the path with an environment variable,
                            // log more loudly than the case where the default path was missing
                            tracing::warn!(path = %path, env = %kind.override_environment_variable(), "config file overridden via environment variable not found")
                        }
                        _other => {
                            tracing::warn!(path = %path, error = %DisplayErrorContext(&e), "failed to read config file")
                        }
                    };
                    Default::default()
                }
            };
            let contents = file_contents_to_string(&expanded, data);
            (Some(Cow::Owned(expanded)), kind, contents)
        }
        EnvConfigFile::FilePath { kind, path } => {
            let data = match fs.read_to_end(&path).await {
                Ok(data) => data,
                Err(e) => {
                    return Err(EnvConfigFileLoadError::CouldNotReadFile(
                        CouldNotReadConfigFile {
                            path: path.clone(),
                            cause: Arc::new(e),
                        },
                    ))
                }
            };
            (
                Some(Cow::Borrowed(path)),
                kind,
                file_contents_to_string(path, data),
            )
        }
        EnvConfigFile::FileContents { kind, contents } => (None, kind, contents.clone()),
    };
    tracing::debug!(path = ?path, size = ?contents.len(), "config file loaded");
    Ok(File {
        kind: *kind,
        // lossy is OK here, the name of this file is just for debugging purposes
        path: path.map(|p| p.to_string_lossy().into()),
        contents,
    })
}

fn expand_home(
    path: impl AsRef<Path>,
    path_is_default: bool,
    home_dir: &Option<String>,
) -> PathBuf {
    let path = path.as_ref();
    let mut components = path.components();
    let start = components.next();
    match start {
        None => path.into(), // empty path,
        Some(Component::Normal(s)) if s == "~" => {
            // do homedir replacement
            let path = match home_dir {
                Some(dir) => {
                    tracing::debug!(home = ?dir, path = ?path, "performing home directory substitution");
                    dir.clone()
                }
                None => {
                    // Only log a warning if the path was explicitly set by the customer.
                    if !path_is_default {
                        warn!(HOME_EXPANSION_FAILURE_WARNING);
                    }
                    // if we can't determine the home directory, just leave it as `~`
                    "~".into()
                }
            };
            let mut path: PathBuf = path.into();
            // rewrite the path using system-specific path separators
            for component in components {
                path.push(component);
            }
            path
        }
        // Finally, handle the case where it doesn't begin with some version of `~/`:
        // NOTE: in this case we aren't performing path rewriting. This is correct because
        // this path comes from an environment variable on the target
        // platform, so in that case, the separators should already be correct.
        _other => path.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::env_config::error::EnvConfigFileLoadError;
    use crate::env_config::file::{EnvConfigFile, EnvConfigFileKind, EnvConfigFiles};
    use crate::env_config::source::{
        expand_home, load, load_config_file, HOME_EXPANSION_FAILURE_WARNING,
    };
    use aws_types::os_shim_internal::{Env, Fs};
    use futures_util::future::FutureExt;
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use tracing_test::traced_test;

    #[test]
    fn only_expand_home_prefix() {
        // ~ is only expanded as a single component (currently)
        let path = "~aws/config";
        assert_eq!(
            expand_home(path, false, &None).to_str().unwrap(),
            "~aws/config"
        );
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct SourceTests {
        tests: Vec<TestCase>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct TestCase {
        name: String,
        environment: HashMap<String, String>,
        platform: String,
        profile: Option<String>,
        config_location: String,
        credentials_location: String,
    }

    /// Run all tests from file-location-tests.json
    #[test]
    fn run_tests() -> Result<(), Box<dyn Error>> {
        let tests = fs::read_to_string("test-data/file-location-tests.json")?;
        let tests: SourceTests = serde_json::from_str(&tests)?;
        for (i, test) in tests.tests.into_iter().enumerate() {
            eprintln!("test: {}", i);
            check(test)
                .now_or_never()
                .expect("these futures should never poll");
        }
        Ok(())
    }

    #[traced_test]
    #[test]
    fn logs_produced_default() {
        let env = Env::from_slice(&[("HOME", "/user/name")]);
        let mut fs = HashMap::new();
        fs.insert(
            "/user/name/.aws/config".to_string(),
            "[default]\nregion = us-east-1",
        );

        let fs = Fs::from_map(fs);

        let _src = load(&env, &fs, &Default::default()).now_or_never();
        assert!(logs_contain("config file loaded"));
        assert!(logs_contain("performing home directory substitution"));
    }

    #[traced_test]
    #[test]
    fn load_config_file_should_not_emit_warning_when_path_not_explicitly_set() {
        let env = Env::from_slice(&[]);
        let fs = Fs::from_slice(&[]);

        let _src = load_config_file(
            &EnvConfigFile::Default(EnvConfigFileKind::Config),
            &None,
            &fs,
            &env,
        )
        .now_or_never();
        assert!(!logs_contain(HOME_EXPANSION_FAILURE_WARNING));
    }

    #[traced_test]
    #[test]
    fn load_config_file_should_emit_warning_when_path_explicitly_set() {
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "~/some/path")]);
        let fs = Fs::from_slice(&[]);

        let _src = load_config_file(
            &EnvConfigFile::Default(EnvConfigFileKind::Config),
            &None,
            &fs,
            &env,
        )
        .now_or_never();
        assert!(logs_contain(HOME_EXPANSION_FAILURE_WARNING));
    }

    async fn check(test_case: TestCase) {
        let fs = Fs::real();
        let env = Env::from(test_case.environment);
        let platform_matches = (cfg!(windows) && test_case.platform == "windows")
            || (!cfg!(windows) && test_case.platform != "windows");
        if platform_matches {
            let source = load(&env, &fs, &Default::default()).await.unwrap();
            if let Some(expected_profile) = test_case.profile {
                assert_eq!(source.profile, expected_profile, "{}", &test_case.name);
            }
            assert_eq!(
                source.files[0].path,
                Some(test_case.config_location),
                "{}",
                &test_case.name
            );
            assert_eq!(
                source.files[1].path,
                Some(test_case.credentials_location),
                "{}",
                &test_case.name
            )
        } else {
            println!(
                "NOTE: ignoring test case for {} which does not apply to our platform: \n  {}",
                &test_case.platform, &test_case.name
            )
        }
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn test_expand_home() {
        let path = "~/.aws/config";
        assert_eq!(
            expand_home(path, false, &Some("/user/foo".to_string()))
                .to_str()
                .unwrap(),
            "/user/foo/.aws/config"
        );
    }

    #[test]
    fn expand_home_no_home() {
        // there is an edge case around expansion when no home directory exists
        // if no home directory can be determined, leave the path as is
        if !cfg!(windows) {
            assert_eq!(
                expand_home("~/config", false, &None).to_str().unwrap(),
                "~/config"
            )
        } else {
            assert_eq!(
                expand_home("~/config", false, &None).to_str().unwrap(),
                "~\\config"
            )
        }
    }

    /// Test that a linux oriented path expands on windows
    #[test]
    #[cfg_attr(not(windows), ignore)]
    fn test_expand_home_windows() {
        let path = "~/.aws/config";
        assert_eq!(
            expand_home(path, true, &Some("C:\\Users\\name".to_string()),)
                .to_str()
                .unwrap(),
            "C:\\Users\\name\\.aws\\config"
        );
    }

    #[tokio::test]
    async fn programmatically_set_credentials_file_contents() {
        let contents = "[default]\n\
            aws_access_key_id = AKIAFAKE\n\
            aws_secret_access_key = FAKE\n\
            ";
        let env = Env::from_slice(&[]);
        let fs = Fs::from_slice(&[]);
        let profile_files = EnvConfigFiles::builder()
            .with_contents(EnvConfigFileKind::Credentials, contents)
            .build();
        let source = load(&env, &fs, &profile_files).await.unwrap();
        assert_eq!(1, source.files.len());
        assert_eq!("default", source.profile);
        assert_eq!(contents, source.files[0].contents);
    }

    #[tokio::test]
    async fn programmatically_set_credentials_file_path() {
        let contents = "[default]\n\
            aws_access_key_id = AKIAFAKE\n\
            aws_secret_access_key = FAKE\n\
            ";
        let mut fs = HashMap::new();
        fs.insert(
            "/custom/path/to/credentials".to_string(),
            contents.to_string(),
        );

        let fs = Fs::from_map(fs);
        let env = Env::from_slice(&[]);
        let profile_files = EnvConfigFiles::builder()
            .with_file(
                EnvConfigFileKind::Credentials,
                "/custom/path/to/credentials",
            )
            .build();
        let source = load(&env, &fs, &profile_files).await.unwrap();
        assert_eq!(1, source.files.len());
        assert_eq!("default", source.profile);
        assert_eq!(contents, source.files[0].contents);
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[tokio::test]
    #[cfg_attr(windows, ignore)]
    async fn programmatically_include_default_files() {
        let config_contents = "[default]\nregion = us-east-1";
        let credentials_contents = "[default]\n\
            aws_access_key_id = AKIAFAKE\n\
            aws_secret_access_key = FAKE\n\
            ";
        let custom_contents = "[profile some-profile]\n\
            aws_access_key_id = AKIAFAKEOTHER\n\
            aws_secret_access_key = FAKEOTHER\n\
            ";
        let mut fs = HashMap::new();
        fs.insert(
            "/user/name/.aws/config".to_string(),
            config_contents.to_string(),
        );
        fs.insert(
            "/user/name/.aws/credentials".to_string(),
            credentials_contents.to_string(),
        );

        let fs = Fs::from_map(fs);
        let env = Env::from_slice(&[("HOME", "/user/name")]);
        let profile_files = EnvConfigFiles::builder()
            .with_contents(EnvConfigFileKind::Config, custom_contents)
            .include_default_credentials_file(true)
            .include_default_config_file(true)
            .build();
        let source = load(&env, &fs, &profile_files).await.unwrap();
        assert_eq!(3, source.files.len());
        assert_eq!("default", source.profile);
        assert_eq!(config_contents, source.files[0].contents);
        assert_eq!(credentials_contents, source.files[1].contents);
        assert_eq!(custom_contents, source.files[2].contents);
    }

    #[tokio::test]
    async fn default_files_must_not_error() {
        let custom_contents = "[profile some-profile]\n\
            aws_access_key_id = AKIAFAKEOTHER\n\
            aws_secret_access_key = FAKEOTHER\n\
            ";

        let fs = Fs::from_slice(&[]);
        let env = Env::from_slice(&[("HOME", "/user/name")]);
        let profile_files = EnvConfigFiles::builder()
            .with_contents(EnvConfigFileKind::Config, custom_contents)
            .include_default_credentials_file(true)
            .include_default_config_file(true)
            .build();
        let source = load(&env, &fs, &profile_files).await.unwrap();
        assert_eq!(3, source.files.len());
        assert_eq!("default", source.profile);
        assert_eq!("", source.files[0].contents);
        assert_eq!("", source.files[1].contents);
        assert_eq!(custom_contents, source.files[2].contents);
    }

    #[tokio::test]
    async fn misconfigured_programmatic_custom_profile_path_must_error() {
        let fs = Fs::from_slice(&[]);
        let env = Env::from_slice(&[]);
        let profile_files = EnvConfigFiles::builder()
            .with_file(EnvConfigFileKind::Config, "definitely-doesnt-exist")
            .build();
        assert!(matches!(
            load(&env, &fs, &profile_files).await,
            Err(EnvConfigFileLoadError::CouldNotReadFile(_))
        ));
    }
}
