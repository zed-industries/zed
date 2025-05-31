use anyhow::{Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use collections::HashMap;
use gpui::{App, AppContext, AsyncApp, Task};
use http_client::github::{AssetKind, GitHubLspBinaryVersion, build_asset_url};
use language::{
    ContextLocation, ContextProvider, File, LanguageToolchainStore, LspAdapter, LspAdapterDelegate,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName};
use node_runtime::NodeRuntime;
use project::{Fs, lsp_store::language_server_settings};
use serde_json::{Value, json};
use smol::{fs, io::BufReader, lock::RwLock, stream::StreamExt};
use std::{
    any::Any,
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, TaskVariables, VariableName};
use util::archive::extract_zip;
use util::merge_json_value_into;
use util::{ResultExt, fs::remove_matching, maybe};

pub(crate) struct TypeScriptContextProvider {
    last_package_json: PackageJsonContents,
}

const TYPESCRIPT_RUNNER_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_RUNNER"));
const TYPESCRIPT_JEST_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_JEST"));
const TYPESCRIPT_MOCHA_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_MOCHA"));

const TYPESCRIPT_VITEST_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_VITEST"));
const TYPESCRIPT_JASMINE_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_JASMINE"));
const TYPESCRIPT_BUILD_SCRIPT_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_BUILD_SCRIPT"));
const TYPESCRIPT_TEST_SCRIPT_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_TEST_SCRIPT"));

#[derive(Clone, Default)]
struct PackageJsonContents(Arc<RwLock<HashMap<PathBuf, PackageJson>>>);

struct PackageJson {
    mtime: DateTime<Local>,
    data: PackageJsonData,
}

#[derive(Clone, Copy, Default)]
struct PackageJsonData {
    jest: bool,
    mocha: bool,
    vitest: bool,
    jasmine: bool,
    build_script: bool,
    test_script: bool,
    runner: Runner,
}

#[derive(Clone, Copy, Default)]
enum Runner {
    #[default]
    Npm,
    Npx,
    Pnpm,
}

impl PackageJsonData {
    fn new(package_json: HashMap<String, Value>) -> Self {
        let mut build_script = false;
        let mut test_script = false;
        if let Some(serde_json::Value::Object(scripts)) = package_json.get("scripts") {
            build_script |= scripts.contains_key("build");
            test_script |= scripts.contains_key("test");
        }

        let mut jest = false;
        let mut mocha = false;
        let mut vitest = false;
        let mut jasmine = false;
        if let Some(serde_json::Value::Object(dependencies)) = package_json.get("devDependencies") {
            jest |= dependencies.contains_key("jest");
            mocha |= dependencies.contains_key("mocha");
            vitest |= dependencies.contains_key("vitest");
            jasmine |= dependencies.contains_key("jasmine");
        }
        if let Some(serde_json::Value::Object(dev_dependencies)) = package_json.get("dependencies")
        {
            jest |= dev_dependencies.contains_key("jest");
            mocha |= dev_dependencies.contains_key("mocha");
            vitest |= dev_dependencies.contains_key("vitest");
            jasmine |= dev_dependencies.contains_key("jasmine");
        }

        let mut runner = Runner::Npm;
        if which::which("pnpm").is_ok() {
            runner = Runner::Pnpm;
        } else if which::which("npx").is_ok() {
            runner = Runner::Npx;
        }

        Self {
            jest,
            mocha,
            vitest,
            jasmine,
            build_script,
            test_script,
            runner,
        }
    }

    fn fill_variables(&self, variables: &mut TaskVariables) {
        let runner = match self.runner {
            Runner::Npm => "npm",
            Runner::Npx => "npx",
            Runner::Pnpm => "pnpm",
        };
        variables.insert(TYPESCRIPT_RUNNER_VARIABLE, runner.to_owned());

        if self.jest {
            variables.insert(TYPESCRIPT_JEST_TASK_VARIABLE, "jest".to_owned());
        }
        if self.mocha {
            variables.insert(TYPESCRIPT_MOCHA_TASK_VARIABLE, "mocha".to_owned());
        }
        if self.vitest {
            variables.insert(TYPESCRIPT_VITEST_TASK_VARIABLE, "vitest".to_owned());
        }
        if self.jasmine {
            variables.insert(TYPESCRIPT_JASMINE_TASK_VARIABLE, "jasmine".to_owned());
        }
        if self.build_script {
            variables.insert(TYPESCRIPT_BUILD_SCRIPT_TASK_VARIABLE, "build".to_owned());
        }
        if self.test_script {
            variables.insert(TYPESCRIPT_TEST_SCRIPT_TASK_VARIABLE, "test".to_owned());
        }
    }
}

impl TypeScriptContextProvider {
    pub fn new() -> Self {
        TypeScriptContextProvider {
            last_package_json: PackageJsonContents::default(),
        }
    }
}

impl ContextProvider for TypeScriptContextProvider {
    fn associated_tasks(&self, _: Option<Arc<dyn File>>, _: &App) -> Option<TaskTemplates> {
        let mut task_templates = TaskTemplates(Vec::new());

        // Jest tasks
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} file test",
                TYPESCRIPT_JEST_TASK_VARIABLE.template_value()
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_JEST_TASK_VARIABLE.template_value(),
                VariableName::RelativeFile.template_value(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} test {}",
                TYPESCRIPT_JEST_TASK_VARIABLE.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_JEST_TASK_VARIABLE.template_value(),
                "--testNamePattern".to_owned(),
                format!("\"{}\"", VariableName::Symbol.template_value()),
                VariableName::RelativeFile.template_value(),
            ],
            tags: vec![
                "ts-test".to_owned(),
                "js-test".to_owned(),
                "tsx-test".to_owned(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });

        // Vitest tasks
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} file test",
                TYPESCRIPT_VITEST_TASK_VARIABLE.template_value()
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_VITEST_TASK_VARIABLE.template_value(),
                "run".to_owned(),
                VariableName::RelativeFile.template_value(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} test {}",
                TYPESCRIPT_VITEST_TASK_VARIABLE.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_VITEST_TASK_VARIABLE.template_value(),
                "run".to_owned(),
                "--testNamePattern".to_owned(),
                format!("\"{}\"", VariableName::Symbol.template_value()),
                VariableName::RelativeFile.template_value(),
            ],
            tags: vec![
                "ts-test".to_owned(),
                "js-test".to_owned(),
                "tsx-test".to_owned(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });

        // Mocha tasks
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} file test",
                TYPESCRIPT_MOCHA_TASK_VARIABLE.template_value()
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_MOCHA_TASK_VARIABLE.template_value(),
                VariableName::RelativeFile.template_value(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} test {}",
                TYPESCRIPT_MOCHA_TASK_VARIABLE.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_MOCHA_TASK_VARIABLE.template_value(),
                "--grep".to_owned(),
                format!("\"{}\"", VariableName::Symbol.template_value()),
                VariableName::RelativeFile.template_value(),
            ],
            tags: vec![
                "ts-test".to_owned(),
                "js-test".to_owned(),
                "tsx-test".to_owned(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });

        // Jasmine tasks
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} file test",
                TYPESCRIPT_JASMINE_TASK_VARIABLE.template_value()
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_JASMINE_TASK_VARIABLE.template_value(),
                VariableName::RelativeFile.template_value(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });
        task_templates.0.push(TaskTemplate {
            label: format!(
                "{} test {}",
                TYPESCRIPT_JASMINE_TASK_VARIABLE.template_value(),
                VariableName::Symbol.template_value(),
            ),
            command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
            args: vec![
                TYPESCRIPT_JASMINE_TASK_VARIABLE.template_value(),
                format!("--filter={}", VariableName::Symbol.template_value()),
                VariableName::RelativeFile.template_value(),
            ],
            tags: vec![
                "ts-test".to_owned(),
                "js-test".to_owned(),
                "tsx-test".to_owned(),
            ],
            cwd: Some(VariableName::WorktreeRoot.template_value()),
            ..TaskTemplate::default()
        });

        for package_json_script in [
            TYPESCRIPT_TEST_SCRIPT_TASK_VARIABLE,
            TYPESCRIPT_BUILD_SCRIPT_TASK_VARIABLE,
        ] {
            task_templates.0.push(TaskTemplate {
                label: format!(
                    "package.json script {}",
                    package_json_script.template_value()
                ),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "--prefix".to_owned(),
                    VariableName::WorktreeRoot.template_value(),
                    "run".to_owned(),
                    package_json_script.template_value(),
                ],
                tags: vec!["package-script".into()],
                cwd: Some(VariableName::WorktreeRoot.template_value()),
                ..TaskTemplate::default()
            });
        }

        task_templates.0.push(TaskTemplate {
            label: format!(
                "execute selection {}",
                VariableName::SelectedText.template_value()
            ),
            command: "node".to_owned(),
            args: vec![
                "-e".to_owned(),
                format!("\"{}\"", VariableName::SelectedText.template_value()),
            ],
            ..TaskTemplate::default()
        });

        Some(task_templates)
    }

    fn build_context(
        &self,
        _variables: &task::TaskVariables,
        location: ContextLocation<'_>,
        _project_env: Option<HashMap<String, String>>,
        _toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut App,
    ) -> Task<Result<task::TaskVariables>> {
        let Some((fs, worktree_root)) = location.fs.zip(location.worktree_root) else {
            return Task::ready(Ok(task::TaskVariables::default()));
        };

        let package_json_contents = self.last_package_json.clone();
        cx.background_spawn(async move {
            let variables = package_json_variables(fs, worktree_root, package_json_contents)
                .await
                .context("package.json context retrieval")
                .log_err()
                .unwrap_or_else(task::TaskVariables::default);
            Ok(variables)
        })
    }
}

