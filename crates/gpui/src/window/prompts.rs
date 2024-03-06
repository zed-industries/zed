use std::ops::Deref;

use futures::channel::oneshot;

use crate::{
    Empty, EventEmitter, IntoElement, PromptLevel, Render, View, ViewContext, VisualContext,
    WindowContext,
};

/// The event emitted when a prompt's option is selected.
/// The usize is the index of the selected option, from the actions
/// passed to the prompt.
pub struct PromptResponse(pub usize);

/// A prompt that can be rendered in the window.
pub trait Prompt: EventEmitter<PromptResponse> + Render {}

impl<V: EventEmitter<PromptResponse> + Render> Prompt for V {}

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
        cx.subscribe(&view, move |_, e: &PromptResponse, _| {
            if let Some(sender) = sender.take() {
                sender.send(e.0).ok();
            }
        })
        .detach();

        RenderablePromptHandle {
            view: Box::new(view),
        }
    }
}

/// A prompt handle capable of being rendered in a window.
pub struct RenderablePromptHandle {
    view: Box<dyn PromptViewHandle>,
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
    let renderer = cx.new_view(|_| FallbackPromptRenderer {
        level,
        message: message.to_string(),
        detail: detail.map(ToString::to_string),
        actions: actions.iter().map(ToString::to_string).collect(),
    });

    handle.with_view(renderer, cx)
}

/// The default GPUI fallback for rendering prompts, when the platform doesn't support it.
pub struct FallbackPromptRenderer {
    level: PromptLevel,
    message: String,
    detail: Option<String>,
    actions: Vec<String>,
}

impl Render for FallbackPromptRenderer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Empty
    }
}

impl EventEmitter<PromptResponse> for FallbackPromptRenderer {}

trait PromptViewHandle {}

impl<V: Prompt> PromptViewHandle for View<V> {}

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
