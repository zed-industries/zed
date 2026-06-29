use crate::{CharClassifier, CharKind, CharScopeContext, LanguageScope};
use anyhow::{Context, anyhow};
use imara_diff::{
    Algorithm, Sink, diff,
    intern::{InternedInput, Interner, Token},
    sources::lines_with_terminator,
};
use std::{fmt::Write, iter, ops::Range, sync::Arc};

const MAX_WORD_DIFF_LEN: usize = 512;
const MAX_WORD_DIFF_LINE_COUNT: usize = 8;

/// Computes a diff between two strings, returning a unified diff string.
pub fn unified_diff(old_text: &str, new_text: &str) -> String {
    unified_diff_with_offsets(old_text, new_text, 0, 0)
}

/// Computes a diff between two strings, returning a unified diff string with
/// hunk headers adjusted to reflect the given starting line numbers (zero-indexed).
pub fn unified_diff_with_offsets(
    old_text: &str,
    new_text: &str,
    old_start_line: u32,
    new_start_line: u32,
) -> String {
    unified_diff_with_context(old_text, new_text, old_start_line, new_start_line, 3)
}

/// Computes a diff between two strings, returning a unified diff string with
/// hunk headers adjusted to reflect the given starting line numbers (zero-indexed),
/// and a configurable number of context lines around changes.
pub fn unified_diff_with_context(
    old_text: &str,
    new_text: &str,
    old_start_line: u32,
    new_start_line: u32,
    context_lines: u32,
) -> String {
    let input = InternedInput::new(old_text, new_text);
    diff(
        Algorithm::Histogram,
        &input,
        OffsetUnifiedDiffBuilder::new(&input, old_start_line, new_start_line, context_lines),
    )
}

/// A unified diff builder that applies line number offsets to hunk headers.
struct OffsetUnifiedDiffBuilder<'a> {
    before: &'a [Token],
    after: &'a [Token],
    interner: &'a Interner<&'a str>,

    pos: u32,
    before_hunk_start: u32,
    after_hunk_start: u32,
    before_hunk_len: u32,
    after_hunk_len: u32,

    old_line_offset: u32,
    new_line_offset: u32,
    context_lines: u32,

    buffer: String,
    dst: String,
}

impl<'a> OffsetUnifiedDiffBuilder<'a> {
    fn new(
        input: &'a InternedInput<&'a str>,
        old_line_offset: u32,
        new_line_offset: u32,
        context_lines: u32,
    ) -> Self {
        Self {
            before_hunk_start: 0,
            after_hunk_start: 0,
            before_hunk_len: 0,
            after_hunk_len: 0,
            old_line_offset,
            new_line_offset,
            context_lines,
            buffer: String::with_capacity(8),
            dst: String::new(),
            interner: &input.interner,
            before: &input.before,
            after: &input.after,
            pos: 0,
        }
    }

    fn print_tokens(&mut self, tokens: &[Token], prefix: char) {
        for &token in tokens {
            writeln!(&mut self.buffer, "{prefix}{}", self.interner[token]).unwrap();
        }
    }

    fn flush(&mut self) {
        if self.before_hunk_len == 0 && self.after_hunk_len == 0 {
            return;
        }

        let end = (self.pos + self.context_lines).min(self.before.len() as u32);
        self.update_pos(end, end);

        writeln!(
            &mut self.dst,
            "@@ -{},{} +{},{} @@",
            self.before_hunk_start + 1 + self.old_line_offset,
            self.before_hunk_len,
            self.after_hunk_start + 1 + self.new_line_offset,
            self.after_hunk_len,
        )
        .unwrap();
        write!(&mut self.dst, "{}", &self.buffer).unwrap();
        self.buffer.clear();
        self.before_hunk_len = 0;
        self.after_hunk_len = 0;
    }

    fn update_pos(&mut self, print_to: u32, move_to: u32) {
        self.print_tokens(&self.before[self.pos as usize..print_to as usize], ' ');
        let len = print_to - self.pos;
        self.pos = move_to;
        self.before_hunk_len += len;
        self.after_hunk_len += len;
    }
}

