use crate::watch;
use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    pub ui_font_family: FamilyId,
    pub ui_font_size: f32,
}

impl Settings {
    pub fn new(font_cache: &FontCache) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&["Fira Code", "Monaco"])?,
            buffer_font_size: 16.0,
            tab_size: 4,
            ui_font_family: font_cache.load_family(&["SF Pro Display"])?,
            ui_font_size: 12.0,
        })
    }
}

pub fn channel(
    font_cache: &FontCache,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    Ok(watch::channel(Settings::new(font_cache)?))
}
