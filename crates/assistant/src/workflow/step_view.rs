use super::WorkflowStep;
use crate::{Assist, Context};
use editor::{
    display_map::{BlockDisposition, BlockProperties, BlockStyle},
    Editor, EditorEvent, ExcerptRange, MultiBuffer,
};
use gpui::{
    div, AnyElement, AppContext, Context as _, Empty, EventEmitter, FocusableView, IntoElement,
    Model, ParentElement as _, Render, SharedString, Styled as _, View, ViewContext,
    VisualContext as _, WeakModel, WindowContext,
};
use language::{language_settings::SoftWrap, Anchor, Buffer, LanguageRegistry};
use std::{ops::DerefMut, sync::Arc};
use text::OffsetRangeExt;
use theme::ActiveTheme as _;
use ui::{
    h_flex, v_flex, ButtonCommon as _, ButtonLike, ButtonStyle, Color, Icon, IconName,
    InteractiveElement as _, Label, LabelCommon as _,
};
use workspace::{
    item::{self, Item},
    pane,
    searchable::SearchableItemHandle,
};

pub struct WorkflowStepView {
    step: WeakModel<WorkflowStep>,
    tool_output_buffer: Model<Buffer>,
    editor: View<Editor>,
}

impl WorkflowStepView {
    pub fn new(
        context: Model<Context>,
        step: Model<WorkflowStep>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let tool_output_buffer =
            cx.new_model(|cx| Buffer::local(step.read(cx).tool_output.clone(), cx));
        let buffer = cx.new_model(|cx| {
            let mut buffer = MultiBuffer::without_headers(0, language::Capability::ReadWrite);
            buffer.push_excerpts(
                context.read(cx).buffer().clone(),
                [ExcerptRange {
                    context: step.read(cx).context_buffer_range.clone(),
                    primary: None,
                }],
                cx,
            );
            buffer.push_excerpts(
                tool_output_buffer.clone(),
                [ExcerptRange {
                    context: Anchor::MIN..Anchor::MAX,
                    primary: None,
                }],
                cx,
            );
            buffer
        });

        let buffer_snapshot = buffer.read(cx).snapshot(cx);
        let output_excerpt = buffer_snapshot.excerpts().skip(1).next().unwrap().0;
        let input_start_anchor = multi_buffer::Anchor::min();
        let output_start_anchor = buffer_snapshot
            .anchor_in_excerpt(output_excerpt, Anchor::MIN)
            .unwrap();
        let output_end_anchor = multi_buffer::Anchor::max();

        let handle = cx.view().downgrade();
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_multibuffer(buffer.clone(), None, false, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_read_only(true);
            editor.set_show_inline_completions(false);
            editor.insert_blocks(
                [
                    BlockProperties {
                        position: input_start_anchor,
                        height: 1,
                        style: BlockStyle::Fixed,
                        render: Box::new(|cx| section_header("Step Input", cx)),
                        disposition: BlockDisposition::Above,
                        priority: 0,
                    },
                    BlockProperties {
                        position: output_start_anchor,
                        height: 1,
                        style: BlockStyle::Fixed,
                        render: Box::new(|cx| section_header("Tool Output", cx)),
                        disposition: BlockDisposition::Above,
                        priority: 0,
                    },
                    BlockProperties {
                        position: output_end_anchor,
                        height: 1,
                        style: BlockStyle::Fixed,
                        render: Box::new(move |cx| {
                            if let Some(result) = handle.upgrade().and_then(|this| {
                                this.update(cx.deref_mut(), |this, cx| this.render_result(cx))
                            }) {
                                v_flex()
                                    .child(section_header("Output", cx))
                                    .child(
                                        div().pl(cx.gutter_dimensions.full_width()).child(result),
                                    )
                                    .into_any_element()
                            } else {
                                Empty.into_any_element()
                            }
                        }),
                        disposition: BlockDisposition::Below,
                        priority: 0,
                    },
                ],
                None,
                cx,
            );
            editor
        });

        cx.observe(&step, Self::step_updated).detach();
        cx.observe_release(&step, Self::step_released).detach();

        cx.spawn(|this, mut cx| async move {
            if let Ok(language) = language_registry.language_for_name("JSON").await {
                this.update(&mut cx, |this, cx| {
                    this.tool_output_buffer.update(cx, |buffer, cx| {
                        buffer.set_language(Some(language), cx);
                    });
                })
                .ok();
            }
        })
        .detach();

