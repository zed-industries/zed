use language::BufferSnapshot;
use std::{borrow::Cow, ops::Range, sync::Arc};
use text::ToOffset;
use util::RangeExt as _;

use crate::{
    OccurrenceSource, Occurrences, SlidingWindow,
    excerpt::{next_line_start, previous_line_start},
};

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
//
// * Configurable ratio of forward scan bytes when limit is reached

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
    pub min_bytes_delta_since_last_window: usize,
    pub min_similarity: f32,
    pub max_scan_bytes: usize,
    pub similarity_metric: SimilarityMetric,
    pub max_result_count: usize,
}

impl SimilarSnippetOptions {
    pub const DEFAULT: Self = Self {
        min_bytes: 128,
        max_bytes: 256,
        min_bytes_delta_since_last_window: 64,
        min_similarity: 0.05,
        max_scan_bytes: 512 * 1024,
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
    target: &Occurrences<S>,
    excerpt_range: Range<usize>,
    buffer: &BufferSnapshot,
    options: &SimilarSnippetOptions,
) -> Vec<SimilarSnippet> {
    let mut backward_range = 0..excerpt_range.start;
    let mut forward_range = excerpt_range.end..buffer.len();
    let full_scan_bytes = backward_range.len() + forward_range.len();
    if full_scan_bytes > options.max_scan_bytes {
        let backward_start = next_line_start(
            excerpt_range
                .start
                .saturating_sub(options.max_scan_bytes / 2),
            buffer,
        )
        .to_offset(buffer);
        backward_range = backward_start..excerpt_range.start;

        let remaining_bytes = options.max_scan_bytes - backward_range.len();
        let forward_end =
            previous_line_start(excerpt_range.end + remaining_bytes, buffer).to_offset(buffer);
        forward_range = excerpt_range.end..forward_end;
    }

    let mut window = SlidingWindow::with_capacity(target, 16);
    let mut top_windows: Vec<TopWindow> = Vec::new();
    add_similar_snippets_in_range(
        forward_range,
        buffer,
        options,
        &mut window,
        &mut top_windows,
    );
    window.clear();
    add_similar_snippets_in_range(
        backward_range,
        buffer,
        options,
        &mut window,
        &mut top_windows,
    );

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

fn add_similar_snippets_in_range<S: OccurrenceSource>(
    range: Range<usize>,
    buffer: &BufferSnapshot,
    options: &SimilarSnippetOptions,
    window: &mut SlidingWindow<usize, &Occurrences<S>, S>,
    top_windows: &mut Vec<TopWindow>,
) {
    let mut bytes = 0;
    let mut bytes_delta_since_last_window = usize::MAX;
    let mut start_offset = range.start;
    // TODO: This could be more efficient if there was a way to iterate the chunks within lines. The
    // chunk bytes can be iterated and provided to occurrences_in_utf8_bytes.
    let mut lines = buffer.text_for_range(range).lines();
    while let Some(line) = lines.next() {
        let len_with_newline = line.len() + 1;
        bytes += len_with_newline;
        if len_with_newline > options.max_bytes {
            window.clear();
            bytes_delta_since_last_window = usize::MAX;
            bytes = 0;
            continue;
        }
        bytes_delta_since_last_window =
            bytes_delta_since_last_window.saturating_add(len_with_newline);
        window.push_back(len_with_newline, S::occurrences_in_str(line));
        while bytes > options.max_bytes {
            let popped_bytes = window.pop_front();
            bytes -= popped_bytes;
            bytes_delta_since_last_window =
                bytes_delta_since_last_window.saturating_add(popped_bytes);
            start_offset += popped_bytes;
        }
        if bytes >= options.min_bytes
            && bytes_delta_since_last_window >= options.min_bytes_delta_since_last_window
        {
            let similarity = match options.similarity_metric {
                SimilarityMetric::Jaccard => window.weighted_jaccard_similarity(),
                SimilarityMetric::OverlapCoefficient => window.weighted_overlap_coefficient(),
            };

            if similarity > options.min_similarity {
                insert_into_top_windows(
                    similarity,
                    start_offset..start_offset + bytes,
                    options,
                    top_windows,
                );
            }
            bytes_delta_since_last_window = 0;
        }
    }
}

struct TopWindow {
    similarity: f32,
    range: Range<usize>,
}

/// Maintains the sort order of `top_windows`. If it overlaps with an existing window that has a
/// lower similarity score, that window is removed.
fn insert_into_top_windows(
    similarity: f32,
    range: Range<usize>,
    options: &SimilarSnippetOptions,
    top_windows: &mut Vec<TopWindow>,
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
