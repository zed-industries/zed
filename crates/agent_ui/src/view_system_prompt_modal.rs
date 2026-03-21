use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, ScrollHandle,
    SharedString, Window,
};
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, prelude::*};
use workspace::{ModalView, Workspace};

pub struct ViewSystemPromptModal {
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    markdown: Entity<Markdown>,
}

impl ViewSystemPromptModal {
    pub fn toggle(
        system_prompt: impl Into<SharedString>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let system_prompt = system_prompt.into();
        let markdown = cx.new(|cx| Markdown::new(system_prompt, None, None, cx));

        workspace.toggle_modal(window, cx, |_window, cx| Self {
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            markdown,
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ViewSystemPromptModal {}

impl Focusable for ViewSystemPromptModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for ViewSystemPromptModal {}

impl Render for ViewSystemPromptModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);
        let markdown_style = MarkdownStyle::themed(MarkdownFont::Editor, window, cx);

        v_flex()
            .id("view-system-prompt-modal")
            .key_context("ViewSystemPromptModal")
            .w(rems(80.))
            .max_h(rems(60.))
            .elevation_3(cx)
            .on_action(cx.listener(Self::cancel))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window, cx);
            }))
            .child(
                Modal::new("view-system-prompt", Some(self.scroll_handle.clone()))
                    .header(ModalHeader::new().headline("System Prompt"))
                    .child(
                        div()
                            .px(DynamicSpacing::Base12.rems(cx))
                            .py(DynamicSpacing::Base04.rems(cx))
                            .child(MarkdownElement::new(self.markdown.clone(), markdown_style)),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            Button::new("close", "Close")
                                .key_binding(
                                    KeyBinding::for_action_in(&menu::Cancel, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(cx.listener(|this, _event, window, cx| {
                                    this.cancel(&menu::Cancel, window, cx)
                                })),
                        ),
                    ),
            )
    }
}
