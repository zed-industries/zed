use crate::Editor;
use gpui::{Context, Hsla, Window};
use language::{Bias, BufferSnapshot};
use std::collections::HashMap;
use std::ops::Range;
use text::ToOffset;

/// Compute rainbow bracket highlights for the visible range
pub fn compute_rainbow_brackets_for_range(
    buffer_snapshot: &BufferSnapshot,
    range: Range<usize>,
) -> Option<HashMap<usize, Vec<Range<usize>>>> {
    let language = buffer_snapshot.language()?;
    let rainbow_config = language.grammar()?.rainbow_config.as_ref()?;
    
    let mut highlights_by_level: HashMap<usize, Vec<Range<usize>>> = HashMap::new();
    
    // Similar to Helix's RainbowScope structure
    #[derive(Debug)]
    struct RainbowScope {
        end_byte: usize,
        node: Option<usize>, // node ID
        level: usize,
    }
    
    let mut scope_stack = Vec::<RainbowScope>::new();
    
    // Use the proper tree-sitter query matching API
    let mut matches = buffer_snapshot.matches(range, |grammar| {
        grammar.rainbow_config.as_ref().map(|c| &c.query)
    });
    
    // Process all matches in order
    while let Some(mat) = matches.peek() {
        let byte_range = mat.captures[0].node.byte_range();
        
        // Pop any scopes that end before this capture begins
        while scope_stack
            .last()
            .is_some_and(|scope| byte_range.start >= scope.end_byte)
        {
            scope_stack.pop();
        }
        
        // Check which capture this is
        let is_scope_capture = rainbow_config
            .scope_capture_ix
            .map_or(false, |ix| mat.captures.iter().any(|c| c.index == ix));
        let is_bracket_capture = rainbow_config
            .bracket_capture_ix
            .map_or(false, |ix| mat.captures.iter().any(|c| c.index == ix));
        
        if is_scope_capture {
            // Process scope capture
            if let Some(scope_capture) = rainbow_config
                .scope_capture_ix
                .and_then(|ix| mat.captures.iter().find(|c| c.index == ix))
            {
                let node = scope_capture.node;
                let byte_range = node.byte_range();
                
                scope_stack.push(RainbowScope {
                    end_byte: byte_range.end,
                    node: if rainbow_config
                        .include_children_patterns
                        .contains(&mat.pattern_index)
                    {
                        None
                    } else {
                        Some(node.id())
                    },
                    level: scope_stack.len(),
                });
            }
        }
        
        if is_bracket_capture {
            // Process bracket capture
            if let Some(bracket_capture) = rainbow_config
                .bracket_capture_ix
                .and_then(|ix| mat.captures.iter().find(|c| c.index == ix))
            {
                let node = bracket_capture.node;
                let byte_range = node.byte_range();
                
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
                        let level = scope.level % 10;
                        highlights_by_level
                            .entry(level)
                            .or_default()
                            .push(byte_range);
                    }
                }
            }
        }
        
        matches.advance();
    }
    
    Some(highlights_by_level)
}

/// Rainbow bracket highlighting uses multiple colors to distinguish bracket nesting levels
pub fn refresh_rainbow_bracket_highlights(
    editor: &mut Editor,
    _window: &mut Window,
    cx: &mut Context<Editor>,
) {
    // Clear existing rainbow highlights for all levels
    clear_current_rainbow_highlights(editor, cx);

    let multi_buffer = editor.buffer().read(cx);
    let multi_buffer_snapshot = multi_buffer.snapshot(cx);

    // For now, handle only singleton buffers
    if let Some((_, _, buffer_snapshot)) = multi_buffer_snapshot.as_singleton() {
        // Compute only for the visible range
        // Get the display map to find visible rows
        let display_map = editor.display_map.update(cx, |map, cx| map.snapshot(cx));
        let scroll_position = editor.scroll_position(cx);
        let height = editor.visible_line_count().unwrap_or(50.0);
        
        // Calculate visible display rows
        let start_row = scroll_position.y.floor() as u32;
        let end_row = ((scroll_position.y + height).ceil() as u32).min(display_map.max_point().row().0);
        
        // Convert display rows to buffer offsets
        let start_point = display_map.display_point_to_point(
            crate::DisplayPoint::new(crate::DisplayRow(start_row), 0),
            crate::Bias::Left
        );
        let end_point = display_map.display_point_to_point(
            crate::DisplayPoint::new(crate::DisplayRow(end_row), 0),
            crate::Bias::Right
        );
        
        let start_offset = start_point.to_offset(buffer_snapshot);
        let end_offset = end_point.to_offset(buffer_snapshot);
        
        if let Some(highlights_by_level) = compute_rainbow_brackets_for_range(
            buffer_snapshot,
            start_offset..end_offset,
        ) {
            // Apply highlights by level
            for (level, ranges) in highlights_by_level {
                // Convert text ranges to multi-buffer anchors
                let multi_buffer_ranges: Vec<_> = ranges
                    .into_iter()
                    .map(|range| {
                        let start = multi_buffer_snapshot.anchor_at(range.start, Bias::Left);
                        let end = multi_buffer_snapshot.anchor_at(range.end, Bias::Right);
                        start..end
                    })
                    .collect();

                // TODO: make it a text style instead of a background highlight
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

fn clear_current_rainbow_highlights(editor: &mut Editor, cx: &mut Context<Editor>) {
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
}

// TODO! Make it configurable from settings
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
