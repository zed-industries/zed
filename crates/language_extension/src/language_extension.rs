mod extension_lsp_adapter;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionChangeListeners, OnGrammarExtensionChange, OnLanguageExtensionChange};
use language::{LanguageMatcher, LanguageName, LanguageRegistry, LoadedLanguage};

pub fn init(
    extension_change_listeners: Arc<ExtensionChangeListeners>,
    language_registry: Arc<LanguageRegistry>,
) {
    extension_change_listeners.register_grammar_listener(GrammarExtensionChangeListener {
        language_registry: language_registry.clone(),
    });
    extension_change_listeners.register_language_listener(LanguageExtensionChangeListener {
        language_registry: language_registry.clone(),
    });

    extension_lsp_adapter::init(extension_change_listeners, language_registry);
}

struct GrammarExtensionChangeListener {
    language_registry: Arc<LanguageRegistry>,
}

impl OnGrammarExtensionChange for GrammarExtensionChangeListener {
    fn register(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        self.language_registry.register_wasm_grammars(grammars)
    }
}

struct LanguageExtensionChangeListener {
    language_registry: Arc<LanguageRegistry>,
}

impl OnLanguageExtensionChange for LanguageExtensionChangeListener {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    ) {
        self.language_registry
            .register_language(language, grammar, matcher, load);
    }

    fn remove_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        self.language_registry
            .remove_languages(&languages_to_remove, &grammars_to_remove);
    }
}
