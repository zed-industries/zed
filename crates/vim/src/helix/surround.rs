use editor::display_map::DisplaySnapshot;
use editor::{Bias, DisplayPoint, MultiBufferOffset};
use gpui::{Context, Window};

use language::BracketPair;

use crate::Vim;
use crate::object::surrounding_markers;
use crate::surrounds::{SURROUND_PAIRS, SurroundPair};

/// Resolve a character to its surround pair for Helix mode.
/// Does NOT support Vim aliases (b, B, r, a) - uses literal characters only.
/// Returns None only for 'm' (match nearest).
/// For unknown chars, returns a symmetric pair (ch, ch) to match Helix behavior.
fn surround_pair_for_char(ch: char) -> Option<SurroundPair> {
    if ch == 'm' {
        return None;
    }
    SURROUND_PAIRS
        .iter()
        .find(|p| p.open == ch || p.close == ch)
        .copied()
        .or_else(|| Some(SurroundPair::new(ch, ch)))
}

/// Get a BracketPair for the given string in Helix mode.
/// Does NOT support Vim aliases - uses literal characters only.
pub fn bracket_pair_for_str(text: &str) -> BracketPair {
    text.chars()
        .next()
        .and_then(surround_pair_for_char)
        .map(|p| p.to_bracket_pair())
        .unwrap_or_else(|| BracketPair {
            start: text.to_string(),
            end: text.to_string(),
            close: true,
            surround: true,
            newline: false,
        })
}

/// Find the nearest surrounding bracket pair around the cursor.
fn find_nearest_surrounding_pair(
    display_map: &DisplaySnapshot,
    cursor: DisplayPoint,
) -> Option<(char, char)> {
    let cursor_offset = cursor.to_offset(display_map, Bias::Left);
    let mut best_pair: Option<(char, char)> = None;
    let mut min_range_size = usize::MAX;

    for pair in SURROUND_PAIRS {
        if let Some(range) =
            surrounding_markers(display_map, cursor, true, true, pair.open, pair.close)
        {
            let start_offset = range.start.to_offset(display_map, Bias::Left);
            let end_offset = range.end.to_offset(display_map, Bias::Right);

            if cursor_offset >= start_offset && cursor_offset <= end_offset {
                let size = end_offset - start_offset;
                if size < min_range_size {
                    min_range_size = size;
                    best_pair = Some((pair.open, pair.close));
                }
            }
        }
    }

    best_pair
}

impl Vim {
    /// ms - Add surrounding characters around selection.
    pub fn helix_surround_add(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.stop_recording(cx);

        let pair = bracket_pair_for_str(text);

        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);

                let display_map = editor.display_snapshot(cx);
                let selections = editor.selections.all_display(&display_map);
                let mut edits = Vec::new();
                let mut anchors = Vec::new();

                for selection in &selections {
                    let range = selection.range();
                    let start = range.start.to_offset(&display_map, Bias::Right);
                    let end = range.end.to_offset(&display_map, Bias::Left);

                    let start_anchor = display_map.buffer_snapshot().anchor_before(start);
                    edits.push((end..end, pair.end.clone()));
                    edits.push((start..start, pair.start.clone()));
                    anchors.push(start_anchor..start_anchor);
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
        let new_pair = bracket_pair_for_str(&new_char_str);

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
                    let markers = match surround_pair_for_char(old_char) {
                        Some(pair) => Some((pair.open, pair.close)),
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
                    let markers = match surround_pair_for_char(target_char) {
                        Some(pair) => Some((pair.open, pair.close)),
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
    async fn test_helix_surround_no_vim_aliases(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // In Helix mode, 'b', 'B', 'r', 'a' are NOT aliases for brackets.
        // They are treated as literal characters, so 'mdb' looks for 'b...b' surrounds.

        // 'b' is not an alias - it looks for literal 'b...b', finds none, does nothing
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d b");
        cx.assert_state("hello (woˇrld) test", Mode::HelixNormal);

        // 'B' looks for literal 'B...B', not {}
        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d B");
        cx.assert_state("hello {woˇrld} test", Mode::HelixNormal);

        // 'r' looks for literal 'r...r', not []
        cx.set_state("hello [woˇrld] test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d r");
        cx.assert_state("hello [woˇrld] test", Mode::HelixNormal);

        // 'a' looks for literal 'a...a', not <>
        cx.set_state("hello <woˇrld> test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d a");
        cx.assert_state("hello <woˇrld> test", Mode::HelixNormal);

        // Arbitrary chars work as symmetric pairs (Helix feature)
        cx.set_state("hello *woˇrld* test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d *");
        cx.assert_state("hello ˇworld test", Mode::HelixNormal);

        // ms (add) also doesn't use aliases - 'msb' adds literal 'b' surrounds
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s b");
        cx.assert_state("hello ˇbwborld", Mode::HelixNormal);

        // mr (replace) also doesn't use aliases
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( b");
        cx.assert_state("hello ˇbworldb test", Mode::HelixNormal);
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
