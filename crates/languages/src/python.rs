use anyhow::{Context as _, ensure};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{AsyncBufReadExt, StreamExt as _};
use gpui::{App, AsyncApp, SharedString, Task};
use http_client::github::{AssetKind, GitHubLspBinaryVersion, latest_github_release};
use language::language_settings::language_settings;
use language::{ContextLocation, LanguageToolchainStore, LspInstaller};
use language::{ContextProvider, LspAdapter, LspAdapterDelegate};
use language::{LanguageName, ManifestName, ManifestProvider, ManifestQuery};
use language::{Toolchain, ToolchainList, ToolchainLister, ToolchainMetadata};
use lsp::LanguageServerBinary;
use lsp::LanguageServerName;
use node_runtime::{NodeRuntime, VersionStrategy};
use pet_core::Configuration;
use pet_core::os_environment::Environment;
use pet_core::python_environment::{PythonEnvironment, PythonEnvironmentKind};
use pet_virtualenv::is_virtualenv_dir;
use project::Fs;
use project::lsp_store::language_server_settings;
use serde_json::{Value, json};
use smol::lock::OnceCell;
use std::cmp::Ordering;
use std::env::consts;
use util::fs::{make_file_executable, remove_matching};

use parking_lot::Mutex;
use std::str::FromStr;
use std::{
    borrow::Cow,
    fmt::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{ShellKind, TaskTemplate, TaskTemplates, VariableName};
use util::{ResultExt, maybe};

use crate::github_download::{GithubBinaryMetadata, download_server_binary};

pub(crate) struct PyprojectTomlManifestProvider;

impl ManifestProvider for PyprojectTomlManifestProvider {
    fn name(&self) -> ManifestName {
        SharedString::new_static("pyproject.toml").into()
    }

    fn search(
        &self,
        ManifestQuery {
            path,
            depth,
            delegate,
        }: ManifestQuery,
    ) -> Option<Arc<Path>> {
        for path in path.ancestors().take(depth) {
            let p = path.join("pyproject.toml");
            if delegate.exists(&p, Some(false)) {
                return Some(path.into());
            }
        }

        None
    }
}

enum TestRunner {
    UNITTEST,
    PYTEST,
}

impl FromStr for TestRunner {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "unittest" => Ok(Self::UNITTEST),
            "pytest" => Ok(Self::PYTEST),
            _ => Err(()),
        }
    }
}

/// Pyright assigns each completion item a `sortText` of the form `XX.YYYY.name`.
/// Where `XX` is the sorting category, `YYYY` is based on most recent usage,
/// and `name` is the symbol name itself.
///
/// The problem with it is that Pyright adjusts the sort text based on previous resolutions (items for which we've issued `completion/resolve` call have their sortText adjusted),
/// which - long story short - makes completion items list non-stable. Pyright probably relies on VSCode's implementation detail.
/// see https://github.com/microsoft/pyright/blob/95ef4e103b9b2f129c9320427e51b73ea7cf78bd/packages/pyright-internal/src/languageService/completionProvider.ts#LL2873
fn process_pyright_completions(items: &mut [lsp::CompletionItem]) {
    for item in items {
        item.sort_text.take();
    }
}

pub struct TyLspAdapter {
    fs: Arc<dyn Fs>,
}

#[cfg(target_os = "macos")]
impl TyLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    const ARCH_SERVER_NAME: &str = "apple-darwin";
}

#[cfg(target_os = "linux")]
impl TyLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Gz;
    const ARCH_SERVER_NAME: &str = "unknown-linux-gnu";
}

#[cfg(target_os = "freebsd")]
impl TyLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Gz;
    const ARCH_SERVER_NAME: &str = "unknown-freebsd";
}

#[cfg(target_os = "windows")]
impl TyLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;
    const ARCH_SERVER_NAME: &str = "pc-windows-msvc";
}

impl TyLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("ty");

    pub fn new(fs: Arc<dyn Fs>) -> TyLspAdapter {
        TyLspAdapter { fs }
    }

    fn build_asset_name() -> Result<(String, String)> {
        let arch = match consts::ARCH {
            "x86" => "i686",
            _ => consts::ARCH,
        };
        let os = Self::ARCH_SERVER_NAME;
        let suffix = match consts::OS {
            "windows" => "zip",
            _ => "tar.gz",
        };
        let asset_name = format!("ty-{arch}-{os}.{suffix}");
        let asset_stem = format!("ty-{arch}-{os}");
        Ok((asset_stem, asset_name))
    }
}

#[async_trait(?Send)]
impl LspAdapter for TyLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        toolchain: Option<Toolchain>,
        _cx: &mut AsyncApp,
    ) -> Result<Value> {
        let mut ret = json!({});
        if let Some(toolchain) = toolchain.and_then(|toolchain| {
            serde_json::from_value::<PythonEnvironment>(toolchain.as_json).ok()
        }) {
            _ = maybe!({
                let uri = url::Url::from_file_path(toolchain.executable?).ok()?;
                let sys_prefix = toolchain.prefix.clone()?;
                let environment = json!({
                    "executable": {
                        "uri": uri,
                        "sysPrefix": sys_prefix
                    }
                });
                ret.as_object_mut()?.insert(
                    "pythonExtension".into(),
                    json!({ "activeEnvironment": environment }),
                );
                Some(())
            });
        }
        Ok(json!({"ty": ret}))
    }
}

impl LspInstaller for TyLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;
    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        let release =
            latest_github_release("astral-sh/ty", true, true, delegate.http_client()).await?;
        let (_, asset_name) = Self::build_asset_name()?;
        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url,
            digest: asset.digest,
        })
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let GitHubLspBinaryVersion {
            name,
            url,
            digest: expected_digest,
        } = latest_version;
        let destination_path = container_dir.join(format!("ty-{name}"));

        async_fs::create_dir_all(&destination_path).await?;

        let server_path = match Self::GITHUB_ASSET_KIND {
            AssetKind::TarGz | AssetKind::Gz => destination_path
                .join(Self::build_asset_name()?.0)
                .join("ty"),
            AssetKind::Zip => destination_path.clone().join("ty.exe"),
        };

        let binary = LanguageServerBinary {
            path: server_path.clone(),
            env: None,
            arguments: vec!["server".into()],
        };

        let metadata_path = destination_path.with_extension("metadata");
        let metadata = GithubBinaryMetadata::read_from_file(&metadata_path)
            .await
            .ok();
        if let Some(metadata) = metadata {
            let validity_check = async || {
                delegate
                    .try_exec(LanguageServerBinary {
                        path: server_path.clone(),
                        arguments: vec!["--version".into()],
                        env: None,
                    })
                    .await
                    .inspect_err(|err| {
                        log::warn!("Unable to run {server_path:?} asset, redownloading: {err}",)
                    })
            };
            if let (Some(actual_digest), Some(expected_digest)) =
                (&metadata.digest, &expected_digest)
            {
                if actual_digest == expected_digest {
                    if validity_check().await.is_ok() {
                        return Ok(binary);
                    }
                } else {
                    log::info!(
                        "SHA-256 mismatch for {destination_path:?} asset, downloading new asset. Expected: {expected_digest}, Got: {actual_digest}"
                    );
                }
            } else if validity_check().await.is_ok() {
                return Ok(binary);
            }
        }

        download_server_binary(
            delegate,
            &url,
            expected_digest.as_deref(),
            &destination_path,
            Self::GITHUB_ASSET_KIND,
        )
        .await?;
        make_file_executable(&server_path).await?;
        remove_matching(&container_dir, |path| path != destination_path).await;
        GithubBinaryMetadata::write_to_file(
            &GithubBinaryMetadata {
                metadata_version: 1,
                digest: expected_digest,
            },
            &metadata_path,
        )
        .await?;

        Ok(LanguageServerBinary {
            path: server_path,
            env: None,
            arguments: vec!["server".into()],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let mut last = None;
            let mut entries = self.fs.read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let path = entry?;
                if path.extension().is_some_and(|ext| ext == "metadata") {
                    continue;
                }
                last = Some(path);
            }

            let path = last.context("no cached binary")?;
            let path = match TyLspAdapter::GITHUB_ASSET_KIND {
                AssetKind::TarGz | AssetKind::Gz => {
                    path.join(Self::build_asset_name()?.0).join("ty")
                }
                AssetKind::Zip => path.join("ty.exe"),
            };

            anyhow::Ok(LanguageServerBinary {
                path,
                env: None,
                arguments: vec!["server".into()],
            })
        })
        .await
        .log_err()
    }
}

