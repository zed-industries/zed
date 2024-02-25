use anyhow::{anyhow, bail, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use collections::{BTreeMap, HashSet};
use fs::{Fs, RemoveOptions};
use futures::channel::mpsc::unbounded;
use futures::StreamExt as _;
use futures::{io::BufReader, AsyncReadExt as _};
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, Task};
use language::{
    LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry, QUERY_FILENAME_PREFIXES,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::{ThemeRegistry, ThemeSettings};
use util::http::{AsyncBody, HttpClientWithUrl};
use util::TryFutureExt;
use util::{http::HttpClient, paths::EXTENSIONS_DIR, ResultExt};

#[cfg(test)]
mod extension_store_test;

#[derive(Deserialize)]
pub struct ExtensionsApiResponse {
    pub data: Vec<Extension>,
}

#[derive(Clone, Deserialize)]
pub struct Extension {
    pub id: Arc<str>,
    pub version: Arc<str>,
    pub name: String,
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
    manifest: Arc<RwLock<Manifest>>,
    fs: Arc<dyn Fs>,
    http_client: Arc<HttpClientWithUrl>,
    extensions_dir: PathBuf,
    extensions_being_installed: HashSet<Arc<str>>,
    extensions_being_uninstalled: HashSet<Arc<str>>,
    manifest_path: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    extension_changes: ExtensionChanges,
    reload_task: Option<Task<Option<()>>>,
    needs_reload: bool,
    _watch_extensions_dir: [Task<()>; 2],
}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Manifest {
    pub extensions: BTreeMap<Arc<str>, Arc<str>>,
    pub grammars: BTreeMap<Arc<str>, GrammarManifestEntry>,
    pub languages: BTreeMap<Arc<str>, LanguageManifestEntry>,
    pub themes: BTreeMap<Arc<str>, ThemeManifestEntry>,
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GrammarManifestEntry {
    extension: String,
    path: PathBuf,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct LanguageManifestEntry {
    extension: String,
    path: PathBuf,
    matcher: LanguageMatcher,
    grammar: Option<Arc<str>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ThemeManifestEntry {
    extension: String,
    path: PathBuf,
}

#[derive(Default)]
struct ExtensionChanges {
    languages: HashSet<Arc<str>>,
    grammars: HashSet<Arc<str>>,
    themes: HashSet<Arc<str>>,
}

actions!(zed, [ReloadExtensions]);

pub fn init(
    fs: Arc<fs::RealFs>,
    http_client: Arc<HttpClientWithUrl>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            fs.clone(),
            http_client.clone(),
            language_registry.clone(),
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
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut this = Self {
            manifest: Default::default(),
            extensions_dir: extensions_dir.join("installed"),
            manifest_path: extensions_dir.join("manifest.json"),
            extensions_being_installed: Default::default(),
            extensions_being_uninstalled: Default::default(),
            reload_task: None,
            needs_reload: false,
            extension_changes: ExtensionChanges::default(),
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

    pub fn load(&mut self, cx: &mut ModelContext<Self>) {
        let (manifest_content, manifest_metadata, extensions_metadata) =
            cx.background_executor().block(async {
                futures::join!(
                    self.fs.load(&self.manifest_path),
                    self.fs.metadata(&self.manifest_path),
                    self.fs.metadata(&self.extensions_dir),
                )
            });

        if let Some(manifest_content) = manifest_content.log_err() {
            if let Some(manifest) = serde_json::from_str(&manifest_content).log_err() {
                self.manifest_updated(manifest, cx);
            }
        }

        let should_reload = if let (Ok(Some(manifest_metadata)), Ok(Some(extensions_metadata))) =
            (manifest_metadata, extensions_metadata)
        {
            extensions_metadata.mtime > manifest_metadata.mtime
        } else {
            true
        };

        if should_reload {
            self.reload(cx)
        }
    }

    pub fn extensions_dir(&self) -> PathBuf {
        self.extensions_dir.clone()
    }

    pub fn extension_status(&self, extension_id: &str) -> ExtensionStatus {
        let is_uninstalling = self.extensions_being_uninstalled.contains(extension_id);
        if is_uninstalling {
            return ExtensionStatus::Removing;
        }

        let installed_version = self.manifest.read().extensions.get(extension_id).cloned();
        let is_installing = self.extensions_being_installed.contains(extension_id);
        match (installed_version, is_installing) {
            (Some(_), true) => ExtensionStatus::Upgrading,
            (Some(version), false) => ExtensionStatus::Installed(version.clone()),
            (None, true) => ExtensionStatus::Installing,
            (None, false) => ExtensionStatus::NotInstalled,
        }
    }

    pub fn fetch_extensions(
        &self,
        search: Option<&str>,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Vec<Extension>>> {
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
            let mut response = http_client
                .get(&url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading extension: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive
                .unpack(extensions_dir.join(extension_id.as_ref()))
                .await?;

            this.update(&mut cx, |this, cx| {
                this.extensions_being_installed
                    .remove(extension_id.as_ref());
                this.reload(cx)
            })
        })
        .detach_and_log_err(cx);
    }

    pub fn uninstall_extension(&mut self, extension_id: Arc<str>, cx: &mut ModelContext<Self>) {
        let extensions_dir = self.extensions_dir();
        let fs = self.fs.clone();

        self.extensions_being_uninstalled
            .insert(extension_id.clone());

        cx.spawn(move |this, mut cx| async move {
            fs.remove_dir(
                &extensions_dir.join(extension_id.as_ref()),
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

            this.update(&mut cx, |this, cx| {
                this.extensions_being_uninstalled
                    .remove(extension_id.as_ref());
                this.reload(cx)
            })
        })
        .detach_and_log_err(cx)
    }

    /// Updates the set of installed extensions.
    ///
    /// First, this unloads any themes, languages, or grammars that are
    /// no longer in the manifest, or whose files have changed on disk.
    /// Then it loads any themes, languages, or grammars that are newly
    /// added to the manifest, or whose files have changed on disk.
    fn manifest_updated(&mut self, manifest: Manifest, cx: &mut ModelContext<Self>) {
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

        let old_manifest = self.manifest.read();
        let (languages_to_remove, languages_to_add) = diff(
            old_manifest.languages.iter(),
            manifest.languages.iter(),
            &self.extension_changes.languages,
        );
        let (grammars_to_remove, grammars_to_add) = diff(
            old_manifest.grammars.iter(),
            manifest.grammars.iter(),
            &self.extension_changes.grammars,
        );
        let (themes_to_remove, themes_to_add) = diff(
            old_manifest.themes.iter(),
            manifest.themes.iter(),
            &self.extension_changes.themes,
        );
        self.extension_changes.clear();
        drop(old_manifest);

        let themes_to_remove = &themes_to_remove
            .into_iter()
            .map(|theme| theme.into())
            .collect::<Vec<_>>();
        self.theme_registry.remove_user_themes(&themes_to_remove);
        self.language_registry
            .remove_languages(&languages_to_remove, &grammars_to_remove);

        self.language_registry
            .register_wasm_grammars(grammars_to_add.iter().map(|grammar_name| {
                let grammar = manifest.grammars.get(grammar_name).unwrap();
                let mut grammar_path = self.extensions_dir.clone();
                grammar_path.extend([grammar.extension.as_ref(), grammar.path.as_path()]);
                (grammar_name.clone(), grammar_path)
            }));

        for language_name in &languages_to_add {
            let language = manifest.languages.get(language_name.as_ref()).unwrap();
            let mut language_path = self.extensions_dir.clone();
            language_path.extend([language.extension.as_ref(), language.path.as_path()]);
            self.language_registry.register_language(
                language_name.clone(),
                language.grammar.clone(),
                language.matcher.clone(),
                vec![],
                move || {
                    let config = std::fs::read_to_string(language_path.join("config.toml"))?;
                    let config: LanguageConfig = ::toml::from_str(&config)?;
                    let queries = load_plugin_queries(&language_path);
                    Ok((config, queries))
                },
            );
        }

        let (reload_theme_tx, mut reload_theme_rx) = unbounded();
        let fs = self.fs.clone();
        let root_dir = self.extensions_dir.clone();
        let theme_registry = self.theme_registry.clone();
        let themes = themes_to_add
            .iter()
            .filter_map(|name| manifest.themes.get(name).cloned())
            .collect::<Vec<_>>();
        cx.background_executor()
            .spawn(async move {
                for theme in &themes {
                    let mut theme_path = root_dir.clone();
                    theme_path.extend([theme.extension.as_ref(), theme.path.as_path()]);

                    theme_registry
                        .load_user_theme(&theme_path, fs.clone())
                        .await
                        .log_err();
                }

                reload_theme_tx.unbounded_send(()).ok();
            })
            .detach();

        cx.spawn(|_, cx| async move {
            while let Some(_) = reload_theme_rx.next().await {
                if cx
                    .update(|cx| ThemeSettings::reload_current_theme(cx))
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();

        *self.manifest.write() = manifest;
        cx.notify();
    }

    fn watch_extensions_dir(&self, cx: &mut ModelContext<Self>) -> [Task<()>; 2] {
        let manifest = self.manifest.clone();
        let fs = self.fs.clone();
        let extensions_dir = self.extensions_dir.clone();

        let (changes_tx, mut changes_rx) = unbounded();

        let events_task = cx.background_executor().spawn(async move {
            let mut events = fs.watch(&extensions_dir, Duration::from_millis(250)).await;
            while let Some(events) = events.next().await {
                let mut changed_grammars = HashSet::default();
                let mut changed_languages = HashSet::default();
                let mut changed_themes = HashSet::default();

                {
                    let manifest = manifest.read();
                    for event in events {
                        for (grammar_name, grammar) in &manifest.grammars {
                            let mut grammar_path = extensions_dir.clone();
                            grammar_path
                                .extend([grammar.extension.as_ref(), grammar.path.as_path()]);
                            if event.path.starts_with(&grammar_path) || event.path == grammar_path {
                                changed_grammars.insert(grammar_name.clone());
                            }
                        }

                        for (language_name, language) in &manifest.languages {
                            let mut language_path = extensions_dir.clone();
                            language_path
                                .extend([language.extension.as_ref(), language.path.as_path()]);
                            if event.path.starts_with(&language_path) || event.path == language_path
                            {
                                changed_languages.insert(language_name.clone());
                            }
                        }

                        for (theme_name, theme) in &manifest.themes {
                            let mut theme_path = extensions_dir.clone();
                            theme_path.extend([theme.extension.as_ref(), theme.path.as_path()]);
                            if event.path.starts_with(&theme_path) || event.path == theme_path {
                                changed_themes.insert(theme_name.clone());
                            }
                        }
                    }
                }

                changes_tx
                    .unbounded_send(ExtensionChanges {
                        languages: changed_languages,
                        grammars: changed_grammars,
                        themes: changed_themes,
                    })
                    .ok();
            }
        });

        let reload_task = cx.spawn(|this, mut cx| async move {
            while let Some(changes) = changes_rx.next().await {
                if this
                    .update(&mut cx, |this, cx| {
                        this.extension_changes.merge(changes);
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
        let extensions_dir = self.extensions_dir.clone();
        let manifest_path = self.manifest_path.clone();
        self.needs_reload = false;
        self.reload_task = Some(cx.spawn(|this, mut cx| {
            async move {
                let manifest = cx
                    .background_executor()
                    .spawn(async move {
                        let mut manifest = Manifest::default();

                        fs.create_dir(&extensions_dir).await.log_err();

                        let extension_paths = fs.read_dir(&extensions_dir).await;
                        if let Ok(mut extension_paths) = extension_paths {
                            while let Some(extension_dir) = extension_paths.next().await {
                                let Ok(extension_dir) = extension_dir else {
                                    continue;
                                };
                                Self::add_extension_to_manifest(
                                    fs.clone(),
                                    extension_dir,
                                    &mut manifest,
                                )
                                .await
                                .log_err();
                            }
                        }

                        if let Ok(manifest_json) = serde_json::to_string_pretty(&manifest) {
                            fs.save(
                                &manifest_path,
                                &manifest_json.as_str().into(),
                                Default::default(),
                            )
                            .await
                            .context("failed to save extension manifest")
                            .log_err();
                        }

                        manifest
                    })
                    .await;

                this.update(&mut cx, |this, cx| {
                    this.manifest_updated(manifest, cx);
                    this.reload_task.take();
                    if this.needs_reload {
                        this.reload(cx);
                    }
                })
            }
            .log_err()
        }));
    }

    async fn add_extension_to_manifest(
        fs: Arc<dyn Fs>,
        extension_dir: PathBuf,
        manifest: &mut Manifest,
    ) -> Result<()> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("invalid extension name"))?;

        #[derive(Deserialize)]
        struct ExtensionJson {
            pub version: String,
        }

        let extension_json_path = extension_dir.join("extension.json");
        let extension_json = fs
            .load(&extension_json_path)
            .await
            .context("failed to load extension.json")?;
        let extension_json: ExtensionJson =
            serde_json::from_str(&extension_json).context("invalid extension.json")?;

        manifest
            .extensions
            .insert(extension_name.into(), extension_json.version.into());

        if let Ok(mut grammar_paths) = fs.read_dir(&extension_dir.join("grammars")).await {
            while let Some(grammar_path) = grammar_paths.next().await {
                let grammar_path = grammar_path?;
                let Ok(relative_path) = grammar_path.strip_prefix(&extension_dir) else {
                    continue;
                };
                let Some(grammar_name) = grammar_path.file_stem().and_then(OsStr::to_str) else {
                    continue;
                };

                manifest.grammars.insert(
                    grammar_name.into(),
                    GrammarManifestEntry {
                        extension: extension_name.into(),
                        path: relative_path.into(),
                    },
                );
            }
        }

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

                manifest.languages.insert(
                    config.name.clone(),
                    LanguageManifestEntry {
                        extension: extension_name.into(),
                        path: relative_path.into(),
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

                for theme in theme_family.themes {
                    let location = ThemeManifestEntry {
                        extension: extension_name.into(),
                        path: relative_path.into(),
                    };

                    manifest.themes.insert(theme.name.into(), location);
                }
            }
        }

        Ok(())
    }
}

impl ExtensionChanges {
    fn clear(&mut self) {
        self.grammars.clear();
        self.languages.clear();
        self.themes.clear();
    }

    fn merge(&mut self, other: Self) {
        self.grammars.extend(other.grammars);
        self.languages.extend(other.languages);
        self.themes.extend(other.themes);
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