async fn package_json_variables(
    fs: Arc<dyn Fs>,
    worktree_root: PathBuf,
    package_json_contents: PackageJsonContents,
) -> anyhow::Result<task::TaskVariables> {
    let package_json_path = worktree_root.join("package.json");
    let metadata = fs
        .metadata(&package_json_path)
        .await
        .with_context(|| format!("getting metadata for {package_json_path:?}"))?
        .with_context(|| format!("missing FS metadata for {package_json_path:?}"))?;
    let mtime = DateTime::<Local>::from(metadata.mtime.timestamp_for_user());
    let existing_data = {
        let contents = package_json_contents.0.read().await;
        contents
            .get(&package_json_path)
            .filter(|package_json| package_json.mtime == mtime)
            .map(|package_json| package_json.data)
    };

    let mut variables = TaskVariables::default();
    if let Some(existing_data) = existing_data {
        existing_data.fill_variables(&mut variables);
    } else {
        let package_json_string = fs
            .load(&package_json_path)
            .await
            .with_context(|| format!("loading package.json from {package_json_path:?}"))?;
        let package_json: HashMap<String, serde_json::Value> =
            serde_json::from_str(&package_json_string)
                .with_context(|| format!("parsing package.json from {package_json_path:?}"))?;
        let new_data = PackageJsonData::new(package_json);
        new_data.fill_variables(&mut variables);
        {
            let mut contents = package_json_contents.0.write().await;
            contents.insert(
                package_json_path,
                PackageJson {
                    mtime,
                    data: new_data,
                },
            );
        }
    }

    Ok(variables)
}