pub struct PyrightLspAdapter {
    node: NodeRuntime,
}

impl PyrightLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("pyright");
    const SERVER_PATH: &str = "node_modules/pyright/langserver.index.js";
    const NODE_MODULE_RELATIVE_SERVER_PATH: &str = "pyright/langserver.index.js";

    pub fn new(node: NodeRuntime) -> Self {
        PyrightLspAdapter { node }
    }

    async fn get_cached_server_binary(
        container_dir: PathBuf,
        node: &NodeRuntime,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);
        if server_path.exists() {
            Some(LanguageServerBinary {
                path: node.binary_path().await.log_err()?,
                env: None,
                arguments: vec![server_path.into(), "--stdio".into()],
            })
        } else {
            log::error!("missing executable in directory {:?}", server_path);
            None
        }
    }
}

#[async_trait(?Send)]
impl LspAdapter for PyrightLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        // Provide minimal initialization options
        // Virtual environment configuration will be handled through workspace configuration
        Ok(Some(json!({
            "python": {
                "analysis": {
                    "autoSearchPaths": true,
                    "useLibraryCodeForTypes": true,
                    "autoImportCompletions": true
                }
            }
        })))
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        process_pyright_completions(items);
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let label = &item.label;
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            lsp::CompletionItemKind::METHOD => grammar.highlight_id_for_name("function.method"),
            lsp::CompletionItemKind::FUNCTION => grammar.highlight_id_for_name("function"),
            lsp::CompletionItemKind::CLASS => grammar.highlight_id_for_name("type"),
            lsp::CompletionItemKind::CONSTANT => grammar.highlight_id_for_name("constant"),
            lsp::CompletionItemKind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => {
                return None;
            }
        };
        let filter_range = item
            .filter_text
            .as_deref()
            .and_then(|filter| label.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..label.len());
        let mut text = label.clone();
        if let Some(completion_details) = item
            .label_details
            .as_ref()
            .and_then(|details| details.description.as_ref())
        {
            write!(&mut text, " {}", completion_details).ok();
        }
        Some(language::CodeLabel {
            runs: highlight_id
                .map(|id| (0..label.len(), id))
                .into_iter()
                .collect(),
            text,
            filter_range,
        })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("def {}():\n", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("class {}:", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("{} = 0", name);
                let filter_range = 0..name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(language::CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }

    async fn workspace_configuration(
        self: Arc<Self>,

        adapter: &Arc<dyn LspAdapterDelegate>,
        toolchain: Option<Toolchain>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        cx.update(move |cx| {
            let mut user_settings =
                language_server_settings(adapter.as_ref(), &Self::SERVER_NAME, cx)
                    .and_then(|s| s.settings.clone())
                    .unwrap_or_default();

            // If we have a detected toolchain, configure Pyright to use it
            if let Some(toolchain) = toolchain
                && let Ok(env) = serde_json::from_value::<
                    pet_core::python_environment::PythonEnvironment,
                >(toolchain.as_json.clone())
            {
                if !user_settings.is_object() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();

                let interpreter_path = toolchain.path.to_string();
                if let Some(venv_dir) = env.prefix {
                    // Set venvPath and venv at the root level
                    // This matches the format of a pyrightconfig.json file
                    if let Some(parent) = venv_dir.parent() {
                        // Use relative path if the venv is inside the workspace
                        let venv_path = if parent == adapter.worktree_root_path() {
                            ".".to_string()
                        } else {
                            parent.to_string_lossy().into_owned()
                        };
                        object.insert("venvPath".to_string(), Value::String(venv_path));
                    }

                    if let Some(venv_name) = venv_dir.file_name() {
                        object.insert(
                            "venv".to_owned(),
                            Value::String(venv_name.to_string_lossy().into_owned()),
                        );
                    }
                }

                // Always set the python interpreter path
                // Get or create the python section
                let python = object
                    .entry("python")
                    .and_modify(|v| {
                        if !v.is_object() {
                            *v = Value::Object(serde_json::Map::default());
                        }
                    })
                    .or_insert(Value::Object(serde_json::Map::default()));
                let python = python.as_object_mut().unwrap();

                // Set both pythonPath and defaultInterpreterPath for compatibility
                python.insert(
                    "pythonPath".to_owned(),
                    Value::String(interpreter_path.clone()),
                );
                python.insert(
                    "defaultInterpreterPath".to_owned(),
                    Value::String(interpreter_path),
                );
            }

            user_settings
        })
    }
}

impl LspInstaller for PyrightLspAdapter {
    type BinaryVersion = String;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<String> {
        self.node
            .npm_package_latest_version(Self::SERVER_NAME.as_ref())
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        if let Some(pyright_bin) = delegate.which("pyright-langserver".as_ref()).await {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: pyright_bin,
                env: Some(env),
                arguments: vec!["--stdio".into()],
            })
        } else {
            let node = delegate.which("node".as_ref()).await?;
            let (node_modules_path, _) = delegate
                .npm_package_installed_version(Self::SERVER_NAME.as_ref())
                .await
                .log_err()??;

            let path = node_modules_path.join(Self::NODE_MODULE_RELATIVE_SERVER_PATH);

            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: node,
                env: Some(env),
                arguments: vec![path.into(), "--stdio".into()],
            })
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);

        self.node
            .npm_install_packages(
                &container_dir,
                &[(Self::SERVER_NAME.as_ref(), latest_version.as_str())],
            )
            .await?;

        let env = delegate.shell_env().await;
        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: Some(env),
            arguments: vec![server_path.into(), "--stdio".into()],
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &Self::BinaryVersion,
        container_dir: &PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::SERVER_NAME.as_ref(),
                &server_path,
                container_dir,
                VersionStrategy::Latest(version),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: Some(env),
                arguments: vec![server_path.into(), "--stdio".into()],
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let mut binary = Self::get_cached_server_binary(container_dir, &self.node).await?;
        binary.env = Some(delegate.shell_env().await);
        Some(binary)
    }
}

pub(crate) struct PythonContextProvider;

const PYTHON_TEST_TARGET_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_TEST_TARGET"));

const PYTHON_ACTIVE_TOOLCHAIN_PATH: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_ACTIVE_ZED_TOOLCHAIN"));

const PYTHON_MODULE_NAME_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_MODULE_NAME"));

