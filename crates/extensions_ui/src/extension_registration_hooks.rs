use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use assistant_slash_command::{ExtensionSlashCommand, SlashCommandRegistry};
use extension::Extension;
use extension_host::extension_lsp_adapter::ExtensionLspAdapter;
use language::{LanguageName, LanguageRegistry, LanguageServerBinaryStatus, LoadedLanguage};
use lsp::LanguageServerName;
use snippet_provider::SnippetRegistry;

pub struct ConcreteExtensionRegistrationHooks {
    slash_command_registry: Arc<SlashCommandRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
    language_registry: Arc<LanguageRegistry>,
}

impl ConcreteExtensionRegistrationHooks {
    pub fn new(
        slash_command_registry: Arc<SlashCommandRegistry>,
        snippet_registry: Arc<SnippetRegistry>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Arc<dyn extension_host::ExtensionRegistrationHooks> {
        Arc::new(Self {
            slash_command_registry,
            snippet_registry,
            language_registry,
        })
    }
}

impl extension_host::ExtensionRegistrationHooks for ConcreteExtensionRegistrationHooks {
    fn register_slash_command(
        &self,
        extension: Arc<dyn Extension>,
        command: extension::SlashCommand,
    ) {
        self.slash_command_registry
            .register_command(ExtensionSlashCommand::new(extension, command), false)
    }

    fn register_snippets(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        self.snippet_registry
            .register_snippets(path, snippet_contents)
    }

    fn update_lsp_status(
        &self,
        server_name: lsp::LanguageServerName,
        status: LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status);
    }

    fn register_lsp_adapter(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        self.language_registry.register_lsp_adapter(
            language.clone(),
            Arc::new(ExtensionLspAdapter::new(
                extension,
                language_server_id,
                language,
            )),
        );
    }

    fn remove_lsp_adapter(
        &self,
        language_name: &language::LanguageName,
        server_name: &lsp::LanguageServerName,
    ) {
        self.language_registry
            .remove_lsp_adapter(language_name, server_name);
    }

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
