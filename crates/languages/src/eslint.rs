use anyhow::{Context as _, Result};
use async_trait::async_trait;
use gpui::AsyncApp;
use http_client::{
    github::{AssetKind, GitHubLspBinaryVersion, build_asset_url},
    github_download::download_server_binary,
};
use language::{LspAdapter, LspAdapterDelegate, LspInstaller, Toolchain};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName, Uri};
use node_runtime::NodeRuntime;
use project::lsp_store::language_server_settings_for;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use settings::SettingsLocation;
use smol::{fs, stream::StreamExt};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::merge_json_value_into;
use util::{fs::remove_matching, rel_path::RelPath};

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![
        "--max-old-space-size=8192".into(),
        server_path.into(),
        "--stdio".into(),
    ]
}

pub struct EsLintLspAdapter {
    node: NodeRuntime,
}

impl EsLintLspAdapter {
    const CURRENT_VERSION: &'static str = "2.4.4";
    const CURRENT_VERSION_TAG_NAME: &'static str = "release/2.4.4";

    #[cfg(not(windows))]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    #[cfg(windows)]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;

    const SERVER_PATH: &'static str = "vscode-eslint/server/out/eslintServer.js";
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("eslint");

    const FLAT_CONFIG_FILE_NAMES: &'static [&'static str] = &[
        "eslint.config.js",
        "eslint.config.mjs",
        "eslint.config.cjs",
        "eslint.config.ts",
        "eslint.config.cts",
        "eslint.config.mts",
    ];

    pub fn new(node: NodeRuntime) -> Self {
        EsLintLspAdapter { node }
    }

    fn build_destination_path(container_dir: &Path) -> PathBuf {
        container_dir.join(format!("vscode-eslint-{}", Self::CURRENT_VERSION))
    }
}

impl LspInstaller for EsLintLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let url = build_asset_url(
            "zed-industries/vscode-eslint",
            Self::CURRENT_VERSION_TAG_NAME,
            Self::GITHUB_ASSET_KIND,
        )?;

        Ok(GitHubLspBinaryVersion {
            name: Self::CURRENT_VERSION.into(),
            digest: None,
            url,
        })
    }

    async fn fetch_server_binary(
        &self,
        version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let destination_path = Self::build_destination_path(&container_dir);
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |_| true).await;

            download_server_binary(
                &*delegate.http_client(),
                &version.url,
                None,
                &destination_path,
                Self::GITHUB_ASSET_KIND,
            )
            .await?;

            let mut dir = fs::read_dir(&destination_path).await?;
            let first = dir.next().await.context("missing first file")??;
            let repo_root = destination_path.join("vscode-eslint");
            fs::rename(first.path(), &repo_root).await?;

            #[cfg(target_os = "windows")]
            {
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("client").join("src").join("shared"),
                )
                .await?;
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("server").join("src").join("shared"),
                )
                .await?;
            }

            self.node
                .run_npm_subcommand(Some(&repo_root), "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(Some(&repo_root), "run-script", &["compile"])
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path =
            Self::build_destination_path(&container_dir).join(EsLintLspAdapter::SERVER_PATH);
        Some(LanguageServerBinary {
            path: self.node.binary_path().await.ok()?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }
}

#[async_trait(?Send)]
impl LspAdapter for EsLintLspAdapter {
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::new("source.fixAll.eslint"),
        ])
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        requested_uri: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let worktree_root = delegate.worktree_root_path();
        let use_flat_config = Self::FLAT_CONFIG_FILE_NAMES
            .iter()
            .any(|file| worktree_root.join(file).is_file());

        let mut default_workspace_configuration = json!({
            "validate": "on",
            "rulesCustomizations": [],
            "run": "onType",
            "nodePath": null,
            "workingDirectory": {
                "mode": "auto"
            },
            "workspaceFolder": {
                "uri": worktree_root,
                "name": worktree_root.file_name()
                    .unwrap_or(worktree_root.as_os_str())
                    .to_string_lossy(),
            },
            "problems": {},
            "codeActionOnSave": {
                // We enable this, but without also configuring code_actions_on_format
                // in the Zed configuration, it doesn't have an effect.
                "enable": true,
            },
            "codeAction": {
                "disableRuleComment": {
                    "enable": true,
                    "location": "separateLine",
                },
                "showDocumentation": {
                    "enable": true
                }
            },
            "experimental": {
                "useFlatConfig": use_flat_config,
            }
        });

        let file_path = requested_uri
            .as_ref()
            .and_then(|uri| {
                (uri.scheme() == "file")
                    .then(|| uri.to_file_path().ok())
                    .flatten()
            })
            .and_then(|abs_path| {
                abs_path
                    .strip_prefix(&worktree_root)
                    .ok()
                    .map(ToOwned::to_owned)
            });
        let file_path = file_path
            .and_then(|p| RelPath::unix(&p).ok().map(ToOwned::to_owned))
            .unwrap_or_else(|| RelPath::empty().to_owned());
        let override_options = cx.update(|cx| {
            language_server_settings_for(
                SettingsLocation {
                    worktree_id: delegate.worktree_id(),
                    path: &file_path,
                },
                &Self::SERVER_NAME,
                cx,
            )
            .and_then(|s| s.settings.clone())
        });

        if let Some(override_options) = override_options {
            let working_directories = override_options.get("workingDirectories").and_then(|wd| {
                serde_json::from_value::<WorkingDirectories>(wd.clone())
                    .ok()
                    .and_then(|wd| wd.0)
            });

            merge_json_value_into(override_options, &mut default_workspace_configuration);

            let working_directory = working_directories
                .zip(requested_uri)
                .and_then(|(wd, uri)| {
                    determine_working_directory(uri, wd, worktree_root.to_owned())
                });

            if let Some(working_directory) = working_directory
                && let Some(wd) = default_workspace_configuration.get_mut("workingDirectory")
            {
                *wd = serde_json::to_value(working_directory)?;
            }
        }

        Ok(json!({
            "": default_workspace_configuration
        }))
    }

    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

