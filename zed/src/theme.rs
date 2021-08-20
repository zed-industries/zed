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
use std::{collections::HashMap, fmt, mem, sync::Arc};

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
    pub workspace: Workspace,
    pub tab: Tab,
    pub active_tab: Tab,
    pub sidebar: ContainerStyle,
    pub sidebar_icon: SidebarIcon,
    pub active_sidebar_icon: SidebarIcon,
    pub selector: Selector,
    pub editor: Editor,
    #[serde(deserialize_with = "deserialize_syntax_theme")]
    pub syntax: Vec<(String, TextStyle)>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Workspace {
    pub background: Color,
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
pub struct SidebarIcon {
    pub color: Color,
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

#[derive(Default)]
struct KeyPathReferenceSet {
    references: Vec<KeyPathReference>,
    reference_ids_by_source: Vec<usize>,
    reference_ids_by_target: Vec<usize>,
    dependencies: Vec<(usize, usize)>,
    dependency_counts: Vec<usize>,
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct KeyPathReference {
    target: KeyPath,
    source: KeyPath,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct KeyPath(Vec<Key>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Array(usize),
    Object(String),
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

        let theme_data = self.load(name, true)?;
        let mut theme = serde_json::from_value::<Theme>(theme_data.as_ref().clone())?;
        theme.name = name.into();
        let theme = Arc::new(theme);
        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }

    fn load(&self, name: &str, evaluate_references: bool) -> Result<Arc<Value>> {
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
                .load(&base_name, false)
                .with_context(|| format!("failed to load base theme {}", base_name))?
                .as_ref()
                .clone();
            if let Value::Object(mut base_theme_object) = base_theme_data {
                deep_merge_json(&mut base_theme_object, theme_data);
                theme_data = base_theme_object;
            }
        }

        // Find all of the key path references in the object, and then sort them according
        // to their dependencies.
        if evaluate_references {
            let mut key_path = KeyPath::default();
            let mut references = KeyPathReferenceSet::default();
            for (key, value) in theme_data.iter() {
                key_path.0.push(Key::Object(key.clone()));
                find_references(value, &mut key_path, &mut references);
                key_path.0.pop();
            }
            let sorted_references = references
                .top_sort()
                .map_err(|key_paths| anyhow!("cycle for key paths: {:?}", key_paths))?;

            // Now update objects to include the fields of objects they extend
            for KeyPathReference { source, target } in sorted_references {
                if let Some(source) = value_at(&mut theme_data, &source).cloned() {
                    let target = value_at(&mut theme_data, &target).unwrap();
                    if let Value::Object(target_object) = target.take() {
                        if let Value::Object(mut source_object) = source {
                            deep_merge_json(&mut source_object, target_object);
                            *target = Value::Object(source_object);
                        } else {
                            Err(anyhow!("extended key path {} is not an object", source))?;
                        }
                    } else {
                        *target = source;
                    }
                } else {
                    Err(anyhow!("invalid key path '{}'", source))?;
                }
            }
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

impl KeyPathReferenceSet {
    fn insert(&mut self, reference: KeyPathReference) {
        let id = self.references.len();
        let source_ix = self
            .reference_ids_by_source
            .binary_search_by_key(&&reference.source, |id| &self.references[*id].source)
            .unwrap_or_else(|i| i);
        let target_ix = self
            .reference_ids_by_target
            .binary_search_by_key(&&reference.target, |id| &self.references[*id].target)
            .unwrap_or_else(|i| i);

        self.populate_dependencies(id, &reference);
        self.reference_ids_by_source.insert(source_ix, id);
        self.reference_ids_by_target.insert(target_ix, id);
        self.references.push(reference);
    }

    fn top_sort(mut self) -> Result<Vec<KeyPathReference>, Vec<KeyPath>> {
        let mut results = Vec::with_capacity(self.references.len());
        let mut root_ids = Vec::with_capacity(self.references.len());

        // Find the initial set of references that have no dependencies.
        for (id, dep_count) in self.dependency_counts.iter().enumerate() {
            if *dep_count == 0 {
                root_ids.push(id);
            }
        }

        while results.len() < root_ids.len() {
            // Just to guarantee a stable result when the inputs are randomized,
            // sort references lexicographically in absence of any dependency relationship.
            root_ids[results.len()..].sort_by_key(|id| &self.references[*id]);

            let root_id = root_ids[results.len()];
            let root = mem::take(&mut self.references[root_id]);
            results.push(root);

            // Remove this reference as a dependency from any of its dependent references.
            if let Ok(dep_ix) = self
                .dependencies
                .binary_search_by_key(&root_id, |edge| edge.0)
            {
                let mut first_dep_ix = dep_ix;
                let mut last_dep_ix = dep_ix + 1;
                while first_dep_ix > 0 && self.dependencies[first_dep_ix - 1].0 == root_id {
                    first_dep_ix -= 1;
                }
                while last_dep_ix < self.dependencies.len()
                    && self.dependencies[last_dep_ix].0 == root_id
                {
                    last_dep_ix += 1;
                }

                // If any reference no longer has any dependencies, then then mark it as a root.
                // Preserve the references' original order where possible.
                for (_, successor_id) in self.dependencies.drain(first_dep_ix..last_dep_ix) {
                    self.dependency_counts[successor_id] -= 1;
                    if self.dependency_counts[successor_id] == 0 {
                        root_ids.push(successor_id);
                    }
                }
            }
        }

        // If any references never became roots, then there are reference cycles
        // in the set. Return an error containing all of the key paths that are
        // directly involved in cycles.
        if results.len() < self.references.len() {
            let mut cycle_ref_ids = (0..self.references.len())
                .filter(|id| !root_ids.contains(id))
                .collect::<Vec<_>>();

            // Iteratively remove any references that have no dependencies,
            // so that the error will only indicate which key paths are directly
            // involved in the cycles.
            let mut done = false;
            while !done {
                done = true;
                cycle_ref_ids.retain(|id| {
                    if self.dependencies.iter().any(|dep| dep.0 == *id) {
                        true
                    } else {
                        done = false;
                        self.dependencies.retain(|dep| dep.1 != *id);
                        false
                    }
                });
            }

            let mut cycle_key_paths = Vec::new();
            for id in cycle_ref_ids {
                let reference = &self.references[id];
                cycle_key_paths.push(reference.target.clone());
                cycle_key_paths.push(reference.source.clone());
            }
            cycle_key_paths.sort_unstable();
            return Err(cycle_key_paths);
        }

        Ok(results)
    }

    fn populate_dependencies(&mut self, new_id: usize, new_reference: &KeyPathReference) {
        self.dependency_counts.push(0);

        // If an existing reference's source path starts with the new reference's
        // target path, then insert this new reference before that existing reference.
        for id in Self::reference_ids_for_key_path(
            &new_reference.target.0,
            &self.references,
            &self.reference_ids_by_source,
            KeyPathReference::source,
            KeyPath::starts_with,
        ) {
            Self::add_dependency(
                (new_id, id),
                &mut self.dependencies,
                &mut self.dependency_counts,
            );
        }

        // If an existing reference's target path starts with the new reference's
        // source path, then insert this new reference after that existing reference.
        for id in Self::reference_ids_for_key_path(
            &new_reference.source.0,
            &self.references,
            &self.reference_ids_by_target,
            KeyPathReference::target,
            KeyPath::starts_with,
        ) {
            Self::add_dependency(
                (id, new_id),
                &mut self.dependencies,
                &mut self.dependency_counts,
            );
        }

        // If an existing reference's source path is a prefix of the new reference's
        // target path, then insert this new reference before that existing reference.
        for prefix in new_reference.target.prefixes() {
            for id in Self::reference_ids_for_key_path(
                prefix,
                &self.references,
                &self.reference_ids_by_source,
                KeyPathReference::source,
                PartialEq::eq,
            ) {
                Self::add_dependency(
                    (new_id, id),
                    &mut self.dependencies,
                    &mut self.dependency_counts,
                );
            }
        }

        // If an existing reference's target path is a prefix of the new reference's
        // source path, then insert this new reference after that existing reference.
        for prefix in new_reference.source.prefixes() {
            for id in Self::reference_ids_for_key_path(
                prefix,
                &self.references,
                &self.reference_ids_by_target,
                KeyPathReference::target,
                PartialEq::eq,
            ) {
                Self::add_dependency(
                    (id, new_id),
                    &mut self.dependencies,
                    &mut self.dependency_counts,
                );
            }
        }
    }

    // Find all existing references that satisfy a given predicate with respect
    // to a given key path. Use a sorted array of reference ids in order to avoid
    // performing unnecessary comparisons.
    fn reference_ids_for_key_path<'a>(
        key_path: &[Key],
        references: &[KeyPathReference],
        sorted_reference_ids: &'a [usize],
        reference_attribute: impl Fn(&KeyPathReference) -> &KeyPath,
        predicate: impl Fn(&KeyPath, &[Key]) -> bool,
    ) -> impl Iterator<Item = usize> + 'a {
        let ix = sorted_reference_ids
            .binary_search_by_key(&key_path, |id| &reference_attribute(&references[*id]).0)
            .unwrap_or_else(|i| i);

        let mut start_ix = ix;
        while start_ix > 0 {
            let reference_id = sorted_reference_ids[start_ix - 1];
            let reference = &references[reference_id];
            if !predicate(&reference_attribute(reference), key_path) {
                break;
            }
            start_ix -= 1;
        }

        let mut end_ix = ix;
        while end_ix < sorted_reference_ids.len() {
            let reference_id = sorted_reference_ids[end_ix];
            let reference = &references[reference_id];
            if !predicate(&reference_attribute(reference), key_path) {
                break;
            }
            end_ix += 1;
        }

        sorted_reference_ids[start_ix..end_ix].iter().copied()
    }

    fn add_dependency(
        (predecessor, successor): (usize, usize),
        dependencies: &mut Vec<(usize, usize)>,
        dependency_counts: &mut Vec<usize>,
    ) {
        let dependency = (predecessor, successor);
        if let Err(i) = dependencies.binary_search(&dependency) {
            dependencies.insert(i, dependency);
        }
        dependency_counts[successor] += 1;
    }
}

impl KeyPathReference {
    fn source(&self) -> &KeyPath {
        &self.source
    }

    fn target(&self) -> &KeyPath {
        &self.target
    }
}

impl KeyPath {
    fn new(string: &str) -> Self {
        Self(
            string
                .split(".")
                .map(|key| Key::Object(key.to_string()))
                .collect(),
        )
    }

    fn starts_with(&self, other: &[Key]) -> bool {
        self.0.starts_with(&other)
    }

    fn prefixes(&self) -> impl Iterator<Item = &[Key]> {
        (1..self.0.len()).map(move |end_ix| &self.0[0..end_ix])
    }
}

impl PartialEq<[Key]> for KeyPath {
    fn eq(&self, other: &[Key]) -> bool {
        self.0.eq(other)
    }
}

impl fmt::Debug for KeyPathReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KeyPathReference {{ {} <- {} }}",
            self.target, self.source
        )
    }
}

