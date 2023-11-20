use crate::prelude::*;
use crate::{example_editor_actions, OrderMethod, Palette};

#[derive(RenderOnce)]
pub struct CommandPalette {
    id: ElementId,
}

impl Component for CommandPalette {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().id(self.id.clone()).child(
            Palette::new("palette")
                .items(example_editor_actions())
                .placeholder("Execute a command...")
                .empty_string("No items found.")
                .default_order(OrderMethod::Ascending),
        )
    }
}

impl CommandPalette {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

use gpui::{Div, RenderOnce, Stateful};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};

    use crate::Story;

    use super::*;

    pub struct CommandPaletteStory;

    impl Render for CommandPaletteStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, CommandPalette>(cx))
                .child(Story::label(cx, "Default"))
                .child(CommandPalette::new("command-palette"))
        }
    }
}
