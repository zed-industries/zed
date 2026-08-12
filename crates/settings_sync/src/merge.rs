use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use serde_json::Value;
use settings_content::{PlatformOverrides, ReleaseChannelOverrides};
use settings_json::{infer_json_indent_size, replace_value_in_json_text};

use crate::classifier::DocumentClassifier;
use crate::sync_path::SyncPath;

pub type PathMap = BTreeMap<SyncPath, Value>;

pub const SETTINGS_SYNC_KEY: &str = "settings_sync";
pub const SETTINGS_SYNC_ENABLED_KEY: &str = "enabled";
pub const SETTINGS_SYNC_EXCLUDE_KEY: &str = "exclude";

pub fn flatten_doc(classifier: &DocumentClassifier, doc: &Value) -> PathMap {
    let mut paths = PathMap::new();
    if let Value::Object(object) = doc {
        let root_view = classifier.container_view(classifier.root_schema());
        for (key, value) in object {
            let child_schema = root_view.as_ref().and_then(|view| view.child_schema(key));
            flatten_value(
                classifier,
                child_schema,
                SyncPath::root().join(key),
                value,
                &mut paths,
            );
        }
    }
    paths
}

fn flatten_value(
    classifier: &DocumentClassifier,
    schema: Option<&Value>,
    path: SyncPath,
    value: &Value,
    paths: &mut PathMap,
) {
    if let Value::Object(object) = value
        && let Some(view) = schema.and_then(|schema| classifier.container_view(schema))
    {
        for (key, child) in object {
            flatten_value(
                classifier,
                view.child_schema(key),
                path.join(key),
                child,
                paths,
            );
        }
    } else {
        paths.insert(path, value.clone());
    }
}

pub fn unflatten(paths: &PathMap) -> Value {
    let mut doc = Value::Object(serde_json::Map::new());
    for (path, value) in paths {
        set_at_path(&mut doc, path, value.clone());
    }
    doc
}

fn set_at_path(doc: &mut Value, path: &SyncPath, value: Value) {
    let mut current = doc;
    let segments = path.segments();
    let Some((leaf_key, parents)) = segments.split_last() else {
        return;
    };
    for segment in parents {
        if !current.is_object() {
            *current = Value::Object(serde_json::Map::new());
        }
        let Value::Object(object) = current else {
            return;
        };
        current = object
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
    }
    if !current.is_object() {
        *current = Value::Object(serde_json::Map::new());
    }
    if let Some(object) = current.as_object_mut() {
        object.insert(leaf_key.clone(), value);
    }
}

pub fn drop_prefix_overlaps(paths: &mut PathMap) {
    let mut overlapping = Vec::new();
    let mut previous: Option<&SyncPath> = None;
    for path in paths.keys() {
        if let Some(previous) = previous
            && path.starts_with(previous)
        {
            overlapping.push(previous.clone());
        }
        previous = Some(path);
    }
    for path in overlapping {
        log::warn!("settings sync: dropping {path}, it overlaps a deeper synced path");
        paths.remove(&path);
    }
}

pub fn value_at_path<'a>(doc: &'a Value, path: &SyncPath) -> Option<&'a Value> {
    let mut current = doc;
    for segment in path.segments() {
        current = current.as_object()?.get(segment)?;
    }
    Some(current)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conflict {
    pub path: SyncPath,
    pub base: Option<Value>,
    pub local: Option<Value>,
    pub remote: Option<Value>,
}

#[derive(Debug, Default, PartialEq)]
pub struct ThreeWayMerge {
    pub merged: PathMap,
    pub conflicts: Vec<Conflict>,
}

