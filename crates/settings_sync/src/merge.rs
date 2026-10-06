use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context as _, Result};
use serde_json::{Map, Value};
use settings_content::{PlatformOverrides, ReleaseChannelOverrides};
use settings_json::{
    infer_json_indent_size, parse_json_with_comments, replace_object_value_in_json_text,
};

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

pub fn unflatten(paths: &PathMap) -> Value {
    let mut doc = Value::Object(Map::new());
    for (path, value) in paths {
        set_at_path(&mut doc, path, value.clone());
    }
    doc
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
    pub changed_by: Option<String>,
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

        if is_exclusion_carrier(path)
            && let Some(exclusion_list) =
                merge_exclusion_lists(base_value, local_value, remote_value)
        {
            if let Some(exclusion_list) = exclusion_list {
                merged.insert(path.clone(), exclusion_list);
            }
            continue;
        }

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
                changed_by: None,
            });
            remote_value
        };
        if let Some(value) = winner {
            merged.insert(path.clone(), value.clone());
        }
    }

    resolve_structural_overlaps(base, local, remote, &mut merged, &mut conflicts);
    conflicts.sort_by(|left, right| left.path.cmp(&right.path));
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
        let (path, new_value) = match op {
            SyncOp::Set { path, value } => (path, Some(value)),
            SyncOp::Delete { path } => (path, None),
        };
        let (range, replacement) =
            replace_object_value_in_json_text(&text, path.segments(), tab_size, new_value);
        text.replace_range(range, &replacement);
    }
    text
}

