use std::sync::Arc;
use std::{fmt::Debug, path::Path};

use anyhow::{Context as _, Result};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use fs::Fs;
use futures::StreamExt;
use gpui::{App, AssetSource, Global, SharedString};
use parking_lot::RwLock;
use thiserror::Error;
use util::ResultExt;

use crate::{
    Appearance, AppearanceContent, ChevronIcons, DEFAULT_ICON_THEME_NAME, DirectoryIcons,
    IconDefinition, IconTheme, Theme, ThemeFamily, ThemeFamilyContent, default_icon_theme,
    read_icon_theme, read_user_theme, refine_theme_family,
};

/// The metadata for a theme.
#[derive(Debug, Clone)]
pub struct ThemeMeta {
    /// The name of the theme.
    pub name: SharedString,
    /// The appearance of the theme.
    pub appearance: Appearance,
}

/// An error indicating that the theme with the given name was not found.
#[derive(Debug, Error, Clone)]
#[error("theme not found: {0}")]
pub struct ThemeNotFoundError(pub SharedString);

/// An error indicating that the icon theme with the given name was not found.
#[derive(Debug, Error, Clone)]
#[error("icon theme not found: {0}")]
pub struct IconThemeNotFoundError(pub SharedString);

/// The global [`ThemeRegistry`].
///
/// This newtype exists for obtaining a unique [`TypeId`](std::any::TypeId) when
/// inserting the [`ThemeRegistry`] into the context as a global.
///
/// This should not be exposed outside of this module.
#[derive(Default, Deref, DerefMut)]
struct GlobalThemeRegistry(Arc<ThemeRegistry>);

impl Global for GlobalThemeRegistry {}

struct ThemeRegistryState {
    themes: HashMap<SharedString, Arc<Theme>>,
    icon_themes: HashMap<SharedString, Arc<IconTheme>>,
    /// Whether the extensions have been loaded yet.
    extensions_loaded: bool,
}

/// The registry for themes.
pub struct ThemeRegistry {
    state: RwLock<ThemeRegistryState>,
    assets: Box<dyn AssetSource>,
}

