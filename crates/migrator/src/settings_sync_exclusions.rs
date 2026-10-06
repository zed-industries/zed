use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context as _, Result};
use serde_json::{Map, Value};
use settings_json::{
    infer_json_indent_size, parse_json_with_comments, replace_object_value_in_json_text,
};

const SETTINGS_SYNC_KEY: &str = "settings_sync";
const EXCLUDE_KEY: &str = "exclude";
const MARKER_PREFIX: &str = "__zed_settings_sync_excluded_value_";

type Pattern = Vec<String>;
type KeyPath = Vec<String>;

pub(crate) fn carry_exclusions(
    original_text: &str,
    migrated_text: &str,
    migrate: impl Fn(&str) -> Result<Option<String>>,
) -> Result<String> {
    let Ok(original) = parse_json_with_comments::<Value>(original_text) else {
        return Ok(migrated_text.to_string());
    };
    let mut patterns = exclusion_patterns(&original);
    if patterns.is_empty() || patterns.iter().any(Vec::is_empty) {
        return Ok(migrated_text.to_string());
    }
    patterns.sort();
    patterns.dedup();
    let marked_patterns = patterns
        .iter()
        .filter_map(|pattern| mark_pattern(&original, pattern))
        .collect::<Vec<_>>();
    if marked_patterns.is_empty() {
        return Ok(migrated_text.to_string());
    }

    let migrated = parse_json_with_comments::<Value>(migrated_text)
        .context("parsing migrated settings to carry settings sync exclusions")?;
    let mut carried_paths = BTreeSet::new();
    let mut exclude_changed_leaves = false;
    for marked_pattern in &marked_patterns {
        match marked_pattern
            .attempts
            .iter()
            .find_map(|marked| track_markers(marked, &migrated, &migrate))
        {
            Some(paths) => carried_paths.extend(paths),
            None => exclude_changed_leaves |= marked_pattern.has_payload,
        }
    }
    if exclude_changed_leaves {
        log::warn!(
            "settings sync: a settings migration depends on excluded values, excluding every setting it changed"
        );
        let original_leaves = leaves(&original);
        carried_paths.extend(
            leaves(&migrated)
                .into_iter()
                .filter(|(path, value)| original_leaves.get(path) != Some(value))
                .map(|(path, _)| path),
        );
    }
    carried_paths.retain(|path| !patterns.iter().any(|pattern| covers(pattern, path)));
    if carried_paths.is_empty() {
        return Ok(migrated_text.to_string());
    }

    let mut exclusion_list = migrated
        .get(SETTINGS_SYNC_KEY)
        .and_then(|settings_sync| settings_sync.get(EXCLUDE_KEY))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    exclusion_list.extend(
        carried_paths
            .iter()
            .map(|path| Value::String(to_pointer(path))),
    );
    let (range, replacement) = replace_object_value_in_json_text(
        migrated_text,
        &[SETTINGS_SYNC_KEY, EXCLUDE_KEY],
        infer_json_indent_size(migrated_text),
        Some(&Value::Array(exclusion_list)),
    );
    let mut carried_text = migrated_text.to_string();
    carried_text.replace_range(range, &replacement);
    Ok(carried_text)
}

fn exclusion_patterns(doc: &Value) -> Vec<Pattern> {
    let Some(root) = doc.as_object() else {
        return Vec::new();
    };
    let profile_scopes = root
        .get("profiles")
        .and_then(Value::as_object)
        .into_iter()
        .flat_map(Map::values)
        .filter_map(|profile| profile.get("settings"));
    std::iter::once(doc)
        .chain(root.values())
        .chain(profile_scopes)
        .filter_map(|scope| scope.get(SETTINGS_SYNC_KEY)?.get(EXCLUDE_KEY)?.as_array())
        .flatten()
        .filter_map(Value::as_str)
        .filter_map(parse_pointer)
        .collect()
}

