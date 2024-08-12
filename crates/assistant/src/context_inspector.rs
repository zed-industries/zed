use std::{ops::Range, sync::Arc};

use collections::{HashMap, HashSet};
use editor::{
    display_map::{BlockDisposition, BlockProperties, BlockStyle, CustomBlockId},
    Editor,
};
use gpui::{Model, View};
use text::ToOffset;
use ui::{
    div, h_flex, Color, Element as _, ParentElement as _, Styled, ViewContext, WindowContext,
};

use crate::{Context, ResolvedWorkflowStep};

type StepRange = Range<language::Anchor>;

struct DebugInfo {
    range: Range<editor::Anchor>,
    block_id: CustomBlockId,
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

    pub(crate) fn refresh(&mut self, range: &StepRange, cx: &mut WindowContext<'_>) {
        if self.deactivate_for(range, cx) {
            self.activate_for_step(range.clone(), cx);
        }
    }
    fn crease_content(&self, range: StepRange, cx: &mut WindowContext<'_>) -> Option<Arc<str>> {
        use std::fmt::Write;
        let step = self.context.read(cx).workflow_step_for_range(range)?;
        let mut output = String::from("\n\n");
        match &step.status {
            crate::WorkflowStepStatus::Resolved(ResolvedWorkflowStep { title, suggestions }) => {
                writeln!(output, "Resolution:").ok()?;
                writeln!(output, "  {title:?}").ok()?;
                writeln!(output, "  {suggestions:?}").ok()?;
            }
            crate::WorkflowStepStatus::Pending(_) => {
                writeln!(output, "Resolution: Pending").ok()?;
            }
            crate::WorkflowStepStatus::Error(error) => {
                writeln!(output, "Resolution: Error").ok()?;
                writeln!(output, "{error:?}").ok()?;
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

            let block_id = editor
                .insert_blocks(
                    [BlockProperties {
                        position: anchor_after,
                        height: 0,
                        style: BlockStyle::Sticky,
                        render: Box::new(move |cx| {
                            div()
                                .w_full()
                                .px(cx.gutter_dimensions.full_width())
                                .child(
                                    h_flex()
                                        .w_full()
                                        .border_t_1()
                                        .border_color(Color::Warning.color(cx)),
                                )
                                .into_any()
                        }),
                        disposition: BlockDisposition::Below,
                        priority: 0,
                    }],
                    None,
                    cx,
                )
                .into_iter()
                .next()?;
            let info = DebugInfo {
                range: anchor_before..anchor_after,
                block_id,
            };
            self.active_debug_views.insert(range, info);
            Some(())
        });
    }

    fn deactivate_impl(editor: &mut Editor, debug_data: DebugInfo, cx: &mut ViewContext<Editor>) {
        editor.remove_blocks(HashSet::from_iter([debug_data.block_id]), None, cx);
        editor.edit([(debug_data.range, Arc::<str>::default())], cx)
    }
    pub(crate) fn deactivate_for(&mut self, range: &StepRange, cx: &mut WindowContext<'_>) -> bool {
        if let Some(debug_data) = self.active_debug_views.remove(range) {
            self.editor.update(cx, |this, cx| {
                Self::deactivate_impl(this, debug_data, cx);
            });
            true
        } else {
            false
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