impl Sink for OffsetUnifiedDiffBuilder<'_> {
    type Out = String;

    fn process_change(&mut self, before: Range<u32>, after: Range<u32>) {
        if before.start - self.pos > self.context_lines * 2 {
            self.flush();
        }
        if self.before_hunk_len == 0 && self.after_hunk_len == 0 {
            self.pos = before.start.saturating_sub(self.context_lines);
            self.before_hunk_start = self.pos;
            self.after_hunk_start = after.start.saturating_sub(self.context_lines);
        }
        self.update_pos(before.start, before.end);
        self.before_hunk_len += before.end - before.start;
        self.after_hunk_len += after.end - after.start;
        self.print_tokens(
            &self.before[before.start as usize..before.end as usize],
            '-',
        );
        self.print_tokens(&self.after[after.start as usize..after.end as usize], '+');
    }

    fn finish(mut self) -> Self::Out {
        self.flush();
        self.dst
    }
}

/// Computes a diff between two strings, returning a vector of old and new row
/// ranges.
pub fn line_diff(old_text: &str, new_text: &str) -> Vec<(Range<u32>, Range<u32>)> {
    let mut edits = Vec::new();
    let input = InternedInput::new(
        lines_with_terminator(old_text),
        lines_with_terminator(new_text),
    );
    diff_internal(&input, &mut |_, _, old_rows, new_rows| {
        edits.push((old_rows, new_rows));
    });
    edits
}

/// Computes a diff between two strings, returning a vector of edits.
///
/// The edits are represented as tuples of byte ranges and replacement strings.
///
/// Internally, this function first performs a line-based diff, and then performs a second
/// word-based diff within hunks that replace small numbers of lines.
pub fn text_diff(old_text: &str, new_text: &str) -> Vec<(Range<usize>, Arc<str>)> {
    text_diff_with_options(old_text, new_text, DiffOptions::default())
}

/// Computes word-level diff ranges between two strings.
///
/// Returns a tuple of (old_ranges, new_ranges) where each vector contains
/// the byte ranges of changed words in the respective text.
pub fn word_diff_ranges(
    old_text: &str,
    new_text: &str,
    options: DiffOptions,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut input: InternedInput<&str> = InternedInput::default();
    input.update_before(tokenize(old_text, options.language_scope.clone()));
    input.update_after(tokenize(new_text, options.language_scope));
    collect_change_ranges(&input)
}

/// Maximum byte gap between change spans that will be merged into one highlight.
/// Keeps highlights from looking choppy around single punctuation or spaces
/// without swallowing unchanged words between larger edits.
const INTRA_LINE_MERGE_GAP: usize = 2;

/// Computes word/token-level intra-line diff ranges between two versions of a line.
///
/// Diffs by words and punctuation tokens (not individual characters) so highlights
/// align with whole tokens, but still returns **byte** ranges on character
/// boundaries. For `"let x = 1"` vs `"let x = 42"` only the `1` / `42` spans
/// are returned.
///
/// Whitespace-only edits (for example a shifted indent) produce ranges covering
/// just that whitespace, not the whole line. Adjacent change spans separated by
/// a tiny gap (≤ [`INTRA_LINE_MERGE_GAP`] bytes) are merged so highlights stay
/// continuous. Identical lines produce empty range lists; when no tokens match,
/// both sides are marked in full.
pub fn intra_line_diff(old_line: &str, new_line: &str) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    if old_line == new_line {
        return (Vec::new(), Vec::new());
    }

    let mut input: InternedInput<&str> = InternedInput::default();
    input.update_before(tokenize(old_line, None));
    input.update_after(tokenize(new_line, None));
    let (old_ranges, new_ranges) = collect_change_ranges(&input);

    let old_ranges = merge_nearby_ranges(old_ranges, INTRA_LINE_MERGE_GAP);
    let new_ranges = merge_nearby_ranges(new_ranges, INTRA_LINE_MERGE_GAP);

    // No shared tokens at all — treat as a full-line replacement so we don't
    // leave unhighlighted islands from accidental single-token matches on
    // wholly different lines. Skip this when the only changes are whitespace
    // so an indent shift stays tightly highlighted.
    if !old_line.is_empty()
        && !new_line.is_empty()
        && !is_whitespace_only_change(old_line, &old_ranges)
        && !is_whitespace_only_change(new_line, &new_ranges)
        && covers_all_non_whitespace(old_line, &old_ranges)
        && covers_all_non_whitespace(new_line, &new_ranges)
    {
        return (vec![0..old_line.len()], vec![0..new_line.len()]);
    }

    (old_ranges, new_ranges)
}