impl ThemeRegistry {
    /// Returns the global [`ThemeRegistry`].
    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalThemeRegistry>().0.clone()
    }

    /// Returns the global [`ThemeRegistry`].
    ///
    /// Inserts a default [`ThemeRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalThemeRegistry>().0.clone()
    }

    /// Sets the global [`ThemeRegistry`].
    pub(crate) fn set_global(assets: Box<dyn AssetSource>, cx: &mut App) {
        cx.set_global(GlobalThemeRegistry(Arc::new(ThemeRegistry::new(assets))));
    }

    /// Creates a new [`ThemeRegistry`] with the given [`AssetSource`].
    pub fn new(assets: Box<dyn AssetSource>) -> Self {
        let registry = Self {
            state: RwLock::new(ThemeRegistryState {
                themes: HashMap::default(),
                icon_themes: HashMap::default(),
                extensions_loaded: false,
            }),
            assets,
        };

        // We're loading the Zed default theme, as we need a theme to be loaded
        // for tests.
        registry.insert_theme_families([crate::fallback_themes::zed_default_themes()]);

        let default_icon_theme = crate::default_icon_theme();
        registry
            .state
            .write()
            .icon_themes
            .insert(default_icon_theme.name.clone(), default_icon_theme);

        registry
    }

    /// Returns whether the extensions have been loaded.
    pub fn extensions_loaded(&self) -> bool {
        self.state.read().extensions_loaded
    }

    /// Sets the flag indicating that the extensions have loaded.
    pub fn set_extensions_loaded(&self) {
        self.state.write().extensions_loaded = true;
    }

    fn insert_theme_families(&self, families: impl IntoIterator<Item = ThemeFamily>) {
        for family in families.into_iter() {
            self.insert_themes(family.themes);
        }
    }

    fn insert_themes(&self, themes: impl IntoIterator<Item = Theme>) {
        let mut state = self.state.write();
        for theme in themes.into_iter() {
            state.themes.insert(theme.name.clone(), Arc::new(theme));
        }
    }

    #[allow(unused)]
    fn insert_user_theme_families(&self, families: impl IntoIterator<Item = ThemeFamilyContent>) {
        for family in families.into_iter() {
            let refined_family = refine_theme_family(family);

            self.insert_themes(refined_family.themes);
        }
    }

    /// Removes the themes with the given names from the registry.
    pub fn remove_user_themes(&self, themes_to_remove: &[SharedString]) {
        self.state
            .write()
            .themes
            .retain(|name, _| !themes_to_remove.contains(name))
    }

    /// Removes all themes from the registry.
    pub fn clear(&self) {
        self.state.write().themes.clear();
    }

    /// Returns the names of all themes in the registry.
    pub fn list_names(&self) -> Vec<SharedString> {
        let mut names = self.state.read().themes.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    /// Returns the metadata of all themes in the registry.
    pub fn list(&self) -> Vec<ThemeMeta> {
        self.state
            .read()
            .themes
            .values()
            .map(|theme| ThemeMeta {
                name: theme.name.clone(),
                appearance: theme.appearance(),
            })
            .collect()
    }

    /// Returns the theme with the given name.
    pub fn get(&self, name: &str) -> Result<Arc<Theme>, ThemeNotFoundError> {
        self.state
            .read()
            .themes
            .get(name)
            .ok_or_else(|| ThemeNotFoundError(name.to_string().into()))
            .cloned()
    }

    /// Loads the themes bundled with the Zed binary and adds them to the registry.
    pub fn load_bundled_themes(&self) {
        let theme_paths = self
            .assets
            .list("themes/")
            .expect("failed to list theme assets")
            .into_iter()
            .filter(|path| path.ends_with(".json"));

        for path in theme_paths {
            let Some(theme) = self.assets.load(&path).log_err().flatten() else {
                continue;
            };

            let Some(theme_family) = serde_json::from_slice(&theme)
                .with_context(|| format!("failed to parse theme at path \"{path}\""))
                .log_err()
            else {
                continue;
            };

            self.insert_user_theme_families([theme_family]);
        }
    }

    /// Loads the user themes from the specified directory and adds them to the registry.
    pub async fn load_user_themes(&self, themes_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
        let mut theme_paths = fs
            .read_dir(themes_path)
            .await
            .with_context(|| format!("reading themes from {themes_path:?}"))?;

        while let Some(theme_path) = theme_paths.next().await {
            let Some(theme_path) = theme_path.log_err() else {
                continue;
            };

            self.load_user_theme(&theme_path, fs.clone())
                .await
                .log_err();
        }

        Ok(())
    }

    /// Loads the user theme from the specified path and adds it to the registry.
    pub async fn load_user_theme(&self, theme_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
        let theme = read_user_theme(theme_path, fs).await?;

        self.insert_user_theme_families([theme]);

        Ok(())
    }

    /// Returns the default icon theme.
    pub fn default_icon_theme(&self) -> Result<Arc<IconTheme>, IconThemeNotFoundError> {
        self.get_icon_theme(DEFAULT_ICON_THEME_NAME)
    }

    /// Returns the metadata of all icon themes in the registry.
    pub fn list_icon_themes(&self) -> Vec<ThemeMeta> {
        self.state
            .read()
            .icon_themes
            .values()
            .map(|theme| ThemeMeta {
                name: theme.name.clone(),
                appearance: theme.appearance,
            })
            .collect()
    }

    /// Returns the icon theme with the specified name.
    pub fn get_icon_theme(&self, name: &str) -> Result<Arc<IconTheme>, IconThemeNotFoundError> {
        self.state
            .read()
            .icon_themes
            .get(name)
            .ok_or_else(|| IconThemeNotFoundError(name.to_string().into()))
            .cloned()
    }

    /// Removes the icon themes with the given names from the registry.
    pub fn remove_icon_themes(&self, icon_themes_to_remove: &[SharedString]) {
        self.state
            .write()
            .icon_themes
            .retain(|name, _| !icon_themes_to_remove.contains(name))
    }

    /// Loads the icon theme from the specified path and adds it to the registry.
    ///
    /// The `icons_root_dir` parameter indicates the root directory from which
    /// the relative paths to icons in the theme should be resolved against.
    pub async fn load_icon_theme(
        &self,
        icon_theme_path: &Path,
        icons_root_dir: &Path,
        fs: Arc<dyn Fs>,
    ) -> Result<()> {
        let icon_theme_family = read_icon_theme(icon_theme_path, fs).await?;

        let resolve_icon_path = |path: SharedString| {
            icons_root_dir
                .join(path.as_ref())
                .to_string_lossy()
                .to_string()
                .into()
        };

        let default_icon_theme = default_icon_theme();

        let mut state = self.state.write();
        for icon_theme in icon_theme_family.themes {
            let mut file_stems = default_icon_theme.file_stems.clone();
            file_stems.extend(icon_theme.file_stems);

            let mut file_suffixes = default_icon_theme.file_suffixes.clone();
            file_suffixes.extend(icon_theme.file_suffixes);

            let mut named_directory_icons = default_icon_theme.named_directory_icons.clone();
            named_directory_icons.extend(icon_theme.named_directory_icons.into_iter().map(
                |(key, value)| {
                    (
                        key,
                        DirectoryIcons {
                            collapsed: value.collapsed.map(resolve_icon_path),
                            expanded: value.expanded.map(resolve_icon_path),
                        },
                    )
                },
            ));

            let icon_theme = IconTheme {
                id: uuid::Uuid::new_v4().to_string(),
                name: icon_theme.name.into(),
                appearance: match icon_theme.appearance {
                    AppearanceContent::Light => Appearance::Light,
                    AppearanceContent::Dark => Appearance::Dark,
                },
                directory_icons: DirectoryIcons {
                    collapsed: icon_theme.directory_icons.collapsed.map(resolve_icon_path),
                    expanded: icon_theme.directory_icons.expanded.map(resolve_icon_path),
                },
                named_directory_icons,
                chevron_icons: ChevronIcons {
                    collapsed: icon_theme.chevron_icons.collapsed.map(resolve_icon_path),
                    expanded: icon_theme.chevron_icons.expanded.map(resolve_icon_path),
                },
                file_stems,
                file_suffixes,
                file_icons: icon_theme
                    .file_icons
                    .into_iter()
                    .map(|(key, icon)| {
                        (
                            key,
                            IconDefinition {
                                path: resolve_icon_path(icon.path),
                            },
                        )
                    })
                    .collect(),
            };

            state
                .icon_themes
                .insert(icon_theme.name.clone(), Arc::new(icon_theme));
        }

        Ok(())
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new(Box::new(()))
    }
}
