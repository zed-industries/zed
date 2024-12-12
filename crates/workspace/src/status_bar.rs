use crate::{ItemHandle, Pane};
use gpui::{
    AnyModel, Decorations, IntoElement, Model, ParentElement, Render, Styled, Subscription,
};
use std::any::TypeId;
use theme::CLIENT_SIDE_DECORATION_ROUNDING;
use ui::{h_flex, prelude::*};
use util::ResultExt;

pub trait StatusItemView: Render {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        model: &Model<Self>,
        cx: &mut AppContext,
    );
}

trait StatusItemViewHandle: Send {
    fn to_any(&self) -> AnyModel;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    );
    fn item_type(&self) -> TypeId;
}

pub struct StatusBar {
    left_items: Vec<Box<dyn StatusItemViewHandle>>,
    right_items: Vec<Box<dyn StatusItemViewHandle>>,
    active_pane: Model<Pane>,
    _observe_active_pane: Subscription,
}

impl Render for StatusBar {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .gap(DynamicSpacing::Base08.rems(cx))
            .py(DynamicSpacing::Base04.rems(cx))
            .px(DynamicSpacing::Base08.rems(cx))
            .bg(cx.theme().colors().status_bar_background)
            .map(|el| match cx.window_decorations() {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el
                    .when(!(tiling.bottom || tiling.right), |el| {
                        el.rounded_br(CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    .when(!(tiling.bottom || tiling.left), |el| {
                        el.rounded_bl(CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    // This border is to avoid a transparent gap in the rounded corners
                    .mb(px(-1.))
                    .border_b(px(1.0))
                    .border_color(cx.theme().colors().status_bar_background),
            })
            .child(self.render_left_tools(cx))
            .child(self.render_right_tools(cx))
    }
}

impl StatusBar {
    fn render_left_tools(&self, model: &Model<Self>, cx: &mut AppContext) -> impl IntoElement {
        h_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
            .overflow_x_hidden()
            .children(self.left_items.iter().map(|item| item.to_any()))
    }

    fn render_right_tools(&self, model: &Model<Self>, cx: &mut AppContext) -> impl IntoElement {
        h_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
            .children(self.right_items.iter().rev().map(|item| item.to_any()))
    }
}

impl StatusBar {
    pub fn new(active_pane: &Model<Pane>, model: &Model<Self>, cx: &mut AppContext) -> Self {
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

    pub fn add_left_item<T>(&mut self, item: Model<T>, model: &Model<Self>, cx: &mut AppContext)
    where
        T: 'static + StatusItemModel,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.left_items.push(Box::new(item));
        model.notify(cx);
    }

    pub fn item_of_type<T: StatusItemView>(&self) -> Option<Model<T>> {
        self.left_items
            .iter()
            .chain(self.right_items.iter())
            .find_map(|item| item.to_any().clone().downcast().log_err())
    }

    pub fn position_of_item<T>(&self) -> Option<usize>
    where
        T: StatusItemView,
    {
        for (index, item) in self.left_items.iter().enumerate() {
            if item.item_type() == TypeId::of::<T>() {
                return Some(index);
            }
        }
        for (index, item) in self.right_items.iter().enumerate() {
            if item.item_type() == TypeId::of::<T>() {
                return Some(index + self.left_items.len());
            }
        }
        None
    }

    pub fn insert_item_after<T>(
        &mut self,
        position: usize,
        item: Model<T>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) where
        T: 'static + StatusItemModel,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        if position < self.left_items.len() {
            self.left_items.insert(position + 1, Box::new(item))
        } else {
            self.right_items
                .insert(position + 1 - self.left_items.len(), Box::new(item))
        }
        model.notify(cx)
    }

    pub fn remove_item_at(&mut self, position: usize, model: &Model<Self>, cx: &mut AppContext) {
        if position < self.left_items.len() {
            self.left_items.remove(position);
        } else {
            self.right_items.remove(position - self.left_items.len());
        }
        model.notify(cx);
    }

    pub fn add_right_item<T>(&mut self, item: Model<T>, model: &Model<Self>, cx: &mut AppContext)
    where
        T: 'static + StatusItemModel,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.right_items.push(Box::new(item));
        model.notify(cx);
    }

    pub fn set_active_pane(
        &mut self,
        active_pane: &Model<Pane>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.active_pane = active_pane.clone();
        self._observe_active_pane =
            cx.observe(active_pane, |this, _, cx| this.update_active_pane_item(cx));
        self.update_active_pane_item(cx);
    }

    fn update_active_pane_item(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        let active_pane_item = self.active_pane.read(cx).active_item();
        for item in self.left_items.iter().chain(&self.right_items) {
            item.set_active_pane_item(active_pane_item.as_deref(), cx);
        }
    }
}

impl<T: StatusItemView> StatusItemViewHandle for Model<T> {
    fn to_any(&self) -> AnyModel {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) {
        self.update(cx, |this, model, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        });
    }

    fn item_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl From<&dyn StatusItemViewHandle> for AnyModel {
    fn from(val: &dyn StatusItemViewHandle) -> Self {
        val.to_any().clone()
    }
}
