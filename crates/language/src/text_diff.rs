use crate::{CharClassifier, CharKind, CharScopeContext, LanguageScope};
use anyhow::{Context, anyhow};
use imara_diff::{Algorithm, Diff, InternedInput, Interner, Token, sources::lines};
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
    // The builder appends its own line terminators, so tokenize without them.
    let mut input = InternedInput::default();
    input.update_before(old_text.lines());
    input.update_after(new_text.lines());
    let diff = Diff::compute(Algorithm::Histogram, &input);
    let mut builder =
        OffsetUnifiedDiffBuilder::new(&input, old_start_line, new_start_line, context_lines);
    for hunk in diff.hunks() {
        builder.process_change(hunk.before, hunk.after);
    }
    builder.finish()
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
        write!(&mut self.dst, "{}", self.buffer).unwrap();
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

impl OffsetUnifiedDiffBuilder<'_> {
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

    fn finish(mut self) -> String {
        self.flush();
        self.dst
    }
}

/// Computes a diff between two strings, returning a vector of old and new row
/// ranges.
pub fn line_diff(old_text: &str, new_text: &str) -> Vec<(Range<u32>, Range<u32>)> {
    let mut edits = Vec::new();
    let input = InternedInput::new(lines(old_text), lines(new_text));
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
/// Lines are first paired by similarity, preserving their order, and words are
/// only diffed within paired lines. Lines without a similar counterpart get no
/// ranges, so they read as whole-line additions or deletions.
///
/// Returns a tuple of (old_ranges, new_ranges) where each vector contains
/// the byte ranges of changed words in the respective text.
pub fn word_diff_ranges(
    old_text: &str,
    new_text: &str,
    options: DiffOptions,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut interner = Interner::default();
    let old_lines = tokenize_lines(old_text, &options.language_scope, &mut interner);
    let new_lines = tokenize_lines(new_text, &options.language_scope, &mut interner);
    let mut diff = Diff::default();
    let mut compare_lines =
        |old_line, new_line| LineDiff::new(old_line, new_line, &mut diff, interner.num_tokens());

    let mut diagonal = Vec::new();
    if old_lines.len() == new_lines.len() {
        diagonal.extend(
            old_lines
                .iter()
                .zip(&new_lines)
                .map(|(old_line, new_line)| compare_lines(old_line, new_line)),
        );

        // Any different alignment would leave at least one line unpaired on each
        // side, at a cost of 2. Let the full search handle ties.
        if diagonal.iter().all(Option::is_some)
            && diagonal
                .iter()
                .flatten()
                .map(|diff| diff.distance)
                .sum::<f32>()
                < 2.0
        {
            return collect_word_diff_ranges(diagonal.into_iter().flatten());
        }
    }

    let mut line_diffs: Vec<Vec<Option<LineDiff>>> = old_lines
        .iter()
        .enumerate()
        .map(|(old_index, old_line)| {
            new_lines
                .iter()
                .enumerate()
                .map(|(new_index, new_line)| {
                    if old_index == new_index
                        && let Some(diff) = diagonal.get_mut(old_index)
                    {
                        diff.take()
                    } else {
                        compare_lines(old_line, new_line)
                    }
                })
                .collect()
        })
        .collect();

    let paired_diffs = align_lines(&line_diffs)
        .into_iter()
        .filter_map(|(old_index, new_index)| line_diffs[old_index][new_index].take());
    collect_word_diff_ranges(paired_diffs)
}

fn collect_word_diff_ranges(
    line_diffs: impl Iterator<Item = LineDiff>,
) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
    let mut old_ranges = Vec::new();
    let mut new_ranges = Vec::new();
    for line_diff in line_diffs {
        old_ranges.extend(line_diff.old_ranges);
        new_ranges.extend(line_diff.new_ranges);
    }
    (old_ranges, new_ranges)
}

/// Lines further apart than this are treated as unrelated rather than modified.
const MAX_LINE_DISTANCE: f32 = 0.6;

/// Word-level diff of a single pair of lines.
struct LineDiff {
    /// Fraction of bytes, across both lines, that changed.
    distance: f32,
    old_ranges: Vec<Range<usize>>,
    new_ranges: Vec<Range<usize>>,
}

impl LineDiff {
    fn new(
        old_line: &TokenizedLine,
        new_line: &TokenizedLine,
        diff: &mut Diff,
        token_count: u32,
    ) -> Option<Self> {
        diff.compute_with(
            Algorithm::Histogram,
            &old_line.tokens,
            &new_line.tokens,
            token_count,
        );

        let mut old_ranges = Vec::new();
        let mut new_ranges = Vec::new();
        let mut changed_len = 0;
        for hunk in diff.hunks() {
            if !hunk.before.is_empty() {
                let range = old_line.byte_range(hunk.before);
                changed_len += range.len();
                old_ranges.push(range);
            }
            if !hunk.after.is_empty() {
                let range = new_line.byte_range(hunk.after);
                changed_len += range.len();
                new_ranges.push(range);
            }
        }

        let total_len = old_line.byte_len + new_line.byte_len;
        let distance = changed_len as f32 / total_len.max(1) as f32;
        (distance <= MAX_LINE_DISTANCE).then_some(Self {
            distance,
            old_ranges,
            new_ranges,
        })
    }
}

