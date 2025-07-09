use text::SelectionGoal;
use ui::{Context, Window};

use crate::{Vim, object::Object};

impl Vim {
    /// Selects the text object each cursor is over.
    pub fn select_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.stop_recording(cx);
        self.update_editor(window, cx, |_, editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.move_with(|map, selection| {
                    let Some(range) = object.helix_range(map, selection.clone(), around) else {
                        return;
                    };

                    selection.set_head(range.end, SelectionGoal::None);
                    selection.start = range.start;
                });
            });
        });
    }
}
