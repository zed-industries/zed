pub mod extension_builder;
mod extension_indexed_docs_provider;
mod extension_slash_command;
mod wasm_host;

#[cfg(test)]
mod extension_store_test;

use crate::extension_indexed_docs_provider::ExtensionIndexedDocsProvider;
use crate::extension_manifest::SchemaVersion;
use crate::extension_slash_command::ExtensionSlashCommand;
use crate::{extension_lsp_adapter::ExtensionLspAdapter, wasm_host::wit};
use anyhow::{anyhow, bail, Context as _, Result};
use assistant_slash_command::SlashCommandRegistry;
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use client::{telemetry::Telemetry, Client, ExtensionMetadata, GetExtensionsResponse};
use collections::{btree_map, BTreeMap, HashSet};
use extension_builder::{CompileExtensionOptions, ExtensionBuilder};
use extension_headless::HeadlessExtensionStore;
use fs::{Fs, RemoveOptions};
use futures::{
    channel::{
        mpsc::{unbounded, UnboundedSender},
        oneshot,
    },
    io::BufReader,
    select_biased, AsyncReadExt as _, Future, FutureExt as _, StreamExt as _,
};
use gpui::{
    actions, AppContext, AsyncAppContext, Context, EventEmitter, Global, Model, ModelContext, Task,
    WeakModel,
};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl};
use indexed_docs::{IndexedDocsRegistry, ProviderId};
use language::{
    LanguageConfig, LanguageMatcher, LanguageName, LanguageQueries, LanguageRegistry,
    LoadedLanguage, QUERY_FILENAME_PREFIXES,
};
use node_runtime::NodeRuntime;
use project::ContextProviderWithTasks;
use release_channel::ReleaseChannel;
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use settings::Settings;
use snippet_provider::SnippetRegistry;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{
    cmp::Ordering,
    path::{self, Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use theme::{ThemeRegistry, ThemeSettings};
use url::Url;
use util::{maybe, ResultExt};
use wasm_host::{
    wit::{is_supported_wasm_api_version, wasm_api_version_range},
    WasmExtension, WasmHost,
};

pub use extension_manifest::{
    ExtensionLibraryKind, ExtensionManifest, GrammarManifestEntry, OldExtensionManifest,
};
pub use extension_settings::ExtensionSettings;

pub struct ExtensionStore {
    headless_store: Model<HeadlessExtensionStore>,
    builder: Arc<ExtensionBuilder>,
    theme_registry: Arc<ThemeRegistry>,
    slash_command_registry: Arc<SlashCommandRegistry>,
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
}

#[derive(Clone, Copy)]
pub enum ExtensionOperation {
    Upgrade,
    Install,
    Remove,
}

#[derive(Clone)]
pub enum Event {
    ExtensionsUpdated,
    StartedReloading,
    ExtensionInstalled(Arc<str>),
    ExtensionFailedToLoad(Arc<str>),
}

impl EventEmitter<Event> for ExtensionStore {}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

actions!(zed, [ReloadExtensions]);

pub struct ExtensionFeatures {
    theme_registry: Arc<ThemeRegistry>,
    slash_command_registry: Arc<SlashCommandRegistry>,
    indexed_docs_registry: Arc<IndexedDocsRegistry>,
    snippet_registry: Arc<SnippetRegistry>,
    language_registry: Arc<LanguageRegistry>,
}

impl extension_headless::ExtensionFeatures for ExtensionFeatures {
    fn remove_user_themes(&self, themes: &[ui::SharedString], _cx: &mut AppContext) {
        self.theme_registry.remove_user_themes(themes_to_remove)
    }

    fn register_wasm_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>, cx: &mut AppContext) {
        self.language_registry.register_wasm_grammars(grammars)
    }

    fn load_user_theme(
        &self,
        themes_path: &Path,
        fs: Arc<dyn Fs>,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        Box::pin(self.theme_registry.load_user_theme(themes_path, fs))
    }

    fn register_snippets(&self, file_path: &Path, contents: &str) -> Result<()> {
        self.snippet_registry.register_snippets(file_path, contents)
    }
}

pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    node_runtime: NodeRuntime,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    ExtensionSettings::register(cx);

    let store = cx.new_model(move |cx| {
        ExtensionStore::new(
            paths::extensions_dir().clone(),
            None,
            fs,
            client.http_client().clone(),
            client.http_client().clone(),
            Some(client.telemetry().clone()),
            node_runtime,
            language_registry,
            theme_registry,
            SlashCommandRegistry::global(cx),
            IndexedDocsRegistry::global(cx),
            SnippetRegistry::global(cx),
            cx,
        )
    });

    cx.on_action(|_: &ReloadExtensions, cx| {
        let store = cx.global::<GlobalExtensionStore>().0.clone();
        store.update(cx, |store, cx| drop(store.reload(None, cx)));
    });

    cx.set_global(GlobalExtensionStore(store));
}