struct TokenizedLine {
    tokens: Vec<Token>,
    offsets: Vec<usize>,
    byte_len: usize,
}

impl TokenizedLine {
    fn byte_range(&self, tokens: Range<u32>) -> Range<usize> {
        self.offsets[tokens.start as usize]..self.offsets[tokens.end as usize]
    }
}

fn tokenize_lines<'a>(
    text: &'a str,
    language_scope: &Option<LanguageScope>,
    interner: &mut Interner<&'a str>,
) -> Vec<TokenizedLine> {
    let mut offset = 0;
    text.split_inclusive('\n')
        .map(|line| {
            let text = line.trim_end_matches('\n');
            let mut tokens = Vec::new();
            let mut offsets = vec![offset];
            for token in tokenize(text, language_scope.clone()).flat_map(split_whitespace) {
                tokens.push(interner.intern(token));
                offset += token.len();
                offsets.push(offset);
            }
            offset += line.len() - text.len();
            TokenizedLine {
                tokens,
                offsets,
                byte_len: text.len(),
            }
        })
        .collect()
}

/// Splits a whitespace token into single characters, so re-indenting a line
/// highlights only the added or removed whitespace, not the whole indent.
fn split_whitespace(token: &str) -> impl Iterator<Item = &str> {
    let is_whitespace = token.starts_with(char::is_whitespace);
    token.split_inclusive(move |_: char| is_whitespace)
}

/// Pairs old and new lines in order, minimizing the total cost where an
/// unpaired line costs 1 and a pair costs its distance. Only pairable lines,
/// those with a `LineDiff`, are considered. Returns `(old_ix, new_ix)` pairs.
fn align_lines(line_diffs: &[Vec<Option<LineDiff>>]) -> Vec<(usize, usize)> {
    #[derive(Clone, Copy)]
    enum Step {
        Delete,
        Insert,
        Pair,
    }

    let old_len = line_diffs.len();
    let new_len = line_diffs.first().map_or(0, Vec::len);
    let mut costs = vec![vec![0.0_f32; new_len + 1]; old_len + 1];
    let mut steps = vec![vec![Step::Delete; new_len + 1]; old_len + 1];
    for old_ix in 0..=old_len {
        for new_ix in 0..=new_len {
            let mut best = (f32::INFINITY, Step::Delete);
            if old_ix > 0 {
                best = (costs[old_ix - 1][new_ix] + 1.0, Step::Delete);
            }
            if new_ix > 0 && costs[old_ix][new_ix - 1] + 1.0 < best.0 {
                best = (costs[old_ix][new_ix - 1] + 1.0, Step::Insert);
            }
            if old_ix > 0
                && new_ix > 0
                && let Some(line_diff) = &line_diffs[old_ix - 1][new_ix - 1]
                && costs[old_ix - 1][new_ix - 1] + line_diff.distance < best.0
            {
                best = (
                    costs[old_ix - 1][new_ix - 1] + line_diff.distance,
                    Step::Pair,
                );
            }
            if old_ix > 0 || new_ix > 0 {
                (costs[old_ix][new_ix], steps[old_ix][new_ix]) = best;
            }
        }
    }

    let mut pairs = Vec::new();
    let (mut old_ix, mut new_ix) = (old_len, new_len);
    while old_ix > 0 || new_ix > 0 {
        match steps[old_ix][new_ix] {
            Step::Delete => old_ix -= 1,
            Step::Insert => new_ix -= 1,
            Step::Pair => {
                old_ix -= 1;
                new_ix -= 1;
                pairs.push((old_ix, new_ix));
            }
        }
    }
    pairs.reverse();
    pairs
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
    let input = InternedInput::new(lines(old_text), lines(new_text));
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
    let diff = Diff::compute(Algorithm::Histogram, input);
    for hunk in diff.hunks() {
        let old_tokens = hunk.before;
        let new_tokens = hunk.after;
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
    }
}

