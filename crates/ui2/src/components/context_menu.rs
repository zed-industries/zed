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

    fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        let theme = theme(cx);

        v_stack()
            .flex()
            .bg(theme.elevated_surface)
            .border()
            .border_color(theme.border)
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
    use crate::story::Story;

    use super::*;

    #[derive(Component)]
    pub struct ContextMenuStory;

    impl ContextMenuStory {
        pub fn new() -> Self {
            Self
        }

        fn render<S: 'static>(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
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
