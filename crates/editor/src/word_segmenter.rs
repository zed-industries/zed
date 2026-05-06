//! Fine-grained word segmentation using ICU4X.
//!
//! This module provides an opt-in word segmentation layer on top of Zed's
//! existing `CharClassifier`. When fine word segmentation is enabled,
//! Zed first finds the existing word ranges and then uses ICU4X to split ranges
//! where ICU4X finds multiple word-like subsegments.

use icu_segmenter::{WordSegmenter, options::WordBreakOptions};
use multi_buffer::ToPoint as _;
use std::{cell::RefCell, ops::Range};

thread_local! {
    static WORD_SEGMENTER_CACHE: RefCell<CachedWordSegmenter> = RefCell::new(CachedWordSegmenter::default());
}

/// A word range returned by the ICU4X segmenter, expressed as byte offsets
/// relative to the start of the line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WordSegmentRange {
    pub start: usize,
    pub end: usize,
}

/// Configuration for fine-grained word segmentation.
///
/// When this is disabled, callers preserve the existing `CharClassifier`-only
/// behavior.
pub struct WordSegmenterConfig {
    /// The resolved ICU4X `WordSegmenter`, or `None` when construction fails.
    segmenter: Option<WordSegmenter>,
}

#[derive(Default)]
struct CachedWordSegmenter {
    enabled: bool,
    config: Option<WordSegmenterConfig>,
    line: String,
    segments: Vec<WordSegmentRange>,
}

impl CachedWordSegmenter {
    fn segments_for_line(&mut self, line: &str, enabled: bool) -> Vec<WordSegmentRange> {
        if !enabled {
            return Vec::new();
        }

        if self.config.is_none() || self.enabled != enabled {
            self.enabled = enabled;
            self.config = Some(WordSegmenterConfig::new());
            self.line.clear();
            self.segments.clear();
        }

        if self.line != line {
            self.line.clear();
            self.line.push_str(line);
            self.segments = self
                .config
                .as_ref()
                .map_or_else(Vec::new, |config| config.word_segments_for_line(line));
        }

        self.segments.clone()
    }
}

impl WordSegmenterConfig {
    /// Build a config using ICU4X's default automatic word segmentation.
    pub fn new() -> Self {
        let options = WordBreakOptions::default();
        let segmenter = WordSegmenter::try_new_auto(options).ok();

        Self { segmenter }
    }

    /// Returns `true` when ICU4X segmentation is active.
    pub fn is_enabled(&self) -> bool {
        self.segmenter.is_some()
    }

    /// Return word-like segment ranges for a single line of text.
    ///
    /// - Returns an empty vector if ICU4X segmenter construction failed.
    /// - First splits the line with Zed's existing `CharClassifier`.
    /// - Word ranges are split again with ICU4X when ICU4X finds multiple
    ///   word-like subsegments; other ranges are returned unchanged.
    /// - Ranges are byte offsets relative to the start of `line`.
    pub fn word_segments_for_line(&self, line: &str) -> Vec<WordSegmentRange> {
        let Some(segmenter) = self.segmenter.as_ref() else {
            return Vec::new();
        };

        let mut result = Vec::new();
        let borrowed = segmenter.as_borrowed();

        for base_range in classifier_word_ranges(line) {
            let segment = &line[base_range.clone()];
            result.extend(
                icu_word_segments_for_range(&borrowed, segment, base_range.clone()).unwrap_or_else(
                    || {
                        vec![WordSegmentRange {
                            start: base_range.start,
                            end: base_range.end,
                        }]
                    },
                ),
            );
        }

        result
    }
}

fn icu_word_segments_for_range(
    segmenter: &icu_segmenter::WordSegmenterBorrowed<'_>,
    segment: &str,
    base_range: Range<usize>,
) -> Option<Vec<WordSegmentRange>> {
    let mut result = Vec::new();
    let mut prev: Option<(usize, icu_segmenter::options::WordType)> = None;

    for (boundary, word_type) in segmenter.segment_str(segment).iter_with_word_type() {
        if let Some((start, prev_word_type)) = prev {
            if word_type.is_word_like() || prev_word_type.is_word_like() {
                result.push(WordSegmentRange {
                    start: base_range.start + start,
                    end: base_range.start + boundary,
                });
            }
        }
        prev = Some((boundary, word_type));
    }

    (result.len() > 1).then_some(result)
}

