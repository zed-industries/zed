use editor::{Autoscroll, Bias};
use gpui::{actions, MutableAppContext, ViewContext};
use workspace::Workspace;

use crate::{motion::Motion, state::Mode, Vim};

actions!(
    vim,
    [
        VisualDelete,
        VisualChange,
        VisualLineDelete,
        VisualLineChange
    ]
);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(change);
    cx.add_action(change_line);
    cx.add_action(delete);
    cx.add_action(delete_line);
}

pub fn visual_motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let (new_head, goal) = motion.move_point(map, selection.head(), selection.goal);
                    let new_head = map.clip_at_line_end(new_head);
                    let was_reversed = selection.reversed;
                    selection.set_head(new_head, goal);

                    if was_reversed && !selection.reversed {
                        // Head was at the start of the selection, and now is at the end. We need to move the start
                        // back by one if possible in order to compensate for this change.
                        *selection.start.column_mut() = selection.start.column().saturating_sub(1);
                        selection.start = map.clip_point(selection.start, Bias::Left);
                    } else if !was_reversed && selection.reversed {
                        // Head was at the end of the selection, and now is at the start. We need to move the end
                        // forward by one if possible in order to compensate for this change.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Left);
                    }
                });
            });
        });
    });
}

pub fn change(_: &mut Workspace, _: &VisualChange, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    if !selection.reversed {
                        // Head was at the end of the selection, and now is at the start. We need to move the end
                        // forward by one if possible in order to compensate for this change.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Left);
                    }
                });
            });
            editor.insert("", cx);
        });
        vim.switch_mode(Mode::Insert, cx);
    });
}

pub fn change_line(_: &mut Workspace, _: &VisualChange, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    selection.start = map.prev_line_boundary(selection.start.to_point(map)).1;
                    selection.end = map.next_line_boundary(selection.end.to_point(map)).1;
                });
            });
            editor.insert("", cx);
        });
        vim.switch_mode(Mode::Insert, cx);
    });
}

pub fn delete(_: &mut Workspace, _: &VisualDelete, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Normal, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    if !selection.reversed {
                        // Head was at the end of the selection, and now is at the start. We need to move the end
                        // forward by one if possible in order to compensate for this change.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Left);
                    }
                });
            });
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head();
                    cursor = map.clip_point(cursor, Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
    });
}

pub fn delete_line(_: &mut Workspace, _: &VisualChange, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.switch_mode(Mode::Normal, cx);
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    selection.start = map.prev_line_boundary(selection.start.to_point(map)).1;

                    if selection.end.row() < map.max_point().row() {
                        *selection.end.row_mut() += 1;
                        *selection.end.column_mut() = 0;
                        // Don't reset the end here
                        return;
                    } else if selection.start.row() > 0 {
                        *selection.start.row_mut() -= 1;
                        *selection.start.column_mut() = map.line_len(selection.start.row());
                    }

                    selection.end = map.next_line_boundary(selection.end.to_point(map)).1;
                });
            });
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head();
                    cursor = map.clip_point(cursor, Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, vim_test_context::VimTestContext};

    #[gpui::test]
    async fn test_enter_visual_mode(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["v", "w", "j"]).mode_after(Mode::Visual);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                The [quick brown
                fox jumps }over
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                the [lazy }dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The quick brown
                fox jumps [over
                }the lazy dog"},
        );
        let mut cx = cx.binding(["v", "b", "k"]).mode_after(Mode::Visual);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                {The q]uick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                {fox jumps over
                the l]azy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The {quick brown
                fox jumps o]ver
                the lazy dog"},
        );
    }

    #[gpui::test]
    async fn test_visual_delete(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["v", "w", "x"]);
        cx.assert("The quick |brown", "The quick| ");
        let mut cx = cx.binding(["v", "w", "j", "x"]);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                The |ver
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                the |og"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The quick brown
                fox jumps |he lazy dog"},
        );
        let mut cx = cx.binding(["v", "b", "k", "x"]);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                |uick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                |azy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The |ver
                the lazy dog"},
        );
    }

    #[gpui::test]
    async fn test_visual_change(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["v", "w", "c"]).mode_after(Mode::Insert);
        cx.assert("The quick |brown", "The quick |");
        let mut cx = cx.binding(["v", "w", "j", "c"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                The |ver
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                the |og"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The quick brown
                fox jumps |he lazy dog"},
        );
        let mut cx = cx.binding(["v", "b", "k", "c"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                |uick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                |azy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The |ver
                the lazy dog"},
        );
    }
}