fn typescript_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![
        "--max-old-space-size=8192".into(),
        server_path.into(),
        "--stdio".into(),
    ]
}

pub struct TypeScriptLspAdapter {
    node: NodeRuntime,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("typescript-language-server");
    const PACKAGE_NAME: &str = "typescript";
    pub fn new(node: NodeRuntime) -> Self {
        TypeScriptLspAdapter { node }
    }
    async fn tsdk_path(fs: &dyn Fs, adapter: &Arc<dyn LspAdapterDelegate>) -> Option<&'static str> {
        let is_yarn = adapter
            .read_text_file(PathBuf::from(".yarn/sdks/typescript/lib/typescript.js"))
            .await
            .is_ok();

        let tsdk_path = if is_yarn {
            ".yarn/sdks/typescript/lib"
        } else {
            "node_modules/typescript/lib"
        };

        if fs
            .is_dir(&adapter.worktree_root_path().join(tsdk_path))
            .await
        {
            Some(tsdk_path)
        } else {
            None
        }
    }
}

struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

#[async_trait(?Send)]
impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("typescript-language-server")
                .await?,
        }) as Box<_>)
    }

    async fn check_if_version_installed(
        &self,
        version: &(dyn 'static + Send + Any),
        container_dir: &PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let version = version.downcast_ref::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                &container_dir,
                version.typescript_version.as_str(),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: None,
                arguments: typescript_server_binary_arguments(&server_path),
            })
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<TypeScriptVersions>().unwrap();
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        self.node
            .npm_install_packages(
                &container_dir,
                &[
                    (
                        Self::PACKAGE_NAME,
                        latest_version.typescript_version.as_str(),
                    ),
                    (
                        "typescript-language-server",
                        latest_version.server_version.as_str(),
                    ),
                ],
            )
            .await?;

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: typescript_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_ts_server_binary(container_dir, &self.node).await
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            Kind::CLASS | Kind::INTERFACE | Kind::ENUM => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
            Kind::VARIABLE => grammar.highlight_id_for_name("variable"),
            _ => None,
        }?;

        let text = if let Some(description) = item
            .label_details
            .as_ref()
            .and_then(|label_details| label_details.description.as_ref())
        {
            format!("{} {}", item.label, description)
        } else if let Some(detail) = &item.detail {
            format!("{} {}", item.label, detail)
        } else {
            item.label.clone()
        };

        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    async fn initialization_options(
        self: Arc<Self>,
        fs: &dyn Fs,
        adapter: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let tsdk_path = Self::tsdk_path(fs, adapter).await;
        Ok(Some(json!({
            "provideFormatter": true,
            "hostInfo": "zed",
            "tsserver": {
                "path": tsdk_path,
            },
            "preferences": {
                "includeInlayParameterNameHints": "all",
                "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
                "includeInlayFunctionParameterTypeHints": true,
                "includeInlayVariableTypeHints": true,
                "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
                "includeInlayPropertyDeclarationTypeHints": true,
                "includeInlayFunctionLikeReturnTypeHints": true,
                "includeInlayEnumMemberValueHints": true,
            }
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let override_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;
        if let Some(options) = override_options {
            return Ok(options);
        }
        Ok(json!({
            "completions": {
              "completeFunctionCalls": true
            }
        }))
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("TypeScript".into(), "typescript".into()),
            ("JavaScript".into(), "javascript".into()),
            ("TSX".into(), "typescriptreact".into()),
        ])
    }
}

async fn get_cached_ts_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let old_server_path = container_dir.join(TypeScriptLspAdapter::OLD_SERVER_PATH);
        let new_server_path = container_dir.join(TypeScriptLspAdapter::NEW_SERVER_PATH);
        if new_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&new_server_path),
            })
        } else if old_server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: typescript_server_binary_arguments(&old_server_path),
            })
        } else {
            anyhow::bail!("missing executable in directory {container_dir:?}")
        }
    })
    .await
    .log_err()
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
        _: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let workspace_root = delegate.worktree_root_path();
        let use_flat_config = Self::FLAT_CONFIG_FILE_NAMES
            .iter()
            .any(|file| workspace_root.join(file).is_file());

        let mut default_workspace_configuration = json!({
            "validate": "on",
            "rulesCustomizations": [],
            "run": "onType",
            "nodePath": null,
            "workingDirectory": {
                "mode": "auto"
            },
            "workspaceFolder": {
                "uri": workspace_root,
                "name": workspace_root.file_name()
                    .unwrap_or(workspace_root.as_os_str())
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
            },
        });

        let override_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = override_options {
            merge_json_value_into(override_options, &mut default_workspace_configuration);
        }

        Ok(json!({
            "": default_workspace_configuration
        }))
    }

    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let url = build_asset_url(
            "zed-industries/vscode-eslint",
            Self::CURRENT_VERSION_TAG_NAME,
            Self::GITHUB_ASSET_KIND,
        )?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: Self::CURRENT_VERSION.into(),
            url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = Self::build_destination_path(&container_dir);
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |entry| entry != destination_path).await;

            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("downloading release")?;
            match Self::GITHUB_ASSET_KIND {
                AssetKind::TarGz => {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&destination_path).await.with_context(|| {
                        format!("extracting {} to {:?}", version.url, destination_path)
                    })?;
                }
                AssetKind::Gz => {
                    let mut decompressed_bytes =
                        GzipDecoder::new(BufReader::new(response.body_mut()));
                    let mut file =
                        fs::File::create(&destination_path).await.with_context(|| {
                            format!(
                                "creating a file {:?} for a download from {}",
                                destination_path, version.url,
                            )
                        })?;
                    futures::io::copy(&mut decompressed_bytes, &mut file)
                        .await
                        .with_context(|| {
                            format!("extracting {} to {:?}", version.url, destination_path)
                        })?;
                }
                AssetKind::Zip => {
                    extract_zip(&destination_path, response.body_mut())
                        .await
                        .with_context(|| {
                            format!("unzipping {} to {:?}", version.url, destination_path)
                        })?;
                }
            }

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
                .run_npm_subcommand(&repo_root, "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(&repo_root, "run-script", &["compile"])
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

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};
    use unindent::Unindent;

    #[gpui::test]
    async fn test_outline(cx: &mut TestAppContext) {
        let language = crate::language(
            "typescript",
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        );

        let text = r#"
            function a() {
              // local variables are omitted
              let a1 = 1;
              // all functions are included
              async function a2() {}
            }
            // top-level variables are included
            let b: C
            function getB() {}
            // exported variables are included
            export const d = e;
        "#
        .unindent();

        let buffer = cx.new(|cx| language::Buffer::local(text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None).unwrap());
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function a()", 0),
                ("async function a2()", 1),
                ("let b", 0),
                ("function getB()", 0),
                ("const d", 0),
            ]
        );
    }
}
