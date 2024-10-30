use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use assistant_slash_command::SlashCommandRegistry;
use extension::wasm_host;
use gpui::{AppContext, BackgroundExecutor, Task};
use indexed_docs::{IndexedDocsRegistry, ProviderId};
use language::LanguageRegistry;
use snippet_provider::SnippetRegistry;
use theme::ThemeRegistry;
use ui::SharedString;

use crate::{extension_indexed_docs_provider, extension_slash_command::ExtensionSlashCommand};

pub struct ExtensionApi {
    slash_command_registry: Arc<SlashCommandRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
    language_registry: Arc<LanguageRegistry>,
    executor: BackgroundExecutor,
}

impl ExtensionApi {
    pub fn new(
        theme_registry: Arc<ThemeRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        indexed_docs_registry: Arc<IndexedDocsRegistry>,
        snippet_registry: Arc<SnippetRegistry>,
        language_registry: Arc<LanguageRegistry>,
        cx: &AppContext,
    ) -> Arc<dyn extension::ExtensionApi> {
        Arc::new(Self {
            theme_registry,
            slash_command_registry,
            indexed_docs_registry,
            snippet_registry,
            language_registry,
            executor: cx.background_executor().clone(),
        })
    }
}

impl extension::ExtensionApi for ExtensionApi {
    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        self.theme_registry.remove_user_themes(&themes);
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn fs::Fs>) -> Task<Result<()>> {
        let theme_registry = self.theme_registry.clone();
        self.executor
            .spawn(async move { theme_registry.load_user_theme(&theme_path, fs).await })
    }

    fn register_slash_command(
        &self,
        command: wasm_host::SlashCommand,
        extension: wasm_host::WasmExtension,
        host: Arc<wasm_host::WasmHost>,
    ) {
        self.slash_command_registry.register_command(
            ExtensionSlashCommand {
                command,
                extension,
                host,
            },
            false,
        )
    }

    fn register_docs_provider(
        &self,
        extension: wasm_host::WasmExtension,
        host: Arc<wasm_host::WasmHost>,
        provider_id: Arc<str>,
    ) {
        self.indexed_docs_registry.register_provider(Box::new(
            extension_indexed_docs_provider::ExtensionIndexedDocsProvider {
                extension,
                host,
                id: ProviderId(provider_id),
            },
        ));
    }

    fn register_snippets(&self, path: &PathBuf, snippet_contents: &str) -> Result<()> {
        self.snippet_registry
            .register_snippets(path, snippet_contents)
    }

    fn update_lsp_status(
        &self,
        server_name: language::LanguageServerName,
        status: LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status);
    }

    fn register_lsp_adapter(&self, language: language::LanguageName, adapter: ExtensionLspAdapter) {
        self.language_registry
            .register_lsp_adapter(language_name, Arc::new(adapter))
    }

    fn remove_lsp_adapter(
        &self,
        language: &language::LanguageName,
        server_name: &language::LanguageServerName,
    ) {
        self.language_registry
            .remove_lsp_adapter(language_name, name);
    }

    fn remove_languages(
        &self,
        languages_to_remove: Vec<language::LanguageName>,
        grammars_to_remove: Vec<Arc<str>>,
    ) {
        self.language_registry
            .remove_languages(languages_to_remove, grammars_to_remove);
    }
}
