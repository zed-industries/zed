use crate::{
    language_settings::{
        all_language_settings, AllLanguageSettingsContent, LanguageSettingsContent,
    },
    task_context::ContextProvider,
    with_parser, CachedLspAdapter, File, Language, LanguageConfig, LanguageId, LanguageMatcher,
    LanguageServerName, LspAdapter, LspAdapterDelegate, PLAIN_TEXT,
};
use anyhow::{anyhow, Context as _, Result};
use collections::{hash_map, HashMap, HashSet};
use futures::TryFutureExt;
use futures::{
    channel::{mpsc, oneshot},
    future::Shared,
    Future, FutureExt as _,
};
use globset::GlobSet;
use gpui::{AppContext, BackgroundExecutor, Task};
use lsp::LanguageServerId;
use parking_lot::{Mutex, RwLock};
use postage::watch;
use std::{
    borrow::Cow,
    ffi::OsStr,
    ops::Not,
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::Bias;
use text::{Point, Rope};
use theme::Theme;
use unicase::UniCase;
use util::{maybe, paths::PathExt, post_inc, ResultExt};

pub struct LanguageRegistry {
    state: RwLock<LanguageRegistryState>,
    language_server_download_dir: Option<Arc<Path>>,
    login_shell_env_loaded: Shared<Task<()>>,
    executor: BackgroundExecutor,
    lsp_binary_status_tx: LspBinaryStatusSender,
}

struct LanguageRegistryState {
    next_language_server_id: usize,
    languages: Vec<Arc<Language>>,
    language_settings: AllLanguageSettingsContent,
    available_languages: Vec<AvailableLanguage>,
    grammars: HashMap<Arc<str>, AvailableGrammar>,
    lsp_adapters: HashMap<Arc<str>, Vec<Arc<CachedLspAdapter>>>,
    available_lsp_adapters:
        HashMap<LanguageServerName, Arc<dyn Fn() -> Arc<CachedLspAdapter> + 'static + Send + Sync>>,
    loading_languages: HashMap<LanguageId, Vec<oneshot::Sender<Result<Arc<Language>>>>>,
    subscription: (watch::Sender<()>, watch::Receiver<()>),
    theme: Option<Arc<Theme>>,
    version: usize,
    reload_count: usize,

    #[cfg(any(test, feature = "test-support"))]
    fake_server_txs:
        HashMap<Arc<str>, Vec<futures::channel::mpsc::UnboundedSender<lsp::FakeLanguageServer>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageServerBinaryStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Failed { error: String },
}

pub struct PendingLanguageServer {
    pub server_id: LanguageServerId,
    pub task: Task<Result<(lsp::LanguageServer, Option<serde_json::Value>)>>,
    pub container_dir: Option<Arc<Path>>,
}

#[derive(Clone)]
struct AvailableLanguage {
    id: LanguageId,
    name: Arc<str>,
    grammar: Option<Arc<str>>,
    matcher: LanguageMatcher,
    load: Arc<
        dyn Fn() -> Result<(
                LanguageConfig,
                LanguageQueries,
                Option<Arc<dyn ContextProvider>>,
            )>
            + 'static
            + Send
            + Sync,
    >,
    loaded: bool,
}

enum AvailableGrammar {
    Native(tree_sitter::Language),
    Loaded(#[allow(unused)] PathBuf, tree_sitter::Language),
    Loading(
        #[allow(unused)] PathBuf,
        Vec<oneshot::Sender<Result<tree_sitter::Language, Arc<anyhow::Error>>>>,
    ),
    Unloaded(PathBuf),
    LoadFailed(Arc<anyhow::Error>),
}

#[derive(Debug)]
pub struct LanguageNotFound;

impl std::fmt::Display for LanguageNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "language not found")
    }
}

pub const QUERY_FILENAME_PREFIXES: &[(
    &str,
    fn(&mut LanguageQueries) -> &mut Option<Cow<'static, str>>,
)] = &[
    ("highlights", |q| &mut q.highlights),
    ("brackets", |q| &mut q.brackets),
    ("outline", |q| &mut q.outline),
    ("indents", |q| &mut q.indents),
    ("embedding", |q| &mut q.embedding),
    ("injections", |q| &mut q.injections),
    ("overrides", |q| &mut q.overrides),
    ("redactions", |q| &mut q.redactions),
    ("runnables", |q| &mut q.runnables),
];

