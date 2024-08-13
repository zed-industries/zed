use std::{ops::Range, sync::Arc};

use collections::{HashMap, HashSet};
use editor::{
    display_map::{BlockDisposition, BlockProperties, BlockStyle, CustomBlockId},
    Editor,
};
use gpui::{AppContext, Model, View};
use text::{Bias, ToOffset, ToPoint};
use ui::{
    div, h_flex, px, Color, Element as _, ParentElement as _, Styled, ViewContext, WindowContext,
};

use crate::{Context, ResolvedWorkflowStep, WorkflowSuggestion};

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
    fn crease_content(
        context: &Model<Context>,
        range: StepRange,
        cx: &mut AppContext,
    ) -> Option<Arc<str>> {
        use std::fmt::Write;
        let step = context.read(cx).workflow_step_for_range(range)?;
        let mut output = String::from("\n\n");
        match &step.status {
            crate::WorkflowStepStatus::Resolved(ResolvedWorkflowStep { title, suggestions }) => {
                writeln!(output, "Resolution:").ok()?;
                writeln!(output, "  {title:?}").ok()?;
                for (buffer, suggestion_groups) in suggestions {
                    let buffer = buffer.read(cx);
                    let buffer_path = buffer
                        .file()
                        .and_then(|file| file.path().to_str())
                        .unwrap_or("untitled");
                    let snapshot = buffer.text_snapshot();
                    writeln!(output, "  {buffer_path}:").ok()?;
                    for group in suggestion_groups {
                        for suggestion in &group.suggestions {
                            pretty_print_workflow_suggestion(&mut output, suggestion, &snapshot);
                        }
                    }
                }
            }
            crate::WorkflowStepStatus::Pending(_) => {
                writeln!(output, "Resolution: Pending").ok()?;
            }
            crate::WorkflowStepStatus::Error(error) => {
                writeln!(output, "Resolution: Error").ok()?;
                writeln!(output, "{error:?}").ok()?;
            }
        }

        Some(output.into())
    }
    pub(crate) fn activate_for_step(&mut self, range: StepRange, cx: &mut WindowContext<'_>) {
        let text = Self::crease_content(&self.context, range.clone(), cx)
            .unwrap_or_else(|| Arc::from("Error fetching debug info"));
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton()?;
            let snapshot = buffer.read(cx).text_snapshot();
            let start_offset = range.end.to_offset(&snapshot) + 1;
            let start_offset = snapshot.clip_offset(start_offset, Bias::Right);
            let text_len = text.len();
            buffer.update(cx, |this, cx| {
                this.edit([(start_offset..start_offset, text)], None, cx);
            });

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
                                .child(h_flex().h(px(1.)).bg(Color::Warning.color(cx)))
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
fn pretty_print_anchor(
    out: &mut String,
    anchor: &language::Anchor,
    snapshot: &text::BufferSnapshot,
) {
    use std::fmt::Write;
    let point = anchor.to_point(snapshot);
    write!(out, "{}:{}", point.row, point.column).ok();
}
fn pretty_print_range(
    out: &mut String,
    range: &Range<language::Anchor>,
    snapshot: &text::BufferSnapshot,
) {
    use std::fmt::Write;
    write!(out, "    Range: ").ok();
    pretty_print_anchor(out, &range.start, snapshot);
    write!(out, "..").ok();
    pretty_print_anchor(out, &range.end, snapshot);
}

fn pretty_print_workflow_suggestion(
    out: &mut String,
    suggestion: &WorkflowSuggestion,
    snapshot: &text::BufferSnapshot,
) {
    use std::fmt::Write;
    let (range, description, position) = match suggestion {
        WorkflowSuggestion::Update { range, description } => (Some(range), Some(description), None),
        WorkflowSuggestion::CreateFile { description } => (None, Some(description), None),
        WorkflowSuggestion::AppendChild {
            position,
            description,
        }
        | WorkflowSuggestion::InsertSiblingBefore {
            position,
            description,
        }
        | WorkflowSuggestion::InsertSiblingAfter {
            position,
            description,
        }
        | WorkflowSuggestion::PrependChild {
            position,
            description,
        } => (None, Some(description), Some(position)),

        WorkflowSuggestion::Delete { range } => (Some(range), None, None),
    };
    if let Some(description) = description {
        writeln!(out, "    Description: {description}").ok();
    }
    if let Some(range) = range {
        pretty_print_range(out, range, snapshot);
    }
    if let Some(position) = position {
        write!(out, "    Position: ").ok();
        pretty_print_anchor(out, position, snapshot);
        write!(out, "\n").ok();
    }
    write!(out, "\n").ok();
}
