use crate::{motion::Motion, object::Object, state::Mode, Vim};
use collections::HashMap;
use editor::{display_map::ToDisplayPoint, Bias, Editor};
use gpui::actions;
use language::SelectionGoal;
use ui::ViewContext;

#[derive(PartialEq, Eq)]
pub(crate) enum IndentDirection {
    In,
    Out,
    Auto,
}

actions!(vim, [Indent, Outdent, AutoIndent]);

pub(crate) fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, _: &Indent, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        vim.store_visual_marks(cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.indent(&Default::default(), cx);
                }
                vim.restore_selection_cursors(editor, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, cx)
        }
    });

    Vim::action(editor, cx, |vim, _: &Outdent, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        vim.store_visual_marks(cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.outdent(&Default::default(), cx);
                }
                vim.restore_selection_cursors(editor, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, cx)
        }
    });

    Vim::action(editor, cx, |vim, _: &AutoIndent, cx| {
        vim.record_current_action(cx);
        let count = Vim::take_count(cx).unwrap_or(1);
        vim.store_visual_marks(cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let original_positions = vim.save_selection_starts(editor, cx);
                for _ in 0..count {
                    editor.autoindent(&Default::default(), cx);
                }
                vim.restore_selection_cursors(editor, cx, original_positions);
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, cx)
        }
    });
}

impl Vim {
    pub(crate) fn indent_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        dir: IndentDirection,
        cx: &mut ViewContext<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.transact(cx, |editor, cx| {
                let mut selection_starts: HashMap<_, _> = Default::default();
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        selection_starts.insert(selection.id, anchor);
                        motion.expand_selection(map, selection, times, false, &text_layout_details);
                    });
                });
                match dir {
                    IndentDirection::In => editor.indent(&Default::default(), cx),
                    IndentDirection::Out => editor.outdent(&Default::default(), cx),
                    IndentDirection::Auto => editor.autoindent(&Default::default(), cx),
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

    pub(crate) fn indent_object(
        &mut self,
        object: Object,
        around: bool,
        dir: IndentDirection,
        cx: &mut ViewContext<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(None, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        original_positions.insert(selection.id, anchor);
                        object.expand_selection(map, selection, around);
                    });
                });
                match dir {
                    IndentDirection::In => editor.indent(&Default::default(), cx),
                    IndentDirection::Out => editor.outdent(&Default::default(), cx),
                    IndentDirection::Auto => editor.autoindent(&Default::default(), cx),
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