impl ContextProvider for PythonContextProvider {
    fn build_context(
        &self,
        variables: &task::TaskVariables,
        location: ContextLocation<'_>,
        _: Option<HashMap<String, String>>,
        toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut gpui::App,
    ) -> Task<Result<task::TaskVariables>> {
        let test_target =
            match selected_test_runner(location.file_location.buffer.read(cx).file(), cx) {
                TestRunner::UNITTEST => self.build_unittest_target(variables),
                TestRunner::PYTEST => self.build_pytest_target(variables),
            };

        let module_target = self.build_module_target(variables);
        let location_file = location.file_location.buffer.read(cx).file().cloned();
        let worktree_id = location_file.as_ref().map(|f| f.worktree_id(cx));

        cx.spawn(async move |cx| {
            let active_toolchain = if let Some(worktree_id) = worktree_id {
                let file_path = location_file
                    .as_ref()
                    .and_then(|f| f.path().parent())
                    .map(Arc::from)
                    .unwrap_or_else(|| Arc::from("".as_ref()));

                toolchains
                    .active_toolchain(worktree_id, file_path, "Python".into(), cx)
                    .await
                    .map_or_else(
                        || String::from("python3"),
                        |toolchain| toolchain.path.to_string(),
                    )
            } else {
                String::from("python3")
            };

            let toolchain = (PYTHON_ACTIVE_TOOLCHAIN_PATH, active_toolchain);

            Ok(task::TaskVariables::from_iter(
                test_target
                    .into_iter()
                    .chain(module_target.into_iter())
                    .chain([toolchain]),
            ))
        })
    }

