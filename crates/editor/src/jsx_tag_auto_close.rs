use anyhow::Result;
use gpui::{Context, Entity};
use multi_buffer::MultiBuffer;
use std::ops::Range;

use language::{BufferSnapshot, JsxTagAutoCloseConfig, Node};
use text::Anchor;

use crate::Editor;

pub struct JsxTagCompletionState {
    edit_index: usize,
    open_tag_range: Range<usize>,
}

/// Index of the named child within an open or close tag
/// that corresponds to the tag name
/// Note that this is not configurable, i.e. we assume the first
/// named child of a tag node is the tag name
const TS_NODE_TAG_NAME_CHILD_INDEX: usize = 0;

/// Maximum number of parent elements to walk back when checking if an open tag
/// is already closed.
///
/// See the comment in `generate_auto_close_edits` for more details
const ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT: usize = 2;

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
        let Some(layer) = buffer.smallest_syntax_layer_containing(edited_range.clone()) else {
            continue;
        };
        let Some(node) = layer
            .node()
            .named_descendant_for_byte_range(edited_range.start, edited_range.end)
        else {
            continue;
        };
        let mut jsx_open_tag_node = node;
        if dbg!(node.grammar_name()) != config.open_tag_node_name {
            if let Some(parent) = node.parent() {
                if dbg!(parent.grammar_name()) == config.open_tag_node_name {
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
        let Some(layer) = buffer.smallest_syntax_layer_containing(edited_range.clone()) else {
            continue;
        };
        // todo! perf: use layer.language().id_for_kind() to get kind ids for faster checking
        let layer_root_node = layer.node();
        let Some(open_tag) = layer_root_node.descendant_for_byte_range(
            auto_edit.open_tag_range.start,
            auto_edit.open_tag_range.end,
        ) else {
            continue;
        };
        assert!(open_tag.grammar_name() == config.open_tag_node_name);
        let tag_name = open_tag
            .named_child(TS_NODE_TAG_NAME_CHILD_INDEX)
            .filter(|node| node.kind() == config.tag_name_node_name)
            .map_or("".to_string(), |node| {
                buffer.text_for_range(node.byte_range()).collect::<String>()
            });

        /*
         * Naive check to see if the tag is already closed
         * Essentially all we do is count the number of open and close tags
         * with the same tag name as the open tag just entered by the user
         * The search is limited to some scope determined by
         * `ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT`
         *
         * The limit is preferable to walking up the tree until we find a non-tag node,
         * and then checking the entire tree, as this is unnecessarily expensive, and
         * risks false positives
         * eg. a `</div>` tag without a corresponding opening tag exists 25 lines away
         *     and the user typed in `<div>`, intuitively we still want to auto-close it because
         *     the other `</div>` tag is almost certainly not supposed to be the closing tag for the
         *     current element
         *
         * We have to walk up the tree some amount because tree-sitters error correction is not
         * designed to handle this case, and usually does not represent the tree structure
         * in the way we might expect,
         *
         * We half to walk up the tree until we hit an element with a different open tag name (`doing_deep_search == true`)
         * because tree-sitter may pair the new open tag with the root of the tree's closing tag leaving the
         * root's opening tag unclosed.
         * e.g
         *      ```
         *      <div>
         *          <div>|cursor here|
         *      </div>
         *      ```
         *     in Astro/vue/svelte tree-sitter represented the tree as
         *      (
         *          (jsx_element
         *              (jsx_opening_element
         *                  "<div>")
         *          )
         *          (jsx_element
         *              (jsx_opening_element
         *                  "<div>") // <- cursor is here
         *              (jsx_closing_element
         *                  "</div>")
         *          )
         *      )
         *     so if we only walked to the first `jsx_element` node,
         *     we would mistakenly identify the div entered by the
         *     user as already being closed, despite this clearly
         *     being false
         *
         * The errors with the tree-sitter tree caused by error correction,
         * are also why the naive algorithm was chosen, as the alternative
         * approach would be to maintain or construct a full parse tree (like tree-sitter)
         * that better represents errors in a way that we can simply check
         * the enclosing scope of the entered tag for a closing tag
         * This is far more complex and expensive, and was deemed impractical
         * given that the naive algorithm is sufficient in the majority of cases.
         */
        {
            let tag_node_name_equals = |node: &Node, tag_name_node_name: &str, name: &str| {
                let is_empty = name.len() == 0;
                if let Some(node_name) = node.named_child(TS_NODE_TAG_NAME_CHILD_INDEX) {
                    if node_name.kind() != tag_name_node_name {
                        return is_empty;
                    }
                    let range = node_name.byte_range();
                    return buffer.text_for_range(range).equals_str(name);
                }
                return is_empty;
            };

            let mut found_non_tag_root = false;
            let tree_root_node = {
                // todo! circular buffer of length ALREADY_CLOSED_PARENT_WALK_BACK_LIMIT using indices rather than Vec
                let mut ancestors = Vec::with_capacity(
                    layer_root_node.descendant_count() - open_tag.descendant_count(),
                );
                ancestors.push(layer_root_node);
                let mut cur = layer_root_node;
                // walk down the tree until we hit the open tag
                // note: this is what node.parent() does internally
                while let Some(descendant) = cur.child_with_descendant(open_tag) {
                    if descendant == open_tag {
                        break;
                    }
                    ancestors.push(descendant);
                    cur = descendant;
                }

                assert!(ancestors.len() > 0);

                let mut tree_root_node = open_tag;

                let mut parent_element_node_count = 0;
                let mut doing_deep_search = false;

                for &ancestor in ancestors.iter().rev() {
                    tree_root_node = ancestor;
                    let is_element = ancestor.kind() == config.jsx_element_node_name;
                    let is_error = ancestor.is_error();
                    if is_error || !is_element {
                        found_non_tag_root = true;
                        break;
                    }
                    if is_element {
                        let is_first = parent_element_node_count == 0;
                        if !is_first {
                            let has_open_tag_with_same_tag_name = ancestor
                                .named_child(0)
                                .filter(|n| n.kind() == config.open_tag_node_name)
                                .map_or(false, |element_open_tag_node| {
                                    tag_node_name_equals(
                                        &element_open_tag_node,
                                        &config.tag_name_node_name,
                                        &tag_name,
                                    )
                                });
                            if has_open_tag_with_same_tag_name {
                                doing_deep_search = true;
                            } else if doing_deep_search {
                                break;
                            }
                        }
                        parent_element_node_count += 1;
                        if !doing_deep_search
                            && parent_element_node_count
                                >= ALREADY_CLOSED_PARENT_ELEMENT_WALK_BACK_LIMIT
                        {
                            break;
                        }
                    }
                }
                tree_root_node
            };

            let mut unclosed_open_tag_count: i32 = 0;

            let mut cursor = tree_root_node.walk();

            let mut stack = Vec::with_capacity(tree_root_node.descendant_count());

            if found_non_tag_root {
                stack.extend(tree_root_node.children(&mut cursor));
            } else {
                stack.push(tree_root_node);
            }

            let mut has_erroneous_close_tag = false;
            let mut erroneous_close_tag_node_name = "";
            let mut erroneous_close_tag_name_node_name = "";
            if let Some(name) = config.erroneous_close_tag_node_name.as_deref() {
                has_erroneous_close_tag = true;
                erroneous_close_tag_node_name = name;
                erroneous_close_tag_name_node_name = config
                    .erroneous_close_tag_name_node_name
                    .as_deref()
                    .unwrap_or(&config.tag_name_node_name);
            }

            // todo! use cursor for more efficient traversal
            // if child -> go to child
            // else if next sibling -> go to next sibling
            // else -> go to parent
            // if parent == tree_root_node -> break
            while let Some(node) = stack.pop() {
                let kind = node.kind();
                if kind == config.open_tag_node_name {
                    if tag_node_name_equals(&node, &config.tag_name_node_name, &tag_name) {
                        unclosed_open_tag_count += 1;
                    }
                } else if kind == config.close_tag_node_name {
                    if tag_node_name_equals(&node, &config.tag_name_node_name, &tag_name) {
                        // todo! node range shouldn't be before open tag
                        unclosed_open_tag_count -= 1;
                    }
                } else if has_erroneous_close_tag && kind == erroneous_close_tag_node_name {
                    if tag_node_name_equals(&node, erroneous_close_tag_name_node_name, &tag_name) {
                        // todo! node range shouldn't be before open tag
                        unclosed_open_tag_count -= 1;
                    }
                } else if kind == config.jsx_element_node_name {
                    // todo! perf: filter only open,close,element nodes
                    stack.extend(node.children(&mut cursor));
                }
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

pub(crate) fn refresh_enabled_in_any_buffer(
    editor: &mut Editor,
    multi_buffer: &Entity<MultiBuffer>,
    cx: &mut Context<Editor>,
) {
    editor.jsx_tag_auto_close_enabled_in_any_buffer = enabled_in_any_buffer(multi_buffer, cx);
}
