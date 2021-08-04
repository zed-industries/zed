use crate::theme;
use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};
use postage::watch;
use std::sync::Arc;

pub use theme::{HighlightId, HighlightMap, Theme, ThemeRegistry};

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    pub ui_font_family: FamilyId,
    pub ui_font_size: f32,
    pub theme: Arc<Theme>,
}

impl Settings {
    pub fn new(font_cache: &FontCache) -> Result<Self> {
        Self::new_with_theme(font_cache, Arc::new(Theme::default()))
    }

    pub fn new_with_theme(font_cache: &FontCache, theme: Arc<Theme>) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&["Fira Code", "Monaco"])?,
            buffer_font_size: 14.0,
            tab_size: 4,
            ui_font_family: font_cache.load_family(&["SF Pro", "Helvetica"])?,
            ui_font_size: 12.0,
            theme,
        })
    }

    pub fn with_tab_size(mut self, tab_size: usize) -> Self {
        self.tab_size = tab_size;
        self
    }
}

pub fn channel(
    font_cache: &FontCache,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    Ok(watch::channel_with(Settings::new(font_cache)?))
}

pub fn channel_with_themes(
    font_cache: &FontCache,
    themes: &ThemeRegistry,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    let theme = match themes.get("dark") {
        Ok(theme) => dbg!(theme),
        Err(err) => {
            panic!("failed to deserialize default theme: {:?}", err)
        }
    };
    Ok(watch::channel_with(Settings::new_with_theme(
        font_cache, theme,
    )?))
}
