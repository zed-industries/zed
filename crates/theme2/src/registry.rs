use crate::{themes, Theme, ThemeMetadata};
use anyhow::{anyhow, Result};
use gpui2::SharedString;
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
    fn insert_themes(&mut self, themes: impl IntoIterator<Item = Theme>) {
        for theme in themes.into_iter() {
            self.themes
                .insert(theme.metadata.name.clone(), Arc::new(theme));
        }
    }

    pub fn list_names(&self, _staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        self.themes.keys().cloned()
    }

    pub fn list(&self, _staff: bool) -> impl Iterator<Item = ThemeMetadata> + '_ {
        self.themes.values().map(|theme| theme.metadata.clone())
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

        this.insert_themes([
            themes::andromeda(),
            themes::atelier_cave_dark(),
            themes::atelier_cave_light(),
            themes::atelier_dune_dark(),
            themes::atelier_dune_light(),
            themes::atelier_estuary_dark(),
            themes::atelier_estuary_light(),
            themes::atelier_forest_dark(),
            themes::atelier_forest_light(),
            themes::atelier_heath_dark(),
            themes::atelier_heath_light(),
            themes::atelier_lakeside_dark(),
            themes::atelier_lakeside_light(),
            themes::atelier_plateau_dark(),
            themes::atelier_plateau_light(),
            themes::atelier_savanna_dark(),
            themes::atelier_savanna_light(),
            themes::atelier_seaside_dark(),
            themes::atelier_seaside_light(),
            themes::atelier_sulphurpool_dark(),
            themes::atelier_sulphurpool_light(),
            themes::ayu_dark(),
            themes::ayu_light(),
            themes::ayu_mirage(),
            themes::gruvbox_dark(),
            themes::gruvbox_dark_hard(),
            themes::gruvbox_dark_soft(),
            themes::gruvbox_light(),
            themes::gruvbox_light_hard(),
            themes::gruvbox_light_soft(),
            themes::one_dark(),
            themes::one_light(),
            themes::rose_pine(),
            themes::rose_pine_dawn(),
            themes::rose_pine_moon(),
            themes::sandcastle(),
            themes::solarized_dark(),
            themes::solarized_light(),
            themes::summercamp(),
        ]);

        this
    }
}
