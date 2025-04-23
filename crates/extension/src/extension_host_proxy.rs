use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use fs::Fs;
use gpui::{App, Global, ReadGlobal, SharedString, Task};
use language::{BinaryStatus, LanguageMatcher, LanguageName, LoadedLanguage};
use lsp::LanguageServerName;
use parking_lot::RwLock;

use crate::{Extension, SlashCommand};

#[derive(Default)]
struct GlobalExtensionHostProxy(Arc<ExtensionHostProxy>);

impl Global for GlobalExtensionHostProxy {}

/// A proxy for interacting with the extension host.
///
/// This object implements each of the individual proxy types so that their
/// methods can be called directly on it.
#[derive(Default)]
pub struct ExtensionHostProxy {
    theme_proxy: RwLock<Option<Arc<dyn ExtensionThemeProxy>>>,
    grammar_proxy: RwLock<Option<Arc<dyn ExtensionGrammarProxy>>>,
    language_proxy: RwLock<Option<Arc<dyn ExtensionLanguageProxy>>>,
    language_server_proxy: RwLock<Option<Arc<dyn ExtensionLanguageServerProxy>>>,
    snippet_proxy: RwLock<Option<Arc<dyn ExtensionSnippetProxy>>>,
    slash_command_proxy: RwLock<Option<Arc<dyn ExtensionSlashCommandProxy>>>,
    context_server_proxy: RwLock<Option<Arc<dyn ExtensionContextServerProxy>>>,
    indexed_docs_provider_proxy: RwLock<Option<Arc<dyn ExtensionIndexedDocsProviderProxy>>>,
}

impl ExtensionHostProxy {
    /// Returns the global [`ExtensionHostProxy`].
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalExtensionHostProxy::global(cx).0.clone()
    }

    /// Returns the global [`ExtensionHostProxy`].
    ///
    /// Inserts a default [`ExtensionHostProxy`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalExtensionHostProxy>().0.clone()
    }

    pub fn new() -> Self {
        Self {
            theme_proxy: RwLock::default(),
            grammar_proxy: RwLock::default(),
            language_proxy: RwLock::default(),
            language_server_proxy: RwLock::default(),
            snippet_proxy: RwLock::default(),
            slash_command_proxy: RwLock::default(),
            context_server_proxy: RwLock::default(),
            indexed_docs_provider_proxy: RwLock::default(),
        }
    }

    pub fn register_theme_proxy(&self, proxy: impl ExtensionThemeProxy) {
        self.theme_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_grammar_proxy(&self, proxy: impl ExtensionGrammarProxy) {
        self.grammar_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_proxy(&self, proxy: impl ExtensionLanguageProxy) {
        self.language_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_server_proxy(&self, proxy: impl ExtensionLanguageServerProxy) {
        self.language_server_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_snippet_proxy(&self, proxy: impl ExtensionSnippetProxy) {
        self.snippet_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_slash_command_proxy(&self, proxy: impl ExtensionSlashCommandProxy) {
        self.slash_command_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_context_server_proxy(&self, proxy: impl ExtensionContextServerProxy) {
        self.context_server_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_indexed_docs_provider_proxy(
        &self,
        proxy: impl ExtensionIndexedDocsProviderProxy,
    ) {
        self.indexed_docs_provider_proxy
            .write()
            .replace(Arc::new(proxy));
    }
}

pub trait ExtensionThemeProxy: Send + Sync + 'static {
    fn set_extensions_loaded(&self);

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_user_themes(&self, themes: Vec<SharedString>);

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_theme(&self, cx: &mut App);

    fn list_icon_theme_names(
        &self,
        icon_theme_path: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<Vec<String>>>;

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>);

    fn load_icon_theme(
        &self,
        icon_theme_path: PathBuf,
        icons_root_dir: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<()>>;

    fn reload_current_icon_theme(&self, cx: &mut App);
}

impl ExtensionThemeProxy for ExtensionHostProxy {
    fn set_extensions_loaded(&self) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.set_extensions_loaded()
    }

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_theme_names(theme_path, fs)
    }

    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_user_themes(themes)
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_user_theme(theme_path, fs)
    }

    fn reload_current_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_theme(cx)
    }

    fn list_icon_theme_names(
        &self,
        icon_theme_path: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_icon_theme_names(icon_theme_path, fs)
    }

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_icon_themes(icon_themes)
    }

    fn load_icon_theme(
        &self,
        icon_theme_path: PathBuf,
        icons_root_dir: PathBuf,
        fs: Arc<dyn Fs>,
    ) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_icon_theme(icon_theme_path, icons_root_dir, fs)
    }

    fn reload_current_icon_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_icon_theme(cx)
    }
}

pub trait ExtensionGrammarProxy: Send + Sync + 'static {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>);
}

impl ExtensionGrammarProxy for ExtensionHostProxy {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        let Some(proxy) = self.grammar_proxy.read().clone() else {
            return;
        };

        proxy.register_grammars(grammars)
    }
}

pub trait ExtensionLanguageProxy: Send + Sync + 'static {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        hidden: bool,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    );

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    );
}

impl ExtensionLanguageProxy for ExtensionHostProxy {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        hidden: bool,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    ) {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return;
        };

        proxy.register_language(language, grammar, matcher, hidden, load)
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return;
        };

        proxy.remove_languages(languages_to_remove, grammars_to_remove)
    }
}

pub trait ExtensionLanguageServerProxy: Send + Sync + 'static {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    );

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
    );

    fn update_language_server_status(
        &self,
        language_server_id: LanguageServerName,
        status: BinaryStatus,
    );
}

impl ExtensionLanguageServerProxy for ExtensionHostProxy {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.register_language_server(extension, language_server_id, language)
    }

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.remove_language_server(language, language_server_id)
    }

    fn update_language_server_status(
        &self,
        language_server_id: LanguageServerName,
        status: BinaryStatus,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.update_language_server_status(language_server_id, status)
    }
}

pub trait ExtensionSnippetProxy: Send + Sync + 'static {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()>;
}

impl ExtensionSnippetProxy for ExtensionHostProxy {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        let Some(proxy) = self.snippet_proxy.read().clone() else {
            return Ok(());
        };

        proxy.register_snippet(path, snippet_contents)
    }
}

pub trait ExtensionSlashCommandProxy: Send + Sync + 'static {
    fn register_slash_command(&self, extension: Arc<dyn Extension>, command: SlashCommand);
}

impl ExtensionSlashCommandProxy for ExtensionHostProxy {
    fn register_slash_command(&self, extension: Arc<dyn Extension>, command: SlashCommand) {
        let Some(proxy) = self.slash_command_proxy.read().clone() else {
            return;
        };

        proxy.register_slash_command(extension, command)
    }
}

pub trait ExtensionContextServerProxy: Send + Sync + 'static {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut App,
    );
}

impl ExtensionContextServerProxy for ExtensionHostProxy {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut App,
    ) {
        let Some(proxy) = self.context_server_proxy.read().clone() else {
            return;
        };

        proxy.register_context_server(extension, server_id, cx)
    }
}

pub trait ExtensionIndexedDocsProviderProxy: Send + Sync + 'static {
    fn register_indexed_docs_provider(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>);
}

impl ExtensionIndexedDocsProviderProxy for ExtensionHostProxy {
    fn register_indexed_docs_provider(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>) {
        let Some(proxy) = self.indexed_docs_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_indexed_docs_provider(extension, provider_id)
    }
}
