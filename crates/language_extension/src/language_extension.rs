mod extension_lsp_adapter;

use std::path::PathBuf;
use std::sync::Arc;

use extension::{ExtensionChangeListeners, OnGrammarExtensionChange};
use language::LanguageRegistry;

pub fn init(
    extension_change_listeners: Arc<ExtensionChangeListeners>,
    language_registry: Arc<LanguageRegistry>,
) {
    extension_change_listeners.register_grammar_listener(GrammarExtensionChangeListener {
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
