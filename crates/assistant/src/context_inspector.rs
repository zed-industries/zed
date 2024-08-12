use std::{ops::Range, sync::Arc};

use collections::{HashMap, HashSet};
use editor::{
    actions::FoldAt,
    display_map::{Crease, CreaseId},
    Editor,
};
use gpui::{Empty, Model, View};
use multi_buffer::MultiBufferRow;
use text::ToOffset;
use ui::{Element as _, ViewContext, WindowContext};

use crate::{
    assistant_panel::{quote_selection_fold_placeholder, render_quote_selection_output_toggle},
    Context, ResolvedWorkflowStep,
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
    pub(crate) fn new(editor: View<Editor>, context: Model<Context>) -> Self {
        Self {
            editor,
            context,
            active_debug_views: Default::default(),
        }
    }
    pub(crate) fn is_active(&self, range: &StepRange) -> bool {
        self.active_debug_views.contains_key(range)
    }
    fn crease_content(&self, range: StepRange, cx: &mut WindowContext<'_>) -> Option<Arc<str>> {
        use std::fmt::Write;
        let step = self.context.read(cx).workflow_step_for_range(range)?;
        let mut output = String::from("\n\n");
        match &step.status {
            crate::WorkflowStepStatus::Resolved(ResolvedWorkflowStep { title, suggestions }) => {
                output.push_str("Resolution:\n");
                output.push_str(&format!("  {:?}\n", title));
                output.push_str(&format!("  {:?}\n", suggestions));
            }
            crate::WorkflowStepStatus::Pending(_) => {
                output.push_str("Resolution: Pending\n");
            }
            crate::WorkflowStepStatus::Error(error) => {
                writeln!(output, "Resolution: Error\n{:?}", error).unwrap();
            }
        }
        output.push('\n');

        Some(output.into())
    }
    pub(crate) fn activate_for_step(&mut self, range: StepRange, cx: &mut WindowContext<'_>) {
        let text = self
            .crease_content(range.clone(), cx)
            .unwrap_or_else(|| Arc::from("Error fetching debug info"));
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton()?;

            let text_len = text.len();
            let snapshot = buffer.update(cx, |this, cx| {
                this.edit([(range.end..range.end, text)], None, cx);
                this.text_snapshot()
            });
            let start_offset = range.end.to_offset(&snapshot);
            let end_offset = start_offset + text_len;
            let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
            let anchor_before = multibuffer_snapshot.anchor_after(start_offset);
            let anchor_after = multibuffer_snapshot.anchor_before(end_offset);

            let start_row =
                MultiBufferRow(multibuffer_snapshot.offset_to_point(start_offset + 1).row + 1);

            let fold_placeholder =
                quote_selection_fold_placeholder("Inspect".into(), cx.view().downgrade());

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

            let info = DebugInfo {
                range: anchor_before..anchor_after,
                crease_id,
            };
            self.active_debug_views.insert(range, info);
            Some(())
        });
    }

    fn deactivate_impl(editor: &mut Editor, debug_data: DebugInfo, cx: &mut ViewContext<Editor>) {
        editor.remove_creases([debug_data.crease_id], cx);
        editor.edit([(debug_data.range, Arc::<str>::default())], cx)
    }
    pub(crate) fn deactivate_for(&mut self, range: &StepRange, cx: &mut WindowContext<'_>) {
        if let Some(debug_data) = self.active_debug_views.remove(range) {
            self.editor.update(cx, |this, cx| {
                Self::deactivate_impl(this, debug_data, cx);
            });
        }
    }

    pub(crate) fn deactivate(&mut self, cx: &mut WindowContext<'_>) {
        let steps_to_disable = std::mem::take(&mut self.active_debug_views);

        self.editor.update(cx, move |editor, cx| {
            for (_, debug_data) in steps_to_disable {
                Self::deactivate_impl(editor, debug_data, cx);
            }
        });
    }
}
