use crate::{prelude::*, ListItemVariant};
use crate::{v_stack, Label, List, ListEntry, ListItem, ListSeparator, ListSubHeader};

pub enum ContextMenuItem {
    Header(SharedString),
    Entry(Label),
    Separator,
}

impl ContextMenuItem {
    fn to_list_item<V: 'static>(self) -> ListItem<V> {
        match self {
            ContextMenuItem::Header(label) => ListSubHeader::new(label).into(),
            ContextMenuItem::Entry(label) => {
                ListEntry::new(label).variant(ListItemVariant::Inset).into()
            }
            ContextMenuItem::Separator => ListSeparator::new().into(),
        }
    }

    pub fn header(label: impl Into<SharedString>) -> Self {
        Self::Header(label.into())
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn entry(label: Label) -> Self {
        Self::Entry(label)
    }
}

#[derive(Component)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
}

impl ContextMenu {
    pub fn new(items: impl IntoIterator<Item = ContextMenuItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        v_stack()
            .flex()
            .bg(cx.theme().colors().elevated_surface)
            .border()
            .border_color(cx.theme().colors().border)
            .child(
                List::new(
                    self.items
                        .into_iter()
                        .map(ContextMenuItem::to_list_item)
                        .collect(),
                )
                .toggle(ToggleState::Toggled),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::story::Story;
    use gpui2::{Div, Render};

    pub struct ContextMenuStory;

    impl Render for ContextMenuStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, ContextMenu>(cx))
                .child(Story::label(cx, "Default"))
                .child(ContextMenu::new([
                    ContextMenuItem::header("Section header"),
                    ContextMenuItem::Separator,
                    ContextMenuItem::entry(Label::new("Some entry")),
                ]))
        }
    }
}
