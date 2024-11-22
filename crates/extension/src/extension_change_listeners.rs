use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use fs::Fs;
use gpui::{AppContext, Global, ReadGlobal, SharedString, Task};
use language::{LanguageMatcher, LanguageName, LanguageServerBinaryStatus, LoadedLanguage};
use lsp::LanguageServerName;
use parking_lot::RwLock;

use crate::{Extension, SlashCommand};

#[derive(Default)]
struct GlobalExtensionChangeListeners(Arc<ExtensionChangeListeners>);

impl Global for GlobalExtensionChangeListeners {}

#[derive(Default)]
pub struct ExtensionChangeListeners {
    theme_listener: RwLock<Option<Arc<dyn OnThemeExtensionChange>>>,
    grammar_listener: RwLock<Option<Arc<dyn OnGrammarExtensionChange>>>,
    language_listener: RwLock<Option<Arc<dyn OnLanguageExtensionChange>>>,
    language_server_listener: RwLock<Option<Arc<dyn OnLanguageServerExtensionChange>>>,
    snippet_listener: RwLock<Option<Arc<dyn OnSnippetExtensionChange>>>,
    slash_command_listener: RwLock<Option<Arc<dyn OnSlashCommandExtensionChange>>>,
    context_server_listener: RwLock<Option<Arc<dyn OnContextServerExtensionChange>>>,
    indexed_docs_provider_listener: RwLock<Option<Arc<dyn OnIndexedDocsProviderExtensionChange>>>,
}

impl ExtensionChangeListeners {
    /// Returns the global [`ExtensionChangeListeners`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalExtensionChangeListeners::global(cx).0.clone()
    }

    /// Returns the global [`ExtensionChangeListeners`].
    ///
    /// Inserts a default [`ExtensionChangeListeners`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalExtensionChangeListeners>()
            .0
            .clone()
    }

    pub fn new() -> Self {
        Self {
            theme_listener: RwLock::default(),
            grammar_listener: RwLock::default(),
            language_listener: RwLock::default(),
            language_server_listener: RwLock::default(),
            snippet_listener: RwLock::default(),
            slash_command_listener: RwLock::default(),
            context_server_listener: RwLock::default(),
            indexed_docs_provider_listener: RwLock::default(),
        }
    }

    pub fn register_theme_listener(
        &self,
        listener: impl OnThemeExtensionChange + Send + Sync + 'static,
    ) {
        self.theme_listener.write().replace(Arc::new(listener));
    }

    pub fn register_grammar_listener(
        &self,
        listener: impl OnGrammarExtensionChange + Send + Sync + 'static,
    ) {
        self.grammar_listener.write().replace(Arc::new(listener));
    }

    pub fn register_language_listener(
        &self,
        listener: impl OnLanguageExtensionChange + Send + Sync + 'static,
    ) {
        self.language_listener.write().replace(Arc::new(listener));
    }

    pub fn register_language_server_listener(
        &self,
        listener: impl OnLanguageServerExtensionChange + Send + Sync + 'static,
    ) {
        self.language_server_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn register_snippet_listener(
        &self,
        listener: impl OnSnippetExtensionChange + Send + Sync + 'static,
    ) {
        self.snippet_listener.write().replace(Arc::new(listener));
    }

    pub fn register_slash_command_listener(
        &self,
        listener: impl OnSlashCommandExtensionChange + Send + Sync + 'static,
    ) {
        self.slash_command_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn register_context_server_listener(
        &self,
        listener: impl OnContextServerExtensionChange + Send + Sync + 'static,
    ) {
        self.context_server_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn register_indexed_docs_provider_listener(
        &self,
        listener: impl OnIndexedDocsProviderExtensionChange + Send + Sync + 'static,
    ) {
        self.indexed_docs_provider_listener
            .write()
            .replace(Arc::new(listener));
    }
}

pub trait OnThemeExtensionChange: Send + Sync + 'static {
    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_user_themes(&self, themes: Vec<SharedString>);

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_theme(&self, cx: &mut AppContext);
}

impl OnThemeExtensionChange for ExtensionChangeListeners {
    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        let Some(listener) = self.theme_listener.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        listener.list_theme_names(theme_path, fs)
    }

    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        let Some(listener) = self.theme_listener.read().clone() else {
            return;
        };

        listener.remove_user_themes(themes)
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let Some(listener) = self.theme_listener.read().clone() else {
            return Task::ready(Ok(()));
        };

        listener.load_user_theme(theme_path, fs)
    }

    fn reload_current_theme(&self, cx: &mut AppContext) {
        let Some(listener) = self.theme_listener.read().clone() else {
            return;
        };

        listener.reload_current_theme(cx)
    }
}

