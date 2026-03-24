use crate::{
    Vim,
    motion::{self, Motion},
    object::{Object, surrounding_markers},
    state::Mode,
};
use editor::{Anchor, Bias, MultiBufferOffset, ToOffset, movement};
use gpui::{Context, Window};
use language::BracketPair;

use std::sync::Arc;

/// A char-based surround pair definition.
/// Single source of truth for all supported surround pairs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurroundPair {
    pub open: char,
    pub close: char,
}

impl SurroundPair {
    pub const fn new(open: char, close: char) -> Self {
        Self { open, close }
    }

    pub fn to_bracket_pair(self) -> BracketPair {
        BracketPair {
            start: self.open.to_string(),
            end: self.close.to_string(),
            close: true,
            surround: true,
            newline: false,
        }
    }

    pub fn to_object(self) -> Option<Object> {
        match self.open {
            '\'' => Some(Object::Quotes),
            '`' => Some(Object::BackQuotes),
            '"' => Some(Object::DoubleQuotes),
            '|' => Some(Object::VerticalBars),
            '(' => Some(Object::Parentheses),
            '[' => Some(Object::SquareBrackets),
            '{' => Some(Object::CurlyBrackets),
            '<' => Some(Object::AngleBrackets),
            _ => None,
        }
    }
}

/// All supported surround pairs - single source of truth.
pub const SURROUND_PAIRS: &[SurroundPair] = &[
    SurroundPair::new('(', ')'),
    SurroundPair::new('[', ']'),
    SurroundPair::new('{', '}'),
    SurroundPair::new('<', '>'),
    SurroundPair::new('"', '"'),
    SurroundPair::new('\'', '\''),
    SurroundPair::new('`', '`'),
    SurroundPair::new('|', '|'),
];

