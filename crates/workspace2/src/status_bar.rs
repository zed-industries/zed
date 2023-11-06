use std::any::TypeId;

use crate::{ItemHandle, Pane};
use gpui::{
    div, AnyView, Component, Div, ParentElement, Render, Styled, Subscription, View, ViewContext,
    WindowContext,
};
use theme2::ActiveTheme;
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
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .py_0p5()
            .px_1()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .bg(cx.theme().colors().status_bar_background)
            .child(self.render_left_tools(cx))
            .child(self.render_right_tools(cx))
    }
}

impl StatusBar {
    fn render_left_tools(&self, cx: &mut ViewContext<Self>) -> impl Component<Self> {
        div()
            .flex()
            .items_center()
            .gap_1()
            .children(self.left_items.iter().map(|item| item.to_any()))
    }

    fn render_right_tools(&self, cx: &mut ViewContext<Self>) -> impl Component<Self> {
        div()
            .flex()
            .items_center()
            .gap_2()
            .children(self.right_items.iter().map(|item| item.to_any()))
    }
}

// todo!()
// impl View for StatusBar {
//     fn ui_name() -> &'static str {
//         "StatusBar"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let theme = &theme::current(cx).workspace.status_bar;

//         StatusBarElement {
//             left: Flex::row()
//                 .with_children(self.left_items.iter().map(|i| {
//                     ChildView::new(i.as_any(), cx)
//                         .aligned()
//                         .contained()
//                         .with_margin_right(theme.item_spacing)
//                 }))
//                 .into_any(),
//             right: Flex::row()
//                 .with_children(self.right_items.iter().rev().map(|i| {
//                     ChildView::new(i.as_any(), cx)
//                         .aligned()
//                         .contained()
//                         .with_margin_left(theme.item_spacing)
//                 }))
//                 .into_any(),
//         }
//         .contained()
//         .with_style(theme.container)
//         .constrained()
//         .with_height(theme.height)
//         .into_any()
//     }
// }

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

// todo!()
// struct StatusBarElement {
//     left: AnyElement<StatusBar>,
//     right: AnyElement<StatusBar>,
// }

// todo!()
// impl Element<StatusBar> for StatusBarElement {
//     type LayoutState = ();
//     type PaintState = ();

//     fn layout(
//         &mut self,
//         mut constraint: SizeConstraint,
//         view: &mut StatusBar,
//         cx: &mut ViewContext<StatusBar>,
//     ) -> (Vector2F, Self::LayoutState) {
//         let max_width = constraint.max.x();
//         constraint.min = vec2f(0., constraint.min.y());

//         let right_size = self.right.layout(constraint, view, cx);
//         let constraint = SizeConstraint::new(
//             vec2f(0., constraint.min.y()),
//             vec2f(max_width - right_size.x(), constraint.max.y()),
//         );

//         self.left.layout(constraint, view, cx);

//         (vec2f(max_width, right_size.y()), ())
//     }

//     fn paint(
//         &mut self,
//         bounds: RectF,
//         visible_bounds: RectF,
//         _: &mut Self::LayoutState,
//         view: &mut StatusBar,
//         cx: &mut ViewContext<StatusBar>,
//     ) -> Self::PaintState {
//         let origin_y = bounds.upper_right().y();
//         let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

//         let left_origin = vec2f(bounds.lower_left().x(), origin_y);
//         self.left.paint(left_origin, visible_bounds, view, cx);

//         let right_origin = vec2f(bounds.upper_right().x() - self.right.size().x(), origin_y);
//         self.right.paint(right_origin, visible_bounds, view, cx);
//     }

//     fn rect_for_text_range(
//         &self,
//         _: Range<usize>,
//         _: RectF,
//         _: RectF,
//         _: &Self::LayoutState,
//         _: &Self::PaintState,
//         _: &StatusBar,
//         _: &ViewContext<StatusBar>,
//     ) -> Option<RectF> {
//         None
//     }

//     fn debug(
//         &self,
//         bounds: RectF,
//         _: &Self::LayoutState,
//         _: &Self::PaintState,
//         _: &StatusBar,
//         _: &ViewContext<StatusBar>,
//     ) -> serde_json::Value {
//         json!({
//             "type": "StatusBarElement",
//             "bounds": bounds.to_json()
//         })
//     }
// }
