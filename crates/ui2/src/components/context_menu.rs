use crate::{prelude::*, ListItemVariant};
use crate::{v_stack, Label, List, ListEntry, ListItem, ListSeparator, ListSubHeader};

pub enum ContextMenuItem<S: 'static + Send + Sync> {
    Header(SharedString),
    Entry(Label<S>),
    Separator,
}

impl<S: 'static + Send + Sync> ContextMenuItem<S> {
    fn to_list_item(self) -> ListItem<S> {
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

    pub fn entry(label: Label<S>) -> Self {
        Self::Entry(label)
    }
}

#[derive(Element)]
pub struct ContextMenu<S: 'static + Send + Sync> {
    items: Vec<ContextMenuItem<S>>,
}

impl<S: 'static + Send + Sync> ContextMenu<S> {
    pub fn new(items: impl IntoIterator<Item = ContextMenuItem<S>>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        v_stack()
            .flex()
            .bg(theme.elevated_surface)
            .border()
            .border_color(theme.border)
            .child(
                List::new(
                    self.items
                        .drain(..)
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
    use std::marker::PhantomData;

    use crate::story::Story;

    use super::*;

    #[derive(Element)]
    pub struct ContextMenuStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> ContextMenuStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, ContextMenu<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(ContextMenu::new([
                    ContextMenuItem::header("Section header"),
                    ContextMenuItem::Separator,
                    ContextMenuItem::entry(Label::new("Some entry")),
                ]))
        }
    }
}
