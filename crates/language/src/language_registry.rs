use crate::{
    CachedLspAdapter, File, Language, LanguageConfig, LanguageId, LanguageMatcher,
    LanguageServerName, LspAdapter, ManifestName, PLAIN_TEXT, ToolchainLister,
    language_settings::all_language_settings, task_context::ContextProvider, with_parser,
};
use anyhow::{Context as _, Result, anyhow};
use collections::{FxHashMap, HashMap, HashSet, hash_map};
use settings::{AllLanguageSettingsContent, LanguageSettingsContent};

use futures::{
    Future,
    channel::{mpsc, oneshot},
};
use globset::GlobSet;
use gpui::{App, BackgroundExecutor, SharedString};
use lsp::LanguageServerId;
use parking_lot::{Mutex, RwLock};
use postage::watch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    borrow::{Borrow, Cow},
    cell::LazyCell,
    ffi::OsStr,
    ops::Not,
    path::{Path, PathBuf},
    sync::Arc,
};
use sum_tree::Bias;
use text::{Point, Rope};
use theme::Theme;
use unicase::UniCase;
use util::{ResultExt, maybe, post_inc};

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct LanguageName(pub SharedString);

impl LanguageName {
    pub fn new(s: &str) -> Self {
        Self(SharedString::new(s))
    }

    pub fn from_proto(s: String) -> Self {
        Self(SharedString::from(s))
    }
    pub fn to_proto(&self) -> String {
        self.0.to_string()
    }
    pub fn lsp_id(&self) -> String {
        match self.0.as_ref() {
            "Plain Text" => "plaintext".to_string(),
            language_name => language_name.to_lowercase(),
        }
    }
}

impl From<LanguageName> for SharedString {
    fn from(value: LanguageName) -> Self {
        value.0
    }
}

impl From<SharedString> for LanguageName {
    fn from(value: SharedString) -> Self {
        LanguageName(value)
    }
}

impl AsRef<str> for LanguageName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for LanguageName {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for LanguageName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for LanguageName {
    fn from(str: &'a str) -> LanguageName {
        LanguageName(SharedString::new(str))
    }
}

impl From<LanguageName> for String {
    fn from(value: LanguageName) -> Self {
        let value: &str = &value.0;
        Self::from(value)
    }
}

pub struct LanguageRegistry {
    state: RwLock<LanguageRegistryState>,
    language_server_download_dir: Option<Arc<Path>>,
    executor: BackgroundExecutor,
    lsp_binary_status_tx: ServerStatusSender,
}

struct LanguageRegistryState {
    next_language_server_id: usize,
    languages: Vec<Arc<Language>>,
    language_settings: AllLanguageSettingsContent,
    available_languages: Vec<AvailableLanguage>,
    grammars: HashMap<Arc<str>, AvailableGrammar>,
    lsp_adapters: HashMap<LanguageName, Vec<Arc<CachedLspAdapter>>>,
    all_lsp_adapters: HashMap<LanguageServerName, Arc<CachedLspAdapter>>,
    available_lsp_adapters:
        HashMap<LanguageServerName, Arc<dyn Fn() -> Arc<CachedLspAdapter> + 'static + Send + Sync>>,
    loading_languages: HashMap<LanguageId, Vec<oneshot::Sender<Result<Arc<Language>>>>>,
    subscription: (watch::Sender<()>, watch::Receiver<()>),
    theme: Option<Arc<Theme>>,
    version: usize,
    reload_count: usize,