fn classifier_word_ranges(line: &str) -> impl Iterator<Item = Range<usize>> + '_ {
    let classifier = language::CharClassifier::new(None)
        .scope_context(Some(language::CharScopeContext::Completion));
    let mut chars = line.char_indices();
    let mut prev = None;
    let mut start_ix = 0;

    std::iter::from_fn(move || {
        for (ix, c) in chars.by_ref() {
            let mut token = None;
            let kind = classifier.kind(c);
            if let Some((prev_char, prev_kind)) = prev
                && (kind != prev_kind
                    || (kind == language::CharKind::Punctuation && c != prev_char))
            {
                token = (prev_kind == language::CharKind::Word).then_some(start_ix..ix);
                start_ix = ix;
            }
            prev = Some((c, kind));
            if token.is_some() {
                return token;
            }
        }

        if start_ix < line.len() {
            let token = (prev.is_some_and(|(_, kind)| kind == language::CharKind::Word))
                .then_some(start_ix..line.len());
            start_ix = line.len();
            return token;
        }

        None
    })
}

fn word_segments_for_line_cached(line: &str, enabled: bool) -> Vec<WordSegmentRange> {
    WORD_SEGMENTER_CACHE.with(|cache| cache.borrow_mut().segments_for_line(line, enabled))
}

/// Find the ICU4X word-like segment that contains the given byte offset within a line.
///
/// Returns `None` when no word-like segment contains the offset, or the offset
/// is on a boundary between segments (in whitespace/punctuation).
pub fn find_word_segment_at_offset(
    segments: &[WordSegmentRange],
    offset: usize,
    line: &str,
) -> Option<Range<usize>> {
    for seg in segments {
        if seg.start <= offset && offset < seg.end {
            return Some(seg.start..seg.end);
        }
    }

    for seg in segments.iter().rev() {
        if seg.end == offset {
            let next_char = line[offset..].chars().next();
            if next_char.is_none_or(|ch| !ch.is_alphanumeric() && ch != '_') {
                return Some(seg.start..seg.end);
            }
        }
    }

    None
}

/// Enhanced `surrounding_word` that first checks ICU4X word segments.
///
/// This function mirrors `MultiBufferSnapshot::surrounding_word` but adds an
/// ICU4X-based layer on top. The logic is:
///
/// 1. If fine word segmentation is disabled, delegate entirely to `MultiBufferSnapshot::surrounding_word`
///    (existing behavior preserved).
/// 2. If fine word segmentation is enabled, find the existing word at `offset`.
/// 3. If ICU4X finds multiple word-like subsegments, return the inner segment
///    containing `offset` with `CharKind::Word`.
/// 4. Otherwise, fall back to `MultiBufferSnapshot::surrounding_word`.
///
/// This does not cross line boundaries, matching the existing behavior.
pub fn surrounding_word<T: multi_buffer::ToOffset>(
    snapshot: &multi_buffer::MultiBufferSnapshot,
    offset: T,
    scope_context: Option<language::CharScopeContext>,
    enabled: bool,
) -> (
    Range<multi_buffer::MultiBufferOffset>,
    Option<language::CharKind>,
) {
    let offset = offset.to_offset(snapshot);
    let standard_result = snapshot.surrounding_word(offset, scope_context);

    if !enabled || standard_result.1 != Some(language::CharKind::Word) {
        return standard_result;
    }

    // Determine the line boundaries in byte offsets.
    let point = snapshot.offset_to_point(offset);
    let line_start =
        multi_buffer::ToOffset::to_offset(&language::Point::new(point.row, 0), snapshot);
    let line_end = if point.row >= snapshot.max_point().row {
        snapshot.len()
    } else {
        multi_buffer::ToOffset::to_offset(&language::Point::new(point.row + 1, 0), snapshot)
    };

    let line_text = snapshot
        .text_for_range(line_start..line_end)
        .collect::<String>();
    // Trim trailing newline so the offset math is consistent.
    let line_text_trimmed = line_text.trim_end_matches('\n');
    let trimmed_len = line_text_trimmed.len();
    let offset_in_line = offset - line_start;

    // If the offset is beyond the trimmed line (e.g. on the newline), fall back.
    if offset_in_line > trimmed_len {
        return standard_result;
    }

    let segments = word_segments_for_line_cached(line_text_trimmed, enabled);

    if let Some(seg) = find_word_segment_at_offset(&segments, offset_in_line, line_text_trimmed) {
        return (
            (line_start + multi_buffer::MultiBufferOffset(seg.start))
                ..(line_start + multi_buffer::MultiBufferOffset(seg.end)),
            Some(language::CharKind::Word),
        );
    }

    standard_result
}

