use anyhow::{anyhow, Context, Result};
use gpui::{
    color::ColorU,
    font_cache::{FamilyId, FontCache},
    fonts::{Properties as FontProperties, Style as FontStyle, Weight as FontWeight},
    AssetSource,
};
use parking_lot::Mutex;
use postage::watch;
use serde::{de::value::MapDeserializer, Deserialize};
use serde_json::Value;
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

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    theme_data: Mutex<HashMap<String, Arc<ThemeToml>>>,
}

#[derive(Clone, Default)]
pub struct Theme {
    pub ui: UiTheme,
    pub editor: EditorTheme,
    pub syntax: Vec<(String, ColorU, FontProperties)>,
}

#[derive(Deserialize)]
struct ThemeToml {
    #[serde(default)]
    extends: Option<String>,
    #[serde(default)]
    variables: HashMap<String, Value>,
    #[serde(default)]
    ui: HashMap<String, Value>,
    #[serde(default)]
    editor: HashMap<String, Value>,
    #[serde(default)]
    syntax: HashMap<String, Value>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct UiTheme {
    pub background: Color,
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

#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct EditorTheme {
    pub background: Color,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub default_text: Color,
    pub replicas: Vec<ReplicaTheme>,
}

#[derive(Clone, Copy, Deserialize, Default)]
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

impl ThemeRegistry {
    pub fn new(source: impl AssetSource) -> Arc<Self> {
        Arc::new(Self {
            assets: Box::new(source),
            themes: Default::default(),
            theme_data: Default::default(),
        })
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        if let Some(theme) = self.themes.lock().get(name) {
            return Ok(theme.clone());
        }

        let theme_toml = self.load(name)?;
        let mut syntax = Vec::<(String, ColorU, FontProperties)>::new();
        for (key, style) in theme_toml.syntax.iter() {
            let mut color = Color::default();
            let mut properties = FontProperties::new();
            match style {
                Value::Object(object) => {
                    if let Some(value) = object.get("color") {
                        color = serde_json::from_value(value.clone())?;
                    }
                    if let Some(Value::Bool(true)) = object.get("italic") {
                        properties.style = FontStyle::Italic;
                    }
                    properties.weight = deserialize_weight(object.get("weight"))?;
                }
                _ => {
                    color = serde_json::from_value(style.clone())?;
                }
            }
            match syntax.binary_search_by_key(&key, |e| &e.0) {
                Ok(i) | Err(i) => {
                    syntax.insert(i, (key.to_string(), color.0, properties));
                }
            }
        }

        let theme = Arc::new(Theme {
            ui: UiTheme::deserialize(MapDeserializer::new(theme_toml.ui.clone().into_iter()))?,
            editor: EditorTheme::deserialize(MapDeserializer::new(
                theme_toml.editor.clone().into_iter(),
            ))?,
            syntax,
        });

        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }

