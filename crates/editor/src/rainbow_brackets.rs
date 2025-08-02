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

fn collect_rainbow_highlights(
    buffer: &language::BufferSnapshot,
    rainbow_config: &language::RainbowConfig,
    highlights_by_level: &mut HashMap<usize, Vec<Range<Anchor>>>,
) {
    #[derive(Debug)]
    struct RainbowScope {
        end_byte: usize,
        node_id: Option<usize>,
        pattern_ix: usize,
        level: usize,
    }

    #[derive(Debug)]
    struct CaptureInfo<'a> {
        byte_pos: usize,
        is_scope: bool,
        node: language::Node<'a>,
        pattern_ix: usize,
    }

    let mut all_captures = Vec::new();

    // Process each syntax layer
    for layer in buffer.syntax_layers() {
        let tree = layer.node();

        // Collect all nodes and check them against the query
        let mut stack = vec![(tree, 0)];
        let mut visited_nodes = collections::HashSet::default();

        while let Some((node, depth)) = stack.pop() {
            let node_id = node.id();
            if !visited_nodes.insert(node_id) {
                continue;
            }

            // Use query cursor to check if this node matches
            language::with_query_cursor(|cursor| {
                cursor.set_byte_range(node.byte_range());

                // Check if this node matches any pattern in the rainbow query
                // We'll use a simplified approach to check for matches
                let rope = buffer.as_rope();
                let node_text = {
                    let byte_range = node.byte_range();
                    rope.chunks_in_range(byte_range)
                        .collect::<String>()
                        .into_bytes()
                };

                // Try to match starting at this node
                let mut matched = false;

                // Check if this node matches our query patterns
                // For now, we'll use a simple heuristic based on node kind
                if let Some(scope_ix) = rainbow_config.scope_capture_ix {
                    // Check if this is a scope node
                    let is_scope_node = match node.kind() {
                        // JavaScript scopes
                        "object"
                        | "array"
                        | "arguments"
                        | "formal_parameters"
                        | "statement_block"
                        | "parenthesized_expression"
                        | "call_expression"
                        | "type_parameters"
                        | "type_arguments"
                        | "jsx_element"
                        | "jsx_self_closing_element" => true,
                        // Rust scopes from the rainbow.scm file
                        "declaration_list"
                        | "field_declaration_list"
                        | "field_initializer_list"
                        | "enum_variant_list"
                        | "block"
                        | "match_block"
                        | "use_list"
                        | "struct_pattern"
                        | "ordered_field_declaration_list"
                        | "parameters"
                        | "tuple_type"
                        | "tuple_expression"
                        | "tuple_pattern"
                        | "tuple_struct_pattern"
                        | "unit_type"
                        | "unit_expression"
                        | "visibility_modifier"
                        | "token_repetition_pattern"
                        | "bracketed_type"
                        | "for_lifetimes"
                        | "array_type"
                        | "array_expression"
                        | "index_expression"
                        | "slice_pattern"
                        | "attribute_item"
                        | "inner_attribute_item"
                        | "token_tree_pattern"
                        | "macro_definition"
                        | "closure_parameters" => true,
                        _ => false,
                    };

                    if is_scope_node {
                        all_captures.push(CaptureInfo {
                            byte_pos: node.start_byte(),
                            is_scope: true,
                            node,
                            pattern_ix: 0, // Simplified - we'd need to match actual pattern
                        });
                        matched = true;
                    }
                }

                if let Some(bracket_ix) = rainbow_config.bracket_capture_ix {
                    // Check if this is a bracket node
                    let is_bracket_node = matches!(
                        node.kind(),
                        "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">" | "#" | "|"
                    );

                    if is_bracket_node {
                        all_captures.push(CaptureInfo {
                            byte_pos: node.start_byte(),
                            is_scope: false,
                            node,
                            pattern_ix: 0,
                        });
                        matched = true;
                    }
                }
            });

            // Add children to stack
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    stack.push((child, depth + 1));
                }
            }
        }
    }

    // Sort captures by byte position to process them in order
    all_captures.sort_by_key(|c| c.byte_pos);

    // Process captures to assign rainbow levels
    let mut scope_stack = Vec::<RainbowScope>::new();

    for capture in all_captures {
        let byte_range = capture.node.byte_range();

        // Pop any scopes that have ended
        while scope_stack
            .last()
            .is_some_and(|scope| byte_range.start >= scope.end_byte)
        {
            scope_stack.pop();
        }

        if capture.is_scope {
            // This is a scope capture - push it onto the stack
            let node_id = if rainbow_config
                .include_children_patterns
                .contains(&capture.pattern_ix)
            {
                None
            } else {
                Some(capture.node.id())
            };

            scope_stack.push(RainbowScope {
                end_byte: byte_range.end,
                node_id,
                pattern_ix: capture.pattern_ix,
                level: scope_stack.len(),
            });
        } else {
            // This is a bracket capture - assign it a color based on the current scope
            if let Some(scope) = scope_stack.last() {
                let should_highlight = if let Some(scope_node_id) = scope.node_id {
                    // Only highlight if bracket is direct child of scope
                    capture
                        .node
                        .parent()
                        .map_or(false, |parent| parent.id() == scope_node_id)
                } else {
                    // include-children mode: highlight all brackets in scope
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
    }
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
