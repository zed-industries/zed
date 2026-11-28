use std::ops::Range;

use editor::{
    DisplayPoint, MultiBufferOffset,
    display_map::{DisplaySnapshot, FoldPoint},
    movement::TextLayoutDetails,
};
use gpui::Context;
use multi_buffer::MultiBufferRow;
use text::{Bias, SelectionGoal};
use ui::Window;

use crate::{Vim, motion::up_down_buffer_rows};

#[derive(Copy, Clone)]
enum Direction {
    Above,
    Below,
}

impl Vim {
    /// Creates a duplicate of every selection below it in the first place that has both its start
    /// and end
    pub(super) fn helix_duplicate_selections_below(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.duplicate_selections(times, window, cx, Direction::Below);
    }

    /// Creates a duplicate of every selection above it in the first place that has both its start
    /// and end
    pub(super) fn helix_duplicate_selections_above(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.duplicate_selections(times, window, cx, Direction::Above);
    }

    fn duplicate_selections(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        direction: Direction,
    ) {
        let times = times.unwrap_or(1);
        self.update_editor(cx, |_, editor, cx| {
            let mut selections = Vec::new();
            let map = editor.display_snapshot(cx);
            let mut original_selections = editor.selections.all_display(&map);
            let text_layout_details = editor.text_layout_details(window, cx);
            // The order matters, because it is recorded when the selections are added.
            if matches!(direction, Direction::Above) {
                original_selections.reverse();
            }

            for origin in original_selections {
                let origin = origin.tail()..origin.head();
                selections.push(display_point_range_to_offset_range(&origin, &map));
                let mut last_origin = origin;
                for _ in 1..=times {
                    if let Some(duplicate) = find_next_valid_duplicate_space(
                        last_origin.clone(),
                        &map,
                        direction,
                        &text_layout_details,
                    ) {
                        selections.push(display_point_range_to_offset_range(&duplicate, &map));
                        last_origin = duplicate;
                    } else {
                        break;
                    }
                }
            }

            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges(selections);
            });
        });
    }
}

fn find_next_valid_duplicate_space(
    origin: Range<DisplayPoint>,
    map: &DisplaySnapshot,
    direction: Direction,
    text_layout_details: &TextLayoutDetails,
) -> Option<Range<DisplayPoint>> {
    // Keep each endpoint on the same soft-wrapped subrow as the original.
    let wrapped_row_for = |point: DisplayPoint| {
        let fold_point = map.display_point_to_fold_point(point, Bias::Left);
        let begin_folded_line = map.fold_point_to_display_point(
            map.fold_snapshot()
                .clip_point(FoldPoint::new(fold_point.row(), 0), Bias::Left),
        );
        point.row().0 - begin_folded_line.row().0
    };

    let start_wrapped_row = wrapped_row_for(origin.start);
    let end_wrapped_row = wrapped_row_for(origin.end);

    let start_x = map.x_for_display_point(origin.start, text_layout_details);
    let end_x = map.x_for_display_point(origin.end, text_layout_details);

    let times = match direction {
        Direction::Above => -1,
        Direction::Below => 1,
    };

    let mut candidate = origin.clone();
    let mut start_goal = SelectionGoal::None;
    let mut end_goal = SelectionGoal::None;

    // Rendered x includes inlay text, so preserve the buffer column when inlays shift it.
    let preserve_buffer_column_across_inlays =
        |origin: DisplayPoint, candidate: DisplayPoint, bias: Bias| {
            let origin_buffer_point = map.display_point_to_point(origin, bias);
            let candidate_buffer_point = map.display_point_to_point(candidate, bias);

            let origin_inlay_point = map.inlay_snapshot().to_inlay_point(origin_buffer_point);
            let candidate_inlay_point = map.inlay_snapshot().to_inlay_point(candidate_buffer_point);

            let has_inlay_before_origin = origin_inlay_point.0.column > origin_buffer_point.column;
            let has_inlay_before_candidate =
                candidate_inlay_point.0.column > candidate_buffer_point.column;

            if origin_buffer_point.column == candidate_buffer_point.column
                || !(has_inlay_before_origin || has_inlay_before_candidate)
            {
                return candidate;
            }

            // NOTE: This byte-column fallback can misalign tabs before the endpoint.
            let mut target_buffer_point = candidate_buffer_point;
            target_buffer_point.column = origin_buffer_point.column.min(
                map.buffer_snapshot()
                    .line_len(MultiBufferRow(target_buffer_point.row)),
            );
            map.point_to_display_point(target_buffer_point, bias)
        };

    loop {
        let previous_boundary = match direction {
            Direction::Above => candidate.start,
            Direction::Below => candidate.end,
        };
        let (candidate_start, next_start_goal) =
            up_down_buffer_rows(map, candidate.start, start_goal, times, text_layout_details);
        let (candidate_end, next_end_goal) =
            up_down_buffer_rows(map, candidate.end, end_goal, times, text_layout_details);

        let candidate_start =
            preserve_buffer_column_across_inlays(origin.start, candidate_start, Bias::Left);
        let candidate_end =
            preserve_buffer_column_across_inlays(origin.end, candidate_end, Bias::Right);
        candidate = candidate_start..candidate_end;

        start_goal = next_start_goal;
        end_goal = next_end_goal;

        let boundary = match direction {
            Direction::Above => candidate.start,
            Direction::Below => candidate.end,
        };

        if map
            .display_point_to_fold_point(previous_boundary, Bias::Left)
            .row()
            == map.display_point_to_fold_point(boundary, Bias::Left).row()
        {
            return None;
        }

        if wrapped_row_for(candidate.start) != start_wrapped_row
            || wrapped_row_for(candidate.end) != end_wrapped_row
        {
            continue;
        }

        let start_row_end_x = map.x_for_display_point(
            DisplayPoint::new(candidate.start.row(), map.line_len(candidate.start.row())),
            text_layout_details,
        );
        let end_row_end_x = map.x_for_display_point(
            DisplayPoint::new(candidate.end.row(), map.line_len(candidate.end.row())),
            text_layout_details,
        );

        if start_x > start_row_end_x || end_x > end_row_end_x {
            continue;
        }

        return Some(candidate);
    }
}

