use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use gpui::{AssetSource, HighlightStyle, SharedString};
use refineable::Refineable;
use util::ResultExt;

use crate::{
    try_parse_color, Appearance, AppearanceContent, PlayerColor, PlayerColors, StatusColors,
    SyntaxTheme, SystemColors, Theme, ThemeColors, ThemeContent, ThemeFamily, ThemeFamilyContent,
    ThemeStyles,
};

#[derive(Debug, Clone)]
pub struct ThemeMeta {
    pub name: SharedString,
    pub appearance: Appearance,
}

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
    pub fn new(assets: Box<dyn AssetSource>) -> Self {
        let mut registry = Self {
            assets,
            themes: HashMap::new(),
        };

        // We're loading our new versions of the One themes by default, as
        // we need them to be loaded for tests.
        //
        // These themes will get overwritten when `load_user_themes` is called
        // when Zed starts, so the One variants used will be the ones ported from Zed1.
        registry.insert_theme_families([crate::one_themes::one_family()]);

        registry
    }

    fn insert_theme_families(&mut self, families: impl IntoIterator<Item = ThemeFamily>) {
        for family in families.into_iter() {
            self.insert_themes(family.themes);
        }
    }

    fn insert_themes(&mut self, themes: impl IntoIterator<Item = Theme>) {
        for theme in themes.into_iter() {
            self.themes.insert(theme.name.clone(), Arc::new(theme));
        }
    }

    #[allow(unused)]
    fn insert_user_theme_families(
        &mut self,
        families: impl IntoIterator<Item = ThemeFamilyContent>,
    ) {
        for family in families.into_iter() {
            self.insert_user_themes(family.themes);
        }
    }

    #[allow(unused)]
    fn insert_user_themes(&mut self, themes: impl IntoIterator<Item = ThemeContent>) {
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
            if !user_theme.style.players.is_empty() {
                player_colors = PlayerColors(
                    user_theme
                        .style
                        .players
                        .into_iter()
                        .map(|player| PlayerColor {
                            cursor: player
                                .cursor
                                .as_ref()
                                .and_then(|color| try_parse_color(&color).ok())
                                .unwrap_or_default(),
                            background: player
                                .background
                                .as_ref()
                                .and_then(|color| try_parse_color(&color).ok())
                                .unwrap_or_default(),
                            selection: player
                                .selection
                                .as_ref()
                                .and_then(|color| try_parse_color(&color).ok())
                                .unwrap_or_default(),
                        })
                        .collect(),
                );
            }

            let mut syntax_colors = match user_theme.appearance {
                AppearanceContent::Light => SyntaxTheme::light(),
                AppearanceContent::Dark => SyntaxTheme::dark(),
            };
            if !user_theme.style.syntax.is_empty() {
                syntax_colors.highlights = user_theme
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
                                    .and_then(|color| try_parse_color(&color).ok()),
                                font_style: highlight.font_style.map(Into::into),
                                font_weight: highlight.font_weight.map(Into::into),
                                ..Default::default()
                            },
                        )
                    })
                    .collect::<Vec<_>>();
            }

            Theme {
                id: uuid::Uuid::new_v4().to_string(),
                name: user_theme.name.into(),
                appearance: match user_theme.appearance {
                    AppearanceContent::Light => Appearance::Light,
                    AppearanceContent::Dark => Appearance::Dark,
                },
                styles: ThemeStyles {
                    system: SystemColors::default(),
                    colors: theme_colors,
                    status: status_colors,
                    player: player_colors,
                    syntax: Arc::new(syntax_colors),
                    accents: Vec::new(),
                },
            }
        }));
    }

    pub fn clear(&mut self) {
        self.themes.clear();
    }

    pub fn list_names(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.keys().cloned()
    }

    pub fn list(&self, _staff: bool) -> impl Iterator<Item = ThemeMeta> + '_ {
        self.themes.values().map(|theme| ThemeMeta {
            name: theme.name.clone(),
            appearance: theme.appearance(),
        })
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
            .cloned()
    }

    pub fn load_user_themes(&mut self) {
        let theme_paths = self
            .assets
            .list("themes/")
            .expect("failed to list theme assets")
            .into_iter()
            .filter(|path| path.ends_with(".json"));

        for path in theme_paths {
            let Some(theme) = self.assets.load(&path).log_err() else {
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

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self::new(Box::new(()))
    }
}