pub fn merge_three_way(base: &PathMap, local: &PathMap, remote: &PathMap) -> ThreeWayMerge {
    let mut paths = BTreeSet::new();
    paths.extend(base.keys());
    paths.extend(local.keys());
    paths.extend(remote.keys());

    let mut merged = PathMap::new();
    let mut conflicts = Vec::new();
    for path in paths {
        let base_value = base.get(path);
        let local_value = local.get(path);
        let remote_value = remote.get(path);

        let winner = if local_value == remote_value {
            local_value
        } else if local_value == base_value {
            remote_value
        } else if remote_value == base_value {
            local_value
        } else {
            conflicts.push(Conflict {
                path: path.clone(),
                base: base_value.cloned(),
                local: local_value.cloned(),
                remote: remote_value.cloned(),
            });
            remote_value
        };
        if let Some(value) = winner {
            merged.insert(path.clone(), value.clone());
        }
    }

    ThreeWayMerge { merged, conflicts }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncOp {
    Set { path: SyncPath, value: Value },
    Delete { path: SyncPath },
}

pub fn diff_paths(current: &PathMap, target: &PathMap) -> Vec<SyncOp> {
    let mut paths = BTreeSet::new();
    paths.extend(current.keys());
    paths.extend(target.keys());

    let mut deletions = Vec::new();
    let mut updates = Vec::new();
    for path in paths {
        match (current.get(path), target.get(path)) {
            (Some(current_value), Some(target_value)) if current_value == target_value => {}
            (_, Some(target_value)) => updates.push(SyncOp::Set {
                path: path.clone(),
                value: target_value.clone(),
            }),
            (Some(_), None) => deletions.push(SyncOp::Delete { path: path.clone() }),
            (None, None) => {}
        }
    }
    deletions.sort_by_key(|op| match op {
        SyncOp::Delete { path } | SyncOp::Set { path, .. } => {
            std::cmp::Reverse(path.segments().len())
        }
    });
    deletions.extend(updates);
    deletions
}

pub fn apply_ops_to_text(text: &str, ops: &[SyncOp]) -> String {
    let tab_size = infer_json_indent_size(text);
    let mut text = text.to_string();
    for op in ops {
        let (key_path, new_value) = match op {
            SyncOp::Set { path, value } => (path.segments(), Some(value)),
            SyncOp::Delete { path } => (path.segments(), None),
        };
        let (range, replacement): (Range<usize>, String) =
            replace_value_in_json_text(&text, key_path, tab_size, new_value, None);
        text.replace_range(range, &replacement);
    }
    text
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PatternSegment {
    Literal(String),
    AnyOne,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExclusionPattern(Vec<PatternSegment>);

impl ExclusionPattern {
    pub fn parse(pointer: &str) -> Option<Self> {
        SyncPath::parse(pointer)
            .map(|path| Self::from_segments(path.segments().iter().map(String::as_str)))
    }

    pub fn from_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> Self {
        Self(
            segments
                .into_iter()
                .map(|segment| {
                    if segment == "*" {
                        PatternSegment::AnyOne
                    } else {
                        PatternSegment::Literal(segment.to_owned())
                    }
                })
                .collect(),
        )
    }

    fn overlaps(&self, path: &SyncPath) -> bool {
        self.0
            .iter()
            .zip(path.segments())
            .all(|(pattern_segment, segment)| match pattern_segment {
                PatternSegment::AnyOne => true,
                PatternSegment::Literal(literal) => literal == segment,
            })
    }
}

// TODO kb cloud: exclusions synced from the group doc are a group-wide
// redaction lever (any member can strip the whole doc); state this in the
// RFC security section. Built-ins cannot be overridden by users — the
// long-term fix is schema-driven `#[machine_scope]` instead of this list;
// changing the list or its replication axes requires a
// NON_MIGRATION_SCHEMA_EPOCH_BUMPS bump.
pub(crate) const BUILT_IN_EXCLUSIONS: &[&[&str]] = &[
    &[SETTINGS_SYNC_KEY, SETTINGS_SYNC_ENABLED_KEY],
    &["proxy"],
    &["server_url"],
    &["credentials_url"],
    &["ssh_connections"],
    &["wsl_connections"],
    &["dev_container_connections"],
    &["audio", "input_audio_device"],
    &["audio", "output_audio_device"],
    &["terminal", "env"],
    &["lsp", "*", "binary", "env"],
    &["dap", "*", "env"],
    &["context_servers", "*", "env"],
    &["context_servers", "*", "headers"],
    &["context_servers", "*", "oauth"],
    &["agent_servers", "*", "env"],
];

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExclusionSet {
    patterns: BTreeSet<ExclusionPattern>,
}

impl ExclusionSet {
    pub fn built_in() -> Self {
        let mut this = Self::default();
        for exclusion in BUILT_IN_EXCLUSIONS {
            this.insert(ExclusionPattern::from_segments(exclusion.iter().copied()));
            for override_key in ReleaseChannelOverrides::OVERRIDE_KEYS
                .iter()
                .chain(PlatformOverrides::OVERRIDE_KEYS)
            {
                this.insert(ExclusionPattern::from_segments(
                    std::iter::once(*override_key).chain(exclusion.iter().copied()),
                ));
            }
            this.insert(ExclusionPattern::from_segments(
                ["profiles", "*", "settings"]
                    .into_iter()
                    .chain(exclusion.iter().copied()),
            ));
        }
        this
    }

    pub fn insert(&mut self, pattern: ExclusionPattern) {
        self.patterns.insert(pattern);
    }

    pub fn extend_from_pointers<'a>(&mut self, pointers: impl IntoIterator<Item = &'a str>) {
        for pointer in pointers {
            match ExclusionPattern::parse(pointer) {
                Some(pattern) => self.insert(pattern),
                None => log::warn!("settings sync: invalid exclusion path {pointer:?}"),
            }
        }
    }

    pub fn extend_from_flattened(&mut self, paths: &PathMap) {
        let exclude_path = SyncPath::from_segments([SETTINGS_SYNC_KEY, SETTINGS_SYNC_EXCLUDE_KEY]);
        let Some(Value::Array(exclusions)) = paths.get(&exclude_path) else {
            return;
        };
        self.extend_from_pointers(exclusions.iter().filter_map(Value::as_str));
    }

    pub fn is_excluded(&self, path: &SyncPath) -> bool {
        self.patterns.iter().any(|pattern| pattern.overlaps(path))
    }

    pub fn strip(&self, paths: &mut PathMap) {
        paths.retain(|path, _| !self.is_excluded(path));
    }
}