    fn associated_tasks(
        &self,
        file: Option<Arc<dyn language::File>>,
        cx: &App,
    ) -> Task<Option<TaskTemplates>> {
        let test_runner = selected_test_runner(file.as_ref(), cx);

        let mut tasks = vec![
            // Execute a selection
            TaskTemplate {
                label: "execute selection".to_owned(),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![
                    "-c".to_owned(),
                    VariableName::SelectedText.template_value_with_whitespace(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Execute an entire file
            TaskTemplate {
                label: format!("run '{}'", VariableName::File.template_value()),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![VariableName::File.template_value_with_whitespace()],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            },
            // Execute a file as module
            TaskTemplate {
                label: format!("run module '{}'", VariableName::File.template_value()),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![
                    "-m".to_owned(),
                    PYTHON_MODULE_NAME_TASK_VARIABLE.template_value(),
                ],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                tags: vec!["python-module-main-method".to_owned()],
                ..TaskTemplate::default()
            },
        ];

        tasks.extend(match test_runner {
            TestRunner::UNITTEST => {
                [
                    // Run tests for an entire file
                    TaskTemplate {
                        label: format!("unittest '{}'", VariableName::File.template_value()),
                        command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                        args: vec![
                            "-m".to_owned(),
                            "unittest".to_owned(),
                            VariableName::File.template_value_with_whitespace(),
                        ],
                        cwd: Some(VariableName::WorktreeRoot.template_value()),
                        ..TaskTemplate::default()
                    },
                    // Run test(s) for a specific target within a file
                    TaskTemplate {
                        label: "unittest $ZED_CUSTOM_PYTHON_TEST_TARGET".to_owned(),
                        command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                        args: vec![
                            "-m".to_owned(),
                            "unittest".to_owned(),
                            PYTHON_TEST_TARGET_TASK_VARIABLE.template_value_with_whitespace(),
                        ],
                        tags: vec![
                            "python-unittest-class".to_owned(),
                            "python-unittest-method".to_owned(),
                        ],
                        cwd: Some(VariableName::WorktreeRoot.template_value()),
                        ..TaskTemplate::default()
                    },
                ]
            }
            TestRunner::PYTEST => {
                [
                    // Run tests for an entire file
                    TaskTemplate {
                        label: format!("pytest '{}'", VariableName::File.template_value()),
                        command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                        args: vec![
                            "-m".to_owned(),
                            "pytest".to_owned(),
                            VariableName::File.template_value_with_whitespace(),
                        ],
                        cwd: Some(VariableName::WorktreeRoot.template_value()),
                        ..TaskTemplate::default()
                    },
                    // Run test(s) for a specific target within a file
                    TaskTemplate {
                        label: "pytest $ZED_CUSTOM_PYTHON_TEST_TARGET".to_owned(),
                        command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                        args: vec![
                            "-m".to_owned(),
                            "pytest".to_owned(),
                            PYTHON_TEST_TARGET_TASK_VARIABLE.template_value_with_whitespace(),
                        ],
                        cwd: Some(VariableName::WorktreeRoot.template_value()),
                        tags: vec![
                            "python-pytest-class".to_owned(),
                            "python-pytest-method".to_owned(),
                        ],
                        ..TaskTemplate::default()
                    },
                ]
            }
        });

        Task::ready(Some(TaskTemplates(tasks)))
    }
}

fn selected_test_runner(location: Option<&Arc<dyn language::File>>, cx: &App) -> TestRunner {
    const TEST_RUNNER_VARIABLE: &str = "TEST_RUNNER";
    language_settings(Some(LanguageName::new("Python")), location, cx)
        .tasks
        .variables
        .get(TEST_RUNNER_VARIABLE)
        .and_then(|val| TestRunner::from_str(val).ok())
        .unwrap_or(TestRunner::PYTEST)
}

impl PythonContextProvider {
    fn build_unittest_target(
        &self,
        variables: &task::TaskVariables,
    ) -> Option<(VariableName, String)> {
        let python_module_name =
            python_module_name_from_relative_path(variables.get(&VariableName::RelativeFile)?);

        let unittest_class_name =
            variables.get(&VariableName::Custom(Cow::Borrowed("_unittest_class_name")));

        let unittest_method_name = variables.get(&VariableName::Custom(Cow::Borrowed(
            "_unittest_method_name",
        )));

        let unittest_target_str = match (unittest_class_name, unittest_method_name) {
            (Some(class_name), Some(method_name)) => {
                format!("{python_module_name}.{class_name}.{method_name}")
            }
            (Some(class_name), None) => format!("{python_module_name}.{class_name}"),
            (None, None) => python_module_name,
            // should never happen, a TestCase class is the unit of testing
            (None, Some(_)) => return None,
        };

        Some((
            PYTHON_TEST_TARGET_TASK_VARIABLE.clone(),
            unittest_target_str,
        ))
    }

    fn build_pytest_target(
        &self,
        variables: &task::TaskVariables,
    ) -> Option<(VariableName, String)> {
        let file_path = variables.get(&VariableName::RelativeFile)?;

        let pytest_class_name =
            variables.get(&VariableName::Custom(Cow::Borrowed("_pytest_class_name")));

        let pytest_method_name =
            variables.get(&VariableName::Custom(Cow::Borrowed("_pytest_method_name")));

        let pytest_target_str = match (pytest_class_name, pytest_method_name) {
            (Some(class_name), Some(method_name)) => {
                format!("{file_path}::{class_name}::{method_name}")
            }
            (Some(class_name), None) => {
                format!("{file_path}::{class_name}")
            }
            (None, Some(method_name)) => {
                format!("{file_path}::{method_name}")
            }
            (None, None) => file_path.to_string(),
        };

        Some((PYTHON_TEST_TARGET_TASK_VARIABLE.clone(), pytest_target_str))
    }

    fn build_module_target(
        &self,
        variables: &task::TaskVariables,
    ) -> Result<(VariableName, String)> {
        let python_module_name = python_module_name_from_relative_path(
            variables.get(&VariableName::RelativeFile).unwrap_or(""),
        );

        let module_target = (PYTHON_MODULE_NAME_TASK_VARIABLE.clone(), python_module_name);

        Ok(module_target)
    }
}

fn python_module_name_from_relative_path(relative_path: &str) -> String {
    let path_with_dots = relative_path.replace('/', ".");
    path_with_dots
        .strip_suffix(".py")
        .unwrap_or(&path_with_dots)
        .to_string()
}

fn is_python_env_global(k: &PythonEnvironmentKind) -> bool {
    matches!(
        k,
        PythonEnvironmentKind::Homebrew
            | PythonEnvironmentKind::Pyenv
            | PythonEnvironmentKind::GlobalPaths
            | PythonEnvironmentKind::MacPythonOrg
            | PythonEnvironmentKind::MacCommandLineTools
            | PythonEnvironmentKind::LinuxGlobal
            | PythonEnvironmentKind::MacXCode
            | PythonEnvironmentKind::WindowsStore
            | PythonEnvironmentKind::WindowsRegistry
    )
}

fn python_env_kind_display(k: &PythonEnvironmentKind) -> &'static str {
    match k {
        PythonEnvironmentKind::Conda => "Conda",
        PythonEnvironmentKind::Pixi => "pixi",
        PythonEnvironmentKind::Homebrew => "Homebrew",
        PythonEnvironmentKind::Pyenv => "global (Pyenv)",
        PythonEnvironmentKind::GlobalPaths => "global",
        PythonEnvironmentKind::PyenvVirtualEnv => "Pyenv",
        PythonEnvironmentKind::Pipenv => "Pipenv",
        PythonEnvironmentKind::Poetry => "Poetry",
        PythonEnvironmentKind::MacPythonOrg => "global (Python.org)",
        PythonEnvironmentKind::MacCommandLineTools => "global (Command Line Tools for Xcode)",
        PythonEnvironmentKind::LinuxGlobal => "global",
        PythonEnvironmentKind::MacXCode => "global (Xcode)",
        PythonEnvironmentKind::Venv => "venv",
        PythonEnvironmentKind::VirtualEnv => "virtualenv",
        PythonEnvironmentKind::VirtualEnvWrapper => "virtualenvwrapper",
        PythonEnvironmentKind::WindowsStore => "global (Windows Store)",
        PythonEnvironmentKind::WindowsRegistry => "global (Windows Registry)",
    }
}

pub(crate) struct PythonToolchainProvider;

static ENV_PRIORITY_LIST: &[PythonEnvironmentKind] = &[
    // Prioritize non-Conda environments.
    PythonEnvironmentKind::Poetry,
    PythonEnvironmentKind::Pipenv,
    PythonEnvironmentKind::VirtualEnvWrapper,
    PythonEnvironmentKind::Venv,
    PythonEnvironmentKind::VirtualEnv,
    PythonEnvironmentKind::PyenvVirtualEnv,
    PythonEnvironmentKind::Pixi,
    PythonEnvironmentKind::Conda,
    PythonEnvironmentKind::Pyenv,
    PythonEnvironmentKind::GlobalPaths,
    PythonEnvironmentKind::Homebrew,
];

fn env_priority(kind: Option<PythonEnvironmentKind>) -> usize {
    if let Some(kind) = kind {
        ENV_PRIORITY_LIST
            .iter()
            .position(|blessed_env| blessed_env == &kind)
            .unwrap_or(ENV_PRIORITY_LIST.len())
    } else {
        // Unknown toolchains are less useful than non-blessed ones.
        ENV_PRIORITY_LIST.len() + 1
    }
}

/// Return the name of environment declared in <worktree-root/.venv.
///
/// https://virtualfish.readthedocs.io/en/latest/plugins.html#auto-activation-auto-activation
async fn get_worktree_venv_declaration(worktree_root: &Path) -> Option<String> {
    let file = async_fs::File::open(worktree_root.join(".venv"))
        .await
        .ok()?;
    let mut venv_name = String::new();
    smol::io::BufReader::new(file)
        .read_line(&mut venv_name)
        .await
        .ok()?;
    Some(venv_name.trim().to_string())
}

fn get_venv_parent_dir(env: &PythonEnvironment) -> Option<PathBuf> {
    // If global, we aren't a virtual environment
    if let Some(kind) = env.kind
        && is_python_env_global(&kind)
    {
        return None;
    }

    // Check to be sure we are a virtual environment using pet's most generic
    // virtual environment type, VirtualEnv
    let venv = env
        .executable
        .as_ref()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .filter(|p| is_virtualenv_dir(p))?;

    venv.parent().map(|parent| parent.to_path_buf())
}

#[async_trait]
impl ToolchainLister for PythonToolchainProvider {
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<Path>,
        project_env: Option<HashMap<String, String>>,
    ) -> ToolchainList {
        let env = project_env.unwrap_or_default();
        let environment = EnvironmentApi::from_env(&env);
        let locators = pet::locators::create_locators(
            Arc::new(pet_conda::Conda::from(&environment)),
            Arc::new(pet_poetry::Poetry::from(&environment)),
            &environment,
        );
        let mut config = Configuration::default();

        debug_assert!(subroot_relative_path.is_relative());
        // `.ancestors()` will yield at least one path, so in case of empty `subroot_relative_path`, we'll just use
        // worktree root as the workspace directory.
        config.workspace_directories = Some(
            subroot_relative_path
                .ancestors()
                .map(|ancestor| worktree_root.join(ancestor))
                .collect(),
        );
        for locator in locators.iter() {
            locator.configure(&config);
        }

        let reporter = pet_reporter::collect::create_reporter();
        pet::find::find_and_report_envs(&reporter, config, &locators, &environment, None);

        let mut toolchains = reporter
            .environments
            .lock()
            .map_or(Vec::new(), |mut guard| std::mem::take(&mut guard));

        let wr = worktree_root;
        let wr_venv = get_worktree_venv_declaration(&wr).await;
        // Sort detected environments by:
        //     environment name matching activation file (<workdir>/.venv)
        //     environment project dir matching worktree_root
        //     general env priority
        //     environment path matching the CONDA_PREFIX env var
        //     executable path
        toolchains.sort_by(|lhs, rhs| {
            // Compare venv names against worktree .venv file
            let venv_ordering =
                wr_venv
                    .as_ref()
                    .map_or(Ordering::Equal, |venv| match (&lhs.name, &rhs.name) {
                        (Some(l), Some(r)) => (r == venv).cmp(&(l == venv)),
                        (Some(l), None) if l == venv => Ordering::Less,
                        (None, Some(r)) if r == venv => Ordering::Greater,
                        _ => Ordering::Equal,
                    });

            // Compare project paths against worktree root
            let proj_ordering = || {
                let lhs_project = lhs.project.clone().or_else(|| get_venv_parent_dir(lhs));
                let rhs_project = rhs.project.clone().or_else(|| get_venv_parent_dir(rhs));
                match (&lhs_project, &rhs_project) {
                    (Some(l), Some(r)) => (r == &wr).cmp(&(l == &wr)),
                    (Some(l), None) if l == &wr => Ordering::Less,
                    (None, Some(r)) if r == &wr => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            };

            // Compare environment priorities
            let priority_ordering = || env_priority(lhs.kind).cmp(&env_priority(rhs.kind));

            // Compare conda prefixes
            let conda_ordering = || {
                if lhs.kind == Some(PythonEnvironmentKind::Conda) {
                    environment
                        .get_env_var("CONDA_PREFIX".to_string())
                        .map(|conda_prefix| {
                            let is_match = |exe: &Option<PathBuf>| {
                                exe.as_ref().is_some_and(|e| e.starts_with(&conda_prefix))
                            };
                            match (is_match(&lhs.executable), is_match(&rhs.executable)) {
                                (true, false) => Ordering::Less,
                                (false, true) => Ordering::Greater,
                                _ => Ordering::Equal,
                            }
                        })
                        .unwrap_or(Ordering::Equal)
                } else {
                    Ordering::Equal
                }
            };

            // Compare Python executables
            let exe_ordering = || lhs.executable.cmp(&rhs.executable);

            venv_ordering
                .then_with(proj_ordering)
                .then_with(priority_ordering)
                .then_with(conda_ordering)
                .then_with(exe_ordering)
        });

        let mut toolchains: Vec<_> = toolchains
            .into_iter()
            .filter_map(venv_to_toolchain)
            .collect();
        toolchains.dedup();
        ToolchainList {
            toolchains,
            default: None,
            groups: Default::default(),
        }
    }
    fn meta(&self) -> ToolchainMetadata {
        ToolchainMetadata {
            term: SharedString::new_static("Virtual Environment"),
            new_toolchain_placeholder: SharedString::new_static(
                "A path to the python3 executable within a virtual environment, or path to virtual environment itself",
            ),
            manifest_name: ManifestName::from(SharedString::new_static("pyproject.toml")),
        }
    }

    async fn resolve(
        &self,
        path: PathBuf,
        env: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Toolchain> {
        let env = env.unwrap_or_default();
        let environment = EnvironmentApi::from_env(&env);
        let locators = pet::locators::create_locators(
            Arc::new(pet_conda::Conda::from(&environment)),
            Arc::new(pet_poetry::Poetry::from(&environment)),
            &environment,
        );
        let toolchain = pet::resolve::resolve_environment(&path, &locators, &environment)
            .context("Could not find a virtual environment in provided path")?;
        let venv = toolchain.resolved.unwrap_or(toolchain.discovered);
        venv_to_toolchain(venv).context("Could not convert a venv into a toolchain")
    }

    async fn activation_script(
        &self,
        toolchain: &Toolchain,
        shell: ShellKind,
        fs: &dyn Fs,
    ) -> Vec<String> {
        let Ok(toolchain) = serde_json::from_value::<pet_core::python_environment::PythonEnvironment>(
            toolchain.as_json.clone(),
        ) else {
            return vec![];
        };
        let mut activation_script = vec![];

        match toolchain.kind {
            Some(PythonEnvironmentKind::Conda) => {
                if let Some(name) = &toolchain.name {
                    activation_script.push(format!("conda activate {name}"));
                } else {
                    activation_script.push("conda activate".to_string());
                }
            }
            Some(PythonEnvironmentKind::Venv | PythonEnvironmentKind::VirtualEnv) => {
                if let Some(prefix) = &toolchain.prefix {
                    let activate_keyword = match shell {
                        ShellKind::Cmd => ".",
                        ShellKind::Nushell => "overlay use",
                        ShellKind::PowerShell => ".",
                        ShellKind::Fish => "source",
                        ShellKind::Csh => "source",
                        ShellKind::Posix => "source",
                    };
                    let activate_script_name = match shell {
                        ShellKind::Posix => "activate",
                        ShellKind::Csh => "activate.csh",
                        ShellKind::Fish => "activate.fish",
                        ShellKind::Nushell => "activate.nu",
                        ShellKind::PowerShell => "activate.ps1",
                        ShellKind::Cmd => "activate.bat",
                    };
                    let path = prefix.join(BINARY_DIR).join(activate_script_name);

                    if let Ok(quoted) =
                        shlex::try_quote(&path.to_string_lossy()).map(Cow::into_owned)
                        && fs.is_file(&path).await
                    {
                        activation_script.push(format!("{activate_keyword} {quoted}"));
                    }
                }
            }
            Some(PythonEnvironmentKind::Pyenv) => {
                let Some(manager) = toolchain.manager else {
                    return vec![];
                };
                let version = toolchain.version.as_deref().unwrap_or("system");
                let pyenv = manager.executable;
                let pyenv = pyenv.display();
                activation_script.extend(match shell {
                    ShellKind::Fish => Some(format!("\"{pyenv}\" shell - fish {version}")),
                    ShellKind::Posix => Some(format!("\"{pyenv}\" shell - sh {version}")),
                    ShellKind::Nushell => Some(format!("\"{pyenv}\" shell - nu {version}")),
                    ShellKind::PowerShell => None,
                    ShellKind::Csh => None,
                    ShellKind::Cmd => None,
                })
            }
            _ => {}
        }
        activation_script
    }
}

fn venv_to_toolchain(venv: PythonEnvironment) -> Option<Toolchain> {
    let mut name = String::from("Python");
    if let Some(ref version) = venv.version {
        _ = write!(name, " {version}");
    }

    let name_and_kind = match (&venv.name, &venv.kind) {
        (Some(name), Some(kind)) => Some(format!("({name}; {})", python_env_kind_display(kind))),
        (Some(name), None) => Some(format!("({name})")),
        (None, Some(kind)) => Some(format!("({})", python_env_kind_display(kind))),
        (None, None) => None,
    };

    if let Some(nk) = name_and_kind {
        _ = write!(name, " {nk}");
    }

    Some(Toolchain {
        name: name.into(),
        path: venv.executable.as_ref()?.to_str()?.to_owned().into(),
        language_name: LanguageName::new("Python"),
        as_json: serde_json::to_value(venv).ok()?,
    })
}

pub struct EnvironmentApi<'a> {
    global_search_locations: Arc<Mutex<Vec<PathBuf>>>,
    project_env: &'a HashMap<String, String>,
    pet_env: pet_core::os_environment::EnvironmentApi,
}

impl<'a> EnvironmentApi<'a> {
    pub fn from_env(project_env: &'a HashMap<String, String>) -> Self {
        let paths = project_env
            .get("PATH")
            .map(|p| std::env::split_paths(p).collect())
            .unwrap_or_default();

        EnvironmentApi {
            global_search_locations: Arc::new(Mutex::new(paths)),
            project_env,
            pet_env: pet_core::os_environment::EnvironmentApi::new(),
        }
    }

    fn user_home(&self) -> Option<PathBuf> {
        self.project_env
            .get("HOME")
            .or_else(|| self.project_env.get("USERPROFILE"))
            .map(|home| pet_fs::path::norm_case(PathBuf::from(home)))
            .or_else(|| self.pet_env.get_user_home())
    }
}

impl pet_core::os_environment::Environment for EnvironmentApi<'_> {
    fn get_user_home(&self) -> Option<PathBuf> {
        self.user_home()
    }

    fn get_root(&self) -> Option<PathBuf> {
        None
    }

    fn get_env_var(&self, key: String) -> Option<String> {
        self.project_env
            .get(&key)
            .cloned()
            .or_else(|| self.pet_env.get_env_var(key))
    }

    fn get_know_global_search_locations(&self) -> Vec<PathBuf> {
        if self.global_search_locations.lock().is_empty() {
            let mut paths =
                std::env::split_paths(&self.get_env_var("PATH".to_string()).unwrap_or_default())
                    .collect::<Vec<PathBuf>>();

            log::trace!("Env PATH: {:?}", paths);
            for p in self.pet_env.get_know_global_search_locations() {
                if !paths.contains(&p) {
                    paths.push(p);
                }
            }

            let mut paths = paths
                .into_iter()
                .filter(|p| p.exists())
                .collect::<Vec<PathBuf>>();

            self.global_search_locations.lock().append(&mut paths);
        }
        self.global_search_locations.lock().clone()
    }
}

pub(crate) struct PyLspAdapter {
    python_venv_base: OnceCell<Result<Arc<Path>, String>>,
}
impl PyLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("pylsp");
    pub(crate) fn new() -> Self {
        Self {
            python_venv_base: OnceCell::new(),
        }
    }
    async fn ensure_venv(delegate: &dyn LspAdapterDelegate) -> Result<Arc<Path>> {
        let python_path = Self::find_base_python(delegate)
            .await
            .context("Could not find Python installation for PyLSP")?;
        let work_dir = delegate
            .language_server_download_dir(&Self::SERVER_NAME)
            .await
            .context("Could not get working directory for PyLSP")?;
        let mut path = PathBuf::from(work_dir.as_ref());
        path.push("pylsp-venv");
        if !path.exists() {
            util::command::new_smol_command(python_path)
                .arg("-m")
                .arg("venv")
                .arg("pylsp-venv")
                .current_dir(work_dir)
                .spawn()?
                .output()
                .await?;
        }

        Ok(path.into())
    }
    // Find "baseline", user python version from which we'll create our own venv.
    async fn find_base_python(delegate: &dyn LspAdapterDelegate) -> Option<PathBuf> {
        for path in ["python3", "python"] {
            if let Some(path) = delegate.which(path.as_ref()).await {
                return Some(path);
            }
        }
        None
    }