impl fmt::Display for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, key) in self.0.iter().enumerate() {
            match key {
                Key::Array(index) => write!(f, "[{}]", index)?,
                Key::Object(key) => {
                    if i > 0 {
                        ".".fmt(f)?;
                    }
                    key.fmt(f)?;
                }
            }
        }
        Ok(())
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

fn find_references(value: &Value, key_path: &mut KeyPath, references: &mut KeyPathReferenceSet) {
    match value {
        Value::Array(vec) => {
            for (ix, value) in vec.iter().enumerate() {
                key_path.0.push(Key::Array(ix));
                find_references(value, key_path, references);
                key_path.0.pop();
            }
        }
        Value::Object(map) => {
            for (key, value) in map.iter() {
                if key == "extends" {
                    if let Some(source_path) = value.as_str().and_then(|s| s.strip_prefix("$")) {
                        references.insert(KeyPathReference {
                            source: KeyPath::new(source_path),
                            target: key_path.clone(),
                        });
                    }
                } else {
                    key_path.0.push(Key::Object(key.to_string()));
                    find_references(value, key_path, references);
                    key_path.0.pop();
                }
            }
        }
        Value::String(string) => {
            if let Some(source_path) = string.strip_prefix("$") {
                references.insert(KeyPathReference {
                    source: KeyPath::new(source_path),
                    target: key_path.clone(),
                });
            }
        }
        _ => {}
    }
}

