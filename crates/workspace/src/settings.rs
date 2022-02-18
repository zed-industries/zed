use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};
use language::Language;
use std::{collections::HashMap, sync::Arc};
use theme::Theme;

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    soft_wrap: SoftWrap,
    preferred_line_length: u32,
    overrides: HashMap<String, Override>,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Default)]
pub struct Override {
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
}

#[derive(Copy, Clone)]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

impl Settings {
    pub fn new(
        buffer_font_family: &str,
        font_cache: &FontCache,
        theme: Arc<Theme>,
    ) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&[buffer_font_family])?,
            buffer_font_size: 15.,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            overrides: Default::default(),
            theme,
        })
    }

    pub fn with_tab_size(mut self, tab_size: usize) -> Self {
        self.tab_size = tab_size;
        self
    }

    pub fn with_overrides(mut self, language_name: impl Into<String>, overrides: Override) -> Self {
        self.overrides.insert(language_name.into(), overrides);
        self
    }

    pub fn soft_wrap(&self, language: Option<&Arc<Language>>) -> SoftWrap {
        language
            .and_then(|language| self.overrides.get(language.name()))
            .and_then(|settings| settings.soft_wrap)
            .unwrap_or(self.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&Arc<Language>>) -> u32 {
        language
            .and_then(|language| self.overrides.get(language.name()))
            .and_then(|settings| settings.preferred_line_length)
            .unwrap_or(self.preferred_line_length)
    }
}