    async fn base_venv(&self, delegate: &dyn LspAdapterDelegate) -> Result<Arc<Path>, String> {
        self.python_venv_base
            .get_or_init(move || async move {
                Self::ensure_venv(delegate)
                    .await
                    .map_err(|e| format!("{e}"))
            })
            .await
            .clone()
    }
}

const BINARY_DIR: &str = if cfg!(target_os = "windows") {
    "Scripts"
} else {
    "bin"
};

#[async_trait(?Send)]
impl LspAdapter for PyLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn process_completions(&self, _items: &mut [lsp::CompletionItem]) {}

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let label = &item.label;
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            lsp::CompletionItemKind::METHOD => grammar.highlight_id_for_name("function.method")?,
            lsp::CompletionItemKind::FUNCTION => grammar.highlight_id_for_name("function")?,
            lsp::CompletionItemKind::CLASS => grammar.highlight_id_for_name("type")?,
            lsp::CompletionItemKind::CONSTANT => grammar.highlight_id_for_name("constant")?,
            _ => return None,
        };
        let filter_range = item
            .filter_text
            .as_deref()
            .and_then(|filter| label.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..label.len());
        Some(language::CodeLabel {
            text: label.clone(),
            runs: vec![(0..label.len(), highlight_id)],
            filter_range,
        })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("def {}():\n", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("class {}:", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("{} = 0", name);
                let filter_range = 0..name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(language::CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        adapter: &Arc<dyn LspAdapterDelegate>,
        toolchain: Option<Toolchain>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        cx.update(move |cx| {
            let mut user_settings =
                language_server_settings(adapter.as_ref(), &Self::SERVER_NAME, cx)
                    .and_then(|s| s.settings.clone())
                    .unwrap_or_else(|| {
                        json!({
                            "plugins": {
                                "pycodestyle": {"enabled": false},
                                "rope_autoimport": {"enabled": true, "memory": true},
                                "pylsp_mypy": {"enabled": false}
                            },
                            "rope": {
                                "ropeFolder": null
                            },
                        })
                    });

            // If user did not explicitly modify their python venv, use one from picker.
            if let Some(toolchain) = toolchain {
                if !user_settings.is_object() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();
                if let Some(python) = object
                    .entry("plugins")
                    .or_insert(Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                {
                    if let Some(jedi) = python
                        .entry("jedi")
                        .or_insert(Value::Object(serde_json::Map::default()))
                        .as_object_mut()
                    {
                        jedi.entry("environment".to_string())
                            .or_insert_with(|| Value::String(toolchain.path.clone().into()));
                    }
                    if let Some(pylint) = python
                        .entry("pylsp_mypy")
                        .or_insert(Value::Object(serde_json::Map::default()))
                        .as_object_mut()
                    {
                        pylint.entry("overrides".to_string()).or_insert_with(|| {
                            Value::Array(vec![
                                Value::String("--python-executable".into()),
                                Value::String(toolchain.path.into()),
                                Value::String("--cache-dir=/dev/null".into()),
                                Value::Bool(true),
                            ])
                        });
                    }
                }
            }
            user_settings = Value::Object(serde_json::Map::from_iter([(
                "pylsp".to_string(),
                user_settings,
            )]));

            user_settings
        })
    }
}

impl LspInstaller for PyLspAdapter {
    type BinaryVersion = ();
    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        toolchain: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        if let Some(pylsp_bin) = delegate.which(Self::SERVER_NAME.as_ref()).await {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: pylsp_bin,
                env: Some(env),
                arguments: vec![],
            })
        } else {
            let toolchain = toolchain?;
            let pylsp_path = Path::new(toolchain.path.as_ref()).parent()?.join("pylsp");
            pylsp_path.exists().then(|| LanguageServerBinary {
                path: toolchain.path.to_string().into(),
                arguments: vec![pylsp_path.into()],
                env: None,
            })
        }
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<()> {
        Ok(())
    }

    async fn fetch_server_binary(
        &self,
        _: (),
        _: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.map_err(|e| anyhow!(e))?;
        let pip_path = venv.join(BINARY_DIR).join("pip3");
        ensure!(
            util::command::new_smol_command(pip_path.as_path())
                .arg("install")
                .arg("python-lsp-server[all]")
                .arg("--upgrade")
                .output()
                .await?
                .status
                .success(),
            "python-lsp-server[all] installation failed"
        );
        ensure!(
            util::command::new_smol_command(pip_path)
                .arg("install")
                .arg("pylsp-mypy")
                .arg("--upgrade")
                .output()
                .await?
                .status
                .success(),
            "pylsp-mypy installation failed"
        );
        let pylsp = venv.join(BINARY_DIR).join("pylsp");
        ensure!(
            delegate.which(pylsp.as_os_str()).await.is_some(),
            "pylsp installation was incomplete"
        );
        Ok(LanguageServerBinary {
            path: pylsp,
            env: None,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.ok()?;
        let pylsp = venv.join(BINARY_DIR).join("pylsp");
        delegate.which(pylsp.as_os_str()).await?;
        Some(LanguageServerBinary {
            path: pylsp,
            env: None,
            arguments: vec![],
        })
    }
}

pub(crate) struct BasedPyrightLspAdapter {
    node: NodeRuntime,
}

impl BasedPyrightLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("basedpyright");
    const BINARY_NAME: &'static str = "basedpyright-langserver";
    const SERVER_PATH: &str = "node_modules/basedpyright/langserver.index.js";
    const NODE_MODULE_RELATIVE_SERVER_PATH: &str = "basedpyright/langserver.index.js";

    pub(crate) fn new(node: NodeRuntime) -> Self {
        BasedPyrightLspAdapter { node }
    }

    async fn get_cached_server_binary(
        container_dir: PathBuf,
        node: &NodeRuntime,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);
        if server_path.exists() {
            Some(LanguageServerBinary {
                path: node.binary_path().await.log_err()?,
                env: None,
                arguments: vec![server_path.into(), "--stdio".into()],
            })
        } else {
            log::error!("missing executable in directory {:?}", server_path);
            None
        }
    }
}

