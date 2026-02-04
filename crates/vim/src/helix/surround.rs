use editor::display_map::DisplaySnapshot;
use editor::{Bias, DisplayPoint, MultiBufferOffset};
use gpui::{Context, Window};
use multi_buffer::Anchor;
use text::Selection;

use crate::Vim;
use crate::object::surrounding_markers;
use crate::surrounds::{SURROUND_PAIRS, bracket_pair_for_str_helix, surround_pair_for_char_helix};

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

fn selection_cursor(map: &DisplaySnapshot, selection: &Selection<DisplayPoint>) -> DisplayPoint {
    if selection.reversed || selection.is_empty() {
        selection.head()
    } else {
        editor::movement::left(map, selection.head())
    }
}

type SurroundEdits = Vec<(std::ops::Range<MultiBufferOffset>, String)>;
type SurroundAnchors = Vec<std::ops::Range<Anchor>>;

fn apply_helix_surround_edits<F>(
    vim: &mut Vim,
    window: &mut Window,
    cx: &mut Context<Vim>,
    mut build: F,
) where
    F: FnMut(&DisplaySnapshot, Vec<Selection<DisplayPoint>>) -> (SurroundEdits, SurroundAnchors),
{
    vim.update_editor(cx, |_, editor, cx| {
        editor.transact(window, cx, |editor, window, cx| {
            editor.set_clip_at_line_ends(false, cx);

            let display_map = editor.display_snapshot(cx);
            let selections = editor.selections.all_display(&display_map);
            let (mut edits, anchors) = build(&display_map, selections);

            edits.sort_by(|a, b| b.0.start.cmp(&a.0.start));
            editor.edit(edits, cx);

            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_anchor_ranges(anchors);
            });
            editor.set_clip_at_line_ends(true, cx);
        });
    });
}

impl Vim {
    /// ms - Add surrounding characters around selection.
    pub fn helix_surround_add(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.stop_recording(cx);

        let pair = bracket_pair_for_str_helix(text);

        apply_helix_surround_edits(self, window, cx, |display_map, selections| {
            let mut edits = Vec::new();
            let mut anchors = Vec::new();

            for selection in selections {
                let range = selection.range();
                let start = range.start.to_offset(display_map, Bias::Right);
                let end = range.end.to_offset(display_map, Bias::Left);

                let end_anchor = display_map.buffer_snapshot().anchor_before(end);
                edits.push((end..end, pair.end.clone()));
                edits.push((start..start, pair.start.clone()));
                anchors.push(end_anchor..end_anchor);
            }

            (edits, anchors)
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
        let new_pair = bracket_pair_for_str_helix(&new_char_str);

        apply_helix_surround_edits(self, window, cx, |display_map, selections| {
            let mut edits: Vec<(std::ops::Range<MultiBufferOffset>, String)> = Vec::new();
            let mut anchors = Vec::new();

            for selection in selections {
                let cursor = selection_cursor(display_map, &selection);

                // For 'm', find the nearest surrounding pair
                let markers = match surround_pair_for_char_helix(old_char) {
                    Some(pair) => Some((pair.open, pair.close)),
                    None => find_nearest_surrounding_pair(display_map, cursor),
                };

                let Some((open_marker, close_marker)) = markers else {
                    let offset = selection.head().to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(offset);
                    anchors.push(anchor..anchor);
                    continue;
                };

                if let Some(range) =
                    surrounding_markers(display_map, cursor, true, true, open_marker, close_marker)
                {
                    let open_start = range.start.to_offset(display_map, Bias::Left);
                    let open_end = open_start + open_marker.len_utf8();
                    let close_end = range.end.to_offset(display_map, Bias::Left);
                    let close_start = close_end - close_marker.len_utf8();

                    edits.push((close_start..close_end, new_pair.end.clone()));
                    edits.push((open_start..open_end, new_pair.start.clone()));

                    let cursor_offset = cursor.to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(cursor_offset);
                    anchors.push(anchor..anchor);
                } else {
                    let offset = selection.head().to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(offset);
                    anchors.push(anchor..anchor);
                }
            }

            (edits, anchors)
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

        apply_helix_surround_edits(self, window, cx, |display_map, selections| {
            let mut edits: Vec<(std::ops::Range<MultiBufferOffset>, String)> = Vec::new();
            let mut anchors = Vec::new();

            for selection in selections {
                let cursor = selection_cursor(display_map, &selection);

                // For 'm', find the nearest surrounding pair
                let markers = match surround_pair_for_char_helix(target_char) {
                    Some(pair) => Some((pair.open, pair.close)),
                    None => find_nearest_surrounding_pair(display_map, cursor),
                };

                let Some((open_marker, close_marker)) = markers else {
                    let offset = selection.head().to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(offset);
                    anchors.push(anchor..anchor);
                    continue;
                };

                if let Some(range) =
                    surrounding_markers(display_map, cursor, true, true, open_marker, close_marker)
                {
                    let open_start = range.start.to_offset(display_map, Bias::Left);
                    let open_end = open_start + open_marker.len_utf8();
                    let close_end = range.end.to_offset(display_map, Bias::Left);
                    let close_start = close_end - close_marker.len_utf8();

                    edits.push((close_start..close_end, String::new()));
                    edits.push((open_start..open_end, String::new()));

                    let cursor_offset = cursor.to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(cursor_offset);
                    anchors.push(anchor..anchor);
                } else {
                    let offset = selection.head().to_offset(display_map, Bias::Left);
                    let anchor = display_map.buffer_snapshot().anchor_before(offset);
                    anchors.push(anchor..anchor);
                }
            }

            (edits, anchors)
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
        cx.assert_state("hello (wˇ)orld", Mode::HelixNormal);

        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s )");
        cx.assert_state("hello (wˇ)orld", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s [");
        cx.assert_state("hello [worlˇ]d", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s \"");
        cx.assert_state("hello \"worlˇ\"d", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("hello \"woˇrld\" test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d \"");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("hello woˇrld test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("(woˇrld)", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_replace(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("hello [woˇrld] test", Mode::HelixNormal);

        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( ]");
        cx.assert_state("hello [woˇrld] test", Mode::HelixNormal);

        cx.set_state("hello \"woˇrld\" test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r \" {");
        cx.assert_state("hello {woˇrld} test", Mode::HelixNormal);

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("([woˇrld])", Mode::HelixNormal);
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
            function test() 
                return ˇvalue;
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
        cx.assert_state("hello {worldˇ} test", Mode::HelixNormal);
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
            heˇllo
            woˇrld"},
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
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        // ms (add) also doesn't use aliases - 'msb' adds literal 'b' surrounds
        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s b");
        cx.assert_state("hello bwˇborld", Mode::HelixNormal);

        // mr (replace) also doesn't use aliases
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( b");
        cx.assert_state("hello bwoˇrldb test", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_match_nearest(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // mdm - delete nearest surrounding pair
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("hello [woˇrld] test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("hello woˇrld test", Mode::HelixNormal);

        // Nested - deletes innermost
        cx.set_state("([woˇrld])", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("(woˇrld)", Mode::HelixNormal);

        // mrm - replace nearest surrounding pair
        cx.set_state("hello (woˇrld) test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m [");
        cx.assert_state("hello [woˇrld] test", Mode::HelixNormal);

        cx.set_state("hello {woˇrld} test", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m (");
        cx.assert_state("hello (woˇrld) test", Mode::HelixNormal);

        // Nested - replaces innermost
        cx.set_state("([woˇrld])", Mode::HelixNormal);
        cx.simulate_keystrokes("m r m {");
        cx.assert_state("({woˇrld})", Mode::HelixNormal);
    }
}
