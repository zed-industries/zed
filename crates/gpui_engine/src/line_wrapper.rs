use crate::{FontId, TextRun, TextSystem};
use gpui_shared_string::SharedString;
use gpui_types::{Pixels, px};
use std::{
    borrow::Cow,
    collections::HashMap,
    iter,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Determines whether to truncate text from the start or end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruncateFrom {
    /// Truncate text from the start.
    Start,
    /// Truncate text from the end.
    End,
    /// Truncate text from the middle, preserving the start and end.
    Middle,
}

/// The GPUI line wrapper, used to wrap lines of text to a given width.
pub struct LineWrapper {
    text_system: Arc<dyn TextSystem>,
    pub(crate) font_id: FontId,
    pub(crate) font_size: Pixels,
    cached_ascii_char_widths: [Option<Pixels>; 128],
    cached_other_char_widths: HashMap<char, Pixels>,
}

impl LineWrapper {
    /// The maximum indent that can be applied to a line.
    pub const MAX_INDENT: u32 = 256;

    /// Creates a wrapper that measures and truncates text through `text_system`.
    pub fn new(font_id: FontId, font_size: Pixels, text_system: Arc<dyn TextSystem>) -> Self {
        Self {
            text_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [None; 128],
            cached_other_char_widths: HashMap::default(),
        }
    }

    /// Wrap a line of text to the given width with this wrapper's font and font size.
    pub fn wrap_line<'a>(
        &'a mut self,
        fragments: &'a [LineFragment],
        wrap_width: Pixels,
    ) -> impl Iterator<Item = Boundary> + 'a {
        let mut width = px(0.);
        let mut first_non_whitespace_ix = None;
        let mut indent = None;
        let mut last_candidate_ix = 0;
        let mut last_candidate_width = px(0.);
        let mut last_wrap_ix = 0;
        let mut prev_c = '\0';
        let mut index = 0;
        let mut candidates = fragments
            .iter()
            .flat_map(move |fragment| fragment.wrap_boundary_candidates())
            .peekable();
        iter::from_fn(move || {
            for candidate in candidates.by_ref() {
                let ix = index;
                index += candidate.len_utf8();
                let mut new_prev_c = prev_c;
                let item_width = match candidate {
                    WrapBoundaryCandidate::Char { character: c } => {
                        if c == '\n' {
                            continue;
                        }

                        if Self::is_word_char(c) {
                            if prev_c == ' ' && c != ' ' && first_non_whitespace_ix.is_some() {
                                last_candidate_ix = ix;
                                last_candidate_width = width;
                            }
                        } else {
                            // CJK may not be space separated, e.g.: `Hello world你好世界`
                            if c != ' ' && first_non_whitespace_ix.is_some() {
                                last_candidate_ix = ix;
                                last_candidate_width = width;
                            }
                        }

                        if c != ' ' && first_non_whitespace_ix.is_none() {
                            first_non_whitespace_ix = Some(ix);
                        }

                        new_prev_c = c;

                        self.width_for_char(c)
                    }
                    WrapBoundaryCandidate::Element {
                        width: element_width,
                        ..
                    } => {
                        if prev_c == ' ' && first_non_whitespace_ix.is_some() {
                            last_candidate_ix = ix;
                            last_candidate_width = width;
                        }

                        if first_non_whitespace_ix.is_none() {
                            first_non_whitespace_ix = Some(ix);
                        }

                        element_width
                    }
                };

                width += item_width;
                if width > wrap_width && ix > last_wrap_ix {
                    if let (None, Some(first_non_whitespace_ix)) = (indent, first_non_whitespace_ix)
                    {
                        indent = Some(
                            Self::MAX_INDENT.min((first_non_whitespace_ix - last_wrap_ix) as u32),
                        );
                    }

                    if last_candidate_ix > 0 {
                        last_wrap_ix = last_candidate_ix;
                        width -= last_candidate_width;
                        last_candidate_ix = 0;
                    } else {
                        last_wrap_ix = ix;
                        width = item_width;
                    }

                    if let Some(indent) = indent {
                        width += self.width_for_char(' ') * indent as f32;
                    }

                    return Some(Boundary::new(last_wrap_ix, indent.unwrap_or(0)));
                }

                prev_c = new_prev_c;
            }

            None
        })
    }

    /// Determines if a line should be truncated based on its width.
    ///
    /// Returns the truncation index in `line`.
    pub fn should_truncate_line(
        &mut self,
        line: &str,
        truncate_width: Pixels,
        truncation_affix: &str,
        truncate_from: TruncateFrom,
    ) -> Option<usize> {
        let mut width = px(0.);
        let suffix_width = truncation_affix
            .chars()
            .map(|c| self.width_for_char(c))
            .fold(px(0.0), |a, x| a + x);
        let mut truncate_ix = 0;

        match truncate_from {
            TruncateFrom::Start => {
                for (ix, c) in line.char_indices().rev() {
                    if width + suffix_width < truncate_width {
                        truncate_ix = ix;
                    }

                    let char_width = self.width_for_char(c);
                    width += char_width;

                    if width.floor() > truncate_width {
                        return Some(truncate_ix);
                    }
                }
            }
            TruncateFrom::End => {
                for (ix, c) in line.char_indices() {
                    if width + suffix_width < truncate_width {
                        truncate_ix = ix;
                    }

                    let char_width = self.width_for_char(c);
                    width += char_width;

                    if width.floor() > truncate_width {
                        return Some(truncate_ix);
                    }
                }
            }
            TruncateFrom::Middle => {}
        }

        None
    }

    fn should_truncate_line_middle(
        &mut self,
        line: &str,
        truncate_width: Pixels,
        truncation_affix: &str,
    ) -> Option<(usize, usize)> {
        let suffix_width = truncation_affix
            .chars()
            .map(|c| self.width_for_char(c))
            .fold(px(0.0), |a, x| a + x);

        let total_width: Pixels = line
            .chars()
            .map(|c| self.width_for_char(c))
            .fold(px(0.0), |a, x| a + x);

        if total_width <= truncate_width {
            return None;
        }

        let content_budget = truncate_width - suffix_width;
        if content_budget <= px(0.) {
            return Some((0, line.len()));
        }

        let front_budget = content_budget * (2.0 / 3.0);
        let back_budget = content_budget - front_budget;

        let mut front_width = px(0.);
        let mut front_end_ix = 0usize;
        for (ix, c) in line.char_indices() {
            let char_width = self.width_for_char(c);
            if front_width + char_width > front_budget {
                break;
            }
            front_width += char_width;
            front_end_ix = ix + c.len_utf8();
        }

        let mut back_width = px(0.);
        let mut back_start_ix = line.len();
        for (ix, c) in line.char_indices().rev() {
            let char_width = self.width_for_char(c);
            if back_width + char_width > back_budget {
                break;
            }
            back_width += char_width;
            back_start_ix = ix;
        }

        if front_end_ix >= back_start_ix {
            return Some((0, line.len()));
        }

        Some((front_end_ix, back_start_ix))
    }

    /// Truncate a line of text to the given width with this wrapper's font and font size.
    pub fn truncate_line<'a>(
        &mut self,
        line: SharedString,
        truncate_width: Pixels,
        truncation_affix: &str,
        runs: &'a [TextRun],
        truncate_from: TruncateFrom,
    ) -> (SharedString, Cow<'a, [TextRun]>) {
        if truncate_from == TruncateFrom::Middle {
            if let Some((front_end_ix, back_start_ix)) =
                self.should_truncate_line_middle(&line, truncate_width, truncation_affix)
            {
                let result = SharedString::from(format!(
                    "{}{truncation_affix}{}",
                    &line[..front_end_ix],
                    &line[back_start_ix..]
                ));
                let mut runs = runs.to_vec();
                update_runs_after_middle_truncation(
                    truncation_affix,
                    &mut runs,
                    front_end_ix,
                    back_start_ix,
                );
                return (result, Cow::Owned(runs));
            } else {
                return (line, Cow::Borrowed(runs));
            }
        }

        if let Some(truncate_ix) =
            self.should_truncate_line(&line, truncate_width, truncation_affix, truncate_from)
        {
            let result = match truncate_from {
                TruncateFrom::Start => SharedString::from(format!(
                    "{truncation_affix}{}",
                    &line[line.ceil_char_boundary(truncate_ix + 1)..]
                )),
                TruncateFrom::End => SharedString::from(format!(
                    "{}{truncation_affix}",
                    line[..truncate_ix]
                        .trim_end_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
                )),
                TruncateFrom::Middle => unreachable!("Middle truncation is handled above"),
            };
            let mut runs = runs.to_vec();
            update_runs_after_truncation(&result, truncation_affix, &mut runs, truncate_from);
            (result, Cow::Owned(runs))
        } else {
            (line, Cow::Borrowed(runs))
        }
    }

    /// Truncate text to fit within a given number of wrapped lines.
    ///
    /// Unlike `truncate_line` which treats the text as a flat width budget
    /// (`width * max_lines`), this method accounts for word-boundary wrapping:
    /// it walks through characters once, tracking wrap boundaries and the
    /// truncation point simultaneously. When text overflows on the last
    /// allowed line, it truncates there and appends the affix.
    ///
    /// For `max_lines == 1`, this delegates to `truncate_line`.
    pub fn truncate_wrapped_line<'a>(
        &mut self,
        text: SharedString,
        wrap_width: Pixels,
        max_lines: usize,
        truncation_affix: &str,
        runs: &'a [TextRun],
        truncate_from: TruncateFrom,
    ) -> (SharedString, Cow<'a, [TextRun]>) {
        if max_lines <= 1 || truncate_from == TruncateFrom::Start {
            return self.truncate_line(
                text,
                wrap_width * max_lines,
                truncation_affix,
                runs,
                truncate_from,
            );
        }
        if truncate_from == TruncateFrom::Middle {
            return self.truncate_line(text, wrap_width, truncation_affix, runs, truncate_from);
        }

        let affix_width: Pixels = truncation_affix
            .chars()
            .map(|c| self.width_for_char(c))
            .sum();

        let mut width = px(0.);
        let mut line = 0usize;
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = 0usize;
        let mut last_candidate_width = px(0.);
        let mut last_wrap_ix = 0usize;
        let mut prev_c = '\0';
        let mut indent: Option<u32> = None;
        let mut truncate_ix = 0usize;

        for (ix, c) in text.char_indices() {
            if c == '\n' {
                if line >= max_lines - 1 && !text[ix + 1..].trim().is_empty() {
                    // Newline on the last allowed line with real content
                    // below. Truncate here.
                    let truncated = text[..truncate_ix]
                        .trim_end_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation());
                    let result = SharedString::from(format!("{truncated}{truncation_affix}"));
                    let mut runs = runs.to_vec();
                    update_runs_after_truncation(
                        &result,
                        truncation_affix,
                        &mut runs,
                        TruncateFrom::End,
                    );
                    return (result, Cow::Owned(runs));
                }

                // Newline before the last line: it consumes a line.
                line += 1;
                width = px(0.);
                first_non_whitespace_ix = None;
                last_candidate_ix = 0;
                last_candidate_width = px(0.);
                last_wrap_ix = ix + 1;
                prev_c = '\0';
                indent = None;
                truncate_ix = ix + 1;
                continue;
            }

            let char_width = self.width_for_char(c);

            if Self::is_word_char(c) {
                if prev_c == ' ' && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = ix;
                    last_candidate_width = width;
                }
            } else if c != ' ' && first_non_whitespace_ix.is_some() {
                last_candidate_ix = ix;
                last_candidate_width = width;
            }

            if c != ' ' && first_non_whitespace_ix.is_none() {
                first_non_whitespace_ix = Some(ix);
            }

            width += char_width;

            if line < max_lines - 1 {
                // Before the last line: replicate wrap_line's boundary logic.
                if width > wrap_width && ix > last_wrap_ix {
                    if let (None, Some(first_nw)) = (indent, first_non_whitespace_ix) {
                        indent = Some(Self::MAX_INDENT.min((first_nw - last_wrap_ix) as u32));
                    }

                    if last_candidate_ix > last_wrap_ix {
                        last_wrap_ix = last_candidate_ix;
                        width -= last_candidate_width;
                        last_candidate_ix = 0;
                    } else {
                        last_wrap_ix = ix;
                        width = char_width;
                    }

                    if let Some(ind) = indent {
                        width += self.width_for_char(' ') * ind as f32;
                    }

                    line += 1;
                    truncate_ix = last_wrap_ix;
                }
            } else {
                // On the last line: track the furthest point where the affix
                // still fits, and stop as soon as the line overflows.
                if width + affix_width <= wrap_width {
                    truncate_ix = ix + c.len_utf8();
                }

                if width > wrap_width {
                    let truncated = text[..truncate_ix]
                        .trim_end_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation());
                    let result = SharedString::from(format!("{truncated}{truncation_affix}"));
                    let mut runs = runs.to_vec();
                    update_runs_after_truncation(
                        &result,
                        truncation_affix,
                        &mut runs,
                        TruncateFrom::End,
                    );
                    return (result, Cow::Owned(runs));
                }
            }

            prev_c = c;
        }

        // Text fits within max_lines without truncation.
        (text, Cow::Borrowed(runs))
    }

    /// Any character in this list should be treated as a word character,
    /// meaning it can be part of a word that should not be wrapped.
    /// Returns whether `c` may be part of a word that should not be wrapped.
    pub fn is_word_char(c: char) -> bool {
        // ASCII alphanumeric characters, for English, numbers: `Hello123`, etc.
        c.is_ascii_alphanumeric() ||
        // Latin script in Unicode for French, German, Spanish, etc.
        // Latin-1 Supplement
        // https://en.wikipedia.org/wiki/Latin-1_Supplement
        matches!(c, '\u{00C0}'..='\u{00FF}') ||
        // Latin Extended-A
        // https://en.wikipedia.org/wiki/Latin_Extended-A
        matches!(c, '\u{0100}'..='\u{017F}') ||
        // Latin Extended-B
        // https://en.wikipedia.org/wiki/Latin_Extended-B
        matches!(c, '\u{0180}'..='\u{024F}') ||
        // Cyrillic for Russian, Ukrainian, etc.
        // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unicode
        matches!(c, '\u{0400}'..='\u{04FF}') ||

        // Vietnamese (https://vietunicode.sourceforge.net/charset/)
        matches!(c, '\u{1E00}'..='\u{1EFF}') || // Latin Extended Additional
        matches!(c, '\u{0300}'..='\u{036F}') || // Combining Diacritical Marks

        // Bengali (https://en.wikipedia.org/wiki/Bengali_(Unicode_block))
        matches!(c, '\u{0980}'..='\u{09FF}') ||

        // Some other known special characters that should be treated as word characters,
        // e.g. `a-b`, `var_name`, `I'm`/`won’t`, '@mention`, `#hashtag`, `100%`, `3.1415`,
        // `2^3`, `a~b`, `a=1`, `Self::new`, etc. Trailing punctuation like `,`, `.`, `:`, `;`
        // is included so it stays attached to the preceding word when wrapping.
        matches!(c, '-' | '_' | '.' | '\'' | '’' | '‘' | '$' | '%' | '@' | '#' | '^' | '~' | ',' | '=' | ':' | ';') ||
        // Closing punctuation never starts a line (UAX #14 LB13: no break
        // before `!`, `)`, `]`, `}`, closing quotes or an ellipsis) — `plz!`,
        // `see)`, `quoted”` wrap as one word instead of orphaning the mark on
        // the next line. `/` and `?` stay break opportunities so long paths
        // and URLs (`a/b`, `foo?b=2`) can wrap.
        matches!(c, '!' | ')' | ']' | '}' | '"' | '”' | '»' | '…') ||
        // `⋯` character is special used in Zed, to keep this at the end of the line.
        matches!(c, '⋯') ||

        // Non-breaking glue characters
        matches!(c, '\u{202F}' | '\u{00A0}' | '\u{2011}')
    }

    /// The advance width of `c` in this wrapper's font, cached after first use.
    #[inline(always)]
    pub fn width_for_char(&mut self, c: char) -> Pixels {
        if (c as u32) < 128 {
            if let Some(cached_width) = self.cached_ascii_char_widths[c as usize] {
                cached_width
            } else {
                let width = self
                    .text_system
                    .layout_width(self.font_id, self.font_size, c);
                self.cached_ascii_char_widths[c as usize] = Some(width);
                width
            }
        } else if let Some(cached_width) = self.cached_other_char_widths.get(&c) {
            *cached_width
        } else {
            let width = self
                .text_system
                .layout_width(self.font_id, self.font_size, c);
            self.cached_other_char_widths.insert(c, width);
            width
        }
    }
}

