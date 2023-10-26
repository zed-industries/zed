use std::marker::PhantomData;

use crate::prelude::*;
use crate::{example_editor_actions, OrderMethod, Palette};

#[derive(Component)]
pub struct CommandPalette<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> CommandPalette<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
        }
    }

    fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
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
    pub struct CommandPaletteStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> CommandPaletteStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
            Story::container(cx)
                .child(Story::title_for::<_, CommandPalette<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(CommandPalette::new("command-palette"))
        }
    }
}