/// On Windows, converts Unix-style separators (/) to Windows-style (\).
/// On Unix, returns the path unchanged
fn normalize_path_separators(path: &str) -> String {
    #[cfg(windows)]
    {
        path.replace('/', "\\")
    }
    #[cfg(not(windows))]
    {
        path.to_string()
    }
}

fn determine_working_directory(
    uri: Uri,
    working_directories: Vec<WorkingDirectory>,
    workspace_folder_path: PathBuf,
) -> Option<ResultWorkingDirectory> {
    let mut working_directory = None;

    for item in working_directories {
        let mut directory: Option<String> = None;
        let mut pattern: Option<String> = None;
        let mut no_cwd = false;
        match item {
            WorkingDirectory::String(contents) => {
                directory = Some(normalize_path_separators(&contents));
            }
            WorkingDirectory::LegacyDirectoryItem(legacy_directory_item) => {
                directory = Some(normalize_path_separators(&legacy_directory_item.directory));
                no_cwd = !legacy_directory_item.change_process_cwd;
            }
            WorkingDirectory::DirectoryItem(directory_item) => {
                directory = Some(normalize_path_separators(&directory_item.directory));
                if let Some(not_cwd) = directory_item.not_cwd {
                    no_cwd = not_cwd;
                }
            }
            WorkingDirectory::PatternItem(pattern_item) => {
                pattern = Some(normalize_path_separators(&pattern_item.pattern));
                if let Some(not_cwd) = pattern_item.not_cwd {
                    no_cwd = not_cwd;
                }
            }
            WorkingDirectory::ModeItem(mode_item) => {
                working_directory = Some(ResultWorkingDirectory::ModeItem(mode_item));
                continue;
            }
        }

        let mut item_value: Option<String> = None;
        if directory.is_some() || pattern.is_some() {
            let file_path: Option<PathBuf> = (uri.scheme() == "file")
                .then(|| uri.to_file_path().ok())
                .flatten();
            if let Some(file_path) = file_path {
                if let Some(mut directory) = directory {
                    if Path::new(&directory).is_relative() {
                        directory = workspace_folder_path
                            .join(directory)
                            .to_string_lossy()
                            .to_string();
                    }
                    if !directory.ends_with(std::path::MAIN_SEPARATOR) {
                        directory.push(std::path::MAIN_SEPARATOR);
                    }
                    if file_path.starts_with(&directory) {
                        item_value = Some(directory);
                    }
                } else if let Some(mut pattern) = pattern
                    && !pattern.is_empty()
                {
                    if Path::new(&pattern).is_relative() {
                        pattern = workspace_folder_path
                            .join(pattern)
                            .to_string_lossy()
                            .to_string();
                    }
                    if !pattern.ends_with(std::path::MAIN_SEPARATOR) {
                        pattern.push(std::path::MAIN_SEPARATOR);
                    }
                    if let Some(matched) = match_glob_pattern(&pattern, &file_path) {
                        item_value = Some(matched);
                    }
                }
            }
        }
        if let Some(item_value) = item_value {
            if working_directory
                .as_ref()
                .is_none_or(|wd| matches!(wd, ResultWorkingDirectory::ModeItem(_)))
            {
                working_directory = Some(ResultWorkingDirectory::DirectoryItem(DirectoryItem {
                    directory: item_value,
                    not_cwd: Some(no_cwd),
                }));
            } else if let Some(ResultWorkingDirectory::DirectoryItem(item)) = &mut working_directory
                && item.directory.len() < item_value.len()
            {
                item.directory = item_value;
                item.not_cwd = Some(no_cwd);
            }
        }
    }

    working_directory
}

