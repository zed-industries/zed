use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use language::{LanguageRegistry, LoadedLanguage};

pub struct ConcreteExtensionRegistrationHooks {
    language_registry: Arc<LanguageRegistry>,
}

impl ConcreteExtensionRegistrationHooks {
    pub fn new(
        language_registry: Arc<LanguageRegistry>,
    ) -> Arc<dyn extension_host::ExtensionRegistrationHooks> {
        Arc::new(Self { language_registry })
    }
}

impl extension_host::ExtensionRegistrationHooks for ConcreteExtensionRegistrationHooks {
    fn remove_languages(
        &self,
        languages_to_remove: &[language::LanguageName],
        grammars_to_remove: &[Arc<str>],
    ) {
        self.language_registry
            .remove_languages(&languages_to_remove, &grammars_to_remove);
    }

    fn register_wasm_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        self.language_registry.register_wasm_grammars(grammars)
    }

    fn register_language(
        &self,
        language: language::LanguageName,
        grammar: Option<Arc<str>>,
        matcher: language::LanguageMatcher,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    ) {
        self.language_registry
            .register_language(language, grammar, matcher, load)
    }
}