/// Bracket-only pairs for AnyBrackets matching.
const BRACKET_PAIRS: &[SurroundPair] = &[
    SurroundPair::new('(', ')'),
    SurroundPair::new('[', ']'),
    SurroundPair::new('{', '}'),
    SurroundPair::new('<', '>'),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SurroundsType {
    Motion(Motion),
    Object(Object, bool),
    Selection,
}

impl Vim {
    pub fn add_surrounds(
        &mut self,
        text: Arc<str>,
        target: SurroundsType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        let count = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        let mode = self.mode;
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let pair = bracket_pair_for_str_vim(&text);
                let surround = pair.end != surround_alias((*text).as_ref());
                let display_map = editor.display_snapshot(cx);
                let display_selections = editor.selections.all_adjusted_display(&display_map);
                let mut edits = Vec::new();
                let mut anchors = Vec::new();

                for selection in &display_selections {
                    let range = match &target {
                        SurroundsType::Object(object, around) => {
                            object.range(&display_map, selection.clone(), *around, None)
                        }
                        SurroundsType::Motion(motion) => {
                            motion
                                .range(
                                    &display_map,
                                    selection.clone(),
                                    count,
                                    &text_layout_details,
                                    forced_motion,
                                )
                                .map(|(mut range, _)| {
                                    // The Motion::CurrentLine operation will contain the newline of the current line and leading/trailing whitespace
                                    if let Motion::CurrentLine = motion {
                                        range.start = motion::first_non_whitespace(
                                            &display_map,
                                            false,
                                            range.start,
                                        );
                                        range.end = movement::saturating_right(
                                            &display_map,
                                            motion::last_non_whitespace(&display_map, range.end, 1),
                                        );
                                    }
                                    range
                                })
                        }
                        SurroundsType::Selection => Some(selection.range()),
                    };

                    if let Some(range) = range {
                        let start = range.start.to_offset(&display_map, Bias::Right);
                        let end = range.end.to_offset(&display_map, Bias::Left);
                        let (start_cursor_str, end_cursor_str) = if mode == Mode::VisualLine {
                            (format!("{}\n", pair.start), format!("\n{}", pair.end))
                        } else {
                            let maybe_space = if surround { " " } else { "" };
                            (
                                format!("{}{}", pair.start, maybe_space),
                                format!("{}{}", maybe_space, pair.end),
                            )
                        };
                        let start_anchor = display_map.buffer_snapshot().anchor_before(start);

                        edits.push((start..start, start_cursor_str));
                        edits.push((end..end, end_cursor_str));
                        anchors.push(start_anchor..start_anchor);
                    } else {
                        let start_anchor = display_map
                            .buffer_snapshot()
                            .anchor_before(selection.head().to_offset(&display_map, Bias::Left));
                        anchors.push(start_anchor..start_anchor);
                    }
                }

                editor.edit(edits, cx);
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(Default::default(), window, cx, |s| {
                    if mode == Mode::VisualBlock {
                        s.select_anchor_ranges(anchors.into_iter().take(1))
                    } else {
                        s.select_anchor_ranges(anchors)
                    }
                });
            });
        });
        self.switch_mode(Mode::Normal, false, window, cx);
    }

    pub fn delete_surrounds(
        &mut self,
        text: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);

        // only legitimate surrounds can be removed
        let Some(first_char) = text.chars().next() else {
            return;
        };
        let Some(surround_pair) = surround_pair_for_char_vim(first_char) else {
            return;
        };
        let Some(pair_object) = surround_pair.to_object() else {
            return;
        };
        let pair = surround_pair.to_bracket_pair();
        let surround = pair.end != *text;

        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let display_map = editor.display_snapshot(cx);
                let display_selections = editor.selections.all_display(&display_map);
                let mut edits = Vec::new();
                let mut anchors = Vec::new();

                for selection in &display_selections {
                    let start = selection.start.to_offset(&display_map, Bias::Left);
                    if let Some(range) =
                        pair_object.range(&display_map, selection.clone(), true, None)
                    {
                        // If the current parenthesis object is single-line,
                        // then we need to filter whether it is the current line or not
                        if !pair_object.is_multiline() {
                            let is_same_row = selection.start.row() == range.start.row()
                                && selection.end.row() == range.end.row();
                            if !is_same_row {
                                anchors.push(start..start);
                                continue;
                            }
                        }
                        // This is a bit cumbersome, and it is written to deal with some special cases, as shown below
                        // hello«ˇ  "hello in a word"  »again.
                        // Sometimes the expand_selection will not be matched at both ends, and there will be extra spaces
                        // In order to be able to accurately match and replace in this case, some cumbersome methods are used
                        let mut chars_and_offset = display_map
                            .buffer_chars_at(range.start.to_offset(&display_map, Bias::Left))
                            .peekable();
                        while let Some((ch, offset)) = chars_and_offset.next() {
                            if ch.to_string() == pair.start {
                                let start = offset;
                                let mut end = start + 1usize;
                                if surround
                                    && let Some((next_ch, _)) = chars_and_offset.peek()
                                    && next_ch.eq(&' ')
                                {
                                    end += 1;
                                }
                                edits.push((start..end, ""));
                                anchors.push(start..start);
                                break;
                            }
                        }
                        let mut reverse_chars_and_offsets = display_map
                            .reverse_buffer_chars_at(range.end.to_offset(&display_map, Bias::Left))
                            .peekable();
                        while let Some((ch, offset)) = reverse_chars_and_offsets.next() {
                            if ch.to_string() == pair.end {
                                let mut start = offset;
                                let end = start + 1usize;
                                if surround
                                    && let Some((next_ch, _)) = reverse_chars_and_offsets.peek()
                                    && next_ch.eq(&' ')
                                {
                                    start -= 1;
                                }
                                edits.push((start..end, ""));
                                break;
                            }
                        }
                    } else {
                        anchors.push(start..start);
                    }
                }

                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_ranges(anchors);
                });
                edits.sort_by_key(|(range, _)| range.start);
                editor.edit(edits, cx);
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }

    pub fn change_surrounds(
        &mut self,
        text: Arc<str>,
        target: Object,
        opening: bool,
        bracket_anchors: Vec<Option<(Anchor, Anchor)>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(will_replace_pair) = self.object_to_bracket_pair(target, cx) {
            self.stop_recording(cx);
            self.update_editor(cx, |_, editor, cx| {
                editor.transact(window, cx, |editor, window, cx| {
                    editor.set_clip_at_line_ends(false, cx);

                    let pair = bracket_pair_for_str_vim(&text);

                    // A single space should be added if the new surround is a
                    // bracket and not a quote (pair.start != pair.end) and if
                    // the bracket used is the opening bracket.
                    let add_space =
                        !(pair.start == pair.end) && (pair.end != surround_alias((*text).as_ref()));

                    // Space should be preserved if either the surrounding
                    // characters being updated are quotes
                    // (will_replace_pair.start == will_replace_pair.end) or if
                    // the bracket used in the command is not an opening
                    // bracket.
                    let preserve_space =
                        will_replace_pair.start == will_replace_pair.end || !opening;

                    let display_map = editor.display_snapshot(cx);
                    let mut edits = Vec::new();

                    // Collect (open_offset, close_offset) pairs to replace from the
                    // pre-computed anchors stored during check_and_move_to_valid_bracket_pair.
                    let mut pairs_to_replace: Vec<(MultiBufferOffset, MultiBufferOffset)> =
                        Vec::new();
                    let snapshot = display_map.buffer_snapshot();
                    for anchors in &bracket_anchors {
                        let Some((open_anchor, close_anchor)) = anchors else {
                            continue;
                        };
                        let pair = (
                            open_anchor.to_offset(&snapshot),
                            close_anchor.to_offset(&snapshot),
                        );
                        if !pairs_to_replace.contains(&pair) {
                            pairs_to_replace.push(pair);
                        }
                    }

                    for (open_offset, close_offset) in pairs_to_replace {
                        let mut open_str = pair.start.clone();
                        let mut chars_and_offset =
                            display_map.buffer_chars_at(open_offset).peekable();
                        chars_and_offset.next(); // skip the bracket itself
                        let mut open_range_end = open_offset + 1usize;
                        while let Some((next_ch, _)) = chars_and_offset.next()
                            && next_ch == ' '
                        {
                            open_range_end += 1;
                            if preserve_space {
                                open_str.push(next_ch);
                            }
                        }
                        if add_space {
                            open_str.push(' ');
                        }
                        let edit_len = open_range_end - open_offset;
                        edits.push((open_offset..open_range_end, open_str));

                        let mut close_str = String::new();
                        let close_end = close_offset + 1usize;
                        let mut close_start = close_offset;
                        for (next_ch, _) in display_map.reverse_buffer_chars_at(close_offset) {
                            if next_ch != ' '
                                || close_str.len() >= edit_len - 1
                                || close_start <= open_range_end
                            {
                                break;
                            }
                            close_start -= 1;
                            if preserve_space {
                                close_str.push(next_ch);
                            }
                        }
                        if add_space {
                            close_str.push(' ');
                        }
                        close_str.push_str(&pair.end);
                        edits.push((close_start..close_end, close_str));
                    }

                    let stable_anchors = editor
                        .selections
                        .disjoint_anchors_arc()
                        .iter()
                        .map(|selection| {
                            let start = selection.start.bias_left(&display_map.buffer_snapshot());
                            start..start
                        })
                        .collect::<Vec<_>>();
                    edits.sort_by_key(|(range, _)| range.start);
                    editor.edit(edits, cx);
                    editor.set_clip_at_line_ends(true, cx);
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_anchor_ranges(stable_anchors);
                    });
                });
            });
        }
    }

    /// **Only intended for use by the `cs` (change surrounds) operator.**
    ///
    /// For each cursor, checks whether it is surrounded by a valid bracket pair for the given
    /// object. Moves each cursor to the opening bracket of its found pair, and returns a
    /// `Vec<Option<(Anchor, Anchor)>>` with one entry per selection containing the pre-computed
    /// open and close bracket positions.
    ///
    /// Storing these anchors avoids re-running the bracket search from the moved cursor position,
    /// which can misidentify the opening bracket for symmetric quote characters when the same
    /// character appears earlier on the line (e.g. `I'm 'good'`).
    ///
    /// Returns an empty `Vec` if no valid pair was found for any cursor.
    pub fn prepare_and_move_to_valid_bracket_pair(
        &mut self,
        object: Object,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Option<(Anchor, Anchor)>> {
        let mut matched_pair_anchors: Vec<Option<(Anchor, Anchor)>> = Vec::new();
        if let Some(pair) = self.object_to_bracket_pair(object, cx) {
            self.update_editor(cx, |_, editor, cx| {
                editor.transact(window, cx, |editor, window, cx| {
                    editor.set_clip_at_line_ends(false, cx);
                    let display_map = editor.display_snapshot(cx);
                    let selections = editor.selections.all_adjusted_display(&display_map);
                    let mut updated_cursor_ranges = Vec::new();

                    for selection in &selections {
                        let start = selection.start.to_offset(&display_map, Bias::Left);
                        let in_range = object
                            .range(&display_map, selection.clone(), true, None)
                            .filter(|range| {
                                object.is_multiline()
                                    || (selection.start.row() == range.start.row()
                                        && selection.end.row() == range.end.row())
                            });
                        let Some(range) = in_range else {
                            updated_cursor_ranges.push(start..start);
                            matched_pair_anchors.push(None);
                            continue;
                        };

                        let range_start = range.start.to_offset(&display_map, Bias::Left);
                        let range_end = range.end.to_offset(&display_map, Bias::Left);
                        let open_offset = display_map
                            .buffer_chars_at(range_start)
                            .find(|(ch, _)| ch.to_string() == pair.start)
                            .map(|(_, offset)| offset);
                        let close_offset = display_map
                            .reverse_buffer_chars_at(range_end)
                            .find(|(ch, _)| ch.to_string() == pair.end)
                            .map(|(_, offset)| offset);

                        if let (Some(open), Some(close)) = (open_offset, close_offset) {
                            let snapshot = &display_map.buffer_snapshot();
                            updated_cursor_ranges.push(open..open);
                            matched_pair_anchors.push(Some((
                                snapshot.anchor_before(open),
                                snapshot.anchor_before(close),
                            )));
                        } else {
                            updated_cursor_ranges.push(start..start);
                            matched_pair_anchors.push(None);
                        }
                    }
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.select_ranges(updated_cursor_ranges);
                    });
                    editor.set_clip_at_line_ends(true, cx);

                    if !matched_pair_anchors.iter().any(|a| a.is_some()) {
                        matched_pair_anchors.clear();
                    }
                });
            });
        }
        matched_pair_anchors
    }

    fn object_to_bracket_pair(
        &self,
        object: Object,
        cx: &mut Context<Self>,
    ) -> Option<BracketPair> {
        if let Some(pair) = object_to_surround_pair(object) {
            return Some(pair.to_bracket_pair());
        }

        if object != Object::AnyBrackets {
            return None;
        }

        // If we're dealing with `AnyBrackets`, which can map to multiple bracket
        // pairs, we'll need to first determine which `BracketPair` to target.
        // As such, we keep track of the smallest range size, so that in cases
        // like `({ name: "John" })` if the cursor is inside the curly brackets,
        // we target the curly brackets instead of the parentheses.
        let mut best_pair = None;
        let mut min_range_size = usize::MAX;

        let _ = self.editor.update(cx, |editor, cx| {
            let display_map = editor.display_snapshot(cx);
            let selections = editor.selections.all_adjusted_display(&display_map);
            // Even if there's multiple cursors, we'll simply rely on the first one
            // to understand what bracket pair to map to. I believe we could, if
            // worth it, go one step above and have a `BracketPair` per selection, so
            // that `AnyBracket` could work in situations where the transformation
            // below could be done.
            //
            // ```
            // (< name:ˇ'Zed' >)
            // <[ name:ˇ'DeltaDB' ]>
            // ```
            //
            // After using `csb{`:
            //
            // ```
            // (ˇ{ name:'Zed' })
            // <ˇ{ name:'DeltaDB' }>
            // ```
            if let Some(selection) = selections.first() {
                let relative_to = selection.head();
                let cursor_offset = relative_to.to_offset(&display_map, Bias::Left);

                for pair in BRACKET_PAIRS {
                    if let Some(range) = surrounding_markers(
                        &display_map,
                        relative_to,
                        true,
                        false,
                        pair.open,
                        pair.close,
                    ) {
                        let start_offset = range.start.to_offset(&display_map, Bias::Left);
                        let end_offset = range.end.to_offset(&display_map, Bias::Right);

                        if cursor_offset >= start_offset && cursor_offset <= end_offset {
                            let size = end_offset - start_offset;
                            if size < min_range_size {
                                min_range_size = size;
                                best_pair = Some(*pair);
                            }
                        }
                    }
                }
            }
        });

        best_pair.map(|p| p.to_bracket_pair())
    }
}

