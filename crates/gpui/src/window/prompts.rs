use crate::{
    Element, Empty, EventEmitter, IntoElement, PromptLevel, Render, View, ViewContext,
    VisualContext, WindowContext,
};

/// The event emitted when a prompt's option is selected.
/// The usize is the index of the selected option, from the actions
/// passed to the prompt. If the prompt was dismissed without action taken, response is None.
pub struct PromptResponse(pub Option<usize>);

/// A prompt that can be rendered in the window.
pub trait Prompt: EventEmitter<PromptResponse> + Render {}

impl<V: EventEmitter<PromptResponse> + Render> Prompt for V {}

/// A handle to a prompt that can be used to interact with it.
pub struct PromptHandle {
    view: Box<dyn PromptViewHandle>,
}

impl PromptHandle {
    /// Construct a new prompt handle from a view of the appropriate types
    pub fn new<V: Prompt>(view: View<V>) -> Self {
        Self {
            view: Box::new(view),
        }
    }
}

trait PromptViewHandle {}

impl<V: Prompt> PromptViewHandle for View<V> {}

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

/// Use this function in conjunction with [AppContext::set_prompt_renderer] to force
/// GPUI to always use the fallback prompt renderer.
pub fn fallback_prompt_renderer(
    level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[&str],
    cx: &mut WindowContext,
) -> PromptHandle {
    PromptHandle::new(cx.new_view(|_| FallbackPromptRenderer {
        level,
        message: message.to_string(),
        detail: detail.map(ToString::to_string),
        actions: actions.iter().map(ToString::to_string).collect(),
    }))
}
