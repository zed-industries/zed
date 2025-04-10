use collections::HashMap;
use convert_case::{Case, Casing};
use std::{ops::Range, sync::LazyLock};
use tree_sitter::{Query, QueryMatch};

use crate::MigrationPatterns;
use crate::patterns::KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN;

pub const KEYMAP_PATTERNS: MigrationPatterns = &[(
    KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN,
    action_argument_snake_case,
)];

fn to_snake_case(text: &str) -> String {
    text.to_case(Case::Snake)
}

fn action_argument_snake_case(
    contents: &str,
    mat: &QueryMatch,
    query: &Query,
) -> Option<(Range<usize>, String)> {
    let array_ix = query.capture_index_for_name("array")?;
    let action_name_ix = query.capture_index_for_name("action_name")?;
    let argument_key_ix = query.capture_index_for_name("argument_key")?;
    let argument_value_ix = query.capture_index_for_name("argument_value")?;
    let action_name = contents.get(
        mat.nodes_for_capture_index(action_name_ix)
            .next()?
            .byte_range(),
    )?;

    let replacement_key = ACTION_ARGUMENT_SNAKE_CASE_REPLACE.get(action_name)?;
    let argument_key = contents.get(
        mat.nodes_for_capture_index(argument_key_ix)
            .next()?
            .byte_range(),
    )?;

    if argument_key != *replacement_key {
        return None;
    }

    let argument_value_node = mat.nodes_for_capture_index(argument_value_ix).next()?;
    let argument_value = contents.get(argument_value_node.byte_range())?;

    let new_key = to_snake_case(argument_key);
    let new_value = if argument_value_node.kind() == "string" {
        format!("\"{}\"", to_snake_case(argument_value.trim_matches('"')))
    } else {
        argument_value.to_string()
    };

    let range_to_replace = mat.nodes_for_capture_index(array_ix).next()?.byte_range();
    let replacement = format!(
        "[\"{}\", {{ \"{}\": {} }}]",
        action_name, new_key, new_value
    );

    Some((range_to_replace, replacement))
}

static ACTION_ARGUMENT_SNAKE_CASE_REPLACE: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from_iter([
        ("vim::NextWordStart", "ignorePunctuation"),
        ("vim::NextWordEnd", "ignorePunctuation"),
        ("vim::PreviousWordStart", "ignorePunctuation"),
        ("vim::PreviousWordEnd", "ignorePunctuation"),
        ("vim::MoveToNext", "partialWord"),
        ("vim::MoveToPrev", "partialWord"),
        ("vim::Down", "displayLines"),
        ("vim::Up", "displayLines"),
        ("vim::EndOfLine", "displayLines"),
        ("vim::StartOfLine", "displayLines"),
        ("vim::FirstNonWhitespace", "displayLines"),
        ("pane::CloseActiveItem", "saveIntent"),
        ("vim::Paste", "preserveClipboard"),
        ("vim::Word", "ignorePunctuation"),
        ("vim::Subword", "ignorePunctuation"),
        ("vim::IndentObj", "includeBelow"),
    ])
});
