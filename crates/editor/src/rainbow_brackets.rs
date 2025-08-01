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
                    let mut highlights_by_level: HashMap<usize, Vec<Range<Anchor>>> = HashMap::new();
                    
                    collect_rainbow_highlights(
                        buffer_snapshot,
                        rainbow_config,
                        &mut highlights_by_level,
                    );
                    
                    // Apply highlights by level
                    for (level, ranges) in highlights_by_level {
                        // Convert text anchors to multi-buffer anchors
                        let multi_buffer_ranges: Vec<_> = ranges.into_iter()
                            .map(|range| {
                                let start_offset = range.start.to_offset(buffer_snapshot);
                                let end_offset = range.end.to_offset(buffer_snapshot);
                                let start = multi_buffer_snapshot.anchor_at(start_offset, Bias::Left);
                                let end = multi_buffer_snapshot.anchor_at(end_offset, Bias::Right);
                                start..end
                            })
                            .collect();
                    
                        // Create a unique type for each level to avoid conflicts
                        match level {
                            0 => editor.highlight_background::<RainbowLevel0>(&multi_buffer_ranges, get_rainbow_color_0, cx),
                            1 => editor.highlight_background::<RainbowLevel1>(&multi_buffer_ranges, get_rainbow_color_1, cx),
                            2 => editor.highlight_background::<RainbowLevel2>(&multi_buffer_ranges, get_rainbow_color_2, cx),
                            3 => editor.highlight_background::<RainbowLevel3>(&multi_buffer_ranges, get_rainbow_color_3, cx),
                            4 => editor.highlight_background::<RainbowLevel4>(&multi_buffer_ranges, get_rainbow_color_4, cx),
                            5 => editor.highlight_background::<RainbowLevel5>(&multi_buffer_ranges, get_rainbow_color_5, cx),
                            6 => editor.highlight_background::<RainbowLevel6>(&multi_buffer_ranges, get_rainbow_color_6, cx),
                            7 => editor.highlight_background::<RainbowLevel7>(&multi_buffer_ranges, get_rainbow_color_7, cx),
                            8 => editor.highlight_background::<RainbowLevel8>(&multi_buffer_ranges, get_rainbow_color_8, cx),
                            _ => editor.highlight_background::<RainbowLevel9>(&multi_buffer_ranges, get_rainbow_color_9, cx),
                        }
                    }
                }
            }
        }
    }
}

fn collect_rainbow_highlights(
    buffer: &language::BufferSnapshot,
    _rainbow_config: &language::RainbowConfig,
    highlights_by_level: &mut HashMap<usize, Vec<Range<Anchor>>>,
) {
    // For now, just collect all brackets without scope tracking
    // This is a simplified implementation to get it working
    for layer in buffer.syntax_layers() {
        let tree = layer.node();
        
        // Find all bracket nodes
        let mut bracket_nodes = Vec::new();
        find_brackets(tree, &mut bracket_nodes);
        
        // Assign colors based on depth
        for (node, depth) in bracket_nodes {
            let byte_range = node.byte_range();
            let start = buffer.anchor_after(byte_range.start);
            let end = buffer.anchor_before(byte_range.end);
            let range = start..end;
            
            highlights_by_level
                .entry(depth % 10) // Cycle through 10 levels
                .or_default()
                .push(range);
        }
    }
}

fn find_brackets<'a>(
    node: language::Node<'a>,
    brackets: &mut Vec<(language::Node<'a>, usize)>,
) {
    // Simple depth-based approach
    fn walk_tree<'a>(node: language::Node<'a>, brackets: &mut Vec<(language::Node<'a>, usize)>, depth: usize) {
        let kind = node.kind();
        
        // Check if this is a bracket
        if matches!(kind, "[" | "]" | "{" | "}" | "(" | ")") {
            brackets.push((node, depth));
        }
        
        // Increase depth for scope nodes
        let new_depth = match kind {
            "object" | "array" | "arguments" | "formal_parameters" | "statement_block" 
            | "parenthesized_expression" | "call_expression" | "type_parameters" 
            | "type_arguments" | "jsx_element" | "jsx_self_closing_element" => depth + 1,
            _ => depth,
        };
        
        // Recurse to children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                walk_tree(child, brackets, new_depth);
            }
        }
    }
    
    walk_tree(node, brackets, 0);
}


fn get_rainbow_color_0(_theme: &theme::Theme) -> Hsla {
    hsla(0.0, 0.7, 0.6, 0.3)  // Red
}

fn get_rainbow_color_1(_theme: &theme::Theme) -> Hsla {
    hsla(30.0, 0.7, 0.6, 0.3)  // Orange
}

fn get_rainbow_color_2(_theme: &theme::Theme) -> Hsla {
    hsla(60.0, 0.7, 0.6, 0.3)  // Yellow
}

fn get_rainbow_color_3(_theme: &theme::Theme) -> Hsla {
    hsla(120.0, 0.7, 0.6, 0.3)  // Green
}

fn get_rainbow_color_4(_theme: &theme::Theme) -> Hsla {
    hsla(180.0, 0.7, 0.6, 0.3)  // Cyan
}

fn get_rainbow_color_5(_theme: &theme::Theme) -> Hsla {
    hsla(240.0, 0.7, 0.6, 0.3)  // Blue
}

fn get_rainbow_color_6(_theme: &theme::Theme) -> Hsla {
    hsla(270.0, 0.7, 0.6, 0.3)  // Purple
}

fn get_rainbow_color_7(_theme: &theme::Theme) -> Hsla {
    hsla(0.0, 0.7, 0.6, 0.3)  // Red (repeat)
}

fn get_rainbow_color_8(_theme: &theme::Theme) -> Hsla {
    hsla(30.0, 0.7, 0.6, 0.3)  // Orange (repeat)
}

fn get_rainbow_color_9(_theme: &theme::Theme) -> Hsla {
    hsla(60.0, 0.7, 0.6, 0.3)  // Yellow (repeat)
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