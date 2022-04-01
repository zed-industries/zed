use crate::{ItemHandle, Pane, Settings};
use gpui::{
    elements::*, AnyViewHandle, ElementBox, Entity, MutableAppContext, RenderContext, Subscription,
    View, ViewContext, ViewHandle,
};

pub trait StatusItemView: View {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    );
}

trait StatusItemViewHandle {
    fn to_any(&self) -> AnyViewHandle;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut MutableAppContext,
    );
}

pub struct StatusBar {
    left_items: Vec<Box<dyn StatusItemViewHandle>>,
    right_items: Vec<Box<dyn StatusItemViewHandle>>,
    active_pane: ViewHandle<Pane>,
    _observe_active_pane: Subscription,
}

impl Entity for StatusBar {
    type Event = ();
}

impl View for StatusBar {
    fn ui_name() -> &'static str {
        "StatusBar"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.workspace.status_bar;
        Flex::row()
            .with_children(self.left_items.iter().map(|i| {
                ChildView::new(i.as_ref())
                    .aligned()
                    .contained()
                    .with_margin_right(theme.item_spacing)
                    .boxed()
            }))
            .with_children(self.right_items.iter().map(|i| {
                ChildView::new(i.as_ref())
                    .aligned()
                    .contained()
                    .with_margin_left(theme.item_spacing)
                    .flex_float()
                    .boxed()
            }))
            .contained()
            .with_style(theme.container)
            .constrained()
            .with_height(theme.height)
            .boxed()
    }
}

impl StatusBar {
    pub fn new(active_pane: &ViewHandle<Pane>, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            left_items: Default::default(),
            right_items: Default::default(),
            active_pane: active_pane.clone(),
            _observe_active_pane: cx
                .observe(active_pane, |this, _, cx| this.update_active_pane_item(cx)),
        };
        this.update_active_pane_item(cx);
        this
    }

    pub fn add_left_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        self.left_items.push(Box::new(item));
        cx.notify();
    }

    pub fn add_right_item<T>(&mut self, item: ViewHandle<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        self.right_items.push(Box::new(item));
        cx.notify();
    }

    pub fn set_active_pane(&mut self, active_pane: &ViewHandle<Pane>, cx: &mut ViewContext<Self>) {
        self.active_pane = active_pane.clone();
        self._observe_active_pane =
            cx.observe(active_pane, |this, _, cx| this.update_active_pane_item(cx));
        self.update_active_pane_item(cx);
    }

    fn update_active_pane_item(&mut self, cx: &mut ViewContext<Self>) {
        let active_pane_item = self.active_pane.read(cx).active_item();
        for item in self.left_items.iter().chain(&self.right_items) {
            item.set_active_pane_item(active_pane_item.as_deref(), cx);
        }
    }
}

impl<T: StatusItemView> StatusItemViewHandle for ViewHandle<T> {
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

impl Into<AnyViewHandle> for &dyn StatusItemViewHandle {
    fn into(self) -> AnyViewHandle {
        self.to_any()
    }
}
