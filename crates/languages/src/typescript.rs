use anyhow::{Context as _, Result};
use async_trait::async_trait;
use chrono::{DateTime, Local};
use collections::HashMap;
use futures::future::join_all;
use gpui::{App, AppContext, AsyncApp, Task};
use http_client::github::{AssetKind, GitHubLspBinaryVersion, build_asset_url};
use http_client::github_download::download_server_binary;
use itertools::Itertools as _;
use language::{
    ContextLocation, ContextProvider, File, LanguageName, LanguageToolchainStore, LspAdapter,
    LspAdapterDelegate, LspInstaller, Toolchain,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::{Fs, lsp_store::language_server_settings};
use serde_json::{Value, json};
use smol::{fs, lock::RwLock, stream::StreamExt};
use std::{
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{ResultExt, fs::remove_matching, maybe};
use util::{merge_json_value_into, rel_path::RelPath};

use crate::{PackageJson, PackageJsonData};

pub(crate) struct TypeScriptContextProvider {
    fs: Arc<dyn Fs>,
    last_package_json: PackageJsonContents,
}

const TYPESCRIPT_RUNNER_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_RUNNER"));

const TYPESCRIPT_JEST_TEST_NAME_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_JEST_TEST_NAME"));

const TYPESCRIPT_VITEST_TEST_NAME_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_VITEST_TEST_NAME"));

const TYPESCRIPT_JEST_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_JEST_PACKAGE_PATH"));

const TYPESCRIPT_MOCHA_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_MOCHA_PACKAGE_PATH"));

const TYPESCRIPT_VITEST_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_VITEST_PACKAGE_PATH"));

const TYPESCRIPT_JASMINE_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_JASMINE_PACKAGE_PATH"));

const TYPESCRIPT_BUN_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_BUN_PACKAGE_PATH"));

const TYPESCRIPT_NODE_PACKAGE_PATH_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_NODE_PACKAGE_PATH"));

#[derive(Clone, Debug, Default)]
struct PackageJsonContents(Arc<RwLock<HashMap<PathBuf, PackageJson>>>);

impl PackageJsonData {
    fn fill_task_templates(&self, task_templates: &mut TaskTemplates) {
        if self.jest_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: "jest file test".to_owned(),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "jest".to_owned(),
                    "--runInBand".to_owned(),
                    VariableName::File.template_value(),
                ],
                cwd: Some(TYPESCRIPT_JEST_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!("jest test {}", VariableName::Symbol.template_value()),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "jest".to_owned(),
                    "--runInBand".to_owned(),
                    "--testNamePattern".to_owned(),
                    format!(
                        "\"{}\"",
                        TYPESCRIPT_JEST_TEST_NAME_VARIABLE.template_value()
                    ),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_JEST_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        if self.vitest_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: format!("{} file test", "vitest".to_owned()),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "vitest".to_owned(),
                    "run".to_owned(),
                    "--poolOptions.forks.minForks=0".to_owned(),
                    "--poolOptions.forks.maxForks=1".to_owned(),
                    VariableName::File.template_value(),
                ],
                cwd: Some(TYPESCRIPT_VITEST_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!(
                    "{} test {}",
                    "vitest".to_owned(),
                    VariableName::Symbol.template_value(),
                ),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "vitest".to_owned(),
                    "run".to_owned(),
                    "--poolOptions.forks.minForks=0".to_owned(),
                    "--poolOptions.forks.maxForks=1".to_owned(),
                    "--testNamePattern".to_owned(),
                    format!(
                        "\"{}\"",
                        TYPESCRIPT_VITEST_TEST_NAME_VARIABLE.template_value()
                    ),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_VITEST_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        if self.mocha_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: format!("{} file test", "mocha".to_owned()),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "mocha".to_owned(),
                    VariableName::File.template_value(),
                ],
                cwd: Some(TYPESCRIPT_MOCHA_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!(
                    "{} test {}",
                    "mocha".to_owned(),
                    VariableName::Symbol.template_value(),
                ),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "mocha".to_owned(),
                    "--grep".to_owned(),
                    format!("\"{}\"", VariableName::Symbol.template_value()),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_MOCHA_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        if self.jasmine_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: format!("{} file test", "jasmine".to_owned()),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "jasmine".to_owned(),
                    VariableName::File.template_value(),
                ],
                cwd: Some(TYPESCRIPT_JASMINE_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!(
                    "{} test {}",
                    "jasmine".to_owned(),
                    VariableName::Symbol.template_value(),
                ),
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec![
                    "exec".to_owned(),
                    "--".to_owned(),
                    "jasmine".to_owned(),
                    format!("--filter={}", VariableName::Symbol.template_value()),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_JASMINE_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        if self.bun_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: format!("{} file test", "bun test".to_owned()),
                command: "bun".to_owned(),
                args: vec!["test".to_owned(), VariableName::File.template_value()],
                cwd: Some(TYPESCRIPT_BUN_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!("bun test {}", VariableName::Symbol.template_value(),),
                command: "bun".to_owned(),
                args: vec![
                    "test".to_owned(),
                    "--test-name-pattern".to_owned(),
                    format!("\"{}\"", VariableName::Symbol.template_value()),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_BUN_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        if self.node_package_path.is_some() {
            task_templates.0.push(TaskTemplate {
                label: format!("{} file test", "node test".to_owned()),
                command: "node".to_owned(),
                args: vec!["--test".to_owned(), VariableName::File.template_value()],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_NODE_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
            task_templates.0.push(TaskTemplate {
                label: format!("node test {}", VariableName::Symbol.template_value()),
                command: "node".to_owned(),
                args: vec![
                    "--test".to_owned(),
                    "--test-name-pattern".to_owned(),
                    format!("\"{}\"", VariableName::Symbol.template_value()),
                    VariableName::File.template_value(),
                ],
                tags: vec![
                    "ts-test".to_owned(),
                    "js-test".to_owned(),
                    "tsx-test".to_owned(),
                ],
                cwd: Some(TYPESCRIPT_NODE_PACKAGE_PATH_VARIABLE.template_value()),
                ..TaskTemplate::default()
            });
        }

        let script_name_counts: HashMap<_, usize> =
            self.scripts
                .iter()
                .fold(HashMap::default(), |mut acc, (_, script)| {
                    *acc.entry(script).or_default() += 1;
                    acc
                });
        for (path, script) in &self.scripts {
            let label = if script_name_counts.get(script).copied().unwrap_or_default() > 1
                && let Some(parent) = path.parent().and_then(|parent| parent.file_name())
            {
                let parent = parent.to_string_lossy();
                format!("{parent}/package.json > {script}")
            } else {
                format!("package.json > {script}")
            };
            task_templates.0.push(TaskTemplate {
                label,
                command: TYPESCRIPT_RUNNER_VARIABLE.template_value(),
                args: vec!["run".to_owned(), script.to_owned()],
                tags: vec!["package-script".into()],
                cwd: Some(
                    path.parent()
                        .unwrap_or(Path::new("/"))
                        .to_string_lossy()
                        .to_string(),
                ),
                ..TaskTemplate::default()
            });
        }
    }
}

impl TypeScriptContextProvider {
    pub fn new(fs: Arc<dyn Fs>) -> Self {
        Self {
            fs,
            last_package_json: PackageJsonContents::default(),
        }
    }

    fn combined_package_json_data(
        &self,
        fs: Arc<dyn Fs>,
        worktree_root: &Path,
        file_relative_path: &RelPath,
        cx: &App,
    ) -> Task<anyhow::Result<PackageJsonData>> {
        let new_json_data = file_relative_path
            .ancestors()
            .map(|path| worktree_root.join(path.as_std_path()))
            .map(|parent_path| {
                self.package_json_data(&parent_path, self.last_package_json.clone(), fs.clone(), cx)
            })
            .collect::<Vec<_>>();

        cx.background_spawn(async move {
            let mut package_json_data = PackageJsonData::default();
            for new_data in join_all(new_json_data).await.into_iter().flatten() {
                package_json_data.merge(new_data);
            }
            Ok(package_json_data)
        })
    }

    fn package_json_data(
        &self,
        directory_path: &Path,
        existing_package_json: PackageJsonContents,
        fs: Arc<dyn Fs>,
        cx: &App,
    ) -> Task<anyhow::Result<PackageJsonData>> {
        let package_json_path = directory_path.join("package.json");
        let metadata_check_fs = fs.clone();
        cx.background_spawn(async move {
            let metadata = metadata_check_fs
                .metadata(&package_json_path)
                .await
                .with_context(|| format!("getting metadata for {package_json_path:?}"))?
                .with_context(|| format!("missing FS metadata for {package_json_path:?}"))?;
            let mtime = DateTime::<Local>::from(metadata.mtime.timestamp_for_user());
            let existing_data = {
                let contents = existing_package_json.0.read().await;
                contents
                    .get(&package_json_path)
                    .filter(|package_json| package_json.mtime == mtime)
                    .map(|package_json| package_json.data.clone())
            };
            match existing_data {
                Some(existing_data) => Ok(existing_data),
                None => {
                    let package_json_string =
                        fs.load(&package_json_path).await.with_context(|| {
                            format!("loading package.json from {package_json_path:?}")
                        })?;
                    let package_json: HashMap<String, serde_json_lenient::Value> =
                        serde_json_lenient::from_str(&package_json_string).with_context(|| {
                            format!("parsing package.json from {package_json_path:?}")
                        })?;
                    let new_data =
                        PackageJsonData::new(package_json_path.as_path().into(), package_json);
                    {
                        let mut contents = existing_package_json.0.write().await;
                        contents.insert(
                            package_json_path,
                            PackageJson {
                                mtime,
                                data: new_data.clone(),
                            },
                        );
                    }
                    Ok(new_data)
                }
            }
        })
    }
}

async fn detect_package_manager(
    worktree_root: PathBuf,
    fs: Arc<dyn Fs>,
    package_json_data: Option<PackageJsonData>,
) -> &'static str {
    if let Some(package_json_data) = package_json_data
        && let Some(package_manager) = package_json_data.package_manager
    {
        return package_manager;
    }
    if fs.is_file(&worktree_root.join("pnpm-lock.yaml")).await {
        return "pnpm";
    }
    if fs.is_file(&worktree_root.join("yarn.lock")).await {
        return "yarn";
    }
    "npm"
}

impl ContextProvider for TypeScriptContextProvider {
    fn associated_tasks(
        &self,
        file: Option<Arc<dyn File>>,
        cx: &App,
    ) -> Task<Option<TaskTemplates>> {
        let Some(file) = project::File::from_dyn(file.as_ref()).cloned() else {
            return Task::ready(None);
        };
        let Some(worktree_root) = file.worktree.read(cx).root_dir() else {
            return Task::ready(None);
        };
        let file_relative_path = file.path().clone();
        let package_json_data = self.combined_package_json_data(
            self.fs.clone(),
            &worktree_root,
            &file_relative_path,
            cx,
        );

        cx.background_spawn(async move {
            let mut task_templates = TaskTemplates(Vec::new());
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

            match package_json_data.await {
                Ok(package_json) => {
                    package_json.fill_task_templates(&mut task_templates);
                }
                Err(e) => {
                    log::error!(
                        "Failed to read package.json for worktree {file_relative_path:?}: {e:#}"
                    );
                }
            }

            Some(task_templates)
        })
    }

    fn build_context(
        &self,
        current_vars: &task::TaskVariables,
        location: ContextLocation<'_>,
        _project_env: Option<HashMap<String, String>>,
        _toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut App,
    ) -> Task<Result<task::TaskVariables>> {
        let mut vars = task::TaskVariables::default();

        if let Some(symbol) = current_vars.get(&VariableName::Symbol) {
            vars.insert(
                TYPESCRIPT_JEST_TEST_NAME_VARIABLE,
                replace_test_name_parameters(symbol),
            );
            vars.insert(
                TYPESCRIPT_VITEST_TEST_NAME_VARIABLE,
                replace_test_name_parameters(symbol),
            );
        }
        let file_path = location
            .file_location
            .buffer
            .read(cx)
            .file()
            .map(|file| file.path());

        let args = location.worktree_root.zip(location.fs).zip(file_path).map(
            |((worktree_root, fs), file_path)| {
                (
                    self.combined_package_json_data(fs.clone(), &worktree_root, file_path, cx),
                    worktree_root,
                    fs,
                )
            },
        );
        cx.background_spawn(async move {
            if let Some((task, worktree_root, fs)) = args {
                let package_json_data = task.await.log_err();
                vars.insert(
                    TYPESCRIPT_RUNNER_VARIABLE,
                    detect_package_manager(worktree_root, fs, package_json_data.clone())
                        .await
                        .to_owned(),
                );

                if let Some(package_json_data) = package_json_data {
                    if let Some(path) = package_json_data.jest_package_path {
                        vars.insert(
                            TYPESCRIPT_JEST_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }

                    if let Some(path) = package_json_data.mocha_package_path {
                        vars.insert(
                            TYPESCRIPT_MOCHA_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }

                    if let Some(path) = package_json_data.vitest_package_path {
                        vars.insert(
                            TYPESCRIPT_VITEST_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }

                    if let Some(path) = package_json_data.jasmine_package_path {
                        vars.insert(
                            TYPESCRIPT_JASMINE_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }

                    if let Some(path) = package_json_data.bun_package_path {
                        vars.insert(
                            TYPESCRIPT_BUN_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }

                    if let Some(path) = package_json_data.node_package_path {
                        vars.insert(
                            TYPESCRIPT_NODE_PACKAGE_PATH_VARIABLE,
                            path.parent()
                                .unwrap_or(Path::new(""))
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
            Ok(vars)
        })
    }
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

fn replace_test_name_parameters(test_name: &str) -> String {
    static PATTERN: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(\$([A-Za-z0-9_\.]+|[\#])|%[psdifjo#\$%])").unwrap());
    PATTERN.split(test_name).map(regex::escape).join("(.+?)")
}

pub struct TypeScriptLspAdapter {
    fs: Arc<dyn Fs>,
    node: NodeRuntime,
}

impl TypeScriptLspAdapter {
    const OLD_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
    const NEW_SERVER_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.mjs";
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("typescript-language-server");
    const PACKAGE_NAME: &str = "typescript";
    pub fn new(node: NodeRuntime, fs: Arc<dyn Fs>) -> Self {
        TypeScriptLspAdapter { fs, node }
    }
    async fn tsdk_path(&self, adapter: &Arc<dyn LspAdapterDelegate>) -> Option<&'static str> {
        let is_yarn = adapter
            .read_text_file(RelPath::unix(".yarn/sdks/typescript/lib/typescript.js").unwrap())
            .await
            .is_ok();

        let tsdk_path = if is_yarn {
            ".yarn/sdks/typescript/lib"
        } else {
            "node_modules/typescript/lib"
        };

        if self
            .fs
            .is_dir(&adapter.worktree_root_path().join(tsdk_path))
            .await
        {
            Some(tsdk_path)
        } else {
            None
        }
    }
}

pub struct TypeScriptVersions {
    typescript_version: String,
    server_version: String,
}

impl LspInstaller for TypeScriptLspAdapter {
    type BinaryVersion = TypeScriptVersions;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<TypeScriptVersions> {
        Ok(TypeScriptVersions {
            typescript_version: self.node.npm_package_latest_version("typescript").await?,
            server_version: self
                .node
                .npm_package_latest_version("typescript-language-server")
                .await?,
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &TypeScriptVersions,
        container_dir: &PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(Self::NEW_SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                container_dir,
                VersionStrategy::Latest(version.typescript_version.as_str()),
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
        latest_version: TypeScriptVersions,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
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
}

#[async_trait(?Send)]
impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
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
        Some(language::CodeLabel::filtered(
            text,
            item.filter_text.as_deref(),
            vec![(0..len, highlight_id)],
        ))
    }

    async fn initialization_options(
        self: Arc<Self>,
        adapter: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let tsdk_path = self.tsdk_path(adapter).await;
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

        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
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

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        HashMap::from_iter([
            (LanguageName::new("TypeScript"), "typescript".into()),
            (LanguageName::new("JavaScript"), "javascript".into()),
            (LanguageName::new("TSX"), "typescriptreact".into()),
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
            }
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
        Self::SERVER_NAME
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
    use std::path::Path;

    use gpui::{AppContext as _, BackgroundExecutor, TestAppContext};
    use language::language_settings;
    use project::{FakeFs, Project};
    use serde_json::json;
    use task::TaskTemplates;
    use unindent::Unindent;
    use util::{path, rel_path::rel_path};

    use crate::typescript::{
        PackageJsonData, TypeScriptContextProvider, replace_test_name_parameters,
    };

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
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None));
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

    #[gpui::test]
    async fn test_generator_function_outline(cx: &mut TestAppContext) {
        let language = crate::language("javascript", tree_sitter_typescript::LANGUAGE_TSX.into());

        let text = r#"
            function normalFunction() {
                console.log("normal");
            }

            function* simpleGenerator() {
                yield 1;
                yield 2;
            }

            async function* asyncGenerator() {
                yield await Promise.resolve(1);
            }

            function* generatorWithParams(start, end) {
                for (let i = start; i <= end; i++) {
                    yield i;
                }
            }

            class TestClass {
                *methodGenerator() {
                    yield "method";
                }

                async *asyncMethodGenerator() {
                    yield "async method";
                }
            }
        "#
        .unindent();

        let buffer = cx.new(|cx| language::Buffer::local(text, cx).with_language(language, cx));
        let outline = buffer.read_with(cx, |buffer, _| buffer.snapshot().outline(None));
        assert_eq!(
            outline
                .items
                .iter()
                .map(|item| (item.text.as_str(), item.depth))
                .collect::<Vec<_>>(),
            &[
                ("function normalFunction()", 0),
                ("function* simpleGenerator()", 0),
                ("async function* asyncGenerator()", 0),
                ("function* generatorWithParams( )", 0),
                ("class TestClass", 0),
                ("*methodGenerator()", 1),
                ("async *asyncMethodGenerator()", 1),
            ]
        );
    }

    #[gpui::test]
    async fn test_package_json_discovery(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::init(cx);
            Project::init_settings(cx);
            language_settings::init(cx);
        });

        let package_json_1 = json!({
            "dependencies": {
                "mocha": "1.0.0",
                "vitest": "1.0.0"
            },
            "scripts": {
                "test": ""
            }
        })
        .to_string();

        let package_json_2 = json!({
            "devDependencies": {
                "vitest": "2.0.0"
            },
            "scripts": {
                "test": ""
            }
        })
        .to_string();

        let fs = FakeFs::new(executor);
        fs.insert_tree(
            path!("/root"),
            json!({
                "package.json": package_json_1,
                "sub": {
                    "package.json": package_json_2,
                    "file.js": "",
                }
            }),
        )
        .await;

        let provider = TypeScriptContextProvider::new(fs.clone());
        let package_json_data = cx
            .update(|cx| {
                provider.combined_package_json_data(
                    fs.clone(),
                    path!("/root").as_ref(),
                    rel_path("sub/file1.js"),
                    cx,
                )
            })
            .await
            .unwrap();
        pretty_assertions::assert_eq!(
            package_json_data,
            PackageJsonData {
                jest_package_path: None,
                mocha_package_path: Some(Path::new(path!("/root/package.json")).into()),
                vitest_package_path: Some(Path::new(path!("/root/sub/package.json")).into()),
                jasmine_package_path: None,
                bun_package_path: None,
                node_package_path: None,
                scripts: [
                    (
                        Path::new(path!("/root/package.json")).into(),
                        "test".to_owned()
                    ),
                    (
                        Path::new(path!("/root/sub/package.json")).into(),
                        "test".to_owned()
                    )
                ]
                .into_iter()
                .collect(),
                package_manager: None,
            }
        );

        let mut task_templates = TaskTemplates::default();
        package_json_data.fill_task_templates(&mut task_templates);
        let task_templates = task_templates
            .0
            .into_iter()
            .map(|template| (template.label, template.cwd))
            .collect::<Vec<_>>();
        pretty_assertions::assert_eq!(
            task_templates,
            [
                (
                    "vitest file test".into(),
                    Some("$ZED_CUSTOM_TYPESCRIPT_VITEST_PACKAGE_PATH".into()),
                ),
                (
                    "vitest test $ZED_SYMBOL".into(),
                    Some("$ZED_CUSTOM_TYPESCRIPT_VITEST_PACKAGE_PATH".into()),
                ),
                (
                    "mocha file test".into(),
                    Some("$ZED_CUSTOM_TYPESCRIPT_MOCHA_PACKAGE_PATH".into()),
                ),
                (
                    "mocha test $ZED_SYMBOL".into(),
                    Some("$ZED_CUSTOM_TYPESCRIPT_MOCHA_PACKAGE_PATH".into()),
                ),
                (
                    "root/package.json > test".into(),
                    Some(path!("/root").into())
                ),
                (
                    "sub/package.json > test".into(),
                    Some(path!("/root/sub").into())
                ),
            ]
        );
    }

    #[test]
    fn test_escaping_name() {
        let cases = [
            ("plain test name", "plain test name"),
            ("test name with $param_name", "test name with (.+?)"),
            ("test name with $nested.param.name", "test name with (.+?)"),
            ("test name with $#", "test name with (.+?)"),
            ("test name with $##", "test name with (.+?)\\#"),
            ("test name with %p", "test name with (.+?)"),
            ("test name with %s", "test name with (.+?)"),
            ("test name with %d", "test name with (.+?)"),
            ("test name with %i", "test name with (.+?)"),
            ("test name with %f", "test name with (.+?)"),
            ("test name with %j", "test name with (.+?)"),
            ("test name with %o", "test name with (.+?)"),
            ("test name with %#", "test name with (.+?)"),
            ("test name with %$", "test name with (.+?)"),
            ("test name with %%", "test name with (.+?)"),
            ("test name with %q", "test name with %q"),
            (
                "test name with regex chars .*+?^${}()|[]\\",
                "test name with regex chars \\.\\*\\+\\?\\^\\$\\{\\}\\(\\)\\|\\[\\]\\\\",
            ),
            (
                "test name with multiple $params and %pretty and %b and (.+?)",
                "test name with multiple (.+?) and (.+?)retty and %b and \\(\\.\\+\\?\\)",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(replace_test_name_parameters(input), expected);
        }
    }

    // The order of test runner tasks is based on inferred user preference:
    // 1. Dedicated test runners (e.g., Jest, Vitest, Mocha, Jasmine) are prioritized.
    // 2. Bun's built-in test runner (`bun test`) comes next.
    // 3. Node.js's built-in test runner (`node --test`) is last.
    // This hierarchy assumes that if a dedicated test framework is installed, it is the
    // preferred testing mechanism. Between runtime-specific options, `bun test` is
    // typically preferred over `node --test` when @types/bun is present.
    #[gpui::test]
    async fn test_task_ordering_with_multiple_test_runners(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            settings::init(cx);
            Project::init_settings(cx);
            language_settings::init(cx);
        });

        // Test case with all test runners present
        let package_json_all_runners = json!({
            "devDependencies": {
                "@types/bun": "1.0.0",
                "@types/node": "^20.0.0",
                "jest": "29.0.0",
                "mocha": "10.0.0",
                "vitest": "1.0.0",
                "jasmine": "5.0.0",
            },
            "scripts": {
                "test": "jest"
            }
        })
        .to_string();

        let fs = FakeFs::new(executor);
        fs.insert_tree(
            path!("/root"),
            json!({
                "package.json": package_json_all_runners,
                "file.js": "",
            }),
        )
        .await;

        let provider = TypeScriptContextProvider::new(fs.clone());

        let package_json_data = cx
            .update(|cx| {
                provider.combined_package_json_data(
                    fs.clone(),
                    path!("/root").as_ref(),
                    rel_path("file.js"),
                    cx,
                )
            })
            .await
            .unwrap();

        assert!(package_json_data.jest_package_path.is_some());
        assert!(package_json_data.mocha_package_path.is_some());
        assert!(package_json_data.vitest_package_path.is_some());
        assert!(package_json_data.jasmine_package_path.is_some());
        assert!(package_json_data.bun_package_path.is_some());
        assert!(package_json_data.node_package_path.is_some());

        let mut task_templates = TaskTemplates::default();
        package_json_data.fill_task_templates(&mut task_templates);

        let test_tasks: Vec<_> = task_templates
            .0
            .iter()
            .filter(|template| {
                template.tags.contains(&"ts-test".to_owned())
                    || template.tags.contains(&"js-test".to_owned())
            })
            .map(|template| &template.label)
            .collect();

        let node_test_index = test_tasks
            .iter()
            .position(|label| label.contains("node test"));
        let jest_test_index = test_tasks.iter().position(|label| label.contains("jest"));
        let bun_test_index = test_tasks
            .iter()
            .position(|label| label.contains("bun test"));

        assert!(
            node_test_index.is_some(),
            "Node test tasks should be present"
        );
        assert!(
            jest_test_index.is_some(),
            "Jest test tasks should be present"
        );
        assert!(bun_test_index.is_some(), "Bun test tasks should be present");

        assert!(
            jest_test_index.unwrap() < bun_test_index.unwrap(),
            "Jest should come before Bun"
        );
        assert!(
            bun_test_index.unwrap() < node_test_index.unwrap(),
            "Bun should come before Node"
        );
    }
}