        Self {
            tool_output_buffer,
            step: step.downgrade(),
            editor,
        }
    }

    pub fn step(&self) -> &WeakModel<WorkflowStep> {
        &self.step
    }

    fn render_result(&mut self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        let step = self.step.upgrade()?;
        let result = step.read(cx).resolution.as_ref()?;
        match result {
            Ok(result) => {
                Some(
                    v_flex()
                        .child(result.title.clone())
                        .children(result.suggestion_groups.iter().filter_map(
                            |(buffer, suggestion_groups)| {
                                let buffer = buffer.read(cx);
                                let path = buffer.file().map(|f| f.path());
                                let snapshot = buffer.snapshot();
                                v_flex()
                                    .mb_2()
                                    .border_b_1()
                                    .children(path.map(|path| format!("path: {}", path.display())))
                                    .children(suggestion_groups.iter().map(|group| {
                                        v_flex().pt_2().pl_2().children(
                                            group.suggestions.iter().map(|suggestion| {
                                                let range = suggestion.range().to_point(&snapshot);
                                                v_flex()
                                                    .children(
                                                        suggestion.description().map(|desc| {
                                                            format!("description: {desc}")
                                                        }),
                                                    )
                                                    .child(format!("kind: {}", suggestion.kind()))
                                                    .children(suggestion.symbol_path().map(
                                                        |path| format!("symbol path: {}", path.0),
                                                    ))
                                                    .child(format!(
                                                        "lines: {} - {}",
                                                        range.start.row + 1,
                                                        range.end.row + 1
                                                    ))
                                            }),
                                        )
                                    }))
                                    .into()
                            },
                        ))
                        .into_any_element(),
                )
            }
            Err(error) => Some(format!("{:?}", error).into_any_element()),
        }
    }

    fn step_updated(&mut self, step: Model<WorkflowStep>, cx: &mut ViewContext<Self>) {
        self.tool_output_buffer.update(cx, |buffer, cx| {
            let text = step.read(cx).tool_output.clone();
            buffer.set_text(text, cx);
        });
        cx.notify();
    }

    fn step_released(&mut self, _: &mut WorkflowStep, cx: &mut ViewContext<Self>) {
        cx.emit(EditorEvent::Closed);
    }

    fn resolve(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        self.step
            .update(cx, |step, cx| {
                step.resolve(cx);
            })
            .ok();
    }
}

fn section_header(
    name: &'static str,
    cx: &mut editor::display_map::BlockContext,
) -> gpui::AnyElement {
    h_flex()
        .pl(cx.gutter_dimensions.full_width())
        .h_11()
        .w_full()
        .relative()
        .gap_1()
        .child(
            ButtonLike::new("role")
                .style(ButtonStyle::Filled)
                .child(Label::new(name).color(Color::Default)),
        )
        .into_any_element()
}

impl Render for WorkflowStepView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("ContextEditor")
            .on_action(cx.listener(Self::resolve))
            .flex_grow()
            .bg(cx.theme().colors().editor_background)
            .child(self.editor.clone())
    }
}

impl EventEmitter<EditorEvent> for WorkflowStepView {}

impl FocusableView for WorkflowStepView {
    fn focus_handle(&self, cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Item for WorkflowStepView {
    type Event = EditorEvent;

    fn tab_content_text(&self, cx: &WindowContext) -> Option<SharedString> {
        let step = self.step.upgrade()?.read(cx);
        let context = step.context.upgrade()?.read(cx);
        let buffer = context.buffer().read(cx);
        let index = context
            .workflow_step_index_for_range(&step.context_buffer_range, buffer)
            .ok()?
            + 1;
        Some(format!("Step {index}").into())
    }

    fn tab_icon(&self, _cx: &WindowContext) -> Option<ui::Icon> {
        Some(Icon::new(IconName::Pencil))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(item::ItemEvent)) {
        match event {
            EditorEvent::Edited { .. } => {
                f(item::ItemEvent::Edit);
            }
            EditorEvent::TitleChanged => {
                f(item::ItemEvent::UpdateTab);
            }
            EditorEvent::Closed => f(item::ItemEvent::CloseItem),
            _ => {}
        }
    }

    fn tab_tooltip_text(&self, _cx: &AppContext) -> Option<SharedString> {
        None
    }

    fn as_searchable(&self, _handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn set_nav_history(&mut self, nav_history: pane::ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, cx)
        })
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }
}
