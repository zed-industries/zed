use std::ops::Range;

use language::BufferSnapshot;

use crate::{Occurrences, SlidingWindow, hashes_of_lowercase_identifier_parts};

// TODO:
//
// * Use Tree-sitter nodes to reduce the number of windows considered / make snippet boundaries more
// sensible.
//
// * Also use adjacent_occurrences.
//
// * Use multiple window sizes, or somehow otherwise allow finding high quality smaller snippets.

// Potential future enhancements:
//
// * Accumulate `Occurrences` for the whole buffer, for BM25/TF-IDF, where the buffer is the corpus.

#[derive(Clone, Debug)]
pub struct SimilarSnippet {
    pub score: f32,
}

pub fn similar_snippets(
    excerpt_occurrences: &Occurrences,
    skip_range: Option<Range<usize>>,
    buffer: &BufferSnapshot,
    min_bytes: usize,
    max_bytes: usize,
) -> Vec<SimilarSnippet> {
    let mut snippets = Vec::new();
    let mut sliding_window = SlidingWindow::new(excerpt_occurrences, 16);
    let mut lines = buffer.as_rope().chunks().lines();
    let mut row = 0;
    let mut start_row = 0;
    while let Some(line) = lines.next() {
        sliding_window.add(row, hashes_of_lowercase_identifier_parts(line));
        row += 1;
    }

    snippets
}