/// Convert an Object to its corresponding SurroundPair.
fn object_to_surround_pair(object: Object) -> Option<SurroundPair> {
    let open = match object {
        Object::Quotes => '\'',
        Object::BackQuotes => '`',
        Object::DoubleQuotes => '"',
        Object::VerticalBars => '|',
        Object::Parentheses => '(',
        Object::SquareBrackets => '[',
        Object::CurlyBrackets { .. } => '{',
        Object::AngleBrackets => '<',
        _ => return None,
    };
    surround_pair_for_char_vim(open)
}

pub fn surround_alias(ch: &str) -> &str {
    match ch {
        "b" => ")",
        "B" => "}",
        "a" => ">",
        "r" => "]",
        _ => ch,
    }
}

fn literal_surround_pair(ch: char) -> Option<SurroundPair> {
    SURROUND_PAIRS
        .iter()
        .find(|p| p.open == ch || p.close == ch)
        .copied()
}

/// Resolve a character (including Vim aliases) to its surround pair.
/// Returns None for 'm' (match nearest) or unknown chars.
pub fn surround_pair_for_char_vim(ch: char) -> Option<SurroundPair> {
    let resolved = match ch {
        'b' => ')',
        'B' => '}',
        'r' => ']',
        'a' => '>',
        'm' => return None,
        _ => ch,
    };
    literal_surround_pair(resolved)
}