fn parse_pointer(pointer: &str) -> Option<Pattern> {
    if pointer.is_empty() {
        return None;
    }
    if pointer == "/" {
        return Some(Vec::new());
    }
    Some(
        pointer
            .strip_prefix('/')
            .unwrap_or(pointer)
            .split('/')
            .map(|segment| segment.replace("~1", "/").replace("~0", "~"))
            .collect(),
    )
}

fn to_pointer(path: &KeyPath) -> String {
    path.iter()
        .map(|segment| format!("/{}", segment.replace('~', "~0").replace('/', "~1")))
        .collect()
}

fn covers(pattern: &Pattern, path: &KeyPath) -> bool {
    let mut states = pattern_closure(pattern, BTreeSet::from([0]));
    for segment in path {
        if states.contains(&pattern.len()) {
            return true;
        }
        if states.is_empty() {
            return false;
        }
        let next = states
            .iter()
            .filter_map(|&state| match pattern.get(state)?.as_str() {
                "**" => Some(state),
                "*" => Some(state + 1),
                literal => (literal == segment).then_some(state + 1),
            })
            .collect();
        states = pattern_closure(pattern, next);
    }
    states.contains(&pattern.len())
}

fn pattern_closure(pattern: &Pattern, mut states: BTreeSet<usize>) -> BTreeSet<usize> {
    let mut pending = states.iter().copied().collect::<Vec<_>>();
    while let Some(state) = pending.pop() {
        if pattern.get(state).is_some_and(|segment| segment == "**") && states.insert(state + 1) {
            pending.push(state + 1);
        }
    }
    states
}

struct MarkedPattern {
    has_payload: bool,
    attempts: Vec<Value>,
}

fn mark_pattern(original: &Value, pattern: &[String]) -> Option<MarkedPattern> {
    let mut marked = original.clone();
    let mut marker_count = 0;
    mark_matching(&mut marked, pattern, &mut marker_count);
    let payload_count = marker_count;
    let mut inserted = marked.clone();
    insert_markers(&mut inserted, pattern, &mut marker_count);

    let mut attempts = Vec::new();
    if marker_count > payload_count {
        attempts.push(inserted);
    }
    if payload_count > 0 {
        attempts.push(marked);
    }
    (!attempts.is_empty()).then_some(MarkedPattern {
        has_payload: payload_count > 0,
        attempts,
    })
}

fn track_markers(
    marked: &Value,
    migrated: &Value,
    migrate: &impl Fn(&str) -> Result<Option<String>>,
) -> Option<BTreeSet<KeyPath>> {
    let marked_migrated = rerun_migrations(marked, migrate)
        .inspect_err(|error| {
            log::debug!("settings sync: migrating marked exclusions failed: {error:#}")
        })
        .ok()?;
    if !tracks_markers(&marked_migrated, migrated) {
        return None;
    }
    let mut paths = BTreeSet::new();
    collect_marker_paths(&marked_migrated, &mut Vec::new(), &mut paths);
    Some(paths)
}

fn rerun_migrations(
    marked: &Value,
    migrate: &impl Fn(&str) -> Result<Option<String>>,
) -> Result<Value> {
    let marked_text = serde_json::to_string_pretty(marked)?;
    let marked_migrated_text = migrate(&marked_text)?.unwrap_or(marked_text);
    parse_json_with_comments::<Value>(&marked_migrated_text)
        .context("parsing marked settings to carry settings sync exclusions")
}

fn mark_matching(value: &mut Value, pattern: &[String], marker_count: &mut usize) {
    let Some((segment, rest)) = pattern.split_first() else {
        mark_leaves(value, marker_count);
        return;
    };
    if segment == "**" {
        let rest = skip_any_depth(rest);
        if rest.is_empty() {
            mark_leaves(value, marker_count);
            return;
        }
        let Value::Object(_) = value else {
            return;
        };
        mark_matching(value, rest, marker_count);
        if let Value::Object(object) = value {
            for child in object.values_mut().filter(|child| child.is_object()) {
                mark_matching(child, pattern, marker_count);
            }
        }
        return;
    }
    match value {
        Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                if segment == "*" || segment == key {
                    mark_matching(child, rest, marker_count);
                }
            }
        }
        Value::Array(_) => mark_leaves(value, marker_count),
        _ => {}
    }
}

