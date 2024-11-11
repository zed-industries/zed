use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use assistant_slash_command::SlashCommandRegistry;
use context_servers::ContextServerFactoryRegistry;
use extension_host::{extension_lsp_adapter::ExtensionLspAdapter, wasm_host};
use fs::Fs;
use gpui::{AppContext, BackgroundExecutor, Task};
use indexed_docs::{IndexedDocsRegistry, ProviderId};
use language::{LanguageRegistry, LanguageServerBinaryStatus, LoadedLanguage};
use snippet_provider::SnippetRegistry;
use theme::{ThemeRegistry, ThemeSettings};
use ui::SharedString;

use crate::extension_context_server::ExtensionContextServer;
use crate::{extension_indexed_docs_provider, extension_slash_command::ExtensionSlashCommand};

pub struct ConcreteExtensionRegistrationHooks {
    slash_command_registry: Arc<SlashCommandRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
    language_registry: Arc<LanguageRegistry>,
    context_server_factory_registry: Arc<ContextServerFactoryRegistry>,
    executor: BackgroundExecutor,
}

impl ConcreteExtensionRegistrationHooks {
    pub fn new(
        theme_registry: Arc<ThemeRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        indexed_docs_registry: Arc<IndexedDocsRegistry>,
        snippet_registry: Arc<SnippetRegistry>,
        language_registry: Arc<LanguageRegistry>,
        context_server_factory_registry: Arc<ContextServerFactoryRegistry>,
        cx: &AppContext,
    ) -> Arc<dyn extension_host::ExtensionRegistrationHooks> {
        Arc::new(Self {
            theme_registry,
            slash_command_registry,
            indexed_docs_registry,
            snippet_registry,
            language_registry,
            context_server_factory_registry,
            executor: cx.background_executor().clone(),
        })
    }
}

impl extension_host::ExtensionRegistrationHooks for ConcreteExtensionRegistrationHooks {
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

    fn register_context_server(
        &self,
        id: Arc<str>,
        extension: wasm_host::WasmExtension,
        host: Arc<wasm_host::WasmHost>,
    ) {
        self.context_server_factory_registry
            .register_server_factory(
                id.clone(),
                Arc::new({
                    move |cx| {
                        let id = id.clone();
                        let extension = extension.clone();
                        let host = host.clone();
                        cx.spawn(|_cx| async move {
                            let context_server =
                                ExtensionContextServer::new(extension, host, id).await?;

                            anyhow::Ok(Arc::new(context_server) as _)
                        })
                    }
                }),
            );
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
        server_name: lsp::LanguageServerName,
        status: LanguageServerBinaryStatus,
    ) {
        self.language_registry
            .update_lsp_status(server_name, status);
    }

    fn register_lsp_adapter(
        &self,
        language_name: language::LanguageName,
        adapter: ExtensionLspAdapter,
    ) {
        self.language_registry
            .register_lsp_adapter(language_name, Arc::new(adapter));
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

    fn reload_current_theme(&self, cx: &mut AppContext) {
        ThemeSettings::reload_current_theme(cx)
    }

    fn list_theme_names(&self, path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        self.executor.spawn(async move {
            let themes = theme::read_user_theme(&path, fs).await?;
            Ok(themes.themes.into_iter().map(|theme| theme.name).collect())
        })
    }
}