/// Trims `runs` to match `result` after `truncate_line` replaced the removed
/// text with `ellipsis`.
pub fn update_runs_after_truncation(
    result: &str,
    ellipsis: &str,
    runs: &mut Vec<TextRun>,
    truncate_from: TruncateFrom,
) {
    let mut truncate_at = result.len() - ellipsis.len();
    match truncate_from {
        TruncateFrom::Start => {
            for (run_index, run) in runs.iter_mut().enumerate().rev() {
                if run.len <= truncate_at {
                    truncate_at -= run.len;
                } else {
                    run.len = truncate_at + ellipsis.len();
                    runs.splice(..run_index, std::iter::empty());
                    break;
                }
            }
        }
        TruncateFrom::End => {
            for (run_index, run) in runs.iter_mut().enumerate() {
                if run.len <= truncate_at {
                    truncate_at -= run.len;
                } else {
                    run.len = truncate_at + ellipsis.len();
                    runs.truncate(run_index + 1);
                    break;
                }
            }
        }
        TruncateFrom::Middle => {
            unreachable!("Middle truncation calls this function with TruncateFrom::End directly")
        }
    }
}

fn update_runs_after_middle_truncation(
    ellipsis: &str,
    runs: &mut Vec<TextRun>,
    front_end_ix: usize,
    back_start_ix: usize,
) {
    let original_runs = std::mem::take(runs);
    let mut result_runs: Vec<TextRun> = Vec::with_capacity(original_runs.len());

    // Front segment [0, front_end_ix) + ellipsis: walk forward until the run
    // that straddles or ends at front_end_ix, then extend that run's length
    // to include the ellipsis.
    let mut front_remaining = front_end_ix;
    let mut front_done = false;
    for run in &original_runs {
        if front_done {
            break;
        }
        if run.len <= front_remaining {
            result_runs.push(run.clone());
            front_remaining -= run.len;
        } else {
            let mut partial = run.clone();
            partial.len = front_remaining + ellipsis.len();
            result_runs.push(partial);
            front_done = true;
        }
    }
    if !front_done {
        // front_end_ix landed exactly on a run boundary; append ellipsis to
        // the last front run (or, if the front is empty, to the first back run).
        if let Some(last) = result_runs.last_mut() {
            last.len += ellipsis.len();
        } else if let Some(first) = original_runs.first() {
            let mut affix_run = first.clone();
            affix_run.len = ellipsis.len();
            result_runs.push(affix_run);
        }
    }

    // Back segment [back_start_ix, original.len()): skip runs entirely in the
    // removed middle, keep the rest.
    let mut byte_pos = 0usize;
    for run in &original_runs {
        let run_end = byte_pos + run.len;
        if run_end > back_start_ix {
            if byte_pos < back_start_ix {
                // Run straddles back_start_ix; keep only the tail.
                let mut partial = run.clone();
                partial.len = run_end - back_start_ix;
                result_runs.push(partial);
            } else {
                result_runs.push(run.clone());
            }
        }
        byte_pos = run_end;
    }

    *runs = result_runs;
}

