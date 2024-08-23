use collections::HashMap;
use editor::{display_map::ToDisplayPoint, scroll::Autoscroll};
use gpui::ViewContext;
use language::{Bias, Point, SelectionGoal};
use multi_buffer::MultiBufferRow;

use crate::{
    motion::Motion,
    normal::{ChangeCase, ConvertToLowerCase, ConvertToUpperCase},
    object::Object,
    state::Mode,
    Vim,
};

pub enum CaseTarget {
    Lowercase,
    Uppercase,
    OppositeCase,
}

impl Vim {
    pub fn change_case_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        mode: CaseTarget,
        cx: &mut ViewContext<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                let mut selection_starts: HashMap<_, _> = Default::default();
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Left);
                        selection_starts.insert(selection.id, anchor);
                        motion.expand_selection(map, selection, times, false, &text_layout_details);
                    });
                });
                match mode {
                    CaseTarget::Lowercase => editor.convert_to_lower_case(&Default::default(), cx),
                    CaseTarget::Uppercase => editor.convert_to_upper_case(&Default::default(), cx),
                    CaseTarget::OppositeCase => {
                        editor.convert_to_opposite_case(&Default::default(), cx)
                    }
                }
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = selection_starts.remove(&selection.id).unwrap();
                        selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                    });
                });
            });
        });
    }

    pub fn change_case_object(
        &mut self,
        object: Object,
        around: bool,
        mode: CaseTarget,
        cx: &mut ViewContext<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        object.expand_selection(map, selection, around);
                        original_positions.insert(
                            selection.id,
                            map.display_point_to_anchor(selection.start, Bias::Left),
                        );
                    });
                });
                match mode {
                    CaseTarget::Lowercase => editor.convert_to_lower_case(&Default::default(), cx),
                    CaseTarget::Uppercase => editor.convert_to_upper_case(&Default::default(), cx),
                    CaseTarget::OppositeCase => {
                        editor.convert_to_opposite_case(&Default::default(), cx)
                    }
                }
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = original_positions.remove(&selection.id).unwrap();
                        selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                    });
                });
            });
        });
    }

    pub fn change_case(&mut self, _: &ChangeCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<Vec<char>>()
            } else {
                c.to_lowercase().collect::<Vec<char>>()
            }
        })
    }

    pub fn convert_to_upper_case(&mut self, _: &ConvertToUpperCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |c| c.to_uppercase().collect::<Vec<char>>())
    }

    pub fn convert_to_lower_case(&mut self, _: &ConvertToLowerCase, cx: &mut ViewContext<Self>) {
        self.manipulate_text(cx, |c| c.to_lowercase().collect::<Vec<char>>())
    }

    fn manipulate_text<F>(&mut self, cx: &mut ViewContext<Self>, transform: F)
    where
        F: Fn(char) -> Vec<char> + Copy,
    {
        self.record_current_action(cx);
        self.store_visual_marks(cx);
        let count = self.take_count(cx).unwrap_or(1) as u32;

        self.update_editor(cx, |vim, editor, cx| {
            let mut ranges = Vec::new();
            let mut cursor_positions = Vec::new();
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            for selection in editor.selections.all::<Point>(cx) {
                match vim.mode {
                    Mode::VisualLine => {
                        let start = Point::new(selection.start.row, 0);
                        let end = Point::new(
                            selection.end.row,
                            snapshot.line_len(MultiBufferRow(selection.end.row)),
                        );
                        ranges.push(start..end);
                        cursor_positions.push(start..start);
                    }
                    Mode::Visual => {
                        ranges.push(selection.start..selection.end);
                        cursor_positions.push(selection.start..selection.start);
                    }
                    Mode::VisualBlock => {
                        ranges.push(selection.start..selection.end);
                        if cursor_positions.len() == 0 {
                            cursor_positions.push(selection.start..selection.start);
                        }
                    }
                    Mode::Insert | Mode::Normal | Mode::Replace => {
                        let start = selection.start;
                        let mut end = start;
                        for _ in 0..count {
                            end = snapshot.clip_point(end + Point::new(0, 1), Bias::Right);
                        }
                        ranges.push(start..end);

                        if end.column == snapshot.line_len(MultiBufferRow(end.row)) {
                            end = snapshot.clip_point(end - Point::new(0, 1), Bias::Left);
                        }
                        cursor_positions.push(end..end)
                    }
                }
            }
            editor.transact(cx, |editor, cx| {
                for range in ranges.into_iter().rev() {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor.buffer().update(cx, |buffer, cx| {
                        let text = snapshot
                            .text_for_range(range.start..range.end)
                            .flat_map(|s| s.chars())
                            .flat_map(|c| transform(c))
                            .collect::<String>();

                        buffer.edit([(range, text)], None, cx)
                    })
                }
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges(cursor_positions)
                })
            });
        });
        self.switch_mode(Mode::Normal, true, cx)
    }
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::NeovimBackedTestContext};

    #[gpui::test]
    async fn test_change_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("ˇabC\n").await;
        cx.simulate_shared_keystrokes("~").await;
        cx.shared_state().await.assert_eq("AˇbC\n");
        cx.simulate_shared_keystrokes("2 ~").await;
        cx.shared_state().await.assert_eq("ABˇc\n");

        // works in visual mode
        cx.set_shared_state("a😀C«dÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes("~").await;
        cx.shared_state().await.assert_eq("a😀CˇDé1*F\n");

        // works with multibyte characters
        cx.simulate_shared_keystrokes("~").await;
        cx.set_shared_state("aˇC😀é1*F\n").await;
        cx.simulate_shared_keystrokes("4 ~").await;
        cx.shared_state().await.assert_eq("ac😀É1ˇ*F\n");

        // works with line selections
        cx.set_shared_state("abˇC\n").await;
        cx.simulate_shared_keystrokes("shift-v ~").await;
        cx.shared_state().await.assert_eq("ˇABc\n");

        // works in visual block mode
        cx.set_shared_state("ˇaa\nbb\ncc").await;
        cx.simulate_shared_keystrokes("ctrl-v j ~").await;
        cx.shared_state().await.assert_eq("ˇAa\nBb\ncc");

        // works with multiple cursors (zed only)
        cx.set_state("aˇßcdˇe\n", Mode::Normal);
        cx.simulate_keystrokes("~");
        cx.assert_state("aSSˇcdˇE\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_convert_to_upper_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // works in visual mode
        cx.set_shared_state("a😀C«dÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes("U").await;
        cx.shared_state().await.assert_eq("a😀CˇDÉ1*F\n");

        // works with line selections
        cx.set_shared_state("abˇC\n").await;
        cx.simulate_shared_keystrokes("shift-v U").await;
        cx.shared_state().await.assert_eq("ˇABC\n");

        // works in visual block mode
        cx.set_shared_state("ˇaa\nbb\ncc").await;
        cx.simulate_shared_keystrokes("ctrl-v j U").await;
        cx.shared_state().await.assert_eq("ˇAa\nBb\ncc");
    }

    #[gpui::test]
    async fn test_convert_to_lower_case(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // works in visual mode
        cx.set_shared_state("A😀c«DÉ1*fˇ»\n").await;
        cx.simulate_shared_keystrokes("u").await;
        cx.shared_state().await.assert_eq("A😀cˇdé1*f\n");

        // works with line selections
        cx.set_shared_state("ABˇc\n").await;
        cx.simulate_shared_keystrokes("shift-v u").await;
        cx.shared_state().await.assert_eq("ˇabc\n");

        // works in visual block mode
        cx.set_shared_state("ˇAa\nBb\nCc").await;
        cx.simulate_shared_keystrokes("ctrl-v j u").await;
        cx.shared_state().await.assert_eq("ˇaa\nbb\nCc");
    }

    #[gpui::test]
    async fn test_change_case_motion(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // works in visual mode
        cx.set_shared_state("ˇabc def").await;
        cx.simulate_shared_keystrokes("g shift-u w").await;
        cx.shared_state().await.assert_eq("ˇABC def");

        cx.simulate_shared_keystrokes("g u w").await;
        cx.shared_state().await.assert_eq("ˇabc def");

        cx.simulate_shared_keystrokes("g ~ w").await;
        cx.shared_state().await.assert_eq("ˇABC def");

        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("ˇabc def");

        cx.set_shared_state("abˇc def").await;
        cx.simulate_shared_keystrokes("g ~ i w").await;
        cx.shared_state().await.assert_eq("ˇABC def");

        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("ˇabc def");
    }
}
