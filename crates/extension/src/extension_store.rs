use anyhow::{Context as _, Result};
use collections::HashMap;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{actions, AppContext, Context, Global, Model, ModelContext, Task};
use language::{
    LanguageConfig, LanguageMatcher, LanguageQueries, LanguageRegistry, QUERY_FILENAME_PREFIXES,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use theme::{ThemeRegistry, ThemeSettings};
use util::{paths::EXTENSIONS_DIR, ResultExt};

#[cfg(test)]
mod extension_store_test;

pub struct ExtensionStore {
    manifest: Arc<RwLock<Manifest>>,
    fs: Arc<dyn Fs>,
    extensions_dir: PathBuf,
    manifest_path: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    _watch_extensions_dir: [Task<()>; 2],
}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

#[derive(Deserialize, Serialize, Default)]
pub struct Manifest {
    pub grammars: HashMap<Arc<str>, GrammarManifestEntry>,
    pub languages: HashMap<Arc<str>, LanguageManifestEntry>,
    pub themes: HashMap<String, ThemeManifestEntry>,
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

actions!(zed, [ReloadExtensions]);

pub fn init(
    fs: Arc<fs::RealFs>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            fs.clone(),
            language_registry.clone(),
            theme_registry,
            cx,
        )
    });

    cx.on_action(|_: &ReloadExtensions, cx| {
        let store = cx.global::<GlobalExtensionStore>().0.clone();
        store
            .update(cx, |store, cx| store.reload(cx))
            .detach_and_log_err(cx);
    });

    cx.set_global(GlobalExtensionStore(store));
}

impl ExtensionStore {
    pub fn new(
        extensions_dir: PathBuf,
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut this = Self {
            manifest: Default::default(),
            extensions_dir: extensions_dir.join("installed"),
            manifest_path: extensions_dir.join("manifest.json"),
            fs,
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
            self.reload(cx).detach_and_log_err(cx);
        }
    }

    fn manifest_updated(&mut self, manifest: Manifest, cx: &mut ModelContext<Self>) {
        for (grammar_name, grammar) in &manifest.grammars {
            let mut grammar_path = self.extensions_dir.clone();
            grammar_path.extend([grammar.extension.as_ref(), grammar.path.as_path()]);
            self.language_registry
                .register_grammar(grammar_name.clone(), grammar_path);
        }
        for (language_name, language) in &manifest.languages {
            let mut language_path = self.extensions_dir.clone();
            language_path.extend([language.extension.as_ref(), language.path.as_path()]);
            self.language_registry.register_extension(
                language_path.into(),
                language_name.clone(),
                language.grammar.clone(),
                language.matcher.clone(),
                load_plugin_queries,
            );
        }
        let fs = self.fs.clone();
        let root_dir = self.extensions_dir.clone();
        let theme_registry = self.theme_registry.clone();
        let themes = manifest.themes.clone();
        cx.background_executor()
            .spawn(async move {
                for theme in themes.values() {
                    let mut theme_path = root_dir.clone();
                    theme_path.extend([theme.extension.as_ref(), theme.path.as_path()]);

                    theme_registry
                        .load_user_theme(&theme_path, fs.clone())
                        .await
                        .log_err();
                }
            })
            .detach();
        *self.manifest.write() = manifest;
    }

