use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, StreamingIterator as _};
use util::RangeExt;

pub fn update_value_in_json_text<'a>(
    text: &mut String,
    key_path: &mut Vec<&'a str>,
    tab_size: usize,
    old_value: &'a Value,
    new_value: &'a Value,
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
                    edits,
                );
            } else {
                // Key was removed from new object, remove the entire key-value pair
                let (range, replacement) =
                    replace_value_in_json_text(text, key_path, 0, None, None);
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
                    edits,
                );
            }
            key_path.pop();
        }
    } else if old_value != new_value {
        let mut new_value = new_value.clone();
        if let Some(new_object) = new_value.as_object_mut() {
            new_object.retain(|_, v| !v.is_null());
        }
        let (range, replacement) =
            replace_value_in_json_text(text, key_path, tab_size, Some(&new_value), None);
        text.replace_range(range.clone(), &replacement);
        edits.push((range, replacement));
    }
}

/// * `replace_key` - When an exact key match according to `key_path` is found, replace the key with `replace_key` if `Some`.
pub fn replace_value_in_json_text<T: AsRef<str>>(
    text: &str,
    key_path: &[T],
    tab_size: usize,
    new_value: Option<&Value>,
    replace_key: Option<&str>,
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
            .zip(key_path.get(depth))
            .and_then(|(key_text, key_path_value)| {
                serde_json::to_string(key_path_value.as_ref())
                    .ok()
                    .map(|key_path| depth < key_path.len() && key_text == key_path)
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

            if let Some(array_replacement) = handle_possible_array_value(
                &mat.captures[0].node,
                &mat.captures[1].node,
                text,
                &key_path[depth..],
                new_value,
                replace_key,
                tab_size,
            ) {
                return array_replacement;
            }

            first_key_start = None;
        }
    }

    // We found the exact key we want
    if depth == key_path.len() {
        if let Some(new_value) = new_value {
            let new_val = to_pretty_json(new_value, tab_size, tab_size * depth);
            if let Some(replace_key) = replace_key.and_then(|str| serde_json::to_string(str).ok()) {
                let new_key = format!("{}: ", replace_key);
                if let Some(key_start) = text[..existing_value_range.start].rfind('"') {
                    if let Some(prev_key_start) = text[..key_start].rfind('"') {
                        existing_value_range.start = prev_key_start;
                    } else {
                        existing_value_range.start = key_start;
                    }
                }
                (existing_value_range, new_key + &new_val)
            } else {
                (existing_value_range, new_val)
            }
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

            let mut removed_comma = false;
            // Look backward for a preceding comma first
            let preceding_text = text.get(0..removal_start).unwrap_or("");
            if let Some(comma_pos) = preceding_text.rfind(',') {
                // Check if there are only whitespace characters between the comma and our key
                let between_comma_and_key = text.get(comma_pos + 1..removal_start).unwrap_or("");
                if between_comma_and_key.trim().is_empty() {
                    removal_start = comma_pos;
                    removed_comma = true;
                }
            }
            if let Some(remaining_text) = text.get(existing_value_range.end..)
                && !removed_comma
            {
                let mut chars = remaining_text.char_indices();
                while let Some((offset, ch)) = chars.next() {
                    if ch == ',' {
                        removal_end = existing_value_range.end + offset + 1;
                        // Also consume whitespace after the comma
                        for (_, next_ch) in chars.by_ref() {
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
        if let Some(first_key_start) = first_key_start {
            // We have key paths, construct the sub objects
            let new_key = key_path[depth].as_ref();
            // We don't have the key, construct the nested objects
            let new_value = construct_json_value(&key_path[(depth + 1)..], new_value);

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
            // We don't have the key, construct the nested objects
            let new_value = construct_json_value(&key_path[depth..], new_value);
            let indent_prefix_len = tab_size * depth;
            let mut new_val = to_pretty_json(&new_value, tab_size, indent_prefix_len);
            if depth == 0 {
                new_val.push('\n');
            }
            // best effort to keep comments with best effort indentation
            let mut replace_text = &text[existing_value_range.clone()];
            while let Some(comment_start) = replace_text.rfind("//") {
                if let Some(comment_end) = replace_text[comment_start..].find('\n') {
                    let mut comment_with_indent_start = replace_text[..comment_start]
                        .rfind('\n')
                        .unwrap_or(comment_start);
                    if !replace_text[comment_with_indent_start..comment_start]
                        .trim()
                        .is_empty()
                    {
                        comment_with_indent_start = comment_start;
                    }
                    new_val.insert_str(
                        1,
                        &replace_text[comment_with_indent_start..comment_start + comment_end],
                    );
                }
                replace_text = &replace_text[..comment_start];
            }

            (existing_value_range, new_val)
        }
    }
}

fn construct_json_value(
    key_path: &[impl AsRef<str>],
    new_value: Option<&serde_json::Value>,
) -> serde_json::Value {
    let mut new_value =
        serde_json::to_value(new_value.unwrap_or(&serde_json::Value::Null)).unwrap();
    for key in key_path.iter().rev() {
        if parse_index_key(key.as_ref()).is_some() {
            new_value = serde_json::json!([new_value]);
        } else {
            new_value = serde_json::json!({ key.as_ref().to_string(): new_value });
        }
    }
    return new_value;
}

fn parse_index_key(index_key: &str) -> Option<usize> {
    index_key.strip_prefix('#')?.parse().ok()
}

fn handle_possible_array_value(
    key_node: &tree_sitter::Node,
    value_node: &tree_sitter::Node,
    text: &str,
    remaining_key_path: &[impl AsRef<str>],
    new_value: Option<&Value>,
    replace_key: Option<&str>,
    tab_size: usize,
) -> Option<(Range<usize>, String)> {
    if remaining_key_path.is_empty() {
        return None;
    }
    let key_path = remaining_key_path;
    let index = parse_index_key(key_path[0].as_ref())?;

    let value_is_array = value_node.kind() == TS_ARRAY_KIND;

    let array_str = if value_is_array {
        &text[value_node.byte_range()]
    } else {
        ""
    };

    let (mut replace_range, mut replace_value) = replace_top_level_array_value_in_json_text(
        array_str,
        &key_path[1..],
        new_value,
        replace_key,
        index,
        tab_size,
    );

    if value_is_array {
        replace_range.start += value_node.start_byte();
        replace_range.end += value_node.start_byte();
    } else {
        // replace the full value if it wasn't an array
        replace_range = value_node.byte_range();
    }
    let non_whitespace_char_count = replace_value.len()
        - replace_value
            .chars()
            .filter(char::is_ascii_whitespace)
            .count();
    let needs_indent = replace_value.ends_with('\n')
        || (replace_value
            .chars()
            .zip(replace_value.chars().skip(1))
            .any(|(c, next_c)| c == '\n' && !next_c.is_ascii_whitespace()));
    let contains_comment = (replace_value.contains("//") && replace_value.contains('\n'))
        || (replace_value.contains("/*") && replace_value.contains("*/"));
    if needs_indent {
        let indent_width = key_node.start_position().column;
        let increased_indent = format!("\n{space:width$}", space = ' ', width = indent_width);
        replace_value = replace_value.replace('\n', &increased_indent);
    } else if non_whitespace_char_count < 32 && !contains_comment {
        // remove indentation
        while let Some(idx) = replace_value.find("\n ") {
            replace_value.remove(idx);
        }
        while let Some(idx) = replace_value.find("  ") {
            replace_value.remove(idx);
        }
    }
    return Some((replace_range, replace_value));
}

const TS_DOCUMENT_KIND: &str = "document";
const TS_ARRAY_KIND: &str = "array";
const TS_COMMENT_KIND: &str = "comment";

pub fn replace_top_level_array_value_in_json_text(
    text: &str,
    key_path: &[impl AsRef<str>],
    new_value: Option<&Value>,
    replace_key: Option<&str>,
    array_index: usize,
    tab_size: usize,
) -> (Range<usize>, String) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();

    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = syntax_tree.walk();

    if cursor.node().kind() == TS_DOCUMENT_KIND {
        cursor.goto_first_child();
    }

    while cursor.node().kind() != TS_ARRAY_KIND {
        if !cursor.goto_next_sibling() {
            let json_value = construct_json_value(key_path, new_value);
            let json_value = serde_json::json!([json_value]);
            return (0..text.len(), to_pretty_json(&json_value, tab_size, 0));
        }
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
                return (0..0, String::new());
            }
        }
    }

    let range = cursor.node().range();
    let indent_width = range.start_point.column;
    let offset = range.start_byte;
    let text_range = range.start_byte..range.end_byte;
    let value_str = &text[text_range.clone()];
    let needs_indent = range.start_point.row > 0;

    if new_value.is_none() && key_path.is_empty() {
        let mut remove_range = text_range;
        if index == 0 {
            while cursor.goto_next_sibling()
                && (cursor.node().is_extra() || cursor.node().is_missing())
            {}
            if cursor.node().kind() == "," {
                remove_range.end = cursor.node().range().end_byte;
            }
            if let Some(next_newline) = &text[remove_range.end + 1..].find('\n')
                && text[remove_range.end + 1..remove_range.end + next_newline]
                    .chars()
                    .all(|c| c.is_ascii_whitespace())
            {
                remove_range.end = remove_range.end + next_newline;
            }
        } else {
            while cursor.goto_previous_sibling()
                && (cursor.node().is_extra() || cursor.node().is_missing())
            {}
            if cursor.node().kind() == "," {
                remove_range.start = cursor.node().range().start_byte;
            }
        }
        (remove_range, String::new())
    } else {
        if let Some(array_replacement) = handle_possible_array_value(
            &cursor.node(),
            &cursor.node(),
            text,
            key_path,
            new_value,
            replace_key,
            tab_size,
        ) {
            return array_replacement;
        }
        let (mut replace_range, mut replace_value) =
            replace_value_in_json_text(value_str, key_path, tab_size, new_value, replace_key);

        replace_range.start += offset;
        replace_range.end += offset;

        if needs_indent {
            let increased_indent = format!("\n{space:width$}", space = ' ', width = indent_width);
            replace_value = replace_value.replace('\n', &increased_indent);
        } else {
            while let Some(idx) = replace_value.find("\n ") {
                replace_value.remove(idx + 1);
            }
            while let Some(idx) = replace_value.find("\n") {
                replace_value.replace_range(idx..idx + 1, " ");
            }
        }

        (replace_range, replace_value)
    }
}

pub fn append_top_level_array_value_in_json_text(
    text: &str,
    new_value: &Value,
    tab_size: usize,
) -> (Range<usize>, String) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();
    let syntax_tree = parser.parse(text, None).unwrap();

    let mut cursor = syntax_tree.walk();

    if cursor.node().kind() == TS_DOCUMENT_KIND {
        cursor.goto_first_child();
    }

    while cursor.node().kind() != TS_ARRAY_KIND {
        if !cursor.goto_next_sibling() {
            let json_value = serde_json::json!([new_value]);
            return (0..text.len(), to_pretty_json(&json_value, tab_size, 0));
        }
    }

    let went_to_last_child = cursor.goto_last_child();
    debug_assert!(
        went_to_last_child && cursor.node().kind() == "]",
        "Malformed JSON syntax tree, expected `]` at end of array"
    );
    let close_bracket_start = cursor.node().start_byte();
    while cursor.goto_previous_sibling()
        && (cursor.node().is_extra() || cursor.node().is_missing())
        && !cursor.node().is_error()
    {}

    let mut comma_range = None;
    let mut prev_item_range = None;

    if cursor.node().kind() == "," || is_error_of_kind(&mut cursor, ",") {
        comma_range = Some(cursor.node().byte_range());
        while cursor.goto_previous_sibling()
            && (cursor.node().is_extra() || cursor.node().is_missing())
        {}

        debug_assert_ne!(cursor.node().kind(), "[");
        prev_item_range = Some(cursor.node().range());
    } else {
        while (cursor.node().is_extra() || cursor.node().is_missing())
            && cursor.goto_previous_sibling()
        {}
        if cursor.node().kind() != "[" {
            prev_item_range = Some(cursor.node().range());
        }
    }

    let (mut replace_range, mut replace_value) =
        replace_value_in_json_text::<&str>("", &[], tab_size, Some(new_value), None);

    replace_range.start = close_bracket_start;
    replace_range.end = close_bracket_start;

    let space = ' ';
    if let Some(prev_item_range) = prev_item_range {
        let needs_newline = prev_item_range.start_point.row > 0;
        let indent_width = text[..prev_item_range.start_byte].rfind('\n').map_or(
            prev_item_range.start_point.column,
            |idx| {
                prev_item_range.start_point.column
                    - text[idx + 1..prev_item_range.start_byte].trim_start().len()
            },
        );

        let prev_item_end = comma_range
            .as_ref()
            .map_or(prev_item_range.end_byte, |range| range.end);
        if text[prev_item_end..replace_range.start].trim().is_empty() {
            replace_range.start = prev_item_end;
        }

        if needs_newline {
            let increased_indent = format!("\n{space:width$}", width = indent_width);
            replace_value = replace_value.replace('\n', &increased_indent);
            replace_value.push('\n');
            replace_value.insert_str(0, &format!("\n{space:width$}", width = indent_width));
        } else {
            while let Some(idx) = replace_value.find("\n ") {
                replace_value.remove(idx + 1);
            }
            while let Some(idx) = replace_value.find('\n') {
                replace_value.replace_range(idx..idx + 1, " ");
            }
            replace_value.insert(0, ' ');
        }

        if comma_range.is_none() {
            replace_value.insert(0, ',');
        }
    } else if replace_value.contains('\n') || text.contains('\n') {
        if let Some(prev_newline) = text[..replace_range.start].rfind('\n')
            && text[prev_newline..replace_range.start].trim().is_empty()
        {
            replace_range.start = prev_newline;
        }
        let indent = format!("\n{space:width$}", width = tab_size);
        replace_value = replace_value.replace('\n', &indent);
        replace_value.insert_str(0, &indent);
        replace_value.push('\n');
    }
    return (replace_range, replace_value);

    fn is_error_of_kind(cursor: &mut tree_sitter::TreeCursor<'_>, kind: &str) -> bool {
        if cursor.node().kind() != "ERROR" {
            return false;
        }

        let descendant_index = cursor.descendant_index();
        let res = cursor.goto_first_child() && cursor.node().kind() == kind;
        cursor.goto_descendant(descendant_index);
        res
    }
}

/// Infers the indentation size used in JSON text by analyzing the tree structure.
/// Returns the detected indent size, or a default of 2 if no indentation is found.
pub fn infer_json_indent_size(text: &str) -> usize {
    const MAX_INDENT_SIZE: usize = 64;

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .unwrap();

    let Some(syntax_tree) = parser.parse(text, None) else {
        return 4;
    };

    let mut cursor = syntax_tree.walk();
    let mut indent_counts = [0u32; MAX_INDENT_SIZE];

    // Traverse the tree to find indentation patterns
    fn visit_node(
        cursor: &mut tree_sitter::TreeCursor,
        indent_counts: &mut [u32; MAX_INDENT_SIZE],
        depth: usize,
    ) {
        if depth >= 3 {
            return;
        }
        let node = cursor.node();
        let node_kind = node.kind();

        // For objects and arrays, check the indentation of their first content child
        if matches!(node_kind, "object" | "array") {
            let container_column = node.start_position().column;
            let container_row = node.start_position().row;

            if cursor.goto_first_child() {
                // Skip the opening bracket
                loop {
                    let child = cursor.node();
                    let child_kind = child.kind();

                    // Look for the first actual content (pair for objects, value for arrays)
                    if (node_kind == "object" && child_kind == "pair")
                        || (node_kind == "array"
                            && !matches!(child_kind, "[" | "]" | "," | "comment"))
                    {
                        let child_column = child.start_position().column;
                        let child_row = child.start_position().row;

                        // Only count if the child is on a different line
                        if child_row > container_row && child_column > container_column {
                            let indent = child_column - container_column;
                            if indent > 0 && indent < MAX_INDENT_SIZE {
                                indent_counts[indent] += 1;
                            }
                        }
                        break;
                    }

                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }

        // Recurse to children
        if cursor.goto_first_child() {
            loop {
                visit_node(cursor, indent_counts, depth + 1);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    visit_node(&mut cursor, &mut indent_counts, 0);

    // Find the indent size with the highest count
    let mut max_count = 0;
    let mut max_indent = 4;

    for (indent, &count) in indent_counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_indent = indent;
        }
    }

    if max_count == 0 { 2 } else { max_indent }
}

pub fn to_pretty_json(
    value: &impl Serialize,
    indent_size: usize,
    indent_prefix_len: usize,
) -> String {
    let mut output = Vec::new();
    let indent = " ".repeat(indent_size);
    let mut ser = serde_json::Serializer::with_formatter(
        &mut output,
        serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes()),
    );

    value.serialize(&mut ser).unwrap();
    let text = String::from_utf8(output).unwrap();

    let mut adjusted_text = String::new();
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            adjusted_text.extend(std::iter::repeat(' ').take(indent_prefix_len));
        }
        adjusted_text.push_str(line);
        adjusted_text.push('\n');
    }
    adjusted_text.pop();
    adjusted_text
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    let mut deserializer = serde_json_lenient::Deserializer::from_str(content);
    Ok(serde_path_to_error::deserialize(&mut deserializer)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use unindent::Unindent;

    #[test]
    fn object_replace() {
        #[track_caller]
        fn check_object_replace(
            input: String,
            key_path: &[&str],
            value: Option<Value>,
            expected: String,
        ) {
            let result = replace_value_in_json_text(&input, key_path, 4, value.as_ref(), None);
            let mut result_str = input;
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
        // Test with comments between object members
        check_object_replace(
            r#"{
                "a": 1,
                // Comment between members
                "b": 2,
                /* Block comment */
                "c": 3
            }"#
            .unindent(),
            &["b"],
            Some(json!({"nested": true})),
            r#"{
                "a": 1,
                // Comment between members
                "b": {
                    "nested": true
                },
                /* Block comment */
                "c": 3
            }"#
            .unindent(),
        );

        // Test with trailing comments on replaced value
        check_object_replace(
            r#"{
                "a": 1, // keep this comment
                "b": 2  // this should stay
            }"#
            .unindent(),
            &["a"],
            Some(json!("changed")),
            r#"{
                "a": "changed", // keep this comment
                "b": 2  // this should stay
            }"#
            .unindent(),
        );

        // Test with deep indentation
        check_object_replace(
            r#"{
                        "deeply": {
                                "nested": {
                                        "value": "old"
                                }
                        }
                }"#
            .unindent(),
            &["deeply", "nested", "value"],
            Some(json!("new")),
            r#"{
                        "deeply": {
                                "nested": {
                                        "value": "new"
                                }
                        }
                }"#
            .unindent(),
        );

        // Test removing value with comment preservation
        check_object_replace(
            r#"{
                // Header comment
                "a": 1,
                // This comment belongs to b
                "b": 2,
                // This comment belongs to c
                "c": 3
            }"#
            .unindent(),
            &["b"],
            None,
            r#"{
                // Header comment
                "a": 1,
                // This comment belongs to b
                // This comment belongs to c
                "c": 3
            }"#
            .unindent(),
        );

        // Test with multiline block comments
        check_object_replace(
            r#"{
                /*
                 * This is a multiline
                 * block comment
                 */
                "value": "old",
                /* Another block */ "other": 123
            }"#
            .unindent(),
            &["value"],
            Some(json!("new")),
            r#"{
                /*
                 * This is a multiline
                 * block comment
                 */
                "value": "new",
                /* Another block */ "other": 123
            }"#
            .unindent(),
        );

        check_object_replace(
            r#"{
                // This object is empty
            }"#
            .unindent(),
            &["key"],
            Some(json!("value")),
            r#"{
                // This object is empty
                "key": "value"
            }
            "#
            .unindent(),
        );

        // Test replacing in object with only comments
        check_object_replace(
            r#"{
                // Comment 1
                // Comment 2
            }"#
            .unindent(),
            &["new"],
            Some(json!(42)),
            r#"{
                // Comment 1
                // Comment 2
                "new": 42
            }
            "#
            .unindent(),
        );

        // Test with inconsistent spacing
        check_object_replace(
            r#"{
              "a":1,
                    "b"  :  2  ,
                "c":   3
            }"#
            .unindent(),
            &["b"],
            Some(json!("spaced")),
            r#"{
              "a":1,
                    "b"  :  "spaced"  ,
                "c":   3
            }"#
            .unindent(),
        );
    }

    #[test]
    fn object_replace_array() {
        // Tests replacing values within arrays that are nested inside objects.
        // Uses "#N" syntax in key paths to indicate array indices.
        #[track_caller]
        fn check_object_replace_array(
            input: String,
            key_path: &[&str],
            value: Option<Value>,
            expected: String,
        ) {
            let result = replace_value_in_json_text(&input, key_path, 4, value.as_ref(), None);
            let mut result_str = input;
            result_str.replace_range(result.0, &result.1);
            pretty_assertions::assert_eq!(expected, result_str);
        }

        // Basic array element replacement
        check_object_replace_array(
            r#"{
                "a": [1, 3],
            }"#
            .unindent(),
            &["a", "#1"],
            Some(json!(2)),
            r#"{
                "a": [1, 2],
            }"#
            .unindent(),
        );

        // Replace first element
        check_object_replace_array(
            r#"{
                "items": [1, 2, 3]
            }"#
            .unindent(),
            &["items", "#0"],
            Some(json!(10)),
            r#"{
                "items": [10, 2, 3]
            }"#
            .unindent(),
        );

        // Replace last element
        check_object_replace_array(
            r#"{
                "items": [1, 2, 3]
            }"#
            .unindent(),
            &["items", "#2"],
            Some(json!(30)),
            r#"{
                "items": [1, 2, 30]
            }"#
            .unindent(),
        );

        // Replace string in array
        check_object_replace_array(
            r#"{
                "names": ["alice", "bob", "charlie"]
            }"#
            .unindent(),
            &["names", "#1"],
            Some(json!("robert")),
            r#"{
                "names": ["alice", "robert", "charlie"]
            }"#
            .unindent(),
        );

        // Replace boolean
        check_object_replace_array(
            r#"{
                "flags": [true, false, true]
            }"#
            .unindent(),
            &["flags", "#0"],
            Some(json!(false)),
            r#"{
                "flags": [false, false, true]
            }"#
            .unindent(),
        );

        // Replace null with value
        check_object_replace_array(
            r#"{
                "values": [null, 2, null]
            }"#
            .unindent(),
            &["values", "#0"],
            Some(json!(1)),
            r#"{
                "values": [1, 2, null]
            }"#
            .unindent(),
        );

        // Replace value with null
        check_object_replace_array(
            r#"{
                "data": [1, 2, 3]
            }"#
            .unindent(),
            &["data", "#1"],
            Some(json!(null)),
            r#"{
                "data": [1, null, 3]
            }"#
            .unindent(),
        );

        // Replace simple value with object
        check_object_replace_array(
            r#"{
                "list": [1, 2, 3]
            }"#
            .unindent(),
            &["list", "#1"],
            Some(json!({"value": 2, "label": "two"})),
            r#"{
                "list": [1, { "value": 2, "label": "two" }, 3]
            }"#
            .unindent(),
        );

        // Replace simple value with nested array
        check_object_replace_array(
            r#"{
                "matrix": [1, 2, 3]
            }"#
            .unindent(),
            &["matrix", "#1"],
            Some(json!([20, 21, 22])),
            r#"{
                "matrix": [1, [ 20, 21, 22 ], 3]
            }"#
            .unindent(),
        );

        // Replace object in array
        check_object_replace_array(
            r#"{
                "users": [
                    {"name": "alice"},
                    {"name": "bob"},
                    {"name": "charlie"}
                ]
            }"#
            .unindent(),
            &["users", "#1"],
            Some(json!({"name": "robert", "age": 30})),
            r#"{
                "users": [
                    {"name": "alice"},
                    { "name": "robert", "age": 30 },
                    {"name": "charlie"}
                ]
            }"#
            .unindent(),
        );

        // Replace property within object in array
        check_object_replace_array(
            r#"{
                "users": [
                    {"name": "alice", "age": 25},
                    {"name": "bob", "age": 30},
                    {"name": "charlie", "age": 35}
                ]
            }"#
            .unindent(),
            &["users", "#1", "age"],
            Some(json!(31)),
            r#"{
                "users": [
                    {"name": "alice", "age": 25},
                    {"name": "bob", "age": 31},
                    {"name": "charlie", "age": 35}
                ]
            }"#
            .unindent(),
        );

        // Add new property to object in array
        check_object_replace_array(
            r#"{
                "items": [
                    {"id": 1},
                    {"id": 2},
                    {"id": 3}
                ]
            }"#
            .unindent(),
            &["items", "#1", "name"],
            Some(json!("Item Two")),
            r#"{
                "items": [
                    {"id": 1},
                    {"name": "Item Two", "id": 2},
                    {"id": 3}
                ]
            }"#
            .unindent(),
        );

        // Remove property from object in array
        check_object_replace_array(
            r#"{
                "items": [
                    {"id": 1, "name": "one"},
                    {"id": 2, "name": "two"},
                    {"id": 3, "name": "three"}
                ]
            }"#
            .unindent(),
            &["items", "#1", "name"],
            None,
            r#"{
                "items": [
                    {"id": 1, "name": "one"},
                    {"id": 2},
                    {"id": 3, "name": "three"}
                ]
            }"#
            .unindent(),
        );

        // Deeply nested: array in object in array
        check_object_replace_array(
            r#"{
                "data": [
                    {
                        "values": [1, 2, 3]
                    },
                    {
                        "values": [4, 5, 6]
                    }
                ]
            }"#
            .unindent(),
            &["data", "#0", "values", "#1"],
            Some(json!(20)),
            r#"{
                "data": [
                    {
                        "values": [1, 20, 3]
                    },
                    {
                        "values": [4, 5, 6]
                    }
                ]
            }"#
            .unindent(),
        );

        // Multiple levels of nesting
        check_object_replace_array(
            r#"{
                "root": {
                    "level1": [
                        {
                            "level2": {
                                "level3": [10, 20, 30]
                            }
                        }
                    ]
                }
            }"#
            .unindent(),
            &["root", "level1", "#0", "level2", "level3", "#2"],
            Some(json!(300)),
            r#"{
                "root": {
                    "level1": [
                        {
                            "level2": {
                                "level3": [10, 20, 300]
                            }
                        }
                    ]
                }
            }"#
            .unindent(),
        );

        // Array with mixed types
        check_object_replace_array(
            r#"{
                "mixed": [1, "two", true, null, {"five": 5}]
            }"#
            .unindent(),
            &["mixed", "#3"],
            Some(json!({"four": 4})),
            r#"{
                "mixed": [1, "two", true, { "four": 4 }, {"five": 5}]
            }"#
            .unindent(),
        );

        // Replace with complex object
        check_object_replace_array(
            r#"{
                "config": [
                    "simple",
                    "values"
                ]
            }"#
            .unindent(),
            &["config", "#0"],
            Some(json!({
                "type": "complex",
                "settings": {
                    "enabled": true,
                    "level": 5
                }
            })),
            r#"{
                "config": [
                    {
                        "type": "complex",
                        "settings": {
                            "enabled": true,
                            "level": 5
                        }
                    },
                    "values"
                ]
            }"#
            .unindent(),
        );

        // Array with trailing comma
        check_object_replace_array(
            r#"{
                "items": [
                    1,
                    2,
                    3,
                ]
            }"#
            .unindent(),
            &["items", "#1"],
            Some(json!(20)),
            r#"{
                "items": [
                    1,
                    20,
                    3,
                ]
            }"#
            .unindent(),
        );

        // Array with comments
        check_object_replace_array(
            r#"{
                "items": [
                    1, // first item
                    2, // second item
                    3  // third item
                ]
            }"#
            .unindent(),
            &["items", "#1"],
            Some(json!(20)),
            r#"{
                "items": [
                    1, // first item
                    20, // second item
                    3  // third item
                ]
            }"#
            .unindent(),
        );

        // Multiple arrays in object
        check_object_replace_array(
            r#"{
                "first": [1, 2, 3],
                "second": [4, 5, 6],
                "third": [7, 8, 9]
            }"#
            .unindent(),
            &["second", "#1"],
            Some(json!(50)),
            r#"{
                "first": [1, 2, 3],
                "second": [4, 50, 6],
                "third": [7, 8, 9]
            }"#
            .unindent(),
        );

        // Empty array - add first element
        check_object_replace_array(
            r#"{
                "empty": []
            }"#
            .unindent(),
            &["empty", "#0"],
            Some(json!("first")),
            r#"{
                "empty": ["first"]
            }"#
            .unindent(),
        );

        // Array of arrays
        check_object_replace_array(
            r#"{
                "matrix": [
                    [1, 2],
                    [3, 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
            &["matrix", "#1", "#0"],
            Some(json!(30)),
            r#"{
                "matrix": [
                    [1, 2],
                    [30, 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
        );

        // Replace nested object property in array element
        check_object_replace_array(
            r#"{
                "users": [
                    {
                        "name": "alice",
                        "address": {
                            "city": "NYC",
                            "zip": "10001"
                        }
                    }
                ]
            }"#
            .unindent(),
            &["users", "#0", "address", "city"],
            Some(json!("Boston")),
            r#"{
                "users": [
                    {
                        "name": "alice",
                        "address": {
                            "city": "Boston",
                            "zip": "10001"
                        }
                    }
                ]
            }"#
            .unindent(),
        );

        // Add element past end of array
        check_object_replace_array(
            r#"{
                "items": [1, 2]
            }"#
            .unindent(),
            &["items", "#5"],
            Some(json!(6)),
            r#"{
                "items": [1, 2, 6]
            }"#
            .unindent(),
        );

        // Complex nested structure
        check_object_replace_array(
            r#"{
                "app": {
                    "modules": [
                        {
                            "name": "auth",
                            "routes": [
                                {"path": "/login", "method": "POST"},
                                {"path": "/logout", "method": "POST"}
                            ]
                        },
                        {
                            "name": "api",
                            "routes": [
                                {"path": "/users", "method": "GET"},
                                {"path": "/users", "method": "POST"}
                            ]
                        }
                    ]
                }
            }"#
            .unindent(),
            &["app", "modules", "#1", "routes", "#0", "method"],
            Some(json!("PUT")),
            r#"{
                "app": {
                    "modules": [
                        {
                            "name": "auth",
                            "routes": [
                                {"path": "/login", "method": "POST"},
                                {"path": "/logout", "method": "POST"}
                            ]
                        },
                        {
                            "name": "api",
                            "routes": [
                                {"path": "/users", "method": "PUT"},
                                {"path": "/users", "method": "POST"}
                            ]
                        }
                    ]
                }
            }"#
            .unindent(),
        );

        // Escaped strings in array
        check_object_replace_array(
            r#"{
                "messages": ["hello", "world"]
            }"#
            .unindent(),
            &["messages", "#0"],
            Some(json!("hello \"quoted\" world")),
            r#"{
                "messages": ["hello \"quoted\" world", "world"]
            }"#
            .unindent(),
        );

        // Block comments
        check_object_replace_array(
            r#"{
                "data": [
                    /* first */ 1,
                    /* second */ 2,
                    /* third */ 3
                ]
            }"#
            .unindent(),
            &["data", "#1"],
            Some(json!(20)),
            r#"{
                "data": [
                    /* first */ 1,
                    /* second */ 20,
                    /* third */ 3
                ]
            }"#
            .unindent(),
        );

        // Inline array
        check_object_replace_array(
            r#"{"items": [1, 2, 3], "count": 3}"#.to_string(),
            &["items", "#1"],
            Some(json!(20)),
            r#"{"items": [1, 20, 3], "count": 3}"#.to_string(),
        );

        // Single element array
        check_object_replace_array(
            r#"{
                "single": [42]
            }"#
            .unindent(),
            &["single", "#0"],
            Some(json!(100)),
            r#"{
                "single": [100]
            }"#
            .unindent(),
        );

        // Inconsistent formatting
        check_object_replace_array(
            r#"{
                "messy": [1,
                    2,
                        3,
                4]
            }"#
            .unindent(),
            &["messy", "#2"],
            Some(json!(30)),
            r#"{
                "messy": [1,
                    2,
                        30,
                4]
            }"#
            .unindent(),
        );

        // Creates array if has numbered key
        check_object_replace_array(
            r#"{
                "array": {"foo": "bar"}
            }"#
            .unindent(),
            &["array", "#3"],
            Some(json!(4)),
            r#"{
                "array": [
                    4
                ]
            }"#
            .unindent(),
        );

        // Replace non-array element within array with array
        check_object_replace_array(
            r#"{
                "matrix": [
                    [1, 2],
                    [3, 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
            &["matrix", "#1", "#0"],
            Some(json!(["foo", "bar"])),
            r#"{
                "matrix": [
                    [1, 2],
                    [[ "foo", "bar" ], 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
        );
        // Replace non-array element within array with array
        check_object_replace_array(
            r#"{
                "matrix": [
                    [1, 2],
                    [3, 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
            &["matrix", "#1", "#0", "#3"],
            Some(json!(["foo", "bar"])),
            r#"{
                "matrix": [
                    [1, 2],
                    [[ [ "foo", "bar" ] ], 4],
                    [5, 6]
                ]
            }"#
            .unindent(),
        );

        // Create array in key that doesn't exist
        check_object_replace_array(
            r#"{
                "foo": {}
            }"#
            .unindent(),
            &["foo", "bar", "#0"],
            Some(json!({"is_object": true})),
            r#"{
                "foo": {
                    "bar": [
                        {
                            "is_object": true
                        }
                    ]
                }
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
            value: Option<Value>,
            expected: impl ToString,
        ) {
            let input = input.to_string();
            let result = replace_top_level_array_value_in_json_text(
                &input,
                key_path,
                value.as_ref(),
                None,
                index,
                4,
            );
            let mut result_str = input;
            result_str.replace_range(result.0, &result.1);
            pretty_assertions::assert_eq!(expected.to_string(), result_str);
        }

        check_array_replace(r#"[1, 3, 3]"#, 1, &[], Some(json!(2)), r#"[1, 2, 3]"#);
        check_array_replace(r#"[1, 3, 3]"#, 2, &[], Some(json!(2)), r#"[1, 3, 2]"#);
        check_array_replace(r#"[1, 3, 3,]"#, 3, &[], Some(json!(2)), r#"[1, 3, 3, 2]"#);
        check_array_replace(r#"[1, 3, 3,]"#, 100, &[], Some(json!(2)), r#"[1, 3, 3, 2]"#);
        check_array_replace(
            r#"[
                1,
                2,
                3,
            ]"#
            .unindent(),
            1,
            &[],
            Some(json!({"foo": "bar", "baz": "qux"})),
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
            Some(json!({"foo": "bar", "baz": "qux"})),
            r#"[1, { "foo": "bar", "baz": "qux" }, 3,]"#,
        );

        check_array_replace(
            r#"[1, { "foo": "bar", "baz": "qux" }, 3,]"#,
            1,
            &["baz"],
            Some(json!({"qux": "quz"})),
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
            Some(json!({"qux": "quz"})),
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
            Some(json!("qux")),
            r#"[
                1,
                {
                    "foo": "bar",
                    "baz": "qux"
                },
                3
            ]"#,
        );

        check_array_replace(
            r#"[
                1,
                {
                    "foo": "bar",
                    // some comment to keep
                    "baz": {
                        // some comment to remove
                        "qux": "quz"
                    }
                    // some other comment to keep
                },
                3
            ]"#,
            1,
            &["baz"],
            Some(json!("qux")),
            r#"[
                1,
                {
                    "foo": "bar",
                    // some comment to keep
                    "baz": "qux"
                    // some other comment to keep
                },
                3
            ]"#,
        );

        // Test with comments between array elements
        check_array_replace(
            r#"[
                1,
                // This is element 2
                2,
                /* Block comment */ 3,
                4 // Trailing comment
            ]"#,
            2,
            &[],
            Some(json!("replaced")),
            r#"[
                1,
                // This is element 2
                2,
                /* Block comment */ "replaced",
                4 // Trailing comment
            ]"#,
        );

        // Test empty array with comments
        check_array_replace(
            r#"[
                // Empty array with comment
            ]"#
            .unindent(),
            0,
            &[],
            Some(json!("first")),
            r#"[
                // Empty array with comment
                "first"
            ]"#
            .unindent(),
        );
        check_array_replace(
            r#"[]"#.unindent(),
            0,
            &[],
            Some(json!("first")),
            r#"["first"]"#.unindent(),
        );

        // Test array with leading comments
        check_array_replace(
            r#"[
                // Leading comment
                // Another leading comment
                1,
                2
            ]"#,
            0,
            &[],
            Some(json!({"new": "object"})),
            r#"[
                // Leading comment
                // Another leading comment
                {
                    "new": "object"
                },
                2
            ]"#,
        );

        // Test with deep indentation
        check_array_replace(
            r#"[
                        1,
                        2,
                        3
                    ]"#,
            1,
            &[],
            Some(json!("deep")),
            r#"[
                        1,
                        "deep",
                        3
                    ]"#,
        );

        // Test with mixed spacing
        check_array_replace(
            r#"[1,2,   3,    4]"#,
            2,
            &[],
            Some(json!("spaced")),
            r#"[1,2,   "spaced",    4]"#,
        );

        // Test replacing nested array element
        check_array_replace(
            r#"[
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9]
            ]"#,
            1,
            &[],
            Some(json!(["a", "b", "c", "d"])),
            r#"[
                [1, 2, 3],
                [
                    "a",
                    "b",
                    "c",
                    "d"
                ],
                [7, 8, 9]
            ]"#,
        );

        // Test with multiline block comments
        check_array_replace(
            r#"[
                /*
                 * This is a
                 * multiline comment
                 */
                "first",
                "second"
            ]"#,
            0,
            &[],
            Some(json!("updated")),
            r#"[
                /*
                 * This is a
                 * multiline comment
                 */
                "updated",
                "second"
            ]"#,
        );

        // Test replacing with null
        check_array_replace(
            r#"[true, false, true]"#,
            1,
            &[],
            Some(json!(null)),
            r#"[true, null, true]"#,
        );

        // Test single element array
        check_array_replace(
            r#"[42]"#,
            0,
            &[],
            Some(json!({"answer": 42})),
            r#"[{ "answer": 42 }]"#,
        );

        // Test array with only comments
        check_array_replace(
            r#"[
                // Comment 1
                // Comment 2
                // Comment 3
            ]"#
            .unindent(),
            10,
            &[],
            Some(json!(123)),
            r#"[
                // Comment 1
                // Comment 2
                // Comment 3
                123
            ]"#
            .unindent(),
        );

        check_array_replace(
            r#"[
                {
                    "key": "value"
                },
                {
                    "key": "value2"
                }
            ]"#
            .unindent(),
            0,
            &[],
            None,
            r#"[
                {
                    "key": "value2"
                }
            ]"#
            .unindent(),
        );

        check_array_replace(
            r#"[
                {
                    "key": "value"
                },
                {
                    "key": "value2"
                },
                {
                    "key": "value3"
                },
            ]"#
            .unindent(),
            1,
            &[],
            None,
            r#"[
                {
                    "key": "value"
                },
                {
                    "key": "value3"
                },
            ]"#
            .unindent(),
        );

        check_array_replace(
            r#""#,
            2,
            &[],
            Some(json!(42)),
            r#"[
                42
            ]"#
            .unindent(),
        );

        check_array_replace(
            r#""#,
            2,
            &["foo", "bar"],
            Some(json!(42)),
            r#"[
                {
                    "foo": {
                        "bar": 42
                    }
                }
            ]"#
            .unindent(),
        );
    }

    #[test]
    fn array_append() {
        #[track_caller]
        fn check_array_append(input: impl ToString, value: Value, expected: impl ToString) {
            let input = input.to_string();
            let result = append_top_level_array_value_in_json_text(&input, &value, 4);
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

        // Test with comments between array elements
        check_array_append(
            r#"[
                1,
                // Comment between elements
                2,
                /* Block comment */ 3
            ]"#
            .unindent(),
            json!(4),
            r#"[
                1,
                // Comment between elements
                2,
                /* Block comment */ 3,
                4
            ]"#
            .unindent(),
        );

        // Test with trailing comment on last element
        check_array_append(
            r#"[
                1,
                2,
                3 // Trailing comment
            ]"#
            .unindent(),
            json!("new"),
            r#"[
                1,
                2,
                3 // Trailing comment
            ,
                "new"
            ]"#
            .unindent(),
        );

        // Test empty array with comments
        check_array_append(
            r#"[
                // Empty array with comment
            ]"#
            .unindent(),
            json!("first"),
            r#"[
                // Empty array with comment
                "first"
            ]"#
            .unindent(),
        );

        // Test with multiline block comment at end
        check_array_append(
            r#"[
                1,
                2
                /*
                 * This is a
                 * multiline comment
                 */
            ]"#
            .unindent(),
            json!(3),
            r#"[
                1,
                2
                /*
                 * This is a
                 * multiline comment
                 */
            ,
                3
            ]"#
            .unindent(),
        );

        // Test with deep indentation
        check_array_append(
            r#"[
                1,
                    2,
                        3
            ]"#
            .unindent(),
            json!("deep"),
            r#"[
                1,
                    2,
                        3,
                        "deep"
            ]"#
            .unindent(),
        );

        // Test with no spacing
        check_array_append(r#"[1,2,3]"#, json!(4), r#"[1,2,3, 4]"#);

        // Test appending complex nested structure
        check_array_append(
            r#"[
                {"a": 1},
                {"b": 2}
            ]"#
            .unindent(),
            json!({"c": {"nested": [1, 2, 3]}}),
            r#"[
                {"a": 1},
                {"b": 2},
                {
                    "c": {
                        "nested": [
                            1,
                            2,
                            3
                        ]
                    }
                }
            ]"#
            .unindent(),
        );

        // Test array ending with comment after bracket
        check_array_append(
            r#"[
                1,
                2,
                3
            ] // Comment after array"#
                .unindent(),
            json!(4),
            r#"[
                1,
                2,
                3,
                4
            ] // Comment after array"#
                .unindent(),
        );

        // Test with inconsistent element formatting
        check_array_append(
            r#"[1,
               2,
                    3,
            ]"#
            .unindent(),
            json!(4),
            r#"[1,
               2,
                    3,
                    4
            ]"#
            .unindent(),
        );

        // Test appending to single-line array with trailing comma
        check_array_append(
            r#"[1, 2, 3,]"#,
            json!({"key": "value"}),
            r#"[1, 2, 3, { "key": "value" }]"#,
        );

        // Test appending null value
        check_array_append(r#"[true, false]"#, json!(null), r#"[true, false, null]"#);

        // Test appending to array with only comments
        check_array_append(
            r#"[
                // Just comments here
                // More comments
            ]"#
            .unindent(),
            json!(42),
            r#"[
                // Just comments here
                // More comments
                42
            ]"#
            .unindent(),
        );

        check_array_append(
            r#""#,
            json!(42),
            r#"[
                42
            ]"#
            .unindent(),
        )
    }

    #[test]
    fn test_infer_json_indent_size() {
        let json_2_spaces = r#"{
  "key1": "value1",
  "nested": {
    "key2": "value2",
    "array": [
      1,
      2,
      3
    ]
  }
}"#;
        assert_eq!(infer_json_indent_size(json_2_spaces), 2);

        let json_4_spaces = r#"{
    "key1": "value1",
    "nested": {
        "key2": "value2",
        "array": [
            1,
            2,
            3
        ]
    }
}"#;
        assert_eq!(infer_json_indent_size(json_4_spaces), 4);

        let json_8_spaces = r#"{
        "key1": "value1",
        "nested": {
                "key2": "value2"
        }
}"#;
        assert_eq!(infer_json_indent_size(json_8_spaces), 8);

        let json_single_line = r#"{"key": "value", "nested": {"inner": "data"}}"#;
        assert_eq!(infer_json_indent_size(json_single_line), 2);

        let json_empty = r#"{}"#;
        assert_eq!(infer_json_indent_size(json_empty), 2);

        let json_array = r#"[
  {
    "id": 1,
    "name": "first"
  },
  {
    "id": 2,
    "name": "second"
  }
]"#;
        assert_eq!(infer_json_indent_size(json_array), 2);

        let json_mixed = r#"{
  "a": {
    "b": {
        "c": "value"
    }
  },
  "d": "value2"
}"#;
        assert_eq!(infer_json_indent_size(json_mixed), 2);
    }
}
