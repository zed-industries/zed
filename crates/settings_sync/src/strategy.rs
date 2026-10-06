use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use parking_lot::Mutex;
use serde_json::Value;
use settings_json::parse_json_with_comments;

use crate::classifier::DocumentClassifier;
use crate::merge::{ExclusionSet, PathMap, apply_target_to_text, flatten_doc, unflatten};
use crate::settings_schema_epoch;
use crate::sync_path::SyncPath;

const EXCLUSION_MARKER: &str = "__zed_settings_sync_exclusion_marker__";

pub trait SyncStrategy: Send + Sync {
    fn flatten(&self, doc: &Value) -> PathMap;
    fn unflatten(&self, paths: &PathMap) -> Value;
    fn apply(&self, text: &str, current: &PathMap, target: &PathMap) -> Result<String>;
    fn migrate(&self, text: &str) -> Result<Option<String>>;
    fn built_in_exclusions(&self) -> ExclusionSet;
    fn schema_epoch(&self) -> u64;

    fn migrate_doc(&self, doc: &Value) -> Result<Value> {
        let text = serde_json::to_string_pretty(doc).context("serializing a synced document")?;
        match self.migrate(&text)? {
            Some(migrated) => parse_json_with_comments::<Value>(&migrated)
                .context("parsing a migrated synced document"),
            None => Ok(doc.clone()),
        }
    }

    fn user_exclusions(&self, docs: &[&Value]) -> ExclusionSet {
        let mut exclusions = ExclusionSet::default();
        for doc in docs {
            exclusions.extend_from_doc(doc);
        }
        exclusions
    }
}

pub struct SettingsJsonStrategy {
    classifier: DocumentClassifier,
    migrated_pointers: Mutex<HashMap<String, Option<String>>>,
}

impl SettingsJsonStrategy {
    pub fn user_settings() -> Arc<dyn SyncStrategy> {
        Arc::new(Self {
            classifier: DocumentClassifier::for_user_settings(),
            migrated_pointers: Mutex::default(),
        })
    }

    fn migrated_pointer(&self, pointer: &str) -> Option<String> {
        if let Some(migrated) = self.migrated_pointers.lock().get(pointer) {
            return migrated.clone();
        }
        let migrated = migrate_pointer(pointer);
        self.migrated_pointers
            .lock()
            .insert(pointer.to_string(), migrated.clone());
        migrated
    }
}

impl SyncStrategy for SettingsJsonStrategy {
    fn flatten(&self, doc: &Value) -> PathMap {
        flatten_doc(&self.classifier, doc)
    }

    fn unflatten(&self, paths: &PathMap) -> Value {
        unflatten(paths)
    }

    fn apply(&self, text: &str, current: &PathMap, target: &PathMap) -> Result<String> {
        apply_target_to_text(&self.classifier, text, current, target)
    }

    fn migrate(&self, text: &str) -> Result<Option<String>> {
        migrator::migrate_settings(text)
    }

    fn built_in_exclusions(&self) -> ExclusionSet {
        ExclusionSet::built_in()
    }

    fn schema_epoch(&self) -> u64 {
        settings_schema_epoch()
    }

    fn user_exclusions(&self, docs: &[&Value]) -> ExclusionSet {
        let mut exclusions = ExclusionSet::default();
        for doc in docs {
            exclusions.extend_from_doc(doc);
        }
        let migrated = exclusions
            .pointers()
            .iter()
            .filter_map(|pointer| self.migrated_pointer(pointer))
            .collect::<Vec<_>>();
        exclusions.extend_from_pointers(migrated.iter().map(String::as_str));
        exclusions
    }
}

fn migrate_pointer(pointer: &str) -> Option<String> {
    let path = SyncPath::parse(pointer).filter(|path| path.first() != Some(""))?;
    let marker = Value::String(EXCLUSION_MARKER.to_string());
    let doc = unflatten(&PathMap::from([(path.clone(), marker.clone())]));
    let migrated = migrator::migrate_settings(&serde_json::to_string(&doc).ok()?).ok()??;
    let migrated = parse_json_with_comments::<Value>(&migrated).ok()?;
    let migrated_path = find_value(&migrated, &marker, SyncPath::root())?;
    (migrated_path != path).then(|| migrated_path.to_string())
}

fn find_value(doc: &Value, needle: &Value, path: SyncPath) -> Option<SyncPath> {
    if doc == needle {
        return Some(path);
    }
    doc.as_object()?
        .iter()
        .find_map(|(key, value)| find_value(value, needle, path.join(key)))
}
