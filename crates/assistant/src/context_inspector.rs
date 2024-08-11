use std::ops::Range;

use collections::{HashMap, HashSet};
use editor::{
    actions::FoldAt,
    display_map::{Crease, CreaseId},
    Editor,
};
use gpui::{Empty, Model, View};
use multi_buffer::MultiBufferRow;
use rope::Point;
use ui::{Element as _, WindowContext};

use crate::{
    assistant_panel::{quote_selection_fold_placeholder, render_quote_selection_output_toggle},
    Context,
};

type StepRange = Range<language::Anchor>;

struct DebugInfo {
    range: Range<editor::Anchor>,
    crease_id: CreaseId,
}
pub(crate) struct ContextInspector {
    active_debug_views: HashMap<Range<language::Anchor>, DebugInfo>,
    context: Model<Context>,
    editor: View<Editor>,
}

impl ContextInspector {
    pub(crate) fn new(
        editor: View<Editor>,
        context: Model<Context>,
        cx: &mut WindowContext<'_>,
    ) -> Self {
        Self {
            editor,
            context,
            active_debug_views: Default::default(),
        }
    }
    pub(crate) fn is_active(&self, range: &StepRange) -> bool {
        self.active_debug_views.contains_key(range)
    }
    pub(crate) fn activate_for_step(&mut self, range: StepRange, cx: &mut WindowContext<'_>) {
        let info = self.editor.update(cx, |editor, cx| {
            editor.insert("\n", cx);

            let point = editor.selections.newest::<Point>(cx).head();
            let start_row = MultiBufferRow(point.row);

            editor.insert("I really think creases are great", cx);

            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let anchor_before = snapshot.anchor_after(point);
            let anchor_after = editor
                .selections
                .newest_anchor()
                .head()
                .bias_left(&snapshot);

            editor.insert("\n", cx);

            let fold_placeholder =
                quote_selection_fold_placeholder("Inspect debug".into(), cx.view().downgrade());
            let crease = Crease::new(
                anchor_before..anchor_after,
                fold_placeholder,
                render_quote_selection_output_toggle,
                |_, _, _| Empty.into_any(),
            );
            let crease_id = editor
                .insert_creases(vec![crease], cx)
                .into_iter()
                .next()
                .unwrap();
            editor.fold_at(
                &FoldAt {
                    buffer_row: start_row,
                },
                cx,
            );
            DebugInfo {
                range: anchor_before..anchor_after,
                crease_id,
            }
        });

        self.active_debug_views.insert(range, info);
    }

    fn remove_creases(&self, ids: impl Iterator<Item = CreaseId>, cx: &mut WindowContext<'_>) {
        self.editor.update(cx, |this, cx| {
            this.remove_creases(ids, cx);
            cx.notify();
        })
    }
    pub(crate) fn deactivate_for(&mut self, range: &StepRange, cx: &mut WindowContext<'_>) {
        if let Some(debug_data) = self.active_debug_views.remove(range) {
            self.remove_creases([debug_data.crease_id].into_iter(), cx)
        }
    }

    pub(crate) fn deactivate(&mut self, cx: &mut WindowContext<'_>) {
        let steps_to_disable = std::mem::take(&mut self.active_debug_views);

        self.remove_creases(
            steps_to_disable.into_iter().map(|(_, info)| info.crease_id),
            cx,
        );
    }
}