fn mark_leaves(value: &mut Value, marker_count: &mut usize) {
    match value {
        Value::Object(object) => {
            for child in object.values_mut() {
                mark_leaves(child, marker_count);
            }
        }
        Value::String(text) if text.starts_with(MARKER_PREFIX) => {}
        _ => *value = next_marker(marker_count),
    }
}

fn insert_markers(value: &mut Value, pattern: &[String], marker_count: &mut usize) {
    let (Some((segment, rest)), Value::Object(object)) = (pattern.split_first(), value) else {
        return;
    };
    if segment == "**" {
        return;
    }
    if segment == "*" {
        for child in object.values_mut() {
            insert_markers(child, rest, marker_count);
        }
    } else if let Some(child) = object.get_mut(segment) {
        insert_markers(child, rest, marker_count);
    } else if !rest.iter().any(|segment| segment == "*" || segment == "**") {
        let absent = rest
            .iter()
            .rev()
            .fold(next_marker(marker_count), |child, key| {
                Value::Object(Map::from_iter([(key.clone(), child)]))
            });
        object.insert(segment.clone(), absent);
    }
}

fn skip_any_depth(pattern: &[String]) -> &[String] {
    let start = pattern
        .iter()
        .position(|segment| segment != "**")
        .unwrap_or(pattern.len());
    &pattern[start..]
}

fn next_marker(marker_count: &mut usize) -> Value {
    let marker = Value::String(format!("{MARKER_PREFIX}{marker_count}"));
    *marker_count += 1;
    marker
}

fn is_marker(value: &Value) -> bool {
    value
        .as_str()
        .is_some_and(|text| text.starts_with(MARKER_PREFIX))
}

fn tracks_markers(marked: &Value, real: &Value) -> bool {
    match (marked, real) {
        (Value::Object(marked), Value::Object(real)) => {
            real.keys().all(|key| marked.contains_key(key))
                && marked
                    .iter()
                    .all(|(key, marked_child)| match real.get(key) {
                        Some(real_child) => tracks_markers(marked_child, real_child),
                        None => only_markers(marked_child),
                    })
        }
        (marked, real) => is_marker(marked) && !real.is_object() || marked == real,
    }
}

fn only_markers(value: &Value) -> bool {
    match value {
        Value::Object(object) => object.values().all(only_markers),
        value => is_marker(value),
    }
}

fn collect_marker_paths(value: &Value, path: &mut KeyPath, paths: &mut BTreeSet<KeyPath>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                path.push(key.clone());
                collect_marker_paths(child, path, paths);
                path.pop();
            }
        }
        Value::Array(items) => {
            if items.iter().any(contains_marker) {
                paths.insert(path.clone());
            }
        }
        value if is_marker(value) => {
            paths.insert(path.clone());
        }
        _ => {}
    }
}

fn contains_marker(value: &Value) -> bool {
    match value {
        Value::Object(object) => object.values().any(contains_marker),
        Value::Array(items) => items.iter().any(contains_marker),
        value => is_marker(value),
    }
}

fn leaves(value: &Value) -> BTreeMap<KeyPath, &Value> {
    fn collect<'a>(
        value: &'a Value,
        path: &mut KeyPath,
        leaves: &mut BTreeMap<KeyPath, &'a Value>,
    ) {
        match value {
            Value::Object(object) if !object.is_empty() => {
                for (key, child) in object {
                    path.push(key.clone());
                    collect(child, path, leaves);
                    path.pop();
                }
            }
            leaf => {
                leaves.insert(path.clone(), leaf);
            }
        }
    }
    let mut result = BTreeMap::new();
    collect(value, &mut Vec::new(), &mut result);
    result
}
