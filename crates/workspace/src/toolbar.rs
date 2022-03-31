use crate::{ItemHandle, Settings};
use gpui::{
    elements::*, AnyViewHandle, ElementBox, Entity, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};

pub trait ToolbarItemView: View {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation;

    fn location_for_event(
        &self,
        _event: &Self::Event,
        current_location: ToolbarItemLocation,
    ) -> ToolbarItemLocation {
        current_location
    }
}

trait ToolbarItemViewHandle {
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    ) -> ToolbarItemLocation;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolbarItemLocation {
    Hidden,
    PrimaryLeft,
    PrimaryRight,
    Secondary,
}

pub struct Toolbar {
    active_pane_item: Option<Box<dyn ItemHandle>>,
    items: Vec<(Box<dyn ToolbarItemViewHandle>, ToolbarItemLocation)>,
}

impl Entity for Toolbar {
    type Event = ();
}

impl View for Toolbar {
    fn ui_name() -> &'static str {
        "Toolbar"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.workspace.toolbar;

        let mut primary_left_items = Vec::new();
        let mut primary_right_items = Vec::new();
        let mut secondary_item = None;

        for (item, position) in &self.items {
            match position {
                ToolbarItemLocation::Hidden => {}
                ToolbarItemLocation::PrimaryLeft => primary_left_items.push(item),
                ToolbarItemLocation::PrimaryRight => primary_right_items.push(item),
                ToolbarItemLocation::Secondary => secondary_item = Some(item),
            }
        }

        Flex::column()
            .with_child(
                Flex::row()
                    .with_children(primary_left_items.iter().map(|i| {
                        ChildView::new(i.as_ref())
                            .aligned()
                            .contained()
                            .with_margin_right(theme.item_spacing)
                            .boxed()
                    }))
                    .with_children(primary_right_items.iter().map(|i| {
                        ChildView::new(i.as_ref())
                            .aligned()
                            .contained()
                            .with_margin_left(theme.item_spacing)
                            .flex_float()
                            .boxed()
                    }))
                    .constrained()
                    .with_height(theme.height)
                    .boxed(),
            )
            .with_children(secondary_item.map(|item| {
                ChildView::new(item.as_ref())
                    .constrained()
                    .with_height(theme.height)
                    .boxed()
            }))
            .contained()
            .with_style(theme.container)
            .boxed()
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            active_pane_item: None,
            items: Default::default(),
        }
    }

    pub fn add_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        let location = item.set_active_pane_item(self.active_pane_item.as_deref(), cx);
        cx.subscribe(&item, |this, item, event, cx| {
            if let Some((_, current_location)) =
                this.items.iter_mut().find(|(i, _)| i.id() == item.id())
            {
                let new_location = item.read(cx).location_for_event(event, *current_location);
                if new_location != *current_location {
                    *current_location = new_location;
                    cx.notify();
                }
            }
        })
        .detach();
        self.items.push((Box::new(item), location));
        cx.notify();
    }

    pub fn set_active_pane_item(
        &mut self,
        pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        self.active_pane_item = pane_item.map(|item| item.boxed_clone());
        for (toolbar_item, current_location) in self.items.iter_mut() {
            let new_location = toolbar_item.set_active_pane_item(pane_item, cx);
            if new_location != *current_location {
                *current_location = new_location;
                cx.notify();
            }
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<ViewHandle<T>> {
        self.items
            .iter()
            .find_map(|(item, _)| item.to_any().downcast())
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for ViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    ) -> ToolbarItemLocation {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        })
    }
}

impl Into<AnyViewHandle> for &dyn ToolbarItemViewHandle {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}