fn tokenize_chars(text: &str) -> impl Iterator<Item = &str> {
    let mut chars = text.char_indices();
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

    fn word_diffs<'a>(old_text: &'a str, new_text: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
        let (old_ranges, new_ranges) = word_diff_ranges(old_text, new_text, DiffOptions::default());
        (
            old_ranges
                .into_iter()
                .map(|range| &old_text[range])
                .collect(),
            new_ranges
                .into_iter()
                .map(|range| &new_text[range])
                .collect(),
        )
    }

    #[test]
    fn test_word_diff_ranges() {
        assert_eq!(
            word_diffs("foo(a, b)\n", "foo(a, c)\n"),
            (vec!["b"], vec!["c"])
        );

        // Similar lines are paired across inserted and deleted lines.
        assert_eq!(
            word_diffs(
                "let x = 1;\nremoved();\nlet y = 2;\n",
                "fn new() {}\nlet x = 10;\nlet y = 20;\n"
            ),
            (vec!["1", "2"], vec!["10", "20"])
        );

        // Unrelated lines are not paired, so they get no word ranges.
        assert_eq!(
            word_diffs("first_function();\n", "let unrelated = 42;\n"),
            (vec![], vec![])
        );

        // Pairing picks the most similar line rather than the first one.
        assert_eq!(
            word_diffs(
                "call(alpha, beta, gamma);\n",
                "call(alpha, delta);\ncall(alpha, beta, omega);\n"
            ),
            (vec!["gamma"], vec!["omega"])
        );

        // At cost 2, pairing by position ties with leaving a gap on each side.
        assert_eq!(
            word_diffs("x aa\nx bb\nx cc\nx dd\n", "x bb\nx cc\nx dd\nx ee\n"),
            (vec![], vec![])
        );

        // Long lines are still diffed.
        let prefix = "a".repeat(600);
        assert_eq!(
            word_diffs(&format!("{prefix} = 1;\n"), &format!("{prefix} = 2;\n")),
            (vec!["1"], vec!["2"])
        );

        // Multibyte lines are paired across an inserted line.
        assert_eq!(
            word_diffs("hello world 世界\n", "\nhello world 朋友\n"),
            (vec!["世界"], vec!["朋友"])
        );

        // The final line may lack a trailing newline.
        assert_eq!(
            word_diffs("value = 1", "inserted();\nvalue = 2"),
            (vec!["1"], vec!["2"])
        );

        assert_eq!(word_diffs("", "added\n"), (vec![], vec![]));

        // Ranges point at the paired line, not an identical earlier one.
        assert_eq!(
            word_diff_ranges(
                "x = 1;\nx = 1;\n",
                "x = 1;\nx = 2;\n",
                DiffOptions::default()
            ),
            (vec![11..12], vec![11..12])
        );

        // A partial rename highlights the whole identifier.
        assert_eq!(
            word_diffs("reader.items_old()\n", "reader.items()\n"),
            (vec!["items_old"], vec!["items"])
        );

        // A multibyte line pairs with its match after an unrelated line is removed.
        assert_eq!(
            word_diffs(
                "猫 猫 犬 猫 猫 犬 猫 猫 犬\n雪 雪 雨 雪 雪 雨 雪 雪 雨\n",
                "雪 雪 雨 雪 雪 霧 雪 雪 雨\n"
            ),
            (vec!["雨"], vec!["霧"])
        );

        // An inserted lifetime includes its separator in the range.
        assert_eq!(
            word_diffs("fn lookup<'a, Entry>(\n", "fn lookup<'a, 'b, Entry>(\n"),
            (vec![], vec!["'b, "])
        );

        // Wrapping an expression highlights the opening and closing tokens separately.
        assert_eq!(
            word_diffs(
                "let limit = settings[\"depth\"];\n",
                "let limit = parse(settings[\"depth\"]);\n"
            ),
            (vec![], vec!["parse(", ")"])
        );

        // Inserting repeated words between identical words produces one range.
        assert_eq!(
            word_diffs("item item\n", "item next next item\n"),
            (vec![], vec!["next next "])
        );

        // A value appended to the line includes its leading space.
        assert_eq!(
            word_diffs("    entries[index] =\n", "    entries[index] = value\n"),
            (vec![], vec![" value"])
        );

        // Removing a word from a string includes its leading space.
        assert_eq!(
            word_diffs("label(\"build metadata\");\n", "label(\"build\");\n"),
            (vec![" metadata"], vec![])
        );

        // Moving the comma outside the brackets produces a range at each position.
        assert_eq!(
            word_diff_ranges("[entry,]\n", "[entry],\n", DiffOptions::default()),
            (vec![6..7], vec![7..8])
        );

        // Of these two edits, only the shorter pair meets the similarity threshold.
        assert_eq!(
            word_diffs("x cat\nx cats\n", "x dog\nx dogs\n"),
            (vec!["cat"], vec!["dog"])
        );

        // Re-indenting a line highlights only the added indent, not the whole indent.
        assert_eq!(
            word_diffs("    from_str::<Config>(text)\n", "        from_str(text)\n"),
            (vec!["::<Config>"], vec!["    "])
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
