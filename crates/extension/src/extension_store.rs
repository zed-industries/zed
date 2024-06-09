pub mod extension_builder;
mod extension_lsp_adapter;
mod extension_manifest;
mod extension_settings;
mod extension_slash_command;
mod wasm_host;

#[cfg(test)]
mod extension_store_test;

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
use http::{AsyncBody, HttpClient, HttpClientWithUrl};
use language::{
    LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry, QUERY_FILENAME_PREFIXES,
};
use node_runtime::NodeRuntime;
use project::ContextProviderWithTasks;
use release_channel::ReleaseChannel;
use semantic_version::SemanticVersion;
use serde::{Deserialize, Serialize};
use settings::Settings;
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
use util::{maybe, paths::EXTENSIONS_DIR, ResultExt};
use wasm_host::{
    wit::{is_supported_wasm_api_version, wasm_api_version_range},
    WasmExtension, WasmHost,
};

pub use extension_manifest::{
    ExtensionLibraryKind, ExtensionManifest, GrammarManifestEntry, OldExtensionManifest,
};
pub use extension_settings::ExtensionSettings;

const RELOAD_DEBOUNCE_DURATION: Duration = Duration::from_millis(200);
const FS_WATCH_LATENCY: Duration = Duration::from_millis(100);

/// The current extension [`SchemaVersion`] supported by Zed.
const CURRENT_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);

/// Returns the [`SchemaVersion`] range that is compatible with this version of Zed.
pub fn schema_version_range() -> RangeInclusive<SchemaVersion> {
    SchemaVersion::ZERO..=CURRENT_SCHEMA_VERSION
}

/// Returns whether the given extension version is compatible with this version of Zed.
pub fn is_version_compatible(
    release_channel: ReleaseChannel,
    extension_version: &ExtensionMetadata,
) -> bool {
    let schema_version = extension_version.manifest.schema_version.unwrap_or(0);
    if CURRENT_SCHEMA_VERSION.0 < schema_version {
        return false;
    }

    if let Some(wasm_api_version) = extension_version
        .manifest
        .wasm_api_version
        .as_ref()
        .and_then(|wasm_api_version| SemanticVersion::from_str(wasm_api_version).ok())
    {
        if !is_supported_wasm_api_version(release_channel, wasm_api_version) {
            return false;
        }
    }

    true
}