    #[cfg(any(test, feature = "test-support"))]
    fake_server_entries: HashMap<LanguageServerName, FakeLanguageServerEntry>,
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeLanguageServerEntry {
    pub capabilities: lsp::ServerCapabilities,
    pub initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
    pub tx: futures::channel::mpsc::UnboundedSender<lsp::FakeLanguageServer>,
    pub _server: Option<lsp::FakeLanguageServer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageServerStatusUpdate {
    Binary(BinaryStatus),
    Health(ServerHealth, Option<SharedString>),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ServerHealth {
    Ok,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Starting,
    Stopping,
    Stopped,
    Failed { error: String },
}

#[derive(Clone)]
pub struct AvailableLanguage {
    id: LanguageId,
    name: LanguageName,
    grammar: Option<Arc<str>>,
    matcher: LanguageMatcher,
    hidden: bool,
    load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    loaded: bool,
    manifest_name: Option<ManifestName>,
}

impl AvailableLanguage {
    pub fn name(&self) -> LanguageName {
        self.name.clone()
    }

    pub fn matcher(&self) -> &LanguageMatcher {
        &self.matcher
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

#[derive(Copy, Clone, Default)]
enum LanguageMatchPrecedence {
    #[default]
    Undetermined,
    PathOrContent(usize),
    UserConfigured(usize),
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
    ("debugger", |q| &mut q.debugger),
    ("textobjects", |q| &mut q.text_objects),
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
    pub text_objects: Option<Cow<'static, str>>,
    pub debugger: Option<Cow<'static, str>>,
}

#[derive(Clone, Default)]
struct ServerStatusSender {
    txs: Arc<Mutex<Vec<mpsc::UnboundedSender<(LanguageServerName, BinaryStatus)>>>>,
}

pub struct LoadedLanguage {
    pub config: LanguageConfig,
    pub queries: LanguageQueries,
    pub context_provider: Option<Arc<dyn ContextProvider>>,
    pub toolchain_provider: Option<Arc<dyn ToolchainLister>>,
    pub manifest_name: Option<ManifestName>,
}

impl LanguageRegistry {
    pub fn new(executor: BackgroundExecutor) -> Self {
        let this = Self {
            state: RwLock::new(LanguageRegistryState {
                next_language_server_id: 0,
                languages: Vec::new(),
                available_languages: Vec::new(),
                grammars: Default::default(),
                language_settings: Default::default(),
                loading_languages: Default::default(),
                lsp_adapters: Default::default(),
                all_lsp_adapters: Default::default(),
                available_lsp_adapters: HashMap::default(),
                subscription: watch::channel(),
                theme: Default::default(),
                version: 0,
                reload_count: 0,

                #[cfg(any(test, feature = "test-support"))]
                fake_server_entries: Default::default(),
            }),
            language_server_download_dir: None,
            lsp_binary_status_tx: Default::default(),
            executor,
        };
        this.add(PLAIN_TEXT.clone());
        this
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(executor: BackgroundExecutor) -> Self {
        let mut this = Self::new(executor);
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
        language: &LanguageName,
        ordered_lsp_adapters: Vec<Arc<CachedLspAdapter>>,
    ) {
        self.state
            .write()
            .reorder_language_servers(language, ordered_lsp_adapters);
    }

    /// Removes the specified languages and grammars from the registry.
    pub fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        self.state
            .write()
            .remove_languages(languages_to_remove, grammars_to_remove)
    }

    pub fn remove_lsp_adapter(&self, language_name: &LanguageName, name: &LanguageServerName) {
        let mut state = self.state.write();
        if let Some(adapters) = state.lsp_adapters.get_mut(language_name) {
            adapters.retain(|adapter| &adapter.name != name)
        }
        state.all_lsp_adapters.remove(name);
        state.available_lsp_adapters.remove(name);

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
            config.hidden,
            None,
            Arc::new(move || {
                Ok(LoadedLanguage {
                    config: config.clone(),
                    queries: Default::default(),
                    toolchain_provider: None,
                    context_provider: None,
                    manifest_name: None,
                })
            }),
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
        adapter: Arc<dyn LspAdapter>,
    ) {
        let mut state = self.state.write();

        if adapter.is_extension()
            && let Some(existing_adapter) = state.all_lsp_adapters.get(&name)
            && !existing_adapter.adapter.is_extension()
        {
            log::warn!(
                "not registering extension-provided language server {name:?}, since a builtin language server exists with that name",
            );
            return;
        }

        state.available_lsp_adapters.insert(
            name,
            Arc::new(move || CachedLspAdapter::new(adapter.clone())),
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

    pub fn register_lsp_adapter(&self, language_name: LanguageName, adapter: Arc<dyn LspAdapter>) {
        let mut state = self.state.write();

        if adapter.is_extension()
            && let Some(existing_adapter) = state.all_lsp_adapters.get(&adapter.name())
            && !existing_adapter.adapter.is_extension()
        {
            log::warn!(
                "not registering extension-provided language server {:?} for language {language_name:?}, since a builtin language server exists with that name",
                adapter.name(),
            );
            return;
        }

        let cached = CachedLspAdapter::new(adapter);
        state
            .lsp_adapters
            .entry(language_name)
            .or_default()
            .push(cached.clone());
        state
            .all_lsp_adapters
            .insert(cached.name.clone(), cached.clone());
    }

    /// Register a fake language server and adapter
    /// The returned channel receives a new instance of the language server every time it is started
    #[cfg(any(feature = "test-support", test))]
    pub fn register_fake_lsp(
        &self,
        language_name: impl Into<LanguageName>,
        mut adapter: crate::FakeLspAdapter,
    ) -> futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let language_name = language_name.into();
        let adapter_name = LanguageServerName(adapter.name.into());
        let capabilities = adapter.capabilities.clone();
        let initializer = adapter.initializer.take();
        let adapter = CachedLspAdapter::new(Arc::new(adapter));
        {
            let mut state = self.state.write();
            state
                .lsp_adapters
                .entry(language_name)
                .or_default()
                .push(adapter.clone());
            state.all_lsp_adapters.insert(adapter.name(), adapter);
        }

        self.register_fake_language_server(adapter_name, capabilities, initializer)
    }

    /// Register a fake lsp adapter (without the language server)
    /// The returned channel receives a new instance of the language server every time it is started
    #[cfg(any(feature = "test-support", test))]
    pub fn register_fake_lsp_adapter(
        &self,
        language_name: impl Into<LanguageName>,
        adapter: crate::FakeLspAdapter,
    ) {
        let language_name = language_name.into();
        let mut state = self.state.write();
        let cached_adapter = CachedLspAdapter::new(Arc::new(adapter));
        state
            .lsp_adapters
            .entry(language_name)
            .or_default()
            .push(cached_adapter.clone());
        state
            .all_lsp_adapters
            .insert(cached_adapter.name(), cached_adapter);
    }

    /// Register a fake language server (without the adapter)
    /// The returned channel receives a new instance of the language server every time it is started
    #[cfg(any(feature = "test-support", test))]
    pub fn register_fake_language_server(
        &self,
        lsp_name: LanguageServerName,
        capabilities: lsp::ServerCapabilities,
        initializer: Option<Box<dyn Fn(&mut lsp::FakeLanguageServer) + Send + Sync>>,
    ) -> futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let (servers_tx, servers_rx) = futures::channel::mpsc::unbounded();
        self.state.write().fake_server_entries.insert(
            lsp_name,
            FakeLanguageServerEntry {
                tx: servers_tx,
                capabilities,
                initializer,
                _server: None,
            },
        );
        servers_rx
    }

    /// Adds a language to the registry, which can be loaded if needed.
    pub fn register_language(
        &self,
        name: LanguageName,
        grammar_name: Option<Arc<str>>,
        matcher: LanguageMatcher,
        hidden: bool,
        manifest_name: Option<ManifestName>,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    ) {
        let state = &mut *self.state.write();

        for existing_language in &mut state.available_languages {
            if existing_language.name == name {
                existing_language.grammar = grammar_name;
                existing_language.matcher = matcher;
                existing_language.load = load;
                existing_language.manifest_name = manifest_name;
                return;
            }
        }

        state.available_languages.push(AvailableLanguage {
            id: LanguageId::new(),
            name,
            grammar: grammar_name,
            matcher,
            load,
            hidden,
            loaded: false,
            manifest_name,
        });
        state.version += 1;
        state.reload_count += 1;
        *state.subscription.0.borrow_mut() = ();
    }

    /// Adds grammars to the registry. Language configurations reference a grammar by name. The
    /// grammar controls how the source code is parsed.
    pub fn register_native_grammars(
        &self,
        grammars: impl IntoIterator<Item = (impl Into<Arc<str>>, impl Into<tree_sitter::Language>)>,
    ) {
        self.state.write().grammars.extend(
            grammars
                .into_iter()
                .map(|(name, grammar)| (name.into(), AvailableGrammar::Native(grammar.into()))),
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

    pub fn language_names(&self) -> Vec<LanguageName> {
        let state = self.state.read();
        let mut result = state
            .available_languages
            .iter()
            .filter_map(|l| l.loaded.not().then_some(l.name.clone()))
            .chain(state.languages.iter().map(|l| l.config.name.clone()))
            .collect::<Vec<_>>();
        result.sort_unstable_by_key(|language_name| language_name.as_ref().to_lowercase());
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
            hidden: language.config.hidden,
            manifest_name: None,
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
    ) -> impl Future<Output = Result<Arc<Language>>> + use<> {
        let name = UniCase::new(name);
        let rx = self.get_or_load_language(|language_name, _, current_best_match| {
            match current_best_match {
                LanguageMatchPrecedence::Undetermined if UniCase::new(&language_name.0) == name => {
                    Some(LanguageMatchPrecedence::PathOrContent(name.len()))
                }
                LanguageMatchPrecedence::Undetermined
                | LanguageMatchPrecedence::UserConfigured(_)
                | LanguageMatchPrecedence::PathOrContent(_) => None,
            }
        });
        async move { rx.await? }
    }

    pub async fn language_for_id(self: &Arc<Self>, id: LanguageId) -> Result<Arc<Language>> {
        let available_language = {
            let state = self.state.read();

            let Some(available_language) = state
                .available_languages
                .iter()
                .find(|lang| lang.id == id)
                .cloned()
            else {
                anyhow::bail!(LanguageNotFound);
            };
            available_language
        };

        self.load_language(&available_language).await?
    }

    pub fn language_name_for_extension(self: &Arc<Self>, extension: &str) -> Option<LanguageName> {
        self.state.try_read().and_then(|state| {
            state
                .available_languages
                .iter()
                .find(|language| {
                    language
                        .matcher()
                        .path_suffixes
                        .iter()
                        .any(|suffix| *suffix == extension)
                })
                .map(|language| language.name.clone())
        })
    }

    pub fn language_for_name_or_extension(
        self: &Arc<Self>,
        string: &str,
    ) -> impl Future<Output = Result<Arc<Language>>> {
        let string = UniCase::new(string);
        let rx = self.get_or_load_language(|name, config, current_best_match| {
            let name_matches = || {
                UniCase::new(&name.0) == string
                    || config
                        .path_suffixes
                        .iter()
                        .any(|suffix| UniCase::new(suffix) == string)
            };

            match current_best_match {
                LanguageMatchPrecedence::Undetermined => {
                    name_matches().then_some(LanguageMatchPrecedence::PathOrContent(string.len()))
                }
                LanguageMatchPrecedence::PathOrContent(len) => (string.len() > len
                    && name_matches())
                .then_some(LanguageMatchPrecedence::PathOrContent(string.len())),
                LanguageMatchPrecedence::UserConfigured(_) => None,
            }
        });
        async move { rx.await? }
    }

    pub fn available_language_for_name(self: &Arc<Self>, name: &str) -> Option<AvailableLanguage> {
        let state = self.state.read();
        state
            .available_languages
            .iter()
            .find(|l| l.name.0.as_ref() == name)
            .cloned()
    }

    pub fn language_for_file(
        self: &Arc<Self>,
        file: &Arc<dyn File>,
        content: Option<&Rope>,
        cx: &App,
    ) -> Option<AvailableLanguage> {
        let user_file_types = all_language_settings(Some(file), cx);

        self.language_for_file_internal(
            &file.full_path(cx),
            content,
            Some(&user_file_types.file_types),
        )
    }

    pub fn language_for_file_path(self: &Arc<Self>, path: &Path) -> Option<AvailableLanguage> {
        self.language_for_file_internal(path, None, None)
    }

    pub fn load_language_for_file_path<'a>(
        self: &Arc<Self>,
        path: &'a Path,
    ) -> impl Future<Output = Result<Arc<Language>>> + 'a {
        let language = self.language_for_file_path(path);

        let this = self.clone();
        async move {
            if let Some(language) = language {
                this.load_language(&language).await?
            } else {
                Err(anyhow!(LanguageNotFound))
            }
        }
    }

    fn language_for_file_internal(
        self: &Arc<Self>,
        path: &Path,
        content: Option<&Rope>,
        user_file_types: Option<&FxHashMap<Arc<str>, GlobSet>>,
    ) -> Option<AvailableLanguage> {
        let filename = path.file_name().and_then(|filename| filename.to_str());
        // `Path.extension()` returns None for files with a leading '.'
        // and no other extension which is not the desired behavior here,
        // as we want `.zshrc` to result in extension being `Some("zshrc")`
        let extension = filename.and_then(|filename| filename.split('.').next_back());
        let path_suffixes = [extension, filename, path.to_str()]
            .iter()
            .filter_map(|suffix| suffix.map(|suffix| (suffix, globset::Candidate::new(suffix))))
            .collect::<SmallVec<[_; 3]>>();
        let content = LazyCell::new(|| {
            content.map(|content| {
                let end = content.clip_point(Point::new(0, 256), Bias::Left);
                let end = content.point_to_offset(end);
                content.chunks_in_range(0..end).collect::<String>()
            })
        });
        self.find_matching_language(move |language_name, config, current_best_match| {
            let path_matches_default_suffix = || {
                let len =
                    config
                        .path_suffixes
                        .iter()
                        .fold(0, |acc: usize, path_suffix: &String| {
                            let ext = ".".to_string() + path_suffix;

                            let matched_suffix_len = path_suffixes
                                .iter()
                                .find(|(suffix, _)| suffix.ends_with(&ext) || suffix == path_suffix)
                                .map(|(suffix, _)| suffix.len());

                            match matched_suffix_len {
                                Some(len) => acc.max(len),
                                None => acc,
                            }
                        });
                (len > 0).then_some(len)
            };

            let path_matches_custom_suffix = || {
                user_file_types
                    .and_then(|types| types.get(language_name.as_ref()))
                    .map_or(None, |custom_suffixes| {
                        path_suffixes
                            .iter()
                            .find(|(_, candidate)| custom_suffixes.is_match_candidate(candidate))
                            .map(|(suffix, _)| suffix.len())
                    })
            };

            let content_matches = || {
                config.first_line_pattern.as_ref().is_some_and(|pattern| {
                    content
                        .as_ref()
                        .is_some_and(|content| pattern.is_match(content))
                })
            };

            // Only return a match for the given file if we have a better match than
            // the current one.
            match current_best_match {
                LanguageMatchPrecedence::PathOrContent(current_len) => {
                    if let Some(len) = path_matches_custom_suffix() {
                        // >= because user config should win tie with system ext len
                        (len >= current_len).then_some(LanguageMatchPrecedence::UserConfigured(len))
                    } else if let Some(len) = path_matches_default_suffix() {
                        // >= because user config should win tie with system ext len
                        (len >= current_len).then_some(LanguageMatchPrecedence::PathOrContent(len))
                    } else {
                        None
                    }
                }
                LanguageMatchPrecedence::Undetermined => {
                    if let Some(len) = path_matches_custom_suffix() {
                        Some(LanguageMatchPrecedence::UserConfigured(len))
                    } else if let Some(len) = path_matches_default_suffix() {
                        Some(LanguageMatchPrecedence::PathOrContent(len))
                    } else if content_matches() {
                        Some(LanguageMatchPrecedence::PathOrContent(1))
                    } else {
                        None
                    }
                }
                LanguageMatchPrecedence::UserConfigured(_) => None,
            }
        })
    }

    fn find_matching_language(
        self: &Arc<Self>,
        callback: impl Fn(
            &LanguageName,
            &LanguageMatcher,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> Option<AvailableLanguage> {
        let state = self.state.read();
        let available_language = state
            .available_languages
            .iter()
            .rev()
            .fold(None, |best_language_match, language| {
                let current_match_type = best_language_match
                    .as_ref()
                    .map_or(LanguageMatchPrecedence::default(), |(_, score)| *score);
                let language_score =
                    callback(&language.name, &language.matcher, current_match_type);

                match (language_score, current_match_type) {
                    // no current best, so our candidate is better
                    (
                        Some(
                            LanguageMatchPrecedence::PathOrContent(_)
                            | LanguageMatchPrecedence::UserConfigured(_),
                        ),
                        LanguageMatchPrecedence::Undetermined,
                    ) => language_score.map(|new_score| (language.clone(), new_score)),

                    // our candidate is better only if the name is longer
                    (
                        Some(LanguageMatchPrecedence::PathOrContent(new_len)),
                        LanguageMatchPrecedence::PathOrContent(current_len),
                    )
                    | (
                        Some(LanguageMatchPrecedence::UserConfigured(new_len)),
                        LanguageMatchPrecedence::UserConfigured(current_len),
                    )
                    | (
                        Some(LanguageMatchPrecedence::PathOrContent(new_len)),
                        LanguageMatchPrecedence::UserConfigured(current_len),
                    ) => {
                        if new_len > current_len {
                            language_score.map(|new_score| (language.clone(), new_score))
                        } else {
                            best_language_match
                        }
                    }

                    // our candidate is better if the name is longer or equal to
                    (
                        Some(LanguageMatchPrecedence::UserConfigured(new_len)),
                        LanguageMatchPrecedence::PathOrContent(current_len),
                    ) => {
                        if new_len >= current_len {
                            language_score.map(|new_score| (language.clone(), new_score))
                        } else {
                            best_language_match
                        }
                    }

                    // no candidate, use current best
                    (None, _) | (Some(LanguageMatchPrecedence::Undetermined), _) => {
                        best_language_match
                    }
                }
            })
            .map(|(available_language, _)| available_language);
        drop(state);
        available_language
    }

    pub fn load_language(
        self: &Arc<Self>,
        language: &AvailableLanguage,
    ) -> oneshot::Receiver<Result<Arc<Language>>> {
        let (tx, rx) = oneshot::channel();

        let mut state = self.state.write();

        // If the language is already loaded, resolve with it immediately.
        for loaded_language in state.languages.iter() {
            if loaded_language.id == language.id {
                tx.send(Ok(loaded_language.clone())).unwrap();
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

                let id = language.id;
                let name = language.name.clone();
                let language_load = language.load.clone();

                self.executor
                    .spawn(async move {
                        let language = async {
                            let loaded_language = (language_load)()?;
                            if let Some(grammar) = loaded_language.config.grammar.clone() {
                                let grammar = Some(this.get_or_load_grammar(grammar).await?);

                                Language::new_with_id(id, loaded_language.config, grammar)
                                    .with_context_provider(loaded_language.context_provider)
                                    .with_toolchain_lister(loaded_language.toolchain_provider)
                                    .with_manifest(loaded_language.manifest_name)
                                    .with_queries(loaded_language.queries)
                            } else {
                                Ok(Language::new_with_id(id, loaded_language.config, None)
                                    .with_context_provider(loaded_language.context_provider)
                                    .with_manifest(loaded_language.manifest_name)
                                    .with_toolchain_lister(loaded_language.toolchain_provider))
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
                                log::error!("failed to load language {name}:\n{e:?}");
                                let mut state = this.state.write();
                                state.mark_language_loaded(id);
                                if let Some(mut txs) = state.loading_languages.remove(&id) {
                                    for tx in txs.drain(..) {
                                        let _ = tx.send(Err(anyhow!(
                                            "failed to load language {name}: {e}",
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

        drop(state);
        rx
    }

    fn get_or_load_language(
        self: &Arc<Self>,
        callback: impl Fn(
            &LanguageName,
            &LanguageMatcher,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> oneshot::Receiver<Result<Arc<Language>>> {
        let Some(language) = self.find_matching_language(callback) else {
            let (tx, rx) = oneshot::channel();
            let _ = tx.send(Err(anyhow!(LanguageNotFound)));
            return rx;
        };

        self.load_language(&language)
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
                    log::trace!("start loading grammar {name:?}");
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
                                    .context("invalid grammar filename")?;
                                anyhow::Ok(with_parser(|parser| {
                                    let mut store = parser.take_wasm_store().unwrap();
                                    let grammar = store.load_language(grammar_name, &wasm_bytes);
                                    parser.set_wasm_store(store).unwrap();
                                    grammar
                                })?)
                            })
                            .map_err(Arc::new);

                            let value = match &grammar_result {
                                Ok(grammar) => AvailableGrammar::Loaded(wasm_path, grammar.clone()),
                                Err(error) => AvailableGrammar::LoadFailed(error.clone()),
                            };

                            log::trace!("finish loading grammar {name:?}");
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
            tx.send(Err(Arc::new(anyhow!("no such grammar {name}"))))
                .ok();
        }

        async move { rx.await?.map_err(|e| anyhow!(e)) }
    }

    pub fn to_vec(&self) -> Vec<Arc<Language>> {
        self.state.read().languages.to_vec()
    }

    pub fn lsp_adapters(&self, language_name: &LanguageName) -> Vec<Arc<CachedLspAdapter>> {
        self.state
            .read()
            .lsp_adapters
            .get(language_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn all_lsp_adapters(&self) -> Vec<Arc<CachedLspAdapter>> {
        self.state
            .read()
            .all_lsp_adapters
            .values()
            .cloned()
            .collect()
    }

    pub fn adapter_for_name(&self, name: &LanguageServerName) -> Option<Arc<CachedLspAdapter>> {
        self.state.read().all_lsp_adapters.get(name).cloned()
    }

    pub fn update_lsp_binary_status(&self, server_name: LanguageServerName, status: BinaryStatus) {
        self.lsp_binary_status_tx.send(server_name, status);
    }

    pub fn next_language_server_id(&self) -> LanguageServerId {
        self.state.write().next_language_server_id()
    }

    pub fn language_server_download_dir(&self, name: &LanguageServerName) -> Option<Arc<Path>> {
        self.language_server_download_dir
            .as_ref()
            .map(|dir| Arc::from(dir.join(name.0.as_ref())))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn create_fake_language_server(
        &self,
        server_id: LanguageServerId,
        name: &LanguageServerName,
        binary: lsp::LanguageServerBinary,
        cx: &mut gpui::AsyncApp,
    ) -> Option<lsp::LanguageServer> {
        use gpui::AppContext as _;

        let mut state = self.state.write();
        let fake_entry = state.fake_server_entries.get_mut(name)?;
        let (server, mut fake_server) = lsp::FakeLanguageServer::new(
            server_id,
            binary,
            name.0.to_string(),
            fake_entry.capabilities.clone(),
            cx,
        );
        fake_entry._server = Some(fake_server.clone());

        if let Some(initializer) = &fake_entry.initializer {
            initializer(&mut fake_server);
        }

        let tx = fake_entry.tx.clone();
        cx.background_spawn(async move {
            if fake_server
                .try_receive_notification::<lsp::notification::Initialized>()
                .await
                .is_some()
            {
                tx.unbounded_send(fake_server.clone()).ok();
            }
        })
        .detach();

        Some(server)
    }

    pub fn language_server_binary_statuses(
        &self,
    ) -> mpsc::UnboundedReceiver<(LanguageServerName, BinaryStatus)> {
        self.lsp_binary_status_tx.subscribe()
    }

    pub async fn delete_server_container(&self, name: LanguageServerName) {
        log::info!("deleting server container");
        let Some(dir) = self.language_server_download_dir(&name) else {
            return;
        };

        smol::fs::remove_dir_all(dir)
            .await
            .context("server container removal")
            .log_err();
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
        self.language_settings.languages.0.insert(
            language.name().0,
            LanguageSettingsContent {
                tab_size: language.config.tab_size,
                hard_tabs: language.config.hard_tabs,
                soft_wrap: language.config.soft_wrap,
                auto_indent_on_paste: language.config.auto_indent_on_paste,
                ..Default::default()
            },
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
        language_name: &LanguageName,
        ordered_lsp_adapters: Vec<Arc<CachedLspAdapter>>,
    ) {
        let Some(lsp_adapters) = self.lsp_adapters.get_mut(language_name) else {
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
        languages_to_remove: &[LanguageName],
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
            .retain(|name, _| !grammars_to_remove.contains(name));
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

impl ServerStatusSender {
    fn subscribe(&self) -> mpsc::UnboundedReceiver<(LanguageServerName, BinaryStatus)> {
        let (tx, rx) = mpsc::unbounded();
        self.txs.lock().push(tx);
        rx
    }

    fn send(&self, name: LanguageServerName, status: BinaryStatus) {
        let mut txs = self.txs.lock();
        txs.retain(|tx| tx.unbounded_send((name.clone(), status.clone())).is_ok());
    }
}
