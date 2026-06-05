use super::*;

// Keep quote matching local enough to avoid pairing unrelated quotes in large files,
// while still covering common multi-line string/template literals.
const QUOTE_LINE_TOLERANCE: usize = 8;

impl Editor {
    pub fn swap_brackets(&mut self, _: &SwapBrackets, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_delimiters(
            DelimiterKind::Bracket,
            DelimiterReplacement::Cycle,
            window,
            cx,
        );
    }

    pub fn remove_brackets(
        &mut self,
        _: &RemoveBrackets,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_delimiters(
            DelimiterKind::Bracket,
            DelimiterReplacement::Remove,
            window,
            cx,
        );
    }

    pub fn change_brackets_to(
        &mut self,
        action: &ChangeBracketsTo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_delimiters(
            DelimiterKind::Bracket,
            DelimiterReplacement::Pair(action.delimiter.pair()),
            window,
            cx,
        );
    }

    pub fn swap_quotes(&mut self, _: &SwapQuotes, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_delimiters(
            DelimiterKind::Quote,
            DelimiterReplacement::Cycle,
            window,
            cx,
        );
    }

    pub fn remove_quotes(&mut self, _: &RemoveQuotes, window: &mut Window, cx: &mut Context<Self>) {
        self.replace_delimiters(
            DelimiterKind::Quote,
            DelimiterReplacement::Remove,
            window,
            cx,
        );
    }

    pub fn change_quotes_to(
        &mut self,
        action: &ChangeQuotesTo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_delimiters(
            DelimiterKind::Quote,
            DelimiterReplacement::Pair(action.delimiter.pair()),
            window,
            cx,
        );
    }

    pub fn select_bracket_content(
        &mut self,
        _: &SelectBracketContent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_delimiter_content(DelimiterKind::Bracket, window, cx);
    }

    pub fn select_quote_content(
        &mut self,
        _: &SelectQuoteContent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_delimiter_content(DelimiterKind::Quote, window, cx);
    }

    fn replace_delimiters(
        &mut self,
        kind: DelimiterKind,
        replacement: DelimiterReplacement,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.read_only(cx) {
            return;
        }

        let snapshot = self.buffer.read(cx).snapshot(cx);
        let text = snapshot.text();
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));
        let matches = deduplicate_delimiter_matches(delimiter_matches(
            &text,
            &snapshot,
            selections.iter(),
            kind,
        ));
        if matches.is_empty() {
            return;
        }

        let mut edits = Vec::with_capacity(matches.len() * 2);
        for delimiter_match in &matches {
            let pair = match replacement {
                DelimiterReplacement::Cycle => {
                    let pairs = delimiter_pairs(kind, delimiter_match.language.as_deref());
                    let Some(pair) = cycle_pair(delimiter_match.pair, &pairs) else {
                        continue;
                    };
                    pair
                }
                DelimiterReplacement::Remove => DelimiterPair::same('\0'),
                DelimiterReplacement::Pair(pair) => pair,
            };

            let (open, close) = if matches!(replacement, DelimiterReplacement::Remove) {
                (String::new(), String::new())
            } else {
                (pair.open.to_string(), pair.close.to_string())
            };

            edits.push((
                MultiBufferOffset(delimiter_match.open)
                    ..MultiBufferOffset(
                        delimiter_match.open + delimiter_match.pair.open.len_utf8(),
                    ),
                open,
            ));
            edits.push((
                MultiBufferOffset(delimiter_match.close)
                    ..MultiBufferOffset(
                        delimiter_match.close + delimiter_match.pair.close.len_utf8(),
                    ),
                close,
            ));
        }

        if edits.is_empty() {
            return;
        }

        let selections = remap_delimiter_selections(&selections, &edits);
        self.transact(window, cx, |this, window, cx| {
            this.buffer.update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });
            this.change_selections(Default::default(), window, cx, |selection_map| {
                selection_map.select(selections);
            });
        });
    }

    fn select_delimiter_content(
        &mut self,
        kind: DelimiterKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let text = snapshot.text();
        let selections = self
            .selections
            .all::<MultiBufferOffset>(&self.display_snapshot(cx));
        let matches = deduplicate_overlapping_delimiter_matches(delimiter_matches(
            &text,
            &snapshot,
            selections.iter(),
            kind,
        ));
        if matches.is_empty() {
            return;
        }

        let selections = matches
            .into_iter()
            .map(|delimiter_match| {
                let content_start = delimiter_match.open + delimiter_match.pair.open.len_utf8();
                let content_end = delimiter_match.close;
                let (start, end) = if delimiter_match.selection.start.0 == content_start
                    && delimiter_match.selection.end.0 == content_end
                {
                    (
                        MultiBufferOffset(delimiter_match.open),
                        MultiBufferOffset(
                            delimiter_match.close + delimiter_match.pair.close.len_utf8(),
                        ),
                    )
                } else {
                    (
                        MultiBufferOffset(content_start),
                        MultiBufferOffset(content_end),
                    )
                };

                Selection {
                    id: delimiter_match.selection.id,
                    start,
                    end,
                    reversed: delimiter_match.selection.reversed,
                    goal: SelectionGoal::None,
                }
            })
            .collect();

        self.change_selections(Default::default(), window, cx, |selection_map| {
            selection_map.select(selections);
        });
    }
}

