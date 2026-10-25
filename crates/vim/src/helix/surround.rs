use editor::display_map::DisplaySnapshot;
use editor::{Bias, DisplayPoint, MultiBufferOffset, movement};
use gpui::{Context, Window};
use multi_buffer::Anchor;
use text::Selection;

use crate::Vim;
use crate::object::{DelimiterRange, innermost_surrounding_pair, surrounding_markers};
use crate::surrounds::{bracket_pair_for_str_helix, surround_pair_for_char_helix};

fn surrounding_markers_containing_cursor(
    display_map: &DisplaySnapshot,
    cursor: DisplayPoint,
    open_marker: char,
    close_marker: char,
) -> Option<std::ops::Range<DisplayPoint>> {
    let range = surrounding_markers(display_map, cursor, true, true, open_marker, close_marker)?;
    let cursor_offset = cursor.to_offset(display_map, Bias::Left);
    let start_offset = range.start.to_offset(display_map, Bias::Left);
    let end_offset = range.end.to_offset(display_map, Bias::Right);

    if cursor_offset >= start_offset && cursor_offset <= end_offset {
        Some(range)
    } else {
        None
    }
}

/// The delimiter ranges of the pair surrounding the cursor: a literal search
/// for an explicit pair character, or the tree-sitter based closest pair for
/// 'm', matching `mim`/`mam`.
fn surrounding_pair_ranges(
    display_map: &DisplaySnapshot,
    cursor: DisplayPoint,
    target_char: char,
) -> Option<DelimiterRange> {
    match surround_pair_for_char_helix(target_char) {
        Some(pair) => {
            let range =
                surrounding_markers_containing_cursor(display_map, cursor, pair.open, pair.close)?;
            let open_start = range.start.to_offset(display_map, Bias::Left);
            let open_end = open_start + pair.open.len_utf8();
            let close_end = range.end.to_offset(display_map, Bias::Left);
            let close_start = close_end - pair.close.len_utf8();
            Some(DelimiterRange {
                open: open_start..open_end,
                close: close_start..close_end,
            })
        }
        None => {
            let cursor_range = cursor..movement::right(display_map, cursor);
            innermost_surrounding_pair(display_map, cursor_range)
        }
    }
}

fn selection_cursor(map: &DisplaySnapshot, selection: &Selection<DisplayPoint>) -> DisplayPoint {
    if selection.reversed || selection.is_empty() {
        selection.head()
    } else {
        editor::movement::left(map, selection.head())
    }
}

/// Anchor a pre-edit selection so it survives the surround edits with its
/// direction intact, the way Helix maps selections through a transaction.
fn preserved_selection_anchors(
    display_map: &DisplaySnapshot,
    selection: &Selection<DisplayPoint>,
) -> std::ops::Range<Anchor> {
    let range = selection.range();
    let snapshot = display_map.buffer_snapshot();
    let start = snapshot.anchor_before(range.start.to_offset(display_map, Bias::Left));
    let end = snapshot.anchor_before(range.end.to_offset(display_map, Bias::Left));
    if selection.reversed {
        end..start
    } else {
        start..end
    }
}

type SurroundEdits = Vec<(std::ops::Range<MultiBufferOffset>, String)>;

enum SurroundSelections {
    PreEditAnchors(Vec<std::ops::Range<Anchor>>),
    PostEditOffsets(Vec<std::ops::Range<MultiBufferOffset>>),
}

