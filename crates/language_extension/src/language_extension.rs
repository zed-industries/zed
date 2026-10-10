mod extension_lsp_adapter;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionGrammarProxy, ExtensionHostProxy, ExtensionLanguageProxy};
use gpui::{App, Entity, WeakEntity};
use language::{ExtensionLanguagesUpdate, LanguageName, LanguageRegistration, LanguageRegistry};
use project::LspStore;

#[derive(Clone)]
pub enum LspAccess {
    ViaLspStore(WeakEntity<LspStore>),
    ViaWorkspaces(Arc<dyn Fn(&mut App) -> Result<Vec<Entity<LspStore>>> + Send + Sync + 'static>),
    Noop,
}

pub fn init(
    lsp_access: LspAccess,
    extension_host_proxy: Arc<ExtensionHostProxy>,
    language_registry: Arc<LanguageRegistry>,
) {
    let language_server_registry_proxy = LanguageServerRegistryProxy {
        language_registry,
        lsp_access,
    };
    extension_host_proxy.register_grammar_proxy(language_server_registry_proxy.clone());
    extension_host_proxy.register_language_proxy(language_server_registry_proxy.clone());
    extension_host_proxy.register_language_server_proxy(language_server_registry_proxy);
}

#[derive(Clone)]
struct LanguageServerRegistryProxy {
    language_registry: Arc<LanguageRegistry>,
    lsp_access: LspAccess,
}

impl ExtensionGrammarProxy for LanguageServerRegistryProxy {
    #[ztracing::instrument(skip_all)]
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        self.language_registry.register_wasm_grammars(grammars)
    }
}

impl ExtensionLanguageProxy for LanguageServerRegistryProxy {
    #[ztracing::instrument(skip_all)]
    fn update_languages(
        &self,
        languages_to_remove: &[LanguageName],
        grammars_to_remove: &[Arc<str>],
        registrations: Vec<LanguageRegistration>,
    ) -> ExtensionLanguagesUpdate {
        self.language_registry.update_extension_languages(
            languages_to_remove,
            grammars_to_remove,
            registrations,
        )
    }
}
