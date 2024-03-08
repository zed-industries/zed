use std::ops::Deref;

use futures::channel::oneshot;

use crate::{
    div, opaque_grey, white, AnyView, EventEmitter, FocusHandle, FocusableView, InteractiveElement,
    IntoElement, ParentElement, PromptLevel, Render, StatefulInteractiveElement, Styled, View,
    ViewContext, VisualContext, WindowContext,
};

/// The event emitted when a prompt's option is selected.
/// The usize is the index of the selected option, from the actions
/// passed to the prompt.
pub struct PromptResponse(pub usize);

/// A prompt that can be rendered in the window.
pub trait Prompt: EventEmitter<PromptResponse> + FocusableView {}

impl<V: EventEmitter<PromptResponse> + FocusableView> Prompt for V {}

/// A handle to a prompt that can be used to interact with it.
pub struct PromptHandle {
    sender: oneshot::Sender<usize>,
}

impl PromptHandle {
    pub(crate) fn new(sender: oneshot::Sender<usize>) -> Self {
        Self { sender }
    }

    /// Construct a new prompt handle from a view of the appropriate types
    pub fn with_view<V: Prompt>(
        self,
        view: View<V>,
        cx: &mut WindowContext,
    ) -> RenderablePromptHandle {
        let mut sender = Some(self.sender);
        let previous_focus = cx.focused();
        cx.subscribe(&view, move |_, e: &PromptResponse, cx| {
            if let Some(sender) = sender.take() {
                sender.send(e.0).ok();
                cx.window.prompt.take();
                if let Some(previous_focus) = &previous_focus {
                    cx.focus(&previous_focus);
                }
            }
        })
        .detach();

        cx.focus_view(&view);

        RenderablePromptHandle {
            view: Box::new(view),
        }
    }
}

/// A prompt handle capable of being rendered in a window.
pub struct RenderablePromptHandle {
    pub(crate) view: Box<dyn PromptViewHandle>,
}

/// Use this function in conjunction with [AppContext::set_prompt_renderer] to force
/// GPUI to always use the fallback prompt renderer.
pub fn fallback_prompt_renderer(
    level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    handle: PromptHandle,
    cx: &mut WindowContext,
) -> RenderablePromptHandle {
    let renderer = cx.new_view({
        |cx| FallbackPromptRenderer {
            _level: level,
            message: message.to_string(),
            detail: detail.map(ToString::to_string),
            actions: actions.iter().map(ToString::to_string).collect(),
            focus: cx.focus_handle(),
        }
    });

    handle.with_view(renderer, cx)
}

/// The default GPUI fallback for rendering prompts, when the platform doesn't support it.
pub struct FallbackPromptRenderer {
    _level: PromptLevel,
    message: String,
    detail: Option<String>,
    actions: Vec<String>,
    focus: FocusHandle,
}

impl Render for FallbackPromptRenderer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let prompt = div()
            .cursor_default()
            .track_focus(&self.focus)
            .w_72()
            .bg(white())
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

impl EventEmitter<PromptResponse> for FallbackPromptRenderer {}

impl FocusableView for FallbackPromptRenderer {
    fn focus_handle(&self, _: &crate::AppContext) -> FocusHandle {
        self.focus.clone()
    }
}

pub(crate) trait PromptViewHandle {
    fn any_view(&self) -> AnyView;
}

impl<V: Prompt> PromptViewHandle for View<V> {
    fn any_view(&self) -> AnyView {
        self.clone().into()
    }
}

pub(crate) enum PromptBuilder {
    Default,
    Custom(
        Box<
            dyn Fn(
                PromptLevel,
                &str,
                Option<&str>,
                &[&str],
                PromptHandle,
                &mut WindowContext,
            ) -> RenderablePromptHandle,
        >,
    ),
}

impl Deref for PromptBuilder {
    type Target = dyn Fn(
        PromptLevel,
        &str,
        Option<&str>,
        &[&str],
        PromptHandle,
        &mut WindowContext,
    ) -> RenderablePromptHandle;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Default => &fallback_prompt_renderer,
            Self::Custom(f) => f.as_ref(),
        }
    }
}
