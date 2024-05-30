use gpui::{
    div, opaque_grey, red, EventEmitter, FocusHandle, FocusableView, InteractiveElement as _,
    IntoElement, ParentElement, PromptHandle, PromptLevel, PromptResponse, Render,
    RenderablePromptHandle, StatefulInteractiveElement, Styled, ViewContext, VisualContext,
    WindowContext,
};

pub fn build(
    level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    handle: PromptHandle,
    cx: &mut WindowContext,
) -> RenderablePromptHandle {
    let prompt = cx.new_view(|cx| ZedPrompt {
        level,
        message: message.to_string(),
        detail: detail.map(ToString::to_string),
        actions: actions.iter().map(ToString::to_string).collect(),
        focus: cx.focus_handle(),
    });

    handle.with_view(prompt, cx)
}

struct ZedPrompt {
    level: PromptLevel,
    message: String,
    detail: Option<String>,
    actions: Vec<String>,
    focus: FocusHandle,
}

impl Render for ZedPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let prompt = div()
            .cursor_default()
            .track_focus(&self.focus)
            .w_72()
            .bg(red())
            .rounded_lg()
            .overflow_hidden()
            .p_3()
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .child(div().overflow_hidden().child(self.message.clone())),
            )
            .children(self.detail.clone().map(|detail| {
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .text_sm()
                    .mb_2()
                    .child(div().child(detail))
            }))
            .children(self.actions.iter().enumerate().map(|(ix, action)| {
                div()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .border_1()
                    .border_color(opaque_grey(0.2, 0.5))
                    .mt_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_sm()
                    .child(action.clone())
                    .id(ix)
                    .on_click(cx.listener(move |_, _, cx| {
                        cx.emit(PromptResponse(ix));
                    }))
            }));

        div()
            .size_full()
            .child(
                div()
                    .size_full()
                    .bg(opaque_grey(0.5, 0.6))
                    .absolute()
                    .top_0()
                    .left_0(),
            )
            .child(
                div()
                    .size_full()
                    .absolute()
                    .top_0()
                    .left_0()
                    .flex()
                    .flex_col()
                    .justify_around()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_around()
                            .child(prompt),
                    ),
            )
    }
}

impl EventEmitter<PromptResponse> for ZedPrompt {}

impl FocusableView for ZedPrompt {
    fn focus_handle(&self, _: &crate::AppContext) -> FocusHandle {
        self.focus.clone()
    }
}