/// Get a BracketPair for the given string, with fallback for unknown chars.
/// For vim surround operations that accept any character as a surround.
pub fn bracket_pair_for_str_vim(text: &str) -> BracketPair {
    text.chars()
        .next()
        .and_then(surround_pair_for_char_vim)
        .map(|p| p.to_bracket_pair())
        .unwrap_or_else(|| BracketPair {
            start: text.to_string(),
            end: text.to_string(),
            close: true,
            surround: true,
            newline: false,
        })
}

/// Resolve a character to its surround pair using Helix semantics (no Vim aliases).
/// Returns None only for 'm' (match nearest). Unknown chars map to symmetric pairs.
pub fn surround_pair_for_char_helix(ch: char) -> Option<SurroundPair> {
    if ch == 'm' {
        return None;
    }
    literal_surround_pair(ch).or_else(|| Some(SurroundPair::new(ch, ch)))
}

/// Get a BracketPair for the given string in Helix mode (literal, symmetric fallback).
pub fn bracket_pair_for_str_helix(text: &str) -> BracketPair {
    text.chars()
        .next()
        .and_then(surround_pair_for_char_helix)
        .map(|p| p.to_bracket_pair())
        .unwrap_or_else(|| BracketPair {
            start: text.to_string(),
            end: text.to_string(),
            close: true,
            surround: true,
            newline: false,
        })
}