#[async_trait(?Send)]
impl LspAdapter for BasedPyrightLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        // Provide minimal initialization options
        // Virtual environment configuration will be handled through workspace configuration
        Ok(Some(json!({
            "python": {
                "analysis": {
                    "autoSearchPaths": true,
                    "useLibraryCodeForTypes": true,
                    "autoImportCompletions": true
                }
            }
        })))
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        process_pyright_completions(items);
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let label = &item.label;
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            lsp::CompletionItemKind::METHOD => grammar.highlight_id_for_name("function.method"),
            lsp::CompletionItemKind::FUNCTION => grammar.highlight_id_for_name("function"),
            lsp::CompletionItemKind::CLASS => grammar.highlight_id_for_name("type"),
            lsp::CompletionItemKind::CONSTANT => grammar.highlight_id_for_name("constant"),
            lsp::CompletionItemKind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => {
                return None;
            }
        };
        let filter_range = item
            .filter_text
            .as_deref()
            .and_then(|filter| label.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..label.len());
        let mut text = label.clone();
        if let Some(completion_details) = item
            .label_details
            .as_ref()
            .and_then(|details| details.description.as_ref())
        {
            write!(&mut text, " {}", completion_details).ok();
        }
        Some(language::CodeLabel {
            runs: highlight_id
                .map(|id| (0..label.len(), id))
                .into_iter()
                .collect(),
            text,
            filter_range,
        })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("def {}():\n", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CLASS => {
                let text = format!("class {}:", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("{} = 0", name);
                let filter_range = 0..name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(language::CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        adapter: &Arc<dyn LspAdapterDelegate>,
        toolchain: Option<Toolchain>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        cx.update(move |cx| {
            let mut user_settings =
                language_server_settings(adapter.as_ref(), &Self::SERVER_NAME, cx)
                    .and_then(|s| s.settings.clone())
                    .unwrap_or_default();

            // If we have a detected toolchain, configure Pyright to use it
            if let Some(toolchain) = toolchain
                && let Ok(env) = serde_json::from_value::<
                    pet_core::python_environment::PythonEnvironment,
                >(toolchain.as_json.clone())
            {
                if !user_settings.is_object() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();

                let interpreter_path = toolchain.path.to_string();
                if let Some(venv_dir) = env.prefix {
                    // Set venvPath and venv at the root level
                    // This matches the format of a pyrightconfig.json file
                    if let Some(parent) = venv_dir.parent() {
                        // Use relative path if the venv is inside the workspace
                        let venv_path = if parent == adapter.worktree_root_path() {
                            ".".to_string()
                        } else {
                            parent.to_string_lossy().into_owned()
                        };
                        object.insert("venvPath".to_string(), Value::String(venv_path));
                    }

                    if let Some(venv_name) = venv_dir.file_name() {
                        object.insert(
                            "venv".to_owned(),
                            Value::String(venv_name.to_string_lossy().into_owned()),
                        );
                    }
                }

                // Set both pythonPath and defaultInterpreterPath for compatibility
                if let Some(python) = object
                    .entry("python")
                    .or_insert(Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                {
                    python.insert(
                        "pythonPath".to_owned(),
                        Value::String(interpreter_path.clone()),
                    );
                    python.insert(
                        "defaultInterpreterPath".to_owned(),
                        Value::String(interpreter_path),
                    );
                }
                // Basedpyright by default uses `strict` type checking, we tone it down as to not surpris users
                maybe!({
                    let basedpyright = object
                        .entry("basedpyright")
                        .or_insert(Value::Object(serde_json::Map::default()));
                    let analysis = basedpyright
                        .as_object_mut()?
                        .entry("analysis")
                        .or_insert(Value::Object(serde_json::Map::default()));
                    if let serde_json::map::Entry::Vacant(v) =
                        analysis.as_object_mut()?.entry("typeCheckingMode")
                    {
                        v.insert(Value::String("standard".to_owned()));
                    }
                    Some(())
                });
            }

            user_settings
        })
    }
}

impl LspInstaller for BasedPyrightLspAdapter {
    type BinaryVersion = String;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<String> {
        self.node
            .npm_package_latest_version(Self::SERVER_NAME.as_ref())
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        if let Some(path) = delegate.which(Self::BINARY_NAME.as_ref()).await {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path,
                env: Some(env),
                arguments: vec!["--stdio".into()],
            })
        } else {
            // TODO shouldn't this be self.node.binary_path()?
            let node = delegate.which("node".as_ref()).await?;
            let (node_modules_path, _) = delegate
                .npm_package_installed_version(Self::SERVER_NAME.as_ref())
                .await
                .log_err()??;

            let path = node_modules_path.join(Self::NODE_MODULE_RELATIVE_SERVER_PATH);

            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: node,
                env: Some(env),
                arguments: vec![path.into(), "--stdio".into()],
            })
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);

        self.node
            .npm_install_packages(
                &container_dir,
                &[(Self::SERVER_NAME.as_ref(), latest_version.as_str())],
            )
            .await?;

        let env = delegate.shell_env().await;
        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: Some(env),
            arguments: vec![server_path.into(), "--stdio".into()],
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &Self::BinaryVersion,
        container_dir: &PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(Self::SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::SERVER_NAME.as_ref(),
                &server_path,
                container_dir,
                VersionStrategy::Latest(version),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: Some(env),
                arguments: vec![server_path.into(), "--stdio".into()],
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let mut binary = Self::get_cached_server_binary(container_dir, &self.node).await?;
        binary.env = Some(delegate.shell_env().await);
        Some(binary)
    }
}