pub fn apply_target_to_text(
    classifier: &DocumentClassifier,
    text: &str,
    current: &PathMap,
    target: &PathMap,
) -> Result<String> {
    let ops = diff_paths(current, target)
        .into_iter()
        .filter(|op| match op {
            SyncOp::Delete { path } => {
                !(is_empty_object(current.get(path)) && has_strict_descendant(target, path))
            }
            SyncOp::Set { path, value } => {
                !(is_empty_object(Some(value)) && has_strict_descendant(current, path))
            }
        })
        .flat_map(|op| match op {
            SyncOp::Set { path, value } => match current.get(&path) {
                Some(current_value) => object_edits(path, current_value, value),
                None => Vec::from([SyncOp::Set { path, value }]),
            },
            delete @ SyncOp::Delete { .. } => Vec::from([delete]),
        })
        .collect::<Vec<_>>();
    if ops.is_empty() {
        return Ok(text.to_string());
    }
    let tab_size = infer_json_indent_size(text);
    let mut new_text = apply_ops_to_text(text, &ops);
    let mut applied = parse_json_with_comments::<Value>(&new_text)
        .context("applying synced settings produced unparsable text")?;
    let target_doc = unflatten(target);
    for op in &ops {
        let SyncOp::Delete { path } = op else {
            continue;
        };
        let segments = path.segments();
        for ancestor_len in (1..segments.len()).rev() {
            let ancestor = SyncPath::from_segments(segments[..ancestor_len].iter().cloned());
            let target_keeps_ancestor = value_at_path(&target_doc, &ancestor).is_some();
            let is_empty_object = value_at_path(&applied, &ancestor)
                .and_then(Value::as_object)
                .is_some_and(Map::is_empty);
            if target_keeps_ancestor || !is_empty_object {
                break;
            }
            let (range, replacement) =
                replace_object_value_in_json_text(&new_text, ancestor.segments(), tab_size, None);
            new_text.replace_range(range, &replacement);
            remove_at_path(&mut applied, &ancestor);
        }
    }

    let applied = parse_json_with_comments::<Value>(&new_text)
        .context("applying synced settings produced unparsable text")?;
    let applied_paths = flatten_doc(classifier, &applied);
    if applied_paths != *target {
        let mismatched = applied_paths
            .keys()
            .chain(target.keys())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .filter(|path| applied_paths.get(*path) != target.get(*path))
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        anyhow::bail!(
            "applying synced settings produced unexpected values at {}",
            mismatched.join(", ")
        );
    }
    Ok(new_text)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PatternSegment {
    Literal(String),
    AnyOne,
    AnyDepth,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExclusionPattern(Vec<PatternSegment>);

impl ExclusionPattern {
    pub fn parse(pointer: &str) -> Option<Self> {
        if pointer == "/" {
            return Some(Self(Vec::new()));
        }
        SyncPath::parse(pointer)
            .map(|path| Self::from_segments(path.segments().iter().map(String::as_str)))
    }

    pub fn from_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> Self {
        let mut pattern = segments
            .into_iter()
            .map(|segment| match segment {
                "*" => PatternSegment::AnyOne,
                "**" => PatternSegment::AnyDepth,
                _ => PatternSegment::Literal(segment.to_owned()),
            })
            .collect::<Vec<_>>();
        pattern.dedup_by(|segment, previous| {
            *segment == PatternSegment::AnyDepth && *previous == PatternSegment::AnyDepth
        });
        Self(pattern)
    }

    fn covers(&self, path: &SyncPath) -> bool {
        let mut states = self.closure(BTreeSet::from([0]));
        for segment in path.segments() {
            if states.contains(&self.0.len()) {
                return true;
            }
            states = self.step(&states, segment);
        }
        states.contains(&self.0.len())
    }

    fn remainders_below(&self, path: &SyncPath) -> Vec<&[PatternSegment]> {
        let mut states = self.closure(BTreeSet::from([0]));
        for segment in path.segments() {
            if states.contains(&self.0.len()) {
                return Vec::new();
            }
            states = self.step(&states, segment);
        }
        if states.contains(&self.0.len()) {
            return Vec::new();
        }
        states
            .iter()
            .filter(|&&state| {
                state == 0
                    || self.0[state - 1] != PatternSegment::AnyDepth
                    || !states.contains(&(state - 1))
            })
            .map(|&state| &self.0[state..])
            .collect()
    }

    fn step(&self, states: &BTreeSet<usize>, key: &str) -> BTreeSet<usize> {
        let next = states
            .iter()
            .filter_map(|&state| match self.0.get(state)? {
                PatternSegment::AnyDepth => Some(state),
                PatternSegment::AnyOne => Some(state + 1),
                PatternSegment::Literal(literal) => (literal == key).then_some(state + 1),
            })
            .collect();
        self.closure(next)
    }

    fn closure(&self, mut states: BTreeSet<usize>) -> BTreeSet<usize> {
        let mut pending = states.iter().copied().collect::<Vec<_>>();
        while let Some(state) = pending.pop() {
            if self.0.get(state) == Some(&PatternSegment::AnyDepth) && states.insert(state + 1) {
                pending.push(state + 1);
            }
        }
        states
    }
}

impl PatternSegment {
    fn matches(&self, key: &str) -> bool {
        match self {
            Self::AnyOne | Self::AnyDepth => true,
            Self::Literal(literal) => literal == key,
        }
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
    &["context_servers", "*", "**", "env"],
    &["context_servers", "*", "headers"],
    &["context_servers", "*", "oauth"],
    &["agent_servers", "*", "env"],
    &["language_models", "lmstudio", "api_key"],
    &["language_models", "*", "custom_headers"],
    &["language_models", "*", "*", "custom_headers"],
];

const EXCLUSION_CARRIERS: &[&[&str]] = &[
    &[SETTINGS_SYNC_KEY, SETTINGS_SYNC_EXCLUDE_KEY],
    &["*", SETTINGS_SYNC_KEY, SETTINGS_SYNC_EXCLUDE_KEY],
    &[
        "profiles",
        "*",
        "settings",
        SETTINGS_SYNC_KEY,
        SETTINGS_SYNC_EXCLUDE_KEY,
    ],
];

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExclusionSet {
    patterns: BTreeSet<ExclusionPattern>,
}

impl ExclusionSet {
    pub fn built_in() -> Self {
        let mut exclusions = Self::default();
        for exclusion in BUILT_IN_EXCLUSIONS {
            exclusions.insert(ExclusionPattern::from_segments(exclusion.iter().copied()));
            for override_key in ReleaseChannelOverrides::OVERRIDE_KEYS
                .iter()
                .chain(PlatformOverrides::OVERRIDE_KEYS)
            {
                exclusions.insert(ExclusionPattern::from_segments(
                    std::iter::once(*override_key).chain(exclusion.iter().copied()),
                ));
            }
            exclusions.insert(ExclusionPattern::from_segments(
                ["profiles", "*", "settings"]
                    .into_iter()
                    .chain(exclusion.iter().copied()),
            ));
        }
        exclusions
    }

    pub fn insert(&mut self, pattern: ExclusionPattern) {
        self.patterns.insert(pattern);
    }

    pub fn extend(&mut self, other: Self) {
        self.patterns.extend(other.patterns);
    }

    pub fn extend_from_pointers<'a>(&mut self, pointers: impl IntoIterator<Item = &'a str>) {
        for pointer in pointers {
            match ExclusionPattern::parse(pointer) {
                Some(pattern) => self.insert(pattern),
                None => log::warn!("settings sync: invalid exclusion path {pointer:?}"),
            }
        }
    }

    pub fn extend_from_doc(&mut self, doc: &Value) {
        let Some(root) = doc.as_object() else {
            return;
        };
        let profile_scopes = root
            .get("profiles")
            .and_then(Value::as_object)
            .into_iter()
            .flat_map(|profiles| profiles.values())
            .filter_map(|profile| profile.get("settings"));
        for scope in std::iter::once(doc)
            .chain(root.values())
            .chain(profile_scopes)
        {
            if let Some(Value::Array(pointers)) = scope
                .get(SETTINGS_SYNC_KEY)
                .and_then(|settings_sync| settings_sync.get(SETTINGS_SYNC_EXCLUDE_KEY))
            {
                self.extend_from_pointers(pointers.iter().filter_map(Value::as_str));
            }
        }
    }

    pub fn is_excluded(&self, path: &SyncPath) -> bool {
        !is_exclusion_carrier(path) && self.patterns.iter().any(|pattern| pattern.covers(path))
    }

    pub fn pointers(&self) -> Vec<String> {
        self.patterns
            .iter()
            .map(|pattern| {
                if pattern.0.is_empty() {
                    return "/".to_string();
                }
                pattern
                    .0
                    .iter()
                    .map(|segment| match segment {
                        PatternSegment::AnyOne => "/*".to_string(),
                        PatternSegment::AnyDepth => "/**".to_string(),
                        PatternSegment::Literal(literal) => {
                            SyncPath::from_segments([literal.as_str()]).to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect()
    }

    pub fn strip(&self, paths: &mut PathMap) {
        paths.retain(|path, value| !self.redact_entry(path, value));
    }

    pub fn restore(&self, local: &PathMap, merged: &PathMap) -> PathMap {
        let mut restored = merged.clone();
        for (path, local_value) in local {
            if self.is_excluded(path) {
                restore_excluded_entry(&mut restored, path, local_value.clone());
                continue;
            }
            if is_exclusion_carrier(path) {
                continue;
            }
            let Some(excluded) = self
                .patterns
                .iter()
                .flat_map(|pattern| pattern.remainders_below(path))
                .filter_map(|remainder| extract_matching(local_value, remainder))
                .reduce(|mut excluded, part| {
                    merge_values(&mut excluded, part);
                    excluded
                })
            else {
                continue;
            };
            match restored.get_mut(path) {
                Some(restored_value) if restored_value.is_object() => {
                    merge_values(restored_value, excluded)
                }
                _ => restore_excluded_entry(&mut restored, path, excluded),
            }
        }
        restored
    }

    fn redact_entry(&self, path: &SyncPath, value: &mut Value) -> bool {
        if is_exclusion_carrier(path) {
            return false;
        }
        if self.is_excluded(path) {
            return true;
        }
        self.patterns.iter().any(|pattern| {
            pattern
                .remainders_below(path)
                .into_iter()
                .any(|remainder| redact_matching(value, remainder))
        })
    }
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
        if object.is_empty() {
            paths.insert(path, Value::Object(Map::new()));
            return;
        }
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

fn set_at_path(doc: &mut Value, path: &SyncPath, value: Value) {
    let mut current = doc;
    let segments = path.segments();
    let Some((leaf_key, parents)) = segments.split_last() else {
        return;
    };
    for segment in parents {
        if !current.is_object() {
            *current = Value::Object(Map::new());
        }
        let Value::Object(object) = current else {
            return;
        };
        current = object
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    if !current.is_object() {
        *current = Value::Object(Map::new());
    }
    if let Some(object) = current.as_object_mut() {
        object.insert(leaf_key.clone(), value);
    }
}

fn remove_at_path(doc: &mut Value, path: &SyncPath) {
    let Some((leaf_key, parents)) = path.segments().split_last() else {
        return;
    };
    let mut current = doc;
    for segment in parents {
        let Some(child) = current
            .as_object_mut()
            .and_then(|object| object.get_mut(segment))
        else {
            return;
        };
        current = child;
    }
    if let Some(object) = current.as_object_mut() {
        object.remove(leaf_key);
    }
}

fn resolve_structural_overlaps(
    base: &PathMap,
    local: &PathMap,
    remote: &PathMap,
    merged: &mut PathMap,
    conflicts: &mut Vec<Conflict>,
) {
    let paths = merged.keys().cloned().collect::<Vec<_>>();
    let mut compatible_markers = BTreeSet::new();
    let mut losers = BTreeSet::new();
    for (index, ancestor) in paths.iter().enumerate() {
        let ancestor_is_empty_marker = merged
            .get(ancestor)
            .and_then(Value::as_object)
            .is_some_and(Map::is_empty);
        for descendant in paths[index + 1..]
            .iter()
            .take_while(|path| path.starts_with(ancestor))
        {
            if ancestor_is_empty_marker {
                compatible_markers.insert(ancestor.clone());
            } else if remote.contains_key(descendant) {
                losers.insert(ancestor.clone());
            } else {
                losers.insert(descendant.clone());
            }
        }
    }

    for marker in compatible_markers {
        merged.remove(&marker);
    }
    for loser in losers {
        merged.remove(&loser);
        merged.extend(
            remote
                .range(loser.clone()..)
                .take_while(|(path, _)| path.starts_with(&loser))
                .map(|(path, value)| (path.clone(), value.clone())),
        );
        conflicts.push(Conflict {
            base: base.get(&loser).cloned(),
            local: local.get(&loser).cloned(),
            remote: remote.get(&loser).cloned(),
            path: loser,
            changed_by: None,
        });
    }
}

fn merge_exclusion_lists(
    base: Option<&Value>,
    local: Option<&Value>,
    remote: Option<&Value>,
) -> Option<Option<Value>> {
    fn as_list(value: Option<&Value>) -> Option<&[Value]> {
        match value {
            None => Some(&[]),
            Some(Value::Array(items)) => Some(items.as_slice()),
            Some(_) => None,
        }
    }
    let (base_items, local_items, remote_items) =
        (as_list(base)?, as_list(local)?, as_list(remote)?);
    let mut items = Vec::new();
    for item in local_items.iter().chain(remote_items) {
        let kept = !base_items.contains(item)
            || (local_items.contains(item) && remote_items.contains(item));
        if kept && !items.contains(item) {
            items.push(item.clone());
        }
    }
    Some(
        if items.is_empty() && (local.is_none() || remote.is_none()) {
            None
        } else {
            Some(Value::Array(items))
        },
    )
}

fn object_edits(path: SyncPath, current: &Value, target: Value) -> Vec<SyncOp> {
    match (current, target) {
        (Value::Object(current), Value::Object(target)) => {
            let mut ops = current
                .keys()
                .filter(|key| !target.contains_key(*key))
                .map(|key| SyncOp::Delete {
                    path: path.join(key),
                })
                .collect::<Vec<_>>();
            for (key, target_value) in target {
                match current.get(&key) {
                    Some(current_value) if *current_value == target_value => {}
                    Some(current_value) => {
                        ops.extend(object_edits(path.join(&key), current_value, target_value))
                    }
                    None => ops.push(SyncOp::Set {
                        path: path.join(&key),
                        value: target_value,
                    }),
                }
            }
            ops
        }
        (_, target) => Vec::from([SyncOp::Set {
            path,
            value: target,
        }]),
    }
}

pub(crate) fn union_exclusion_lists(merged: &mut PathMap, local: &PathMap, remote: &PathMap) {
    let carriers = local
        .keys()
        .chain(remote.keys())
        .filter(|path| is_exclusion_carrier(path))
        .cloned()
        .collect::<BTreeSet<_>>();
    for carrier in carriers {
        if let Some(Some(exclusion_list)) =
            merge_exclusion_lists(None, local.get(&carrier), remote.get(&carrier))
        {
            merged.retain(|path, value| {
                !(carrier.starts_with(path) && *path != carrier && is_empty_object(Some(value)))
            });
            merged.insert(carrier, exclusion_list);
        }
    }
}

fn is_empty_object(value: Option<&Value>) -> bool {
    value.and_then(Value::as_object).is_some_and(Map::is_empty)
}

fn has_strict_descendant(paths: &PathMap, path: &SyncPath) -> bool {
    paths
        .range(path.clone()..)
        .take_while(|(descendant, _)| descendant.starts_with(path))
        .any(|(descendant, _)| descendant != path)
}

fn is_exclusion_carrier(path: &SyncPath) -> bool {
    EXCLUSION_CARRIERS.iter().any(|carrier| {
        carrier.len() == path.segments().len()
            && carrier
                .iter()
                .zip(path.segments())
                .all(|(carrier_segment, segment)| {
                    *carrier_segment == "*" || carrier_segment == segment
                })
    })
}

fn restore_excluded_entry(restored: &mut PathMap, path: &SyncPath, value: Value) {
    restored.retain(|restored_path, restored_value| {
        let overlaps = restored_path.starts_with(path) || path.starts_with(restored_path);
        if overlaps && !restored_value.as_object().is_some_and(Map::is_empty) {
            log::warn!(
                "settings sync: keeping local excluded {path}, it replaces synced {restored_path}"
            );
        }
        !overlaps
    });
    restored.insert(path.clone(), value);
}

fn redact_matching(value: &mut Value, segments: &[PatternSegment]) -> bool {
    let Some((first, rest)) = segments.split_first() else {
        return true;
    };
    if *first == PatternSegment::AnyDepth {
        if !value.is_object() {
            return rest.is_empty();
        }
        if redact_matching(value, rest) {
            return true;
        }
        let Value::Object(object) = value else {
            return false;
        };
        let was_empty = object.is_empty();
        object.retain(|_, child| !(child.is_object() && redact_matching(child, segments)));
        return !was_empty && object.is_empty();
    }
    match value {
        Value::Array(_) => true,
        Value::Object(object) => {
            let was_empty = object.is_empty();
            object.retain(|key, child| !(first.matches(key) && redact_matching(child, rest)));
            !was_empty && object.is_empty()
        }
        _ => false,
    }
}

fn extract_matching(value: &Value, segments: &[PatternSegment]) -> Option<Value> {
    let Some((first, rest)) = segments.split_first() else {
        return Some(value.clone());
    };
    if *first == PatternSegment::AnyDepth {
        let Value::Object(object) = value else {
            return rest.is_empty().then(|| value.clone());
        };
        let mut extracted = extract_matching(value, rest);
        let nested = object
            .iter()
            .filter(|(_, child)| child.is_object())
            .filter_map(|(key, child)| Some((key.clone(), extract_matching(child, segments)?)))
            .collect::<Map<_, _>>();
        if !nested.is_empty() {
            match &mut extracted {
                Some(extracted) => merge_values(extracted, Value::Object(nested)),
                None => extracted = Some(Value::Object(nested)),
            }
        }
        return extracted;
    }
    match value {
        Value::Array(_) => Some(value.clone()),
        Value::Object(object) => {
            let extracted = object
                .iter()
                .filter(|(key, _)| first.matches(key))
                .filter_map(|(key, child)| Some((key.clone(), extract_matching(child, rest)?)))
                .collect::<Map<_, _>>();
            (!extracted.is_empty()).then_some(Value::Object(extracted))
        }
        _ => None,
    }
}

fn merge_values(target: &mut Value, source: Value) {
    match (target, source) {
        (Value::Object(target), Value::Object(source)) => {
            for (key, value) in source {
                match target.get_mut(&key) {
                    Some(existing) => merge_values(existing, value),
                    None => {
                        target.insert(key, value);
                    }
                }
            }
        }
        (target, source) => *target = source,
    }
}