#[cfg(test)]
mod test {
    use gpui::KeyBinding;
    use indoc::indoc;

    use crate::{PushAddSurrounds, object::AnyBrackets, state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_add_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // test add surrounds with around
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w {");
        cx.assert_state(
            indoc! {"
            The ˇ{ quick } brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds not with around
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w }");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with motion
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s $ }");
        cx.assert_state(
            indoc! {"
            The quˇ{ick brown}
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with multi cursor
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w '");
        cx.assert_state(
            indoc! {"
            The ˇ'quick' brown
            fox jumps over
            the ˇ'lazy' dog."},
            Mode::Normal,
        );

        // test multi cursor add surrounds with motion
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s $ '");
        cx.assert_state(
            indoc! {"
            The quˇ'ick brown'
            fox jumps over
            the laˇ'zy dog.'"},
            Mode::Normal,
        );

        // test multi cursor add surrounds with motion and custom string
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s $ 1");
        cx.assert_state(
            indoc! {"
            The quˇ1ick brown1
            fox jumps over
            the laˇ1zy dog.1"},
            Mode::Normal,
        );

        // test add surrounds with motion current line
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s s {");
        cx.assert_state(
            indoc! {"
            ˇ{ The quick brown }
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
                The quˇick brown•
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s s {");
        cx.assert_state(
            indoc! {"
                ˇ{ The quick brown }•
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("2 y s s )");
        cx.assert_state(
            indoc! {"
                ˇ({ The quick brown }•
            fox jumps over)
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds around object
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s a ] )");
        cx.assert_state(
            indoc! {"
            The ˇ([quick]) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds inside object
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i ] )");
        cx.assert_state(
            indoc! {"
            The [ˇ(quick)] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_add_surrounds_visual(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.update(|_, cx| {
            cx.bind_keys([KeyBinding::new(
                "shift-s",
                PushAddSurrounds {},
                Some("vim_mode == visual"),
            )])
        });

        // test add surrounds with around
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v i w shift-s {");
        cx.assert_state(
            indoc! {"
            The ˇ{ quick } brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds not with around
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v i w shift-s }");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with motion
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v e shift-s }");
        cx.assert_state(
            indoc! {"
            The quˇ{ick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test add surrounds with multi cursor
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the laˇzy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("v i w shift-s '");
        cx.assert_state(
            indoc! {"
            The ˇ'quick' brown
            fox jumps over
            the ˇ'lazy' dog."},
            Mode::Normal,
        );

        // test add surrounds with visual block
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("ctrl-v i w j j shift-s '");
        cx.assert_state(
            indoc! {"
            The ˇ'quick' brown
            fox 'jumps' over
            the 'lazy 'dog."},
            Mode::Normal,
        );

        // test add surrounds with visual line
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("j shift-v shift-s '");
        cx.assert_state(
            indoc! {"
            The quick brown
            ˇ'
            fox jumps over
            '
            the lazy dog."},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_delete_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // test delete surround
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s {");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test delete not exist surrounds
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s [");
        cx.assert_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test delete surround forward exist, in the surrounds plugin of other editors,
        // the bracket pair in front of the current line will be deleted here, which is not implemented at the moment
        cx.set_state(
            indoc! {"
            The {quick} brˇown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s {");
        cx.assert_state(
            indoc! {"
            The {quick} brˇown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test cursor delete inner surrounds
        cx.set_state(
            indoc! {"
            The { quick brown
            fox jumˇps over }
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s {");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete surrounds
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s ]");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete surrounds with around
        cx.set_state(
            indoc! {"
            Tˇhe [ quick ] brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s [");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            Tˇhe [ quick ] brown
            fox jumps over
            the [laˇzy ] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s [");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test multi cursor delete different surrounds
        // the pair corresponding to the two cursors is the same,
        // so they are combined into one cursor
        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s {");
        cx.assert_state(
            indoc! {"
            The [quick] brown
            fox jumps over
            the ˇlazy dog."},
            Mode::Normal,
        );

        // test delete surround with multi cursor and nest surrounds
        cx.set_state(
            indoc! {"
            fn test_surround() {
                ifˇ 2 > 1 {
                    ˇprintln!(\"it is fine\");
                };
            }"},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s }");
        cx.assert_state(
            indoc! {"
            fn test_surround() ˇ
                if 2 > 1 ˇ
                    println!(\"it is fine\");
                ;
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_change_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { [");
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // test multi cursor change surrounds
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { [");
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the ˇ[ lazy ] dog."},
            Mode::Normal,
        );

        // test multi cursor delete different surrounds with after cursor
        cx.set_state(
            indoc! {"
            Thˇe {quick} brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { [");
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the ˇ[ lazy ] dog."},
            Mode::Normal,
        );

        // test multi cursor change surrount with not around
        cx.set_state(
            indoc! {"
            Thˇe { quick } brown
            fox jumps over
            the {laˇzy} dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { ]");
        cx.assert_state(
            indoc! {"
            The ˇ[quick] brown
            fox jumps over
            the ˇ[lazy] dog."},
            Mode::Normal,
        );

        // test multi cursor change with not exist surround
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the [laˇzy] dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s [ '");
        cx.assert_state(
            indoc! {"
            The {quick} brown
            fox jumps over
            the ˇ'lazy' dog."},
            Mode::Normal,
        );

        // test change nesting surrounds
        cx.set_state(
            indoc! {"
            fn test_surround() {
                ifˇ 2 > 1 {
                    ˇprintln!(\"it is fine\");
                }
            };"},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s } ]");
        cx.assert_state(
            indoc! {"
            fn test_surround() ˇ[
                if 2 > 1 ˇ[
                    println!(\"it is fine\");
                ]
            ];"},
            Mode::Normal,
        );

        // test spaces with quote change surrounds
        cx.set_state(
            indoc! {"
            fn test_surround() {
                \"ˇ \"
            };"},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s \" '");
        cx.assert_state(
            indoc! {"
            fn test_surround() {
                ˇ' '
            };"},
            Mode::Normal,
        );

        // Currently, the same test case but using the closing bracket `]`
        // actually removes a whitespace before the closing bracket, something
        // that might need to be fixed?
        cx.set_state(
            indoc! {"
            fn test_surround() {
                ifˇ 2 > 1 {
                    ˇprintln!(\"it is fine\");
                }
            };"},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { ]");
        cx.assert_state(
            indoc! {"
            fn test_surround() ˇ[
                if 2 > 1 ˇ[
                    println!(\"it is fine\");
                ]
            ];"},
            Mode::Normal,
        );

        // test change quotes.
        cx.set_state(indoc! {"'  ˇstr  '"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' \"");
        cx.assert_state(indoc! {"ˇ\"  str  \""}, Mode::Normal);

        // test multi cursor change quotes
        cx.set_state(
            indoc! {"
            '  ˇstr  '
            some example text here
            ˇ'  str  '
        "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s ' \"");
        cx.assert_state(
            indoc! {"
            ˇ\"  str  \"
            some example text here
            ˇ\"  str  \"
        "},
            Mode::Normal,
        );

        // test quote to bracket spacing.
        cx.set_state(indoc! {"'ˇfoobar'"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' {");
        cx.assert_state(indoc! {"ˇ{ foobar }"}, Mode::Normal);

        cx.set_state(indoc! {"'ˇfoobar'"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' }");
        cx.assert_state(indoc! {"ˇ{foobar}"}, Mode::Normal);

        cx.set_state(indoc! {"I'm 'goˇod'"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' \"");
        cx.assert_state(indoc! {"I'm ˇ\"good\""}, Mode::Normal);

        cx.set_state(indoc! {"I'm 'goˇod'"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' {");
        cx.assert_state(indoc! {"I'm ˇ{ good }"}, Mode::Normal);
    }

    #[gpui::test]
    async fn test_change_surrounds_any_brackets(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Update keybindings so that using `csb` triggers Vim's `AnyBrackets`
        // action.
        cx.update(|_, cx| {
            cx.bind_keys([KeyBinding::new(
                "b",
                AnyBrackets,
                Some("vim_operator == a || vim_operator == i || vim_operator == cs"),
            )]);
        });

        cx.set_state(indoc! {"{braˇcketed}"}, Mode::Normal);
        cx.simulate_keystrokes("c s b [");
        cx.assert_state(indoc! {"ˇ[ bracketed ]"}, Mode::Normal);

        cx.set_state(indoc! {"[braˇcketed]"}, Mode::Normal);
        cx.simulate_keystrokes("c s b {");
        cx.assert_state(indoc! {"ˇ{ bracketed }"}, Mode::Normal);

        cx.set_state(indoc! {"<braˇcketed>"}, Mode::Normal);
        cx.simulate_keystrokes("c s b [");
        cx.assert_state(indoc! {"ˇ[ bracketed ]"}, Mode::Normal);

        cx.set_state(indoc! {"(braˇcketed)"}, Mode::Normal);
        cx.simulate_keystrokes("c s b [");
        cx.assert_state(indoc! {"ˇ[ bracketed ]"}, Mode::Normal);

        cx.set_state(indoc! {"(< name: ˇ'Zed' >)"}, Mode::Normal);
        cx.simulate_keystrokes("c s b }");
        cx.assert_state(indoc! {"(ˇ{ name: 'Zed' })"}, Mode::Normal);

        cx.set_state(
            indoc! {"
            (< name: ˇ'Zed' >)
            (< nˇame: 'DeltaDB' >)
        "},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s b {");
        cx.set_state(
            indoc! {"
            (ˇ{ name: 'Zed' })
            (ˇ{ name: 'DeltaDB' })
        "},
            Mode::Normal,
        );
    }

    // The following test cases all follow tpope/vim-surround's behaviour
    // and are more focused on how whitespace is handled.
    #[gpui::test]
    async fn test_change_surrounds_vim(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Changing quote to quote should never change the surrounding
        // whitespace.
        cx.set_state(indoc! {"'  ˇa  '"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' \"");
        cx.assert_state(indoc! {"ˇ\"  a  \""}, Mode::Normal);

        cx.set_state(indoc! {"\"  ˇa  \""}, Mode::Normal);
        cx.simulate_keystrokes("c s \" '");
        cx.assert_state(indoc! {"ˇ'  a  '"}, Mode::Normal);

        // Changing quote to bracket adds one more space when the opening
        // bracket is used, does not affect whitespace when the closing bracket
        // is used.
        cx.set_state(indoc! {"'  ˇa  '"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' {");
        cx.assert_state(indoc! {"ˇ{   a   }"}, Mode::Normal);

        cx.set_state(indoc! {"'  ˇa  '"}, Mode::Normal);
        cx.simulate_keystrokes("c s ' }");
        cx.assert_state(indoc! {"ˇ{  a  }"}, Mode::Normal);

        // Changing bracket to quote should remove all space when the
        // opening bracket is used and preserve all space when the
        // closing one is used.
        cx.set_state(indoc! {"{  ˇa  }"}, Mode::Normal);
        cx.simulate_keystrokes("c s { '");
        cx.assert_state(indoc! {"ˇ'a'"}, Mode::Normal);

        cx.set_state(indoc! {"{  ˇa  }"}, Mode::Normal);
        cx.simulate_keystrokes("c s } '");
        cx.assert_state(indoc! {"ˇ'  a  '"}, Mode::Normal);

        // Changing bracket to bracket follows these rules:
        // * opening → opening – keeps only one space.
        // * opening → closing – removes all space.
        // * closing → opening – adds one space.
        // * closing → closing – does not change space.
        cx.set_state(indoc! {"{   ˇa   }"}, Mode::Normal);
        cx.simulate_keystrokes("c s { [");
        cx.assert_state(indoc! {"ˇ[ a ]"}, Mode::Normal);

        cx.set_state(indoc! {"{   ˇa   }"}, Mode::Normal);
        cx.simulate_keystrokes("c s { ]");
        cx.assert_state(indoc! {"ˇ[a]"}, Mode::Normal);

        cx.set_state(indoc! {"{  ˇa  }"}, Mode::Normal);
        cx.simulate_keystrokes("c s } [");
        cx.assert_state(indoc! {"ˇ[   a   ]"}, Mode::Normal);

        cx.set_state(indoc! {"{  ˇa  }"}, Mode::Normal);
        cx.simulate_keystrokes("c s } ]");
        cx.assert_state(indoc! {"ˇ[  a  ]"}, Mode::Normal);
    }

    #[gpui::test]
    async fn test_surrounds(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w [");
        cx.assert_state(
            indoc! {"
            The ˇ[ quick ] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes("c s [ }");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes("d s {");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.simulate_keystrokes("u");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_surround_aliases(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // add aliases
        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w b");
        cx.assert_state(
            indoc! {"
            The ˇ(quick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w B");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w a");
        cx.assert_state(
            indoc! {"
            The ˇ<quick> brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The quˇick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("y s i w r");
        cx.assert_state(
            indoc! {"
            The ˇ[quick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // change aliases
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s { b");
        cx.assert_state(
            indoc! {"
            The ˇ(quick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The (quˇick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s ( B");
        cx.assert_state(
            indoc! {"
            The ˇ{quick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The (quˇick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s ( a");
        cx.assert_state(
            indoc! {"
            The ˇ<quick> brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The <quˇick> brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s < b");
        cx.assert_state(
            indoc! {"
            The ˇ(quick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The (quˇick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s ( r");
        cx.assert_state(
            indoc! {"
            The ˇ[quick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("c s [ b");
        cx.assert_state(
            indoc! {"
            The ˇ(quick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        // delete alias
        cx.set_state(
            indoc! {"
            The {quˇick} brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s B");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The (quˇick) brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s b");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The [quˇick] brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s r");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );

        cx.set_state(
            indoc! {"
            The <quˇick> brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
        cx.simulate_keystrokes("d s a");
        cx.assert_state(
            indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog."},
            Mode::Normal,
        );
    }

    #[test]
    fn test_surround_pair_for_char() {
        use super::{SURROUND_PAIRS, surround_pair_for_char_helix, surround_pair_for_char_vim};

        fn as_tuple(pair: Option<super::SurroundPair>) -> Option<(char, char)> {
            pair.map(|p| (p.open, p.close))
        }

        assert_eq!(as_tuple(surround_pair_for_char_vim('b')), Some(('(', ')')));
        assert_eq!(as_tuple(surround_pair_for_char_vim('B')), Some(('{', '}')));
        assert_eq!(as_tuple(surround_pair_for_char_vim('r')), Some(('[', ']')));
        assert_eq!(as_tuple(surround_pair_for_char_vim('a')), Some(('<', '>')));

        assert_eq!(surround_pair_for_char_vim('m'), None);

        for pair in SURROUND_PAIRS {
            assert_eq!(
                as_tuple(surround_pair_for_char_vim(pair.open)),
                Some((pair.open, pair.close))
            );
            assert_eq!(
                as_tuple(surround_pair_for_char_vim(pair.close)),
                Some((pair.open, pair.close))
            );
        }

        // Test unknown char returns None
        assert_eq!(surround_pair_for_char_vim('x'), None);

        // Helix resolves literal chars and falls back to symmetric pairs.
        assert_eq!(
            as_tuple(surround_pair_for_char_helix('*')),
            Some(('*', '*'))
        );
        assert_eq!(surround_pair_for_char_helix('m'), None);
    }
}