fn merge_nearby_ranges(ranges: Vec<Range<usize>>, max_gap: usize) -> Vec<Range<usize>> {
    let mut merged: Vec<Range<usize>> = Vec::new();
    for range in ranges {
        if range.is_empty() {
            continue;
        }
        if let Some(last) = merged.last_mut()
            && range.start <= last.end + max_gap
        {
            last.end = last.end.max(range.end);
        } else {
            merged.push(range);
        }
    }
    merged
}

fn is_whitespace_only_change(text: &str, ranges: &[Range<usize>]) -> bool {
    !ranges.is_empty()
        && ranges
            .iter()
            .all(|range| text[range.clone()].chars().all(|c| c.is_whitespace()))
}

fn covers_all_non_whitespace(text: &str, ranges: &[Range<usize>]) -> bool {
    if ranges.is_empty() {
        return text.chars().all(|c| c.is_whitespace());
    }
    let mut covered = vec![false; text.len()];
    for range in ranges {
        for byte in &mut covered[range.clone()] {
            *byte = true;
        }
    }
    text.char_indices().all(|(ix, c)| {
        c.is_whitespace() || covered[ix..ix + c.len_utf8()].iter().all(|&b| b)
    })
}

fn collect_change_ranges(
    input: &InternedInput<&str>,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut old_ranges: Vec<Range<usize>> = Vec::new();
    let mut new_ranges: Vec<Range<usize>> = Vec::new();

    diff_internal(input, &mut |old_byte_range, new_byte_range, _, _| {
        if !old_byte_range.is_empty() {
            if let Some(last) = old_ranges.last_mut()
                && last.end >= old_byte_range.start
            {
                last.end = old_byte_range.end;
            } else {
                old_ranges.push(old_byte_range);
            }
        }

        if !new_byte_range.is_empty() {
            if let Some(last) = new_ranges.last_mut()
                && last.end >= new_byte_range.start
            {
                last.end = new_byte_range.end;
            } else {
                new_ranges.push(new_byte_range);
            }
        }
    });

    (old_ranges, new_ranges)
}

/// Computes character-level diff between two strings.
///
/// Usually, you should use `text_diff`, which performs a word-wise diff.
pub fn char_diff<'a>(old_text: &'a str, new_text: &'a str) -> Vec<(Range<usize>, &'a str)> {
    let mut input: InternedInput<&str> = InternedInput::default();
    input.update_before(tokenize_chars(old_text));
    input.update_after(tokenize_chars(new_text));
    let mut edits: Vec<(Range<usize>, &str)> = Vec::new();
    diff_internal(&input, &mut |old_byte_range, new_byte_range, _, _| {
        let replacement = if new_byte_range.is_empty() {
            ""
        } else {
            &new_text[new_byte_range]
        };
        edits.push((old_byte_range, replacement));
    });
    edits
}

pub struct DiffOptions {
    pub language_scope: Option<LanguageScope>,
    pub max_word_diff_len: usize,
    pub max_word_diff_line_count: usize,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            language_scope: Default::default(),
            max_word_diff_len: MAX_WORD_DIFF_LEN,
            max_word_diff_line_count: MAX_WORD_DIFF_LINE_COUNT,
        }
    }
}