fn match_glob_pattern(pattern: &str, file_path: &Path) -> Option<String> {
    use globset::GlobBuilder;

    let glob = GlobBuilder::new(pattern)
        .literal_separator(true)
        .build()
        .ok()?
        .compile_matcher();

    let mut current = file_path.to_path_buf();
    let mut matched: Option<String> = None;

    while let Some(parent) = current.parent() {
        let mut prefix = parent.to_string_lossy().to_string();
        if !prefix.ends_with(std::path::MAIN_SEPARATOR) {
            prefix.push(std::path::MAIN_SEPARATOR);
        }
        if glob.is_match(&prefix) {
            matched = Some(prefix);
        }
        current = parent.to_path_buf();
    }

    matched
}

#[cfg(target_os = "windows")]
async fn handle_symlink(src_dir: PathBuf, dest_dir: PathBuf) -> Result<()> {
    anyhow::ensure!(
        fs::metadata(&src_dir).await.is_ok(),
        "Directory {src_dir:?} is not present"
    );
    if fs::metadata(&dest_dir).await.is_ok() {
        fs::remove_file(&dest_dir).await?;
    }
    fs::create_dir_all(&dest_dir).await?;
    let mut entries = fs::read_dir(&src_dir).await?;
    while let Some(entry) = entries.try_next().await? {
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = dest_dir.join(&entry_name);
        fs::copy(&entry_path, &dest_path).await?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct LegacyDirectoryItem {
    directory: String,
    #[serde(rename = "changeProcessCWD")]
    change_process_cwd: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DirectoryItem {
    directory: String,
    #[serde(rename = "!cwd")]
    not_cwd: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PatternItem {
    pattern: String,
    #[serde(rename = "!cwd")]
    not_cwd: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModeItem {
    mode: ModeEnum,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ModeEnum {
    Auto,
    Location,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum WorkingDirectory {
    String(String),
    LegacyDirectoryItem(LegacyDirectoryItem),
    DirectoryItem(DirectoryItem),
    PatternItem(PatternItem),
    ModeItem(ModeItem),
}
#[derive(Serialize, Deserialize)]
struct WorkingDirectories(Option<Vec<WorkingDirectory>>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultWorkingDirectory {
    ModeItem(ModeItem),
    DirectoryItem(DirectoryItem),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod glob_patterns {
        use super::*;

        #[test]
        fn test_match_glob_pattern() {
            let pattern = unix_path_to_platform("/test/*/");
            let file_path = PathBuf::from(unix_path_to_platform("/test/foo/bar/file.txt"));
            let matched = match_glob_pattern(&pattern, &file_path);
            assert_eq!(matched, Some(unix_path_to_platform("/test/foo/")));
        }

        #[test]
        fn test_match_glob_pattern_globstar() {
            let pattern = unix_path_to_platform("/workspace/**/src/");
            let file_path = PathBuf::from(unix_path_to_platform(
                "/workspace/packages/core/src/index.ts",
            ));
            let matched = match_glob_pattern(&pattern, &file_path);
            assert_eq!(
                matched,
                Some(unix_path_to_platform("/workspace/packages/core/src/"))
            );
        }

        #[test]
        fn test_match_glob_pattern_no_match() {
            let pattern = unix_path_to_platform("/other/*/");
            let file_path = PathBuf::from(unix_path_to_platform("/test/foo/bar/file.txt"));
            let matched = match_glob_pattern(&pattern, &file_path);
            assert_eq!(matched, None);
        }
    }

    mod unix_style_paths {
        use super::*;

        #[test]
        fn test_working_directory_string() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::String("packages/foo".to_string())];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_absolute_path() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::String(unix_path_to_platform(
                "/workspace/packages/foo",
            ))];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_directory_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::DirectoryItem(DirectoryItem {
                directory: "packages/foo".to_string(),
                not_cwd: Some(true),
            })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                true,
            );
        }

        #[test]
        fn test_working_directory_legacy_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories =
                vec![WorkingDirectory::LegacyDirectoryItem(LegacyDirectoryItem {
                    directory: "packages/foo".to_string(),
                    change_process_cwd: false,
                })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                true,
            );
        }

        #[test]
        fn test_working_directory_pattern_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::PatternItem(PatternItem {
                pattern: "packages/*/".to_string(),
                not_cwd: Some(false),
            })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_multiple_patterns() {
            let uri = make_uri("/workspace/apps/web/src/file.ts");
            let working_directories = vec![
                WorkingDirectory::PatternItem(PatternItem {
                    pattern: "packages/*/".to_string(),
                    not_cwd: None,
                }),
                WorkingDirectory::PatternItem(PatternItem {
                    pattern: "apps/*/".to_string(),
                    not_cwd: None,
                }),
            ];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/apps/web/"),
                false,
            );
        }
    }

    #[cfg(windows)]
    mod windows_style_paths {
        use super::*;

        #[test]
        fn test_working_directory_string() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::String("packages\\foo".to_string())];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_absolute_path() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::String(
                unix_path_to_platform("/workspace/packages/foo").replace('/', "\\"),
            )];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_directory_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::DirectoryItem(DirectoryItem {
                directory: "packages\\foo".to_string(),
                not_cwd: Some(true),
            })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                true,
            );
        }

        #[test]
        fn test_working_directory_legacy_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories =
                vec![WorkingDirectory::LegacyDirectoryItem(LegacyDirectoryItem {
                    directory: "packages\\foo".to_string(),
                    change_process_cwd: false,
                })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                true,
            );
        }

        #[test]
        fn test_working_directory_pattern_item() {
            let uri = make_uri("/workspace/packages/foo/src/file.ts");
            let working_directories = vec![WorkingDirectory::PatternItem(PatternItem {
                pattern: "packages\\*\\".to_string(),
                not_cwd: Some(false),
            })];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/packages/foo/"),
                false,
            );
        }

        #[test]
        fn test_working_directory_multiple_patterns() {
            let uri = make_uri("/workspace/apps/web/src/file.ts");
            let working_directories = vec![
                WorkingDirectory::PatternItem(PatternItem {
                    pattern: "packages\\*\\".to_string(),
                    not_cwd: None,
                }),
                WorkingDirectory::PatternItem(PatternItem {
                    pattern: "apps\\*\\".to_string(),
                    not_cwd: None,
                }),
            ];
            let workspace_folder = PathBuf::from(unix_path_to_platform("/workspace"));

            let result = determine_working_directory(uri, working_directories, workspace_folder);
            assert_directory_result(
                result,
                &unix_path_to_platform("/workspace/apps/web/"),
                false,
            );
        }
    }

    /// Converts a Unix-style path to a platform-specific path.
    /// On Windows, converts "/workspace/foo/bar" to "C:\workspace\foo\bar"
    /// On Unix, returns the path unchanged.
    fn unix_path_to_platform(path: &str) -> String {
        #[cfg(windows)]
        {
            if path.starts_with('/') {
                format!("C:{}", path.replace('/', "\\"))
            } else {
                path.replace('/', "\\")
            }
        }
        #[cfg(not(windows))]
        {
            path.to_string()
        }
    }

    fn make_uri(path: &str) -> Uri {
        let platform_path = unix_path_to_platform(path);
        Uri::from_file_path(&platform_path).unwrap()
    }

    fn assert_directory_result(
        result: Option<ResultWorkingDirectory>,
        expected_directory: &str,
        expected_not_cwd: bool,
    ) {
        match result {
            Some(ResultWorkingDirectory::DirectoryItem(item)) => {
                assert_eq!(item.directory, expected_directory);
                assert_eq!(item.not_cwd, Some(expected_not_cwd));
            }
            other => panic!("Expected DirectoryItem, got {:?}", other),
        }
    }
}
