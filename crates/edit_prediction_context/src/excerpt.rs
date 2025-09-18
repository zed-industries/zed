use language::BufferSnapshot;
use std::ops::Range;
use text::{Point, ToOffset as _, ToPoint as _};
use tree_sitter::{Node, TreeCursor};
use util::RangeExt;

use crate::{BufferDeclaration, declaration::DeclarationId, syntax_index::SyntaxIndexState};

// TODO:
//
// - Test parent signatures
//
// - Decide whether to count signatures against the excerpt size. Could instead defer this to prompt
// planning.
//
// - Still return an excerpt even if the line around the cursor doesn't fit (e.g. for a markdown
// paragraph).
//
// - Truncation of long lines.
//
// - Filter outer syntax layers that don't support edit prediction.

#[derive(Debug, Clone)]
pub struct EditPredictionExcerptOptions {
    /// Limit for the number of bytes in the window around the cursor.
    pub max_bytes: usize,
    /// Minimum number of bytes in the window around the cursor. When syntax tree selection results
    /// in an excerpt smaller than this, it will fall back on line-based selection.
    pub min_bytes: usize,
    /// Target ratio of bytes before the cursor divided by total bytes in the window.
    pub target_before_cursor_over_total_bytes: f32,
}

#[derive(Debug, Clone)]
pub struct EditPredictionExcerpt {
    pub range: Range<usize>,
    pub parent_declarations: Vec<(DeclarationId, Range<usize>)>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct EditPredictionExcerptText {
    pub body: String,
    pub parent_signatures: Vec<String>,
}

impl EditPredictionExcerpt {
    pub fn text(&self, buffer: &BufferSnapshot) -> EditPredictionExcerptText {
        let body = buffer
            .text_for_range(self.range.clone())
            .collect::<String>();
        let parent_signatures = self
            .parent_declarations
            .iter()
            .map(|(_, range)| buffer.text_for_range(range.clone()).collect::<String>())
            .collect();
        EditPredictionExcerptText {
            body,
            parent_signatures,
        }
    }

    /// Selects an excerpt around a buffer position, attempting to choose logical boundaries based
    /// on TreeSitter structure and approximately targeting a goal ratio of bytesbefore vs after the
    /// cursor.
    ///
    /// When `index` is provided, the excerpt will include the signatures of parent outline items.
    ///
    /// First tries to use AST node boundaries to select the excerpt, and falls back on line-based
    /// expansion.
    ///
    /// Returns `None` if the line around the cursor doesn't fit.
    pub fn select_from_buffer(
        query_point: Point,
        buffer: &BufferSnapshot,
        options: &EditPredictionExcerptOptions,
        syntax_index: Option<&SyntaxIndexState>,
    ) -> Option<Self> {
        if buffer.len() <= options.max_bytes {
            log::debug!(
                "using entire file for excerpt since source length ({}) <= window max bytes ({})",
                buffer.len(),
                options.max_bytes
            );
            return Some(EditPredictionExcerpt::new(0..buffer.len(), Vec::new()));
        }

        let query_offset = query_point.to_offset(buffer);
        let query_range = Point::new(query_point.row, 0).to_offset(buffer)
            ..Point::new(query_point.row + 1, 0).to_offset(buffer);
        if query_range.len() >= options.max_bytes {
            return None;
        }

        let parent_declarations = if let Some(syntax_index) = syntax_index {
            syntax_index
                .buffer_declarations_containing_range(buffer.remote_id(), query_range.clone())
                .collect()
        } else {
            Vec::new()
        };

        let excerpt_selector = ExcerptSelector {
            query_offset,
            query_range,
            parent_declarations: &parent_declarations,
            buffer,
            options,
        };

        if let Some(excerpt) = excerpt_selector.select_tree_sitter_nodes() {
            if excerpt.size >= options.min_bytes {
                return Some(excerpt);
            }
            log::debug!(
                "tree-sitter excerpt was {} bytes, smaller than min of {}, falling back on line-based selection",
                excerpt.size,
                options.min_bytes
            );
        } else {
            log::debug!(
                "couldn't find excerpt via tree-sitter, falling back on line-based selection"
            );
        }

        excerpt_selector.select_lines()
    }

    fn new(range: Range<usize>, parent_declarations: Vec<(DeclarationId, Range<usize>)>) -> Self {
        let size = range.len()
            + parent_declarations
                .iter()
                .map(|(_, range)| range.len())
                .sum::<usize>();
        Self {
            range,
            parent_declarations,
            size,
        }
    }

