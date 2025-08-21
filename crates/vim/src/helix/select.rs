use text::SelectionGoal;
use ui::{Context, Window};

use crate::{Vim, helix::object::cursor_range, object::Object};

impl Vim {
    /// Selects the object each cursor is over.
    /// Follows helix convention.
    pub fn select_current_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let Some(range) = object
                        .helix_range(map, selection.clone(), around)
                        .unwrap_or({
                            let vim_range = object.range(map, selection.clone(), around, None);
                            vim_range.filter(|r| r.start <= cursor_range(selection, map).start)
                        })
                    else {
                        return;
                    };

                    selection.set_head_tail(range.end, range.start, SelectionGoal::None);
                });
            });
        });
    }

    /// Selects the next object from each cursor which the cursor is not over.
    /// Follows helix convention.
    pub fn select_next_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let Ok(Some(range)) = object.helix_next_range(map, selection.clone(), around)
                    else {
                        return;
                    };

                    selection.set_head_tail(range.end, range.start, SelectionGoal::None);
                });
            });
        });
    }

    /// Selects the previous object from each cursor which the cursor is not over.
    /// Follows helix convention.
    pub fn select_previous_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let Ok(Some(range)) =
                        object.helix_previous_range(map, selection.clone(), around)
                    else {
                        return;
                    };

                    selection.set_head_tail(range.start, range.end, SelectionGoal::None);
                });
            });
        });
    }
}
