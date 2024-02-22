use crate::ItemHandle;
use gpui::{
    AnyView, Entity, EntityId, EventEmitter, ParentElement as _, Render, Styled, View, ViewContext,
    WindowContext,
};
use ui::prelude::*;
use ui::{h_flex, v_flex};

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
    fn has_any_visible_items(&self) -> bool {
        self.items
            .iter()
            .any(|(_item, location)| *location != ToolbarItemLocation::Hidden)
    }

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

    fn secondary_items(&self) -> impl Iterator<Item = &dyn ToolbarItemViewHandle> {
        self.items.iter().filter_map(|(item, location)| {
            if *location == ToolbarItemLocation::Secondary {
                Some(item.as_ref())
            } else {
                None
            }
        })
    }
}

impl Render for Toolbar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if !self.has_any_visible_items() {
            return div();
        }

        let secondary_item = self.secondary_items().next().map(|item| item.to_any());

        let has_left_items = self.left_items().count() > 0;
        let has_right_items = self.right_items().count() > 0;

        v_flex()
            .p_2()
            .when(has_left_items || has_right_items, |this| this.gap_2())
            .border_b()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().toolbar_background)
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .when(has_left_items, |this| {
                        this.child(
                            h_flex()
                                .flex_auto()
                                .justify_start()
                                .overflow_x_hidden()
                                .children(self.left_items().map(|item| item.to_any())),
                        )
                    })
                    .when(has_right_items, |this| {
                        this.child(
                            h_flex()
                                // We're using `flex_none` here to prevent some flickering that can occur when the
                                // size of the left items container changes.
                                .when_else(has_left_items, Div::flex_none, Div::flex_auto)
                                .justify_end()
                                .children(self.right_items().map(|item| item.to_any())),
                        )
                    }),
            )
            .children(secondary_item)
    }
}

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
