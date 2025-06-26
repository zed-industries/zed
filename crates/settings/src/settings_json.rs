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

fn replace_value_in_json_text(
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

const TS_DOCUMENT_KIND: &'static str = "document";
const TS_ARRAY_KIND: &'static str = "array";
const TS_COMMENT_KIND: &'static str = "comment";

pub fn replace_top_level_array_value_in_json_text(
    text: &str,
    key_path: &[&str],
    new_value: Option<&Value>,
    array_index: usize,
    tab_size: usize,
) -> Result<(Range<usize>, String)> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();
    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = syntax_tree.walk();

    if cursor.node().kind() == TS_DOCUMENT_KIND {
        anyhow::ensure!(
            cursor.goto_first_child(),
            "Document empty - No top level array"
        );
    }

    while cursor.node().kind() != TS_ARRAY_KIND {
        anyhow::ensure!(cursor.goto_next_sibling(), "EOF - No top level array");
    }

    // false if no children
    //
    cursor.goto_first_child();
    debug_assert_eq!(cursor.node().kind(), "[");

    let mut index = 0;

    while index <= array_index {
        let node = cursor.node();
        if !matches!(node.kind(), "[" | "]" | TS_COMMENT_KIND | ",")
            && !node.is_extra()
            && !node.is_missing()
        {
            if index == array_index {
                break;
            }
            index += 1;
        }
        if !cursor.goto_next_sibling() {
            if let Some(new_value) = new_value {
                return append_top_level_array_value_in_json_text(text, new_value, tab_size);
            } else {
                return Ok((0..0, String::new()));
            }
        }
    }

    let range = cursor.node().range();
    let indent_width = range.start_point.column;
    let offset = range.start_byte;
    let value_str = std::str::from_utf8(&text.as_bytes()[range.start_byte..range.end_byte])?;
    let needs_indent = range.start_point.row > 0;

    let (mut replace_range, mut replace_value) =
        replace_value_in_json_text(value_str, key_path, tab_size, new_value);

    replace_range.start += offset;
    replace_range.end += offset;

    if needs_indent {
        let increased_indent = format!("\n{space:width$}", space = ' ', width = indent_width);
        replace_value = replace_value.replace('\n', &increased_indent);
        // replace_value.push('\n');
    } else {
        while let Some(idx) = replace_value.find("\n ") {
            replace_value.remove(idx + 1);
        }
        while let Some(idx) = replace_value.find("\n") {
            replace_value.replace_range(idx..idx + 1, " ");
        }
    }

    return Ok((replace_range, replace_value));
}

