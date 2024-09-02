use crate::{motion::Motion, object::Object, Vim};
use collections::HashMap;
use editor::{display_map::ToDisplayPoint, Bias};
use language::SelectionGoal;
use ui::ViewContext;

#[derive(PartialEq, Eq)]
pub(crate) enum IndentDirection {
    In,
    Out,
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
                if dir == IndentDirection::In {
                    editor.indent(&Default::default(), cx);
                } else {
                    editor.outdent(&Default::default(), cx);
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
                if dir == IndentDirection::In {
                    editor.indent(&Default::default(), cx);
                } else {
                    editor.outdent(&Default::default(), cx);
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
