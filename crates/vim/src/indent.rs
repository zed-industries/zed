use crate::{Vim, motion::Motion, object::Object, state::Mode};
use collections::HashMap;
use editor::SelectionEffects;
use editor::{Bias, Editor, display_map::ToDisplayPoint};
use gpui::actions;
use gpui::{Context, Window};
use language::SelectionGoal;

#[derive(PartialEq, Eq)]
pub(crate) enum IndentDirection {
    In,
    Out,
    Auto,
}

actions!(
    vim,
    [
        /// Increases indentation of selected lines.
        Indent,
        /// Decreases indentation of selected lines.
        Outdent,
        /// Automatically adjusts indentation based on syntax.
        AutoIndent
    ]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &Indent, window, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        vim.store_visual_marks(window, cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.indent(&Default::default(), window, cx);
                }
                vim.restore_selection_cursors(editor, window, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, window, cx)
        }
    });

    Vim::action(editor, cx, |vim, _: &Outdent, window, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        vim.store_visual_marks(window, cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.outdent(&Default::default(), window, cx);
                }
                vim.restore_selection_cursors(editor, window, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, window, cx)
        }
    });

    Vim::action(editor, cx, |vim, _: &AutoIndent, window, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        vim.store_visual_marks(window, cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.autoindent(&Default::default(), window, cx);
                }
                vim.restore_selection_cursors(editor, window, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, window, cx)
        }
    });
}

impl Vim {
    pub(crate) fn indent_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        forced_motion: bool,
        dir: IndentDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.transact(window, cx, |editor, window, cx| {
                let mut selection_starts: HashMap<_, _> = Default::default();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        selection_starts.insert(selection.id, anchor);
                        motion.expand_selection(
                            map,
                            selection,
                            times,
                            &text_layout_details,
                            forced_motion,
                        );
                    });
                });
                match dir {
                    IndentDirection::In => editor.indent(&Default::default(), window, cx),
                    IndentDirection::Out => editor.outdent(&Default::default(), window, cx),
                    IndentDirection::Auto => editor.autoindent(&Default::default(), window, cx),
                }
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = selection_starts.remove(&selection.id).unwrap();
                        selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                    });
                });
            });
        });
    }

    pub(crate) fn indent_object(
        &mut self,
        object: Object,
        around: bool,
        dir: IndentDirection,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        original_positions.insert(selection.id, anchor);
                        object.expand_selection(map, selection, around, times);
                    });
                });
                match dir {
                    IndentDirection::In => editor.indent(&Default::default(), window, cx),
                    IndentDirection::Out => editor.outdent(&Default::default(), window, cx),
                    IndentDirection::Auto => editor.autoindent(&Default::default(), window, cx),
                }
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = original_positions.remove(&selection.id).unwrap();
                        selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                    });
                });
            });
        });
    }
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use indoc::indoc;

    #[gpui::test]
    async fn test_indent_gv(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_neovim_option("shiftwidth=4").await;

        cx.set_shared_state("ˇhello\nworld\n").await;
        cx.simulate_shared_keystrokes("v j > g v").await;
        cx.shared_state()
            .await
            .assert_eq("«    hello\n ˇ»   world\n");
    }

    #[gpui::test]
    async fn test_autoindent_op(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc!(
                "
            fn a() {
                b();
                c();

                    d();
                    ˇe();
                    f();

                g();
            }
        "
            ),
            Mode::Normal,
        );

        cx.simulate_keystrokes("= a p");
        cx.assert_state(
            indoc!(
                "
                fn a() {
                    b();
                    c();

                    d();
                    ˇe();
                    f();

                    g();
                }
            "
            ),
            Mode::Normal,
        );
    }
}