    fn with_expanded_range(&self, new_range: Range<usize>) -> Self {
        if !new_range.contains_inclusive(&self.range) {
            // this is an issue because parent_signature_ranges may be incorrect
            log::error!("bug: with_expanded_range called with disjoint range");
        }
        let mut parent_declarations = Vec::with_capacity(self.parent_declarations.len());
        for (declaration_id, range) in &self.parent_declarations {
            if !range.contains_inclusive(&new_range) {
                break;
            }
            parent_declarations.push((*declaration_id, range.clone()));
        }
        Self::new(new_range, parent_declarations)
    }

    fn parent_signatures_size(&self) -> usize {
        self.size - self.range.len()
    }
}

struct ExcerptSelector<'a> {
    query_offset: usize,
    query_range: Range<usize>,
    parent_declarations: &'a [(DeclarationId, &'a BufferDeclaration)],
    buffer: &'a BufferSnapshot,
    options: &'a EditPredictionExcerptOptions,
}

impl<'a> ExcerptSelector<'a> {
    /// Finds the largest node that is smaller than the window size and contains `query_range`.
    fn select_tree_sitter_nodes(&self) -> Option<EditPredictionExcerpt> {
        let selected_layer_root = self.select_syntax_layer()?;
        let mut cursor = selected_layer_root.walk();

        loop {
            let excerpt_range = node_line_start(cursor.node()).to_offset(&self.buffer)
                ..node_line_end(cursor.node()).to_offset(&self.buffer);
            if excerpt_range.contains_inclusive(&self.query_range) {
                let excerpt = self.make_excerpt(excerpt_range);
                if excerpt.size <= self.options.max_bytes {
                    return Some(self.expand_to_siblings(&mut cursor, excerpt));
                }
            } else {
                // TODO: Should still be able to handle this case via AST nodes. For example, this
                // can happen if the cursor is between two methods in a large class file.
                return None;
            }

            if cursor
                .goto_first_child_for_byte(self.query_range.start)
                .is_none()
            {
                return None;
            }
        }
    }

    /// Select the smallest syntax layer that exceeds max_len, or the largest if none exceed max_len.
    fn select_syntax_layer(&self) -> Option<Node<'_>> {
        let mut smallest_exceeding_max_len: Option<Node<'_>> = None;
        let mut largest: Option<Node<'_>> = None;
        for layer in self
            .buffer
            .syntax_layers_for_range(self.query_range.start..self.query_range.start, true)
        {
            let layer_range = layer.node().byte_range();
            if !layer_range.contains_inclusive(&self.query_range) {
                continue;
            }

            if layer_range.len() > self.options.max_bytes {
                match &smallest_exceeding_max_len {
                    None => smallest_exceeding_max_len = Some(layer.node()),
                    Some(existing) => {
                        if layer_range.len() < existing.byte_range().len() {
                            smallest_exceeding_max_len = Some(layer.node());
                        }
                    }
                }
            } else {
                match &largest {
                    None => largest = Some(layer.node()),
                    Some(existing) if layer_range.len() > existing.byte_range().len() => {
                        largest = Some(layer.node())
                    }
                    _ => {}
                }
            }
        }