/// Returns the ICU4X word-like segments for the line containing the given offset.
///
/// Returns byte ranges relative to the buffer, not the line.
pub fn word_segments_for_offset(
    snapshot: &multi_buffer::MultiBufferSnapshot,
    offset: multi_buffer::MultiBufferOffset,
    enabled: bool,
) -> Vec<Range<usize>> {
    if !enabled {
        return Vec::new();
    }

    let point = snapshot.offset_to_point(offset);
    let line_start =
        multi_buffer::ToOffset::to_offset(&language::Point::new(point.row, 0), snapshot);
    let line_end = if point.row >= snapshot.max_point().row {
        snapshot.len()
    } else {
        multi_buffer::ToOffset::to_offset(&language::Point::new(point.row + 1, 0), snapshot)
    };

    let line_text = snapshot
        .text_for_range(line_start..line_end)
        .collect::<String>();
    let line_text_trimmed = line_text.trim_end_matches('\n');
    let offset_in_line = offset - line_start;

    if offset_in_line > line_text_trimmed.len() {
        return Vec::new();
    }

    word_segments_for_line_cached(line_text_trimmed, enabled)
        .into_iter()
        .map(|segment| line_start.0 + segment.start..line_start.0 + segment.end)
        .collect()
}

/// Find the previous ICU4X word boundary before the given offset.
///
/// Returns `None` if there is no ICU4X boundary before the offset on this line,
/// or if fine word segmentation is disabled.
pub fn previous_icu4x_word_boundary(
    snapshot: &multi_buffer::MultiBufferSnapshot,
    offset: multi_buffer::MultiBufferOffset,
    enabled: bool,
) -> Option<usize> {
    let segments = word_segments_for_offset(snapshot, offset, enabled);
    let offset_usize = offset.0;
    segments
        .into_iter()
        .filter_map(|segment| (segment.start < offset_usize).then_some(segment.start))
        .max()
}

/// Find the next ICU4X word boundary after the given offset.
///
/// Returns `None` if there is no ICU4X boundary after the offset on this line,
/// or if fine word segmentation is disabled.
pub fn next_icu4x_word_boundary(
    snapshot: &multi_buffer::MultiBufferSnapshot,
    offset: multi_buffer::MultiBufferOffset,
    enabled: bool,
) -> Option<usize> {
    let segments = word_segments_for_offset(snapshot, offset, enabled);
    let offset_usize = offset.0;
    segments
        .into_iter()
        .filter_map(|segment| (segment.end > offset_usize).then_some(segment.end))
        .min()
}

/// Enhanced `previous_word_start` that incorporates ICU4X word boundaries.
///
/// When fine word segmentation is enabled, ICU4X word segment boundaries are treated as
/// additional word boundaries. The function returns the nearest boundary from
/// either the standard `CharClassifier`-based logic or ICU4X.
pub fn previous_word_start_with_segmenter(
    map: &crate::DisplaySnapshot,
    point: crate::DisplayPoint,
    enabled: bool,
) -> crate::DisplayPoint {
    use crate::ToDisplayPoint;

    // Compute standard movement first
    let standard_result = super::movement::previous_word_start(map, point);

    if !enabled {
        return standard_result;
    }

    let buffer = map.buffer_snapshot();
    let offset = point.to_offset(map, crate::Bias::Left);

    // Compute ICU4X boundary
    if let Some(icu4x_boundary) = previous_icu4x_word_boundary(&buffer, offset, enabled) {
        let icu4x_offset = multi_buffer::MultiBufferOffset(icu4x_boundary);
        let icu4x_point = icu4x_offset.to_point(&buffer).to_display_point(map);
        // Return whichever boundary is closer to the starting point
        // For "previous" movement, we want the position that is *after* the other one
        // (closer to the original point)
        if icu4x_point > standard_result {
            return icu4x_point;
        }
    }

    standard_result
}

