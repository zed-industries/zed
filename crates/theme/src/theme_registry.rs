use crate::{Theme, ThemeMeta};
use anyhow::{Context, Result};
use gpui::{fonts, AssetSource, FontCache};
use parking_lot::Mutex;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    theme_data: Mutex<HashMap<String, Arc<Value>>>,
    font_cache: Arc<FontCache>,
    next_theme_id: AtomicUsize,
}

impl ThemeRegistry {
    pub fn new(source: impl AssetSource, font_cache: Arc<FontCache>) -> Arc<Self> {
        let this = Arc::new(Self {
            assets: Box::new(source),
            themes: Default::default(),
            theme_data: Default::default(),
            next_theme_id: Default::default(),
            font_cache,
        });

        this.themes.lock().insert(
            settings::EMPTY_THEME_NAME.to_string(),
            gpui::fonts::with_font_cache(this.font_cache.clone(), || {
                let mut theme = Theme::default();
                theme.meta.id = this.next_theme_id.fetch_add(1, SeqCst);
                theme.meta.name = settings::EMPTY_THEME_NAME.into();
                Arc::new(theme)
            }),
        );

        this
    }

    pub fn list(&self, staff: bool) -> impl Iterator<Item = ThemeMeta> + '_ {
        let mut dirs = self.assets.list("themes/");

        if !staff {
            dirs = dirs
                .into_iter()
                .filter(|path| !path.starts_with("themes/staff"))
                .collect()
        }

        dirs.into_iter().filter_map(|path| {
            let filename = path.strip_prefix("themes/")?;
            let theme_name = filename.strip_suffix(".json")?;
            self.get(theme_name).ok().map(|theme| theme.meta.clone())
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

        // Allocate into the heap directly, the Theme struct is too large to fit in the stack.
        let mut theme = fonts::with_font_cache(self.font_cache.clone(), || {
            let mut theme = Box::new(Theme::default());
            let mut deserializer = serde_json::Deserializer::from_slice(&theme_json);
            let result = Theme::deserialize_in_place(&mut deserializer, &mut theme);
            result.map(|_| theme)
        })?;

        // Reset name to be the file path, so that we can use it to access the stored themes
        theme.meta.name = name.into();
        theme.meta.id = self.next_theme_id.fetch_add(1, SeqCst);
        let theme: Arc<Theme> = theme.into();
        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }
}
