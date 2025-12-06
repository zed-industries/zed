use editor::display_map::DisplaySnapshot;
use editor::{Bias, DisplayPoint, MultiBufferOffset};
use gpui::{Context, Window};
use language::BracketPair;

use crate::object::surrounding_markers;
use crate::surrounds::{all_support_surround_pair, find_surround_pair};
use crate::Vim;

/// Find the nearest surrounding bracket pair around the cursor.
fn find_nearest_surrounding_pair(
    display_map: &DisplaySnapshot,
    cursor: DisplayPoint,
) -> Option<(char, char)> {
    let bracket_pairs = [
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
        ('"', '"'),
        ('\'', '\''),
        ('`', '`'),
        ('|', '|'),
    ];

    let cursor_offset = cursor.to_offset(display_map, Bias::Left);
    let mut best_pair: Option<(char, char)> = None;
    let mut min_range_size = usize::MAX;

    for (open, close) in bracket_pairs {
        if let Some(range) = surrounding_markers(display_map, cursor, true, true, open, close) {
            let start_offset = range.start.to_offset(display_map, Bias::Left);
            let end_offset = range.end.to_offset(display_map, Bias::Right);

            if cursor_offset >= start_offset && cursor_offset <= end_offset {
                let size = end_offset - start_offset;
                if size < min_range_size {
                    min_range_size = size;
                    best_pair = Some((open, close));
                }
            }
        }
    }

    best_pair
}

impl Vim {
    /// mr - Replace innermost surrounding pair containing the cursor.
    pub fn helix_surround_replace(
        &mut self,
        old_char: char,
        new_char: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);

        let new_char_str = new_char.to_string();

        let new_pair = find_surround_pair(&all_support_surround_pair(), &new_char_str)
            .cloned()
            .unwrap_or_else(|| BracketPair {
                start: new_char_str.clone(),
                end: new_char_str.clone(),
                close: true,
                surround: true,
                newline: false,
            });

        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let display_map = editor.display_snapshot(cx);
                let selections = editor.selections.all_display(&display_map);
                let mut edits: Vec<(std::ops::Range<MultiBufferOffset>, String)> = Vec::new();
                let mut anchors = Vec::new();

                for selection in &selections {
                    let cursor = if selection.reversed || selection.is_empty() {
                        selection.head()
                    } else {
                        editor::movement::left(&display_map, selection.head())
                    };

                    // For 'm', find the nearest surrounding pair
                    let markers = match bracket_pair_for_char(old_char) {
                        Some((open, close)) => Some((open, close)),
                        None => find_nearest_surrounding_pair(&display_map, cursor),
                    };

                    let Some((open_marker, close_marker)) = markers else {
                        let offset = selection.head().to_offset(&display_map, Bias::Left);
                        let anchor = display_map.buffer_snapshot().anchor_before(offset);
                        anchors.push(anchor..anchor);
                        continue;
                    };

                    if let Some(range) = surrounding_markers(
                        &display_map,
                        cursor,
                        true,
                        true,
                        open_marker,
                        close_marker,
                    ) {
                        let open_start = range.start.to_offset(&display_map, Bias::Left);
                        let open_end = open_start + open_marker.len_utf8();
                        let close_end = range.end.to_offset(&display_map, Bias::Left);
                        let close_start = close_end - close_marker.len_utf8();

                        edits.push((close_start..close_end, new_pair.end.clone()));
                        edits.push((open_start..open_end, new_pair.start.clone()));

                        let anchor = display_map.buffer_snapshot().anchor_before(open_start);
                        anchors.push(anchor..anchor);
                    } else {
                        let offset = selection.head().to_offset(&display_map, Bias::Left);
                        let anchor = display_map.buffer_snapshot().anchor_before(offset);
                        anchors.push(anchor..anchor);
                    }
                }

