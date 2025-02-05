use anyhow::Result;
use core::str;
use serde::{de::DeserializeOwned, Serialize};
use std::{ops::Range, sync::LazyLock};
use tree_sitter::Query;
use util::RangeExt;

pub fn update_value_in_json_text<'a>(
    text: &mut String,
    key_path: &mut Vec<&'a str>,
    tab_size: usize,
    old_value: &'a serde_json::Value,
    new_value: &'a serde_json::Value,
    preserved_keys: &[&str],
    edits: &mut Vec<(Range<usize>, String)>,
) {
    // If the old and new values are both objects, then compare them key by key,
    // preserving the comments and formatting of the unchanged parts. Otherwise,
    // replace the old value with the new value.
    if let (serde_json::Value::Object(old_object), serde_json::Value::Object(new_object)) =
        (old_value, new_value)
    {
        for (key, old_sub_value) in old_object.iter() {
            key_path.push(key);
            let new_sub_value = new_object.get(key).unwrap_or(&serde_json::Value::Null);
            update_value_in_json_text(
                text,
                key_path,
                tab_size,
                old_sub_value,
                new_sub_value,
                preserved_keys,
                edits,
            );
            key_path.pop();
        }
        for (key, new_sub_value) in new_object.iter() {
            key_path.push(key);
            if !old_object.contains_key(key) {
                update_value_in_json_text(
                    text,
                    key_path,
                    tab_size,
                    &serde_json::Value::Null,
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
        let (range, replacement) = replace_value_in_json_text(text, key_path, tab_size, &new_value);
        text.replace_range(range.clone(), &replacement);
        edits.push((range, replacement));
    }
}

fn replace_value_in_json_text(
    text: &str,
    key_path: &[&str],
    tab_size: usize,
    new_value: &serde_json::Value,
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
    let matches = cursor.matches(&PAIR_QUERY, syntax_tree.root_node(), text.as_bytes());
    for mat in matches {
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

    // We found the exact key we want, insert the new value
    if depth == key_path.len() {
        let new_val = to_pretty_json(&new_value, tab_size, tab_size * depth);
        (existing_value_range, new_val)
    } else {
        // We have key paths, construct the sub objects
        let new_key = key_path[depth];

        // We don't have the key, construct the nested objects
        let mut new_value = serde_json::to_value(new_value).unwrap();
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

fn to_pretty_json(value: &impl Serialize, indent_size: usize, indent_prefix_len: usize) -> String {
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