    fn load(&self, name: &str) -> Result<Arc<ThemeToml>> {
        if let Some(data) = self.theme_data.lock().get(name) {
            return Ok(data.clone());
        }

        let asset_path = format!("themes/{}.toml", name);
        let source_code = self
            .assets
            .load(&asset_path)
            .with_context(|| format!("failed to load theme file {}", asset_path))?;

        let mut theme_toml: ThemeToml = toml::from_slice(source_code.as_ref())
            .with_context(|| format!("failed to parse {}.toml", name))?;

        // If this theme extends another base theme, merge in the raw data from the base theme.
        if let Some(base_name) = theme_toml.extends.as_ref() {
            let base_theme_toml = self
                .load(base_name)
                .with_context(|| format!("failed to load base theme {}", base_name))?;
            merge_map(&mut theme_toml.ui, &base_theme_toml.ui);
            merge_map(&mut theme_toml.editor, &base_theme_toml.editor);
            merge_map(&mut theme_toml.syntax, &base_theme_toml.syntax);
            merge_map(&mut theme_toml.variables, &base_theme_toml.variables);
        }

        // Substitute any variable references for their definitions.
        let values = theme_toml
            .ui
            .values_mut()
            .chain(theme_toml.editor.values_mut())
            .chain(theme_toml.syntax.values_mut());
        let mut name_stack = Vec::new();
        for value in values {
            name_stack.clear();
            evaluate_variables(value, &theme_toml.variables, &mut name_stack)?;
        }

        let result = Arc::new(theme_toml);
        self.theme_data
            .lock()
            .insert(name.to_string(), result.clone());
        Ok(result)
    }
}

impl Theme {
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

impl Default for EditorTheme {
    fn default() -> Self {
        Self {
            background: Default::default(),
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            default_text: Default::default(),
            replicas: vec![ReplicaTheme::default()],
        }
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

impl Color {
    fn from_u32(rgba: u32) -> Self {
        Self(ColorU::from_u32(rgba))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let rgb = u32::deserialize(deserializer)?;
        Ok(Self::from_u32((rgb << 8) + 0xFF))
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

pub fn channel_with_themes(
    font_cache: &FontCache,
    themes: &ThemeRegistry,
) -> Result<(watch::Sender<Settings>, watch::Receiver<Settings>)> {
    Ok(watch::channel_with(Settings::new_with_theme(
        font_cache,
        themes.get("dark").expect("failed to load default theme"),
    )?))
}

fn deserialize_weight(weight: Option<&Value>) -> Result<FontWeight> {
    match weight {
        None => return Ok(FontWeight::NORMAL),
        Some(Value::Number(number)) => {
            if let Some(weight) = number.as_f64() {
                return Ok(FontWeight(weight as f32));
            }
        }
        Some(Value::String(s)) => match s.as_str() {
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

fn evaluate_variables(
    expr: &mut Value,
    variables: &HashMap<String, Value>,
    stack: &mut Vec<String>,
) -> Result<()> {
    match expr {
        Value::String(s) => {
            if let Some(name) = s.strip_prefix("$") {
                if stack.iter().any(|e| e == name) {
                    Err(anyhow!("variable {} is defined recursively", name))?;
                }
                if validate_variable_name(name) {
                    stack.push(name.to_string());
                    if let Some(definition) = variables.get(name).cloned() {
                        *expr = definition;
                        evaluate_variables(expr, variables, stack)?;
                    }
                    stack.pop();
                }
            }
        }
        Value::Array(a) => {
            for value in a.iter_mut() {
                evaluate_variables(value, variables, stack)?;
            }
        }
        Value::Object(object) => {
            for value in object.values_mut() {
                evaluate_variables(value, variables, stack)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_variable_name(name: &str) -> bool {
    let mut chars = name.chars();
    if let Some(first) = chars.next() {
        if first.is_alphabetic() || first == '_' {
            if chars.all(|c| c.is_alphanumeric() || c == '_') {
                return true;
            }
        }
    }
    false
}

fn merge_map(left: &mut HashMap<String, Value>, right: &HashMap<String, Value>) {
    for (name, value) in right {
        if !left.contains_key(name) {
            left.insert(name.clone(), value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_theme() {
        let assets = TestAssets(&[(
            "themes/my-theme.toml",
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
        )]);

        let registry = ThemeRegistry::new(assets);
        let theme = registry.get("my-theme").unwrap();

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
    fn test_parse_extended_theme() {
        let assets = TestAssets(&[
            (
                "themes/base.toml",
                r#"
                [ui]
                tab_background = 0x111111
                tab_text = "$variable_1"

                [editor]
                background = 0x222222
                default_text = "$variable_2"
                "#,
            ),
            (
                "themes/light.toml",
                r#"
                extends = "base"

                [variables]
                variable_1 = 0x333333
                variable_2 = 0x444444

                [ui]
                tab_background = 0x555555

                [editor]
                background = 0x666666
                "#,
            ),
        ]);

        let registry = ThemeRegistry::new(assets);
        let theme = registry.get("light").unwrap();

        assert_eq!(theme.ui.tab_background, ColorU::from_u32(0x555555ff));
        assert_eq!(theme.ui.tab_text, ColorU::from_u32(0x333333ff));
        assert_eq!(theme.editor.background, ColorU::from_u32(0x666666ff));
        assert_eq!(theme.editor.default_text, ColorU::from_u32(0x444444ff));
    }

    #[test]
    fn test_parse_empty_theme() {
        let assets = TestAssets(&[("themes/my-theme.toml", "")]);
        let registry = ThemeRegistry::new(assets);
        registry.get("my-theme").unwrap();
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

    struct TestAssets(&'static [(&'static str, &'static str)]);

    impl AssetSource for TestAssets {
        fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
            if let Some(row) = self.0.iter().find(|e| e.0 == path) {
                Ok(row.1.as_bytes().into())
            } else {
                Err(anyhow!("no such path {}", path))
            }
        }
    }
}
