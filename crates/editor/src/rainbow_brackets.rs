use crate::Editor;
use collections::HashMap;
use gpui::{Context, Hsla, Window};
use language::Bias;
use std::ops::Range;
use text::{Anchor, ToOffset};

/// Rainbow bracket highlighting uses multiple colors to distinguish bracket nesting levels
pub fn refresh_rainbow_bracket_highlights(
    editor: &mut Editor,
    _window: &mut Window,
    cx: &mut Context<Editor>,
) {
    // Clear existing rainbow highlights for all levels
    editor.clear_background_highlights::<RainbowLevel0>(cx);
    editor.clear_background_highlights::<RainbowLevel1>(cx);
    editor.clear_background_highlights::<RainbowLevel2>(cx);
    editor.clear_background_highlights::<RainbowLevel3>(cx);
    editor.clear_background_highlights::<RainbowLevel4>(cx);
    editor.clear_background_highlights::<RainbowLevel5>(cx);
    editor.clear_background_highlights::<RainbowLevel6>(cx);
    editor.clear_background_highlights::<RainbowLevel7>(cx);
    editor.clear_background_highlights::<RainbowLevel8>(cx);
    editor.clear_background_highlights::<RainbowLevel9>(cx);

    let multi_buffer = editor.buffer().read(cx);
    let multi_buffer_snapshot = multi_buffer.snapshot(cx);

    // For now, handle only singleton buffers
    if let Some((_, _, buffer_snapshot)) = multi_buffer_snapshot.as_singleton() {
        let language = buffer_snapshot.language();

        if let Some(language) = language {
            if let Some(grammar) = language.grammar() {
                if let Some(rainbow_config) = &grammar.rainbow_config {
                    let mut highlights_by_level: HashMap<usize, Vec<Range<Anchor>>> =
                        HashMap::new();

                    collect_rainbow_highlights(
                        buffer_snapshot,
                        rainbow_config,
                        &mut highlights_by_level,
                    );

                    // Apply highlights by level
                    for (level, ranges) in highlights_by_level {
                        // Convert text anchors to multi-buffer anchors
                        let multi_buffer_ranges: Vec<_> = ranges
                            .into_iter()
                            .map(|range| {
                                let start_offset = range.start.to_offset(buffer_snapshot);
                                let end_offset = range.end.to_offset(buffer_snapshot);
                                let start =
                                    multi_buffer_snapshot.anchor_at(start_offset, Bias::Left);
                                let end = multi_buffer_snapshot.anchor_at(end_offset, Bias::Right);
                                start..end
                            })
                            .collect();

                        // Create a unique type for each level to avoid conflicts
                        match level {
                            0 => editor.highlight_background::<RainbowLevel0>(
                                &multi_buffer_ranges,
                                get_rainbow_color_0,
                                cx,
                            ),
                            1 => editor.highlight_background::<RainbowLevel1>(
                                &multi_buffer_ranges,
                                get_rainbow_color_1,
                                cx,
                            ),
                            2 => editor.highlight_background::<RainbowLevel2>(
                                &multi_buffer_ranges,
                                get_rainbow_color_2,
                                cx,
                            ),
                            3 => editor.highlight_background::<RainbowLevel3>(
                                &multi_buffer_ranges,
                                get_rainbow_color_3,
                                cx,
                            ),
                            4 => editor.highlight_background::<RainbowLevel4>(
                                &multi_buffer_ranges,
                                get_rainbow_color_4,
                                cx,
                            ),
                            5 => editor.highlight_background::<RainbowLevel5>(
                                &multi_buffer_ranges,
                                get_rainbow_color_5,
                                cx,
                            ),
                            6 => editor.highlight_background::<RainbowLevel6>(
                                &multi_buffer_ranges,
                                get_rainbow_color_6,
                                cx,
                            ),
                            7 => editor.highlight_background::<RainbowLevel7>(
                                &multi_buffer_ranges,
                                get_rainbow_color_7,
                                cx,
                            ),
                            8 => editor.highlight_background::<RainbowLevel8>(
                                &multi_buffer_ranges,
                                get_rainbow_color_8,
                                cx,
                            ),
                            _ => editor.highlight_background::<RainbowLevel9>(
                                &multi_buffer_ranges,
                                get_rainbow_color_9,
                                cx,
                            ),
                        }
                    }
                }
            }
        }
    }
}

// Similar to Helix's RainbowScope structure
#[derive(Debug)]
struct RainbowScope {
    end_byte: usize,
    node: Option<usize>, // node ID, similar to Helix's Option<Node>
    level: usize,
}

fn collect_rainbow_highlights(
    buffer: &language::BufferSnapshot,
    rainbow_config: &language::RainbowConfig,
    highlights_by_level: &mut HashMap<usize, Vec<Range<Anchor>>>,
) {
    let mut scope_stack = Vec::<RainbowScope>::new();

    // TODO: Currently we can't use tree-sitter queries properly due to API limitations
    // in Zed. The proper implementation would use syntax.matches() but that's not
    // publicly accessible. For now, we have to iterate through the tree manually.

    // This is a temporary workaround until we can access the query matching API
    for layer in buffer.syntax_layers() {
        let tree = layer.node();

        // Walk the tree and match against the rainbow query
        // In the future, this should use proper query matching like Helix does
        walk_tree_for_rainbow(
            tree,
            buffer,
            rainbow_config,
            &mut scope_stack,
            highlights_by_level,
        );
    }
}