    fn watch_extensions_dir(&self, cx: &mut ModelContext<Self>) -> [Task<()>; 2] {
        let manifest = self.manifest.clone();
        let fs = self.fs.clone();
        let language_registry = self.language_registry.clone();
        let theme_registry = self.theme_registry.clone();
        let extensions_dir = self.extensions_dir.clone();

        let (reload_theme_tx, mut reload_theme_rx) = futures::channel::mpsc::unbounded();

        let events_task = cx.background_executor().spawn(async move {
            let mut events = fs.watch(&extensions_dir, Duration::from_millis(250)).await;
            while let Some(events) = events.next().await {
                let mut changed_grammars = Vec::default();
                let mut changed_languages = Vec::default();
                let mut changed_themes = Vec::default();

                {
                    let manifest = manifest.read();
                    for event in events {
                        for (grammar_name, grammar) in &manifest.grammars {
                            let mut grammar_path = extensions_dir.clone();
                            grammar_path
                                .extend([grammar.extension.as_ref(), grammar.path.as_path()]);
                            if event.path.starts_with(&grammar_path) || event.path == grammar_path {
                                changed_grammars.push(grammar_name.clone());
                            }
                        }

                        for (language_name, language) in &manifest.languages {
                            let mut language_path = extensions_dir.clone();
                            language_path
                                .extend([language.extension.as_ref(), language.path.as_path()]);
                            if event.path.starts_with(&language_path) || event.path == language_path
                            {
                                changed_languages.push(language_name.clone());
                            }
                        }

                        for (_theme_name, theme) in &manifest.themes {
                            let mut theme_path = extensions_dir.clone();
                            theme_path.extend([theme.extension.as_ref(), theme.path.as_path()]);
                            if event.path.starts_with(&theme_path) || event.path == theme_path {
                                changed_themes.push(theme_path.clone());
                            }
                        }
                    }
                }

                language_registry.reload_languages(&changed_languages, &changed_grammars);

                for theme_path in &changed_themes {
                    theme_registry
                        .load_user_theme(&theme_path, fs.clone())
                        .await
                        .context("failed to load user theme")
                        .log_err();
                }

                if !changed_themes.is_empty() {
                    reload_theme_tx.unbounded_send(()).ok();
                }
            }
        });

        let reload_theme_task = cx.spawn(|_, cx| async move {
            while let Some(_) = reload_theme_rx.next().await {
                if cx
                    .update(|cx| ThemeSettings::reload_current_theme(cx))
                    .is_err()
                {
                    break;
                }
            }
        });

        [events_task, reload_theme_task]
    }

    pub fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        let extensions_dir = self.extensions_dir.clone();
        let manifest_path = self.manifest_path.clone();
        cx.spawn(|this, mut cx| async move {
            let manifest = cx
                .background_executor()
                .spawn(async move {
                    let mut manifest = Manifest::default();

                    let mut extension_paths = fs
                        .read_dir(&extensions_dir)
                        .await
                        .context("failed to read extensions directory")?;
                    while let Some(extension_dir) = extension_paths.next().await {
                        let extension_dir = extension_dir?;
                        let Some(extension_name) =
                            extension_dir.file_name().and_then(OsStr::to_str)
                        else {
                            continue;
                        };

                        if let Ok(mut grammar_paths) =
                            fs.read_dir(&extension_dir.join("grammars")).await
                        {
                            while let Some(grammar_path) = grammar_paths.next().await {
                                let grammar_path = grammar_path?;
                                let Ok(relative_path) = grammar_path.strip_prefix(&extension_dir)
                                else {
                                    continue;
                                };
                                let Some(grammar_name) =
                                    grammar_path.file_stem().and_then(OsStr::to_str)
                                else {
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

                        if let Ok(mut language_paths) =
                            fs.read_dir(&extension_dir.join("languages")).await
                        {
                            while let Some(language_path) = language_paths.next().await {
                                let language_path = language_path?;
                                let Ok(relative_path) = language_path.strip_prefix(&extension_dir)
                                else {
                                    continue;
                                };
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

                        if let Ok(mut theme_paths) =
                            fs.read_dir(&extension_dir.join("themes")).await
                        {
                            while let Some(theme_path) = theme_paths.next().await {
                                let theme_path = theme_path?;
                                let Ok(relative_path) = theme_path.strip_prefix(&extension_dir)
                                else {
                                    continue;
                                };

                                let Some(theme_family) =
                                    ThemeRegistry::read_user_theme(&theme_path, fs.clone())
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

                                    manifest.themes.insert(theme.name, location);
                                }
                            }
                        }
                    }

                    fs.save(
                        &manifest_path,
                        &serde_json::to_string_pretty(&manifest)?.as_str().into(),
                        Default::default(),
                    )
                    .await
                    .context("failed to save extension manifest")?;

                    anyhow::Ok(manifest)
                })
                .await?;
            this.update(&mut cx, |this, cx| this.manifest_updated(manifest, cx))
        })
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
