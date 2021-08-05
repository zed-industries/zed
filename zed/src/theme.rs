use anyhow::{anyhow, Context, Result};
use gpui::{
    color::Color,
    elements::{ContainerStyle, LabelStyle},
    fonts::TextStyle,
    AssetSource,
};
use json::{Map, Value};
use parking_lot::Mutex;
use serde::{Deserialize, Deserializer};
use serde_json as json;
use std::{cmp::Ordering, collections::HashMap, sync::Arc};

const DEFAULT_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);
pub const DEFAULT_THEME_NAME: &'static str = "dark";

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    theme_data: Mutex<HashMap<String, Arc<Value>>>,
}

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug)]
pub struct HighlightId(u32);

#[derive(Debug, Default, Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub ui: Ui,
    pub editor: Editor,
    #[serde(deserialize_with = "deserialize_syntax_theme")]
    pub syntax: Vec<(String, TextStyle)>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Ui {
    pub background: Color,
    pub tab: Tab,
    pub active_tab: Tab,
    pub selector: Selector,
}

#[derive(Debug, Deserialize)]
pub struct Editor {
    pub background: Color,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub text: Color,
    pub replicas: Vec<Replica>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Replica {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Debug, Default, Deserialize)]
pub struct Tab {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon_close: Color,
    pub icon_dirty: Color,
    pub icon_conflict: Color,
}

#[derive(Debug, Default, Deserialize)]
pub struct Selector {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,

    pub item: SelectorItem,
    pub active_item: SelectorItem,
}

