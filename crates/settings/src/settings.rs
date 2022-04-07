use anyhow::Result;
use gpui::font_cache::{FamilyId, FontCache};
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    JsonSchema,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, sync::Arc};
use theme::{Theme, ThemeRegistry};
use util::ResultExt as _;

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub vim_mode: bool,
    pub tab_size: u32,
    pub soft_wrap: SoftWrap,
    pub preferred_line_length: u32,
    pub language_overrides: HashMap<Arc<str>, LanguageOverride>,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct LanguageOverride {
    pub tab_size: Option<u32>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum ThemeNames {
    Dark,
    Light,
}

impl Display for ThemeNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeNames::Dark => write!(f, "Dark"),
            ThemeNames::Light => write!(f, "Light"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct LanguageOverrides {
    C: Option<LanguageOverride>,
    JSON: Option<LanguageOverride>,
    Markdown: Option<LanguageOverride>,
    PlainText: Option<LanguageOverride>,
    Rust: Option<LanguageOverride>,
    TSX: Option<LanguageOverride>,
    TypeScript: Option<LanguageOverride>,
}

impl IntoIterator for LanguageOverrides {
    type Item = (String, LanguageOverride);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("C", self.C),
            ("JSON", self.JSON),
            ("Markdown", self.Markdown),
            ("PlainText", self.PlainText),
            ("Rust", self.Rust),
            ("TSX", self.TSX),
            ("TypeScript", self.TypeScript),
        ]
        .into_iter()
        .filter_map(|(name, language_override)| language_override.map(|lo| (name.to_owned(), lo)))
        .collect::<Vec<_>>()
        .into_iter()
    }
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct SettingsFileContent {
    #[serde(default)]
    pub buffer_font_family: Option<String>,
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    #[serde(default)]
    pub vim_mode: Option<bool>,
    #[serde(flatten)]
    pub editor: LanguageOverride,
    #[serde(default)]
    pub language_overrides: LanguageOverrides,
    #[serde(default)]
    pub theme: Option<ThemeNames>,
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
            vim_mode: false,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
            theme,
        })
    }

    pub fn file_json_schema() -> serde_json::Value {
        let settings = SchemaSettings::draft07().with(|settings| {
            settings.option_nullable = true;
            settings.option_add_null_type = false;
        });
        let generator = SchemaGenerator::new(settings);
        serde_json::to_value(generator.into_root_schema_for::<SettingsFileContent>()).unwrap()
    }

    pub fn with_overrides(
        mut self,
        language_name: impl Into<Arc<str>>,
        overrides: LanguageOverride,
    ) -> Self {
        self.language_overrides
            .insert(language_name.into(), overrides);
        self
    }

    pub fn tab_size(&self, language: Option<&str>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.tab_size)
            .unwrap_or(self.tab_size)
    }

    pub fn soft_wrap(&self, language: Option<&str>) -> SoftWrap {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.soft_wrap)
            .unwrap_or(self.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&str>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language))
            .and_then(|settings| settings.preferred_line_length)
            .unwrap_or(self.preferred_line_length)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            vim_mode: false,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
            theme: gpui::fonts::with_font_cache(cx.font_cache().clone(), || Default::default()),
        }
    }

    pub fn merge(
        &mut self,
        data: &SettingsFileContent,
        theme_registry: &ThemeRegistry,
        font_cache: &FontCache,
    ) {
        if let Some(value) = &data.buffer_font_family {
            if let Some(id) = font_cache.load_family(&[value]).log_err() {
                self.buffer_font_family = id;
            }
        }
        if let Some(value) = &data.theme {
            if let Some(theme) = theme_registry.get(&value.to_string()).log_err() {
                self.theme = theme;
            }
        }

        merge(&mut self.buffer_font_size, data.buffer_font_size);
        merge(&mut self.vim_mode, data.vim_mode);
        merge(&mut self.soft_wrap, data.editor.soft_wrap);
        merge(&mut self.tab_size, data.editor.tab_size);
        merge(
            &mut self.preferred_line_length,
            data.editor.preferred_line_length,
        );

        for (language_name, settings) in data.language_overrides.clone().into_iter() {
            let target = self
                .language_overrides
                .entry(language_name.into())
                .or_default();

            merge_option(&mut target.tab_size, settings.tab_size);
            merge_option(&mut target.soft_wrap, settings.soft_wrap);
            merge_option(
                &mut target.preferred_line_length,
                settings.preferred_line_length,
            );
        }
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

fn merge_option<T: Copy>(target: &mut Option<T>, value: Option<T>) {
    if value.is_some() {
        *target = value;
    }
}
