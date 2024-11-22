use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use fs::Fs;
use gpui::{AppContext, Global, ReadGlobal, SharedString, Task};
use language::{LanguageMatcher, LanguageName, LanguageServerBinaryStatus, LoadedLanguage};
use lsp::LanguageServerName;
use parking_lot::RwLock;

use crate::{Extension, SlashCommand};

pub trait OnThemeExtensionChange: Send + Sync + 'static {
    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_user_themes(&self, themes: Vec<SharedString>);

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_theme(&self, cx: &mut AppContext);
}

pub trait OnGrammarExtensionChange: Send + Sync + 'static {
    fn register(&self, grammars: Vec<(Arc<str>, PathBuf)>);
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

pub trait OnSnippetExtensionChange: Send + Sync + 'static {
    fn register(&self, path: &PathBuf, snippet_contents: &str) -> Result<()>;
}

pub trait OnSlashCommandExtensionChange: Send + Sync + 'static {
    fn register(&self, extension: Arc<dyn Extension>, command: SlashCommand);
}

pub trait OnContextServerExtensionChange: Send + Sync + 'static {
    fn register(&self, extension: Arc<dyn Extension>, server_id: Arc<str>, cx: &mut AppContext);
}

pub trait OnIndexedDocsProviderExtensionChange: Send + Sync + 'static {
    fn register(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>);
}

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

    pub fn theme_listener(&self) -> Option<Arc<dyn OnThemeExtensionChange>> {
        self.theme_listener.read().clone()
    }

    pub fn register_theme_listener(
        &self,
        listener: impl OnThemeExtensionChange + Send + Sync + 'static,
    ) {
        self.theme_listener.write().replace(Arc::new(listener));
    }

    pub fn grammar_listener(&self) -> Option<Arc<dyn OnGrammarExtensionChange>> {
        self.grammar_listener.read().clone()
    }

    pub fn register_grammar_listener(
        &self,
        listener: impl OnGrammarExtensionChange + Send + Sync + 'static,
    ) {
        self.grammar_listener.write().replace(Arc::new(listener));
    }

    pub fn language_listener(&self) -> Option<Arc<dyn OnLanguageExtensionChange>> {
        self.language_listener.read().clone()
    }

    pub fn register_language_listener(
        &self,
        listener: impl OnLanguageExtensionChange + Send + Sync + 'static,
    ) {
        self.language_listener.write().replace(Arc::new(listener));
    }

    pub fn language_server_listener(&self) -> Option<Arc<dyn OnLanguageServerExtensionChange>> {
        self.language_server_listener.read().clone()
    }

    pub fn register_language_server_listener(
        &self,
        listener: impl OnLanguageServerExtensionChange + Send + Sync + 'static,
    ) {
        self.language_server_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn snippet_listener(&self) -> Option<Arc<dyn OnSnippetExtensionChange>> {
        self.snippet_listener.read().clone()
    }

    pub fn register_snippet_listener(
        &self,
        listener: impl OnSnippetExtensionChange + Send + Sync + 'static,
    ) {
        self.snippet_listener.write().replace(Arc::new(listener));
    }

    pub fn slash_command_listener(&self) -> Option<Arc<dyn OnSlashCommandExtensionChange>> {
        self.slash_command_listener.read().clone()
    }

    pub fn register_slash_command_listener(
        &self,
        listener: impl OnSlashCommandExtensionChange + Send + Sync + 'static,
    ) {
        self.slash_command_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn context_server_listener(&self) -> Option<Arc<dyn OnContextServerExtensionChange>> {
        self.context_server_listener.read().clone()
    }

    pub fn register_context_server_listener(
        &self,
        listener: impl OnContextServerExtensionChange + Send + Sync + 'static,
    ) {
        self.context_server_listener
            .write()
            .replace(Arc::new(listener));
    }

    pub fn indexed_docs_provider_listener(
        &self,
    ) -> Option<Arc<dyn OnIndexedDocsProviderExtensionChange>> {
        self.indexed_docs_provider_listener.read().clone()
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

impl OnLanguageExtensionChange for ExtensionChangeListeners {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    ) {
        let Some(listener) = self.language_listener() else {
            return;
        };

        listener.register_language(language, grammar, matcher, load)
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        let Some(listener) = self.language_listener() else {
            return;
        };

        listener.remove_languages(languages_to_remove, grammars_to_remove)
    }
}
