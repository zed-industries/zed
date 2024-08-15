use std::sync::Arc;

use super::WorkflowStepResolution;
use crate::{Assist, Context};
use editor::{
    display_map::{BlockDisposition, BlockProperties, BlockStyle},
    Editor, EditorEvent, ExcerptRange, MultiBuffer,
};
use gpui::{
    div, AppContext, Context as _, EventEmitter, FocusableView, IntoElement, Model,
    ParentElement as _, Render, SharedString, Styled as _, View, ViewContext, VisualContext as _,
    WeakModel, WindowContext,
};
use language::{language_settings::SoftWrap, Anchor, Buffer, LanguageRegistry};
use theme::ActiveTheme as _;
use ui::{
    h_flex, ButtonCommon as _, ButtonLike, ButtonStyle, Color, InteractiveElement as _, Label,
    LabelCommon as _,
};
use workspace::{
    item::{self, Item},
    pane,
    searchable::SearchableItemHandle,
};

pub struct WorkflowStepView {
    step: WeakModel<WorkflowStepResolution>,
    tool_output_buffer: Model<Buffer>,
    editor: View<Editor>,
}

impl WorkflowStepView {
    pub fn new(
        context: Model<Context>,
        step: Model<WorkflowStepResolution>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let tool_output_buffer = cx.new_model(|cx| Buffer::local(step.read(cx).output.clone(), cx));
        let buffer = cx.new_model(|cx| {
            let mut buffer = MultiBuffer::without_headers(0, language::Capability::ReadWrite);
            buffer.push_excerpts(
                context.read(cx).buffer().clone(),
                [ExcerptRange {
                    context: step.read(cx).tagged_range.clone(),
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

    fn step_updated(&mut self, step: Model<WorkflowStepResolution>, cx: &mut ViewContext<Self>) {
        self.tool_output_buffer.update(cx, |buffer, cx| {
            let text = step.read(cx).output.clone();
            buffer.set_text(text, cx);
        });
        cx.notify();
    }

    fn step_released(&mut self, _: &mut WorkflowStepResolution, cx: &mut ViewContext<Self>) {
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

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("workflow step".into())
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
