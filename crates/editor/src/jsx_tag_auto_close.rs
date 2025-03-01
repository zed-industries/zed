use anyhow::Result;
use gpui::{Context, Entity};
use multi_buffer::MultiBuffer;
use std::ops::Range;

use language::{BufferSnapshot, JsxTagAutoCloseConfig};
use text::Anchor;

use crate::Editor;

pub struct JsxTagCompletionState {
    edit_index: usize,
    open_tag_range: Range<usize>,
}

pub(crate) fn should_auto_close(
    buffer: &BufferSnapshot,
    edited_ranges: &[Range<usize>],
    config: &JsxTagAutoCloseConfig,
) -> Option<Vec<JsxTagCompletionState>> {
    let mut to_auto_edit = vec![];
    for (index, edited_range) in edited_ranges.iter().enumerate() {
        let text = buffer
            .text_for_range(edited_range.clone())
            .collect::<String>();
        if !text.ends_with(">") {
            continue;
        }
        let Some(layer) = buffer.syntax_layer_at(edited_range.start) else {
            continue;
        };
        let Some(node) = layer
            .node()
            .descendant_for_byte_range(edited_range.start, edited_range.end)
        else {
            continue;
        };
        let mut jsx_open_tag_node = node;
        if node.grammar_name() != config.open_tag_node_name {
            if let Some(parent) = node.parent() {
                if parent.grammar_name() == config.open_tag_node_name {
                    jsx_open_tag_node = parent;
                }
            }
        }
        if jsx_open_tag_node.grammar_name() != config.open_tag_node_name {
            continue;
        }

        let first_two_chars: Option<[char; 2]> = {
            let mut chars = buffer
                .text_for_range(jsx_open_tag_node.byte_range())
                .flat_map(|chunk| chunk.chars());
            if let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
                Some([c1, c2])
            } else {
                None
            }
        };
        if let Some(chars) = first_two_chars {
            if chars[0] != '<' {
                continue;
            }
            if chars[1] == '!' || chars[1] == '/' {
                continue;
            }
        }

        to_auto_edit.push(JsxTagCompletionState {
            edit_index: index,
            open_tag_range: jsx_open_tag_node.byte_range(),
        });
    }
    if to_auto_edit.is_empty() {
        return None;
    } else {
        return Some(to_auto_edit);
    }
}

pub(crate) fn generate_auto_close_edits(
    buffer: &BufferSnapshot,
    ranges: &[Range<usize>],
    config: &JsxTagAutoCloseConfig,
    state: Vec<JsxTagCompletionState>,
) -> Result<Vec<(Range<Anchor>, String)>> {
    let mut edits = Vec::with_capacity(state.len());
    for auto_edit in state {
        let edited_range = ranges[auto_edit.edit_index].clone();
        let Some(layer) = buffer.syntax_ancestor(edited_range.clone()) else {
            continue;
        };
        let Some(open_tag) = layer.descendant_for_byte_range(
            auto_edit.open_tag_range.start,
            auto_edit.open_tag_range.end,
        ) else {
            continue;
        };
        assert!(open_tag.grammar_name() == config.open_tag_node_name);
        let tag_name_range = open_tag
            .child_by_field_name("name")
            .map_or(0..0, |node| node.byte_range());

        let tag_name = buffer.text_for_range(tag_name_range).collect::<String>();
        {
            let mut tree_root_node = open_tag;
            // todo! child_with_descendant
            while let Some(parent) = tree_root_node.parent() {
                tree_root_node = parent;
                if parent.is_error()
                    || (parent.kind() != config.jsx_element_node_name
                        && parent.kind() != config.open_tag_node_name)
                {
                    break;
                }
            }

            let mut unclosed_open_tag_count: i32 = 0;

            let mut stack = Vec::with_capacity(tree_root_node.descendant_count());
            stack.push(tree_root_node);

            let mut cursor = tree_root_node.walk();

            // todo! use cursor for more efficient traversal
            // if child -> go to child
            // else if next sibling -> go to next sibling
            // else -> go to parent
            // if parent == tree_root_node -> break
            while let Some(node) = stack.pop() {
                if node.kind() == config.open_tag_node_name {
                    if node.child_by_field_name("name").map_or(false, |node| {
                        buffer
                            .text_for_range(node.byte_range())
                            .equals_str(&tag_name)
                    }) {
                        unclosed_open_tag_count += 1;
                    }
                    continue;
                } else if node.kind() == config.close_tag_node_name {
                    if node.child_by_field_name("name").map_or(false, |node| {
                        buffer
                            .text_for_range(node.byte_range())
                            .equals_str(&tag_name)
                    }) {
                        unclosed_open_tag_count -= 1;
                    }
                    continue;
                } else if node.kind() == "jsx_self_closing_element" {
                    // don't recurse into jsx self-closing elements
                    continue;
                } else if node.kind() == "jsx_expression" {
                    // don't recurse into jsx expressions (it forms a new scope)
                    continue;
                }

                stack.extend(node.children(&mut cursor));
            }

            if unclosed_open_tag_count <= 0 {
                // skip if already closed
                continue;
            }
        }
        let edit_anchor = buffer.anchor_after(edited_range.end);
        let edit_range = edit_anchor..edit_anchor;
        edits.push((edit_range, format!("</{}>", tag_name)));
    }
    return Ok(edits);
}

pub(crate) fn enabled_in_any_buffer(
    multi_buffer: &Entity<MultiBuffer>,
    cx: &mut Context<Editor>,
) -> bool {
    let multi_buffer = multi_buffer.read(cx);
    let mut found_enabled = false;
    multi_buffer.for_each_buffer(|buffer| {
        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        for syntax_layer in snapshot.syntax_layers() {
            let language = syntax_layer.language;
            if language.config().jsx_tag_auto_close.is_none() {
                continue;
            }
            let language_settings = language::language_settings::language_settings(
                Some(language.name()),
                snapshot.file(),
                cx,
            );
            if language_settings.jsx_tag_auto_close.enabled {
                found_enabled = true;
            }
        }
    });

    return found_enabled;
}
