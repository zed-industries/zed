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
    let input = InternedInput::new(old_text, new_text);
    diff(
        Algorithm::Histogram,
        &input,
        OffsetUnifiedDiffBuilder::new(&input, old_start_line, new_start_line),
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

    buffer: String,
    dst: String,
}

impl<'a> OffsetUnifiedDiffBuilder<'a> {
    fn new(input: &'a InternedInput<&'a str>, old_line_offset: u32, new_line_offset: u32) -> Self {
        Self {
            before_hunk_start: 0,
            after_hunk_start: 0,
            before_hunk_len: 0,
            after_hunk_len: 0,
            old_line_offset,
            new_line_offset,
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

        let end = (self.pos + 3).min(self.before.len() as u32);
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
        if before.start - self.pos > 6 {
            self.flush();
        }
        if self.before_hunk_len == 0 && self.after_hunk_len == 0 {
            self.pos = before.start.saturating_sub(3);
            self.before_hunk_start = self.pos;
            self.after_hunk_start = after.start.saturating_sub(3);
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
    diff_internal(&input, |_, _, old_rows, new_rows| {
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

    let mut old_ranges: Vec<Range<usize>> = Vec::new();
    let mut new_ranges: Vec<Range<usize>> = Vec::new();

    diff_internal(&input, |old_byte_range, new_byte_range, _, _| {
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
    diff_internal(
        &input,
        |old_byte_range, new_byte_range, old_rows, new_rows| {
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
                diff_internal(&hunk_input, |old_byte_range, new_byte_range, _, _| {
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
        },
    );
    edits
}

pub fn apply_diff_patch(base_text: &str, patch: &str) -> Result<String, anyhow::Error> {
    let patch = diffy::Patch::from_str(patch).context("Failed to parse patch")?;
    let result = diffy::apply(base_text, &patch);
    result.map_err(|err| anyhow!(err))
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
    mut on_change: impl FnMut(Range<usize>, Range<usize>, Range<u32>, Range<u32>),
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
}
