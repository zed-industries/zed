use crate::{
    CachedLspAdapter, Language, LanguageConfig, LanguageContextProvider, LanguageId,
    LanguageMatcher, LanguageServerName, LspAdapter, LspAdapterDelegate, PARSER, PLAIN_TEXT,
};
use anyhow::{anyhow, Context as _, Result};
use collections::{hash_map, HashMap};
use futures::{
    channel::{mpsc, oneshot},
    future::Shared,
    Future, FutureExt as _,
};
use gpui::{AppContext, BackgroundExecutor, Task};
use lsp::{LanguageServerBinary, LanguageServerId};
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
use util::{paths::PathExt, post_inc, ResultExt};

pub struct LanguageRegistry {
    state: RwLock<LanguageRegistryState>,
    language_server_download_dir: Option<Arc<Path>>,
    login_shell_env_loaded: Shared<Task<()>>,
    #[allow(clippy::type_complexity)]
    lsp_binary_paths: Mutex<
        HashMap<LanguageServerName, Shared<Task<Result<LanguageServerBinary, Arc<anyhow::Error>>>>>,
    >,
    executor: Option<BackgroundExecutor>,
    lsp_binary_status_tx: LspBinaryStatusSender,
}

struct LanguageRegistryState {
    next_language_server_id: usize,
    languages: Vec<Arc<Language>>,
    available_languages: Vec<AvailableLanguage>,
    grammars: HashMap<Arc<str>, AvailableGrammar>,
    lsp_adapters: HashMap<Arc<str>, Vec<Arc<CachedLspAdapter>>>,
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
    CheckingForUpdate,
    Downloading,
    Downloaded,
    Cached,
    Failed { error: String },
}

pub struct PendingLanguageServer {
    pub server_id: LanguageServerId,
    pub task: Task<Result<lsp::LanguageServer>>,
    pub container_dir: Option<Arc<Path>>,
}

#[derive(Clone)]
struct AvailableLanguage {
    id: LanguageId,
    name: Arc<str>,
    grammar: Option<Arc<str>>,
    matcher: LanguageMatcher,
    load: Arc<dyn Fn() -> Result<(LanguageConfig, LanguageQueries)> + 'static + Send + Sync>,
    loaded: bool,
    context_provider: Option<Arc<dyn LanguageContextProvider>>,
}

enum AvailableGrammar {
    Native(tree_sitter::Language),
    Loaded(#[allow(dead_code)] PathBuf, tree_sitter::Language),
    Loading(PathBuf, Vec<oneshot::Sender<Result<tree_sitter::Language>>>),
    Unloaded(PathBuf),
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
}

#[derive(Clone, Default)]
struct LspBinaryStatusSender {
    txs: Arc<Mutex<Vec<mpsc::UnboundedSender<(LanguageServerName, LanguageServerBinaryStatus)>>>>,
}

impl LanguageRegistry {
    pub fn new(login_shell_env_loaded: Task<()>) -> Self {
        Self {
            state: RwLock::new(LanguageRegistryState {
                next_language_server_id: 0,
                languages: vec![PLAIN_TEXT.clone()],
                available_languages: Default::default(),
                grammars: Default::default(),
                loading_languages: Default::default(),
                lsp_adapters: Default::default(),
                subscription: watch::channel(),
                theme: Default::default(),
                version: 0,
                reload_count: 0,

                #[cfg(any(test, feature = "test-support"))]
                fake_server_txs: Default::default(),
            }),
            language_server_download_dir: None,
            login_shell_env_loaded: login_shell_env_loaded.shared(),
            lsp_binary_paths: Default::default(),
            executor: None,
            lsp_binary_status_tx: Default::default(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        let mut this = Self::new(Task::ready(()));
        this.language_server_download_dir = Some(Path::new("/the-download-dir").into());
        this
    }

    pub fn set_executor(&mut self, executor: BackgroundExecutor) {
        self.executor = Some(executor);
    }

    /// Clears out all of the loaded languages and reload them from scratch.
    pub fn reload(&self) {
        self.state.write().reload();
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
            None,
            move || Ok((config.clone(), Default::default())),
        )
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
        context_provider: Option<Arc<dyn LanguageContextProvider>>,
        load: impl Fn() -> Result<(LanguageConfig, LanguageQueries)> + 'static + Send + Sync,
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

            context_provider,
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

    pub fn add(&self, language: Arc<Language>) {
        self.state.write().add(language);
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
        let rx = self.get_or_load_language(|language_name, _| UniCase::new(language_name) == name);
        async move { rx.await? }
    }

    pub fn language_for_name_or_extension(
        self: &Arc<Self>,
        string: &str,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let string = UniCase::new(string);
        let rx = self.get_or_load_language(|name, config| {
            UniCase::new(name) == string
                || config
                    .path_suffixes
                    .iter()
                    .any(|suffix| UniCase::new(suffix) == string)
        });
        async move { rx.await? }
    }

    pub fn language_for_file(
        self: &Arc<Self>,
        path: &Path,
        content: Option<&Rope>,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension_or_hidden_file_name();
        let path_suffixes = [extension, filename];
        let rx = self.get_or_load_language(move |_, config| {
            let path_matches = config
                .path_suffixes
                .iter()
                .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())));
            let content_matches = content.zip(config.first_line_pattern.as_ref()).map_or(
                false,
                |(content, pattern)| {
                    let end = content.clip_point(Point::new(0, 256), Bias::Left);
                    let end = content.point_to_offset(end);
                    let text = content.chunks_in_range(0..end).collect::<String>();
                    pattern.is_match(&text)
                },
            );
            path_matches || content_matches
        });
        async move { rx.await? }
    }

