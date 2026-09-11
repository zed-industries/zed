use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};

use anyhow::{Context as _, Result};
use client::{TypedEnvelope, proto};
use collections::{BTreeMap, FxHasher, HashSet};
use extension::{
    Event, Extension, ExtensionDebugAdapterProviderProxy, ExtensionEvents, ExtensionHostProxy,
    ExtensionLanguageProxy, ExtensionLanguageServerProxy, ExtensionManifest,
};
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::{
    StreamExt as _,
    future::{FutureExt as _, join_all},
};
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Task, WeakEntity};
use http_client::HttpClient;
use language::{LanguageConfig, LanguageName, LanguageQueries, LoadedLanguage};
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use util::ResultExt as _;

use crate::wasm_host::{WasmExtension, WasmHost};

#[derive(Clone, Debug)]
pub struct ExtensionVersion {
    pub id: String,
    pub version: String,
    pub dev: bool,
    pub content_fingerprint: Option<u64>,
}

pub(crate) const STALE_UPLOAD_TTL: Duration =
    Duration::from_secs(crate::REMOTE_SYNC_TIMEOUT.as_secs() * 3);

pub struct HeadlessExtensionStore {
    pub fs: Arc<dyn Fs>,
    pub extension_dir: PathBuf,
    pub proxy: Arc<ExtensionHostProxy>,
    pub wasm_host: Arc<WasmHost>,
    pub(crate) loaded_extensions: BTreeMap<Arc<str>, LoadedExtension>,
    failed_removals: HashSet<Arc<str>>,
    operation_lock: Arc<futures::lock::Mutex<()>>,
    _stale_uploads_sweep: Task<()>,
}

#[derive(Clone)]
pub(crate) struct LoadedExtension {
    pub version: Arc<str>,
    pub languages: Vec<(LanguageName, LanguageConfig)>,
    pub language_servers: Vec<(LanguageServerName, LanguageName)>,
    pub debug_adapters: Vec<(Arc<str>, PathBuf)>,
    pub debug_locators: Vec<Arc<str>>,
    pub wasm_extension: Option<Arc<dyn Extension>>,
    pub content_fingerprint: Option<u64>,
}