pub fn append_top_level_array_value_in_json_text(
    text: &str,
    new_value: &Value,
    tab_size: usize,
) -> Result<(Range<usize>, String)> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();
    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = syntax_tree.walk();

    if cursor.node().kind() == TS_DOCUMENT_KIND {
        anyhow::ensure!(
            cursor.goto_first_child(),
            "Document empty - No top level array"
        );
    }

    while cursor.node().kind() != TS_ARRAY_KIND {
        anyhow::ensure!(cursor.goto_next_sibling(), "EOF - No top level array");
    }

    // false if no children
    if !cursor.goto_last_child() {
        todo!("appends (if index is 0)")
    }
    debug_assert_eq!(cursor.node().kind(), "]");
    let close_bracket_start = cursor.node().start_byte();
    cursor.goto_previous_sibling();
    while (cursor.node().is_extra() || cursor.node().is_missing()) && cursor.goto_previous_sibling()
    {
    }

    let mut comma_range = None;
    let mut prev_item_range = None;
    let mut bracket_range = None;

    if cursor.node().kind() == "," {
        comma_range = Some(cursor.node().byte_range());
        while cursor.goto_previous_sibling() && cursor.node().is_extra() {}

        debug_assert_ne!(cursor.node().kind(), "[");
        prev_item_range = Some(cursor.node().range());
    } else {
        while (cursor.node().is_extra() || cursor.node().is_missing())
            && cursor.goto_previous_sibling()
        {}
        if cursor.node().kind() != "[" {
            prev_item_range = Some(cursor.node().range());
        } else {
            bracket_range = Some(cursor.node().byte_range());
        }
    }

    let (mut replace_range, mut replace_value) =
        replace_value_in_json_text("", &[], tab_size, Some(new_value));

    let offset = if let Some(comma_range) = &comma_range {
        comma_range.end
    } else if let Some(prev_item_range) = &prev_item_range {
        prev_item_range.end_byte
    } else {
        bracket_range.unwrap().end
    };

    replace_range.start += offset;
    replace_range.end = close_bracket_start;

    let space = ' ';
    if let Some(prev_item_range) = prev_item_range {
        let needs_newline = prev_item_range.start_point.row > 0;
        let indent_width = prev_item_range.start_point.column;

        if needs_newline {
            let increased_indent = format!("\n{space:width$}", width = indent_width);
            replace_value = replace_value.replace('\n', &increased_indent);
            replace_value.push('\n');
            replace_value.insert_str(0, &format!("\n{space:width$}", width = indent_width));
        } else {
            while let Some(idx) = replace_value.find("\n ") {
                replace_value.remove(idx + 1);
            }
            while let Some(idx) = replace_value.find("\n") {
                replace_value.replace_range(idx..idx + 1, " ");
            }
            replace_value.insert(0, ' ');
        }

        if comma_range.is_none() {
            replace_value.insert(0, ',');
        }
    } else {
        let indent = format!("\n{space:width$}", width = tab_size);
        replace_value = replace_value.replace('\n', &indent);
        replace_value.insert_str(0, &indent);
        replace_value.push('\n');
    }

    return Ok((replace_range, replace_value));
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
    use super::*;
    use serde_json::{Value, json};
    use unindent::Unindent;

    #[test]
    fn object_replace() {
        fn check_object_replace(
            input: String,
            key_path: &[&str],
            value: Option<Value>,
            expected: String,
        ) {
            let result = replace_value_in_json_text(&input, key_path, 4, value.as_ref());
            let mut result_str = input.to_string();
            result_str.replace_range(result.0, &result.1);
            pretty_assertions::assert_eq!(expected, result_str);
        }
        check_object_replace(
            r#"{
                "a": 1,
                "b": 2
            }"#
            .unindent(),
            &["b"],
            Some(json!(3)),
            r#"{
                "a": 1,
                "b": 3
            }"#
            .unindent(),
        );
        check_object_replace(
            r#"{
                "a": 1,
                "b": 2
            }"#
            .unindent(),
            &["b"],
            None,
            r#"{
                "a": 1
            }"#
            .unindent(),
        );
        check_object_replace(
            r#"{
                "a": 1,
                "b": 2
            }"#
            .unindent(),
            &["c"],
            Some(json!(3)),
            r#"{
                "c": 3,
                "a": 1,
                "b": 2
            }"#
            .unindent(),
        );
        check_object_replace(
            r#"{
                "a": 1,
                "b": {
                    "c": 2,
                    "d": 3,
                }
            }"#
            .unindent(),
            &["b", "c"],
            Some(json!([1, 2, 3])),
            r#"{
                "a": 1,
                "b": {
                    "c": [
                        1,
                        2,
                        3
                    ],
                    "d": 3,
                }
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "name": "old_name",
                "id": 123
            }"#
            .unindent(),
            &["name"],
            Some(json!("new_name")),
            r#"{
                "name": "new_name",
                "id": 123
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "enabled": false,
                "count": 5
            }"#
            .unindent(),
            &["enabled"],
            Some(json!(true)),
            r#"{
                "enabled": true,
                "count": 5
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "value": null,
                "other": "test"
            }"#
            .unindent(),
            &["value"],
            Some(json!(42)),
            r#"{
                "value": 42,
                "other": "test"
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "config": {
                    "old": true
                },
                "name": "test"
            }"#
            .unindent(),
            &["config"],
            Some(json!({"new": false, "count": 3})),
            r#"{
                "config": {
                    "new": false,
                    "count": 3
                },
                "name": "test"
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                // This is a comment
                "a": 1,
                "b": 2 // Another comment
            }"#
            .unindent(),
            &["b"],
            Some(json!({"foo": "bar"})),
            r#"{
                // This is a comment
                "a": 1,
                "b": {
                    "foo": "bar"
                } // Another comment
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{}"#.to_string(),
            &["new_key"],
            Some(json!("value")),
            r#"{
                "new_key": "value"
            }
            "#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "only_key": 123
            }"#
            .unindent(),
            &["only_key"],
            None,
            "{\n    \n}".to_string(),
        );

        check_object_replace(
            r#"{
                "level1": {
                    "level2": {
                        "level3": {
                            "target": "old"
                        }
                    }
                }
            }"#
            .unindent(),
            &["level1", "level2", "level3", "target"],
            Some(json!("new")),
            r#"{
                "level1": {
                    "level2": {
                        "level3": {
                            "target": "new"
                        }
                    }
                }
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "parent": {}
            }"#
            .unindent(),
            &["parent", "child"],
            Some(json!("value")),
            r#"{
                "parent": {
                    "child": "value"
                }
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "a": 1,
                "b": 2,
            }"#
            .unindent(),
            &["b"],
            Some(json!(3)),
            r#"{
                "a": 1,
                "b": 3,
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "items": [1, 2, 3],
                "count": 3
            }"#
            .unindent(),
            &["items", "1"],
            Some(json!(5)),
            r#"{
                "items": {
                    "1": 5
                },
                "count": 3
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "items": [1, 2, 3],
                "count": 3
            }"#
            .unindent(),
            &["items", "1"],
            None,
            r#"{
                "items": {
                    "1": null
                },
                "count": 3
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "items": [1, 2, 3],
                "count": 3
            }"#
            .unindent(),
            &["items"],
            Some(json!(["a", "b", "c", "d"])),
            r#"{
                "items": [
                    "a",
                    "b",
                    "c",
                    "d"
                ],
                "count": 3
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                "0": "zero",
                "1": "one"
            }"#
            .unindent(),
            &["1"],
            Some(json!("ONE")),
            r#"{
                "0": "zero",
                "1": "ONE"
            }"#
            .unindent(),
        );
    }

    #[test]
    fn array_replace() {
        #[track_caller]
        fn check_array_replace(
            input: impl ToString,
            index: usize,
            key_path: &[&str],
            value: Value,
            expected: impl ToString,
        ) {
            let input = input.to_string();
            let result = replace_top_level_array_value_in_json_text(
                &input,
                key_path,
                Some(&value),
                index,
                4,
            )
            .expect("replace succeeded");
            let mut result_str = input;
            result_str.replace_range(result.0, &result.1);
            pretty_assertions::assert_eq!(expected.to_string(), result_str);
        }

        check_array_replace(r#"[1, 3, 3]"#, 1, &[], json!(2), r#"[1, 2, 3]"#);
        check_array_replace(r#"[1, 3, 3]"#, 2, &[], json!(2), r#"[1, 3, 2]"#);
        check_array_replace(r#"[1, 3, 3,]"#, 3, &[], json!(2), r#"[1, 3, 3, 2]"#);
        check_array_replace(r#"[1, 3, 3,]"#, 100, &[], json!(2), r#"[1, 3, 3, 2]"#);
        check_array_replace(
            r#"[
                1,
                2,
                3,
            ]"#
            .unindent(),
            1,
            &[],
            json!({"foo": "bar", "baz": "qux"}),
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": "qux"
                },
                3,
            ]"#
            .unindent(),
        );
        check_array_replace(
            r#"[1, 3, 3,]"#,
            1,
            &[],
            json!({"foo": "bar", "baz": "qux"}),
            r#"[1, { "foo": "bar", "baz": "qux" }, 3,]"#,
        );

        check_array_replace(
            r#"[1, { "foo": "bar", "baz": "qux" }, 3,]"#,
            1,
            &["baz"],
            json!({"qux": "quz"}),
            r#"[1, { "foo": "bar", "baz": { "qux": "quz" } }, 3,]"#,
        );

        check_array_replace(
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": "qux"
                },
                3
            ]"#,
            1,
            &["baz"],
            json!({"qux": "quz"}),
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": {
                        "qux": "quz"
                    }
                },
                3
            ]"#,
        );

        check_array_replace(
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": {
                        "qux": "quz"
                    }
                },
                3
            ]"#,
            1,
            &["baz"],
            json!("qux"),
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": "qux"
                },
                3
            ]"#,
        );
    }

    #[test]
    fn array_append() {
        #[track_caller]
        fn check_array_append(input: impl ToString, value: Value, expected: impl ToString) {
            let input = input.to_string();
            let result = append_top_level_array_value_in_json_text(&input, &value, 4)
                .expect("append succeeded");
            let mut result_str = input;
            result_str.replace_range(result.0, &result.1);
            pretty_assertions::assert_eq!(expected.to_string(), result_str);
        }
        check_array_append(r#"[1, 3, 3]"#, json!(4), r#"[1, 3, 3, 4]"#);
        check_array_append(r#"[1, 3, 3,]"#, json!(4), r#"[1, 3, 3, 4]"#);
        check_array_append(r#"[1, 3, 3   ]"#, json!(4), r#"[1, 3, 3, 4]"#);
        check_array_append(r#"[1, 3, 3,   ]"#, json!(4), r#"[1, 3, 3, 4]"#);
        check_array_append(
            r#"[
                1,
                2,
                3
            ]"#
            .unindent(),
            json!(4),
            r#"[
                1,
                2,
                3,
                4
            ]"#
            .unindent(),
        );
        check_array_append(
            r#"[
                1,
                2,
                3,
            ]"#
            .unindent(),
            json!(4),
            r#"[
                1,
                2,
                3,
                4
            ]"#
            .unindent(),
        );
        check_array_append(
            r#"[
                1,
                2,
                3,
            ]"#
            .unindent(),
            json!({"foo": "bar", "baz": "qux"}),
            r#"[
                1,
                2,
                3,
                {
                    "foo": "bar",
                    "baz": "qux"
                }
            ]"#
            .unindent(),
        );
        check_array_append(
            r#"[ 1, 2, 3, ]"#.unindent(),
            json!({"foo": "bar", "baz": "qux"}),
            r#"[ 1, 2, 3, { "foo": "bar", "baz": "qux" }]"#.unindent(),
        );
        check_array_append(
            r#"[]"#,
            json!({"foo": "bar"}),
            r#"[
                {
                    "foo": "bar"
                }
            ]"#
            .unindent(),
        );
    }
}