        smallest_exceeding_max_len.or(largest)
    }

    // motivation for this and `goto_previous_named_sibling` is to avoid including things like
    // trailing unnamed "}" in body nodes
    fn goto_next_named_sibling(cursor: &mut TreeCursor) -> bool {
        while cursor.goto_next_sibling() {
            if cursor.node().is_named() {
                return true;
            }
        }
        false
    }

    fn goto_previous_named_sibling(cursor: &mut TreeCursor) -> bool {
        while cursor.goto_previous_sibling() {
            if cursor.node().is_named() {
                return true;
            }
        }
        false
    }

    fn expand_to_siblings(
        &self,
        cursor: &mut TreeCursor,
        mut excerpt: EditPredictionExcerpt,
    ) -> EditPredictionExcerpt {
        let mut forward_cursor = cursor.clone();
        let backward_cursor = cursor;
        let mut forward_done = !Self::goto_next_named_sibling(&mut forward_cursor);
        let mut backward_done = !Self::goto_previous_named_sibling(backward_cursor);
        loop {
            if backward_done && forward_done {
                break;
            }

            let mut forward = None;
            while !forward_done {
                let new_end = node_line_end(forward_cursor.node()).to_offset(&self.buffer);
                if new_end > excerpt.range.end {
                    let new_excerpt = excerpt.with_expanded_range(excerpt.range.start..new_end);
                    if new_excerpt.size <= self.options.max_bytes {
                        forward = Some(new_excerpt);
                        break;
                    } else {
                        log::debug!("halting forward expansion, as it doesn't fit");
                        forward_done = true;
                        break;
                    }
                }
                forward_done = !Self::goto_next_named_sibling(&mut forward_cursor);
            }

            let mut backward = None;
            while !backward_done {
                let new_start = node_line_start(backward_cursor.node()).to_offset(&self.buffer);
                if new_start < excerpt.range.start {
                    let new_excerpt = excerpt.with_expanded_range(new_start..excerpt.range.end);
                    if new_excerpt.size <= self.options.max_bytes {
                        backward = Some(new_excerpt);
                        break;
                    } else {
                        log::debug!("halting backward expansion, as it doesn't fit");
                        backward_done = true;
                        break;
                    }
                }
                backward_done = !Self::goto_previous_named_sibling(backward_cursor);
            }

            let go_forward = match (forward, backward) {
                (Some(forward), Some(backward)) => {
                    let go_forward = self.is_better_excerpt(&forward, &backward);
                    if go_forward {
                        excerpt = forward;
                    } else {
                        excerpt = backward;
                    }
                    go_forward
                }
                (Some(forward), None) => {
                    log::debug!("expanding forward, since backward expansion has halted");
                    excerpt = forward;
                    true
                }
                (None, Some(backward)) => {
                    log::debug!("expanding backward, since forward expansion has halted");
                    excerpt = backward;
                    false
                }
                (None, None) => break,
            };

            if go_forward {
                forward_done = !Self::goto_next_named_sibling(&mut forward_cursor);
            } else {
                backward_done = !Self::goto_previous_named_sibling(backward_cursor);
            }
        }

        excerpt
    }

    fn select_lines(&self) -> Option<EditPredictionExcerpt> {
        // early return if line containing query_offset is already too large
        let excerpt = self.make_excerpt(self.query_range.clone());
        if excerpt.size > self.options.max_bytes {
            log::debug!(
                "excerpt for cursor line is {} bytes, which exceeds the window",
                excerpt.size
            );
            return None;
        }
        let signatures_size = excerpt.parent_signatures_size();
        let bytes_remaining = self.options.max_bytes.saturating_sub(signatures_size);

        let before_bytes =
            (self.options.target_before_cursor_over_total_bytes * bytes_remaining as f32) as usize;

        let start_point = {
            let offset = self.query_offset.saturating_sub(before_bytes);
            let point = offset.to_point(self.buffer);
            Point::new(point.row + 1, 0)
        };
        let start_offset = start_point.to_offset(&self.buffer);
        let end_point = {
            let offset = start_offset + bytes_remaining;
            let point = offset.to_point(self.buffer);
            Point::new(point.row, 0)
        };
        let end_offset = end_point.to_offset(&self.buffer);

        // this could be expanded further since recalculated `signature_size` may be smaller, but
        // skipping that for now for simplicity
        //
        // TODO: could also consider checking if lines immediately before / after fit.
        let excerpt = self.make_excerpt(start_offset..end_offset);
        if excerpt.size > self.options.max_bytes {
            log::error!(
                "bug: line-based excerpt selection has size {}, \
                which is {} bytes larger than the max size",
                excerpt.size,
                excerpt.size - self.options.max_bytes
            );
        }
        return Some(excerpt);
    }

    fn make_excerpt(&self, range: Range<usize>) -> EditPredictionExcerpt {
        let parent_declarations = self
            .parent_declarations
            .iter()
            .filter(|(_, declaration)| declaration.item_range.contains_inclusive(&range))
            .map(|(id, declaration)| (*id, declaration.signature_range.clone()))
            .collect();
        EditPredictionExcerpt::new(range, parent_declarations)
    }

    /// Returns `true` if the `forward` excerpt is a better choice than the `backward` excerpt.
    fn is_better_excerpt(
        &self,
        forward: &EditPredictionExcerpt,
        backward: &EditPredictionExcerpt,
    ) -> bool {
        let forward_ratio = self.excerpt_range_ratio(forward);
        let backward_ratio = self.excerpt_range_ratio(backward);
        let forward_delta =
            (forward_ratio - self.options.target_before_cursor_over_total_bytes).abs();
        let backward_delta =
            (backward_ratio - self.options.target_before_cursor_over_total_bytes).abs();
        let forward_is_better = forward_delta <= backward_delta;
        if forward_is_better {
            log::debug!(
                "expanding forward since {} is closer than {} to {}",
                forward_ratio,
                backward_ratio,
                self.options.target_before_cursor_over_total_bytes
            );
        } else {
            log::debug!(
                "expanding backward since {} is closer than {} to {}",
                backward_ratio,
                forward_ratio,
                self.options.target_before_cursor_over_total_bytes
            );
        }
        forward_is_better
    }

    /// Returns the ratio of bytes before the cursor over bytes within the range.
    fn excerpt_range_ratio(&self, excerpt: &EditPredictionExcerpt) -> f32 {
        let Some(bytes_before_cursor) = self.query_offset.checked_sub(excerpt.range.start) else {
            log::error!("bug: edit prediction cursor offset is not outside the excerpt");
            return 0.0;
        };
        bytes_before_cursor as f32 / excerpt.range.len() as f32
    }
}

