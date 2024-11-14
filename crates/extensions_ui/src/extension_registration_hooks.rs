use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use assistant_slash_command::{ExtensionSlashCommand, SlashCommandRegistry};
use context_servers::manager::ServerCommand;
use context_servers::ContextServerFactoryRegistry;
use db::smol::future::FutureExt as _;
use extension::Extension;
use extension_host::wasm_host::ExtensionProject;
use extension_host::{extension_lsp_adapter::ExtensionLspAdapter, wasm_host};
use fs::Fs;
use gpui::{AppContext, BackgroundExecutor, Model, Task};
use indexed_docs::{ExtensionIndexedDocsProvider, IndexedDocsRegistry, ProviderId};
use language::{LanguageRegistry, LanguageServerBinaryStatus, LoadedLanguage};
use snippet_provider::SnippetRegistry;
use theme::{ThemeRegistry, ThemeSettings};
use ui::SharedString;
use wasmtime_wasi::WasiView as _;

pub struct ConcreteExtensionRegistrationHooks {
    slash_command_registry: Arc<SlashCommandRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
    language_registry: Arc<LanguageRegistry>,
    context_server_factory_registry: Model<ContextServerFactoryRegistry>,
    executor: BackgroundExecutor,
}

impl ConcreteExtensionRegistrationHooks {
    pub fn new(
        theme_registry: Arc<ThemeRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        indexed_docs_registry: Arc<IndexedDocsRegistry>,
        snippet_registry: Arc<SnippetRegistry>,
        language_registry: Arc<LanguageRegistry>,
        context_server_factory_registry: Model<ContextServerFactoryRegistry>,
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
        extension: Arc<dyn Extension>,
        command: extension::SlashCommand,
    ) {
        self.slash_command_registry
            .register_command(ExtensionSlashCommand::new(extension, command), false)
    }

    fn register_context_server(
        &self,
        id: Arc<str>,
        extension: wasm_host::WasmExtension,
        cx: &mut AppContext,
    ) {
        self.context_server_factory_registry
            .update(cx, |registry, _| {
                registry.register_server_factory(
                    id.clone(),
                    Arc::new({
                        move |project, cx| {
                            log::info!(
                                "loading command for context server {id} from extension {}",
                                extension.manifest.id
                            );

                            let id = id.clone();
                            let extension = extension.clone();
                            cx.spawn(|mut cx| async move {
                                let extension_project =
                                    project.update(&mut cx, |project, cx| ExtensionProject {
                                        worktree_ids: project
                                            .visible_worktrees(cx)
                                            .map(|worktree| worktree.read(cx).id().to_proto())
                                            .collect(),
                                    })?;

                                let command = extension
                                    .call({
                                        let id = id.clone();
                                        |extension, store| {
                                            async move {
                                                let project = store
                                                    .data_mut()
                                                    .table()
                                                    .push(extension_project)?;
                                                let command = extension
                                                    .call_context_server_command(
                                                        store,
                                                        id.clone(),
                                                        project,
                                                    )
                                                    .await?
                                                    .map_err(|e| anyhow!("{}", e))?;
                                                anyhow::Ok(command)
                                            }
                                            .boxed()
                                        }
                                    })
                                    .await?;

                                log::info!("loaded command for context server {id}: {command:?}");

                                Ok(ServerCommand {
                                    path: command.command,
                                    args: command.args,
                                    env: Some(command.env.into_iter().collect()),
                                })
                            })
                        }
                    }),
                )
            });
    }

    fn register_docs_provider(&self, extension: Arc<dyn Extension>, provider_id: Arc<str>) {
        self.indexed_docs_registry
            .register_provider(Box::new(ExtensionIndexedDocsProvider::new(
                extension,
                ProviderId(provider_id),
            )));
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
