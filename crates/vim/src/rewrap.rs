use crate::{Vim, motion::Motion, object::Object, state::Mode};
use collections::HashMap;
use editor::{Bias, Editor, RewrapOptions, SelectionEffects, display_map::ToDisplayPoint};
use gpui::{Context, Window, actions};
use language::SelectionGoal;

actions!(
    vim,
    [
        /// Rewraps the selected text to fit within the line width.
        Rewrap
    ]
);

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &Rewrap, window, cx| {
        vim.record_current_action(cx);
        Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.store_visual_marks(window, cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let mut positions = vim.save_selection_starts(editor, cx);
                editor.rewrap_impl(
                    RewrapOptions {
                        override_language_settings: true,
                        ..Default::default()
                    },
                    cx,
                );
                editor.change_selections(Default::default(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        if let Some(anchor) = positions.remove(&selection.id) {
                            let mut point = anchor.to_display_point(map);
                            *point.column_mut() = 0;
                            selection.collapse_to(point, SelectionGoal::None);
                        }
                    });
                });
            });
        });
        if vim.mode.is_visual() {
            vim.switch_mode(Mode::Normal, true, window, cx)
        }
    });
}

impl Vim {
    pub(crate) fn rewrap_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        forced_motion: bool,
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
                editor.rewrap_impl(
                    RewrapOptions {
                        override_language_settings: true,
                        ..Default::default()
                    },
                    cx,
                );
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = selection_starts.remove(&selection.id).unwrap();
                        let mut point = anchor.to_display_point(map);
                        *point.column_mut() = 0;
                        selection.collapse_to(point, SelectionGoal::None);
                    });
                });
            });
        });
    }

    pub(crate) fn rewrap_object(
        &mut self,
        object: Object,
        around: bool,
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
                editor.rewrap_impl(
                    RewrapOptions {
                        override_language_settings: true,
                        ..Default::default()
                    },
                    cx,
                );
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = original_positions.remove(&selection.id).unwrap();
                        let mut point = anchor.to_display_point(map);
                        *point.column_mut() = 0;
                        selection.collapse_to(point, SelectionGoal::None);
                    });
                });
            });
        });
    }
}

#[cfg(test)]
mod test {
    use crate::test::NeovimBackedTestContext;

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
}