impl ExtensionStore {
    pub fn try_global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<GlobalExtensionStore>()
            .map(|store| store.0.clone())
    }

    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalExtensionStore>().0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        extensions_dir: PathBuf,
        build_dir: Option<PathBuf>,
        fs: Arc<dyn Fs>,
        http_client: Arc<HttpClientWithUrl>,
        builder_client: Arc<dyn HttpClient>,
        telemetry: Option<Arc<Telemetry>>,
        node_runtime: NodeRuntime,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        indexed_docs_registry: Arc<IndexedDocsRegistry>,
        snippet_registry: Arc<SnippetRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let work_dir = extensions_dir.join("work");
        let build_dir = build_dir.unwrap_or_else(|| extensions_dir.join("build"));
        let installed_dir = extensions_dir.join("installed");
        let index_path = extensions_dir.join("index.json");

        let (reload_tx, mut reload_rx) = unbounded();
        let mut this = Self {
            headless_store: cx.new_model(|cx| {
                HeadlessExtensionStore::new(
                    extensions_dir,
                    fs,
                    http_client,
                    telemetry,
                    node_runtime,
                    language_registry,
                    feature_provider,
                    cx,
                )
            }),
            builder: Arc::new(ExtensionBuilder::new(builder_client, build_dir)),
            theme_registry,
            slash_command_registry,
            indexed_docs_registry,
            snippet_registry,
        };
    }

    pub fn install_dev_extension(
        &mut self,
        extension_source_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let extensions_dir = self.extensions_dir();
        let fs = self.fs.clone();
        let builder = self.builder.clone();

        cx.spawn(move |this, mut cx| async move {
            let mut extension_manifest =
                ExtensionManifest::load(fs.clone(), &extension_source_path).await?;
            let extension_id = extension_manifest.id.clone();

            if !this.update(&mut cx, |this, cx| {
                match this.outstanding_operations.entry(extension_id.clone()) {
                    btree_map::Entry::Occupied(_) => return false,
                    btree_map::Entry::Vacant(e) => e.insert(ExtensionOperation::Remove),
                };
                cx.notify();
                true
            })? {
                return Ok(());
            }

            let _finish = util::defer({
                let this = this.clone();
                let mut cx = cx.clone();
                let extension_id = extension_id.clone();
                move || {
                    this.update(&mut cx, |this, cx| {
                        this.outstanding_operations.remove(extension_id.as_ref());
                        cx.notify();
                    })
                    .ok();
                }
            });

            cx.background_executor()
                .spawn({
                    let extension_source_path = extension_source_path.clone();
                    async move {
                        builder
                            .compile_extension(
                                &extension_source_path,
                                &mut extension_manifest,
                                CompileExtensionOptions { release: false },
                            )
                            .await
                    }
                })
                .await?;

            let output_path = &extensions_dir.join(extension_id.as_ref());
            if let Some(metadata) = fs.metadata(output_path).await? {
                if metadata.is_symlink {
                    fs.remove_file(
                        output_path,
                        RemoveOptions {
                            recursive: false,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await?;
                } else {
                    bail!("extension {extension_id} is already installed");
                }
            }

            fs.create_symlink(output_path, extension_source_path)
                .await?;

            this.update(&mut cx, |this, cx| this.reload(None, cx))?
                .await;
            Ok(())
        })
    }

    pub fn rebuild_dev_extension(&mut self, extension_id: Arc<str>, cx: &mut ModelContext<Self>) {
        let path = self.installed_dir.join(extension_id.as_ref());
        let builder = self.builder.clone();
        let fs = self.fs.clone();

        match self.outstanding_operations.entry(extension_id.clone()) {
            btree_map::Entry::Occupied(_) => return,
            btree_map::Entry::Vacant(e) => e.insert(ExtensionOperation::Upgrade),
        };

        cx.notify();
        let compile = cx.background_executor().spawn(async move {
            let mut manifest = ExtensionManifest::load(fs, &path).await?;
            builder
                .compile_extension(
                    &path,
                    &mut manifest,
                    CompileExtensionOptions { release: true },
                )
                .await
        });

        cx.spawn(|this, mut cx| async move {
            let result = compile.await;

            this.update(&mut cx, |this, cx| {
                this.outstanding_operations.remove(&extension_id);
                cx.notify();
            })?;

            if result.is_ok() {
                this.update(&mut cx, |this, cx| this.reload(Some(extension_id), cx))?
                    .await;
            }

            result
        })
        .detach_and_log_err(cx)
    }
}