/// Computes a diff between two strings, using a specific language scope's
/// word characters for word-level diffing.
pub fn text_diff_with_options(
    old_text: &str,
    new_text: &str,
    options: DiffOptions,
) -> Vec<(Range<usize>, Arc<str>)> {
    let empty: Arc<str> = Arc::default();
    let mut edits = Vec::new();
    let mut hunk_input = InternedInput::default();
    let input = InternedInput::new(
        lines_with_terminator(old_text),
        lines_with_terminator(new_text),
    );
    diff_internal(&input, &mut |old_byte_range,
                                new_byte_range,
                                old_rows,
                                new_rows| {
        if should_perform_word_diff_within_hunk(
            &old_rows,
            &old_byte_range,
            &new_rows,
            &new_byte_range,
            &options,
        ) {
            let old_offset = old_byte_range.start;
            let new_offset = new_byte_range.start;
            hunk_input.clear();
            hunk_input.update_before(tokenize(
                &old_text[old_byte_range],
                options.language_scope.clone(),
            ));
            hunk_input.update_after(tokenize(
                &new_text[new_byte_range],
                options.language_scope.clone(),
            ));
            diff_internal(&hunk_input, &mut |old_byte_range, new_byte_range, _, _| {
                let old_byte_range =
                    old_offset + old_byte_range.start..old_offset + old_byte_range.end;
                let new_byte_range =
                    new_offset + new_byte_range.start..new_offset + new_byte_range.end;
                let replacement_text = if new_byte_range.is_empty() {
                    empty.clone()
                } else {
                    new_text[new_byte_range].into()
                };
                edits.push((old_byte_range, replacement_text));
            });
        } else {
            let replacement_text = if new_byte_range.is_empty() {
                empty.clone()
            } else {
                new_text[new_byte_range].into()
            };
            edits.push((old_byte_range, replacement_text));
        }
    });
    edits
}

pub fn apply_diff_patch(base_text: &str, patch: &str) -> Result<String, anyhow::Error> {
    let patch = diffy::Patch::from_str(patch).context("Failed to parse patch")?;
    let result = diffy::apply(base_text, &patch);
    result.map_err(|err| anyhow!(err))
}

pub fn apply_reversed_diff_patch(base_text: &str, patch: &str) -> Result<String, anyhow::Error> {
    let patch = diffy::Patch::from_str(patch).context("Failed to parse patch")?;
    let reversed = patch.reverse();
    diffy::apply(base_text, &reversed).map_err(|err| anyhow!(err))
}

fn should_perform_word_diff_within_hunk(
    old_row_range: &Range<u32>,
    old_byte_range: &Range<usize>,
    new_row_range: &Range<u32>,
    new_byte_range: &Range<usize>,
    options: &DiffOptions,
) -> bool {
    !old_byte_range.is_empty()
        && !new_byte_range.is_empty()
        && old_byte_range.len() <= options.max_word_diff_len
        && new_byte_range.len() <= options.max_word_diff_len
        && old_row_range.len() <= options.max_word_diff_line_count
        && new_row_range.len() <= options.max_word_diff_line_count
}

fn diff_internal(
    input: &InternedInput<&str>,
    on_change: &mut dyn FnMut(Range<usize>, Range<usize>, Range<u32>, Range<u32>),
) {
    let mut old_offset = 0;
    let mut new_offset = 0;
    let mut old_token_ix = 0;
    let mut new_token_ix = 0;
    diff(
        Algorithm::Histogram,
        input,
        |old_tokens: Range<u32>, new_tokens: Range<u32>| {
            old_offset += token_len(
                input,
                &input.before[old_token_ix as usize..old_tokens.start as usize],
            );
            new_offset += token_len(
                input,
                &input.after[new_token_ix as usize..new_tokens.start as usize],
            );
            let old_len = token_len(
                input,
                &input.before[old_tokens.start as usize..old_tokens.end as usize],
            );
            let new_len = token_len(
                input,
                &input.after[new_tokens.start as usize..new_tokens.end as usize],
            );
            let old_byte_range = old_offset..old_offset + old_len;
            let new_byte_range = new_offset..new_offset + new_len;
            old_token_ix = old_tokens.end;
            new_token_ix = new_tokens.end;
            old_offset = old_byte_range.end;
            new_offset = new_byte_range.end;
            on_change(old_byte_range, new_byte_range, old_tokens, new_tokens);
        },
    );
}

