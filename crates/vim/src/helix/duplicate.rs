use std::ops::Range;

use editor::{
    DisplayPoint, MultiBufferOffset, ToPoint,
    display_map::{DisplayRow, DisplaySnapshot, ToDisplayPoint},
};
use gpui::Context;
use language::PointUtf16;
use multi_buffer::MultiBufferRow;
use text::Bias;
use ui::Window;

use crate::Vim;

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
            // The order matters, because it is recorded when the selections are added.
            if matches!(direction, Direction::Above) {
                original_selections.reverse();
            }

            for origin in original_selections {
                let origin = origin.tail()..origin.head();
                selections.push(display_point_range_to_offset_range(&origin, &map));
                let mut last_origin = origin;
                for _ in 1..=times {
                    if let Some(duplicate) =
                        find_next_valid_duplicate_space(last_origin.clone(), &map, direction)
                    {
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
) -> Option<Range<DisplayPoint>> {
    let buffer = map.buffer_snapshot();
    let start_col_utf16 = buffer
        .point_to_point_utf16(origin.start.to_point(map))
        .column;
    let end_col_utf16 = buffer.point_to_point_utf16(origin.end.to_point(map)).column;

    let mut candidate_start_row = origin.start.to_point(map).row;
    let mut candidate_end_row = origin.end.to_point(map).row;

    loop {
        match direction {
            Direction::Below => {
                if candidate_end_row >= map.max_row().0 {
                    return None;
                }
                candidate_start_row += 1;
                candidate_end_row += 1;
            }
            Direction::Above => {
                if candidate_start_row == 0 {
                    return None;
                }
                candidate_start_row = candidate_start_row.saturating_sub(1);
                candidate_end_row = candidate_end_row.saturating_sub(1);
            }
        }

        if map.is_line_folded(MultiBufferRow(candidate_start_row))
            || map.is_line_folded(MultiBufferRow(candidate_end_row))
        {
            continue;
        }

        if start_col_utf16 > buffer.line_len_utf16(MultiBufferRow(candidate_start_row))
            || end_col_utf16 > buffer.line_len_utf16(MultiBufferRow(candidate_end_row))
        {
            continue;
        }

        let candidate_start = buffer
            .point_utf16_to_point(PointUtf16::new(candidate_start_row, start_col_utf16))
            .to_display_point(map);
        let candidate_end = buffer
            .point_utf16_to_point(PointUtf16::new(candidate_end_row, end_col_utf16))
            .to_display_point(map);

        return Some(candidate_start..candidate_end);
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
    use db::indoc;
    use gpui::{AppContext, UpdateGlobal};
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
    async fn test_selection_duplication_softwrap(cx: &mut gpui::TestAppContext) {
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
                    .preferred_line_length = Some(8);
            });
        });

        cx.set_state(
            indoc! {"
            The quick brown
            foˇx jumps over the
            lazy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("C");

        cx.assert_state(
            indoc! {"
            The quick brown
            foˇx jumps over the
            laˇzy dog."},
            Mode::HelixNormal,
        );

        cx.simulate_keystrokes("alt-C");

        cx.assert_state(
            indoc! {"
            Thˇe quick brown
            foˇx jumps over the
            laˇzy dog."},
            Mode::HelixNormal,
        );
    }
}
