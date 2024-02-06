use anyhow::Result;
use collections::HashMap;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{AppContext, Context, Global, Model, ModelContext, Task};
use language::{LanguageConfig, LanguageRegistry};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, path::PathBuf, sync::Arc};
use theme::ThemeRegistry;
use util::{paths::EXTENSIONS_DIR, ResultExt};

#[cfg(test)]
mod extension_store_test;

pub struct ExtensionStore {
    manifest: Manifest,
    fs: Arc<dyn Fs>,
    root_dir: PathBuf,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
}

struct GlobalExtensionStore(Model<ExtensionStore>);

impl Global for GlobalExtensionStore {}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GrammarLocation {
    extension: String,
    grammar_name: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct LanguageLocation {
    extension: String,
    language_dir: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct ThemeLocation {
    extension: String,
    filename: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Manifest {
    pub grammars: Vec<GrammarLocation>,
    pub languages_by_path_suffix: HashMap<String, LanguageLocation>,
    pub languages_by_name: HashMap<String, LanguageLocation>,
    pub themes_by_name: HashMap<String, ThemeLocation>,
}

pub fn init(
    fs: Arc<fs::RealFs>,
    language_registry: Arc<LanguageRegistry>,
    theme_registry: Arc<ThemeRegistry>,
    cx: &mut AppContext,
) {
    let store = cx.new_model(|cx| {
        let mut store = ExtensionStore::new(
            EXTENSIONS_DIR.clone(),
            fs,
            language_registry,
            theme_registry,
            cx,
        );
        store.load(cx);
        store
    });

    cx.set_global(GlobalExtensionStore(store));
}

impl ExtensionStore {
    pub fn new(
        root_dir: PathBuf,
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        theme_registry: Arc<ThemeRegistry>,
        cx: &mut AppContext,
    ) -> Self {
        Self {
            manifest: Manifest::default(),
            root_dir,
            fs,
            language_registry,
            theme_registry,
        }
    }

    pub fn load(&mut self, cx: &mut ModelContext<Self>) -> Result<()> {
        let manifest = cx
            .background_executor()
            .block(self.fs.load(&self.root_dir.join("manifest.json")))?;
        self.manifest = serde_json::from_str(&manifest)?;

        // add languages to registry, load themes in background

        Ok(())
    }

    pub fn rebuild_manifest(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        let root_dir = self.root_dir.clone();
        cx.spawn(|this, mut cx| async move {
            let manifest = cx
                .background_executor()
                .spawn(async move {
                    let mut manifest = Manifest::default();

                    let extensions_dir = root_dir.join("extensions");

                    let mut extension_paths = fs.read_dir(&extensions_dir).await?;
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
                                let Some(grammar_name) =
                                    grammar_path.file_stem().and_then(OsStr::to_str)
                                else {
                                    continue;
                                };

                                manifest.grammars.push(GrammarLocation {
                                    extension: extension_name.into(),
                                    grammar_name: grammar_name.into(),
                                });
                            }
                        }

                        if let Ok(mut language_paths) =
                            fs.read_dir(&extension_dir.join("languages")).await
                        {
                            while let Some(language_path) = language_paths.next().await {
                                let language_path = language_path?;
                                let Some(dir_name) =
                                    language_path.file_name().and_then(OsStr::to_str)
                                else {
                                    continue;
                                };
                                let location = LanguageLocation {
                                    extension: extension_name.into(),
                                    language_dir: dir_name.into(),
                                };
                                let config = fs.load(&language_path.join("config.toml")).await?;
                                let config = ::toml::from_str::<LanguageConfig>(&config)?;
                                for suffix in config.path_suffixes {
                                    manifest
                                        .languages_by_path_suffix
                                        .insert(suffix, location.clone());
                                }
                                manifest
                                    .languages_by_name
                                    .insert(config.name.to_string(), location);
                            }
                        }

                        if let Ok(mut theme_paths) =
                            fs.read_dir(&extension_dir.join("themes")).await
                        {
                            while let Some(theme_path) = theme_paths.next().await {
                                let theme_path = theme_path?;
                                let Some(theme_filename) =
                                    theme_path.file_name().and_then(OsStr::to_str)
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
                                    let location = ThemeLocation {
                                        extension: extension_name.into(),
                                        filename: theme_filename.into(),
                                    };

                                    manifest.themes_by_name.insert(theme.name, location);
                                }
                            }
                        }
                    }

                    manifest.grammars.sort();

                    anyhow::Ok(manifest)
                })
                .await?;
            this.update(&mut cx, |this, cx| this.manifest = manifest)
        })
    }
}
