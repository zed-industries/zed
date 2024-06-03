use std::sync::Arc;
use std::{fmt::Debug, path::Path};

use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, AssetSource, Global, HighlightStyle, SharedString};
use parking_lot::RwLock;
use refineable::Refineable;
use util::ResultExt;

use crate::{
    try_parse_color, AccentColors, Appearance, AppearanceContent, PlayerColors, StatusColors,
    SyntaxTheme, SystemColors, Theme, ThemeColors, ThemeContent, ThemeFamily, ThemeFamilyContent,
    ThemeStyles,
};

#[derive(Debug, Clone)]
pub struct ThemeMeta {
    pub name: SharedString,
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

    pub fn new(assets: Box<dyn AssetSource>) -> Self {
        let registry = Self {
            state: RwLock::new(ThemeRegistryState {
                themes: HashMap::default(),
            }),
            assets,
        };

        // We're loading our new versions of the One themes by default, as
        // we need them to be loaded for tests.
        //
        // These themes will get overwritten when `load_user_themes` is called
        // when Zed starts, so the One variants used will be the ones ported from Zed1.
        registry.insert_theme_families([crate::one_themes::one_family()]);

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
            self.insert_user_themes(family.themes);
        }
    }

    pub fn insert_user_themes(&self, themes: impl IntoIterator<Item = ThemeContent>) {
        self.insert_themes(themes.into_iter().map(|user_theme| {
            let mut theme_colors = match user_theme.appearance {
                AppearanceContent::Light => ThemeColors::light(),
                AppearanceContent::Dark => ThemeColors::dark(),
            };
            theme_colors.refine(&user_theme.style.theme_colors_refinement());

            let mut status_colors = match user_theme.appearance {
                AppearanceContent::Light => StatusColors::light(),
                AppearanceContent::Dark => StatusColors::dark(),
            };
            status_colors.refine(&user_theme.style.status_colors_refinement());

            let mut player_colors = match user_theme.appearance {
                AppearanceContent::Light => PlayerColors::light(),
                AppearanceContent::Dark => PlayerColors::dark(),
            };
            player_colors.merge(&user_theme.style.players);

            let mut accent_colors = match user_theme.appearance {
                AppearanceContent::Light => AccentColors::light(),
                AppearanceContent::Dark => AccentColors::dark(),
            };
            accent_colors.merge(&user_theme.style.accents);

            let syntax_highlights = user_theme
                .style
                .syntax
                .iter()
                .map(|(syntax_token, highlight)| {
                    (
                        syntax_token.clone(),
                        HighlightStyle {
                            color: highlight
                                .color
                                .as_ref()
                                .and_then(|color| try_parse_color(color).ok()),
                            background_color: highlight
                                .background_color
                                .as_ref()
                                .and_then(|color| try_parse_color(color).ok()),
                            font_style: highlight.font_style.map(Into::into),
                            font_weight: highlight.font_weight.map(Into::into),
                            ..Default::default()
                        },
                    )
                })
                .collect::<Vec<_>>();
            let syntax_theme =
                SyntaxTheme::merge(Arc::new(SyntaxTheme::default()), syntax_highlights);

            let window_background_appearance = user_theme
                .style
                .window_background_appearance
                .map(Into::into)
                .unwrap_or_default();

            Theme {
                id: uuid::Uuid::new_v4().to_string(),
                name: user_theme.name.into(),
                appearance: match user_theme.appearance {
                    AppearanceContent::Light => Appearance::Light,
                    AppearanceContent::Dark => Appearance::Dark,
                },
                styles: ThemeStyles {
                    system: SystemColors::default(),
                    window_background_appearance,
                    accents: accent_colors,
                    colors: theme_colors,
                    status: status_colors,
                    player: player_colors,
                    syntax: syntax_theme,
                },
            }
        }));
    }

    /// Removes the themes with the given names from the registry.
    pub fn remove_user_themes(&self, themes_to_remove: &[SharedString]) {
        self.state
            .write()
            .themes
            .retain(|name, _| !themes_to_remove.contains(name))
    }

    pub fn clear(&mut self) {
        self.state.write().themes.clear();
    }

    pub fn list_names(&self, _staff: bool) -> Vec<SharedString> {
        let mut names = self.state.read().themes.keys().cloned().collect::<Vec<_>>();
        names.sort();
        names
    }

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

    pub async fn read_user_theme(theme_path: &Path, fs: Arc<dyn Fs>) -> Result<ThemeFamilyContent> {
        let reader = fs.open_sync(theme_path).await?;
        let theme = serde_json_lenient::from_reader(reader)?;

        Ok(theme)
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
