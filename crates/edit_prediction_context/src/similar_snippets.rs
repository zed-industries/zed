use language::BufferSnapshot;
use std::{borrow::Cow, ops::Range, sync::Arc};
use util::RangeExt as _;

use crate::{OccurrenceSource, Occurrences, SlidingWindow};

// TODO:
//
// * Use Tree-sitter nodes to reduce the number of windows considered / make snippet boundaries more
// sensible.
//
// * Also use adjacent_occurrences.
//
// * Use multiple window sizes, or somehow otherwise allow finding high quality smaller snippets.

// Potential future enhancements / things to consider:
//
// * Use edit history info? Exclude snippets that are part of edit history?
//
// * Tokenizer that includes symbols
//
// * Accumulate `Occurrences` for the whole buffer, for BM25/TF-IDF, where the buffer is the corpus.

#[derive(Clone, Debug)]
pub struct SimilarSnippet {
    pub score: f32,
    pub range: Range<usize>,
    pub text: Arc<str>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimilarityMetric {
    Jaccard,
    OverlapCoefficient,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimilarSnippetOptions {
    pub min_bytes: usize,
    pub max_bytes: usize,
    pub min_similarity: f32,
    pub similarity_metric: SimilarityMetric,
    pub max_result_count: usize,
}

impl SimilarSnippetOptions {
    pub const DEFAULT: Self = Self {
        min_bytes: 128,
        max_bytes: 256,
        min_similarity: 0.01,
        similarity_metric: SimilarityMetric::OverlapCoefficient,
        max_result_count: 5,
    };
}

impl Default for SimilarSnippetOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub fn similar_snippets<S: OccurrenceSource>(
    excerpt_trigram_occurrences: &Occurrences<S>,
    // todo!
    // backward_range: Option<Range<usize>>,
    forward_range: Range<usize>,
    buffer: &BufferSnapshot,
    options: &SimilarSnippetOptions,
) -> Vec<SimilarSnippet> {
    let mut window = SlidingWindow::with_capacity(excerpt_trigram_occurrences, 16);
    let mut lines = buffer.text_for_range(forward_range.clone()).lines();

    let mut top_windows: Vec<TopWindow> = Vec::new();

    let mut bytes = 0;
    let mut start_offset = forward_range.start;
    while let Some(line) = lines.next() {
        let len_with_newline = line.len() + 1;
        bytes += len_with_newline;
        if len_with_newline > options.max_bytes {
            window.clear();
            bytes = 0;
            continue;
        }
        window.push_back(len_with_newline, S::occurrences_in_str(line));
        while bytes > options.max_bytes {
            let popped_bytes = window.pop_front();
            start_offset += popped_bytes;
            bytes -= popped_bytes;
        }
        if bytes >= options.min_bytes {
            let similarity = match options.similarity_metric {
                SimilarityMetric::Jaccard => window.weighted_jaccard_similarity(),
                SimilarityMetric::OverlapCoefficient => window.weighted_overlap_coefficient(),
            };

            if similarity > options.min_similarity {
                insert_into_top_windows(
                    &mut top_windows,
                    similarity,
                    start_offset..start_offset + bytes,
                    options,
                );
            }
        }
    }

    top_windows
        .into_iter()
        .map(|window| SimilarSnippet {
            score: window.similarity,
            range: window.range.clone(),
            text: buffer
                .text_for_range(window.range)
                .collect::<Cow<str>>()
                .into(),
        })
        .collect()
}

struct TopWindow {
    similarity: f32,
    range: Range<usize>,
}

fn insert_into_top_windows(
    top_windows: &mut Vec<TopWindow>,
    similarity: f32,
    range: Range<usize>,
    options: &SimilarSnippetOptions,
) {
    if top_windows.len() >= options.max_result_count
        && let Some(min_top_window) = top_windows.last()
        && similarity <= min_top_window.similarity
    {
        return;
    }

    let mut ix = 0;
    let mut inserted = false;
    while ix < top_windows.len() {
        if top_windows[ix].similarity < similarity {
            let new_top_window = TopWindow {
                similarity,
                range: range.clone(),
            };
            if top_windows[ix].range.overlaps(&range) {
                top_windows[ix] = new_top_window;
                return;
            } else {
                top_windows.insert(ix, new_top_window);
                ix += 1;
                inserted = true;
                break;
            }
        } else {
            if top_windows[ix].range.overlaps(&range) {
                return;
            }
        }
        ix += 1;
    }

    if inserted {
        for ix in ix..top_windows.len() {
            if top_windows[ix].range.overlaps(&range) {
                top_windows.remove(ix);
                return;
            }
        }
        if top_windows.len() > options.max_result_count {
            top_windows.pop();
        }
    } else if top_windows.len() < options.max_result_count {
        top_windows.push(TopWindow { similarity, range });
    }
}
