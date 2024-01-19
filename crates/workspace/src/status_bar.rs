use crate::{ItemHandle, Pane};
use gpui::{
    AnyView, IntoElement, ParentElement, Render, Styled, Subscription, View, ViewContext,
    WindowContext,
};
use std::any::TypeId;
use ui::{h_flex, prelude::*};
use util::ResultExt;

pub trait StatusItemView: Render {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    );
}

trait StatusItemViewHandle: Send {
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    );
    fn item_type(&self) -> TypeId;
}

pub struct StatusBar {
    left_items: Vec<Box<dyn StatusItemViewHandle>>,
    right_items: Vec<Box<dyn StatusItemViewHandle>>,
    active_pane: View<Pane>,
    _observe_active_pane: Subscription,
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .gap_2()
            .py_0p5()
            .px_1()
            .h_8()
            .bg(cx.theme().colors().status_bar_background)
            .child(self.render_left_tools(cx))
            .child(self.render_right_tools(cx))
    }
}

impl StatusBar {
    fn render_left_tools(&self, _: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .overflow_x_hidden()
            .children(self.left_items.iter().map(|item| item.to_any()))
    }

    fn render_right_tools(&self, _: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .children(self.right_items.iter().rev().map(|item| item.to_any()))
    }
}

impl StatusBar {
    pub fn new(active_pane: &View<Pane>, cx: &mut ViewContext<Self>) -> Self {
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

    pub fn add_left_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.left_items.push(Box::new(item));
        cx.notify();
    }

    pub fn item_of_type<T: StatusItemView>(&self) -> Option<View<T>> {
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
        return None;
    }

    pub fn insert_item_after<T>(
        &mut self,
        position: usize,
        item: View<T>,
        cx: &mut ViewContext<Self>,
    ) where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        if position < self.left_items.len() {
            self.left_items.insert(position + 1, Box::new(item))
        } else {
            self.right_items
                .insert(position + 1 - self.left_items.len(), Box::new(item))
        }
        cx.notify()
    }

    pub fn remove_item_at(&mut self, position: usize, cx: &mut ViewContext<Self>) {
        if position < self.left_items.len() {
            self.left_items.remove(position);
        } else {
            self.right_items.remove(position - self.left_items.len());
        }
        cx.notify();
    }

    pub fn add_right_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.right_items.push(Box::new(item));
        cx.notify();
    }

    pub fn set_active_pane(&mut self, active_pane: &View<Pane>, cx: &mut ViewContext<Self>) {
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

impl<T: StatusItemView> StatusItemViewHandle for View<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        });
    }

    fn item_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl From<&dyn StatusItemViewHandle> for AnyView {
    fn from(val: &dyn StatusItemViewHandle) -> Self {
        val.to_any().clone()
    }
}