#[derive(Debug, Default, Deserialize)]
pub struct SelectorItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            background: Default::default(),
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            text: Default::default(),
            replicas: vec![Replica::default()],
        }
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

    pub fn list(&self) -> impl Iterator<Item = String> {
        self.assets.list("themes/").into_iter().filter_map(|path| {
            let filename = path.strip_prefix("themes/")?;
            let theme_name = filename.strip_suffix(".toml")?;
            if theme_name.starts_with('_') {
                None
            } else {
                Some(theme_name.to_string())
            }
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

        let theme_data = self.load(name)?;
        let mut theme = serde_json::from_value::<Theme>(theme_data.as_ref().clone())?;
        theme.name = name.into();
        let theme = Arc::new(theme);
        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }

    fn load(&self, name: &str) -> Result<Arc<Value>> {
        if let Some(data) = self.theme_data.lock().get(name) {
            return Ok(data.clone());
        }

        let asset_path = format!("themes/{}.toml", name);
        let source_code = self
            .assets
            .load(&asset_path)
            .with_context(|| format!("failed to load theme file {}", asset_path))?;

        let mut theme_data: Map<String, Value> = toml::from_slice(source_code.as_ref())
            .with_context(|| format!("failed to parse {}.toml", name))?;

        // If this theme extends another base theme, deeply merge it into the base theme's data
        if let Some(base_name) = theme_data
            .get("extends")
            .and_then(|name| name.as_str())
            .map(str::to_string)
        {
            let base_theme_data = self
                .load(&base_name)
                .with_context(|| format!("failed to load base theme {}", base_name))?
                .as_ref()
                .clone();
            if let Value::Object(mut base_theme_object) = base_theme_data {
                deep_merge_json(&mut base_theme_object, theme_data);
                theme_data = base_theme_object;
            }
        }

        // Evaluate `extends` fields in styles
        // First, find the key paths of all objects with `extends` directives
        let mut directives = Vec::new();
        let mut key_path = Vec::new();
        for (key, value) in theme_data.iter() {
            if value.is_array() || value.is_object() {
                key_path.push(Key::Object(key.clone()));
                find_extensions(value, &mut key_path, &mut directives);
                key_path.pop();
            }
        }
        // If you extend something with an extend directive, process the source's extend directive first
        directives.sort_unstable();

        // Now update objects to include the fields of objects they extend
        for ExtendDirective {
            source_path,
            target_path,
        } in directives
        {
            let source = value_at(&mut theme_data, &source_path)?.clone();
            let target = value_at(&mut theme_data, &target_path)?;
            if let (Value::Object(mut source_object), Value::Object(target_object)) =
                (source, target.take())
            {
                deep_merge_json(&mut source_object, target_object);
                *target = Value::Object(source_object);
            }
        }

        // Evaluate any variables
        if let Some((key, variables)) = theme_data.remove_entry("variables") {
            if let Some(variables) = variables.as_object() {
                for value in theme_data.values_mut() {
                    evaluate_variables(value, &variables, &mut Vec::new())?;
                }
            }
            theme_data.insert(key, variables);
        }

        let result = Arc::new(Value::Object(theme_data));
        self.theme_data
            .lock()
            .insert(name.to_string(), result.clone());

        Ok(result)
    }
}

impl Theme {
    pub fn highlight_style(&self, id: HighlightId) -> TextStyle {
        self.syntax
            .get(id.0 as usize)
            .map(|entry| entry.1.clone())
            .unwrap_or_else(|| TextStyle {
                color: self.editor.text,
                font_properties: Default::default(),
            })
    }

    #[cfg(test)]
    pub fn highlight_name(&self, id: HighlightId) -> Option<&str> {
        self.syntax.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

impl HighlightMap {
    pub fn new(capture_names: &[String], theme: &Theme) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        HighlightMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    theme
                        .syntax
                        .iter()
                        .enumerate()
                        .filter_map(|(i, (key, _))| {
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
                        .map_or(DEFAULT_HIGHLIGHT_ID, |(i, _)| HighlightId(i as u32))
                })
                .collect(),
        )
    }

    pub fn get(&self, capture_id: u32) -> HighlightId {
        self.0
            .get(capture_id as usize)
            .copied()
            .unwrap_or(DEFAULT_HIGHLIGHT_ID)
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl Default for HighlightId {
    fn default() -> Self {
        DEFAULT_HIGHLIGHT_ID
    }
}

fn deep_merge_json(base: &mut Map<String, Value>, extension: Map<String, Value>) {
    for (key, extension_value) in extension {
        if let Value::Object(extension_object) = extension_value {
            if let Some(base_object) = base.get_mut(&key).and_then(|value| value.as_object_mut()) {
                deep_merge_json(base_object, extension_object);
            } else {
                base.insert(key, Value::Object(extension_object));
            }
        } else {
            base.insert(key, extension_value);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Key {
    Array(usize),
    Object(String),
}

#[derive(Debug, PartialEq, Eq)]
struct ExtendDirective {
    source_path: Vec<Key>,
    target_path: Vec<Key>,
}

impl Ord for ExtendDirective {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.target_path.starts_with(&other.source_path)
            || other.source_path.starts_with(&self.target_path)
        {
            Ordering::Less
        } else if other.target_path.starts_with(&self.source_path)
            || self.source_path.starts_with(&other.target_path)
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for ExtendDirective {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_extensions(value: &Value, key_path: &mut Vec<Key>, directives: &mut Vec<ExtendDirective>) {
    match value {
        Value::Array(vec) => {
            for (ix, value) in vec.iter().enumerate() {
                key_path.push(Key::Array(ix));
                find_extensions(value, key_path, directives);
                key_path.pop();
            }
        }
        Value::Object(map) => {
            for (key, value) in map.iter() {
                if key == "extends" {
                    if let Some(source_path) = value.as_str() {
                        directives.push(ExtendDirective {
                            source_path: source_path
                                .split(".")
                                .map(|key| Key::Object(key.to_string()))
                                .collect(),
                            target_path: key_path.clone(),
                        });
                    }
                } else if value.is_array() || value.is_object() {
                    key_path.push(Key::Object(key.to_string()));
                    find_extensions(value, key_path, directives);
                    key_path.pop();
                }
            }
        }
        _ => {}
    }
}

fn value_at<'a>(object: &'a mut Map<String, Value>, key_path: &Vec<Key>) -> Result<&'a mut Value> {
    let mut key_path = key_path.iter();
    if let Some(Key::Object(first_key)) = key_path.next() {
        let mut cur_value = object.get_mut(first_key);
        for key in key_path {
            if let Some(value) = cur_value {
                match key {
                    Key::Array(ix) => cur_value = value.get_mut(ix),
                    Key::Object(key) => cur_value = value.get_mut(key),
                }
            } else {
                return Err(anyhow!("invalid key path"));
            }
        }
        cur_value.ok_or_else(|| anyhow!("invalid key path"))
    } else {
        Err(anyhow!("invalid key path"))
    }
}

fn evaluate_variables(
    value: &mut Value,
    variables: &Map<String, Value>,
    stack: &mut Vec<String>,
) -> Result<()> {
    match value {
        Value::String(s) => {
            if let Some(name) = s.strip_prefix("$") {
                if stack.iter().any(|e| e == name) {
                    Err(anyhow!("variable {} is defined recursively", name))?;
                }
                if validate_variable_name(name) {
                    stack.push(name.to_string());
                    if let Some(definition) = variables.get(name).cloned() {
                        *value = definition;
                        evaluate_variables(value, variables, stack)?;
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

pub fn deserialize_syntax_theme<'de, D>(
    deserializer: D,
) -> Result<Vec<(String, TextStyle)>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut result = Vec::<(String, TextStyle)>::new();

    let syntax_data: HashMap<String, TextStyle> = Deserialize::deserialize(deserializer)?;
    for (key, style) in syntax_data {
        match result.binary_search_by(|(needle, _)| needle.cmp(&key)) {
            Ok(i) | Err(i) => {
                result.insert(i, (key, style));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::assets::Assets;

    use super::*;

    #[test]
    fn test_bundled_themes() {
        let registry = ThemeRegistry::new(Assets);
        let mut has_default_theme = false;
        for theme_name in registry.list() {
            let theme = registry.get(&theme_name).unwrap();
            if theme.name == DEFAULT_THEME_NAME {
                has_default_theme = true;
            }
            assert_eq!(theme.name, theme_name);
        }
        assert!(has_default_theme);
    }

    #[test]
    fn test_theme_extension() {
        let assets = TestAssets(&[
            (
                "themes/_base.toml",
                r##"
                [ui.active_tab]
                extends = "ui.tab"
                border.color = "#666666"
                text = "$bright_text"

                [ui.tab]
                extends = "ui.element"
                text = "$dull_text"

                [ui.element]
                background = "#111111"
                border = {width = 2.0, color = "#00000000"}

                [editor]
                background = "#222222"
                default_text = "$regular_text"
                "##,
            ),
            (
                "themes/light.toml",
                r##"
                extends = "_base"

                [variables]
                bright_text = "#ffffff"
                regular_text = "#eeeeee"
                dull_text = "#dddddd"

                [editor]
                background = "#232323"
                "##,
            ),
        ]);

        let registry = ThemeRegistry::new(assets);
        let theme_data = registry.load("light").unwrap();
        assert_eq!(
            theme_data.as_ref(),
            &serde_json::json!({
              "ui": {
                "active_tab": {
                  "background": "#111111",
                  "border": {
                    "width": 2.0,
                    "color": "#666666"
                  },
                  "extends": "ui.tab",
                  "text": "#ffffff"
                },
                "tab": {
                  "background": "#111111",
                  "border": {
                    "width": 2.0,
                    "color": "#00000000"
                  },
                  "extends": "ui.element",
                  "text": "#dddddd"
                },
                "element": {
                  "background": "#111111",
                  "border": {
                    "width": 2.0,
                    "color": "#00000000"
                  }
                }
              },
              "editor": {
                "background": "#232323",
                "default_text": "#eeeeee"
              },
              "extends": "_base",
              "variables": {
                "bright_text": "#ffffff",
                "regular_text": "#eeeeee",
                "dull_text": "#dddddd"
              }
            })
        );
    }

    #[test]
    fn test_highlight_map() {
        let theme = Theme {
            name: "test".into(),
            ui: Default::default(),
            editor: Default::default(),
            syntax: [
                ("function", Color::from_u32(0x100000ff)),
                ("function.method", Color::from_u32(0x200000ff)),
                ("function.async", Color::from_u32(0x300000ff)),
                ("variable.builtin.self.rust", Color::from_u32(0x400000ff)),
                ("variable.builtin", Color::from_u32(0x500000ff)),
                ("variable", Color::from_u32(0x600000ff)),
            ]
            .iter()
            .map(|(name, color)| (name.to_string(), (*color).into()))
            .collect(),
        };

        let capture_names = &[
            "function.special".to_string(),
            "function.async.rust".to_string(),
            "variable.builtin.self".to_string(),
        ];

        let map = HighlightMap::new(capture_names, &theme);
        assert_eq!(theme.highlight_name(map.get(0)), Some("function"));
        assert_eq!(theme.highlight_name(map.get(1)), Some("function.async"));
        assert_eq!(theme.highlight_name(map.get(2)), Some("variable.builtin"));
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

        fn list(&self, prefix: &str) -> Vec<std::borrow::Cow<'static, str>> {
            self.0
                .iter()
                .copied()
                .filter_map(|(path, _)| {
                    if path.starts_with(prefix) {
                        Some(path.into())
                    } else {
                        None
                    }
                })
                .collect()
        }
    }
}