impl HeadlessExtensionStore {
    pub fn new(
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        extension_dir: PathBuf,
        extension_host_proxy: Arc<ExtensionHostProxy>,
        node_runtime: NodeRuntime,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let stale_uploads_sweep = cx.spawn({
                let fs = fs.clone();
                async move |_, cx| {
                    loop {
                        remove_stale_uploads(&fs, paths::remote_extensions_uploads_dir()).await;
                        cx.background_executor().timer(STALE_UPLOAD_TTL).await;
                    }
                }
            });

            Self {
                fs: fs.clone(),
                wasm_host: WasmHost::new(
                    fs.clone(),
                    http_client.clone(),
                    node_runtime,
                    extension_host_proxy.clone(),
                    extension_dir.join("work"),
                    cx,
                ),
                extension_dir,
                proxy: extension_host_proxy,
                loaded_extensions: BTreeMap::default(),
                failed_removals: HashSet::default(),
                operation_lock: Arc::default(),
                _stale_uploads_sweep: stale_uploads_sweep,
            }
        })
    }

    pub fn sync_extensions(
        &mut self,
        extensions: Vec<ExtensionVersion>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<ExtensionVersion>>> {
        let operation_lock = self.operation_lock.clone();
        cx.spawn(async move |store, cx| {
            let _operation_guard = operation_lock.lock().await;
            let (to_remove, to_load) = store.update(cx, |store, _cx| {
                let on_client = HashSet::from_iter(extensions.iter().map(|e| e.id.as_str()));
                store
                    .failed_removals
                    .retain(|id| !on_client.contains(id.as_ref()));
                let mut to_remove = store
                    .loaded_extensions
                    .keys()
                    .filter(|id| !on_client.contains(id.as_ref()))
                    .cloned()
                    .collect::<Vec<Arc<str>>>();
                to_remove.extend(store.failed_removals.drain());
                let to_load = extensions
                    .into_iter()
                    .filter(|extension| {
                        let is_new = store
                            .loaded_extensions
                            .get(extension.id.as_str())
                            .is_none_or(|loaded| {
                                loaded.version.as_ref() != extension.version.as_str()
                            });
                        extension.dev || is_new
                    })
                    .collect::<Vec<ExtensionVersion>>();
                (to_remove, to_load)
            })?;

            let mut extensions_changed = false;
            let result = async {
                let mut missing = Vec::new();

                for extension_id in to_remove {
                    log::info!("removing extension: {extension_id}");
                    let (was_loaded, removal) = store.update(cx, |store, cx| {
                        (
                            store.loaded_extensions.contains_key(&extension_id),
                            store.uninstall_extension(&extension_id, cx),
                        )
                    })?;
                    if was_loaded {
                        extensions_changed = true;
                    }
                    if let Err(error) = removal.await {
                        log::error!("failed to remove extension: {extension_id}, {error:#}");
                        store.update(cx, |store, _cx| {
                            store.failed_removals.insert(extension_id.clone());
                        })?;
                    }
                }

                for extension in to_load {
                    match Self::load_extension(store.clone(), &extension, cx).await {
                        Ok(changed) => {
                            if changed {
                                extensions_changed = true;
                            }
                            if extension.dev
                                && !Self::is_loaded_content_up_to_date(&store, &extension, cx)?
                            {
                                missing.push(extension)
                            }
                        }
                        Err(error) => {
                            log::info!("failed to load extension: {}, {:#}", extension.id, error);
                            missing.push(extension)
                        }
                    }
                }

                anyhow::Ok(missing)
            }
            .await;

            if extensions_changed {
                store.update(cx, |_, cx| notify_extensions_changed(cx)).ok();
            }

            result
        })
    }

    fn is_loaded_content_up_to_date(
        store: &WeakEntity<Self>,
        extension: &ExtensionVersion,
        cx: &AsyncApp,
    ) -> Result<bool> {
        let loaded_fingerprint = store.read_with(cx, |store, _cx| {
            store
                .loaded_extensions
                .get(extension.id.as_str())
                .and_then(|loaded| loaded.content_fingerprint)
        })?;
        Ok(extension.content_fingerprint.is_some()
            && extension.content_fingerprint == loaded_fingerprint)
    }

    async fn load_extension(
        store: WeakEntity<Self>,
        extension: &ExtensionVersion,
        cx: &mut AsyncApp,
    ) -> Result<bool> {
        let (fs, wasm_host, extension_dir, loaded_fingerprint) =
            store.read_with(cx, |store, _cx| {
                (
                    store.fs.clone(),
                    store.wasm_host.clone(),
                    store.extension_dir.join(&extension.id),
                    store
                        .loaded_extensions
                        .get(extension.id.as_str())
                        .filter(|loaded| loaded.version.as_ref() == extension.version.as_str())
                        .and_then(|loaded| loaded.content_fingerprint),
                )
            })?;
        let content_fingerprint = if extension.dev {
            fingerprint_directory(&fs, &extension_dir, cx).await
        } else {
            None
        };
        if content_fingerprint.is_some() && content_fingerprint == loaded_fingerprint {
            return Ok(false);
        }
        let loaded = Self::prepare_extension(
            fs,
            wasm_host,
            extension_dir.clone(),
            extension_dir,
            extension,
            content_fingerprint,
            cx,
        )
        .await?;
        let removal_tasks = store.update(cx, |store, cx| {
            store.commit_extension(extension.id.as_str().into(), Some(loaded), cx)
        })?;
        for removal in join_all(removal_tasks).await {
            removal.log_err();
        }
        Ok(true)
    }

    async fn prepare_extension(
        fs: Arc<dyn Fs>,
        wasm_host: Arc<WasmHost>,
        load_dir: PathBuf,
        installed_dir: PathBuf,
        extension: &ExtensionVersion,
        content_fingerprint: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Result<LoadedExtension> {
        let manifest = Arc::new(ExtensionManifest::load(fs.clone(), &load_dir).await?);

        debug_assert!(!manifest.languages.is_empty() || manifest.allow_remote_load());

        anyhow::ensure!(
            manifest.version.as_ref() == extension.version.as_str(),
            "mismatched versions: ({}) != ({})",
            manifest.version,
            extension.version,
        );

        let mut languages = Vec::new();
        for language_path in &manifest.languages {
            let config_path = load_dir.join(language_path).join(LanguageConfig::FILE_NAME);
            let config = fs.load(&config_path).await?;
            let mut config = ::toml::from_str::<LanguageConfig>(&config)?;
            config.grammar = None;
            languages.push((config.name.clone(), config));
        }

        let mut language_servers = Vec::new();
        let mut debug_adapters = Vec::new();
        let mut debug_locators = Vec::new();
        let mut wasm_extension: Option<Arc<dyn Extension>> = None;
        if manifest.allow_remote_load() {
            wasm_extension = Some(Arc::new(
                WasmExtension::load(&load_dir, &manifest, wasm_host, cx).await?,
            ));

            for (language_server_id, language_server_config) in &manifest.language_servers {
                for language in language_server_config.languages() {
                    language_servers.push((language_server_id.clone(), language));
                }
            }

            for (debug_adapter, meta) in &manifest.debug_adapters {
                let schema_path = extension::build_debug_adapter_schema_path(debug_adapter, meta)?;
                debug_adapters.push((debug_adapter.clone(), installed_dir.join(schema_path)));
            }

            debug_locators = manifest.debug_locators.keys().cloned().collect();
        }

        Ok(LoadedExtension {
            version: extension.version.as_str().into(),
            languages,
            language_servers,
            debug_adapters,
            debug_locators,
            wasm_extension,
            content_fingerprint,
        })
    }

    pub(crate) fn commit_extension(
        &mut self,
        extension_id: Arc<str>,
        loaded: Option<LoadedExtension>,
        cx: &mut App,
    ) -> Vec<Task<Result<()>>> {
        let previous = match loaded {
            Some(loaded) => self.loaded_extensions.insert(extension_id.clone(), loaded),
            None => self.loaded_extensions.remove(&extension_id),
        };
        let current = self.loaded_extensions.get(&extension_id).cloned();

        let mut removal_tasks = Vec::new();
        if let Some(previous) = previous {
            let mut languages_to_remove = Vec::new();
            for (language, _) in &previous.languages {
                if current.as_ref().is_some_and(|current| {
                    current.languages.iter().any(|(name, _)| name == language)
                }) {
                    continue;
                }
                match self.surviving_language_config(language) {
                    Some(config) => register_language_from_config(&self.proxy, config),
                    None => languages_to_remove.push(language.clone()),
                }
            }
            self.proxy.remove_languages(&languages_to_remove, &[]);

            for (server_name, language) in &previous.language_servers {
                removal_tasks.push(self.proxy.remove_language_server(language, server_name, cx));
                let in_current = current.as_ref().is_some_and(|current| {
                    current
                        .language_servers
                        .iter()
                        .any(|(name, language_name)| {
                            name == server_name && language_name == language
                        })
                });
                if in_current {
                    continue;
                }
                if let Some(extension) = self.surviving_language_server(server_name, language) {
                    self.proxy.register_language_server(
                        extension,
                        server_name.clone(),
                        language.clone(),
                    );
                }
            }

            for (adapter_name, _) in &previous.debug_adapters {
                if current.as_ref().is_some_and(|current| {
                    current
                        .debug_adapters
                        .iter()
                        .any(|(name, _)| name == adapter_name)
                }) {
                    continue;
                }
                match self.surviving_debug_adapter(adapter_name) {
                    Some((schema_path, extension)) => {
                        self.proxy.register_debug_adapter(
                            extension,
                            adapter_name.clone(),
                            &schema_path,
                        );
                    }
                    None => self.proxy.unregister_debug_adapter(adapter_name.clone()),
                }
            }

            for locator_name in &previous.debug_locators {
                if current.as_ref().is_some_and(|current| {
                    current
                        .debug_locators
                        .iter()
                        .any(|name| name == locator_name)
                }) {
                    continue;
                }
                match self.surviving_debug_locator(locator_name) {
                    Some(extension) => {
                        self.proxy
                            .register_debug_locator(extension, locator_name.clone());
                    }
                    None => self.proxy.unregister_debug_locator(locator_name.clone()),
                }
            }
        }

        if let Some(current) = &current {
            for (_, config) in &current.languages {
                register_language_from_config(&self.proxy, config.clone());
            }
            if let Some(wasm_extension) = &current.wasm_extension {
                for (server_name, language) in &current.language_servers {
                    self.proxy.register_language_server(
                        wasm_extension.clone(),
                        server_name.clone(),
                        language.clone(),
                    );
                    log::info!("Loaded language server: {server_name}");
                }
                for (adapter_name, schema_path) in &current.debug_adapters {
                    self.proxy.register_debug_adapter(
                        wasm_extension.clone(),
                        adapter_name.clone(),
                        schema_path,
                    );
                    log::info!("Loaded debug adapter: {adapter_name}");
                }
                for locator_name in &current.debug_locators {
                    self.proxy
                        .register_debug_locator(wasm_extension.clone(), locator_name.clone());
                    log::info!("Loaded debug locator: {locator_name}");
                }
            }
        }

        removal_tasks
    }

    fn surviving_language_config(&self, language: &LanguageName) -> Option<LanguageConfig> {
        self.loaded_extensions.values().find_map(|extension| {
            extension
                .languages
                .iter()
                .find(|(name, _)| name == language)
                .map(|(_, config)| config.clone())
        })
    }

    fn surviving_language_server(
        &self,
        server_name: &LanguageServerName,
        language: &LanguageName,
    ) -> Option<Arc<dyn Extension>> {
        self.loaded_extensions.values().find_map(|extension| {
            let wasm_extension = extension.wasm_extension.clone()?;
            extension
                .language_servers
                .iter()
                .any(|(name, language_name)| name == server_name && language_name == language)
                .then_some(wasm_extension)
        })
    }

    fn surviving_debug_adapter(
        &self,
        adapter_name: &Arc<str>,
    ) -> Option<(PathBuf, Arc<dyn Extension>)> {
        self.loaded_extensions.values().find_map(|extension| {
            let wasm_extension = extension.wasm_extension.clone()?;
            extension
                .debug_adapters
                .iter()
                .find(|(name, _)| name == adapter_name)
                .map(|(_, schema_path)| (schema_path.clone(), wasm_extension))
        })
    }

    fn surviving_debug_locator(&self, locator_name: &Arc<str>) -> Option<Arc<dyn Extension>> {
        self.loaded_extensions.values().find_map(|extension| {
            let wasm_extension = extension.wasm_extension.clone()?;
            extension
                .debug_locators
                .iter()
                .any(|name| name == locator_name)
                .then_some(wasm_extension)
        })
    }

    fn uninstall_extension(
        &mut self,
        extension_id: &Arc<str>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let removal_tasks = self.commit_extension(extension_id.clone(), None, cx);

        let path = self.extension_dir.join(extension_id.as_ref());
        let fs = self.fs.clone();
        cx.spawn(async move |_, _cx| {
            for removal in join_all(removal_tasks).await {
                removal.log_err();
            }

            fs.remove_dir(
                &path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .with_context(|| format!("Removing directory {path:?}"))
        })
    }

    pub fn install_extension(
        &mut self,
        extension: ExtensionVersion,
        tmp_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let path = self.extension_dir.join(&extension.id);
        let fs = self.fs.clone();
        let wasm_host = self.wasm_host.clone();
        let operation_lock = self.operation_lock.clone();

        cx.spawn(async move |store, cx| {
            let _operation_guard = operation_lock.lock().await;

            let loaded_fingerprint = store.read_with(cx, |store, _cx| {
                store
                    .loaded_extensions
                    .get(extension.id.as_str())
                    .filter(|loaded| loaded.version.as_ref() == extension.version.as_str())
                    .and_then(|loaded| loaded.content_fingerprint)
            })?;
            let content_fingerprint = if extension.dev {
                fingerprint_directory(&fs, &tmp_path, cx).await
            } else {
                None
            };
            if content_fingerprint.is_some() && content_fingerprint == loaded_fingerprint {
                fs.remove_dir(
                    &tmp_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .log_err();
                return Ok(());
            }

            let mut installed_dir_touched = false;
            let result = async {
                let loaded = Self::prepare_extension(
                    fs.clone(),
                    wasm_host,
                    tmp_path.clone(),
                    path.clone(),
                    &extension,
                    content_fingerprint,
                    cx,
                )
                .await?;

                installed_dir_touched = true;
                fs.remove_dir(
                    &path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .with_context(|| format!("Removing existing directory {path:?}"))?;
                fs.rename(&tmp_path, &path, RenameOptions::default())
                    .await
                    .with_context(|| format!("Failed to rename {tmp_path:?} to {path:?}"))?;

                let removal_tasks = store.update(cx, |store, cx| {
                    store.commit_extension(extension.id.as_str().into(), Some(loaded), cx)
                })?;
                installed_dir_touched = false;
                for removal in join_all(removal_tasks).await {
                    removal.log_err();
                }

                store.update(cx, |_, cx| notify_extensions_changed(cx))?;
                anyhow::Ok(())
            }
            .await;

            if result.is_err() {
                fs.remove_dir(
                    &tmp_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .log_err();

                if installed_dir_touched
                    && let Ok(removal_tasks) = store.update(cx, |store, cx| {
                        store
                            .failed_removals
                            .insert(Arc::from(extension.id.as_str()));
                        store.commit_extension(extension.id.as_str().into(), None, cx)
                    })
                {
                    for removal in join_all(removal_tasks).await {
                        removal.log_err();
                    }
                    store.update(cx, |_, cx| notify_extensions_changed(cx)).ok();
                }
            }

            result
        })
    }

    pub async fn handle_sync_extensions(
        extension_store: Entity<HeadlessExtensionStore>,
        envelope: TypedEnvelope<proto::SyncExtensions>,
        mut cx: AsyncApp,
    ) -> Result<proto::SyncExtensionsResponse> {
        let requested_extensions =
            envelope
                .payload
                .extensions
                .into_iter()
                .map(|p| ExtensionVersion {
                    id: p.id,
                    version: p.version,
                    dev: p.dev,
                    content_fingerprint: p.content_fingerprint,
                });
        let missing_extensions = extension_store
            .update(&mut cx, |extension_store, cx| {
                extension_store.sync_extensions(requested_extensions.collect(), cx)
            })
            .await?;

        let fs = extension_store.read_with(&cx, |extension_store, _cx| extension_store.fs.clone());
        remove_stale_uploads(&fs, paths::remote_extensions_uploads_dir()).await;

        Ok(proto::SyncExtensionsResponse {
            missing_extensions: missing_extensions
                .into_iter()
                .map(|e| proto::Extension {
                    id: e.id,
                    version: e.version,
                    dev: e.dev,
                    content_fingerprint: e.content_fingerprint,
                })
                .collect(),
            tmp_dir: paths::remote_extensions_uploads_dir()
                .to_string_lossy()
                .to_string(),
        })
    }

    pub async fn handle_install_extension(
        extensions: Entity<HeadlessExtensionStore>,
        envelope: TypedEnvelope<proto::InstallExtension>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        let extension = envelope
            .payload
            .extension
            .context("Invalid InstallExtension request")?;

        extensions
            .update(&mut cx, |extensions, cx| {
                extensions.install_extension(
                    ExtensionVersion {
                        id: extension.id,
                        version: extension.version,
                        dev: extension.dev,
                        content_fingerprint: extension.content_fingerprint,
                    },
                    PathBuf::from(envelope.payload.tmp_dir),
                    cx,
                )
            })
            .await?;

        Ok(proto::Ack {})
    }
}

pub(crate) async fn remove_stale_uploads(fs: &Arc<dyn Fs>, uploads_dir: &Path) {
    let Ok(mut entries) = fs.read_dir(uploads_dir).await else {
        return;
    };
    let now = SystemTime::now();
    while let Some(entry) = entries.next().await {
        let Some(path) = entry.log_err() else {
            continue;
        };
        let Ok(Some(metadata)) = fs.metadata(&path).await else {
            continue;
        };
        let is_stale = match now.duration_since(metadata.mtime.timestamp_for_user()) {
            Ok(age) => age > STALE_UPLOAD_TTL,
            Err(error) => error.duration() > STALE_UPLOAD_TTL,
        };
        if !is_stale {
            continue;
        }
        if metadata.is_dir {
            fs.remove_dir(
                &path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .log_err();
        } else {
            fs.remove_file(
                &path,
                RemoveOptions {
                    recursive: false,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .log_err();
        }
    }
}

async fn fingerprint_directory(fs: &Arc<dyn Fs>, path: &Path, cx: &AsyncApp) -> Option<u64> {
    let fs = fs.clone();
    let path = path.to_path_buf();
    cx.background_executor()
        .spawn(async move { hash_directory_contents(&fs, &path).await })
        .await
        .log_err()
}

pub(crate) async fn hash_directory_contents(fs: &Arc<dyn Fs>, root: &Path) -> Result<u64> {
    let mut hasher = FxHasher::default();
    let mut directories = vec![root.to_path_buf()];
    while let Some(directory) = directories.pop() {
        let mut paths = Vec::new();
        let mut entries = fs.read_dir(&directory).await?;
        while let Some(entry) = entries.next().await {
            paths.push(entry?);
        }
        paths.sort();
        for path in paths {
            let metadata = fs
                .metadata(&path)
                .await?
                .with_context(|| format!("missing metadata for {path:?}"))?;
            for component in path.strip_prefix(root)?.components() {
                component.as_os_str().to_string_lossy().hash(&mut hasher);
            }
            if metadata.is_symlink {
                2_u8.hash(&mut hasher);
                fs.read_link(&path)
                    .await?
                    .to_string_lossy()
                    .hash(&mut hasher);
            } else if metadata.is_dir {
                1_u8.hash(&mut hasher);
                directories.push(path);
            } else {
                0_u8.hash(&mut hasher);
                fs.load_bytes(&path).await?.hash(&mut hasher);
            }
        }
    }
    Ok(hasher.finish())
}

fn notify_extensions_changed(cx: &mut App) {
    if let Some(events) = ExtensionEvents::try_global(cx) {
        events.update(cx, |events, cx| {
            events.emit(Event::ExtensionsInstalledChanged, cx)
        });
    }
}

fn register_language_from_config(proxy: &ExtensionHostProxy, config: LanguageConfig) {
    proxy.register_language(
        config.name.clone(),
        None,
        config.matcher.clone(),
        config.hidden,
        Arc::new(move || {
            let config = config.clone();
            async move {
                Ok(LoadedLanguage {
                    config,
                    queries: LanguageQueries::default(),
                    context_provider: None,
                    toolchain_provider: None,
                    manifest_name: None,
                })
            }
            .boxed()
        }),
    );
}
