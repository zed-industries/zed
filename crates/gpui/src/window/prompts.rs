use std::ops::Deref;

use futures::channel::oneshot;

use crate::{
    div, opaque_grey, white, AnyView, App, AppContext as _, Context, Entity, EventEmitter,
    FocusHandle, Focusable, FocusableElement, InteractiveElement, IntoElement, KeyDownEvent,
    Keystroke, ParentElement, PromptLevel, Render, StatefulInteractiveElement, Styled,
};

use super::Window;

/// The event emitted when a prompt's option is selected.
/// The usize is the index of the selected option, from the actions
/// passed to the prompt.
pub struct PromptResponse(pub usize);

/// A prompt that can be rendered in the window.
pub trait Prompt: EventEmitter<PromptResponse> + Focusable {}

impl<V: EventEmitter<PromptResponse> + Focusable> Prompt for V {}

/// A handle to a prompt that can be used to interact with it.
pub struct PromptHandle {
    sender: oneshot::Sender<usize>,
}

impl PromptHandle {
    pub(crate) fn new(sender: oneshot::Sender<usize>) -> Self {
        Self { sender }
    }

    /// Construct a new prompt handle from a view of the appropriate types
    pub fn with_view<V: Prompt + Render>(
        self,
        view: Entity<V>,
        window: &mut Window,
        cx: &mut App,
    ) -> RenderablePromptHandle {
        let mut sender = Some(self.sender);
        let previous_focus = window.focused(cx);
        let window_handle = window.window_handle();
        cx.subscribe(&view, move |_: Entity<V>, e: &PromptResponse, cx| {
            if let Some(sender) = sender.take() {
                sender.send(e.0).ok();
                window_handle
                    .update(cx, |_, window, _cx| {
                        window.prompt.take();
                        if let Some(previous_focus) = &previous_focus {
                            window.focus(previous_focus);
                        }
                    })
                    .ok();
            }
        })
        .detach();

        window.focus(&view.focus_handle(cx));

        RenderablePromptHandle {
            view: Box::new(view),
        }
    }
}

/// A prompt handle capable of being rendered in a window.
pub struct RenderablePromptHandle {
    pub(crate) view: Box<dyn PromptViewHandle>,
}

/// Use this function in conjunction with [App::set_prompt_builder] to force
/// GPUI to always use the fallback prompt renderer.
pub fn fallback_prompt_renderer(
    level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    handle: PromptHandle,
    window: &mut Window,
    cx: &mut App,
) -> RenderablePromptHandle {
    let renderer =
        cx.new(|cx| FallbackPromptRenderer::new(level, message, detail, actions, window, cx));

    handle.with_view(renderer, window, cx)
}

/// The default GPUI fallback for rendering prompts, when the platform doesn't support it.
pub struct FallbackPromptRenderer {
    _level: PromptLevel,
    _highlighted_action: Option<usize>,
    actions: Vec<(String, FocusHandle)>,
    message: String,
    detail: Option<String>,
    focus: FocusHandle,
}

impl FallbackPromptRenderer {
    fn new(
        level: PromptLevel,
        message: &str,
        detail: Option<&str>,
        actions: &[&str],
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut actions: Vec<_> = actions
            .into_iter()
            .map(|item| (item.to_string(), cx.focus_handle()))
            .collect();

        let _highlighted_action = if actions.len() > 0 {
            Some(0usize)
        } else {
            None
        };

        Self {
            _level: level,
            _highlighted_action,
            message: message.to_string(),
            detail: detail.map(ToString::to_string),
            actions,
            focus: cx.focus_handle(),
        }
    }

    fn handle_key_down_event(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        if event.keystroke.should_match(&Keystroke {
            key: "up".to_string(),
            ..Keystroke::default()
        }) {
            self.highlight_previous(window, cx);
        } else if event.keystroke.should_match(&Keystroke {
            key: "down".to_string(),
            ..Keystroke::default()
        }) {
            self.highlight_next(window, cx);
        } else if event.keystroke.should_match(&Keystroke {
            key: "enter".to_string(),
            ..Keystroke::default()
        }) {
            self.handle_submit(window, cx);
        }
    }

    fn handle_submit(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self._highlighted_action {
            cx.emit(PromptResponse(ix));
        }
    }

    fn highlight_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self._highlighted_action {
            if ix == self.actions.len() - 1 {
                self._highlighted_action = Some(0);
            } else {
                self._highlighted_action = Some(ix + 1);
            }
            self.actions[self._highlighted_action.unwrap()]
                .1
                .focus(window);

            cx.notify();
        }
    }

    fn highlight_previous(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self._highlighted_action {
            if ix == 0 {
                self._highlighted_action = Some(self.actions.len() - 1);
                self.actions[self._highlighted_action.unwrap()]
                    .1
                    .focus(window);
            } else {
                self._highlighted_action = Some(ix - 1);
            }
            self.actions[self._highlighted_action.unwrap()]
                .1
                .focus(window);

            cx.notify();
        }
    }
}

impl Render for FallbackPromptRenderer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        debug_assert!(
            self._highlighted_action
                .is_none_or(|f| f < self.actions.len()),
            "highlighted action out of bounds"
        );

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
            .children(
                self.actions
                    .iter()
                    .enumerate()
                    .map(|(ix, (action, focus_handle))| {
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
                            .track_focus(focus_handle)
                            .focus(|s| s.bg(opaque_grey(0.2, 0.5)))
                            .on_click(cx.listener(move |_, _, _, cx| {
                                cx.emit(PromptResponse(ix));
                            }))
                    }),
            );

        div()
            .size_full()
            .key_context("prompts")
            .child(
                div()
                    .size_full()
                    .bg(opaque_grey(0.5, 0.6))
                    .absolute()
                    .top_0()
                    .left_0(),
            )
            .on_key_down(cx.listener(Self::handle_key_down_event))
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

impl Focusable for FallbackPromptRenderer {
    fn focus_handle(&self, _: &crate::App) -> FocusHandle {
        if self.actions.len() > 0 {
            self.actions.first().unwrap().1.clone()
        } else {
            self.focus.clone()
        }
    }
}

pub(crate) trait PromptViewHandle {
    fn any_view(&self) -> AnyView;
}

impl<V: Prompt + Render> PromptViewHandle for Entity<V> {
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
                &mut Window,
                &mut App,
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
        &mut Window,
        &mut App,
    ) -> RenderablePromptHandle;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Default => &fallback_prompt_renderer,
            Self::Custom(f) => f.as_ref(),
        }
    }
}
