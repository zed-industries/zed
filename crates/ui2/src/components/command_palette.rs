use crate::prelude::*;
use crate::{example_editor_actions, OrderMethod, Palette};

#[derive(Component)]
pub struct CommandPalette {
    id: ElementId,
}

impl CommandPalette {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        div().id(self.id.clone()).child(
            Palette::new("palette")
                .items(example_editor_actions())
                .placeholder("Execute a command...")
                .empty_string("No items found.")
                .default_order(OrderMethod::Ascending),
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
    pub struct CommandPaletteStory;

    impl CommandPaletteStory {
        pub fn new() -> Self {
            Self
        }

        fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
            Story::container(cx)
                .child(Story::title_for::<_, CommandPalette>(cx))
                .child(Story::label(cx, "Default"))
                .child(CommandPalette::new("command-palette"))
        }
    }
}