fn value_at<'a>(object: &'a mut Map<String, Value>, key_path: &KeyPath) -> Option<&'a mut Value> {
    let mut key_path = key_path.0.iter();
    if let Some(Key::Object(first_key)) = key_path.next() {
        let mut cur_value = object.get_mut(first_key);
        for key in key_path {
            if let Some(value) = cur_value {
                match key {
                    Key::Array(ix) => cur_value = value.get_mut(ix),
                    Key::Object(key) => cur_value = value.get_mut(key),
                }
            } else {
                return None;
            }
        }
        cur_value
    } else {
        None
    }
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
    use rand::{prelude::StdRng, Rng};

    use super::*;
    use crate::assets::Assets;

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
                extends = "$ui.tab"
                border.color = "#666666"
                text = "$text_colors.bright"

                [ui.tab]
                extends = "$ui.element"
                text = "$text_colors.dull"

                [ui.element]
                background = "#111111"
                border = {width = 2.0, color = "#00000000"}

                [editor]
                background = "#222222"
                default_text = "$text_colors.regular"
                "##,
            ),
            (
                "themes/light.toml",
                r##"
                extends = "_base"

                [text_colors]
                bright = "#ffffff"
                regular = "#eeeeee"
                dull = "#dddddd"

                [editor]
                background = "#232323"
                "##,
            ),
        ]);

        let registry = ThemeRegistry::new(assets);
        let theme_data = registry.load("light", true).unwrap();
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
                  "extends": "$ui.tab",
                  "text": "#ffffff"
                },
                "tab": {
                  "background": "#111111",
                  "border": {
                    "width": 2.0,
                    "color": "#00000000"
                  },
                  "extends": "$ui.element",
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
              "text_colors": {
                "bright": "#ffffff",
                "regular": "#eeeeee",
                "dull": "#dddddd"
              }
            })
        );
    }

    #[test]
    fn test_highlight_map() {
        let theme = Theme {
            name: "test".into(),
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
            ..Default::default()
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

    #[test]
    fn test_key_path_reference_set_simple() {
        let input_references = build_refs(&[
            ("r", "a"),
            ("a.b.c", "d"),
            ("d.e", "f"),
            ("t.u", "v"),
            ("v.w", "x"),
            ("v.y", "x"),
            ("d.h", "i"),
            ("v.z", "x"),
            ("f.g", "d.h"),
        ]);
        let expected_references = build_refs(&[
            ("d.h", "i"),
            ("f.g", "d.h"),
            ("d.e", "f"),
            ("a.b.c", "d"),
            ("r", "a"),
            ("v.w", "x"),
            ("v.y", "x"),
            ("v.z", "x"),
            ("t.u", "v"),
        ])
        .collect::<Vec<_>>();

        let mut reference_set = KeyPathReferenceSet::default();
        for reference in input_references {
            reference_set.insert(reference);
        }
        assert_eq!(reference_set.top_sort().unwrap(), expected_references);
    }

    #[test]
    fn test_key_path_reference_set_with_cycles() {
        let input_references = build_refs(&[
            ("x", "a.b"),
            ("y", "x.c"),
            ("a.b.c", "d.e"),
            ("d.e.f", "g.h"),
            ("g.h.i", "a"),
        ]);

        let mut reference_set = KeyPathReferenceSet::default();
        for reference in input_references {
            reference_set.insert(reference);
        }

        assert_eq!(
            reference_set.top_sort().unwrap_err(),
            &[
                KeyPath::new("a"),
                KeyPath::new("a.b.c"),
                KeyPath::new("d.e"),
                KeyPath::new("d.e.f"),
                KeyPath::new("g.h"),
                KeyPath::new("g.h.i"),
            ]
        );
    }

    #[gpui::test(iterations = 20)]
    async fn test_key_path_reference_set_random(mut rng: StdRng) {
        let examples: &[&[_]] = &[
            &[
                ("n.d.h", "i"),
                ("f.g", "n.d.h"),
                ("n.d.e", "f"),
                ("a.b.c", "n.d"),
                ("r", "a"),
                ("q.q.q", "r.s"),
                ("r.t", "q"),
                ("x.x", "r.r"),
                ("v.w", "x"),
                ("v.y", "x"),
                ("v.z", "x"),
                ("t.u", "v"),
            ],
            &[
                ("w.x.y.z", "t.u.z"),
                ("x", "w.x"),
                ("a.b.c1", "x.b1.c"),
                ("a.b.c2", "x.b2.c"),
            ],
            &[
                ("x.y", "m.n.n.o.q"),
                ("x.y.z", "m.n.n.o.p"),
                ("u.v.w", "x.y.z"),
                ("a.b.c.d", "u.v"),
                ("a.b.c.d.e", "u.v"),
                ("a.b.c.d.f", "u.v"),
                ("a.b.c.d.g", "u.v"),
            ],
        ];

        for example in examples {
            let expected_references = build_refs(example).collect::<Vec<_>>();
            let mut input_references = expected_references.clone();
            input_references.sort_by_key(|_| rng.gen_range(0..1000));
            let mut reference_set = KeyPathReferenceSet::default();
            for reference in input_references {
                reference_set.insert(reference);
            }
            assert_eq!(reference_set.top_sort().unwrap(), expected_references);
        }
    }

    fn build_refs<'a>(rows: &'a [(&str, &str)]) -> impl Iterator<Item = KeyPathReference> + 'a {
        rows.iter().map(|(target, source)| KeyPathReference {
            target: KeyPath::new(target),
            source: KeyPath::new(source),
        })
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
