use crate::{prelude::*, Button, Label, LabelColor, Modal};

#[derive(Component)]
pub struct CopilotModal {
    id: ElementId,
}

impl CopilotModal {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
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

    #[derive(Component)]
    pub struct CopilotModalStory;

    impl CopilotModalStory {
        pub fn new() -> Self {
            Self
        }

        fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
            Story::container(cx)
                .child(Story::title_for::<_, CopilotModal>(cx))
                .child(Story::label(cx, "Default"))
                .child(CopilotModal::new("copilot-modal"))
        }
    }
}
