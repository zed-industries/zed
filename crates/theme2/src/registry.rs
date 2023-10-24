use crate::{Theme, ThemeMetadata, themes::one_dark};
use anyhow::Result;
use gpui2::{AnyAssetSource, SharedString};
use std::{
    collections::HashMap,
    sync::Arc
};

pub struct ThemeRegistry {
    themes: HashMap<SharedString, Arc<Theme>>,
}

impl ThemeRegistry {
    pub fn new(assets: AnyAssetSource) -> Self {
        let mut this = Self {
            themes: HashMap::default(),
        };

        this.insert_themes([one_dark()]);

        this
    }

    fn insert_themes(&mut self, themes: impl IntoIterator<Item = Theme>) {
        for theme in themes.into_iter() {
            self.themes.insert(theme.metadata.name.clone(), Arc::new(theme));
        }
    }

    pub fn list_names(&self, staff: bool) -> impl Iterator<Item = SharedString> + '_ {
        None.into_iter()
    }

    pub fn list(&self, staff: bool) -> impl Iterator<Item = ThemeMetadata> + '_ {
        None.into_iter()
    }

    pub fn get(&self, name: impl Into<SharedString>) -> Result<Arc<Theme>> {
        todo!()
    }
}
