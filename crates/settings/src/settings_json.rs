use std::{ops::Range, sync::LazyLock};

use anyhow::Result;
use schemars::schema::{
    ArrayValidation, InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tree_sitter::{Query, StreamingIterator as _};
use util::RangeExt;

pub struct SettingsJsonSchemaParams<'a> {
    pub language_names: &'a [String],
    pub font_names: &'a [String],
}

impl SettingsJsonSchemaParams<'_> {
    pub fn font_family_schema(&self) -> Schema {
        let available_fonts: Vec<_> = self.font_names.iter().cloned().map(Value::String).collect();

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(available_fonts),
            ..Default::default()
        }
        .into()
    }

    pub fn font_fallback_schema(&self) -> Schema {
        SchemaObject {
            instance_type: Some(SingleOrVec::Vec(vec![
                InstanceType::Array,
                InstanceType::Null,
            ])),
            array: Some(Box::new(ArrayValidation {
                items: Some(schemars::schema::SingleOrVec::Single(Box::new(
                    self.font_family_schema(),
                ))),
                unique_items: Some(true),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

type PropertyName<'a> = &'a str;
type ReferencePath<'a> = &'a str;

/// Modifies the provided [`RootSchema`] by adding references to all of the specified properties.
///
/// # Examples
///
/// ```
/// # let root_schema = RootSchema::default();
/// add_references_to_properties(&mut root_schema, &[
///     ("property_a", "#/definitions/DefinitionA"),
///     ("property_b", "#/definitions/DefinitionB"),
/// ])
/// ```
pub fn add_references_to_properties(
    root_schema: &mut RootSchema,
    properties_with_references: &[(PropertyName, ReferencePath)],
) {
    for (property, definition) in properties_with_references {
        let Some(schema) = root_schema.schema.object().properties.get_mut(*property) else {
            log::warn!("property '{property}' not found in JSON schema");
            continue;
        };

        match schema {
            Schema::Object(schema) => {
                schema.reference = Some(definition.to_string());
            }
            Schema::Bool(_) => {
                // Boolean schemas can't have references.
            }
        }
    }
}

pub fn update_value_in_json_text<'a>(
    text: &mut String,
    key_path: &mut Vec<&'a str>,
    tab_size: usize,
    old_value: &'a Value,
    new_value: &'a Value,
    preserved_keys: &[&str],
    edits: &mut Vec<(Range<usize>, String)>,
) {
    // If the old and new values are both objects, then compare them key by key,
    // preserving the comments and formatting of the unchanged parts. Otherwise,
    // replace the old value with the new value.
    if let (Value::Object(old_object), Value::Object(new_object)) = (old_value, new_value) {
        for (key, old_sub_value) in old_object.iter() {
            key_path.push(key);
            if let Some(new_sub_value) = new_object.get(key) {
                // Key exists in both old and new, recursively update
                update_value_in_json_text(
                    text,
                    key_path,
                    tab_size,
                    old_sub_value,
                    new_sub_value,
                    preserved_keys,
                    edits,
                );
            } else {
                // Key was removed from new object, remove the entire key-value pair
                let (range, replacement) = replace_value_in_json_text(text, key_path, 0, None);
                text.replace_range(range.clone(), &replacement);
                edits.push((range, replacement));
            }
            key_path.pop();
        }
        for (key, new_sub_value) in new_object.iter() {
            key_path.push(key);
            if !old_object.contains_key(key) {
                update_value_in_json_text(
                    text,
                    key_path,
                    tab_size,
                    &Value::Null,
                    new_sub_value,
                    preserved_keys,
                    edits,
                );
            }
            key_path.pop();
        }
    } else if key_path
        .last()
        .map_or(false, |key| preserved_keys.contains(key))
        || old_value != new_value
    {
        let mut new_value = new_value.clone();
        if let Some(new_object) = new_value.as_object_mut() {
            new_object.retain(|_, v| !v.is_null());
        }
        let (range, replacement) =
            replace_value_in_json_text(text, key_path, tab_size, Some(&new_value));
        text.replace_range(range.clone(), &replacement);
        edits.push((range, replacement));
    }
}

pub fn replace_value_in_json_text(
    text: &str,
    key_path: &[&str],
    tab_size: usize,
    new_value: Option<&Value>,
) -> (Range<usize>, String) {
    static PAIR_QUERY: LazyLock<Query> = LazyLock::new(|| {
        Query::new(
            &tree_sitter_json::LANGUAGE.into(),
            "(pair key: (string) @key value: (_) @value)",
        )
        .expect("Failed to create PAIR_QUERY")
    });

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();
    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = tree_sitter::QueryCursor::new();

    let mut depth = 0;
    let mut last_value_range = 0..0;
    let mut first_key_start = None;
    let mut existing_value_range = 0..text.len();
    let mut matches = cursor.matches(&PAIR_QUERY, syntax_tree.root_node(), text.as_bytes());
    while let Some(mat) = matches.next() {
        if mat.captures.len() != 2 {
            continue;
        }

        let key_range = mat.captures[0].node.byte_range();
        let value_range = mat.captures[1].node.byte_range();

        // Don't enter sub objects until we find an exact
        // match for the current keypath
        if last_value_range.contains_inclusive(&value_range) {
            continue;
        }

        last_value_range = value_range.clone();

        if key_range.start > existing_value_range.end {
            break;
        }

        first_key_start.get_or_insert(key_range.start);

        let found_key = text
            .get(key_range.clone())
            .map(|key_text| {
                depth < key_path.len() && key_text == format!("\"{}\"", key_path[depth])
            })
            .unwrap_or(false);

        if found_key {
            existing_value_range = value_range;
            // Reset last value range when increasing in depth
            last_value_range = existing_value_range.start..existing_value_range.start;
            depth += 1;

            if depth == key_path.len() {
                break;
            }

            first_key_start = None;
        }
    }

    // We found the exact key we want
    if depth == key_path.len() {
        if let Some(new_value) = new_value {
            let new_val = to_pretty_json(new_value, tab_size, tab_size * depth);
            (existing_value_range, new_val)
        } else {
            let mut removal_start = first_key_start.unwrap_or(existing_value_range.start);
            let mut removal_end = existing_value_range.end;

            // Find the actual key position by looking for the key in the pair
            // We need to extend the range to include the key, not just the value
            if let Some(key_start) = text[..existing_value_range.start].rfind('"') {
                if let Some(prev_key_start) = text[..key_start].rfind('"') {
                    removal_start = prev_key_start;
                } else {
                    removal_start = key_start;
                }
            }

            // Look backward for a preceding comma first
            let preceding_text = text.get(0..removal_start).unwrap_or("");
            if let Some(comma_pos) = preceding_text.rfind(',') {
                // Check if there are only whitespace characters between the comma and our key
                let between_comma_and_key = text.get(comma_pos + 1..removal_start).unwrap_or("");
                if between_comma_and_key.trim().is_empty() {
                    removal_start = comma_pos;
                }
            }

            if let Some(remaining_text) = text.get(existing_value_range.end..) {
                let mut chars = remaining_text.char_indices();
                while let Some((offset, ch)) = chars.next() {
                    if ch == ',' {
                        removal_end = existing_value_range.end + offset + 1;
                        // Also consume whitespace after the comma
                        while let Some((_, next_ch)) = chars.next() {
                            if next_ch.is_whitespace() {
                                removal_end += next_ch.len_utf8();
                            } else {
                                break;
                            }
                        }
                        break;
                    } else if !ch.is_whitespace() {
                        break;
                    }
                }
            }
            (removal_start..removal_end, String::new())
        }
    } else {
        // We have key paths, construct the sub objects
        let new_key = key_path[depth];

        // We don't have the key, construct the nested objects
        let mut new_value =
            serde_json::to_value(new_value.unwrap_or(&serde_json::Value::Null)).unwrap();
        for key in key_path[(depth + 1)..].iter().rev() {
            new_value = serde_json::json!({ key.to_string(): new_value });
        }

        if let Some(first_key_start) = first_key_start {
            let mut row = 0;
            let mut column = 0;
            for (ix, char) in text.char_indices() {
                if ix == first_key_start {
                    break;
                }
                if char == '\n' {
                    row += 1;
                    column = 0;
                } else {
                    column += char.len_utf8();
                }
            }

            if row > 0 {
                // depth is 0 based, but division needs to be 1 based.
                let new_val = to_pretty_json(&new_value, column / (depth + 1), column);
                let space = ' ';
                let content = format!("\"{new_key}\": {new_val},\n{space:width$}", width = column);
                (first_key_start..first_key_start, content)
            } else {
                let new_val = serde_json::to_string(&new_value).unwrap();
                let mut content = format!(r#""{new_key}": {new_val},"#);
                content.push(' ');
                (first_key_start..first_key_start, content)
            }
        } else {
            new_value = serde_json::json!({ new_key.to_string(): new_value });
            let indent_prefix_len = 4 * depth;
            let mut new_val = to_pretty_json(&new_value, 4, indent_prefix_len);
            if depth == 0 {
                new_val.push('\n');
            }

            (existing_value_range, new_val)
        }
    }
}

pub fn to_pretty_json(
    value: &impl Serialize,
    indent_size: usize,
    indent_prefix_len: usize,
) -> String {
    const SPACES: [u8; 32] = [b' '; 32];

    debug_assert!(indent_size <= SPACES.len());
    debug_assert!(indent_prefix_len <= SPACES.len());

    let mut output = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut output,
        serde_json::ser::PrettyFormatter::with_indent(&SPACES[0..indent_size.min(SPACES.len())]),
    );

    value.serialize(&mut ser).unwrap();
    let text = String::from_utf8(output).unwrap();

    let mut adjusted_text = String::new();
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            adjusted_text.push_str(str::from_utf8(&SPACES[0..indent_prefix_len]).unwrap());
        }
        adjusted_text.push_str(line);
        adjusted_text.push('\n');
    }
    adjusted_text.pop();
    adjusted_text
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json_lenient::from_str(content)?)
}

#[cfg(test)]
mod tests {
    use gpui::App;
    use unindent::Unindent;

    use crate::{
        Settings, SettingsStore,
        settings_store::tests::{
            LanguageSettingEntry, LanguageSettings, MultiKeySettings, UserSettings,
        },
    };

    fn check_settings_update<T: Settings>(
        store: &mut SettingsStore,
        old_json: String,
        update: fn(&mut T::FileContent),
        expected_new_json: String,
        cx: &mut App,
    ) {
        store.set_user_settings(&old_json, cx).ok();
        let edits = store.edits_for_update::<T>(&old_json, update);
        let mut new_json = old_json;
        for (range, replacement) in edits.into_iter() {
            new_json.replace_range(range, &replacement);
        }
        pretty_assertions::assert_eq!(new_json, expected_new_json);
    }

    fn check_keymap_update() {}

    #[gpui::test]
    fn test_setting_store_update(cx: &mut App) {
        let mut store = SettingsStore::new(cx);
        store.register_setting::<MultiKeySettings>(cx);
        store.register_setting::<UserSettings>(cx);
        store.register_setting::<LanguageSettings>(cx);

        // entries added and updated
        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                     "languages": {
                         "JSON": {
                             "language_setting_1": true
                         }
                     }
                 }"#
            .unindent(),
            |settings| {
                settings
                    .languages
                    .get_mut("JSON")
                    .unwrap()
                    .language_setting_1 = Some(false);
                settings.languages.insert(
                    "Rust".into(),
                    LanguageSettingEntry {
                        language_setting_2: Some(true),
                        ..Default::default()
                    },
                );
            },
            r#"{
                     "languages": {
                         "Rust": {
                             "language_setting_2": true
                         },
                         "JSON": {
                             "language_setting_1": false
                         }
                     }
                 }"#
            .unindent(),
            cx,
        );

        // entries removed
        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                     "languages": {
                         "Rust": {
                             "language_setting_2": true
                         },
                         "JSON": {
                             "language_setting_1": false
                         }
                     }
                 }"#
            .unindent(),
            |settings| {
                settings.languages.remove("JSON").unwrap();
            },
            r#"{
                     "languages": {
                         "Rust": {
                             "language_setting_2": true
                         }
                     }
                 }"#
            .unindent(),
            cx,
        );

        check_settings_update::<LanguageSettings>(
            &mut store,
            r#"{
                     "languages": {
                         "Rust": {
                             "language_setting_2": true
                         },
                         "JSON": {
                             "language_setting_1": false
                         }
                     }
                 }"#
            .unindent(),
            |settings| {
                settings.languages.remove("Rust").unwrap();
            },
            r#"{
                     "languages": {
                         "JSON": {
                             "language_setting_1": false
                         }
                     }
                 }"#
            .unindent(),
            cx,
        );

        // weird formatting
        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
                     "user":   { "age": 36, "name": "Max", "staff": true }
                 }"#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                     "user":   { "age": 37, "name": "Max", "staff": true }
                 }"#
            .unindent(),
            cx,
        );

        // single-line formatting, other keys
        check_settings_update::<MultiKeySettings>(
            &mut store,
            r#"{ "one": 1, "two": 2 }"#.unindent(),
            |settings| settings.key1 = Some("x".into()),
            r#"{ "key1": "x", "one": 1, "two": 2 }"#.unindent(),
            cx,
        );

        // empty object
        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
                     "user": {}
                 }"#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                     "user": {
                         "age": 37
                     }
                 }"#
            .unindent(),
            cx,
        );

        // no content
        check_settings_update::<UserSettings>(
            &mut store,
            r#""#.unindent(),
            |settings| settings.age = Some(37),
            r#"{
                     "user": {
                         "age": 37
                     }
                 }
                 "#
            .unindent(),
            cx,
        );

        check_settings_update::<UserSettings>(
            &mut store,
            r#"{
                 }
                 "#
            .unindent(),
            |settings| settings.age = Some(37),
            r#"{
                     "user": {
                         "age": 37
                     }
                 }
                 "#
            .unindent(),
            cx,
        );
    }
}