fn display_point_range_to_offset_range(
    range: &Range<DisplayPoint>,
    map: &DisplaySnapshot,
) -> Range<MultiBufferOffset> {
    range.start.to_offset(map, Bias::Left)..range.end.to_offset(map, Bias::Right)
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use db::indoc;
    use editor::{Inlay, MultiBufferOffset};
    use settings::SettingsStore;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_selection_duplication(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            The quick brown
            fox «jumpsˇ»
            over the
            lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox «jumpsˇ»
            over the
            lazy« dog.ˇ»"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
            The quick brown
            fox «jumpsˇ»
            over the
            lazy« dog.ˇ»"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("alt-C");

        cx.assert_state(
            indoc! {"
            The «quickˇ» brown
            fox «jumpsˇ»
            over the
            lazy« dog.ˇ»"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes(",");

        cx.assert_state(
            indoc! {"
            The «quickˇ» brown
            fox jumps
            over the
            lazy dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_backwards(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            The quick brown
            «ˇfox» jumps
            over the
            lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C C alt-C");

        cx.assert_state(
            indoc! {"
            «ˇThe» quick brown
            «ˇfox» jumps
            «ˇove»r the
            «ˇlaz»y dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_count(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
            The «qˇ»uick brown
            fox jumps
            over the
            lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("9 C");

        cx.assert_state(
            indoc! {"
            The «qˇ»uick brown
            fox «jˇ»umps
            over« ˇ»the
            lazy« ˇ»dog."},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_multiline_multibyte(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Multiline selection on rows with multibyte chars should preserve
        // the visual column on both start and end rows.
        cx.set_state(
            indoc! {"
            «Häˇ»llo
            Hëllo
            Hallo"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
            «Häˇ»llo
            «Hëˇ»llo
            Hallo"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_multibyte(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        // Selection on a line with multibyte chars should duplicate to the
        // same character column on the next line, not skip it.
        cx.set_state(
            indoc! {"
            H«äˇ»llo
            Hallo"},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
            H«äˇ»llo
            H«aˇ»llo"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_with_inlay_hints(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();

        cx.set_state(
            indoc! {"
               let x = «1ˇ»;
               let y = 2;"},
            Mode::HelixNormal,
        );

        cx.update_editor(|editor, window, cx| {
            let buffer = &editor.snapshot(window, cx).buffer;
            editor.splice_inlays(
                &[],
                vec![
                    Inlay::mock_hint(0, buffer.anchor_after(MultiBufferOffset(5)), ": i32"),
                    Inlay::mock_hint(1, buffer.anchor_after(MultiBufferOffset(16)), ": i32"),
                ],
                cx,
            );
        });

        cx.simulate_keystrokes("C");

        assert_eq!(
            cx.display_text(),
            "let x: i32 = 1;
let y: i32 = 2;",
        );

        cx.assert_state(
            indoc! {"
               let x = «1ˇ»;
               let y = «2ˇ»;"},
            Mode::HelixNormal,
        );

        cx.set_state(
            indoc! {"
               let x «=ˇ» 1;
               let xyz = 2;"},
            Mode::HelixNormal,
        );

        cx.update_editor(|editor, window, cx| {
            let buffer = &editor.snapshot(window, cx).buffer;
            editor.splice_inlays(
                &[],
                vec![
                    Inlay::mock_hint(0, buffer.anchor_after(MultiBufferOffset(5)), ": i32"),
                    Inlay::mock_hint(1, buffer.anchor_after(MultiBufferOffset(18)), ": i32"),
                ],
                cx,
            );
        });

        cx.simulate_keystrokes("C");

        assert_eq!(
            cx.display_text(),
            "let x: i32 = 1;
let xyz: i32 = 2;",
        );

        cx.assert_state(
            indoc! {"
               let x «=ˇ» 1;
               let xy«zˇ» = 2;"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_with_tab(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.update_global(|settings: &mut SettingsStore, cx| {
            settings.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.hard_tabs = Some(true);
            });
        });

        cx.set_state(
            indoc! {"
                \t1234«5ˇ»
                \t\t5
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
                \t1234«5ˇ»
                \t\t«5ˇ»
            "},
            Mode::HelixNormal,
        );

        cx.update_global(|settings: &mut SettingsStore, cx| {
            settings.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.tab_size = NonZero::new(1);
            });
        });

        cx.set_state(
            indoc! {"
                \t1«2ˇ»345
                \t\t2
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
                \t1«2ˇ»345
                \t\t«2ˇ»
            "},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_with_tab_and_inlay_hint(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.update_global(|settings: &mut SettingsStore, cx| {
            settings.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.hard_tabs = Some(true);
            });
        });

        cx.set_state(
            indoc! {"
                \t12345«6ˇ»
                \t\t56"},
            Mode::HelixNormal,
        );

        cx.update_editor(|editor, window, cx| {
            let buffer = &editor.snapshot(window, cx).buffer;
            editor.splice_inlays(
                &[],
                vec![
                    Inlay::mock_hint(0, buffer.anchor_after(MultiBufferOffset(6)), ": foo"),
                    Inlay::mock_hint(1, buffer.anchor_after(MultiBufferOffset(11)), ": foo"),
                ],
                cx,
            );
        });

        assert_eq!(
            cx.display_text(),
            "    12345: foo6
        5: foo6"
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
                \t12345«6ˇ»
                \t\t5«6ˇ»"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_selection_duplication_with_softwrap(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.enable_helix();
        cx.update_global(|settings: &mut SettingsStore, cx| {
            settings.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.soft_wrap =
                    Some(settings::SoftWrap::Bounded);
                settings
                    .project
                    .all_languages
                    .defaults
                    .preferred_line_length = Some(12);
            });
        });

        cx.set_state(
            indoc! {"
                12345678901234567890
                1234567890123«4ˇ»567890
                12345678901234567890
                12345678901234567890
                12345678901234567890
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("8 C");

        cx.assert_state(
            indoc! {"
                12345678901234567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
            "},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("alt-C");

        cx.assert_state(
            indoc! {"
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
                1234567890123«4ˇ»567890
            "},
            Mode::HelixNormal,
        );
    }
}
