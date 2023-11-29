use crate::ItemHandle;
use gpui::{
    div, AnyView, Div, Entity, EntityId, EventEmitter, ParentElement as _, Render, Styled, View,
    ViewContext, WindowContext,
};
use ui::prelude::*;
use ui::{h_stack, v_stack, Button, Color, Icon, IconButton, Label};

pub enum ToolbarItemEvent {
    ChangeLocation(ToolbarItemLocation),
}

pub trait ToolbarItemView: Render + EventEmitter<ToolbarItemEvent> {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation;

    fn pane_focus_update(&mut self, _pane_focused: bool, _cx: &mut ViewContext<Self>) {}

    /// Number of times toolbar's height will be repeated to get the effective height.
    /// Useful when multiple rows one under each other are needed.
    /// The rows have the same width and act as a whole when reacting to resizes and similar events.
    fn row_count(&self, _cx: &WindowContext) -> usize {
        1
    }
}

trait ToolbarItemViewHandle: Send {
    fn id(&self) -> EntityId;
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation;
    fn focus_changed(&mut self, pane_focused: bool, cx: &mut WindowContext);
    fn row_count(&self, cx: &WindowContext) -> usize;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ToolbarItemLocation {
    Hidden,
    PrimaryLeft,
    PrimaryRight,
    Secondary,
}

pub struct Toolbar {
    active_item: Option<Box<dyn ItemHandle>>,
    hidden: bool,
    can_navigate: bool,
    items: Vec<(Box<dyn ToolbarItemViewHandle>, ToolbarItemLocation)>,
}

impl Toolbar {
    fn left_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::PrimaryLeft {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }

    fn right_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::PrimaryRight {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }
}

impl Render for Toolbar {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        //dbg!(&self.items.len());
        v_stack()
            .border_b()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().toolbar_background)
            .child(
                h_stack()
                    .justify_between()
                    .child(
                        // Toolbar left side
                        h_stack()
                            .border()
                            .border_color(gpui::red())
                            .p_1()
                            .child(Button::new("crates"))
                            .child(Label::new("/").color(Color::Muted))
                            .child(Button::new("workspace2")),
                    )
                    // Toolbar right side
                    .child(
                        h_stack()
                            .p_1()
                            .child(
                                div()
                                    .border()
                                    .border_color(gpui::red())
                                    .child(IconButton::new("buffer-search", Icon::MagnifyingGlass)),
                            )
                            .child(
                                div()
                                    .border()
                                    .border_color(gpui::red())
                                    .child(IconButton::new("inline-assist", Icon::MagicWand)),
                            ),
                    ),
            )
            .children(self.items.iter().map(|(child, _)| child.to_any()))
    }
}

// todo!()
// impl View for Toolbar {
//     fn ui_name() -> &'static str {
//         "Toolbar"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = &theme::current(cx).workspace.toolbar;

//         let mut primary_left_items = Vec::new();
//         let mut primary_right_items = Vec::new();
//         let mut secondary_item = None;
//         let spacing = theme.item_spacing;
//         let mut primary_items_row_count = 1;

//         for (item, position) in &self.items {
//             match *position {
//                 ToolbarItemLocation::Hidden => {}

//                 ToolbarItemLocation::PrimaryLeft { flex } => {
//                     primary_items_row_count = primary_items_row_count.max(item.row_count(cx));
//                     let left_item = ChildView::new(item.as_any(), cx).aligned();
//                     if let Some((flex, expanded)) = flex {
//                         primary_left_items.push(left_item.flex(flex, expanded).into_any());
//                     } else {
//                         primary_left_items.push(left_item.into_any());
//                     }
//                 }

//                 ToolbarItemLocation::PrimaryRight { flex } => {
//                     primary_items_row_count = primary_items_row_count.max(item.row_count(cx));
//                     let right_item = ChildView::new(item.as_any(), cx).aligned().flex_float();
//                     if let Some((flex, expanded)) = flex {
//                         primary_right_items.push(right_item.flex(flex, expanded).into_any());
//                     } else {
//                         primary_right_items.push(right_item.into_any());
//                     }
//                 }

//                 ToolbarItemLocation::Secondary => {
//                     secondary_item = Some(
//                         ChildView::new(item.as_any(), cx)
//                             .constrained()
//                             .with_height(theme.height * item.row_count(cx) as f32)
//                             .into_any(),
//                     );
//                 }
//             }
//         }

//         let container_style = theme.container;
//         let height = theme.height * primary_items_row_count as f32;

//         let mut primary_items = Flex::row().with_spacing(spacing);
//         primary_items.extend(primary_left_items);
//         primary_items.extend(primary_right_items);

//         let mut toolbar = Flex::column();
//         if !primary_items.is_empty() {
//             toolbar.add_child(primary_items.constrained().with_height(height));
//         }
//         if let Some(secondary_item) = secondary_item {
//             toolbar.add_child(secondary_item);
//         }

//         if toolbar.is_empty() {
//             toolbar.into_any_named("toolbar")
//         } else {
//             toolbar
//                 .contained()
//                 .with_style(container_style)
//                 .into_any_named("toolbar")
//         }
//     }
// }

impl Toolbar {
    pub fn new() -> Self {
        Self {
            active_item: None,
            items: Default::default(),
            hidden: false,
            can_navigate: true,
        }
    }

    pub fn set_can_navigate(&mut self, can_navigate: bool, cx: &mut ViewContext<Self>) {
        self.can_navigate = can_navigate;
        cx.notify();
    }

    pub fn add_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        let location = item.set_active_pane_item(self.active_item.as_deref(), cx);
        cx.subscribe(&item, |this, item, event, cx| {
            if let Some((_, current_location)) =
                this.items.iter_mut().find(|(i, _)| i.id() == item.id())
            {
                match event {
                    ToolbarItemEvent::ChangeLocation(new_location) => {
                        if new_location != current_location {
                            *current_location = *new_location;
                            cx.notify();
                        }
                    }
                }
            }
        })
        .detach();
        self.items.push((Box::new(item), location));
        cx.notify();
    }

    pub fn set_active_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        self.active_item = item.map(|item| item.boxed_clone());
        self.hidden = self
            .active_item
            .as_ref()
            .map(|item| !item.show_toolbar(cx))
            .unwrap_or(false);

        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(item, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn focus_changed(&mut self, focused: bool, cx: &mut ViewContext<Self>) {
        for (toolbar_item, _) in self.items.iter_mut() {
            toolbar_item.focus_changed(focused, cx);
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<View<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.to_any().downcast().ok())
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for View<T> {
    fn id(&self) -> EntityId {
        self.entity_id()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        })
    }

    fn focus_changed(&mut self, pane_focused: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| {
            this.pane_focus_update(pane_focused, cx);
            cx.notify();
        });
    }

    fn row_count(&self, cx: &WindowContext) -> usize {
        self.read(cx).row_count(cx)
    }
}

// todo!()
// impl From<&dyn ToolbarItemViewHandle> for AnyViewHandle {
//     fn from(val: &dyn ToolbarItemViewHandle) -> Self {
//         val.as_any().clone()
//     }
// }
