mod build_extension;
mod extension_lsp_adapter;
mod extension_manifest;
mod wasm_host;

#[cfg(test)]
mod extension_store_test;

use crate::{extension_lsp_adapter::ExtensionLspAdapter, wasm_host::wit};
use anyhow::{anyhow, bail, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use build_extension::{CompileExtensionOptions, ExtensionBuilder};
use collections::{BTreeMap, HashSet};
use extension_manifest::ExtensionLibraryKind;
use fs::{Fs, RemoveOptions};
use futures::{channel::mpsc::unbounded, io::BufReader, AsyncReadExt as _, StreamExt as _};
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, Task};
use language::{
    LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry, QUERY_FILENAME_PREFIXES,
};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ffi::OsStr,
    path::{self, Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::{ThemeRegistry, ThemeSettings};
use util::{
    http::{AsyncBody, HttpClient, HttpClientWithUrl},
    paths::EXTENSIONS_DIR,
    ResultExt, TryFutureExt,
};
use wasm_host::{WasmExtension, WasmHost};

pub use extension_manifest::{ExtensionManifest, GrammarManifestEntry, OldExtensionManifest};

#[derive(Deserialize)]
pub struct ExtensionsApiResponse {
    pub data: Vec<ExtensionApiResponse>,
}

#[derive(Clone, Deserialize)]
pub struct ExtensionApiResponse {
    pub id: Arc<str>,
    pub name: String,
    pub version: Arc<str>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: String,
    pub download_count: usize,
}

#[derive(Clone)]
pub enum ExtensionStatus {
    NotInstalled,
    Installing,
    Upgrading,
    Installed(Arc<str>),
    Removing,
}

impl ExtensionStatus {
    pub fn is_installing(&self) -> bool {
        matches!(self, Self::Installing)
    }

    pub fn is_upgrading(&self) -> bool {
        matches!(self, Self::Upgrading)
    }

    pub fn is_removing(&self) -> bool {
        matches!(self, Self::Removing)
    }
}

pub struct ExtensionStore {
    builder: Arc<ExtensionBuilder>,
    extension_index: ExtensionIndex,
    fs: Arc<dyn Fs>,
    http_client: Arc<HttpClientWithUrl>,
    extensions_dir: PathBuf,
    extensions_being_installed: HashSet<Arc<str>>,
    extensions_being_uninstalled: HashSet<Arc<str>>,
    index_path: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    modified_extensions: HashSet<Arc<str>>,
    wasm_host: Arc<WasmHost>,
    wasm_extensions: Vec<(Arc<ExtensionManifest>, WasmExtension)>,
    reload_task: Option<Task<Option<()>>>,
    needs_reload: bool,
    _watch_extensions_dir: [Task<()>; 2],
}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ExtensionIndex {
    pub extensions: BTreeMap<Arc<str>, Arc<ExtensionManifest>>,
    pub themes: BTreeMap<Arc<str>, ExtensionIndexEntry>,
    pub languages: BTreeMap<Arc<str>, ExtensionIndexLanguageEntry>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct ExtensionIndexEntry {
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
    fs: Arc<fs::RealFs>,
    http_client: Arc<HttpClientWithUrl>,
    node_runtime: Arc<dyn NodeRuntime>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    let store = cx.new_model(move |cx| {
        ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            fs,
            http_client,
            node_runtime,
            language_registry,
            theme_registry,
            cx,
        )
    });

    cx.on_action(|_: &ReloadExtensions, cx| {
        let store = cx.global::<GlobalExtensionStore>().0.clone();
        store.update(cx, |store, cx| store.reload(cx))
    });

    cx.set_global(GlobalExtensionStore(store));
}

impl ExtensionStore {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalExtensionStore>().0.clone()
    }

    pub fn new(
        extensions_dir: PathBuf,
        fs: Arc<dyn Fs>,
        http_client: Arc<HttpClientWithUrl>,
        node_runtime: Arc<dyn NodeRuntime>,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut this = Self {
            extension_index: Default::default(),
            extensions_dir: extensions_dir.join("installed"),
            index_path: extensions_dir.join("index.json"),
            builder: Arc::new(ExtensionBuilder::new(
                extensions_dir.join("build"),
                http_client.clone(),
            )),
            extensions_being_installed: Default::default(),
            extensions_being_uninstalled: Default::default(),
            reload_task: None,
            wasm_host: WasmHost::new(
                fs.clone(),
                http_client.clone(),
                node_runtime,
                language_registry.clone(),
                extensions_dir.join("work"),
            ),
            wasm_extensions: Vec::new(),
            needs_reload: false,
            modified_extensions: Default::default(),
            fs,
            http_client,
            language_registry,
            theme_registry,
            _watch_extensions_dir: [Task::ready(()), Task::ready(())],
        };
        this._watch_extensions_dir = this.watch_extensions_dir(cx);
        this.load(cx);
        this
    }

    /// Load the initial installed extensions. Normally on startup, we can discover
    /// what extensions are installed just by consulting the index file.
    fn load(&mut self, cx: &mut ModelContext<Self>) {
        let (index_content, index_metadata, extensions_metadata) =
            cx.background_executor().block(async {
                futures::join!(
                    self.fs.load(&self.index_path),
                    self.fs.metadata(&self.index_path),
                    self.fs.metadata(&self.extensions_dir),
                )
            });

        if let Some(index_content) = index_content.log_err() {
            if let Some(index) = serde_json::from_str(&index_content).log_err() {
                let update_task = self.extensions_updated(index, cx);
                self.reload_task = Some(cx.spawn(|this, mut cx| async move {
                    update_task.await.log_err();

                    this.update(&mut cx, |this, cx| {
                        this.reload_task.take();
                        if this.needs_reload {
                            this.reload(cx);
                        }
                    })
                    .ok()
                }));

                // Normally on startup, the extensions index will already be up-to-date,
                // so there is no need to perform all of the IO necessary to rebuild the
                // index based on the content of the installed extensions directory.
                // But if that directory's metadata indicates that it was modified more
                // recently than the index file, then we need to rebuild the index.
                if let (Ok(Some(index_metadata)), Ok(Some(extensions_metadata))) =
                    (index_metadata, extensions_metadata)
                {
                    if index_metadata.mtime > extensions_metadata.mtime {
                        return;
                    }
                }
            }
        }

        self.reload(cx)
    }

    fn extensions_dir(&self) -> PathBuf {
        self.extensions_dir.clone()
    }

    pub fn extension_status(&self, extension_id: &str) -> ExtensionStatus {
        let is_uninstalling = self.extensions_being_uninstalled.contains(extension_id);
        if is_uninstalling {
            return ExtensionStatus::Removing;
        }

        let installed_version = self
            .extension_index
            .extensions
            .get(extension_id)
            .map(|manifest| manifest.version.clone());
        let is_installing = self.extensions_being_installed.contains(extension_id);
        match (installed_version, is_installing) {
            (Some(_), true) => ExtensionStatus::Upgrading,
            (Some(version), false) => ExtensionStatus::Installed(version),
            (None, true) => ExtensionStatus::Installing,
            (None, false) => ExtensionStatus::NotInstalled,
        }
    }

    pub fn fetch_extensions(
        &self,
        search: Option<&str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<ExtensionApiResponse>>> {
        let url = self.http_client.build_zed_api_url(&format!(
            "/extensions{query}",
            query = search
                .map(|search| format!("?filter={search}"))
                .unwrap_or_default()
        ));
        let http_client = self.http_client.clone();
        cx.spawn(move |_, _| async move {
            let mut response = http_client.get(&url, AsyncBody::empty(), true).await?;

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

            let response: ExtensionsApiResponse = serde_json::from_slice(&body)?;

            Ok(response.data)
        })
    }

    pub fn install_extension(
        &mut self,
        extension_id: Arc<str>,
        version: Arc<str>,
        cx: &mut ModelContext<Self>,
    ) {
        log::info!("installing extension {extension_id} {version}");
        let url = self
            .http_client
            .build_zed_api_url(&format!("/extensions/{extension_id}/{version}/download"));

        let extensions_dir = self.extensions_dir();
        let http_client = self.http_client.clone();

        self.extensions_being_installed.insert(extension_id.clone());

        cx.spawn(move |this, mut cx| async move {
            let _finish = util::defer({
                let this = this.clone();
                let mut cx = cx.clone();
                let extension_id = extension_id.clone();
                move || {
                    this.update(&mut cx, |this, cx| {
                        this.extensions_being_installed
                            .remove(extension_id.as_ref());
                        cx.notify();
                    })
                    .ok();
                }
            });

            let mut response = http_client
                .get(&url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading extension: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive
                .unpack(extensions_dir.join(extension_id.as_ref()))
                .await?;
            this.update(&mut cx, |this, cx| this.reload(cx))
        })
        .detach_and_log_err(cx);
    }

    pub fn uninstall_extension(&mut self, extension_id: Arc<str>, cx: &mut ModelContext<Self>) {
        let extensions_dir = self.extensions_dir();
        let fs = self.fs.clone();

        self.extensions_being_uninstalled
            .insert(extension_id.clone());

        cx.spawn(move |this, mut cx| async move {
            let _finish = util::defer({
                let this = this.clone();
                let mut cx = cx.clone();
                let extension_id = extension_id.clone();
                move || {
                    this.update(&mut cx, |this, cx| {
                        this.extensions_being_uninstalled
                            .remove(extension_id.as_ref());
                        cx.notify();
                    })
                    .ok();
                }
            });

            fs.remove_dir(
                &extensions_dir.join(extension_id.as_ref()),
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            this.update(&mut cx, |this, cx| this.reload(cx))
        })
        .detach_and_log_err(cx)
    }

    pub fn install_local_extension(
        &mut self,
        extension_source_path: PathBuf,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<()>> {
        let extensions_dir = self.extensions_dir();
        let fs = self.fs.clone();
        let builder = self.builder.clone();

        cx.spawn(move |this, mut cx| async move {
            let extension_manifest =
                Self::load_extension_manifest(fs.clone(), &extension_source_path).await?;
            let extension_id = extension_manifest.id.clone();

            this.update(&mut cx, |this, cx| {
                this.extensions_being_installed.insert(extension_id.clone());
                cx.notify();
            })?;

            let _finish = util::defer({
                let this = this.clone();
                let mut cx = cx.clone();
                let extension_id = extension_id.clone();
                move || {
                    this.update(&mut cx, |this, cx| {
                        this.extensions_being_installed
                            .remove(extension_id.as_ref());
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
                                CompileExtensionOptions { release: true },
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

            this.update(&mut cx, |this, cx| this.reload(cx))
        })
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
    ) -> Task<Result<()>> {
        fn diff<'a, T, I1, I2>(
            old_keys: I1,
            new_keys: I2,
            modified_keys: &HashSet<Arc<str>>,
        ) -> (Vec<Arc<str>>, Vec<Arc<str>>)
        where
            T: PartialEq,
            I1: Iterator<Item = (&'a Arc<str>, T)>,
            I2: Iterator<Item = (&'a Arc<str>, T)>,
        {
            let mut removed_keys = Vec::default();
            let mut added_keys = Vec::default();
            let mut old_keys = old_keys.peekable();
            let mut new_keys = new_keys.peekable();
            loop {
                match (old_keys.peek(), new_keys.peek()) {
                    (None, None) => return (removed_keys, added_keys),
                    (None, Some(_)) => {
                        added_keys.push(new_keys.next().unwrap().0.clone());
                    }
                    (Some(_), None) => {
                        removed_keys.push(old_keys.next().unwrap().0.clone());
                    }
                    (Some((old_key, _)), Some((new_key, _))) => match old_key.cmp(&new_key) {
                        Ordering::Equal => {
                            let (old_key, old_value) = old_keys.next().unwrap();
                            let (new_key, new_value) = new_keys.next().unwrap();
                            if old_value != new_value || modified_keys.contains(old_key) {
                                removed_keys.push(old_key.clone());
                                added_keys.push(new_key.clone());
                            }
                        }
                        Ordering::Less => {
                            removed_keys.push(old_keys.next().unwrap().0.clone());
                        }
                        Ordering::Greater => {
                            added_keys.push(new_keys.next().unwrap().0.clone());
                        }
                    },
                }
            }
        }

        let old_index = &self.extension_index;
        let (extensions_to_unload, extensions_to_load) = diff(
            old_index.extensions.iter(),
            new_index.extensions.iter(),
            &self.modified_extensions,
        );
        self.modified_extensions.clear();

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
        let empty = Default::default();
        let grammars_to_remove = extensions_to_unload
            .iter()
            .flat_map(|extension_id| {
                old_index
                    .extensions
                    .get(extension_id)
                    .map_or(&empty, |extension| &extension.grammars)
                    .keys()
                    .cloned()
            })
            .collect::<Vec<_>>();

        self.wasm_extensions
            .retain(|(extension, _)| !extensions_to_unload.contains(&extension.id));

        for extension_id in &extensions_to_unload {
            if let Some(extension) = old_index.extensions.get(extension_id) {
                for (language_server_name, config) in extension.language_servers.iter() {
                    self.language_registry
                        .remove_lsp_adapter(config.language.as_ref(), language_server_name);
                }
            }
        }

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

            grammars_to_add.extend(extension.grammars.keys().map(|grammar_name| {
                let mut grammar_path = self.extensions_dir.clone();
                grammar_path.extend([extension_id.as_ref(), "grammars"]);
                grammar_path.push(grammar_name.as_ref());
                grammar_path.set_extension("wasm");
                (grammar_name.clone(), grammar_path)
            }));
            themes_to_add.extend(extension.themes.iter().map(|theme_path| {
                let mut path = self.extensions_dir.clone();
                path.extend([Path::new(extension_id.as_ref()), theme_path.as_path()]);
                path
            }));
        }

        self.language_registry
            .register_wasm_grammars(grammars_to_add);

        for (language_name, language) in languages_to_add {
            let mut language_path = self.extensions_dir.clone();
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
                    Ok((config, queries))
                },
            );
        }

        let fs = self.fs.clone();
        let wasm_host = self.wasm_host.clone();
        let root_dir = self.extensions_dir.clone();
        let theme_registry = self.theme_registry.clone();
        let extension_manifests = extensions_to_load
            .iter()
            .filter_map(|name| new_index.extensions.get(name).cloned())
            .collect::<Vec<_>>();

        self.extension_index = new_index;
        cx.notify();

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
            for extension_manifest in extension_manifests {
                if extension_manifest.lib.kind.is_none() {
                    continue;
                };

                let mut path = root_dir.clone();
                path.extend([extension_manifest.id.as_ref(), "extension.wasm"]);
                let mut wasm_file = fs
                    .open_sync(&path)
                    .await
                    .context("failed to open wasm file")?;
                let mut wasm_bytes = Vec::new();
                wasm_file
                    .read_to_end(&mut wasm_bytes)
                    .context("failed to read wasm")?;
                let wasm_extension = wasm_host
                    .load_extension(
                        wasm_bytes,
                        extension_manifest.clone(),
                        cx.background_executor().clone(),
                    )
                    .await
                    .context("failed to load wasm extension")?;
                wasm_extensions.push((extension_manifest.clone(), wasm_extension));
            }

            this.update(&mut cx, |this, cx| {
                for (manifest, wasm_extension) in &wasm_extensions {
                    for (language_server_name, language_server_config) in &manifest.language_servers
                    {
                        this.language_registry.register_lsp_adapter(
                            language_server_config.language.clone(),
                            Arc::new(ExtensionLspAdapter {
                                extension: wasm_extension.clone(),
                                work_dir: this.wasm_host.work_dir.join(manifest.id.as_ref()),
                                config: wit::LanguageServerConfig {
                                    name: language_server_name.0.to_string(),
                                    language_name: language_server_config.language.to_string(),
                                },
                            }),
                        );
                    }
                }
                this.wasm_extensions.extend(wasm_extensions);
                ThemeSettings::reload_current_theme(cx)
            })
            .ok();
            Ok(())
        })
    }

    fn watch_extensions_dir(&self, cx: &mut ModelContext<Self>) -> [Task<()>; 2] {
        let fs = self.fs.clone();
        let extensions_dir = self.extensions_dir.clone();
        let (changed_extensions_tx, mut changed_extensions_rx) = unbounded();

        let events_task = cx.background_executor().spawn(async move {
            let mut events = fs.watch(&extensions_dir, Duration::from_millis(250)).await;
            while let Some(events) = events.next().await {
                for event in events {
                    let Ok(event_path) = event.path.strip_prefix(&extensions_dir) else {
                        continue;
                    };

                    if let Some(path::Component::Normal(extension_dir_name)) =
                        event_path.components().next()
                    {
                        if let Some(extension_id) = extension_dir_name.to_str() {
                            changed_extensions_tx
                                .unbounded_send(Arc::from(extension_id))
                                .ok();
                        }
                    }
                }
            }
        });

        let reload_task = cx.spawn(|this, mut cx| async move {
            while let Some(changed_extension_id) = changed_extensions_rx.next().await {
                if this
                    .update(&mut cx, |this, cx| {
                        this.modified_extensions.insert(changed_extension_id);
                        this.reload(cx);
                    })
                    .is_err()
                {
                    break;
                }
            }
        });

        [events_task, reload_task]
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) {
        if self.reload_task.is_some() {
            self.needs_reload = true;
            return;
        }

        let fs = self.fs.clone();
        let work_dir = self.wasm_host.work_dir.clone();
        let extensions_dir = self.extensions_dir.clone();
        let index_path = self.index_path.clone();
        self.needs_reload = false;
        self.reload_task = Some(cx.spawn(|this, mut cx| {
            async move {
                let extension_index = cx
                    .background_executor()
                    .spawn(async move {
                        let mut index = ExtensionIndex::default();

                        fs.create_dir(&work_dir).await.log_err();
                        fs.create_dir(&extensions_dir).await.log_err();

                        let extension_paths = fs.read_dir(&extensions_dir).await;
                        if let Ok(mut extension_paths) = extension_paths {
                            while let Some(extension_dir) = extension_paths.next().await {
                                let Ok(extension_dir) = extension_dir else {
                                    continue;
                                };
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

                        index
                    })
                    .await;

                if let Ok(task) = this.update(&mut cx, |this, cx| {
                    this.extensions_updated(extension_index, cx)
                }) {
                    task.await.log_err();
                }

                this.update(&mut cx, |this, cx| {
                    this.reload_task.take();
                    if this.needs_reload {
                        this.reload(cx);
                    }
                })
            }
            .log_err()
        }));
    }

    async fn add_extension_to_index(
        fs: Arc<dyn Fs>,
        extension_dir: PathBuf,
        index: &mut ExtensionIndex,
    ) -> Result<()> {
        let mut extension_manifest =
            Self::load_extension_manifest(fs.clone(), &extension_dir).await?;
        let extension_id = extension_manifest.id.clone();

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
                        ExtensionIndexEntry {
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

        index
            .extensions
            .insert(extension_id.clone(), Arc::new(extension_manifest));

        Ok(())
    }

    async fn load_extension_manifest(
        fs: Arc<dyn Fs>,
        extension_dir: &Path,
    ) -> Result<ExtensionManifest> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("invalid extension name"))?;

        let mut extension_manifest_path = extension_dir.join("extension.json");
        if fs.is_file(&extension_manifest_path).await {
            let manifest_content = fs
                .load(&extension_manifest_path)
                .await
                .with_context(|| format!("failed to load {extension_name} extension.json"))?;
            let manifest_json = serde_json::from_str::<OldExtensionManifest>(&manifest_content)
                .with_context(|| {
                    format!("invalid extension.json for extension {extension_name}")
                })?;

            Ok(manifest_from_old_manifest(manifest_json, extension_name))
        } else {
            extension_manifest_path.set_extension("toml");
            let manifest_content = fs
                .load(&extension_manifest_path)
                .await
                .with_context(|| format!("failed to load {extension_name} extension.toml"))?;
            toml::from_str(&manifest_content)
                .with_context(|| format!("invalid extension.json for extension {extension_name}"))
        }
    }
}

fn manifest_from_old_manifest(
    manifest_json: OldExtensionManifest,
    extension_id: &str,
) -> ExtensionManifest {
    ExtensionManifest {
        id: extension_id.into(),
        name: manifest_json.name,
        version: manifest_json.version,
        description: manifest_json.description,
        repository: manifest_json.repository,
        authors: manifest_json.authors,
        lib: Default::default(),
        themes: {
            let mut themes = manifest_json.themes.into_values().collect::<Vec<_>>();
            themes.sort();
            themes.dedup();
            themes
        },
        languages: {
            let mut languages = manifest_json.languages.into_values().collect::<Vec<_>>();
            languages.sort();
            languages.dedup();
            languages
        },
        grammars: manifest_json
            .grammars
            .into_iter()
            .map(|(grammar_name, _)| (grammar_name, Default::default()))
            .collect(),
        language_servers: Default::default(),
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
