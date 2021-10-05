use crate::theme::{self, DEFAULT_THEME_NAME};
use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};
use postage::watch;
use std::sync::Arc;
pub use theme::{Theme, ThemeRegistry};

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    pub theme: Arc<Theme>,
}

impl Settings {
    pub fn new(
        buffer_font_family: &str,
        font_cache: &FontCache,
        theme: Arc<Theme>,
    ) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&[buffer_font_family])?,
            buffer_font_size: 16.,
            tab_size: 4,
            theme,
        })
    }

    pub fn with_tab_size(mut self, tab_size: usize) -> Self {
        self.tab_size = tab_size;
        self
    }
}

pub fn channel(
    buffer_font_family: &str,
    font_cache: &FontCache,
    themes: &ThemeRegistry,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    let theme = match themes.get(DEFAULT_THEME_NAME) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("failed to deserialize default theme: {:?}", err)
        }
    };
    Ok(watch::channel_with(Settings::new(
        buffer_font_family,
        font_cache,
        theme,
    )?))
}
