use crate::{Vim, motion::Motion, object::Object, state::Mode};
use collections::HashMap;
use editor::{
    Bias, DisplayPoint, Editor, MultiBufferOffset, RewrapOptions, SelectionEffects,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement,
};
use gpui::{Action, Context, Window};
use language::SelectionGoal;
use schemars::JsonSchema;
use serde::Deserialize;
use text::Selection;

/// Rewraps the selected text to fit within the line width.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
pub(crate) struct Rewrap {
    pub line_length: Option<usize>,
    pub keep_cursor: bool,
}

fn head_offset_for_keep_cursor(
    display_map: &DisplaySnapshot,
    selection: &Selection<DisplayPoint>,
) -> MultiBufferOffset {
    let point = selection.head();

    // When the selection isn't reversed, the offset works out to be after the head position. It
    // needs to be adjusted so that the cursor ends up in the same position as it does in neovim.
    if selection.reversed {
        point.to_offset(display_map, Bias::Left)
    } else {
        movement::saturating_left(display_map, point).to_offset(display_map, Bias::Left)
    }
}

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, action: &Rewrap, window, cx| {
        vim.record_current_action(cx);
        Vim::take_count(cx);
        Vim::take_forced_motion(cx);
        vim.store_visual_marks(window, cx);
        vim.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let mut positions = vim.save_selection_starts(editor, cx);
                let display_map = editor.display_snapshot(cx);
                let mut selection_head_offsets: HashMap<_, _> = editor
                    .selections
                    .all_display(&display_map)
                    .iter()
                    .map(|s| (s.id, head_offset_for_keep_cursor(&display_map, s)))
                    .collect();
                editor.rewrap(
                    RewrapOptions {
                        override_language_settings: true,
                        line_length: action.line_length,
                        ..Default::default()
                    },
                    cx,
                );
                if action.keep_cursor {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            if let Some(offset) = selection_head_offsets.remove(&selection.id) {
                                let point = map.clip_at_line_end(offset.to_display_point(map));
                                selection.collapse_to(point, SelectionGoal::None);
                            }
                        });
                    });
                } else {
                    editor.change_selections(Default::default(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            if let Some(anchor) = positions.remove(&selection.id) {
                                let mut point = anchor.to_display_point(map);
                                *point.column_mut() = 0;
                                selection.collapse_to(point, SelectionGoal::None);
                            }
                        });
                    });
                }
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
        keep_cursor: bool,
        forced_motion: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(window, cx);
            editor.transact(window, cx, |editor, window, cx| {
                let mut selection_head_offsets: HashMap<_, _> = Default::default();
                let mut selection_starts: HashMap<_, _> = Default::default();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(&mut |map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        selection_starts.insert(selection.id, anchor);
                        selection_head_offsets
                            .insert(selection.id, selection.head().to_offset(map, Bias::Right));
                        motion.expand_selection(
                            map,
                            selection,
                            times,
                            &text_layout_details,
                            forced_motion,
                        );
                    });
                });
                editor.rewrap(
                    RewrapOptions {
                        override_language_settings: true,
                        ..Default::default()
                    },
                    cx,
                );
                if keep_cursor {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            let offset = selection_head_offsets.remove(&selection.id).unwrap();
                            let point = map.clip_at_line_end(offset.to_display_point(map));
                            selection.collapse_to(point, SelectionGoal::None);
                        });
                    });
                } else {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            let anchor = selection_starts.remove(&selection.id).unwrap();
                            let mut point = anchor.to_display_point(map);
                            *point.column_mut() = 0;
                            selection.collapse_to(point, SelectionGoal::None);
                        });
                    });
                }
            });
        });
    }

    pub(crate) fn rewrap_object(
        &mut self,
        object: Object,
        around: bool,
        keep_cursor: bool,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                let mut selection_head_offsets: HashMap<_, _> = Default::default();
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(&mut |map, selection| {
                        let anchor = map.display_point_to_anchor(selection.head(), Bias::Right);
                        original_positions.insert(selection.id, anchor);
                        selection_head_offsets
                            .insert(selection.id, selection.head().to_offset(map, Bias::Right));
                        object.expand_selection(map, selection, around, times);
                    });
                });
                editor.rewrap(
                    RewrapOptions {
                        override_language_settings: true,
                        ..Default::default()
                    },
                    cx,
                );
                if keep_cursor {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            let offset = selection_head_offsets.remove(&selection.id).unwrap();
                            let point = offset.to_display_point(map);
                            selection.collapse_to(point, SelectionGoal::None);
                        });
                    });
                } else {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.move_with(&mut |map, selection| {
                            let anchor = original_positions.remove(&selection.id).unwrap();
                            let mut point = anchor.to_display_point(map);
                            *point.column_mut() = 0;
                            selection.collapse_to(point, SelectionGoal::None);
                        });
                    });
                }
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