#[derive(Clone, Copy)]
enum DelimiterKind {
    Bracket,
    Quote,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DelimiterPair {
    open: char,
    close: char,
}

impl DelimiterPair {
    const fn new(open: char, close: char) -> Self {
        Self { open, close }
    }

    const fn same(character: char) -> Self {
        Self {
            open: character,
            close: character,
        }
    }
}

#[derive(Clone, Copy)]
enum DelimiterReplacement {
    Cycle,
    Remove,
    Pair(DelimiterPair),
}

struct DelimiterMatch {
    open: usize,
    close: usize,
    pair: DelimiterPair,
    selection: Selection<MultiBufferOffset>,
    language: Option<String>,
}

impl BracketDelimiter {
    fn pair(self) -> DelimiterPair {
        match self {
            BracketDelimiter::Parentheses => DelimiterPair::new('(', ')'),
            BracketDelimiter::Square => DelimiterPair::new('[', ']'),
            BracketDelimiter::Curly => DelimiterPair::new('{', '}'),
            BracketDelimiter::Angle => DelimiterPair::new('<', '>'),
        }
    }
}

impl QuoteDelimiter {
    fn pair(self) -> DelimiterPair {
        match self {
            QuoteDelimiter::Single => DelimiterPair::same('\''),
            QuoteDelimiter::Double => DelimiterPair::same('"'),
            QuoteDelimiter::Backtick => DelimiterPair::same('`'),
        }
    }
}

fn delimiter_matches<'a>(
    text: &str,
    snapshot: &MultiBufferSnapshot,
    selections: impl Iterator<Item = &'a Selection<MultiBufferOffset>>,
    kind: DelimiterKind,
) -> Vec<DelimiterMatch> {
    selections
        .filter_map(|selection| {
            let language = normalized_delimiter_language(snapshot, selection.start);
            let pairs = delimiter_pairs(kind, language.as_deref());
            match kind {
                DelimiterKind::Bracket => {
                    find_brackets_around_selection(text, selection.clone(), &pairs)
                }
                DelimiterKind::Quote => find_quotes_around_selection(
                    text,
                    selection.clone(),
                    &pairs,
                    QUOTE_LINE_TOLERANCE,
                ),
            }
            .map(|mut delimiter_match| {
                delimiter_match.language = language;
                delimiter_match
            })
        })
        .collect()
}

fn deduplicate_delimiter_matches(matches: Vec<DelimiterMatch>) -> Vec<DelimiterMatch> {
    let mut seen_ranges = std::collections::HashSet::new();
    matches
        .into_iter()
        .filter(|delimiter_match| seen_ranges.insert((delimiter_match.open, delimiter_match.close)))
        .collect()
}

fn deduplicate_overlapping_delimiter_matches(
    mut matches: Vec<DelimiterMatch>,
) -> Vec<DelimiterMatch> {
    matches.sort_by(|left, right| {
        left.open
            .cmp(&right.open)
            .then_with(|| right.close.cmp(&left.close))
    });

    let mut deduplicated = Vec::new();
    for delimiter_match in matches {
        if deduplicated.iter().any(|current: &DelimiterMatch| {
            delimiter_match.open >= current.open && delimiter_match.close <= current.close
        }) {
            continue;
        }
        deduplicated.push(delimiter_match);
    }
    deduplicated
}

