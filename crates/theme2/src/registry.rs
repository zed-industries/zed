use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use gpui::SharedString;
use refineable::Refineable;

use crate::{
    zed_pro_family, Appearance, GitStatusColors, PlayerColors, StatusColors, SyntaxTheme,
    SystemColors, Theme, ThemeColors, ThemeFamily, ThemeStyles, UserTheme, UserThemeFamily,
};

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
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

    fn insert_user_theme_familes(&mut self, families: impl IntoIterator<Item = UserThemeFamily>) {
        for family in families.into_iter() {
            self.insert_user_themes(family.themes);
        }
    }

    fn insert_user_themes(&mut self, themes: impl IntoIterator<Item = UserTheme>) {
        self.insert_themes(themes.into_iter().map(|user_theme| {
            let mut theme_colors = match user_theme.appearance {
                Appearance::Light => ThemeColors::default_light(),
                Appearance::Dark => ThemeColors::default_dark(),
            };

            theme_colors.refine(&user_theme.styles.colors);

            Theme {
                id: uuid::Uuid::new_v4().to_string(),
                name: user_theme.name.into(),
                appearance: user_theme.appearance,
                styles: ThemeStyles {
                    system: SystemColors::default(),
                    colors: theme_colors,
                    status: StatusColors::default(),
                    git: GitStatusColors::default(),
                    player: PlayerColors::default(),
                    syntax: match user_theme.appearance {
                        Appearance::Light => SyntaxTheme::default_light(),
                        Appearance::Dark => SyntaxTheme::default_dark(),
                    },
                },
            }
        }));
    }

    pub fn list_names(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.keys().cloned()
    }

    pub fn list(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.values().map(|theme| theme.name.clone())
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        self.themes
            .get(name)
            .ok_or_else(|| anyhow!("theme not found: {}", name))
            .cloned()
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        let mut this = Self {
            themes: HashMap::default(),
        };

        this.insert_theme_families([zed_pro_family()]);
        this.insert_user_theme_familes(crate::all_user_themes());

        this
    }
}
