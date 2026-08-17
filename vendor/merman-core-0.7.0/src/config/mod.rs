use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct MermaidConfig(Arc<Value>);

impl Default for MermaidConfig {
    fn default() -> Self {
        Self::empty_object()
    }
}

impl MermaidConfig {
    pub fn empty_object() -> Self {
        Self(Arc::new(Value::Object(Map::new())))
    }

    pub fn from_value(value: Value) -> Self {
        Self(Arc::new(value))
    }

    pub fn as_value(&self) -> &Value {
        self.0.as_ref()
    }

    pub fn as_value_mut(&mut self) -> &mut Value {
        self.value_mut()
    }

    pub fn get_str(&self, dotted_path: &str) -> Option<&str> {
        let mut cur: &Value = self.0.as_ref();
        for segment in dotted_path.split('.') {
            cur = cur.as_object()?.get(segment)?;
        }
        cur.as_str()
    }

    pub fn get_bool(&self, dotted_path: &str) -> Option<bool> {
        let mut cur: &Value = self.0.as_ref();
        for segment in dotted_path.split('.') {
            cur = cur.as_object()?.get(segment)?;
        }
        cur.as_bool()
    }

    pub fn set_value(&mut self, dotted_path: &str, value: Value) {
        let root_value = self.value_mut();
        // Be defensive: callers can construct `MermaidConfig` from any JSON value via
        // `from_value`. Mermaid configs are objects; if we see a non-object here, coerce it
        // to an object so this API never panics on user input.
        if !root_value.is_object() {
            replace_value_nonrecursive(root_value, Value::Object(Map::new()));
        }

        let Value::Object(root) = root_value else {
            return;
        };
        let mut cur: &mut Map<String, Value> = root;
        let mut segments = dotted_path.split('.').peekable();
        while let Some(seg) = segments.next() {
            if segments.peek().is_none() {
                if let Some(old) = cur.insert(seg.to_string(), value) {
                    drop_value_nonrecursive(old);
                }
                return;
            }
            let slot = cur.entry(seg).or_insert_with(|| Value::Object(Map::new()));
            if !slot.is_object() {
                replace_value_nonrecursive(slot, Value::Object(Map::new()));
            }
            let Some(next) = slot.as_object_mut() else {
                return;
            };
            cur = next;
        }
    }

    pub fn deep_merge(&mut self, other: &Value) {
        let Value::Object(m) = other else {
            let base = self.value_mut();
            deep_merge_value(base, other);
            return;
        };
        if m.is_empty() {
            return;
        }
        let base = self.value_mut();
        deep_merge_value(base, other);
    }

    fn value_mut(&mut self) -> &mut Value {
        if Arc::strong_count(&self.0) != 1 || Arc::weak_count(&self.0) != 0 {
            self.0 = Arc::new(clone_value_nonrecursive(self.0.as_ref()));
        }
        Arc::make_mut(&mut self.0)
    }
}

impl Drop for MermaidConfig {
    fn drop(&mut self) {
        if let Some(value) = Arc::get_mut(&mut self.0) {
            let old = std::mem::replace(value, Value::Null);
            drop_value_nonrecursive(old);
        }
    }
}

pub(crate) fn mirror_legacy_font_family_into_theme_variables(config: &mut MermaidConfig) {
    let value = config.value_mut();
    mirror_legacy_font_family_into_theme_variables_value(value);
}

pub(crate) fn mirror_legacy_font_family_into_theme_variables_value(value: &mut Value) {
    let Some(root) = value.as_object_mut() else {
        return;
    };
    let Some(font_family) = root
        .get("fontFamily")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
    else {
        return;
    };

    let has_theme_font_family = root
        .get("themeVariables")
        .and_then(Value::as_object)
        .and_then(|theme_variables| theme_variables.get("fontFamily"))
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty());
    if has_theme_font_family {
        return;
    }

    let theme_variables = root
        .entry("themeVariables")
        .or_insert_with(|| Value::Object(Map::new()));
    if !theme_variables.is_object() {
        replace_value_nonrecursive(theme_variables, Value::Object(Map::new()));
    }
    if let Some(theme_variables) = theme_variables.as_object_mut() {
        if let Some(old) =
            theme_variables.insert("fontFamily".to_string(), Value::String(font_family))
        {
            drop_value_nonrecursive(old);
        }
    }
}

