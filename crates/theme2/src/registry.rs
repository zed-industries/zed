use crate::{zed_pro_family, ThemeFamily, ThemeVariant};
use anyhow::{anyhow, Result};
use gpui2::SharedString;
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<ThemeVariant>>,
}

impl ThemeRegistry {
    fn insert_theme_families(&mut self, families: impl IntoIterator<Item = ThemeFamily>) {
        for family in families.into_iter() {
            self.insert_themes(family.themes);
        }
    }

    fn insert_themes(&mut self, themes: impl IntoIterator<Item = ThemeVariant>) {
        for theme in themes.into_iter() {
            self.themes.insert(theme.name.clone(), Arc::new(theme));
        }
    }

    pub fn list_names(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.keys().cloned()
    }

    pub fn list(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.values().map(|theme| theme.name.clone())
    }

    pub fn get(&self, name: &str) -> Result<Arc<ThemeVariant>> {
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

        this
    }
}