/// Tree-sitter language queries for a given language.
#[derive(Debug, Default)]
pub struct LanguageQueries {
    pub highlights: Option<Cow<'static, str>>,
    pub brackets: Option<Cow<'static, str>>,
    pub indents: Option<Cow<'static, str>>,
    pub outline: Option<Cow<'static, str>>,
    pub embedding: Option<Cow<'static, str>>,
    pub injections: Option<Cow<'static, str>>,
    pub overrides: Option<Cow<'static, str>>,
    pub redactions: Option<Cow<'static, str>>,
    pub runnables: Option<Cow<'static, str>>,
}

#[derive(Clone, Default)]
struct LspBinaryStatusSender {
    txs: Arc<Mutex<Vec<mpsc::UnboundedSender<(LanguageServerName, LanguageServerBinaryStatus)>>>>,
}

impl LanguageRegistry {
    pub fn new(login_shell_env_loaded: Task<()>, executor: BackgroundExecutor) -> Self {
        let this = Self {
            state: RwLock::new(LanguageRegistryState {
                next_language_server_id: 0,
                languages: Vec::new(),
                available_languages: Vec::new(),
                grammars: Default::default(),
                language_settings: Default::default(),
                loading_languages: Default::default(),
                lsp_adapters: Default::default(),
                available_lsp_adapters: HashMap::default(),
                subscription: watch::channel(),
                theme: Default::default(),
                version: 0,
                reload_count: 0,

                #[cfg(any(test, feature = "test-support"))]
                fake_server_txs: Default::default(),
            }),
            language_server_download_dir: None,
            login_shell_env_loaded: login_shell_env_loaded.shared(),
            lsp_binary_status_tx: Default::default(),
            executor,
        };
        this.add(PLAIN_TEXT.clone());
        this
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(executor: BackgroundExecutor) -> Self {
        let mut this = Self::new(Task::ready(()), executor);
        this.language_server_download_dir = Some(Path::new("/the-download-dir").into());
        this
    }

    /// Clears out all of the loaded languages and reload them from scratch.
    pub fn reload(&self) {
        self.state.write().reload();
    }

    /// Reorders the list of language servers for the given language.
    ///
    /// Uses the provided list of ordered [`CachedLspAdapters`] as the desired order.
    ///
    /// Any existing language servers not present in `ordered_lsp_adapters` will be
    /// appended to the end.
    pub fn reorder_language_servers(
        &self,
        language: &Arc<Language>,
        ordered_lsp_adapters: Vec<Arc<CachedLspAdapter>>,
    ) {
        self.state
            .write()
            .reorder_language_servers(language, ordered_lsp_adapters);
    }

    /// Removes the specified languages and grammars from the registry.
    pub fn remove_languages(
        &self,
        languages_to_remove: &[Arc<str>],
        grammars_to_remove: &[Arc<str>],
    ) {
        self.state
            .write()
            .remove_languages(languages_to_remove, grammars_to_remove)
    }

    pub fn remove_lsp_adapter(&self, language_name: &str, name: &LanguageServerName) {
        let mut state = self.state.write();
        if let Some(adapters) = state.lsp_adapters.get_mut(language_name) {
            adapters.retain(|adapter| &adapter.name != name)
        }
        state.version += 1;
        state.reload_count += 1;
        *state.subscription.0.borrow_mut() = ();
    }

    #[cfg(any(feature = "test-support", test))]
    pub fn register_test_language(&self, config: LanguageConfig) {
        self.register_language(
            config.name.clone(),
            config.grammar.clone(),
            config.matcher.clone(),
            move || Ok((config.clone(), Default::default(), None)),
        )
    }

    /// Registers an available language server adapter.
    ///
    /// The language server is registered under the language server name, but
    /// not bound to a particular language.
    ///
    /// When a language wants to load this particular language server, it will
    /// invoke the `load` function.
    pub fn register_available_lsp_adapter(
        &self,
        name: LanguageServerName,
        load: impl Fn() -> Arc<dyn LspAdapter> + 'static + Send + Sync,
    ) {
        self.state.write().available_lsp_adapters.insert(
            name,
            Arc::new(move || {
                let lsp_adapter = load();
                CachedLspAdapter::new(lsp_adapter)
            }),
        );
    }

    /// Loads the language server adapter for the language server with the given name.
    pub fn load_available_lsp_adapter(
        &self,
        name: &LanguageServerName,
    ) -> Option<Arc<CachedLspAdapter>> {
        let state = self.state.read();
        let load_lsp_adapter = state.available_lsp_adapters.get(name)?;

        Some(load_lsp_adapter())
    }

    pub fn register_lsp_adapter(&self, language_name: Arc<str>, adapter: Arc<dyn LspAdapter>) {
        self.state
            .write()
            .lsp_adapters
            .entry(language_name)
            .or_default()
            .push(CachedLspAdapter::new(adapter));
    }

    #[cfg(any(feature = "test-support", test))]
    pub fn register_fake_lsp_adapter(
        &self,
        language_name: &str,
        adapter: crate::FakeLspAdapter,
    ) -> futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        self.state
            .write()
            .lsp_adapters
            .entry(language_name.into())
            .or_default()
            .push(CachedLspAdapter::new(Arc::new(adapter)));
        self.fake_language_servers(language_name)
    }

