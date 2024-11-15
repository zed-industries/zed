use anyhow::ensure;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use gpui::AsyncAppContext;
use gpui::{AppContext, Task};
use language::LanguageName;
use language::LanguageToolchainStore;
use language::Toolchain;
use language::ToolchainList;
use language::ToolchainLister;
use language::{ContextProvider, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use pet_core::os_environment::Environment;
use pet_core::python_environment::PythonEnvironmentKind;
use pet_core::Configuration;
use project::lsp_store::language_server_settings;
use serde_json::{json, Value};
use smol::{lock::OnceCell, process::Command};
use std::cmp::Ordering;

use std::sync::Mutex;
use std::{
    any::Any,
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::ResultExt;

const SERVER_PATH: &str = "node_modules/pyright/langserver.index.js";
const NODE_MODULE_RELATIVE_SERVER_PATH: &str = "pyright/langserver.index.js";

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
        Self::SERVER_NAME.clone()
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        let node = delegate.which("node".as_ref()).await?;
        let (node_modules_path, _) = delegate
            .npm_package_installed_version(Self::SERVER_NAME.as_ref())
            .await
            .log_err()??;

        let path = node_modules_path.join(NODE_MODULE_RELATIVE_SERVER_PATH);

        Some(LanguageServerBinary {
            path: node,
            env: None,
            arguments: server_binary_arguments(&path),
        })
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
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::SERVER_NAME.as_ref(),
                &server_path,
                &container_dir,
                &latest_version,
            )
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[(Self::SERVER_NAME.as_ref(), latest_version.as_str())],
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &self.node).await
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
        Some(language::CodeLabel {
            text: label.clone(),
            runs: vec![(0..label.len(), highlight_id)],
            filter_range: 0..label.len(),
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
        toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let toolchain = toolchains
            .active_toolchain(adapter.worktree_id(), LanguageName::new("Python"), cx)
            .await;
        cx.update(move |cx| {
            let mut user_settings =
                language_server_settings(adapter.as_ref(), &Self::SERVER_NAME, cx)
                    .and_then(|s| s.settings.clone())
                    .unwrap_or_default();

            // If python.pythonPath is not set in user config, do so using our toolchain picker.
            if let Some(toolchain) = toolchain {
                if user_settings.is_null() {
                    user_settings = Value::Object(serde_json::Map::default());
                }
                let object = user_settings.as_object_mut().unwrap();
                if let Some(python) = object
                    .entry("python")
                    .or_insert(Value::Object(serde_json::Map::default()))
                    .as_object_mut()
                {
                    python
                        .entry("pythonPath")
                        .or_insert(Value::String(toolchain.path.into()));
                }
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

const PYTHON_UNITTEST_TARGET_TASK_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_UNITTEST_TARGET"));

const PYTHON_ACTIVE_TOOLCHAIN_PATH: VariableName =
    VariableName::Custom(Cow::Borrowed("PYTHON_ACTIVE_ZED_TOOLCHAIN"));
impl ContextProvider for PythonContextProvider {
    fn build_context(
        &self,
        variables: &task::TaskVariables,
        location: &project::Location,
        _: Option<HashMap<String, String>>,
        toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<task::TaskVariables>> {
        let python_module_name = python_module_name_from_relative_path(
            variables.get(&VariableName::RelativeFile).unwrap_or(""),
        );
        let unittest_class_name =
            variables.get(&VariableName::Custom(Cow::Borrowed("_unittest_class_name")));
        let unittest_method_name = variables.get(&VariableName::Custom(Cow::Borrowed(
            "_unittest_method_name",
        )));

        let unittest_target_str = match (unittest_class_name, unittest_method_name) {
            (Some(class_name), Some(method_name)) => {
                format!("{}.{}.{}", python_module_name, class_name, method_name)
            }
            (Some(class_name), None) => format!("{}.{}", python_module_name, class_name),
            (None, None) => python_module_name,
            (None, Some(_)) => return Task::ready(Ok(task::TaskVariables::default())), // should never happen, a TestCase class is the unit of testing
        };

        let unittest_target = (
            PYTHON_UNITTEST_TARGET_TASK_VARIABLE.clone(),
            unittest_target_str,
        );
        let worktree_id = location.buffer.read(cx).file().map(|f| f.worktree_id(cx));
        cx.spawn(move |mut cx| async move {
            let active_toolchain = if let Some(worktree_id) = worktree_id {
                toolchains
                    .active_toolchain(worktree_id, "Python".into(), &mut cx)
                    .await
                    .map_or_else(|| "python3".to_owned(), |toolchain| toolchain.path.into())
            } else {
                String::from("python3")
            };
            let toolchain = (PYTHON_ACTIVE_TOOLCHAIN_PATH, active_toolchain);
            Ok(task::TaskVariables::from_iter([unittest_target, toolchain]))
        })
    }

    fn associated_tasks(
        &self,
        _: Option<Arc<dyn language::File>>,
        _: &AppContext,
    ) -> Option<TaskTemplates> {
        Some(TaskTemplates(vec![
            TaskTemplate {
                label: "execute selection".to_owned(),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec!["-c".to_owned(), VariableName::SelectedText.template_value()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("run '{}'", VariableName::File.template_value()),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![VariableName::File.template_value()],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("unittest '{}'", VariableName::File.template_value()),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![
                    "-m".to_owned(),
                    "unittest".to_owned(),
                    VariableName::File.template_value(),
                ],
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: "unittest $ZED_CUSTOM_PYTHON_UNITTEST_TARGET".to_owned(),
                command: PYTHON_ACTIVE_TOOLCHAIN_PATH.template_value(),
                args: vec![
                    "-m".to_owned(),
                    "unittest".to_owned(),
                    "$ZED_CUSTOM_PYTHON_UNITTEST_TARGET".to_owned(),
                ],
                tags: vec![
                    "python-unittest-class".to_owned(),
                    "python-unittest-method".to_owned(),
                ],
                ..TaskTemplate::default()
            },
        ]))
    }
}

fn python_module_name_from_relative_path(relative_path: &str) -> String {
    let path_with_dots = relative_path.replace('/', ".");
    path_with_dots
        .strip_suffix(".py")
        .unwrap_or(&path_with_dots)
        .to_string()
}

#[derive(Default)]
pub(crate) struct PythonToolchainProvider {}

static ENV_PRIORITY_LIST: &'static [PythonEnvironmentKind] = &[
    // Prioritize non-Conda environments.
    PythonEnvironmentKind::Poetry,
    PythonEnvironmentKind::Pipenv,
    PythonEnvironmentKind::VirtualEnvWrapper,
    PythonEnvironmentKind::Venv,
    PythonEnvironmentKind::VirtualEnv,
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

#[async_trait(?Send)]
impl ToolchainLister for PythonToolchainProvider {
    async fn list(
        &self,
        worktree_root: PathBuf,
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
        config.workspace_directories = Some(vec![worktree_root]);
        for locator in locators.iter() {
            locator.configure(&config);
        }

        let reporter = pet_reporter::collect::create_reporter();
        pet::find::find_and_report_envs(&reporter, config, &locators, &environment, None);

        let mut toolchains = reporter
            .environments
            .lock()
            .ok()
            .map_or(Vec::new(), |mut guard| std::mem::take(&mut guard));

        toolchains.sort_by(|lhs, rhs| {
            env_priority(lhs.kind)
                .cmp(&env_priority(rhs.kind))
                .then_with(|| {
                    if lhs.kind == Some(PythonEnvironmentKind::Conda) {
                        environment
                            .get_env_var("CONDA_PREFIX".to_string())
                            .map(|conda_prefix| {
                                let is_match = |exe: &Option<PathBuf>| {
                                    exe.as_ref().map_or(false, |e| e.starts_with(&conda_prefix))
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
                })
                .then_with(|| lhs.executable.cmp(&rhs.executable))
        });

        let mut toolchains: Vec<_> = toolchains
            .into_iter()
            .filter_map(|toolchain| {
                let name = if let Some(version) = &toolchain.version {
                    format!("Python {version} ({:?})", toolchain.kind?)
                } else {
                    format!("{:?}", toolchain.kind?)
                }
                .into();
                Some(Toolchain {
                    name,
                    path: toolchain.executable?.to_str()?.to_owned().into(),
                    language_name: LanguageName::new("Python"),
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

impl<'a> pet_core::os_environment::Environment for EnvironmentApi<'a> {
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
        if self.global_search_locations.lock().unwrap().is_empty() {
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

            self.global_search_locations
                .lock()
                .unwrap()
                .append(&mut paths);
        }
        self.global_search_locations.lock().unwrap().clone()
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
            .ok_or_else(|| anyhow!("Could not find Python installation for PyLSP"))?;
        let work_dir = delegate
            .language_server_download_dir(&Self::SERVER_NAME)
            .await
            .ok_or_else(|| anyhow!("Could not get working directory for PyLSP"))?;
        let mut path = PathBuf::from(work_dir.as_ref());
        path.push("pylsp-venv");
        if !path.exists() {
            Command::new(python_path)
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

#[async_trait(?Send)]
impl LspAdapter for PyLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
    }

    async fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: &AsyncAppContext,
    ) -> Option<LanguageServerBinary> {
        // We don't support user-provided pylsp, as global packages are discouraged in Python ecosystem.
        None
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        // let uri = "https://pypi.org/pypi/python-lsp-server/json";
        // let mut root_manifest = delegate
        //     .http_client()
        //     .get(&uri, Default::default(), true)
        //     .await?;
        // let mut body = Vec::new();
        // root_manifest.body_mut().read_to_end(&mut body).await?;
        // let as_str = String::from_utf8(body)?;
        // let json = serde_json::Value::from_str(&as_str)?;
        // let latest_version = json
        //     .get("info")
        //     .and_then(|info| info.get("version"))
        //     .and_then(|version| version.as_str().map(ToOwned::to_owned))
        //     .ok_or_else(|| {
        //         anyhow!("PyPI response did not contain version info for python-language-server")
        //     })?;
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let venv = self.base_venv(delegate).await.map_err(|e| anyhow!(e))?;
        let pip_path = venv.join("bin").join("pip3");
        ensure!(
            Command::new(pip_path.as_path())
                .arg("install")
                .arg("python-lsp-server")
                .output()
                .await?
                .status
                .success(),
            "python-lsp-server installation failed"
        );
        ensure!(
            Command::new(pip_path.as_path())
                .arg("install")
                .arg("python-lsp-server[all]")
                .output()
                .await?
                .status
                .success(),
            "python-lsp-server[all] installation failed"
        );
        ensure!(
            Command::new(pip_path)
                .arg("install")
                .arg("pylsp-mypy")
                .output()
                .await?
                .status
                .success(),
            "pylsp-mypy installation failed"
        );
        let pylsp = venv.join("bin").join("pylsp");
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
        let pylsp = venv.join("bin").join("pylsp");
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
        Some(language::CodeLabel {
            text: label.clone(),
            runs: vec![(0..label.len(), highlight_id)],
            filter_range: 0..label.len(),
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
        toolchains: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let toolchain = toolchains
            .active_toolchain(adapter.worktree_id(), LanguageName::new("Python"), cx)
            .await;
        cx.update(move |cx| {
            let mut user_settings =
                language_server_settings(adapter.as_ref(), &Self::SERVER_NAME, cx)
                    .and_then(|s| s.settings.clone())
                    .unwrap_or_else(|| {
                        json!({
                            "plugins": {
                                "rope_autoimport": {"enabled": true},
                                "mypy": {"enabled": true}
                            }
                        })
                    });

            // If python.pythonPath is not set in user config, do so using our toolchain picker.
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
                        jedi.insert(
                            "environment".to_string(),
                            Value::String(toolchain.path.clone().into()),
                        );
                    }
                    if let Some(pylint) = python
                        .entry("mypy")
                        .or_insert(Value::Object(serde_json::Map::default()))
                        .as_object_mut()
                    {
                        pylint.insert(
                            "overrides".to_string(),
                            Value::Array(vec![
                                Value::String("--python-executable".into()),
                                Value::String(toolchain.path.into()),
                            ]),
                        );
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

#[cfg(test)]
mod tests {
    use gpui::{BorrowAppContext, Context, ModelContext, TestAppContext};
    use language::{language_settings::AllLanguageSettings, AutoindentMode, Buffer};
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

        cx.new_model(|cx| {
            let mut buffer = Buffer::local("", cx).with_language(language, cx);
            let append = |buffer: &mut Buffer, text: &str, cx: &mut ModelContext<Buffer>| {
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

            // indent after an open paren. the closing  paren is not indented
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