pub struct ExtensionStore {
    builder: Arc<ExtensionBuilder>,
    extension_index: ExtensionIndex,
    fs: Arc<dyn Fs>,
    http_client: Arc<HttpClientWithUrl>,
    telemetry: Option<Arc<Telemetry>>,
    reload_tx: UnboundedSender<Option<Arc<str>>>,
    reload_complete_senders: Vec<oneshot::Sender<()>>,
    installed_dir: PathBuf,
    outstanding_operations: BTreeMap<Arc<str>, ExtensionOperation>,
    index_path: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    slash_command_registry: Arc<SlashCommandRegistry>,
    modified_extensions: HashSet<Arc<str>>,
    wasm_host: Arc<WasmHost>,
    wasm_extensions: Vec<(Arc<ExtensionManifest>, WasmExtension)>,
    tasks: Vec<Task<()>>,
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

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ExtensionIndex {
    pub extensions: BTreeMap<Arc<str>, ExtensionIndexEntry>,
    pub themes: BTreeMap<Arc<str>, ExtensionIndexThemeEntry>,
    pub languages: BTreeMap<Arc<str>, ExtensionIndexLanguageEntry>,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct ExtensionIndexEntry {
    pub manifest: Arc<ExtensionManifest>,
    pub dev: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct ExtensionIndexThemeEntry {
    extension: Arc<str>,
    path: PathBuf,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct ExtensionIndexLanguageEntry {
    extension: Arc<str>,
    path: PathBuf,
    matcher: LanguageMatcher,
    grammar: Option<Arc<str>>,
}

actions!(zed, [ReloadExtensions]);

pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    node_runtime: Arc<dyn NodeRuntime>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    ExtensionSettings::register(cx);

    let store = cx.new_model(move |cx| {
        ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            None,
            fs,
            client.http_client().clone(),
            Some(client.telemetry().clone()),
            node_runtime,
            language_registry,
            theme_registry,
            SlashCommandRegistry::global(cx),
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
        telemetry: Option<Arc<Telemetry>>,
        node_runtime: Arc<dyn NodeRuntime>,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        slash_command_registry: Arc<SlashCommandRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let work_dir = extensions_dir.join("work");
        let build_dir = build_dir.unwrap_or_else(|| extensions_dir.join("build"));
        let installed_dir = extensions_dir.join("installed");
        let index_path = extensions_dir.join("index.json");

        let (reload_tx, mut reload_rx) = unbounded();
        let mut this = Self {
            extension_index: Default::default(),
            installed_dir,
            index_path,
            builder: Arc::new(ExtensionBuilder::new(build_dir)),
            outstanding_operations: Default::default(),
            modified_extensions: Default::default(),
            reload_complete_senders: Vec::new(),
            wasm_host: WasmHost::new(
                fs.clone(),
                http_client.clone(),
                node_runtime,
                language_registry.clone(),
                work_dir,
                cx,
            ),
            wasm_extensions: Vec::new(),
            fs,
            http_client,
            telemetry,
            language_registry,
            theme_registry,
            slash_command_registry,
            reload_tx,
            tasks: Vec::new(),
        };

        // The extensions store maintains an index file, which contains a complete
        // list of the installed extensions and the resources that they provide.
        // This index is loaded synchronously on startup.
        let (index_content, index_metadata, extensions_metadata) =
            cx.background_executor().block(async {
                futures::join!(
                    this.fs.load(&this.index_path),
                    this.fs.metadata(&this.index_path),
                    this.fs.metadata(&this.installed_dir),
                )
            });

        // Normally, there is no need to rebuild the index. But if the index file
        // is invalid or is out-of-date according to the filesystem mtimes, then
        // it must be asynchronously rebuilt.
        let mut extension_index = ExtensionIndex::default();
        let mut extension_index_needs_rebuild = true;
        if let Some(index_content) = index_content.ok() {
            if let Some(index) = serde_json::from_str(&index_content).log_err() {
                extension_index = index;
                if let (Ok(Some(index_metadata)), Ok(Some(extensions_metadata))) =
                    (index_metadata, extensions_metadata)
                {
                    if index_metadata.mtime > extensions_metadata.mtime {
                        extension_index_needs_rebuild = false;
                    }
                }
            }
        }

        // Immediately load all of the extensions in the initial manifest. If the
        // index needs to be rebuild, then enqueue
        let load_initial_extensions = this.extensions_updated(extension_index, cx);
        let mut reload_future = None;
        if extension_index_needs_rebuild {
            reload_future = Some(this.reload(None, cx));
        }

        cx.spawn(|this, mut cx| async move {
            if let Some(future) = reload_future {
                future.await;
            }
            this.update(&mut cx, |this, cx| this.auto_install_extensions(cx))
                .ok();
            this.update(&mut cx, |this, cx| this.check_for_updates(cx))
                .ok();
        })
        .detach();

        // Perform all extension loading in a single task to ensure that we
        // never attempt to simultaneously load/unload extensions from multiple
        // parallel tasks.
        this.tasks.push(cx.spawn(|this, mut cx| {
            async move {
                load_initial_extensions.await;

                let mut debounce_timer = cx
                    .background_executor()
                    .spawn(futures::future::pending())
                    .fuse();
                loop {
                    select_biased! {
                        _ = debounce_timer => {
                            let index = this
                                .update(&mut cx, |this, cx| this.rebuild_extension_index(cx))?
                                .await;
                            this.update(&mut cx, |this, cx| this.extensions_updated(index, cx))?
                                .await;
                        }
                        extension_id = reload_rx.next() => {
                            let Some(extension_id) = extension_id else { break; };
                            this.update(&mut cx, |this, _| {
                                this.modified_extensions.extend(extension_id);
                            })?;
                            debounce_timer = cx
                                .background_executor()
                                .timer(RELOAD_DEBOUNCE_DURATION)
                                .fuse();
                        }
                    }
                }

                anyhow::Ok(())
            }
            .map(drop)
        }));

        // Watch the installed extensions directory for changes. Whenever changes are
        // detected, rebuild the extension index, and load/unload any extensions that
        // have been added, removed, or modified.
        this.tasks.push(cx.background_executor().spawn({
            let fs = this.fs.clone();
            let reload_tx = this.reload_tx.clone();
            let installed_dir = this.installed_dir.clone();
            async move {
                let mut paths = fs.watch(&installed_dir, FS_WATCH_LATENCY).await;
                while let Some(paths) = paths.next().await {
                    for path in paths {
                        let Ok(event_path) = path.strip_prefix(&installed_dir) else {
                            continue;
                        };

                        if let Some(path::Component::Normal(extension_dir_name)) =
                            event_path.components().next()
                        {
                            if let Some(extension_id) = extension_dir_name.to_str() {
                                reload_tx.unbounded_send(Some(extension_id.into())).ok();
                            }
                        }
                    }
                }
            }
        }));

        this
    }

    fn reload(
        &mut self,
        modified_extension: Option<Arc<str>>,
        cx: &mut ModelContext<Self>,
    ) -> impl Future<Output = ()> {
        let (tx, rx) = oneshot::channel();
        self.reload_complete_senders.push(tx);
        self.reload_tx
            .unbounded_send(modified_extension)
            .expect("reload task exited");
        cx.emit(Event::StartedReloading);

        async move {
            rx.await.ok();
        }
    }

    fn extensions_dir(&self) -> PathBuf {
        self.installed_dir.clone()
    }

    pub fn outstanding_operations(&self) -> &BTreeMap<Arc<str>, ExtensionOperation> {
        &self.outstanding_operations
    }

    pub fn installed_extensions(&self) -> &BTreeMap<Arc<str>, ExtensionIndexEntry> {
        &self.extension_index.extensions
    }

    pub fn dev_extensions(&self) -> impl Iterator<Item = &Arc<ExtensionManifest>> {
        self.extension_index
            .extensions
            .values()
            .filter_map(|extension| extension.dev.then_some(&extension.manifest))
    }

    /// Returns the names of themes provided by extensions.
    pub fn extension_themes<'a>(
        &'a self,
        extension_id: &'a str,
    ) -> impl Iterator<Item = &'a Arc<str>> {
        self.extension_index
            .themes
            .iter()
            .filter_map(|(name, theme)| theme.extension.as_ref().eq(extension_id).then_some(name))
    }

    pub fn fetch_extensions(
        &self,
        search: Option<&str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ExtensionMetadata>>> {
        let version = CURRENT_SCHEMA_VERSION.to_string();
        let mut query = vec![("max_schema_version", version.as_str())];
        if let Some(search) = search {
            query.push(("filter", search));
        }

        self.fetch_extensions_from_api("/extensions", &query, cx)
    }

    pub fn fetch_extensions_with_update_available(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ExtensionMetadata>>> {
        let schema_versions = schema_version_range();
        let wasm_api_versions = wasm_api_version_range(ReleaseChannel::global(cx));
        let extension_settings = ExtensionSettings::get_global(cx);
        let extension_ids = self
            .extension_index
            .extensions
            .keys()
            .map(|id| id.as_ref())
            .filter(|id| extension_settings.should_auto_update(id))
            .collect::<Vec<_>>()
            .join(",");
        let task = self.fetch_extensions_from_api(
            "/extensions/updates",
            &[
                ("min_schema_version", &schema_versions.start().to_string()),
                ("max_schema_version", &schema_versions.end().to_string()),
                (
                    "min_wasm_api_version",
                    &wasm_api_versions.start().to_string(),
                ),
                ("max_wasm_api_version", &wasm_api_versions.end().to_string()),
                ("ids", &extension_ids),
            ],
            cx,
        );
        cx.spawn(move |this, mut cx| async move {
            let extensions = task.await?;
            this.update(&mut cx, |this, _cx| {
                extensions
                    .into_iter()
                    .filter(|extension| {
                        this.extension_index.extensions.get(&extension.id).map_or(
                            true,
                            |installed_extension| {
                                installed_extension.manifest.version != extension.manifest.version
                            },
                        )
                    })
                    .collect()
            })
        })
    }

    pub fn fetch_extension_versions(
        &self,
        extension_id: &str,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ExtensionMetadata>>> {
        self.fetch_extensions_from_api(&format!("/extensions/{extension_id}"), &[], cx)
    }

    /// Installs any extensions that should be included with Zed by default.
    ///
    /// This can be used to make certain functionality provided by extensions
    /// available out-of-the-box.
    pub fn auto_install_extensions(&mut self, cx: &mut ModelContext<Self>) {
        let extension_settings = ExtensionSettings::get_global(cx);

        let extensions_to_install = extension_settings
            .auto_install_extensions
            .keys()
            .filter(|extension_id| extension_settings.should_auto_install(extension_id))
            .filter(|extension_id| {
                let is_already_installed = self
                    .extension_index
                    .extensions
                    .contains_key(extension_id.as_ref());
                !is_already_installed
            })
            .cloned()
            .collect::<Vec<_>>();

        cx.spawn(move |this, mut cx| async move {
            for extension_id in extensions_to_install {
                this.update(&mut cx, |this, cx| {
                    this.install_latest_extension(extension_id.clone(), cx);
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn check_for_updates(&mut self, cx: &mut ModelContext<Self>) {
        let task = self.fetch_extensions_with_update_available(cx);
        cx.spawn(move |this, mut cx| async move {
            Self::upgrade_extensions(this, task.await?, &mut cx).await
        })
        .detach();
    }

    async fn upgrade_extensions(
        this: WeakModel<Self>,
        extensions: Vec<ExtensionMetadata>,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        for extension in extensions {
            let task = this.update(cx, |this, cx| {
                if let Some(installed_extension) =
                    this.extension_index.extensions.get(&extension.id)
                {
                    let installed_version =
                        SemanticVersion::from_str(&installed_extension.manifest.version).ok()?;
                    let latest_version =
                        SemanticVersion::from_str(&extension.manifest.version).ok()?;

                    if installed_version >= latest_version {
                        return None;
                    }
                }

                Some(this.upgrade_extension(extension.id, extension.manifest.version, cx))
            })?;

            if let Some(task) = task {
                task.await.log_err();
            }
        }
        anyhow::Ok(())
    }

    fn fetch_extensions_from_api(
        &self,
        path: &str,
        query: &[(&str, &str)],
        cx: &mut ModelContext<'_, ExtensionStore>,
    ) -> Task<Result<Vec<ExtensionMetadata>>> {
        let url = self.http_client.build_zed_api_url(path, &query);
        let http_client = self.http_client.clone();
        cx.spawn(move |_, _| async move {
            let mut response = http_client
                .get(&url?.as_ref(), AsyncBody::empty(), true)
                .await?;

            let mut body = Vec::new();
            response
                .body_mut()
                .read_to_end(&mut body)
                .await
                .context("error reading extensions")?;

            if response.status().is_client_error() {
                let text = String::from_utf8_lossy(body.as_slice());
                bail!(
                    "status error {}, response: {text:?}",
                    response.status().as_u16()
                );
            }

            let response: GetExtensionsResponse = serde_json::from_slice(&body)?;
            Ok(response.data)
        })
    }

    pub fn install_extension(
        &mut self,
        extension_id: Arc<str>,
        version: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) {
        self.install_or_upgrade_extension(extension_id, version, ExtensionOperation::Install, cx)
            .detach_and_log_err(cx);
    }

    fn install_or_upgrade_extension_at_endpoint(
        &mut self,
        extension_id: Arc<str>,
        url: Url,
        operation: ExtensionOperation,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let extension_dir = self.installed_dir.join(extension_id.as_ref());
        let http_client = self.http_client.clone();
        let fs = self.fs.clone();

        match self.outstanding_operations.entry(extension_id.clone()) {
            btree_map::Entry::Occupied(_) => return Task::ready(Ok(())),
            btree_map::Entry::Vacant(e) => e.insert(operation),
        };
        cx.notify();

        cx.spawn(move |this, mut cx| async move {
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

            let mut response = http_client
                .get(&url.as_ref(), Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading extension: {}", err))?;

            fs.remove_dir(
                &extension_dir,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            let content_length = response
                .headers()
                .get(isahc::http::header::CONTENT_LENGTH)
                .and_then(|value| value.to_str().ok()?.parse::<usize>().ok());

            let mut body = BufReader::new(response.body_mut());
            let mut tar_gz_bytes = Vec::new();
            body.read_to_end(&mut tar_gz_bytes).await?;

            if let Some(content_length) = content_length {
                let actual_len = tar_gz_bytes.len();
                if content_length != actual_len {
                    bail!("downloaded extension size {actual_len} does not match content length {content_length}");
                }
            }
            let decompressed_bytes = GzipDecoder::new(BufReader::new(tar_gz_bytes.as_slice()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(extension_dir).await?;
            this.update(&mut cx, |this, cx| {
                this.reload(Some(extension_id.clone()), cx)
            })?
            .await;

            match operation {
                ExtensionOperation::Install => {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(Event::ExtensionInstalled(extension_id));
                    })
                    .ok();
                }
                _ => {}
            }

            anyhow::Ok(())
        })
    }

    pub fn install_latest_extension(
        &mut self,
        extension_id: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) {
        log::info!("installing extension {extension_id} latest version");

        let schema_versions = schema_version_range();
        let wasm_api_versions = wasm_api_version_range(ReleaseChannel::global(cx));

        let Some(url) = self
            .http_client
            .build_zed_api_url(
                &format!("/extensions/{extension_id}/download"),
                &[
                    ("min_schema_version", &schema_versions.start().to_string()),
                    ("max_schema_version", &schema_versions.end().to_string()),
                    (
                        "min_wasm_api_version",
                        &wasm_api_versions.start().to_string(),
                    ),
                    ("max_wasm_api_version", &wasm_api_versions.end().to_string()),
                ],
            )
            .log_err()
        else {
            return;
        };

        self.install_or_upgrade_extension_at_endpoint(
            extension_id,
            url,
            ExtensionOperation::Install,
            cx,
        )
        .detach_and_log_err(cx);
    }

    pub fn upgrade_extension(
        &mut self,
        extension_id: Arc<str>,
        version: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        self.install_or_upgrade_extension(extension_id, version, ExtensionOperation::Upgrade, cx)
    }

    fn install_or_upgrade_extension(
        &mut self,
        extension_id: Arc<str>,
        version: Arc<str>,
        operation: ExtensionOperation,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        log::info!("installing extension {extension_id} {version}");
        let Some(url) = self
            .http_client
            .build_zed_api_url(
                &format!("/extensions/{extension_id}/{version}/download"),
                &[],
            )
            .log_err()
        else {
            return Task::ready(Ok(()));
        };

        self.install_or_upgrade_extension_at_endpoint(extension_id, url, operation, cx)
    }

    pub fn uninstall_extension(&mut self, extension_id: Arc<str>, cx: &mut ModelContext<Self>) {
        let extension_dir = self.installed_dir.join(extension_id.as_ref());
        let fs = self.fs.clone();

        match self.outstanding_operations.entry(extension_id.clone()) {
            btree_map::Entry::Occupied(_) => return,
            btree_map::Entry::Vacant(e) => e.insert(ExtensionOperation::Remove),
        };

        cx.spawn(move |this, mut cx| async move {
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

            fs.remove_dir(
                &extension_dir,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            this.update(&mut cx, |this, cx| this.reload(None, cx))?
                .await;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
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
            if let Some(metadata) = fs.metadata(&output_path).await? {
                if metadata.is_symlink {
                    fs.remove_file(
                        &output_path,
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

    /// Updates the set of installed extensions.
    ///
    /// First, this unloads any themes, languages, or grammars that are
    /// no longer in the manifest, or whose files have changed on disk.
    /// Then it loads any themes, languages, or grammars that are newly
    /// added to the manifest, or whose files have changed on disk.
    fn extensions_updated(
        &mut self,
        new_index: ExtensionIndex,
        cx: &mut ModelContext<Self>,
    ) -> Task<()> {
        let old_index = &self.extension_index;

        // Determine which extensions need to be loaded and unloaded, based
        // on the changes to the manifest and the extensions that we know have been
        // modified.
        let mut extensions_to_unload = Vec::default();
        let mut extensions_to_load = Vec::default();
        {
            let mut old_keys = old_index.extensions.iter().peekable();
            let mut new_keys = new_index.extensions.iter().peekable();
            loop {
                match (old_keys.peek(), new_keys.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => {
                        extensions_to_load.push(new_keys.next().unwrap().0.clone());
                    }
                    (Some(_), None) => {
                        extensions_to_unload.push(old_keys.next().unwrap().0.clone());
                    }
                    (Some((old_key, _)), Some((new_key, _))) => match old_key.cmp(&new_key) {
                        Ordering::Equal => {
                            let (old_key, old_value) = old_keys.next().unwrap();
                            let (new_key, new_value) = new_keys.next().unwrap();
                            if old_value != new_value || self.modified_extensions.contains(old_key)
                            {
                                extensions_to_unload.push(old_key.clone());
                                extensions_to_load.push(new_key.clone());
                            }
                        }
                        Ordering::Less => {
                            extensions_to_unload.push(old_keys.next().unwrap().0.clone());
                        }
                        Ordering::Greater => {
                            extensions_to_load.push(new_keys.next().unwrap().0.clone());
                        }
                    },
                }
            }
            self.modified_extensions.clear();
        }

        if extensions_to_load.is_empty() && extensions_to_unload.is_empty() {
            return Task::ready(());
        }

        let reload_count = extensions_to_unload
            .iter()
            .filter(|id| extensions_to_load.contains(id))
            .count();

        log::info!(
            "extensions updated. loading {}, reloading {}, unloading {}",
            extensions_to_load.len() - reload_count,
            reload_count,
            extensions_to_unload.len() - reload_count
        );

        if let Some(telemetry) = &self.telemetry {
            for extension_id in &extensions_to_load {
                if let Some(extension) = new_index.extensions.get(extension_id) {
                    telemetry.report_extension_event(
                        extension_id.clone(),
                        extension.manifest.version.clone(),
                    );
                }
            }
        }

        let themes_to_remove = old_index
            .themes
            .iter()
            .filter_map(|(name, entry)| {
                if extensions_to_unload.contains(&entry.extension) {
                    Some(name.clone().into())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let languages_to_remove = old_index
            .languages
            .iter()
            .filter_map(|(name, entry)| {
                if extensions_to_unload.contains(&entry.extension) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut grammars_to_remove = Vec::new();
        for extension_id in &extensions_to_unload {
            let Some(extension) = old_index.extensions.get(extension_id) else {
                continue;
            };
            grammars_to_remove.extend(extension.manifest.grammars.keys().cloned());
            for (language_server_name, config) in extension.manifest.language_servers.iter() {
                for language in config.languages() {
                    self.language_registry
                        .remove_lsp_adapter(&language, language_server_name);
                }
            }
        }

        self.wasm_extensions
            .retain(|(extension, _)| !extensions_to_unload.contains(&extension.id));
        self.theme_registry.remove_user_themes(&themes_to_remove);
        self.language_registry
            .remove_languages(&languages_to_remove, &grammars_to_remove);

        let languages_to_add = new_index
            .languages
            .iter()
            .filter(|(_, entry)| extensions_to_load.contains(&entry.extension))
            .collect::<Vec<_>>();
        let mut grammars_to_add = Vec::new();
        let mut themes_to_add = Vec::new();
        for extension_id in &extensions_to_load {
            let Some(extension) = new_index.extensions.get(extension_id) else {
                continue;
            };

            grammars_to_add.extend(extension.manifest.grammars.keys().map(|grammar_name| {
                let mut grammar_path = self.installed_dir.clone();
                grammar_path.extend([extension_id.as_ref(), "grammars"]);
                grammar_path.push(grammar_name.as_ref());
                grammar_path.set_extension("wasm");
                (grammar_name.clone(), grammar_path)
            }));
            themes_to_add.extend(extension.manifest.themes.iter().map(|theme_path| {
                let mut path = self.installed_dir.clone();
                path.extend([Path::new(extension_id.as_ref()), theme_path.as_path()]);
                path
            }));
        }

        self.language_registry
            .register_wasm_grammars(grammars_to_add);

        for (language_name, language) in languages_to_add {
            let mut language_path = self.installed_dir.clone();
            language_path.extend([
                Path::new(language.extension.as_ref()),
                language.path.as_path(),
            ]);
            self.language_registry.register_language(
                language_name.clone(),
                language.grammar.clone(),
                language.matcher.clone(),
                move || {
                    let config = std::fs::read_to_string(language_path.join("config.toml"))?;
                    let config: LanguageConfig = ::toml::from_str(&config)?;
                    let queries = load_plugin_queries(&language_path);
                    let tasks = std::fs::read_to_string(language_path.join("tasks.json"))
                        .ok()
                        .and_then(|contents| {
                            let definitions = serde_json_lenient::from_str(&contents).log_err()?;
                            Some(Arc::new(ContextProviderWithTasks::new(definitions)) as Arc<_>)
                        });

                    Ok((config, queries, tasks))
                },
            );
        }

        let fs = self.fs.clone();
        let wasm_host = self.wasm_host.clone();
        let root_dir = self.installed_dir.clone();
        let theme_registry = self.theme_registry.clone();
        let extension_entries = extensions_to_load
            .iter()
            .filter_map(|name| new_index.extensions.get(name).cloned())
            .collect::<Vec<_>>();

        self.extension_index = new_index;
        cx.notify();
        cx.emit(Event::ExtensionsUpdated);

        cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .spawn({
                    let fs = fs.clone();
                    async move {
                        for theme_path in &themes_to_add {
                            theme_registry
                                .load_user_theme(&theme_path, fs.clone())
                                .await
                                .log_err();
                        }
                    }
                })
                .await;

            let mut wasm_extensions = Vec::new();
            for extension in extension_entries {
                if extension.manifest.lib.kind.is_none() {
                    continue;
                };

                let wasm_extension = maybe!(async {
                    let mut path = root_dir.clone();
                    path.extend([extension.manifest.clone().id.as_ref(), "extension.wasm"]);
                    let mut wasm_file = fs
                        .open_sync(&path)
                        .await
                        .context("failed to open wasm file")?;

                    let mut wasm_bytes = Vec::new();
                    wasm_file
                        .read_to_end(&mut wasm_bytes)
                        .context("failed to read wasm")?;

                    wasm_host
                        .load_extension(
                            wasm_bytes,
                            extension.manifest.clone().clone(),
                            cx.background_executor().clone(),
                        )
                        .await
                        .with_context(|| {
                            format!("failed to load wasm extension {}", extension.manifest.id)
                        })
                })
                .await;

                if let Some(wasm_extension) = wasm_extension.log_err() {
                    wasm_extensions.push((extension.manifest.clone(), wasm_extension));
                } else {
                    this.update(&mut cx, |_, cx| {
                        cx.emit(Event::ExtensionFailedToLoad(extension.manifest.id.clone()))
                    })
                    .ok();
                }
            }

            this.update(&mut cx, |this, cx| {
                this.reload_complete_senders.clear();

                for (manifest, wasm_extension) in &wasm_extensions {
                    for (language_server_id, language_server_config) in &manifest.language_servers {
                        for language in language_server_config.languages() {
                            this.language_registry.register_lsp_adapter(
                                language.clone(),
                                Arc::new(ExtensionLspAdapter {
                                    extension: wasm_extension.clone(),
                                    host: this.wasm_host.clone(),
                                    language_server_id: language_server_id.clone(),
                                    config: wit::LanguageServerConfig {
                                        name: language_server_id.0.to_string(),
                                        language_name: language.to_string(),
                                    },
                                }),
                            );
                        }
                    }

                    for (slash_command_name, slash_command) in &manifest.slash_commands {
                        this.slash_command_registry.register_command(
                            ExtensionSlashCommand {
                                command: crate::wit::SlashCommand {
                                    name: slash_command_name.to_string(),
                                    description: slash_command.description.to_string(),
                                    tooltip_text: slash_command.tooltip_text.to_string(),
                                    requires_argument: slash_command.requires_argument,
                                },
                                extension: wasm_extension.clone(),
                                host: this.wasm_host.clone(),
                            },
                            false,
                        );
                    }
                }
                this.wasm_extensions.extend(wasm_extensions);
                ThemeSettings::reload_current_theme(cx)
            })
            .ok();
        })
    }

    fn rebuild_extension_index(&self, cx: &mut ModelContext<Self>) -> Task<ExtensionIndex> {
        let fs = self.fs.clone();
        let work_dir = self.wasm_host.work_dir.clone();
        let extensions_dir = self.installed_dir.clone();
        let index_path = self.index_path.clone();
        cx.background_executor().spawn(async move {
            let start_time = Instant::now();
            let mut index = ExtensionIndex::default();

            fs.create_dir(&work_dir).await.log_err();
            fs.create_dir(&extensions_dir).await.log_err();

            let extension_paths = fs.read_dir(&extensions_dir).await;
            if let Ok(mut extension_paths) = extension_paths {
                while let Some(extension_dir) = extension_paths.next().await {
                    let Ok(extension_dir) = extension_dir else {
                        continue;
                    };

                    if extension_dir
                        .file_name()
                        .map_or(false, |file_name| file_name == ".DS_Store")
                    {
                        continue;
                    }

                    Self::add_extension_to_index(fs.clone(), extension_dir, &mut index)
                        .await
                        .log_err();
                }
            }

            if let Ok(index_json) = serde_json::to_string_pretty(&index) {
                fs.save(&index_path, &index_json.as_str().into(), Default::default())
                    .await
                    .context("failed to save extension index")
                    .log_err();
            }

            log::info!("rebuilt extension index in {:?}", start_time.elapsed());
            index
        })
    }

    async fn add_extension_to_index(
        fs: Arc<dyn Fs>,
        extension_dir: PathBuf,
        index: &mut ExtensionIndex,
    ) -> Result<()> {
        let mut extension_manifest = ExtensionManifest::load(fs.clone(), &extension_dir).await?;
        let extension_id = extension_manifest.id.clone();

        // TODO: distinguish dev extensions more explicitly, by the absence
        // of a checksum file that we'll create when downloading normal extensions.
        let is_dev = fs
            .metadata(&extension_dir)
            .await?
            .ok_or_else(|| anyhow!("directory does not exist"))?
            .is_symlink;

        if let Ok(mut language_paths) = fs.read_dir(&extension_dir.join("languages")).await {
            while let Some(language_path) = language_paths.next().await {
                let language_path = language_path?;
                let Ok(relative_path) = language_path.strip_prefix(&extension_dir) else {
                    continue;
                };
                let Ok(Some(fs_metadata)) = fs.metadata(&language_path).await else {
                    continue;
                };
                if !fs_metadata.is_dir {
                    continue;
                }
                let config = fs.load(&language_path.join("config.toml")).await?;
                let config = ::toml::from_str::<LanguageConfig>(&config)?;

                let relative_path = relative_path.to_path_buf();
                if !extension_manifest.languages.contains(&relative_path) {
                    extension_manifest.languages.push(relative_path.clone());
                }

                index.languages.insert(
                    config.name.clone(),
                    ExtensionIndexLanguageEntry {
                        extension: extension_id.clone(),
                        path: relative_path,
                        matcher: config.matcher,
                        grammar: config.grammar,
                    },
                );
            }
        }

        if let Ok(mut theme_paths) = fs.read_dir(&extension_dir.join("themes")).await {
            while let Some(theme_path) = theme_paths.next().await {
                let theme_path = theme_path?;
                let Ok(relative_path) = theme_path.strip_prefix(&extension_dir) else {
                    continue;
                };

                let Some(theme_family) = ThemeRegistry::read_user_theme(&theme_path, fs.clone())
                    .await
                    .log_err()
                else {
                    continue;
                };

                let relative_path = relative_path.to_path_buf();
                if !extension_manifest.themes.contains(&relative_path) {
                    extension_manifest.themes.push(relative_path.clone());
                }

                for theme in theme_family.themes {
                    index.themes.insert(
                        theme.name.into(),
                        ExtensionIndexThemeEntry {
                            extension: extension_id.clone(),
                            path: relative_path.clone(),
                        },
                    );
                }
            }
        }

        let extension_wasm_path = extension_dir.join("extension.wasm");
        if fs.is_file(&extension_wasm_path).await {
            extension_manifest
                .lib
                .kind
                .get_or_insert(ExtensionLibraryKind::Rust);
        }

        index.extensions.insert(
            extension_id.clone(),
            ExtensionIndexEntry {
                dev: is_dev,
                manifest: Arc::new(extension_manifest),
            },
        );

        Ok(())
    }
}

fn load_plugin_queries(root_path: &Path) -> LanguageQueries {
    let mut result = LanguageQueries::default();
    if let Some(entries) = std::fs::read_dir(root_path).log_err() {
        for entry in entries {
            let Some(entry) = entry.log_err() else {
                continue;
            };
            let path = entry.path();
            if let Some(remainder) = path.strip_prefix(root_path).ok().and_then(|p| p.to_str()) {
                if !remainder.ends_with(".scm") {
                    continue;
                }
                for (name, query) in QUERY_FILENAME_PREFIXES {
                    if remainder.starts_with(name) {
                        if let Some(contents) = std::fs::read_to_string(&path).log_err() {
                            match query(&mut result) {
                                None => *query(&mut result) = Some(contents.into()),
                                Some(r) => r.to_mut().push_str(contents.as_ref()),
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    result
}