fn apply_helix_surround_edits<F>(
    vim: &mut Vim,
    window: &mut Window,
    cx: &mut Context<Vim>,
    mut build: F,
) where
    F: FnMut(&DisplaySnapshot, Vec<Selection<DisplayPoint>>) -> (SurroundEdits, SurroundSelections),
{
    vim.update_editor(cx, |_, editor, cx| {
        editor.transact(window, cx, |editor, window, cx| {
            editor.set_clip_at_line_ends(false, cx);

            let display_map = editor.display_snapshot(cx);
            let selections = editor.selections.all_display(&display_map);
            let (mut edits, new_selections) = build(&display_map, selections);

            // The sort is stable, so insertions at the same offset keep the
            // order they were pushed in.
            edits.sort_by_key(|edit| edit.0.start);
            editor.edit(edits, cx);

            editor.change_selections(Default::default(), window, cx, |s| match new_selections {
                SurroundSelections::PreEditAnchors(anchors) => s.select_anchor_ranges(anchors),
                SurroundSelections::PostEditOffsets(ranges) => s.select_ranges(ranges),
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
            let mut ranges = Vec::new();

            // Like Helix, the new selection covers the surrounded text
            // including the added delimiters, so that surrounds compose and
            // `i`/`a` land outside the pair. The selections are computed from
            // the insertions made before them rather than from anchors: when
            // two selections touch, both the closing delimiter of the first
            // and the opening delimiter of the second are inserted at the
            // same offset, so no anchor can end up between them.
            let mut inserted_len = 0;
            for selection in selections {
                let range = selection.range();
                let start = range.start.to_offset(display_map, Bias::Right);
                let end = range.end.to_offset(display_map, Bias::Left);

                edits.push((start..start, pair.start.clone()));
                edits.push((end..end, pair.end.clone()));

                let new_start = start + inserted_len;
                inserted_len += pair.start.len() + pair.end.len();
                let new_end = end + inserted_len;
                if selection.reversed {
                    ranges.push(new_end..new_start);
                } else {
                    ranges.push(new_start..new_end);
                }
            }

            (edits, SurroundSelections::PostEditOffsets(ranges))
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
                anchors.push(preserved_selection_anchors(display_map, &selection));

                if let Some(pair) = surrounding_pair_ranges(display_map, cursor, old_char) {
                    edits.push((pair.close, new_pair.end.clone()));
                    edits.push((pair.open, new_pair.start.clone()));
                }
            }

            (edits, SurroundSelections::PreEditAnchors(anchors))
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
                anchors.push(preserved_selection_anchors(display_map, &selection));

                if let Some(pair) = surrounding_pair_ranges(display_map, cursor, target_char) {
                    edits.push((pair.close, String::new()));
                    edits.push((pair.open, String::new()));
                }
            }

            (edits, SurroundSelections::PreEditAnchors(anchors))
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
        cx.assert_state("hello «(w)ˇ»orld", Mode::HelixNormal);

        cx.set_state("hello ˇworld", Mode::HelixNormal);
        cx.simulate_keystrokes("m s )");
        cx.assert_state("hello «(w)ˇ»orld", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s [");
        cx.assert_state("hello «[worl]ˇ»d", Mode::HelixNormal);

        cx.set_state("hello «worlˇ»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s \"");
        cx.assert_state("hello «\"worl\"ˇ»d", Mode::HelixNormal);

        // The selection direction is preserved.
        cx.set_state("hello «ˇworl»d", Mode::HelixNormal);
        cx.simulate_keystrokes("m s (");
        cx.assert_state("hello «ˇ(worl)»d", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_add_adjacent_selections(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state("«aˇ»«bˇ»«cˇ»«dˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("m s \"");
        cx.assert_state("«\"a\"ˇ»«\"b\"ˇ»«\"c\"ˇ»«\"d\"ˇ»", Mode::HelixNormal);

        cx.set_state("«ifˇ»  «ifˇ»«ifˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("m s (");
        cx.assert_state("«(if)ˇ»  «(if)ˇ»«(if)ˇ»", Mode::HelixNormal);

        cx.set_state("«ˇno»«ˇob»", Mode::HelixNormal);
        cx.simulate_keystrokes("m s [");
        cx.assert_state("«ˇ[no]»«ˇ[ob]»", Mode::HelixNormal);

        cx.set_state("«가ˇ»«나ˇ»«다ˇ»", Mode::HelixNormal);
        cx.simulate_keystrokes("m s \"");
        cx.assert_state("«\"가\"ˇ»«\"나\"ˇ»«\"다\"ˇ»", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_surround_add_composes(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // The new selection includes the added delimiters, so `i` prepends
        // before the opening delimiter: i32 -> Vec<i32>.
        cx.set_state("let x: iˇ32 = 5;", Mode::HelixNormal);
        cx.simulate_keystrokes("m i w m s <");
        cx.assert_state("let x: «<i32>ˇ» = 5;", Mode::HelixNormal);
        cx.simulate_keystrokes("i");
        cx.assert_state("let x: ˇ<i32> = 5;", Mode::Insert);
        cx.simulate_keystrokes("V e c");
        cx.assert_state("let x: Vecˇ<i32> = 5;", Mode::Insert);

        // `a` appends after the closing delimiter.
        cx.set_state("hello woˇrld test", Mode::HelixNormal);
        cx.simulate_keystrokes("m i w m s ( a");
        cx.assert_state("hello (world)ˇ test", Mode::Insert);

        // Surround adds chain, each wrapping the previous result.
        cx.set_state("hello woˇrld test", Mode::HelixNormal);
        cx.simulate_keystrokes("m i w m s \" m s (");
        cx.assert_state("hello «(\"world\")ˇ» test", Mode::HelixNormal);
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

        cx.set_state(
            indoc! {"
            \"heˇllo\"
            fn world() {
            }"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m d (");
        cx.assert_state(
            indoc! {"
            \"heˇllo\"
            fn world() {
            }"},
            Mode::HelixNormal,
        );

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("(woˇrld)", Mode::HelixNormal);

        // A non-empty selection survives the deletion.
        cx.set_state("(«woˇ»rld)", Mode::HelixNormal);
        cx.simulate_keystrokes("m d (");
        cx.assert_state("«woˇ»rld", Mode::HelixNormal);
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

        cx.set_state(
            indoc! {"
            \"heˇllo\"
            fn world() {
            }"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state(
            indoc! {"
            \"heˇllo\"
            fn world() {
            }"},
            Mode::HelixNormal,
        );

        cx.set_state("((woˇrld))", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("([woˇrld])", Mode::HelixNormal);

        // A non-empty selection survives the replacement.
        cx.set_state("(«woˇ»rld)", Mode::HelixNormal);
        cx.simulate_keystrokes("m r ( [");
        cx.assert_state("[«woˇ»rld]", Mode::HelixNormal);
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
        cx.assert_state("hello «{world}ˇ» test", Mode::HelixNormal);
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
        cx.assert_state("hello «bwbˇ»orld", Mode::HelixNormal);

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

        // 'm' matches via the language's bracket queries, so brackets inside
        // a string literal are plain text and the quotes are the closest pair.
        cx.set_state("let s = (\"a (bˇc) d\");", Mode::HelixNormal);
        cx.simulate_keystrokes("m d m");
        cx.assert_state("let s = (a (bˇc) d);", Mode::HelixNormal);

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
