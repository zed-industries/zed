use anyhow::{Context as _, ensure};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use gpui::{App, Task};
use gpui::{AsyncApp, SharedString};
use language::Toolchain;
use language::ToolchainList;
use language::ToolchainLister;
use language::language_settings::language_settings;
use language::{ContextLocation, LanguageToolchainStore};
use language::{ContextProvider, LspAdapter, LspAdapterDelegate};
use language::{LanguageName, ManifestName, ManifestProvider, ManifestQuery};
use lsp::LanguageServerBinary;
use lsp::LanguageServerName;
use node_runtime::{NodeRuntime, VersionStrategy};
use pet_core::Configuration;
use pet_core::os_environment::Environment;
use pet_core::python_environment::PythonEnvironmentKind;
use project::Fs;
use project::lsp_store::language_server_settings;
use serde_json::{Value, json};
use smol::lock::OnceCell;
use std::cmp::Ordering;

use parking_lot::Mutex;
use std::str::FromStr;
use std::{
    any::Any,
    borrow::Cow,
    ffi::OsString,
    fmt::Write,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::ResultExt;

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

const SERVER_PATH: &str = "node_modules/pyright/langserver.index.js";
const NODE_MODULE_RELATIVE_SERVER_PATH: &str = "pyright/langserver.index.js";

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

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct PythonLspAdapter {
    node: NodeRuntime,
}

impl PythonLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("pyright");

    pub fn new(node: NodeRuntime) -> Self {
        PythonLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for PythonLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &dyn Fs,
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

            let path = node_modules_path.join(NODE_MODULE_RELATIVE_SERVER_PATH);

            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: node,
                env: Some(env),
                arguments: server_binary_arguments(&path),
            })
        }
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version(Self::SERVER_NAME.as_ref())
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);

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
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &(dyn 'static + Send + Any),
        container_dir: &PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let version = version.downcast_ref::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);

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
                arguments: server_binary_arguments(&server_path),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let mut binary = get_cached_server_binary(container_dir, &self.node).await?;
        binary.env = Some(delegate.shell_env().await);
        Some(binary)
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        // Pyright assigns each completion item a `sortText` of the form `XX.YYYY.name`.
        // Where `XX` is the sorting category, `YYYY` is based on most recent usage,
        // and `name` is the symbol name itself.
        //
        // Because the symbol name is included, there generally are not ties when
        // sorting by the `sortText`, so the symbol's fuzzy match score is not taken
        // into account. Here, we remove the symbol name from the sortText in order
        // to allow our own fuzzy score to be used to break ties.
        //
        // see https://github.com/microsoft/pyright/blob/95ef4e103b9b2f129c9320427e51b73ea7cf78bd/packages/pyright-internal/src/languageService/completionProvider.ts#LL2873
        for item in items {
            let Some(sort_text) = &mut item.sort_text else {
                continue;
            };
            let mut parts = sort_text.split('.');
            let Some(first) = parts.next() else { continue };
            let Some(second) = parts.next() else { continue };
            let Some(_) = parts.next() else { continue };
            sort_text.replace_range(first.len() + second.len() + 1.., "");
        }
    }

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
        _: &dyn Fs,
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
            if let Some(toolchain) = toolchain {
                if user_settings.is_null() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();

                let interpreter_path = toolchain.path.to_string();

                // Detect if this is a virtual environment
                if let Some(interpreter_dir) = Path::new(&interpreter_path).parent()
                    && let Some(venv_dir) = interpreter_dir.parent()
                {
                    // Check if this looks like a virtual environment
                    if venv_dir.join("pyvenv.cfg").exists()
                        || venv_dir.join("bin/activate").exists()
                        || venv_dir.join("Scripts/activate.bat").exists()
                    {
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
                }

                // Always set the python interpreter path
                // Get or create the python section
                let python = object
                    .entry("python")
                    .or_insert(Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                    .unwrap();

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

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    let server_path = container_dir.join(SERVER_PATH);
    if server_path.exists() {
        Some(LanguageServerBinary {
            path: node.binary_path().await.log_err()?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    } else {
        log::error!("missing executable in directory {:?}", server_path);
        None
    }
}

pub(crate) struct PythonContextProvider;

const PYTHON_TEST_TARGET_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_TEST_TARGET"));

const PYTHON_ACTIVE_TOOLCHAIN_PATH: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_ACTIVE_ZED_TOOLCHAIN"));

const PYTHON_ACTIVE_TOOLCHAIN_PATH_RAW: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_ACTIVE_ZED_TOOLCHAIN_RAW"));

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
            let raw_toolchain = if let Some(worktree_id) = worktree_id {
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

            let active_toolchain = format!("\"{raw_toolchain}\"");
            let toolchain = (PYTHON_ACTIVE_TOOLCHAIN_PATH, active_toolchain);
            let raw_toolchain_var = (PYTHON_ACTIVE_TOOLCHAIN_PATH_RAW, raw_toolchain);

            Ok(task::TaskVariables::from_iter(
                test_target
                    .into_iter()
                    .chain(module_target.into_iter())
                    .chain([toolchain, raw_toolchain_var]),
            ))
        })
    }

    fn associated_tasks(
        &self,
        _: Arc<dyn Fs>,
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
                cwd: Some("$ZED_WORKTREE_ROOT".into()),
                ..TaskTemplate::default()
            },
            // Execute an entire file
            TaskTemplate {
                label: format!("run '{}'", VariableName::File.template_value()),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![VariableName::File.template_value_with_whitespace()],
                cwd: Some("$ZED_WORKTREE_ROOT".into()),
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
                cwd: Some("$ZED_WORKTREE_ROOT".into()),
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
                        cwd: Some("$ZED_WORKTREE_ROOT".into()),
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
                        cwd: Some("$ZED_WORKTREE_ROOT".into()),
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
                        cwd: Some("$ZED_WORKTREE_ROOT".into()),
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
                        cwd: Some("$ZED_WORKTREE_ROOT".into()),
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

pub(crate) struct PythonToolchainProvider {
    term: SharedString,
}

impl Default for PythonToolchainProvider {
    fn default() -> Self {
        Self {
            term: SharedString::new_static("Virtual Environment"),
        }
    }
}

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
fn get_worktree_venv_declaration(worktree_root: &Path) -> Option<String> {
    fs::File::open(worktree_root.join(".venv"))
        .and_then(|file| {
            let mut venv_name = String::new();
            io::BufReader::new(file).read_line(&mut venv_name)?;
            Ok(venv_name.trim().to_string())
        })
        .ok()
}

#[async_trait]
impl ToolchainLister for PythonToolchainProvider {
    fn manifest_name(&self) -> language::ManifestName {
        ManifestName::from(SharedString::new_static("pyproject.toml"))
    }
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
        let wr_venv = get_worktree_venv_declaration(&wr);
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
            let proj_ordering = || match (&lhs.project, &rhs.project) {
                (Some(l), Some(r)) => (r == &wr).cmp(&(l == &wr)),
                (Some(l), None) if l == &wr => Ordering::Less,
                (None, Some(r)) if r == &wr => Ordering::Greater,
                _ => Ordering::Equal,
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
            .filter_map(|toolchain| {
                let mut name = String::from("Python");
                if let Some(ref version) = toolchain.version {
                    _ = write!(name, " {version}");
                }

                let name_and_kind = match (&toolchain.name, &toolchain.kind) {
                    (Some(name), Some(kind)) => {
                        Some(format!("({name}; {})", python_env_kind_display(kind)))
                    }
                    (Some(name), None) => Some(format!("({name})")),
                    (None, Some(kind)) => Some(format!("({})", python_env_kind_display(kind))),
                    (None, None) => None,
                };

                if let Some(nk) = name_and_kind {
                    _ = write!(name, " {nk}");
                }

                Some(Toolchain {
                    name: name.into(),
                    path: toolchain.executable.as_ref()?.to_str()?.to_owned().into(),
                    language_name: LanguageName::new("Python"),
                    as_json: serde_json::to_value(toolchain).ok()?,
                })
            })
            .collect();
        toolchains.dedup();
        ToolchainList {
            toolchains,
            default: None,
            groups: Default::default(),
        }
    }
    fn term(&self) -> SharedString {
        self.term.clone()
    }
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
            let venv = toolchain?;
            let pylsp_path = Path::new(venv.path.as_ref()).parent()?.join("pylsp");
            pylsp_path.exists().then(|| LanguageServerBinary {
                path: venv.path.to_string().into(),
                arguments: vec![pylsp_path.into()],
                env: None,
            })
        }
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.map_err(|e| anyhow!(e))?;
        let pip_path = venv.join(BINARY_DIR).join("pip3");
        ensure!(
            util::command::new_smol_command(pip_path.as_path())
                .arg("install")
                .arg("python-lsp-server")
                .arg("-U")
                .output()
                .await?
                .status
                .success(),
            "python-lsp-server installation failed"
        );
        ensure!(
            util::command::new_smol_command(pip_path.as_path())
                .arg("install")
                .arg("python-lsp-server[all]")
                .arg("-U")
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
                .arg("-U")
                .output()
                .await?
                .status
                .success(),
            "pylsp-mypy installation failed"
        );
        let pylsp = venv.join(BINARY_DIR).join("pylsp");
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
        Some(LanguageServerBinary {
            path: pylsp,
            env: None,
            arguments: vec![],
        })
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
        _: &dyn Fs,
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
                if user_settings.is_null() {
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

pub(crate) struct BasedPyrightLspAdapter {
    python_venv_base: OnceCell<Result<Arc<Path>, String>>,
}

impl BasedPyrightLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("basedpyright");
    const BINARY_NAME: &'static str = "basedpyright-langserver";

    pub(crate) fn new() -> Self {
        Self {
            python_venv_base: OnceCell::new(),
        }
    }

    async fn ensure_venv(delegate: &dyn LspAdapterDelegate) -> Result<Arc<Path>> {
        let python_path = Self::find_base_python(delegate)
            .await
            .context("Could not find Python installation for basedpyright")?;
        let work_dir = delegate
            .language_server_download_dir(&Self::SERVER_NAME)
            .await
            .context("Could not get working directory for basedpyright")?;
        let mut path = PathBuf::from(work_dir.as_ref());
        path.push("basedpyright-venv");
        if !path.exists() {
            util::command::new_smol_command(python_path)
                .arg("-m")
                .arg("venv")
                .arg("basedpyright-venv")
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

#[async_trait(?Send)]
impl LspAdapter for BasedPyrightLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &dyn Fs,
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

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        toolchain: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        if let Some(bin) = delegate.which(Self::BINARY_NAME.as_ref()).await {
            let env = delegate.shell_env().await;
            Some(LanguageServerBinary {
                path: bin,
                env: Some(env),
                arguments: vec!["--stdio".into()],
            })
        } else {
            let path = Path::new(toolchain?.path.as_ref())
                .parent()?
                .join(Self::BINARY_NAME);
            path.exists().then(|| LanguageServerBinary {
                path,
                arguments: vec!["--stdio".into()],
                env: None,
            })
        }
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _latest_version: Box<dyn 'static + Send + Any>,
        _container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.map_err(|e| anyhow!(e))?;
        let pip_path = venv.join(BINARY_DIR).join("pip3");
        ensure!(
            util::command::new_smol_command(pip_path.as_path())
                .arg("install")
                .arg("basedpyright")
                .arg("-U")
                .output()
                .await?
                .status
                .success(),
            "basedpyright installation failed"
        );
        let pylsp = venv.join(BINARY_DIR).join(Self::BINARY_NAME);
        Ok(LanguageServerBinary {
            path: pylsp,
            env: None,
            arguments: vec!["--stdio".into()],
        })
    }

    async fn cached_server_binary(
        &self,
        _container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.ok()?;
        let pylsp = venv.join(BINARY_DIR).join(Self::BINARY_NAME);
        Some(LanguageServerBinary {
            path: pylsp,
            env: None,
            arguments: vec!["--stdio".into()],
        })
    }

    async fn process_completions(&self, items: &mut [lsp::CompletionItem]) {
        // Pyright assigns each completion item a `sortText` of the form `XX.YYYY.name`.
        // Where `XX` is the sorting category, `YYYY` is based on most recent usage,
        // and `name` is the symbol name itself.
        //
        // Because the symbol name is included, there generally are not ties when
        // sorting by the `sortText`, so the symbol's fuzzy match score is not taken
        // into account. Here, we remove the symbol name from the sortText in order
        // to allow our own fuzzy score to be used to break ties.
        //
        // see https://github.com/microsoft/pyright/blob/95ef4e103b9b2f129c9320427e51b73ea7cf78bd/packages/pyright-internal/src/languageService/completionProvider.ts#LL2873
        for item in items {
            let Some(sort_text) = &mut item.sort_text else {
                continue;
            };
            let mut parts = sort_text.split('.');
            let Some(first) = parts.next() else { continue };
            let Some(second) = parts.next() else { continue };
            let Some(_) = parts.next() else { continue };
            sort_text.replace_range(first.len() + second.len() + 1.., "");
        }
    }

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
        _: &dyn Fs,
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
            if let Some(toolchain) = toolchain {
                if user_settings.is_null() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();

                let interpreter_path = toolchain.path.to_string();

                // Detect if this is a virtual environment
                if let Some(interpreter_dir) = Path::new(&interpreter_path).parent()
                    && let Some(venv_dir) = interpreter_dir.parent()
                {
                    // Check if this looks like a virtual environment
                    if venv_dir.join("pyvenv.cfg").exists()
                        || venv_dir.join("bin/activate").exists()
                        || venv_dir.join("Scripts/activate.bat").exists()
                    {
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
                }

                // Always set the python interpreter path
                // Get or create the python section
                let python = object
                    .entry("python")
                    .or_insert(Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                    .unwrap();

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

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, BorrowAppContext, Context, TestAppContext};
    use language::{AutoindentMode, Buffer, language_settings::AllLanguageSettings};
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
                store.update_user_settings::<AllLanguageSettings>(cx, |s| {
                    s.defaults.tab_size = NonZeroU32::new(2);
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

            // reset to a simple if statement
            buffer.edit([(0..buffer.len(), "if a:\n  b(\n  )")], None, cx);

            // dedent "else" on the line after a closing paren
            append(&mut buffer, "\n  else:\n", cx);
            assert_eq!(buffer.text(), "if a:\n  b(\n  )\nelse:\n  ");

            buffer
        });
    }
}