                edits.sort_by(|a, b| b.0.start.cmp(&a.0.start));
                editor.edit(edits, cx);

                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_anchor_ranges(anchors);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }

    /// md - Delete innermost surrounding pair containing the cursor.
    pub fn helix_surround_delete(
        &mut self,
        target_char: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);

        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let display_map = editor.display_snapshot(cx);
                let selections = editor.selections.all_display(&display_map);
                let mut edits: Vec<(std::ops::Range<MultiBufferOffset>, String)> = Vec::new();
                let mut anchors = Vec::new();

                for selection in &selections {
                    let cursor = if selection.reversed || selection.is_empty() {
                        selection.head()
                    } else {
                        editor::movement::left(&display_map, selection.head())
                    };

                    // For 'm', find the nearest surrounding pair
                    let markers = match bracket_pair_for_char(target_char) {
                        Some((open, close)) => Some((open, close)),
                        None => find_nearest_surrounding_pair(&display_map, cursor),
                    };

                    let Some((open_marker, close_marker)) = markers else {
                        let offset = selection.head().to_offset(&display_map, Bias::Left);
                        let anchor = display_map.buffer_snapshot().anchor_before(offset);
                        anchors.push(anchor..anchor);
                        continue;
                    };

                    if let Some(range) = surrounding_markers(
                        &display_map,
                        cursor,
                        true,
                        true,
                        open_marker,
                        close_marker,
                    ) {
                        let open_start = range.start.to_offset(&display_map, Bias::Left);
                        let open_end = open_start + open_marker.len_utf8();
                        let close_end = range.end.to_offset(&display_map, Bias::Left);
                        let close_start = close_end - close_marker.len_utf8();

                        edits.push((close_start..close_end, String::new()));
                        edits.push((open_start..open_end, String::new()));

                        let anchor = display_map.buffer_snapshot().anchor_before(open_start);
                        anchors.push(anchor..anchor);
                    } else {
                        let offset = selection.head().to_offset(&display_map, Bias::Left);
                        let anchor = display_map.buffer_snapshot().anchor_before(offset);
                        anchors.push(anchor..anchor);
                    }
                }

                edits.sort_by(|a, b| b.0.start.cmp(&a.0.start));
                editor.edit(edits, cx);

                editor.change_selections(Default::default(), window, cx, |s| {
                    s.select_anchor_ranges(anchors);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    }
}

/// Convert a character to its open/close bracket pair.
/// Returns None for 'm' which means "find nearest matching pair".
fn bracket_pair_for_char(ch: char) -> Option<(char, char)> {
    match ch {
        '(' | ')' | 'b' => Some(('(', ')')),
        '[' | ']' | 'r' => Some(('[', ']')),
        '{' | '}' | 'B' => Some(('{', '}')),
        '<' | '>' | 'a' => Some(('<', '>')),
        '"' => Some(('"', '"')),
        '\'' => Some(('\'', '\'')),
        '`' => Some(('`', '`')),
        '|' => Some(('|', '|')),
        'm' => None, // Special case: find nearest matching pair
        _ => Some((ch, ch)),
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_helix_surround_add(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s (");
        cx.assert_state("hello ˇ(w)orld", Mode::HelixNormal);

        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s )");
        cx.assert_state("hello ˇ(w)orld", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s [");
        cx.assert_state("hello ˇ[worl]d", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s \"");
        cx.assert_state("hello ˇ\"worl\"d", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello \"woˇrld\" test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d \"");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello woˇrld test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("(ˇworld)", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_replace(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("hello ˇ[world] test", Mode::HelixNormal);

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( ]");
        cx.assert_state("hello ˇ[world] test", Mode::HelixNormal);

        cx.set_state("hello \"woˇrld\" test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r \" {");
        cx.assert_state("hello ˇ{world} test", Mode::HelixNormal);

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("(ˇ[world])", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_multiline(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            function test() {
                return ˇvalue;
            }"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m d {");
        cx.assert_state(
            indoc! {"
            function test() ˇ
                return value;
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_surround_select_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello «worldˇ» test", Mode::HelixSelect);
        cx.simulate_keystrokes("m s {");
        cx.assert_state("hello ˇ{world} test", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_multi_cursor(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            (heˇllo)
            (woˇrld)"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m d (");
        cx.assert_state(
            indoc! {"
            ˇhello
            ˇworld"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_surround_escape_cancels(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m escape");
        cx.assert_state("hello ˇworld", Mode::HelixNormal);

        cx.set_state("hello (woˇrld)", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( escape");
        cx.assert_state("hello (woˇrld)", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_aliases(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d b");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d B");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello [woˇrld] test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d r");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello <woˇrld> test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d a");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_match_nearest(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // mdm - delete nearest surrounding pair
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello [woˇrld] test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        // Nested - deletes innermost
        cx.set_state("([woˇrld])", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("(ˇworld)", Mode::HelixNormal);

        // mrm - replace nearest surrounding pair
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m [");
        cx.assert_state("hello ˇ[world] test", Mode::HelixNormal);

        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m (");
        cx.assert_state("hello ˇ(world) test", Mode::HelixNormal);

        // Nested - replaces innermost
        cx.set_state("([woˇrld])", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m {");
        cx.assert_state("(ˇ{world})", Mode::HelixNormal);
    }
}