fn tokenize_chars(text: &str) -> impl Iterator<Item = &str> {
    let mut chars = text.char_indices().peekable();
    iter::from_fn(move || {
        let (start, c) = chars.next()?;
        Some(&text[start..start + c.len_utf8()])
    })
}

fn tokenize(text: &str, language_scope: Option<LanguageScope>) -> impl Iterator<Item = &str> {
    let classifier =
        CharClassifier::new(language_scope).scope_context(Some(CharScopeContext::Completion));
    let mut chars = text.char_indices();
    let mut prev = None;
    let mut start_ix = 0;
    iter::from_fn(move || {
        for (ix, c) in chars.by_ref() {
            let mut token = None;
            let kind = classifier.kind(c);
            if let Some((prev_char, prev_kind)) = prev
                && (kind != prev_kind || (kind == CharKind::Punctuation && c != prev_char))
            {
                token = Some(&text[start_ix..ix]);
                start_ix = ix;
            }
            prev = Some((c, kind));
            if token.is_some() {
                return token;
            }
        }
        if start_ix < text.len() {
            let token = &text[start_ix..];
            start_ix = text.len();
            return Some(token);
        }
        None
    })
}

fn token_len(input: &InternedInput<&str>, tokens: &[Token]) -> usize {
    tokens
        .iter()
        .map(|token| input.interner[*token].len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_tokenize() {
        let text = "";
        assert_eq!(tokenize(text, None).collect::<Vec<_>>(), Vec::<&str>::new());

        let text = " ";
        assert_eq!(tokenize(text, None).collect::<Vec<_>>(), vec![" "]);

        let text = "one";
        assert_eq!(tokenize(text, None).collect::<Vec<_>>(), vec!["one"]);

        let text = "one\n";
        assert_eq!(tokenize(text, None).collect::<Vec<_>>(), vec!["one", "\n"]);

        let text = "one.two(three)";
        assert_eq!(
            tokenize(text, None).collect::<Vec<_>>(),
            vec!["one", ".", "two", "(", "three", ")"]
        );

        let text = "one two three()";
        assert_eq!(
            tokenize(text, None).collect::<Vec<_>>(),
            vec!["one", " ", "two", " ", "three", "(", ")"]
        );

        let text = "   one\n two three";
        assert_eq!(
            tokenize(text, None).collect::<Vec<_>>(),
            vec!["   ", "one", "\n ", "two", " ", "three"]
        );
    }

    #[test]
    fn test_text_diff() {
        let old_text = "one two three";
        let new_text = "one TWO three";
        assert_eq!(text_diff(old_text, new_text), [(4..7, "TWO".into()),]);

        let old_text = "one\ntwo\nthree\n";
        let new_text = "one\ntwo\nAND\nTHEN\nthree\n";
        assert_eq!(
            text_diff(old_text, new_text),
            [(8..8, "AND\nTHEN\n".into()),]
        );

        let old_text = "one two\nthree four five\nsix seven eight nine\nten\n";
        let new_text = "one two\nthree FOUR five\nsix SEVEN eight nine\nten\nELEVEN\n";
        assert_eq!(
            text_diff(old_text, new_text),
            [
                (14..18, "FOUR".into()),
                (28..33, "SEVEN".into()),
                (49..49, "ELEVEN\n".into())
            ]
        );
    }

    #[test]
    fn test_apply_diff_patch() {
        let old_text = "one two\nthree four five\nsix seven eight nine\nten\n";
        let new_text = "one two\nthree FOUR five\nsix SEVEN eight nine\nten\nELEVEN\n";
        let patch = unified_diff(old_text, new_text);
        assert_eq!(apply_diff_patch(old_text, &patch).unwrap(), new_text);
    }

    #[test]
    fn test_apply_reversed_diff_patch() {
        let old_text = "one two\nthree four five\nsix seven eight nine\nten\n";
        let new_text = "one two\nthree FOUR five\nsix SEVEN eight nine\nten\nELEVEN\n";
        let patch = unified_diff(old_text, new_text);
        assert_eq!(
            apply_reversed_diff_patch(new_text, &patch).unwrap(),
            old_text
        );
    }

    #[test]
    fn test_char_diff() {
        assert_eq!(char_diff("", ""), vec![]);
        assert_eq!(char_diff("", "abc"), vec![(0..0, "abc")]);
        assert_eq!(char_diff("abc", ""), vec![(0..3, "")]);
        assert_eq!(char_diff("ac", "abc"), vec![(1..1, "b")]); // "b" inserted
        assert_eq!(char_diff("abc", "ac"), vec![(1..2, "")]); // "b" deleted
        assert_eq!(char_diff("abc", "adc"), vec![(1..2, "d")]); // "b" replaced with "d"
        assert_eq!(char_diff("日", "日本語"), vec![(3..3, "本語")]); // "本語" inserted
        assert_eq!(char_diff("日本語", "日"), vec![(3..9, "")]); // "本語" deleted
        assert_eq!(char_diff("🎉", "🎉🎊🎈"), vec![(4..4, "🎊🎈")]); // "🎊🎈" inserted
        assert_eq!(
            char_diff("test日本", "test日本語です"),
            vec![(10..10, "語です")]
        );
    }

    fn assert_ranges_valid(text: &str, ranges: &[Range<usize>]) {
        let mut prev_end = 0;
        for range in ranges {
            assert!(
                range.start <= range.end,
                "range start > end for {text:?}: {range:?}"
            );
            assert!(
                range.end <= text.len(),
                "range out of bounds for {text:?}: {range:?}"
            );
            assert!(
                text.is_char_boundary(range.start),
                "range start not on char boundary for {text:?}: {range:?}"
            );
            assert!(
                text.is_char_boundary(range.end),
                "range end not on char boundary for {text:?}: {range:?}"
            );
            assert!(
                range.start >= prev_end,
                "ranges not ordered / overlapping for {text:?}: {ranges:?}"
            );
            // Non-overlapping: next range must start at or after previous end.
            // Adjacent (touching) ranges are allowed.
            prev_end = range.end;
            // Slicing must not panic.
            let _ = &text[range.clone()];
        }
    }

    fn assert_intra_line_invariants(old_line: &str, new_line: &str) {
        let (old_ranges, new_ranges) = intra_line_diff(old_line, new_line);
        assert_ranges_valid(old_line, &old_ranges);
        assert_ranges_valid(new_line, &new_ranges);
        if old_line == new_line {
            assert!(old_ranges.is_empty());
            assert!(new_ranges.is_empty());
        }
    }

    fn highlighted<'a>(text: &'a str, ranges: &[Range<usize>]) -> Vec<&'a str> {
        ranges.iter().map(|r| &text[r.clone()]).collect()
    }

    #[test]
    fn test_intra_line_diff_identical() {
        assert_eq!(intra_line_diff("", ""), (vec![], vec![]));
        assert_eq!(intra_line_diff("let x = 1", "let x = 1"), (vec![], vec![]));
        assert_eq!(
            intra_line_diff("hello 日本語 🎉", "hello 日本語 🎉"),
            (vec![], vec![])
        );
        assert_intra_line_invariants("unchanged line", "unchanged line");
    }

    #[test]
    fn test_intra_line_diff_word_change_mid_line() {
        let old = "let x = 1";
        let new = "let x = 42";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["1"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["42"]);
        assert_intra_line_invariants(old, new);

        // Whole identifier tokens (underscores stay inside a word token).
        let old = "fn compute_total(items: &[Item])";
        let new = "fn compute_sum(items: &[Item])";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["compute_total"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["compute_sum"]);
        assert_intra_line_invariants(old, new);

        let old = "hello world foo bar";
        let new = "hello WORLD foo BAR";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["world", "bar"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["WORLD", "BAR"]);
        assert_intra_line_invariants(old, new);
    }

    #[test]
    fn test_intra_line_diff_prefix_and_suffix_changes() {
        // Prefix change only (leading word token).
        let old = "old = value";
        let new = "new = value";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["old"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["new"]);
        assert_intra_line_invariants(old, new);

        // Suffix change only (trailing word token).
        let old = "prefix alpha";
        let new = "prefix beta";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["alpha"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["beta"]);
        assert_intra_line_invariants(old, new);

        // Pure insertion in the middle.
        let old = "hello world";
        let new = "hello there world";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert!(old_ranges.is_empty());
        assert_eq!(highlighted(new, &new_ranges), vec!["there "]);
        assert_intra_line_invariants(old, new);

        // Pure deletion in the middle.
        let (old_ranges, new_ranges) = intra_line_diff(new, old);
        assert_eq!(highlighted(new, &old_ranges), vec!["there "]);
        assert!(new_ranges.is_empty());
        assert_intra_line_invariants(new, old);
    }

    #[test]
    fn test_intra_line_diff_totally_different() {
        let (old_ranges, new_ranges) = intra_line_diff("hello", "world");
        assert_eq!(old_ranges, vec![0.."hello".len()]);
        assert_eq!(new_ranges, vec![0.."world".len()]);

        let (old_ranges, new_ranges) = intra_line_diff("abc", "xyz");
        assert_eq!(old_ranges, vec![0..3]);
        assert_eq!(new_ranges, vec![0..3]);

        let (old_ranges, new_ranges) = intra_line_diff("one two three", "alpha beta gamma");
        assert_eq!(old_ranges, vec![0.."one two three".len()]);
        assert_eq!(new_ranges, vec![0.."alpha beta gamma".len()]);

        assert_eq!(intra_line_diff("", "abc"), (vec![], vec![0..3]));
        assert_eq!(intra_line_diff("abc", ""), (vec![0..3], vec![]));
        assert_intra_line_invariants("completely", "different");
    }

    #[test]
    fn test_intra_line_diff_whitespace_only() {
        // Leading indent insertion — do not light up the whole line.
        let old = "foo bar";
        let new = "    foo bar";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert!(old_ranges.is_empty());
        assert_eq!(highlighted(new, &new_ranges), vec!["    "]);
        assert_intra_line_invariants(old, new);

        // Indent width change.
        let old = "  foo";
        let new = "    foo";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["  "]);
        assert_eq!(highlighted(new, &new_ranges), vec!["    "]);
        assert_intra_line_invariants(old, new);

        // Trailing whitespace change.
        let old = "foo  ";
        let new = "foo    ";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(highlighted(old, &old_ranges), vec!["  "]);
        assert_eq!(highlighted(new, &new_ranges), vec!["    "]);
        assert_intra_line_invariants(old, new);

        // Tabs vs spaces as indent.
        let old = "\tfoo";
        let new = "    foo";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert!(old_ranges
            .iter()
            .all(|r| old[r.clone()].chars().all(|c| c.is_whitespace())));
        assert!(new_ranges
            .iter()
            .all(|r| new[r.clone()].chars().all(|c| c.is_whitespace())));
        assert_intra_line_invariants(old, new);
    }

    #[test]
    fn test_intra_line_diff_gap_merging() {
        // Tiny gap (single punctuation) between changes is merged.
        let old = "a,b";
        let new = "x,y";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(old_ranges, vec![0..3]);
        assert_eq!(new_ranges, vec![0..3]);
        assert_intra_line_invariants(old, new);

        // Larger gap between word edits must not merge.
        let old = "hello world foo bar";
        let new = "hello WORLD foo BAR";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(old_ranges.len(), 2);
        assert_eq!(new_ranges.len(), 2);
        assert!(old_ranges[0].end + INTRA_LINE_MERGE_GAP < old_ranges[1].start);
        assert_intra_line_invariants(old, new);

        // Adjacent punctuation-separated tokens within the merge gap.
        let old = "a.b";
        let new = "x.y";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_eq!(old_ranges.len(), 1);
        assert_eq!(new_ranges.len(), 1);
        assert_eq!(&old[old_ranges[0].clone()], "a.b");
        assert_eq!(&new[new_ranges[0].clone()], "x.y");
        assert_intra_line_invariants(old, new);
    }

    #[test]
    fn test_intra_line_diff_unicode() {
        let old = "greet 日";
        let new = "greet 日本語";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_ranges_valid(old, &old_ranges);
        assert_ranges_valid(new, &new_ranges);
        assert_eq!(highlighted(old, &old_ranges), vec!["日"]);
        assert_eq!(highlighted(new, &new_ranges), vec!["日本語"]);

        let old = "🎉 party";
        let new = "🎉🎊 party";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_ranges_valid(old, &old_ranges);
        assert_ranges_valid(new, &new_ranges);
        assert!(old_ranges.is_empty());
        assert_eq!(highlighted(new, &new_ranges), vec!["🎊"]);

        let old = "café résumé";
        let new = "cafe resume";
        let (old_ranges, new_ranges) = intra_line_diff(old, new);
        assert_ranges_valid(old, &old_ranges);
        assert_ranges_valid(new, &new_ranges);
        // Whole differing word tokens.
        assert!(!old_ranges.is_empty() || !new_ranges.is_empty());
        assert_intra_line_invariants(old, new);

        let old = "emoji 👨👩";
        let new = "emoji 👨👩👧";
        assert_intra_line_invariants(old, new);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        #[test]
        fn prop_intra_line_diff_ranges_valid_ordered_non_overlapping(
            old_line in "\\PC{0,64}",
            new_line in "\\PC{0,64}",
        ) {
            let (old_ranges, new_ranges) = intra_line_diff(&old_line, &new_line);
            assert_ranges_valid(&old_line, &old_ranges);
            assert_ranges_valid(&new_line, &new_ranges);

            if old_line == new_line {
                prop_assert!(old_ranges.is_empty());
                prop_assert!(new_ranges.is_empty());
            }

            // Ranges on each side must be strictly non-overlapping (end of one
            // <= start of next) and sorted by start.
            for window in old_ranges.windows(2) {
                prop_assert!(window[0].start <= window[1].start);
                prop_assert!(window[0].end <= window[1].start);
            }
            for window in new_ranges.windows(2) {
                prop_assert!(window[0].start <= window[1].start);
                prop_assert!(window[0].end <= window[1].start);
            }
        }
    }

    #[test]
    fn test_unified_diff_with_offsets() {
        let old_text = "foo\nbar\nbaz\n";
        let new_text = "foo\nBAR\nbaz\n";

        let expected_diff_body = " foo\n-bar\n+BAR\n baz\n";

        let diff_no_offset = unified_diff(old_text, new_text);
        assert_eq!(
            diff_no_offset,
            format!("@@ -1,3 +1,3 @@\n{}", expected_diff_body)
        );

        let diff_with_offset = unified_diff_with_offsets(old_text, new_text, 9, 11);
        assert_eq!(
            diff_with_offset,
            format!("@@ -10,3 +12,3 @@\n{}", expected_diff_body)
        );

        let diff_with_offset = unified_diff_with_offsets(old_text, new_text, 99, 104);
        assert_eq!(
            diff_with_offset,
            format!("@@ -100,3 +105,3 @@\n{}", expected_diff_body)
        );
    }

    #[test]
    fn test_unified_diff_with_context() {
        // Test that full context includes all lines from the start
        let old_text = "line1\nline2\nline3\nline4\nline5\nCHANGE_ME\nline7\nline8\n";
        let new_text = "line1\nline2\nline3\nline4\nline5\nCHANGED\nline7\nline8\n";

        // With default 3 lines of context, the diff starts at line 3
        let diff_default = unified_diff_with_offsets(old_text, new_text, 0, 0);
        assert_eq!(
            diff_default,
            "@@ -3,6 +3,6 @@\n line3\n line4\n line5\n-CHANGE_ME\n+CHANGED\n line7\n line8\n"
        );

        // With full context (8 lines), the diff starts at line 1
        let diff_full_context = unified_diff_with_context(old_text, new_text, 0, 0, 8);
        assert_eq!(
            diff_full_context,
            "@@ -1,8 +1,8 @@\n line1\n line2\n line3\n line4\n line5\n-CHANGE_ME\n+CHANGED\n line7\n line8\n"
        );

        // With 0 context, only the changed line is shown
        let diff_no_context = unified_diff_with_context(old_text, new_text, 0, 0, 0);
        assert_eq!(diff_no_context, "@@ -6,1 +6,1 @@\n-CHANGE_ME\n+CHANGED\n");
    }
}