fn find_brackets_around_selection(
    text: &str,
    selection: Selection<MultiBufferOffset>,
    pairs: &[DelimiterPair],
) -> Option<DelimiterMatch> {
    let (before_end, after_start) = selection_bounds_for_brackets(text, &selection)?;
    let (open, pair) = find_opening_bracket(text, before_end, pairs)?;
    let close = find_closing_bracket(text, after_start, pair)?;
    Some(DelimiterMatch {
        open,
        close,
        pair,
        selection,
        language: None,
    })
}

fn find_opening_bracket(
    text: &str,
    before_end: usize,
    pairs: &[DelimiterPair],
) -> Option<(usize, DelimiterPair)> {
    let mut stack = Vec::new();
    for (index, character) in text[..before_end].char_indices().rev() {
        if let Some(pair) = pairs.iter().copied().find(|pair| pair.close == character) {
            stack.push(pair.open);
        } else if let Some(pair) = pairs.iter().copied().find(|pair| pair.open == character) {
            if stack.last().copied() == Some(character) {
                stack.pop();
            } else {
                return Some((index, pair));
            }
        }
    }
    None
}

fn find_closing_bracket(
    text: &str,
    after_start: usize,
    target_pair: DelimiterPair,
) -> Option<usize> {
    let mut depth = 0;
    for (relative_index, character) in text[after_start..].char_indices() {
        if character == target_pair.open {
            depth += 1;
        } else if character == target_pair.close {
            if depth == 0 {
                return Some(after_start + relative_index);
            }
            depth -= 1;
        }
    }
    None
}

fn selection_bounds_for_brackets(
    text: &str,
    selection: &Selection<MultiBufferOffset>,
) -> Option<(usize, usize)> {
    let start = selection.start.0;
    let end = selection.end.0;
    if start > end || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
        return None;
    }

    if start == end {
        Some((
            previous_word_boundary(text, start),
            next_word_boundary(text, start),
        ))
    } else {
        Some((
            previous_word_boundary(text, start),
            next_word_boundary(text, end),
        ))
    }
}

fn previous_word_boundary(text: &str, offset: usize) -> usize {
    let mut boundary = offset;
    for (index, character) in text[..offset].char_indices().rev() {
        if !is_word_character(character) {
            break;
        }
        boundary = index;
    }
    boundary
}

fn next_word_boundary(text: &str, offset: usize) -> usize {
    for (relative_index, character) in text[offset..].char_indices() {
        if !is_word_character(character) {
            return offset + relative_index;
        }
    }
    text.len()
}

fn is_word_character(character: char) -> bool {
    character == '_' || character.is_alphanumeric()
}

fn find_quotes_around_selection(
    text: &str,
    selection: Selection<MultiBufferOffset>,
    pairs: &[DelimiterPair],
    line_tolerance: usize,
) -> Option<DelimiterMatch> {
    let start = selection.start.0;
    let end = selection.end.0;
    if start > end || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
        return None;
    }

    let search_range = line_tolerant_range(text, start, end, line_tolerance)?;
    find_quote_in_range(text, selection.clone(), pairs, search_range)
        .or_else(|| find_quote_in_range(text, selection, pairs, 0..text.len()))
}

fn find_quote_in_range(
    text: &str,
    selection: Selection<MultiBufferOffset>,
    pairs: &[DelimiterPair],
    range: Range<usize>,
) -> Option<DelimiterMatch> {
    let cursor = selection.start.0;
    let mut best_match = None;

    for pair in pairs {
        if pair.open != pair.close {
            continue;
        }

        let mut open = None;
        let mut escaped = false;
        for (relative_index, character) in text[range.clone()].char_indices() {
            let index = range.start + relative_index;
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character != pair.open {
                continue;
            }

            match open {
                Some(open_index) => {
                    if cursor > open_index && cursor < index + character.len_utf8() {
                        let candidate = DelimiterMatch {
                            open: open_index,
                            close: index,
                            pair: *pair,
                            selection: selection.clone(),
                            language: None,
                        };
                        if best_match.as_ref().is_none_or(|current: &DelimiterMatch| {
                            candidate.open >= current.open && candidate.close <= current.close
                        }) {
                            best_match = Some(candidate);
                        }
                    }
                    open = None;
                }
                None => open = Some(index),
            }
        }
    }

    best_match
}

