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
    );
}

trait ToolbarItemViewHandle {
    fn to_any(&self) -> AnyViewHandle;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    );
}

pub struct Toolbar {
    active_pane_item: Option<Box<dyn ItemHandle>>,
    left_items: Vec<Box<dyn ToolbarItemViewHandle>>,
    right_items: Vec<Box<dyn ToolbarItemViewHandle>>,
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
        Flex::row()
            .with_children(self.left_items.iter().map(|i| {
                ChildView::new(i.as_ref())
                    .aligned()
                    .contained()
                    .with_margin_right(theme.item_spacing)
                    .boxed()
            }))
            .with_child(Empty::new().flexible(1., true).boxed())
            .with_children(self.right_items.iter().map(|i| {
                ChildView::new(i.as_ref())
                    .aligned()
                    .contained()
                    .with_margin_left(theme.item_spacing)
                    .boxed()
            }))
            .contained()
            .with_style(theme.container)
            .constrained()
            .with_height(theme.height)
            .boxed()
    }
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            active_pane_item: None,
            left_items: Default::default(),
            right_items: Default::default(),
        }
    }

    pub fn add_left_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        item.set_active_pane_item(self.active_pane_item.as_deref(), cx);
        self.left_items.push(Box::new(item));
        cx.notify();
    }

    pub fn add_right_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + ToolbarItemView,
    {
        item.set_active_pane_item(self.active_pane_item.as_deref(), cx);
        self.right_items.push(Box::new(item));
        cx.notify();
    }

    pub fn set_active_pane_item(
        &mut self,
        item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        self.active_pane_item = item.map(|item| item.boxed_clone());
        for tool in self.left_items.iter().chain(&self.right_items) {
            tool.set_active_pane_item(item, cx);
        }
    }

    pub fn item_of_type<T: ToolbarItemView>(&self) -> Option<ViewHandle<T>> {
        self.left_items
            .iter()
            .chain(&self.right_items)
            .find_map(|tool| tool.to_any().downcast())
    }
}

impl<T: ToolbarItemView> ToolbarItemViewHandle for ViewHandle<T> {
    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    ) {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        });
    }
}

impl Into<AnyViewHandle> for &dyn ToolbarItemViewHandle {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}
