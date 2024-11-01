use std::sync::Arc;
use std::{fmt::Debug, path::Path};

use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, AssetSource, Global, SharedString};
use parking_lot::RwLock;
use util::ResultExt;

use crate::{refine_theme_family, Appearance, Theme, ThemeFamily, ThemeFamilyContent};

/// The metadata for a theme.
#[derive(Debug, Clone)]
pub struct ThemeMeta {
    /// The name of the theme.
    pub name: SharedString,
    /// The appearance of the theme.
    pub appearance: Appearance,
}

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
}

/// The registry for themes.
pub struct ThemeRegistry {
    state: RwLock<ThemeRegistryState>,
    assets: Box<dyn AssetSource>,
}

impl ThemeRegistry {
    /// Returns the global [`ThemeRegistry`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        cx.global::<GlobalThemeRegistry>().0.clone()
    }

    /// Returns the global [`ThemeRegistry`].
    ///
    /// Inserts a default [`ThemeRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalThemeRegistry>().0.clone()
    }

    /// Sets the global [`ThemeRegistry`].
    pub(crate) fn set_global(assets: Box<dyn AssetSource>, cx: &mut AppContext) {
        cx.set_global(GlobalThemeRegistry(Arc::new(ThemeRegistry::new(assets))));
    }

    /// Creates a new [`ThemeRegistry`] with the given [`AssetSource`].
    pub fn new(assets: Box<dyn AssetSource>) -> Self {
        let registry = Self {
            state: RwLock::new(ThemeRegistryState {
                themes: HashMap::default(),
            }),
            assets,
        };

        // We're loading the Zed default theme, as we need a theme to be loaded
        // for tests.
        registry.insert_theme_families([crate::fallback_themes::zed_default_themes()]);

        registry
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
    pub fn list_names(&self, _staff: bool) -> Vec<SharedString> {
        let mut names = self.state.read().themes.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    /// Returns the metadata of all themes in the registry.
    pub fn list(&self, _staff: bool) -> Vec<ThemeMeta> {
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
    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.state
            .read()
            .themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
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

    /// Asynchronously reads the user theme from the specified path.
    pub async fn read_user_theme(theme_path: &Path, fs: Arc<dyn Fs>) -> Result<ThemeFamilyContent> {
        let reader = fs.open_sync(theme_path).await?;
        let theme_family: ThemeFamilyContent = serde_json_lenient::from_reader(reader)?;

        for theme in &theme_family.themes {
            if theme
                .style
                .colors
                .deprecated_scrollbar_thumb_background
                .is_some()
            {
                log::warn!(
                    r#"Theme "{theme_name}" is using a deprecated style property: scrollbar_thumb.background. Use `scrollbar.thumb.background` instead."#,
                    theme_name = theme.name
                )
            }
        }

        Ok(theme_family)
    }

    /// Loads the user theme from the specified path and adds it to the registry.
    pub async fn load_user_theme(&self, theme_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
        let theme = Self::read_user_theme(theme_path, fs).await?;

        self.insert_user_theme_families([theme]);

        Ok(())
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new(Box::new(()))
    }
}
