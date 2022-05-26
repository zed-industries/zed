use collections::HashMap;
use editor::{display_map::ToDisplayPoint, Autoscroll, Bias};
use gpui::{actions, MutableAppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

use crate::{motion::Motion, state::Mode, utils::copy_selections_content, Vim};

actions!(vim, [VisualDelete, VisualChange, VisualYank]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(change);
    cx.add_action(delete);
    cx.add_action(yank);
}

pub fn visual_motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let (new_head, goal) = motion.move_point(map, selection.head(), selection.goal);
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
                        selection.end = map.clip_point(selection.end, Bias::Right);
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
            // Compute edits and resulting anchor selections. If in line mode, adjust
            // the anchor location and additional newline
            let mut edits = Vec::new();
            let mut new_selections = Vec::new();
            let line_mode = editor.selections.line_mode;
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if !selection.reversed {
                        // Head is at the end of the selection. Adjust the end position to
                        // to include the character under the cursor.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Right);
                    }

                    if line_mode {
                        let range = selection.map(|p| p.to_point(map)).range();
                        let expanded_range = map.expand_to_line(range);
                        // If we are at the last line, the anchor needs to be after the newline so that
                        // it is on a line of its own. Otherwise, the anchor may be after the newline
                        let anchor = if expanded_range.end == map.buffer_snapshot.max_point() {
                            map.buffer_snapshot.anchor_after(expanded_range.end)
                        } else {
                            map.buffer_snapshot.anchor_before(expanded_range.start)
                        };

                        edits.push((expanded_range, "\n"));
                        new_selections.push(selection.map(|_| anchor.clone()));
                    } else {
                        let range = selection.map(|p| p.to_point(map)).range();
                        let anchor = map.buffer_snapshot.anchor_after(range.end);
                        edits.push((range, ""));
                        new_selections.push(selection.map(|_| anchor.clone()));
                    }
                    selection.goal = SelectionGoal::None;
                });
            });
            copy_selections_content(editor, editor.selections.line_mode, cx);
            editor.edit_with_autoindent(edits, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select_anchors(new_selections);
            });
        });
        vim.switch_mode(Mode::Insert, cx);
    });
}

pub fn delete(_: &mut Workspace, _: &VisualDelete, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_columns: HashMap<_, _> = Default::default();
            let line_mode = editor.selections.line_mode;
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    if line_mode {
                        original_columns
                            .insert(selection.id, selection.head().to_point(&map).column);
                    } else if !selection.reversed {
                        // Head is at the end of the selection. Adjust the end position to
                        // to include the character under the cursor.
                        *selection.end.column_mut() = selection.end.column() + 1;
                        selection.end = map.clip_point(selection.end, Bias::Right);
                    }
                    selection.goal = SelectionGoal::None;
                });
            });
            copy_selections_content(editor, line_mode, cx);
            editor.insert("", cx);

            // Fixup cursor position after the deletion
            editor.set_clip_at_line_ends(true, cx);
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head().to_point(map);

                    if let Some(column) = original_columns.get(&selection.id) {
                        cursor.column = *column
                    }
                    let cursor = map.clip_point(cursor.to_display_point(map), Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
        vim.switch_mode(Mode::Normal, cx);
    });
}

