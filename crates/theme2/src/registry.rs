use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use gpui::{HighlightStyle, SharedString};
use refineable::Refineable;

use crate::{
    Appearance, PlayerColors, StatusColors, SyntaxTheme, SystemColors, Theme, ThemeColors,
    ThemeFamily, ThemeStyles, UserTheme, UserThemeFamily,
};

#[derive(Debug, Clone)]
pub struct ThemeMeta {
    pub name: SharedString,
    pub appearance: Appearance,
}

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
    #[allow(unused)]
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
    fn insert_user_theme_familes(&mut self, families: impl IntoIterator<Item = UserThemeFamily>) {
        for family in families.into_iter() {
            self.insert_user_themes(family.themes);
        }
    }

    #[allow(unused)]
    fn insert_user_themes(&mut self, themes: impl IntoIterator<Item = UserTheme>) {
        self.insert_themes(themes.into_iter().map(|user_theme| {
            let mut theme_colors = match user_theme.appearance {
                Appearance::Light => ThemeColors::light(),
                Appearance::Dark => ThemeColors::dark(),
            };
            theme_colors.refine(&user_theme.styles.colors);

            let mut status_colors = StatusColors::dark();
            status_colors.refine(&user_theme.styles.status);

            let mut syntax_colors = match user_theme.appearance {
                Appearance::Light => SyntaxTheme::light(),
                Appearance::Dark => SyntaxTheme::dark(),
            };
            if let Some(user_syntax) = user_theme.styles.syntax {
                syntax_colors.highlights = user_syntax
                    .highlights
                    .iter()
                    .map(|(syntax_token, highlight)| {
                        (
                            syntax_token.clone(),
                            HighlightStyle {
                                color: highlight.color,
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
                appearance: user_theme.appearance,
                styles: ThemeStyles {
                    system: SystemColors::default(),
                    colors: theme_colors,
                    status: status_colors,
                    player: match user_theme.appearance {
                        Appearance::Light => PlayerColors::light(),
                        Appearance::Dark => PlayerColors::dark(),
                    },
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
        #[cfg(not(feature = "importing-themes"))]
        self.insert_user_theme_familes(crate::all_user_themes());
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        Self {
            themes: HashMap::default(),
        }
    }
}
