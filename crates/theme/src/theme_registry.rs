use crate::Theme;
use anyhow::{Context, Result};
use gpui::{fonts, AssetSource, FontCache};
use parking_lot::Mutex;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    theme_data: Mutex<HashMap<String, Arc<Value>>>,
    font_cache: Arc<FontCache>,
}

impl ThemeRegistry {
    pub fn new(source: impl AssetSource, font_cache: Arc<FontCache>) -> Arc<Self> {
        Arc::new(Self {
            assets: Box::new(source),
            themes: Default::default(),
            theme_data: Default::default(),
            font_cache,
        })
    }

    pub fn list(&self) -> impl Iterator<Item = String> {
        self.assets.list("themes/").into_iter().filter_map(|path| {
            let filename = path.strip_prefix("themes/")?;
            let theme_name = filename.strip_suffix(".json")?;
            Some(theme_name.to_string())
        })
    }

    pub fn clear(&self) {
        self.theme_data.lock().clear();
        self.themes.lock().clear();
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        if let Some(theme) = self.themes.lock().get(name) {
            return Ok(theme.clone());
        }

        let asset_path = format!("themes/{}.json", name);
        let theme_json = self
            .assets
            .load(&asset_path)
            .with_context(|| format!("failed to load theme file {}", asset_path))?;

        let mut theme: Theme = fonts::with_font_cache(self.font_cache.clone(), || {
            serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(&theme_json))
        })?;

        theme.name = name.into();
        let theme = Arc::new(theme);
        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }
}