pub fn yank(_: &mut Workspace, _: &VisualYank, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let line_mode = editor.selections.line_mode;
            if !editor.selections.line_mode {
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        if !selection.reversed {
                            // Head is at the end of the selection. Adjust the end position to
                            // to include the character under the cursor.
                            *selection.end.column_mut() = selection.end.column() + 1;
                            selection.end = map.clip_point(selection.end, Bias::Right);
                        }
                    });
                });
            }
            copy_selections_content(editor, line_mode, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    selection.collapse_to(selection.start, SelectionGoal::None)
                });
            });
        });
        vim.switch_mode(Mode::Normal, cx);
    });
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{state::Mode, vim_test_context::VimTestContext};

    #[gpui::test]
    async fn test_enter_visual_mode(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx
            .binding(["v", "w", "j"])
            .mode_after(Mode::Visual { line: false });
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
        let mut cx = cx
            .binding(["v", "b", "k"])
            .mode_after(Mode::Visual { line: false });
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
        // Test pasting code copied on delete
        cx.simulate_keystrokes(["j", "p"]);
        cx.assert_editor_state(indoc! {"
            The ver
            the l|quick brown
            fox jumps oazy dog"});

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
    async fn test_visual_line_delete(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-V", "x"]);
        cx.assert(
            indoc! {"
                The qu|ick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                fox ju|mps over
                the lazy dog"},
        );
        // Test pasting code copied on delete
        cx.simulate_keystroke("p");
        cx.assert_editor_state(indoc! {"
            fox jumps over
            |The quick brown
            the lazy dog"});

        cx.assert(
            indoc! {"
                The quick brown
                fox ju|mps over
                the lazy dog"},
            indoc! {"
                The quick brown
                the la|zy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the la|zy dog"},
            indoc! {"
                The quick brown
                fox ju|mps over"},
        );
        let mut cx = cx.binding(["shift-V", "j", "x"]);
        cx.assert(
            indoc! {"
                The qu|ick brown
                fox jumps over
                the lazy dog"},
            "the la|zy dog",
        );
        // Test pasting code copied on delete
        cx.simulate_keystroke("p");
        cx.assert_editor_state(indoc! {"
            the lazy dog
            |The quick brown
            fox jumps over"});

        cx.assert(
            indoc! {"
                The quick brown
                fox ju|mps over
                the lazy dog"},
            "The qu|ick brown",
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the la|zy dog"},
            indoc! {"
                The quick brown
                fox ju|mps over"},
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

    #[gpui::test]
    async fn test_visual_line_change(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["shift-V", "c"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The qu|ick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                |
                fox jumps over
                the lazy dog"},
        );
        // Test pasting code copied on change
        cx.simulate_keystrokes(["escape", "j", "p"]);
        cx.assert_editor_state(indoc! {"
            
            fox jumps over
            |The quick brown
            the lazy dog"});

        cx.assert(
            indoc! {"
                The quick brown
                fox ju|mps over
                the lazy dog"},
            indoc! {"
                The quick brown
                |
                the lazy dog"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the la|zy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                |"},
        );
        let mut cx = cx.binding(["shift-V", "j", "c"]).mode_after(Mode::Insert);
        cx.assert(
            indoc! {"
                The qu|ick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                |
                the lazy dog"},
        );
        // Test pasting code copied on delete
        cx.simulate_keystrokes(["escape", "j", "p"]);
        cx.assert_editor_state(indoc! {"
            
            the lazy dog
            |The quick brown
            fox jumps over"});
        cx.assert(
            indoc! {"
                The quick brown
                fox ju|mps over
                the lazy dog"},
            indoc! {"
                The quick brown
                |"},
        );
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the la|zy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                |"},
        );
    }

    #[gpui::test]
    async fn test_visual_yank(cx: &mut gpui::TestAppContext) {
        let cx = VimTestContext::new(cx, true).await;
        let mut cx = cx.binding(["v", "w", "y"]);
        cx.assert("The quick |brown", "The quick |brown");
        cx.assert_clipboard_content(Some("brown"));
        let mut cx = cx.binding(["v", "w", "j", "y"]);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            quick brown
            fox jumps o"}));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
        );
        cx.assert_clipboard_content(Some("lazy d"));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
                over
                t"}));
        let mut cx = cx.binding(["v", "b", "k", "y"]);
        cx.assert(
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
            indoc! {"
                |The quick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some("The q"));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps over
                the |lazy dog"},
            indoc! {"
                The quick brown
                |fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            fox jumps over
            the l"}));
        cx.assert(
            indoc! {"
                The quick brown
                fox jumps |over
                the lazy dog"},
            indoc! {"
                The |quick brown
                fox jumps over
                the lazy dog"},
        );
        cx.assert_clipboard_content(Some(indoc! {"
            quick brown
            fox jumps o"}));
    }
}