pub(crate) struct RuffLspAdapter {
    fs: Arc<dyn Fs>,
}

#[cfg(target_os = "macos")]
impl RuffLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    const ARCH_SERVER_NAME: &str = "apple-darwin";
}

#[cfg(target_os = "linux")]
impl RuffLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    const ARCH_SERVER_NAME: &str = "unknown-linux-gnu";
}

#[cfg(target_os = "freebsd")]
impl RuffLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    const ARCH_SERVER_NAME: &str = "unknown-freebsd";
}

#[cfg(target_os = "windows")]
impl RuffLspAdapter {
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;
    const ARCH_SERVER_NAME: &str = "pc-windows-msvc";
}

impl RuffLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("ruff");

    pub fn new(fs: Arc<dyn Fs>) -> RuffLspAdapter {
        RuffLspAdapter { fs }
    }

    fn build_asset_name() -> Result<(String, String)> {
        let arch = match consts::ARCH {
            "x86" => "i686",
            _ => consts::ARCH,
        };
        let os = Self::ARCH_SERVER_NAME;
        let suffix = match consts::OS {
            "windows" => "zip",
            _ => "tar.gz",
        };
        let asset_name = format!("ruff-{arch}-{os}.{suffix}");
        let asset_stem = format!("ruff-{arch}-{os}");
        Ok((asset_stem, asset_name))
    }
}

#[async_trait(?Send)]
impl LspAdapter for RuffLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