pub trait OnGrammarExtensionChange: Send + Sync + 'static {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>);
}

impl OnGrammarExtensionChange for ExtensionChangeListeners {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        let Some(listener) = self.grammar_listener.read().clone() else {
            return;
        };

        listener.register_grammars(grammars)
    }
}

pub trait OnLanguageExtensionChange: Send + Sync + 'static {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    );

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    );
}

impl OnLanguageExtensionChange for ExtensionChangeListeners {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    ) {
        let Some(listener) = self.language_listener.read().clone() else {
            return;
        };

        listener.register_language(language, grammar, matcher, load)
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        let Some(listener) = self.language_listener.read().clone() else {
            return;
        };

        listener.remove_languages(languages_to_remove, grammars_to_remove)
    }
}

pub trait OnLanguageServerExtensionChange: Send + Sync + 'static {
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
        status: LanguageServerBinaryStatus,
    );
}

impl OnLanguageServerExtensionChange for ExtensionChangeListeners {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        let Some(listener) = self.language_server_listener.read().clone() else {
            return;
        };

        listener.register_language_server(extension, language_server_id, language)
    }

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
    ) {
        let Some(listener) = self.language_server_listener.read().clone() else {
            return;
        };

        listener.remove_language_server(language, language_server_id)
    }

    fn update_language_server_status(
        &self,
        language_server_id: LanguageServerName,
        status: LanguageServerBinaryStatus,
    ) {
        let Some(listener) = self.language_server_listener.read().clone() else {
            return;
        };

        listener.update_language_server_status(language_server_id, status)
    }
}

pub trait OnSnippetExtensionChange: Send + Sync + 'static {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()>;
}

impl OnSnippetExtensionChange for ExtensionChangeListeners {
    fn register_snippet(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        let Some(listener) = self.snippet_listener.read().clone() else {
            return Ok(());
        };

        listener.register_snippet(path, snippet_contents)
    }
}

pub trait OnSlashCommandExtensionChange: Send + Sync + 'static {
    fn register_slash_command(&self, extension: Arc<dyn Extension>, command: SlashCommand);
}

impl OnSlashCommandExtensionChange for ExtensionChangeListeners {
    fn register_slash_command(&self, extension: Arc<dyn Extension>, command: SlashCommand) {
        let Some(listener) = self.slash_command_listener.read().clone() else {
            return;
        };

        listener.register_slash_command(extension, command)
    }
}

pub trait OnContextServerExtensionChange: Send + Sync + 'static {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut AppContext,
    );
}

impl OnContextServerExtensionChange for ExtensionChangeListeners {
    fn register_context_server(
        &self,
        extension: Arc<dyn Extension>,
        server_id: Arc<str>,
        cx: &mut AppContext,
    ) {
        let Some(listener) = self.context_server_listener.read().clone() else {
            return;
        };

        listener.register_context_server(extension, server_id, cx)
    }
}

pub trait OnIndexedDocsProviderExtensionChange: Send + Sync + 'static {
    fn register_indexed_docs_provider(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>);
}

impl OnIndexedDocsProviderExtensionChange for ExtensionChangeListeners {
    fn register_indexed_docs_provider(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>) {
        let Some(listener) = self.indexed_docs_provider_listener.read().clone() else {
            return;
        };

        listener.register_indexed_docs_provider(extension, provider_id)
    }
}
