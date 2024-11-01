use std::sync::Arc;
use std::{fmt::Debug, path::Path};

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, AssetSource, Global, SharedString};
use parking_lot::RwLock;
use util::ResultExt;

use crate::{
    read_user_theme, refine_theme_family, Appearance, Theme, ThemeFamily, ThemeFamilyContent,
};

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
#[derive(Deref, DerefMut)]
struct GlobalThemeRegistry(Arc<dyn ThemeRegistry>);

impl Global for GlobalThemeRegistry {}

/// A registry for themes.
#[async_trait]
pub trait ThemeRegistry: Send + Sync + 'static {
    /// Returns the names of all themes in the registry.
    fn list_names(&self) -> Vec<SharedString>;

    /// Returns the metadata of all themes in the registry.
    fn list(&self) -> Vec<ThemeMeta>;

    /// Returns the theme with the given name.
    fn get(&self, name: &str) -> Result<Arc<Theme>>;

    /// Loads the user theme from the specified path and adds it to the registry.
    async fn load_user_theme(&self, theme_path: &Path, fs: Arc<dyn Fs>) -> Result<()>;

    /// Loads the user themes from the specified directory and adds them to the registry.
    async fn load_user_themes(&self, themes_path: &Path, fs: Arc<dyn Fs>) -> Result<()>;

    /// Removes the themes with the given names from the registry.
    fn remove_user_themes(&self, themes_to_remove: &[SharedString]);
}

impl dyn ThemeRegistry {
    /// Returns the global [`ThemeRegistry`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        cx.global::<GlobalThemeRegistry>().0.clone()
    }

    /// Returns the global [`ThemeRegistry`].
    ///
    /// Inserts a default [`ThemeRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        if let Some(registry) = cx.try_global::<GlobalThemeRegistry>() {
            return registry.0.clone();
        }

        let registry = Arc::new(RealThemeRegistry::default());
        cx.set_global(GlobalThemeRegistry(registry.clone()));

        registry
    }
}

struct RealThemeRegistryState {
    themes: HashMap<SharedString, Arc<Theme>>,
}

/// The registry for themes.
pub struct RealThemeRegistry {
    state: RwLock<RealThemeRegistryState>,
    assets: Box<dyn AssetSource>,
}

impl RealThemeRegistry {
    /// Sets the global [`ThemeRegistry`].
    pub(crate) fn set_global(self: Arc<Self>, cx: &mut AppContext) {
        cx.set_global(GlobalThemeRegistry(self));
    }

    /// Creates a new [`ThemeRegistry`] with the given [`AssetSource`].
    pub fn new(assets: Box<dyn AssetSource>) -> Self {
        let registry = Self {
            state: RwLock::new(RealThemeRegistryState {
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

    /// Removes all themes from the registry.
    pub fn clear(&self) {
        self.state.write().themes.clear();
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
}

impl Default for RealThemeRegistry {
    fn default() -> Self {
        Self::new(Box::new(()))
    }
}

#[async_trait]
impl ThemeRegistry for RealThemeRegistry {
    fn list_names(&self) -> Vec<SharedString> {
        let mut names = self.state.read().themes.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

    fn list(&self) -> Vec<ThemeMeta> {
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

    fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.state
            .read()
            .themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
            .cloned()
    }

    async fn load_user_theme(&self, theme_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
        let theme = read_user_theme(theme_path, fs).await?;

        self.insert_user_theme_families([theme]);

        Ok(())
    }

    async fn load_user_themes(&self, themes_path: &Path, fs: Arc<dyn Fs>) -> Result<()> {
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

    fn remove_user_themes(&self, themes_to_remove: &[SharedString]) {
        self.state
            .write()
            .themes
            .retain(|name, _| !themes_to_remove.contains(name))
    }
}

/// A theme registry that doesn't have any behavior.
pub struct VoidThemeRegistry;

#[async_trait]
impl ThemeRegistry for VoidThemeRegistry {
    fn list_names(&self) -> Vec<SharedString> {
        Vec::new()
    }

    fn list(&self) -> Vec<ThemeMeta> {
        Vec::new()
    }

    fn get(&self, name: &str) -> Result<Arc<Theme>> {
        bail!("cannot retrieve theme {name:?} from a void theme registry")
    }

    async fn load_user_theme(&self, _theme_path: &Path, _fs: Arc<dyn Fs>) -> Result<()> {
        Ok(())
    }

    async fn load_user_themes(&self, _themes_path: &Path, _fs: Arc<dyn Fs>) -> Result<()> {
        Ok(())
    }

    fn remove_user_themes(&self, _themes_to_remove: &[SharedString]) {}
}