/// A fragment of a line that can be wrapped.
pub enum LineFragment<'a> {
    /// A text fragment consisting of characters.
    Text {
        /// The text content of the fragment.
        text: &'a str,
    },
    /// A non-text element with a fixed width.
    Element {
        /// The width of the element in pixels.
        width: Pixels,
        /// The UTF-8 encoded length of the element.
        len_utf8: usize,
    },
}

impl<'a> LineFragment<'a> {
    /// Creates a new text fragment from the given text.
    pub fn text(text: &'a str) -> Self {
        LineFragment::Text { text }
    }

    /// Creates a new non-text element with the given width and UTF-8 encoded length.
    pub fn element(width: Pixels, len_utf8: usize) -> Self {
        LineFragment::Element { width, len_utf8 }
    }

    fn wrap_boundary_candidates(&self) -> impl Iterator<Item = WrapBoundaryCandidate> {
        let text = match self {
            LineFragment::Text { text } => text,
            LineFragment::Element { .. } => "\0",
        };
        text.chars().map(move |character| {
            if let LineFragment::Element { width, len_utf8 } = self {
                WrapBoundaryCandidate::Element {
                    width: *width,
                    len_utf8: *len_utf8,
                }
            } else {
                WrapBoundaryCandidate::Char { character }
            }
        })
    }
}

