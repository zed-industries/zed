use crate::theme::{self, DEFAULT_THEME_NAME};
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
    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Self {
        lazy_static::lazy_static! {
            static ref DEFAULT_THEME: parking_lot::Mutex<Option<Arc<Theme>>> = Default::default();
        }

        let mut theme_guard = DEFAULT_THEME.lock();
        let theme = if let Some(theme) = theme_guard.as_ref() {
            theme.clone()
        } else {
            let theme = ThemeRegistry::new(crate::assets::Assets, cx.font_cache().clone())
                .get(DEFAULT_THEME_NAME)
                .expect("failed to load default theme in tests");
            *theme_guard = Some(theme.clone());
            theme
        };

        Self::new(cx.font_cache(), theme).unwrap()
    }

    pub fn new(font_cache: &FontCache, theme: Arc<Theme>) -> Result<Self> {
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

#[cfg(any(test, feature = "test-support"))]
pub fn test(cx: &gpui::AppContext) -> (watch::Sender<Settings>, watch::Receiver<Settings>) {
    watch::channel_with(Settings::test(cx))
}

pub fn channel(
    font_cache: &FontCache,
    themes: &ThemeRegistry,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    let theme = match themes.get(DEFAULT_THEME_NAME) {
        Ok(theme) => theme,
        Err(err) => {
            panic!("failed to deserialize default theme: {:?}", err)
        }
    };
    Ok(watch::channel_with(Settings::new(font_cache, theme)?))
}