// Temporary tree walking function until we can use proper query matching
fn walk_tree_for_rainbow(
    node: language::Node,
    buffer: &language::BufferSnapshot,
    rainbow_config: &language::RainbowConfig,
    scope_stack: &mut Vec<RainbowScope>,
    highlights_by_level: &mut HashMap<usize, Vec<Range<Anchor>>>,
) {
    let byte_range = node.byte_range();

    // Pop any scopes that end before this node begins
    while scope_stack
        .last()
        .is_some_and(|scope| byte_range.start >= scope.end_byte)
    {
        scope_stack.pop();
    }

    // TODO: This is where we would use the actual query match result
    // For now, we check if this node should be a scope or bracket based on the query

    // Temporary: Check if this node matches a scope pattern
    // In proper implementation, this would come from query match results
    if rainbow_config.scope_capture_ix.is_some() && is_potential_scope_node(node.kind()) {
        scope_stack.push(RainbowScope {
            end_byte: byte_range.end,
            node: Some(node.id()),
            level: scope_stack.len(),
        });
    }

    // Check if this node is a bracket
    if rainbow_config.bracket_capture_ix.is_some() && is_bracket_node(node.kind()) {
        if let Some(scope) = scope_stack.last() {
            // Check if this bracket should be highlighted
            let should_highlight = if let Some(scope_node_id) = scope.node {
                // Only highlight if bracket is a direct child of the scope node
                node.parent()
                    .map_or(false, |parent| parent.id() == scope_node_id)
            } else {
                // include-children mode: highlight all brackets in this scope
                true
            };

            if should_highlight {
                let start = buffer.anchor_after(byte_range.start);
                let end = buffer.anchor_before(byte_range.end);
                let range = start..end;

                highlights_by_level
                    .entry(scope.level % 10)
                    .or_default()
                    .push(range);
            }
        }
    }

    // Recurse to children
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            walk_tree_for_rainbow(
                child,
                buffer,
                rainbow_config,
                scope_stack,
                highlights_by_level,
            );
        }
    }
}

// Temporary helper until we can use query results
// This is not ideal - we should be using the actual query matches
fn is_potential_scope_node(kind: &str) -> bool {
    // This function exists only because we can't access the proper query matching API
    // In a proper implementation, this would be determined by the rainbow.scm query
    matches!(
        kind,
        // Common scope nodes across languages
        "block"
            | "statement_block"
            | "compound_statement"
            | "object"
            | "array"
            | "list"
            | "arguments"
            | "parameters"
            | "formal_parameters"
            | "parenthesized_expression"
            | "tuple_expression"
            | "declaration_list"
            | "field_declaration_list"
            | "call_expression"
            | "function_call"
    ) || kind.contains("block")
        || kind.contains("list")
        || kind.contains("expression")
}

fn is_bracket_node(kind: &str) -> bool {
    matches!(kind, "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">")
}

fn get_rainbow_color_0(_theme: &theme::Theme) -> Hsla {
    hsla(0.0, 0.7, 0.6, 0.3) // Red
}

fn get_rainbow_color_1(_theme: &theme::Theme) -> Hsla {
    hsla(30.0, 0.7, 0.6, 0.3) // Orange
}

fn get_rainbow_color_2(_theme: &theme::Theme) -> Hsla {
    hsla(60.0, 0.7, 0.6, 0.3) // Yellow
}

fn get_rainbow_color_3(_theme: &theme::Theme) -> Hsla {
    hsla(120.0, 0.7, 0.6, 0.3) // Green
}

fn get_rainbow_color_4(_theme: &theme::Theme) -> Hsla {
    hsla(180.0, 0.7, 0.6, 0.3) // Cyan
}

fn get_rainbow_color_5(_theme: &theme::Theme) -> Hsla {
    hsla(240.0, 0.7, 0.6, 0.3) // Blue
}

fn get_rainbow_color_6(_theme: &theme::Theme) -> Hsla {
    hsla(270.0, 0.7, 0.6, 0.3) // Purple
}

fn get_rainbow_color_7(_theme: &theme::Theme) -> Hsla {
    hsla(0.0, 0.7, 0.6, 0.3) // Red (repeat)
}

fn get_rainbow_color_8(_theme: &theme::Theme) -> Hsla {
    hsla(30.0, 0.7, 0.6, 0.3) // Orange (repeat)
}

fn get_rainbow_color_9(_theme: &theme::Theme) -> Hsla {
    hsla(60.0, 0.7, 0.6, 0.3) // Yellow (repeat)
}

fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Hsla {
    Hsla {
        h: hue / 360.0,
        s: saturation,
        l: lightness,
        a: alpha,
    }
}

// Marker types for different rainbow levels
enum RainbowLevel0 {}
enum RainbowLevel1 {}
enum RainbowLevel2 {}
enum RainbowLevel3 {}
enum RainbowLevel4 {}
enum RainbowLevel5 {}
enum RainbowLevel6 {}
enum RainbowLevel7 {}
enum RainbowLevel8 {}
enum RainbowLevel9 {}
