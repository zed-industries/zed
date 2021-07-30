use super::assets::Assets;
use anyhow::{anyhow, Context, Result};
use gpui::{
    color::ColorU,
    font_cache::{FamilyId, FontCache},
    fonts::{Properties as FontProperties, Style as FontStyle, Weight as FontWeight},
};
use postage::watch;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

const DEFAULT_STYLE_ID: StyleId = StyleId(u32::MAX);

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: usize,
    pub ui_font_family: FamilyId,
    pub ui_font_size: f32,
    pub theme: Arc<Theme>,
}

#[derive(Clone, Default)]
pub struct Theme {
    pub ui: UiTheme,
    pub editor: EditorTheme,
    syntax: Vec<(String, ColorU, FontProperties)>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct UiTheme {
    pub tab_background: Color,
    pub tab_background_active: Color,
    pub tab_text: Color,
    pub tab_text_active: Color,
    pub tab_border: Color,
    pub tab_icon_close: Color,
    pub tab_icon_dirty: Color,
    pub tab_icon_conflict: Color,
    pub modal_background: Color,
    pub modal_match_background: Color,
    pub modal_match_background_active: Color,
    pub modal_match_border: Color,
    pub modal_match_text: Color,
    pub modal_match_text_highlight: Color,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct EditorTheme {
    pub background: Color,
    pub gutter_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub default_text: Color,
    pub replicas: Vec<ReplicaTheme>,
}

#[derive(Clone, Copy, Deserialize)]
pub struct ReplicaTheme {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Clone, Copy, Default)]
pub struct Color(pub ColorU);

#[derive(Clone, Debug)]
pub struct ThemeMap(Arc<[StyleId]>);

#[derive(Clone, Copy, Debug)]
pub struct StyleId(u32);

impl Settings {
    pub fn new(font_cache: &FontCache) -> Result<Self> {
        Ok(Self {
            buffer_font_family: font_cache.load_family(&["Fira Code", "Monaco"])?,
            buffer_font_size: 14.0,
            tab_size: 4,
            ui_font_family: font_cache.load_family(&["SF Pro", "Helvetica"])?,
            ui_font_size: 12.0,
            theme: Arc::new(
                Theme::parse(Assets::get("themes/dark.toml").unwrap())
                    .expect("Failed to parse built-in theme"),
            ),
        })
    }

    pub fn with_tab_size(mut self, tab_size: usize) -> Self {
        self.tab_size = tab_size;
        self
    }
}

impl Theme {
    pub fn parse(source: impl AsRef<[u8]>) -> Result<Self> {
        #[derive(Deserialize)]
        struct ThemeToml {
            #[serde(default)]
            ui: UiTheme,
            #[serde(default)]
            editor: EditorTheme,
            #[serde(default)]
            syntax: HashMap<String, StyleToml>,
        }

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StyleToml {
            Color(Color),
            Full {
                color: Option<Color>,
                weight: Option<toml::Value>,
                #[serde(default)]
                italic: bool,
            },
        }

        let theme_toml: ThemeToml =
            toml::from_slice(source.as_ref()).context("failed to parse theme TOML")?;

        let mut syntax = Vec::<(String, ColorU, FontProperties)>::new();
        for (key, style) in theme_toml.syntax {
            let (color, weight, italic) = match style {
                StyleToml::Color(color) => (color, None, false),
                StyleToml::Full {
                    color,
                    weight,
                    italic,
                } => (color.unwrap_or(Color::default()), weight, italic),
            };
            match syntax.binary_search_by_key(&&key, |e| &e.0) {
                Ok(i) | Err(i) => {
                    let mut properties = FontProperties::new();
                    properties.weight = deserialize_weight(weight)?;
                    if italic {
                        properties.style = FontStyle::Italic;
                    }
                    syntax.insert(i, (key, color.0, properties));
                }
            }
        }

        Ok(Theme {
            ui: theme_toml.ui,
            editor: theme_toml.editor,
            syntax,
        })
    }

    pub fn syntax_style(&self, id: StyleId) -> (ColorU, FontProperties) {
        self.syntax.get(id.0 as usize).map_or(
            (self.editor.default_text.0, FontProperties::new()),
            |entry| (entry.1, entry.2),
        )
    }

