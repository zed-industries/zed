use std::marker::PhantomData;

use crate::{prelude::*, Button, Label, LabelColor, Modal};

#[derive(Element)]
pub struct CopilotModal<S: 'static + Send + Sync + Clone> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> CopilotModal<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<S> {
        div().id(self.id.clone()).child(
            Modal::new("some-id")
                .title("Connect Copilot to Zed")
                .child(Label::new("You can update your settings or sign out from the Copilot menu in the status bar.").color(LabelColor::Muted))
                .primary_action(Button::new("Connect to Github").variant(ButtonVariant::Filled)),
        )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct CopilotModalStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> CopilotModalStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<S> {
            Story::container(cx)
                .child(Story::title_for::<_, CopilotModal<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(CopilotModal::new("copilot-modal"))
        }
    }
}