    fn get_or_load_language(
        self: &Arc<Self>,
        callback: impl Fn(&str, &LanguageMatcher) -> bool,
    ) -> oneshot::Receiver<Result<Arc<Language>>> {
        let (tx, rx) = oneshot::channel();

        let mut state = self.state.write();
        if let Some(language) = state
            .languages
            .iter()
            .find(|language| callback(language.config.name.as_ref(), &language.config.matcher))
        {
            let _ = tx.send(Ok(language.clone()));
        } else if let Some(executor) = self.executor.clone() {
            if let Some(language) = state
                .available_languages
                .iter()
                .rfind(|l| !l.loaded && callback(&l.name, &l.matcher))
                .cloned()
            {
                match state.loading_languages.entry(language.id) {
                    hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(tx),
                    hash_map::Entry::Vacant(entry) => {
                        let this = self.clone();
                        executor
                            .spawn(async move {
                                let id = language.id;
                                let name = language.name.clone();
                                let provider = language.context_provider.clone();
                                let language = async {
                                    let (config, queries) = (language.load)()?;

                                    let grammar = if let Some(grammar) = config.grammar.clone() {
                                        Some(this.get_or_load_grammar(grammar).await?)
                                    } else {
                                        None
                                    };

                                    Language::new_with_id(id, config, grammar)
                                        .with_context_provider(provider)
                                        .with_queries(queries)
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
            } else {
                let _ = tx.send(Err(anyhow!("language not found")));
            }
        } else {
            let _ = tx.send(Err(anyhow!("executor does not exist")));
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
                AvailableGrammar::Native(grammar) | AvailableGrammar::Loaded(_, grammar) => {
                    tx.send(Ok(grammar.clone())).ok();
                }
                AvailableGrammar::Loading(_, txs) => {
                    txs.push(tx);
                }
                AvailableGrammar::Unloaded(wasm_path) => {
                    if let Some(executor) = &self.executor {
                        let this = self.clone();
                        executor
                            .spawn({
                                let wasm_path = wasm_path.clone();
                                async move {
                                    let wasm_bytes = std::fs::read(&wasm_path)?;
                                    let grammar_name = wasm_path
                                        .file_stem()
                                        .and_then(OsStr::to_str)
                                        .ok_or_else(|| anyhow!("invalid grammar filename"))?;
                                    let grammar = PARSER.with(|parser| {
                                        let mut parser = parser.borrow_mut();
                                        let mut store = parser.take_wasm_store().unwrap();
                                        let grammar =
                                            store.load_language(&grammar_name, &wasm_bytes);
                                        parser.set_wasm_store(store).unwrap();
                                        grammar
                                    })?;

                                    if let Some(AvailableGrammar::Loading(_, txs)) =
                                        this.state.write().grammars.insert(
                                            name,
                                            AvailableGrammar::Loaded(wasm_path, grammar.clone()),
                                        )
                                    {
                                        for tx in txs {
                                            tx.send(Ok(grammar.clone())).ok();
                                        }
                                    }

                                    anyhow::Ok(())
                                }
                            })
                            .detach();
                        *grammar = AvailableGrammar::Loading(wasm_path.clone(), vec![tx]);
                    }
                }
            }
        } else {
            tx.send(Err(anyhow!("no such grammar {}", name))).ok();
        }

        async move { rx.await? }
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

    pub fn create_pending_language_server(
        self: &Arc<Self>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        root_path: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
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

                let binary = adapter
                    .clone()
                    .get_language_server_command(
                        language.clone(),
                        container_dir,
                        delegate.clone(),
                        &mut cx,
                    )
                    .await?;

                if let Some(task) = adapter.will_start_server(&delegate, &mut cx) {
                    task.await?;
                }

                #[cfg(any(test, feature = "test-support"))]
                if true {
                    let capabilities = adapter
                        .as_fake()
                        .map(|fake_adapter| fake_adapter.capabilities.clone())
                        .unwrap_or_default();

                    let (server, mut fake_server) = lsp::FakeLanguageServer::new(
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

                    return Ok(server);
                }

                drop(this);
                lsp::LanguageServer::new(
                    stderr_capture,
                    server_id,
                    binary,
                    &root_path,
                    adapter.code_action_kinds(),
                    cx,
                )
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

        let mut lock = self.lsp_binary_paths.lock();
        lock.remove(&adapter.name);

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

#[cfg(any(test, feature = "test-support"))]
impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::test()
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