fn line_tolerant_range(
    text: &str,
    start: usize,
    end: usize,
    line_tolerance: usize,
) -> Option<Range<usize>> {
    let selection_line_start = line_start(text, start);
    let selection_line_end = line_end(text, end);
    let mut range_start = selection_line_start;
    let mut range_end = selection_line_end;

    for _ in 0..line_tolerance {
        if range_start == 0 {
            break;
        }
        range_start = line_start(text, range_start.saturating_sub(1));
    }

    for _ in 0..line_tolerance {
        if range_end >= text.len() {
            break;
        }
        range_end = line_end(text, range_end + 1);
    }

    (text.is_char_boundary(range_start) && text.is_char_boundary(range_end))
        .then_some(range_start..range_end)
}

fn line_start(text: &str, offset: usize) -> usize {
    text[..offset].rfind('\n').map_or(0, |index| index + 1)
}

fn line_end(text: &str, offset: usize) -> usize {
    text[offset..]
        .find('\n')
        .map_or(text.len(), |index| offset + index)
}

fn remap_delimiter_selections(
    selections: &[Selection<MultiBufferOffset>],
    edits: &[(Range<MultiBufferOffset>, String)],
) -> Vec<Selection<MultiBufferOffset>> {
    let mut edits = edits.iter().collect::<Vec<_>>();
    edits.sort_by_key(|(range, _)| range.start);

    selections
        .iter()
        .map(|selection| Selection {
            id: selection.id,
            start: MultiBufferOffset(remap_delimiter_offset(selection.start.0, &edits)),
            end: MultiBufferOffset(remap_delimiter_offset(selection.end.0, &edits)),
            reversed: selection.reversed,
            goal: SelectionGoal::None,
        })
        .collect()
}

fn remap_delimiter_offset(offset: usize, edits: &[&(Range<MultiBufferOffset>, String)]) -> usize {
    let mut delta = 0isize;
    for (range, new_text) in edits {
        let edit_start = range.start.0;
        let edit_end = range.end.0;
        if offset < edit_start {
            break;
        }

        if offset == edit_start {
            return offset.saturating_add_signed(delta);
        }

        if offset < edit_end {
            return edit_start
                .saturating_add(new_text.len())
                .saturating_add_signed(delta);
        }

        let old_len = edit_end.saturating_sub(edit_start);
        delta += new_text.len() as isize - old_len as isize;
    }

    offset.saturating_add_signed(delta)
}

fn cycle_pair(current: DelimiterPair, pairs: &[DelimiterPair]) -> Option<DelimiterPair> {
    let index = pairs.iter().position(|pair| *pair == current)?;
    Some(pairs[(index + 1) % pairs.len()])
}

fn delimiter_pairs(kind: DelimiterKind, language: Option<&str>) -> Vec<DelimiterPair> {
    match kind {
        DelimiterKind::Bracket => bracket_pairs(language),
        DelimiterKind::Quote => quote_pairs(language),
    }
}

fn bracket_pairs(language: Option<&str>) -> Vec<DelimiterPair> {
    match language {
        Some("json") => vec![DelimiterPair::new('[', ']'), DelimiterPair::new('{', '}')],
        Some("css") | Some("html") => {
            vec![DelimiterPair::new('(', ')'), DelimiterPair::new('{', '}')]
        }
        Some("typescript") | Some("typescriptreact") | Some("tsx") => vec![
            DelimiterPair::new('(', ')'),
            DelimiterPair::new('[', ']'),
            DelimiterPair::new('{', '}'),
        ],
        _ => vec![
            DelimiterPair::new('(', ')'),
            DelimiterPair::new('[', ']'),
            DelimiterPair::new('{', '}'),
        ],
    }
}

fn quote_pairs(language: Option<&str>) -> Vec<DelimiterPair> {
    match language {
        Some("javascript") | Some("typescript") | Some("typescriptreact") | Some("tsx") => {
            vec![
                DelimiterPair::same('\''),
                DelimiterPair::same('"'),
                DelimiterPair::same('`'),
            ]
        }
        Some("json") => vec![DelimiterPair::same('"')],
        _ => vec![DelimiterPair::same('"'), DelimiterPair::same('\'')],
    }
}

fn normalized_delimiter_language(
    snapshot: &MultiBufferSnapshot,
    offset: MultiBufferOffset,
) -> Option<String> {
    snapshot.language_at(offset).map(|language| {
        language
            .name()
            .0
            .to_string()
            .to_lowercase()
            .replace([' ', '-'], "")
    })
}
