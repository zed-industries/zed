use std::ops::Range;

use editor::{DisplayPoint, MultiBufferOffset, display_map::DisplaySnapshot};
use gpui::Context;
use text::Bias;
use ui::Window;

use crate::Vim;

impl Vim {
    /// Creates a duplicate of every selection below it in the first place that has both its start
    /// and end
    pub(super) fn helix_duplicate_selections_below(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.duplicate_selections(
            times,
            window,
            cx,
            |prev_point| *prev_point.row_mut() += 1,
            |prev_range, map| prev_range.end.row() >= map.max_point().row(),
            false,
        );
    }

    /// Creates a duplicate of every selection above it in the first place that has both its start
    /// and end
    pub(super) fn helix_duplicate_selections_above(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.duplicate_selections(
            times,
            window,
            cx,
            |prev_point| *prev_point.row_mut() = prev_point.row().0.saturating_sub(1),
            |prev_range, _| prev_range.start.row() == DisplayPoint::zero().row(),
            true,
        );
    }

    fn duplicate_selections(
        &mut self,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
        advance_search: impl Fn(&mut DisplayPoint),
        end_search: impl Fn(&Range<DisplayPoint>, &DisplaySnapshot) -> bool,
        above: bool,
    ) {
        let times = times.unwrap_or(1);
        self.update_editor(cx, |_, editor, cx| {
            let mut selections = Vec::new();
            let map = editor.display_snapshot(cx);
            let mut original_selections = editor.selections.all_display(&map);
            // The order matters, because it is recorded when the selections are added.
            if above {
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
                        &advance_search,
                        &end_search,
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
    mut origin: Range<DisplayPoint>,
    map: &DisplaySnapshot,
    advance_search: &impl Fn(&mut DisplayPoint),
    end_search: &impl Fn(&Range<DisplayPoint>, &DisplaySnapshot) -> bool,
) -> Option<Range<DisplayPoint>> {
    while !end_search(&origin, map) {
        advance_search(&mut origin.start);
        advance_search(&mut origin.end);

        if map.clip_point(origin.start, Bias::Left) == origin.start
            && map.clip_point(origin.end, Bias::Right) == origin.end
        {
            return Some(origin);
        }
    }
    None
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
}
