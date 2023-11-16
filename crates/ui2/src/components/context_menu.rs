use crate::{prelude::*, ListItemVariant};
use crate::{v_stack, Label, List, ListEntry, ListItem, ListSeparator, ListSubHeader};

pub enum ContextMenuItem {
    Header(SharedString),
    Entry(Label, Box<dyn gpui::Action>),
    Separator,
}

impl Clone for ContextMenuItem {
    fn clone(&self) -> Self {
        match self {
            ContextMenuItem::Header(name) => ContextMenuItem::Header(name.clone()),
            ContextMenuItem::Entry(label, action) => {
                ContextMenuItem::Entry(label.clone(), action.boxed_clone())
            }
            ContextMenuItem::Separator => ContextMenuItem::Separator,
        }
    }
}
impl ContextMenuItem {
    fn to_list_item<V: 'static>(self) -> ListItem {
        match self {
            ContextMenuItem::Header(label) => ListSubHeader::new(label).into(),
            ContextMenuItem::Entry(label, action) => ListEntry::new(label)
                .variant(ListItemVariant::Inset)
                .on_click(action)
                .into(),
            ContextMenuItem::Separator => ListSeparator::new().into(),
        }
    }

    pub fn header(label: impl Into<SharedString>) -> Self {
        Self::Header(label.into())
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn entry(label: Label, action: impl Action) -> Self {
        Self::Entry(label, Box::new(action))
    }
}

#[derive(Component, Clone)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
}

impl ContextMenu {
    pub fn new(items: impl IntoIterator<Item = ContextMenuItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
    // todo!()
    // cx.add_action(ContextMenu::select_first);
    // cx.add_action(ContextMenu::select_last);
    // cx.add_action(ContextMenu::select_next);
    // cx.add_action(ContextMenu::select_prev);
    // cx.add_action(ContextMenu::confirm);
    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        v_stack()
            .flex()
            .bg(cx.theme().colors().elevated_surface_background)
            .border()
            .border_color(cx.theme().colors().border)
            .child(List::new(
                self.items
                    .into_iter()
                    .map(ContextMenuItem::to_list_item::<V>)
                    .collect(),
            ))
            .on_mouse_down_out(|_, _, cx| cx.dispatch_action(Box::new(menu::Cancel)))
    }
}

use gpui::Action;
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::story::Story;
    use gpui::{action, Div, Render};

    pub struct ContextMenuStory;

    impl Render for ContextMenuStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            #[action]
            struct PrintCurrentDate {}

            Story::container(cx)
                .child(Story::title_for::<_, ContextMenu>(cx))
                .child(Story::label(cx, "Default"))
                .child(ContextMenu::new([
                    ContextMenuItem::header("Section header"),
                    ContextMenuItem::Separator,
                    ContextMenuItem::entry(Label::new("Print current time"), PrintCurrentDate {}),
                ]))
                .on_action(|_, _: &PrintCurrentDate, _| {
                    if let Ok(unix_time) = std::time::UNIX_EPOCH.elapsed() {
                        println!("Current Unix time is {:?}", unix_time.as_secs());
                    }
                })
        }
    }
}