    #[cfg(any(feature = "test-support", test))]
    pub fn fake_language_servers(
        &self,
        language_name: &str,
    ) -> futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let (servers_tx, servers_rx) = futures::channel::mpsc::unbounded();
        self.state
            .write()
            .fake_server_txs
            .entry(language_name.into())
            .or_default()
            .push(servers_tx);
        servers_rx
    }

    /// Adds a language to the registry, which can be loaded if needed.
    pub fn register_language(
        &self,
        name: Arc<str>,
        grammar_name: Option<Arc<str>>,
        matcher: LanguageMatcher,
        load: impl Fn() -> Result<(
                LanguageConfig,
                LanguageQueries,
                Option<Arc<dyn ContextProvider>>,
            )>
            + 'static
            + Send
            + Sync,
    ) {
        let load = Arc::new(load);
        let state = &mut *self.state.write();

        for existing_language in &mut state.available_languages {
            if existing_language.name == name {
                existing_language.grammar = grammar_name;
                existing_language.matcher = matcher;
                existing_language.load = load;
                return;
            }
        }

        state.available_languages.push(AvailableLanguage {
            id: LanguageId::new(),
            name,
            grammar: grammar_name,
            matcher,
            load,
            loaded: false,
        });
        state.version += 1;
        state.reload_count += 1;
        *state.subscription.0.borrow_mut() = ();
    }

    /// Adds grammars to the registry. Language configurations reference a grammar by name. The
    /// grammar controls how the source code is parsed.
    pub fn register_native_grammars(
        &self,
        grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, tree_sitter::Language)>,
    ) {
        self.state.write().grammars.extend(
            grammars
                .into_iter()
                .map(|(name, grammar)| (name.into(), AvailableGrammar::Native(grammar))),
        );
    }

    /// Adds paths to WASM grammar files, which can be loaded if needed.
    pub fn register_wasm_grammars(
        &self,
        grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, PathBuf)>,
    ) {
        let mut state = self.state.write();
        state.grammars.extend(
            grammars
                .into_iter()
                .map(|(name, path)| (name.into(), AvailableGrammar::Unloaded(path))),
        );
        state.version += 1;
        state.reload_count += 1;
        *state.subscription.0.borrow_mut() = ();
    }

    pub fn language_settings(&self) -> AllLanguageSettingsContent {
        self.state.read().language_settings.clone()
    }

    pub fn language_names(&self) -> Vec<String> {
        let state = self.state.read();
        let mut result = state
            .available_languages
            .iter()
            .filter_map(|l| l.loaded.not().then_some(l.name.to_string()))
            .chain(state.languages.iter().map(|l| l.config.name.to_string()))
            .collect::<Vec<_>>();
        result.sort_unstable_by_key(|language_name| language_name.to_lowercase());
        result
    }

    pub fn grammar_names(&self) -> Vec<Arc<str>> {
        let state = self.state.read();
        let mut result = state.grammars.keys().cloned().collect::<Vec<_>>();
        result.sort_unstable_by_key(|grammar_name| grammar_name.to_lowercase());
        result
    }

    /// Add a pre-loaded language to the registry.
    pub fn add(&self, language: Arc<Language>) {
        let mut state = self.state.write();
        state.available_languages.push(AvailableLanguage {
            id: language.id,
            name: language.name(),
            grammar: language.config.grammar.clone(),
            matcher: language.config.matcher.clone(),
            load: Arc::new(|| Err(anyhow!("already loaded"))),
            loaded: true,
        });
        state.add(language);
    }

    pub fn subscribe(&self) -> watch::Receiver<()> {
        self.state.read().subscription.1.clone()
    }

    /// Returns the number of times that the registry has been changed,
    /// by adding languages or reloading.
    pub fn version(&self) -> usize {
        self.state.read().version
    }

    /// Returns the number of times that the registry has been reloaded.
    pub fn reload_count(&self) -> usize {
        self.state.read().reload_count
    }

    pub fn set_theme(&self, theme: Arc<Theme>) {
        let mut state = self.state.write();
        state.theme = Some(theme.clone());
        for language in &state.languages {
            language.set_theme(theme.syntax());
        }
    }

    pub fn set_language_server_download_dir(&mut self, path: impl Into<Arc<Path>>) {
        self.language_server_download_dir = Some(path.into());
    }

    pub fn language_for_name(
        self: &Arc<Self>,
        name: &str,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let name = UniCase::new(name);
        let rx = self.get_or_load_language(|language_name, _| {
            if UniCase::new(language_name) == name {
                1
            } else {
                0
            }
        });
        async move { rx.await? }
    }

    pub fn language_for_name_or_extension(
        self: &Arc<Self>,
        string: &str,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let string = UniCase::new(string);
        let rx = self.get_or_load_language(|name, config| {
            if UniCase::new(name) == string
                || config
                    .path_suffixes
                    .iter()
                    .any(|suffix| UniCase::new(suffix) == string)
            {
                1
            } else {
                0
            }
        });
        async move { rx.await? }
    }

    pub fn language_for_file(
        self: &Arc<Self>,
        file: &Arc<dyn File>,
        content: Option<&Rope>,
        cx: &AppContext,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let user_file_types = all_language_settings(Some(file), cx);
        self.language_for_file_internal(
            &file.full_path(cx),
            content,
            Some(&user_file_types.file_types),
        )
    }

    pub fn language_for_file_path<'a>(
        self: &Arc<Self>,
        path: &'a Path,
    ) -> impl Future<Output = Result<Arc<Language>>> + 'a {
        self.language_for_file_internal(path, None, None)
            .map_err(|error| error.context(format!("language for file path {}", path.display())))
    }

    fn language_for_file_internal(
        self: &Arc<Self>,
        path: &Path,
        content: Option<&Rope>,
        user_file_types: Option<&HashMap<Arc<str>, GlobSet>>,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension_or_hidden_file_name();
        let path_suffixes = [extension, filename, path.to_str()];
        let empty = GlobSet::empty();

        let rx = self.get_or_load_language(move |language_name, config| {
            let path_matches_default_suffix = config
                .path_suffixes
                .iter()
                .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())));
            let custom_suffixes = user_file_types
                .and_then(|types| types.get(language_name))
                .unwrap_or(&empty);
            let path_matches_custom_suffix = path_suffixes
                .iter()
                .map(|suffix| suffix.unwrap_or(""))
                .any(|suffix| custom_suffixes.is_match(suffix));
            let content_matches = content.zip(config.first_line_pattern.as_ref()).map_or(
                false,
                |(content, pattern)| {
                    let end = content.clip_point(Point::new(0, 256), Bias::Left);
                    let end = content.point_to_offset(end);
                    let text = content.chunks_in_range(0..end).collect::<String>();
                    pattern.is_match(&text)
                },
            );
            if path_matches_custom_suffix {
                2
            } else if path_matches_default_suffix || content_matches {
                1
            } else {
                0
            }
        });
        async move { rx.await? }
    }

    fn get_or_load_language(
        self: &Arc<Self>,
        callback: impl Fn(&str, &LanguageMatcher) -> usize,
    ) -> oneshot::Receiver<Result<Arc<Language>>> {
        let (tx, rx) = oneshot::channel();

        let mut state = self.state.write();
        let Some((language, _)) = state
            .available_languages
            .iter()
            .filter_map(|language| {
                let score = callback(&language.name, &language.matcher);
                if score > 0 {
                    Some((language.clone(), score))
                } else {
                    None
                }
            })
            .max_by_key(|e| e.1)
            .clone()
        else {
            let _ = tx.send(Err(anyhow!(LanguageNotFound)));
            return rx;
        };

        // If the language is already loaded, resolve with it immediately.
        for loaded_language in state.languages.iter() {
            if loaded_language.id == language.id {
                let _ = tx.send(Ok(loaded_language.clone()));
                return rx;
            }
        }

        match state.loading_languages.entry(language.id) {
            // If the language is already being loaded, then add this
            // channel to a list that will be sent to when the load completes.
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(tx),

            // Otherwise, start loading the language.
            hash_map::Entry::Vacant(entry) => {
                let this = self.clone();
                self.executor
                    .spawn(async move {
                        let id = language.id;
                        let name = language.name.clone();
                        let language = async {
                            let (config, queries, provider) = (language.load)()?;

                            if let Some(grammar) = config.grammar.clone() {
                                let grammar = Some(this.get_or_load_grammar(grammar).await?);
                                Language::new_with_id(id, config, grammar)
                                    .with_context_provider(provider)
                                    .with_queries(queries)
                            } else {
                                Ok(Language::new_with_id(id, config, None)
                                    .with_context_provider(provider))
                            }
                        }
                        .await;

                        match language {
                            Ok(language) => {
                                let language = Arc::new(language);
                                let mut state = this.state.write();

                                state.add(language.clone());
                                state.mark_language_loaded(id);
                                if let Some(mut txs) = state.loading_languages.remove(&id) {
                                    for tx in txs.drain(..) {
                                        let _ = tx.send(Ok(language.clone()));
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("failed to load language {name}:\n{:?}", e);
                                let mut state = this.state.write();
                                state.mark_language_loaded(id);
                                if let Some(mut txs) = state.loading_languages.remove(&id) {
                                    for tx in txs.drain(..) {
                                        let _ = tx.send(Err(anyhow!(
                                            "failed to load language {}: {}",
                                            name,
                                            e
                                        )));
                                    }
                                }
                            }
                        };
                    })
                    .detach();
                entry.insert(vec![tx]);
            }
        }

        rx
    }

    fn get_or_load_grammar(
        self: &Arc<Self>,
        name: Arc<str>,
    ) -> impl Future<Output = Result<tree_sitter::Language>> {
        let (tx, rx) = oneshot::channel();
        let mut state = self.state.write();

        if let Some(grammar) = state.grammars.get_mut(name.as_ref()) {
            match grammar {
                AvailableGrammar::LoadFailed(error) => {
                    tx.send(Err(error.clone())).ok();
                }
                AvailableGrammar::Native(grammar) | AvailableGrammar::Loaded(_, grammar) => {
                    tx.send(Ok(grammar.clone())).ok();
                }
                AvailableGrammar::Loading(_, txs) => {
                    txs.push(tx);
                }
                AvailableGrammar::Unloaded(wasm_path) => {
                    let this = self.clone();
                    let wasm_path = wasm_path.clone();
                    *grammar = AvailableGrammar::Loading(wasm_path.clone(), vec![tx]);
                    self.executor
                        .spawn(async move {
                            let grammar_result = maybe!({
                                let wasm_bytes = std::fs::read(&wasm_path)?;
                                let grammar_name = wasm_path
                                    .file_stem()
                                    .and_then(OsStr::to_str)
                                    .ok_or_else(|| anyhow!("invalid grammar filename"))?;
                                anyhow::Ok(with_parser(|parser| {
                                    let mut store = parser.take_wasm_store().unwrap();
                                    let grammar = store.load_language(&grammar_name, &wasm_bytes);
                                    parser.set_wasm_store(store).unwrap();
                                    grammar
                                })?)
                            })
                            .map_err(Arc::new);

                            let value = match &grammar_result {
                                Ok(grammar) => AvailableGrammar::Loaded(wasm_path, grammar.clone()),
                                Err(error) => AvailableGrammar::LoadFailed(error.clone()),
                            };

                            let old_value = this.state.write().grammars.insert(name, value);
                            if let Some(AvailableGrammar::Loading(_, txs)) = old_value {
                                for tx in txs {
                                    tx.send(grammar_result.clone()).ok();
                                }
                            }
                        })
                        .detach();
                }
            }
        } else {
            tx.send(Err(Arc::new(anyhow!("no such grammar {}", name))))
                .ok();
        }

        async move { rx.await?.map_err(|e| anyhow!(e)) }
    }

    pub fn to_vec(&self) -> Vec<Arc<Language>> {
        self.state.read().languages.iter().cloned().collect()
    }

    pub fn lsp_adapters(&self, language: &Arc<Language>) -> Vec<Arc<CachedLspAdapter>> {
        self.state
            .read()
            .lsp_adapters
            .get(&language.config.name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn update_lsp_status(
        &self,
        server_name: LanguageServerName,
        status: LanguageServerBinaryStatus,
    ) {
        self.lsp_binary_status_tx.send(server_name, status);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_pending_language_server(
        self: &Arc<Self>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        root_path: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cli_environment: Option<HashMap<String, String>>,
        cx: &mut AppContext,
    ) -> Option<PendingLanguageServer> {
        let server_id = self.state.write().next_language_server_id();
        log::info!(
            "starting language server {:?}, path: {root_path:?}, id: {server_id}",
            adapter.name.0
        );

        let download_dir = self
            .language_server_download_dir
            .clone()
            .ok_or_else(|| anyhow!("language server download directory has not been assigned before starting server"))
            .log_err()?;
        let language = language.clone();
        let container_dir: Arc<Path> = Arc::from(download_dir.join(adapter.name.0.as_ref()));
        let root_path = root_path.clone();
        let login_shell_env_loaded = self.login_shell_env_loaded.clone();
        let this = Arc::downgrade(self);

        let task = cx.spawn({
            let container_dir = container_dir.clone();
            move |mut cx| async move {
                // If we want to install a binary globally, we need to wait for
                // the login shell to be set on our process.
                login_shell_env_loaded.await;

                let binary_result = adapter
                    .clone()
                    .get_language_server_command(
                        language.clone(),
                        container_dir,
                        delegate.clone(),
                        &mut cx,
                    )
                    .await;

                delegate.update_status(adapter.name.clone(), LanguageServerBinaryStatus::None);

                let mut binary = binary_result?;

                // If this Zed project was opened from the CLI and the language server command itself
                // doesn't have an environment (which it would have, if it was found in $PATH), then
                // we pass along the CLI environment that we inherited.
                if binary.env.is_none() && cli_environment.is_some() {
                    log::info!(
                        "using CLI environment for language server {:?}, id: {server_id}",
                        adapter.name.0
                    );
                    binary.env = cli_environment.clone();
                }

                let options = adapter
                    .adapter
                    .clone()
                    .initialization_options(&delegate)
                    .await?;

                if let Some(task) = adapter.will_start_server(&delegate, &mut cx) {
                    task.await?;
                }

                #[cfg(any(test, feature = "test-support"))]
                if true {
                    let capabilities = adapter
                        .as_fake()
                        .map(|fake_adapter| fake_adapter.capabilities.clone())
                        .unwrap_or_else(|| lsp::ServerCapabilities {
                            completion_provider: Some(Default::default()),
                            ..Default::default()
                        });

                    let (server, mut fake_server) = lsp::FakeLanguageServer::new(
                        server_id,
                        binary,
                        adapter.name.0.to_string(),
                        capabilities,
                        cx.clone(),
                    );

                    if let Some(fake_adapter) = adapter.as_fake() {
                        if let Some(initializer) = &fake_adapter.initializer {
                            initializer(&mut fake_server);
                        }
                    }

                    cx.background_executor()
                        .spawn(async move {
                            if fake_server
                                .try_receive_notification::<lsp::notification::Initialized>()
                                .await
                                .is_some()
                            {
                                if let Some(this) = this.upgrade() {
                                    if let Some(txs) = this
                                        .state
                                        .write()
                                        .fake_server_txs
                                        .get_mut(language.name().as_ref())
                                    {
                                        for tx in txs {
                                            tx.unbounded_send(fake_server.clone()).ok();
                                        }
                                    }
                                }
                            }
                        })
                        .detach();

                    return Ok((server, options));
                }

                drop(this);
                Ok((
                    lsp::LanguageServer::new(
                        stderr_capture,
                        server_id,
                        binary,
                        &root_path,
                        adapter.code_action_kinds(),
                        cx,
                    )?,
                    options,
                ))
            }
        });

        Some(PendingLanguageServer {
            server_id,
            task,
            container_dir: Some(container_dir),
        })
    }

    pub fn language_server_binary_statuses(
        &self,
    ) -> mpsc::UnboundedReceiver<(LanguageServerName, LanguageServerBinaryStatus)> {
        self.lsp_binary_status_tx.subscribe()
    }

    pub fn delete_server_container(
        &self,
        adapter: Arc<CachedLspAdapter>,
        cx: &mut AppContext,
    ) -> Task<()> {
        log::info!("deleting server container");

        let download_dir = self
            .language_server_download_dir
            .clone()
            .expect("language server download directory has not been assigned before deleting server container");

        cx.spawn(|_| async move {
            let container_dir = download_dir.join(adapter.name.0.as_ref());
            smol::fs::remove_dir_all(container_dir)
                .await
                .context("server container removal")
                .log_err();
        })
    }

    pub fn next_language_server_id(&self) -> LanguageServerId {
        self.state.write().next_language_server_id()
    }
}

impl LanguageRegistryState {
    fn next_language_server_id(&mut self) -> LanguageServerId {
        LanguageServerId(post_inc(&mut self.next_language_server_id))
    }

    fn add(&mut self, language: Arc<Language>) {
        if let Some(theme) = self.theme.as_ref() {
            language.set_theme(theme.syntax());
        }
        self.language_settings.languages.insert(
            language.name(),
            LanguageSettingsContent {
                tab_size: language.config.tab_size,
                hard_tabs: language.config.hard_tabs,
                soft_wrap: language.config.soft_wrap,
                ..Default::default()
            }
            .clone(),
        );
        self.languages.push(language);
        self.version += 1;
        *self.subscription.0.borrow_mut() = ();
    }

    fn reload(&mut self) {
        self.languages.clear();
        self.version += 1;
        self.reload_count += 1;
        for language in &mut self.available_languages {
            language.loaded = false;
        }
        *self.subscription.0.borrow_mut() = ();
    }

    /// Reorders the list of language servers for the given language.
    ///
    /// Uses the provided list of ordered [`CachedLspAdapters`] as the desired order.
    ///
    /// Any existing language servers not present in `ordered_lsp_adapters` will be
    /// appended to the end.
    fn reorder_language_servers(
        &mut self,
        language: &Arc<Language>,
        ordered_lsp_adapters: Vec<Arc<CachedLspAdapter>>,
    ) {
        let Some(lsp_adapters) = self.lsp_adapters.get_mut(&language.config.name) else {
            return;
        };

        let ordered_lsp_adapter_ids = ordered_lsp_adapters
            .iter()
            .map(|lsp_adapter| lsp_adapter.name.clone())
            .collect::<HashSet<_>>();

        let mut new_lsp_adapters = ordered_lsp_adapters;
        for adapter in lsp_adapters.iter() {
            if !ordered_lsp_adapter_ids.contains(&adapter.name) {
                new_lsp_adapters.push(adapter.clone());
            }
        }

        *lsp_adapters = new_lsp_adapters;
    }

    fn remove_languages(
        &mut self,
        languages_to_remove: &[Arc<str>],
        grammars_to_remove: &[Arc<str>],
    ) {
        if languages_to_remove.is_empty() && grammars_to_remove.is_empty() {
            return;
        }

        self.languages
            .retain(|language| !languages_to_remove.contains(&language.name()));
        self.available_languages
            .retain(|language| !languages_to_remove.contains(&language.name));
        self.grammars
            .retain(|name, _| !grammars_to_remove.contains(&name));
        self.version += 1;
        self.reload_count += 1;
        *self.subscription.0.borrow_mut() = ();
    }

    /// Mark the given language as having been loaded, so that the
    /// language registry won't try to load it again.
    fn mark_language_loaded(&mut self, id: LanguageId) {
        for language in &mut self.available_languages {
            if language.id == id {
                language.loaded = true;
                break;
            }
        }
    }
}

impl LspBinaryStatusSender {
    fn subscribe(
        &self,
    ) -> mpsc::UnboundedReceiver<(LanguageServerName, LanguageServerBinaryStatus)> {
        let (tx, rx) = mpsc::unbounded();
        self.txs.lock().push(tx);
        rx
    }

    fn send(&self, name: LanguageServerName, status: LanguageServerBinaryStatus) {
        let mut txs = self.txs.lock();
        txs.retain(|tx| tx.unbounded_send((name.clone(), status.clone())).is_ok());
    }
}