impl LspInstaller for RuffLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;
    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        toolchain: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let ruff_in_venv = if let Some(toolchain) = toolchain
            && toolchain.language_name.as_ref() == "Python"
        {
            Path::new(toolchain.path.as_str())
                .parent()
                .map(|path| path.join("ruff"))
        } else {
            None
        };

        for path in ruff_in_venv.into_iter().chain(["ruff".into()]) {
            if let Some(ruff_bin) = delegate.which(path.as_os_str()).await {
                let env = delegate.shell_env().await;
                return Some(LanguageServerBinary {
                    path: ruff_bin,
                    env: Some(env),
                    arguments: vec!["server".into()],
                });
            }
        }

        None
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release =
            latest_github_release("astral-sh/ruff", true, false, delegate.http_client()).await?;
        let (_, asset_name) = Self::build_asset_name()?;
        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url,
            digest: asset.digest,
        })
    }

    async fn fetch_server_binary(
        &self,
        latest_version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let GitHubLspBinaryVersion {
            name,
            url,
            digest: expected_digest,
        } = latest_version;
        let destination_path = container_dir.join(format!("ruff-{name}"));
        let server_path = match Self::GITHUB_ASSET_KIND {
            AssetKind::TarGz | AssetKind::Gz => destination_path
                .join(Self::build_asset_name()?.0)
                .join("ruff"),
            AssetKind::Zip => destination_path.clone().join("ruff.exe"),
        };

        let binary = LanguageServerBinary {
            path: server_path.clone(),
            env: None,
            arguments: vec!["server".into()],
        };

        let metadata_path = destination_path.with_extension("metadata");
        let metadata = GithubBinaryMetadata::read_from_file(&metadata_path)
            .await
            .ok();
        if let Some(metadata) = metadata {
            let validity_check = async || {
                delegate
                    .try_exec(LanguageServerBinary {
                        path: server_path.clone(),
                        arguments: vec!["--version".into()],
                        env: None,
                    })
                    .await
                    .inspect_err(|err| {
                        log::warn!("Unable to run {server_path:?} asset, redownloading: {err}",)
                    })
            };
            if let (Some(actual_digest), Some(expected_digest)) =
                (&metadata.digest, &expected_digest)
            {
                if actual_digest == expected_digest {
                    if validity_check().await.is_ok() {
                        return Ok(binary);
                    }
                } else {
                    log::info!(
                        "SHA-256 mismatch for {destination_path:?} asset, downloading new asset. Expected: {expected_digest}, Got: {actual_digest}"
                    );
                }
            } else if validity_check().await.is_ok() {
                return Ok(binary);
            }
        }

        download_server_binary(
            delegate,
            &url,
            expected_digest.as_deref(),
            &destination_path,
            Self::GITHUB_ASSET_KIND,
        )
        .await?;
        make_file_executable(&server_path).await?;
        remove_matching(&container_dir, |path| path != destination_path).await;
        GithubBinaryMetadata::write_to_file(
            &GithubBinaryMetadata {
                metadata_version: 1,
                digest: expected_digest,
            },
            &metadata_path,
        )
        .await?;

        Ok(LanguageServerBinary {
            path: server_path,
            env: None,
            arguments: vec!["server".into()],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let mut last = None;
            let mut entries = self.fs.read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let path = entry?;
                if path.extension().is_some_and(|ext| ext == "metadata") {
                    continue;
                }
                last = Some(path);
            }

            let path = last.context("no cached binary")?;
            let path = match Self::GITHUB_ASSET_KIND {
                AssetKind::TarGz | AssetKind::Gz => {
                    path.join(Self::build_asset_name()?.0).join("ruff")
                }
                AssetKind::Zip => path.join("ruff.exe"),
            };

            anyhow::Ok(LanguageServerBinary {
                path,
                env: None,
                arguments: vec!["server".into()],
            })
        })
        .await
        .log_err()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, Context, TestAppContext};
    use language::{AutoindentMode, Buffer};
    use settings::SettingsStore;
    use std::num::NonZeroU32;

    #[gpui::test]
    async fn test_python_autoindent(cx: &mut TestAppContext) {
        cx.executor().set_block_on_ticks(usize::MAX..=usize::MAX);
        let language = crate::language("python", tree_sitter_python::LANGUAGE.into());
        cx.update(|cx| {
            let test_settings = SettingsStore::test(cx);
            cx.set_global(test_settings);
            language::init(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.project.all_languages.defaults.tab_size = NonZeroU32::new(2);
                });
            });
        });

        cx.new(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);
            let append = |buffer: &mut Buffer, text: &str, cx: &mut Context<Buffer>| {
                let ix = buffer.len();
                buffer.edit([(ix..ix, text)], Some(AutoindentMode::EachLine), cx);
            };

            // indent after "def():"
            append(&mut buffer, "def a():\n", cx);
            assert_eq!(buffer.text(), "def a():\n  ");

            // preserve indent after blank line
            append(&mut buffer, "\n  ", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  ");

            // indent after "if"
            append(&mut buffer, "if a:\n  ", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    ");

            // preserve indent after statement
            append(&mut buffer, "b()\n", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n    ");

            // preserve indent after statement
            append(&mut buffer, "else", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n    else");

            // dedent "else""
            append(&mut buffer, ":", cx);
            assert_eq!(buffer.text(), "def a():\n  \n  if a:\n    b()\n  else:");

            // indent lines after else
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    "
            );

            // indent after an open paren. the closing paren is not indented
            // because there is another token before it on the same line.
            append(&mut buffer, "foo(\n1)", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n      1)"
            );

            // dedent the closing paren if it is shifted to the beginning of the line
            let argument_ix = buffer.text().find('1').unwrap();
            buffer.edit(
                [(argument_ix..argument_ix + 1, "")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )"
            );

            // preserve indent after the close paren
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n    "
            );

            // manually outdent the last line
            let end_whitespace_ix = buffer.len() - 4;
            buffer.edit(
                [(end_whitespace_ix..buffer.len(), "")],
                Some(AutoindentMode::EachLine),
                cx,
            );
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n"
            );

            // preserve the newly reduced indentation on the next newline
            append(&mut buffer, "\n", cx);
            assert_eq!(
                buffer.text(),
                "def a():\n  \n  if a:\n    b()\n  else:\n    foo(\n    )\n\n"
            );

            // reset to a for loop statement
            let statement = "for i in range(10):\n  print(i)\n";
            buffer.edit([(0..buffer.len(), statement)], None, cx);

            // insert single line comment after each line
            let eol_ixs = statement
                .char_indices()
                .filter_map(|(ix, c)| if c == '\n' { Some(ix) } else { None })
                .collect::<Vec<usize>>();
            let editions = eol_ixs
                .iter()
                .enumerate()
                .map(|(i, &eol_ix)| (eol_ix..eol_ix, format!(" # comment {}", i + 1)))
                .collect::<Vec<(std::ops::Range<usize>, String)>>();
            buffer.edit(editions, Some(AutoindentMode::EachLine), cx);
            assert_eq!(
                buffer.text(),
                "for i in range(10): # comment 1\n  print(i) # comment 2\n"
            );

            // reset to a simple if statement
            buffer.edit([(0..buffer.len(), "if a:\n  b(\n  )")], None, cx);

            // dedent "else" on the line after a closing paren
            append(&mut buffer, "\n  else:\n", cx);
            assert_eq!(buffer.text(), "if a:\n  b(\n  )\nelse:\n  ");

            buffer
        });
    }
}
