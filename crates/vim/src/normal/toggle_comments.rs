use crate::{Vim, motion::Motion, object::Object};
use collections::HashMap;
use editor::{Bias, SelectionEffects, display_map::ToDisplayPoint};
use gpui::{Context, Window};
use language::SelectionGoal;

impl Vim {
    pub fn toggle_comments_motion(
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
                editor.toggle_comments(&Default::default(), window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let anchor = selection_starts.remove(&selection.id).unwrap();
                        selection.collapse_to(anchor.to_display_point(map), SelectionGoal::None);
                    });
                });
            });
        });
    }

    pub fn toggle_comments_object(
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
                editor.toggle_comments(&Default::default(), window, cx);
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
