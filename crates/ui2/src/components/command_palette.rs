use std::marker::PhantomData;

use crate::prelude::*;
use crate::{example_editor_actions, OrderMethod, Palette};

#[derive(Element)]
pub struct CommandPalette<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    scroll_state: ScrollState,
}

impl<S: 'static + Send + Sync + Clone> CommandPalette<S> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            state_type: PhantomData,
            scroll_state,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        div().child(
            Palette::new(self.scroll_state.clone())
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

    #[derive(Element)]
    pub struct CommandPaletteStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> CommandPaletteStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
            Story::container(cx)
                .child(Story::title_for::<_, CommandPalette<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(CommandPalette::new(ScrollState::default()))
        }
    }
}
