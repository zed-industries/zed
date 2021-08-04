use anyhow::{anyhow, Context, Result};
use gpui::{
    color::Color,
    elements::{ContainerStyle, LabelStyle},
    fonts::Properties as FontProperties,
    AssetSource,
};
use json::{Map, Value};
use parking_lot::Mutex;
use serde::Deserialize;
use serde_json as json;
use std::{cmp::Ordering, collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    theme_data: Mutex<HashMap<String, Arc<Map<String, Value>>>>,
}

#[derive(Debug, Default)]
pub struct Theme {
    pub ui: Ui,
    pub editor: Editor,
    pub syntax: Vec<(String, Color, FontProperties)>,
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

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        todo!()
        // if let Some(theme) = self.themes.lock().get(name) {
        //     return Ok(theme.clone());
        // }

        // let theme_toml = self.load(name)?;
        // let mut syntax = Vec::<(String, Color, FontProperties)>::new();
        // for (key, style) in theme_toml.syntax.iter() {
        //     let mut color = Color::default();
        //     let mut properties = FontProperties::new();
        //     match style {
        //         Value::Object(object) => {
        //             if let Some(value) = object.get("color") {
        //                 color = serde_json::from_value(value.clone())?;
        //             }
        //             if let Some(Value::Bool(true)) = object.get("italic") {
        //                 properties.style = FontStyle::Italic;
        //             }
        //             properties.weight = deserialize_weight(object.get("weight"))?;
        //         }
        //         _ => {
        //             color = serde_json::from_value(style.clone())?;
        //         }
        //     }
        //     match syntax.binary_search_by_key(&key, |e| &e.0) {
        //         Ok(i) | Err(i) => {
        //             syntax.insert(i, (key.to_string(), color, properties));
        //         }
        //     }
        // }

        // let theme = Arc::new(Theme {
        //     ui: theme::Ui::deserialize(MapDeserializer::new(theme_toml.ui.clone().into_iter()))?,
        //     editor: theme::Editor::deserialize(MapDeserializer::new(
        //         theme_toml.editor.clone().into_iter(),
        //     ))?,
        //     syntax,
        // });

        // self.themes.lock().insert(name.to_string(), theme.clone());
        // Ok(theme)
    }

    fn load(&self, name: &str) -> Result<Arc<Map<String, Value>>> {
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
            let mut base_theme_data = self
                .load(&base_name)
                .with_context(|| format!("failed to load base theme {}", base_name))?
                .as_ref()
                .clone();
            deep_merge_json(&mut base_theme_data, theme_data);
            theme_data = base_theme_data;
        }

        // Evaluate `extends` fields in styles
        let mut directives = Vec::new();
        let mut key_path = Vec::new();
        for (key, value) in theme_data.iter() {
            if value.is_array() || value.is_object() {
                key_path.push(Key::Object(key.clone()));
                find_extensions(value, &mut key_path, &mut directives);
                key_path.pop();
            }
        }
        directives.sort_unstable();
        for ExtendDirective {
            source_path,
            target_path,
        } in directives
        {
            let source = value_at(&mut theme_data, &source_path)?.clone();
            let target = value_at(&mut theme_data, &target_path)?;
            if let Value::Object(source_object) = source {
                deep_merge_json(target.as_object_mut().unwrap(), source_object);
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

        let result = Arc::new(theme_data);
        self.theme_data
            .lock()
            .insert(name.to_string(), result.clone());

        Ok(result)
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

#[derive(Clone, PartialEq, Eq)]
enum Key {
    Array(usize),
    Object(String),
}

#[derive(PartialEq, Eq)]
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