fn node_line_start(node: Node) -> Point {
    Point::new(node.start_position().row as u32, 0)
}

fn node_line_end(node: Node) -> Point {
    Point::new(node.end_position().row as u32 + 1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
    use util::test::{generate_marked_text, marked_text_offsets_by};

    fn create_buffer(text: &str, cx: &mut TestAppContext) -> BufferSnapshot {
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_lang().into(), cx));
        buffer.read_with(cx, |buffer, _| buffer.snapshot())
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }

    fn cursor_and_excerpt_range(text: &str) -> (String, usize, Range<usize>) {
        let (text, offsets) = marked_text_offsets_by(text, vec!['ˇ', '«', '»']);
        (text, offsets[&'ˇ'][0], offsets[&'«'][0]..offsets[&'»'][0])
    }

    fn check_example(options: EditPredictionExcerptOptions, text: &str, cx: &mut TestAppContext) {
        let (text, cursor, expected_excerpt) = cursor_and_excerpt_range(text);

        let buffer = create_buffer(&text, cx);
        let cursor_point = cursor.to_point(&buffer);

        let excerpt =
            EditPredictionExcerpt::select_from_buffer(cursor_point, &buffer, &options, None)
                .expect("Should select an excerpt");
        pretty_assertions::assert_eq!(
            generate_marked_text(&text, std::slice::from_ref(&excerpt.range), false),
            generate_marked_text(&text, &[expected_excerpt], false)
        );
        assert!(excerpt.size <= options.max_bytes);
        assert!(excerpt.range.contains(&cursor));
    }

    #[gpui::test]
    fn test_ast_based_selection_current_node(cx: &mut TestAppContext) {
        zlog::init_test();
        let text = r#"
fn main() {
    let x = 1;
«    let ˇy = 2;
»    let z = 3;
}"#;

        let options = EditPredictionExcerptOptions {
            max_bytes: 20,
            min_bytes: 10,
            target_before_cursor_over_total_bytes: 0.5,
        };

        check_example(options, text, cx);
    }

    #[gpui::test]
    fn test_ast_based_selection_parent_node(cx: &mut TestAppContext) {
        zlog::init_test();
        let text = r#"
fn foo() {}

«fn main() {
    let x = 1;
    let ˇy = 2;
    let z = 3;
}
»
fn bar() {}"#;

        let options = EditPredictionExcerptOptions {
            max_bytes: 65,
            min_bytes: 10,
            target_before_cursor_over_total_bytes: 0.5,
        };

        check_example(options, text, cx);
    }

    #[gpui::test]
    fn test_ast_based_selection_expands_to_siblings(cx: &mut TestAppContext) {
        zlog::init_test();
        let text = r#"
fn main() {
«    let x = 1;
    let ˇy = 2;
    let z = 3;
»}"#;

        let options = EditPredictionExcerptOptions {
            max_bytes: 50,
            min_bytes: 10,
            target_before_cursor_over_total_bytes: 0.5,
        };

        check_example(options, text, cx);
    }

    #[gpui::test]
    fn test_line_based_selection(cx: &mut TestAppContext) {
        zlog::init_test();
        let text = r#"
fn main() {
    let x = 1;
«    if true {
        let ˇy = 2;
    }
    let z = 3;
»}"#;

        let options = EditPredictionExcerptOptions {
            max_bytes: 60,
            min_bytes: 45,
            target_before_cursor_over_total_bytes: 0.5,
        };

        check_example(options, text, cx);
    }

    #[gpui::test]
    fn test_line_based_selection_with_before_cursor_ratio(cx: &mut TestAppContext) {
        zlog::init_test();
        let text = r#"
    fn main() {
«        let a = 1;
        let b = 2;
        let c = 3;
        let ˇd = 4;
        let e = 5;
        let f = 6;
»
        let g = 7;
    }"#;

        let options = EditPredictionExcerptOptions {
            max_bytes: 120,
            min_bytes: 10,
            target_before_cursor_over_total_bytes: 0.6,
        };

        check_example(options, text, cx);
    }
}