    #[cfg(test)]
    pub fn syntax_style_name(&self, id: StyleId) -> Option<&str> {
        self.syntax.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

impl ThemeMap {
    pub fn new(capture_names: &[String], theme: &Theme) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        ThemeMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    theme
                        .syntax
                        .iter()
                        .enumerate()
                        .filter_map(|(i, (key, _, _))| {
                            let mut len = 0;
                            let capture_parts = capture_name.split('.');
                            for key_part in key.split('.') {
                                if capture_parts.clone().any(|part| part == key_part) {
                                    len += 1;
                                } else {
                                    return None;
                                }
                            }
                            Some((i, len))
                        })
                        .max_by_key(|(_, len)| *len)
                        .map_or(DEFAULT_STYLE_ID, |(i, _)| StyleId(i as u32))
                })
                .collect(),
        )
    }

    pub fn get(&self, capture_id: u32) -> StyleId {
        self.0
            .get(capture_id as usize)
            .copied()
            .unwrap_or(DEFAULT_STYLE_ID)
    }
}

impl Default for ThemeMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl Default for StyleId {
    fn default() -> Self {
        DEFAULT_STYLE_ID
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let rgba_value = u32::deserialize(deserializer)?;
        Ok(Self(ColorU::from_u32((rgba_value << 8) + 0xFF)))
    }
}

impl Into<ColorU> for Color {
    fn into(self) -> ColorU {
        self.0
    }
}

impl Deref for Color {
    type Target = ColorU;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<ColorU> for Color {
    fn eq(&self, other: &ColorU) -> bool {
        self.0.eq(other)
    }
}

pub fn channel(
    font_cache: &FontCache,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    Ok(watch::channel_with(Settings::new(font_cache)?))
}

fn deserialize_weight(weight: Option<toml::Value>) -> Result<FontWeight> {
    match &weight {
        None => return Ok(FontWeight::NORMAL),
        Some(toml::Value::Integer(i)) => return Ok(FontWeight(*i as f32)),
        Some(toml::Value::String(s)) => match s.as_str() {
            "normal" => return Ok(FontWeight::NORMAL),
            "bold" => return Ok(FontWeight::BOLD),
            "light" => return Ok(FontWeight::LIGHT),
            "semibold" => return Ok(FontWeight::SEMIBOLD),
            _ => {}
        },
        _ => {}
    }
    Err(anyhow!("Invalid weight {}", weight.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_theme() {
        let theme = Theme::parse(
            r#"
            [ui]
            tab_background_active = 0x100000

            [editor]
            background = 0x00ed00
            line_number = 0xdddddd

            [syntax]
            "beta.two" = 0xAABBCC
            "alpha.one" = {color = 0x112233, weight = "bold"}
            "gamma.three" = {weight = "light", italic = true}
            "#,
        )
        .unwrap();

        assert_eq!(theme.ui.tab_background_active, ColorU::from_u32(0x100000ff));
        assert_eq!(theme.editor.background, ColorU::from_u32(0x00ed00ff));
        assert_eq!(theme.editor.line_number, ColorU::from_u32(0xddddddff));
        assert_eq!(
            theme.syntax,
            &[
                (
                    "alpha.one".to_string(),
                    ColorU::from_u32(0x112233ff),
                    *FontProperties::new().weight(FontWeight::BOLD)
                ),
                (
                    "beta.two".to_string(),
                    ColorU::from_u32(0xaabbccff),
                    *FontProperties::new().weight(FontWeight::NORMAL)
                ),
                (
                    "gamma.three".to_string(),
                    ColorU::from_u32(0x00000000),
                    *FontProperties::new()
                        .weight(FontWeight::LIGHT)
                        .style(FontStyle::Italic),
                ),
            ]
        );
    }

    #[test]
    fn test_parse_empty_theme() {
        Theme::parse("").unwrap();
    }

    #[test]
    fn test_theme_map() {
        let theme = Theme {
            ui: Default::default(),
            editor: Default::default(),
            syntax: [
                ("function", ColorU::from_u32(0x100000ff)),
                ("function.method", ColorU::from_u32(0x200000ff)),
                ("function.async", ColorU::from_u32(0x300000ff)),
                ("variable.builtin.self.rust", ColorU::from_u32(0x400000ff)),
                ("variable.builtin", ColorU::from_u32(0x500000ff)),
                ("variable", ColorU::from_u32(0x600000ff)),
            ]
            .iter()
            .map(|e| (e.0.to_string(), e.1, FontProperties::new()))
            .collect(),
        };

        let capture_names = &[
            "function.special".to_string(),
            "function.async.rust".to_string(),
            "variable.builtin.self".to_string(),
        ];

        let map = ThemeMap::new(capture_names, &theme);
        assert_eq!(theme.syntax_style_name(map.get(0)), Some("function"));
        assert_eq!(theme.syntax_style_name(map.get(1)), Some("function.async"));
        assert_eq!(
            theme.syntax_style_name(map.get(2)),
            Some("variable.builtin")
        );
    }
}