/// Enhanced `next_word_end` that incorporates ICU4X word boundaries.
///
/// When fine word segmentation is enabled, ICU4X word segment boundaries are treated as
/// additional word boundaries. The function returns the nearest boundary from
/// either the standard `CharClassifier`-based logic or ICU4X.
pub fn next_word_end_with_segmenter(
    map: &crate::DisplaySnapshot,
    point: crate::DisplayPoint,
    enabled: bool,
) -> crate::DisplayPoint {
    use crate::ToDisplayPoint;

    // Compute standard movement first
    let standard_result = super::movement::next_word_end(map, point);

    if !enabled {
        return standard_result;
    }

    let buffer = map.buffer_snapshot();
    let offset = point.to_offset(map, crate::Bias::Right);

    // Compute ICU4X boundary
    if let Some(icu4x_boundary) = next_icu4x_word_boundary(&buffer, offset, enabled) {
        let icu4x_offset = multi_buffer::MultiBufferOffset(icu4x_boundary);
        let icu4x_point = icu4x_offset.to_point(&buffer).to_display_point(map);
        // For "next" movement, we want the position that is *before* the other one
        // (closer to the original point)
        if icu4x_point < standard_result {
            return icu4x_point;
        }
    }

    standard_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zh_cn_enabled() {
        let config = WordSegmenterConfig::new();
        assert!(config.is_enabled());

        let segments = config.word_segments_for_line("这是一个测试");
        // Should produce at least one word-like segment
        assert!(!segments.is_empty());

        // All segments should be non-empty and within the line
        let line_len = "这是一个测试".len();
        for seg in &segments {
            assert!(seg.start < seg.end);
            assert!(seg.end <= line_len);
        }
    }

    fn segment_words<'a>(line: &'a str, segments: &[WordSegmentRange]) -> Vec<&'a str> {
        segments
            .iter()
            .map(|segment| &line[segment.start..segment.end])
            .collect()
    }

    #[test]
    fn test_ja_segmentation() {
        let config = WordSegmenterConfig::new();
        assert!(config.is_enabled());

        let line = "これはテストです";
        let segments = config.word_segments_for_line(line);
        assert_eq!(
            segment_words(line, &segments),
            ["これ", "は", "テスト", "です"]
        );

        let line_len = line.len();
        for seg in &segments {
            assert!(seg.start < seg.end);
            assert!(seg.end <= line_len);
        }
    }

    #[test]
    fn test_ascii_text_word_segments() {
        let config = WordSegmenterConfig::new();
        let segments = config.word_segments_for_line("hello world");
        // Should find "hello" and "world" as word-like segments
        assert!(segments.len() >= 2);

        let words = segment_words("hello world", &segments);
        assert!(words.contains(&"hello"));
        assert!(words.contains(&"world"));
    }

    #[test]
    fn test_mixed_code_and_cjk() {
        let config = WordSegmenterConfig::new();
        let line = "let message = \"这是一个测试\";";
        let segments = config.word_segments_for_line(line);

        // Should find word-like segments for both code identifiers and CJK text
        let words = segment_words(line, &segments);

        assert!(words.contains(&"let"));
        assert!(words.contains(&"message"));
        // The CJK portion inside the string should be segmented
        assert!(words.iter().any(|w| w.contains('这') || w.contains('测')));
    }

    #[test]
    fn test_chinese_sentence_keeps_final_word_before_punctuation() {
        let config = WordSegmenterConfig::new();
        let line = "源代码控制系统以及问题追踪或协作系统上的沟通。";
        let segments = config.word_segments_for_line(line);
        let words = segment_words(line, &segments);

        assert!(words.contains(&"沟通"));
        assert!(!words.contains(&"。"));
    }

    #[test]
    fn test_find_word_segment_at_offset() {
        let config = WordSegmenterConfig::new();
        let segments = config.word_segments_for_line("これはテストです");

        if !segments.is_empty() {
            // Test that we can find the segment containing the start of the first segment
            let first = &segments[0];
            let found = find_word_segment_at_offset(&segments, first.start, "これはテストです");
            assert_eq!(found, Some(first.start..first.end));

            // Test offset in the middle of the first segment
            let mid = (first.start + first.end) / 2;
            let found = find_word_segment_at_offset(&segments, mid, "これはテストです");
            assert_eq!(found, Some(first.start..first.end));
        }
    }

    #[test]
    fn test_find_word_segment_at_offset_before_punctuation() {
        let config = WordSegmenterConfig::new();
        let line = "的完整详细信息。";
        let segments = config.word_segments_for_line(line);
        let words = segment_words(line, &segments);
        let segment_index = words
            .iter()
            .position(|word| *word == "信息")
            .expect("expected 信息 segment");
        let segment = &segments[segment_index];

        assert_eq!(line[segment.end..].chars().next(), Some('。'));
        assert_eq!(
            find_word_segment_at_offset(&segments, segment.end, line),
            Some(segment.start..segment.end)
        );
    }

    #[test]
    fn test_previous_and_next_boundaries_use_segment_starts_and_ends() {
        let line = "  これはテストです  ";
        let segments = word_segments_for_line_cached(line, true);
        assert_eq!(
            segment_words(line, &segments),
            ["これ", "は", "テスト", "です"]
        );

        let first = &segments[0];
        let previous_start = segments
            .iter()
            .filter_map(|segment| (segment.start < first.end).then_some(segment.start))
            .max();
        let next_end = segments
            .iter()
            .filter_map(|segment| (segment.end > first.start - 1).then_some(segment.end))
            .min();

        assert_eq!(previous_start, Some(first.start));
        assert_eq!(next_end, Some(first.end));
        assert_ne!(previous_start, Some(first.end));
    }
}