enum WrapBoundaryCandidate {
    Char { character: char },
    Element { width: Pixels, len_utf8: usize },
}

impl WrapBoundaryCandidate {
    pub fn len_utf8(&self) -> usize {
        match self {
            WrapBoundaryCandidate::Char { character } => character.len_utf8(),
            WrapBoundaryCandidate::Element { len_utf8: len, .. } => *len,
        }
    }
}

/// A boundary between two lines of text.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boundary {
    /// The index of the last character in a line
    pub ix: usize,
    /// The indent of the next line.
    pub next_indent: u32,
}

impl Boundary {
    /// Creates a boundary ending at byte index `ix`, with `next_indent` spaces.
    pub fn new(ix: usize, next_indent: u32) -> Self {
        Self { ix, next_indent }
    }
}

/// A line wrapper borrowed from a [`TextSystem`] pool, returned on drop.
pub struct LineWrapperHandle {
    wrapper: Option<LineWrapper>,
    recycle: Option<Box<dyn FnOnce(LineWrapper) + Send + Sync>>,
}

impl LineWrapperHandle {
    /// Wraps `wrapper`, passing it back through `recycle` when dropped.
    pub fn new(
        wrapper: LineWrapper,
        recycle: impl FnOnce(LineWrapper) + Send + Sync + 'static,
    ) -> Self {
        Self {
            wrapper: Some(wrapper),
            recycle: Some(Box::new(recycle)),
        }
    }
}

impl Deref for LineWrapperHandle {
    type Target = LineWrapper;

    fn deref(&self) -> &Self::Target {
        self.wrapper.as_ref().expect("wrapper present until drop")
    }
}

impl DerefMut for LineWrapperHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.wrapper.as_mut().expect("wrapper present until drop")
    }
}

impl Drop for LineWrapperHandle {
    fn drop(&mut self) {
        let wrapper = self.wrapper.take().expect("wrapper present until drop");
        let recycle = self.recycle.take().expect("recycle present until drop");
        recycle(wrapper);
    }
}