fn deep_merge_value(base: &mut Value, incoming: &Value) {
    let mut stack: Vec<Vec<String>> = vec![Vec::new()];

    while let Some(path) = stack.pop() {
        let Some(in_value) = value_at_key_path(incoming, &path) else {
            continue;
        };
        let Some(base_slot) = value_at_key_path_mut(base, &path) else {
            continue;
        };

        match (base_slot, in_value) {
            (Value::Object(base_map), Value::Object(in_map)) => {
                for (key, in_child) in in_map {
                    if base_map.contains_key(key) {
                        let mut child_path = path.clone();
                        child_path.push(key.clone());
                        stack.push(child_path);
                    } else {
                        base_map.insert(key.clone(), clone_value_nonrecursive(in_child));
                    }
                }
            }
            (base_slot, in_value) => {
                replace_value_nonrecursive(base_slot, clone_value_nonrecursive(in_value));
            }
        }
    }
}

fn value_at_key_path<'a>(mut value: &'a Value, path: &[String]) -> Option<&'a Value> {
    for key in path {
        value = value.as_object()?.get(key)?;
    }
    Some(value)
}

fn value_at_key_path_mut<'a>(mut value: &'a mut Value, path: &[String]) -> Option<&'a mut Value> {
    for key in path {
        value = value.as_object_mut()?.get_mut(key)?;
    }
    Some(value)
}

pub(crate) fn replace_value_nonrecursive(slot: &mut Value, value: Value) {
    let old = std::mem::replace(slot, value);
    drop_value_nonrecursive(old);
}

pub(crate) fn clone_value_nonrecursive(value: &Value) -> Value {
    let mut cloned: HashMap<*const Value, Value> = HashMap::new();
    let mut stack = vec![(value, false)];

    while let Some((current, visited)) = stack.pop() {
        let current_ptr = std::ptr::from_ref(current);
        if visited {
            let value = match current {
                Value::Null => Value::Null,
                Value::Bool(v) => Value::Bool(*v),
                Value::Number(v) => Value::Number(v.clone()),
                Value::String(v) => Value::String(v.clone()),
                Value::Array(items) => Value::Array(
                    items
                        .iter()
                        .filter_map(|item| cloned.remove(&std::ptr::from_ref(item)))
                        .collect(),
                ),
                Value::Object(entries) => {
                    let mut out = Map::new();
                    for (key, child) in entries {
                        if let Some(value) = cloned.remove(&std::ptr::from_ref(child)) {
                            out.insert(key.clone(), value);
                        }
                    }
                    Value::Object(out)
                }
            };
            cloned.insert(current_ptr, value);
        } else {
            stack.push((current, true));
            match current {
                Value::Array(items) => {
                    for item in items.iter().rev() {
                        stack.push((item, false));
                    }
                }
                Value::Object(entries) => {
                    for child in entries.values().rev() {
                        stack.push((child, false));
                    }
                }
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
            }
        }
    }

    cloned
        .remove(&std::ptr::from_ref(value))
        .unwrap_or(Value::Null)
}

pub(crate) fn drop_value_nonrecursive(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        match value {
            Value::Array(items) => {
                stack.extend(items);
            }
            Value::Object(entries) => {
                stack.extend(entries.into_values());
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mirror_legacy_font_family_populates_missing_theme_variable() {
        let mut cfg = MermaidConfig::from_value(json!({
            "fontFamily": "Courier"
        }));

        mirror_legacy_font_family_into_theme_variables(&mut cfg);

        assert_eq!(cfg.get_str("themeVariables.fontFamily"), Some("Courier"));
    }

    #[test]
    fn mirror_legacy_font_family_preserves_explicit_theme_variable() {
        let mut cfg = MermaidConfig::from_value(json!({
            "fontFamily": "Courier",
            "themeVariables": {
                "fontFamily": "Inter"
            }
        }));

        mirror_legacy_font_family_into_theme_variables(&mut cfg);

        assert_eq!(cfg.get_str("themeVariables.fontFamily"), Some("Inter"));
    }

    fn deep_config_value(depth: usize) -> Value {
        let mut value = Value::String("leaf".to_string());
        for idx in (0..depth).rev() {
            let mut map = Map::new();
            map.insert(format!("k{idx}"), value);
            value = Value::Object(map);
        }
        value
    }

    #[test]
    fn clone_on_write_handles_deep_config_with_small_stack() {
        const DEPTH: usize = 2_048;
        let value = deep_config_value(DEPTH);
        let handle = std::thread::Builder::new()
            .name("mermaid-config-deep-clone-on-write".to_string())
            .stack_size(64 * 1024)
            .spawn(move || {
                let original = MermaidConfig::from_value(value);
                let mut cloned = original.clone();
                cloned.set_value("theme", Value::String("default".to_string()));
                assert_eq!(cloned.get_str("theme"), Some("default"));
            })
            .expect("spawn deep config clone-on-write test");
        handle
            .join()
            .expect("deep config clone-on-write should finish without stack overflow");
    }
}
