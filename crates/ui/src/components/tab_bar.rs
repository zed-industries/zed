#![allow(missing_docs)]
use gpui::{AnyElement, ScrollHandle};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    start_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    end_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            start_children: SmallVec::new(),
            children: SmallVec::new(),
            end_children: SmallVec::new(),
            scroll_handle: None,
        }
    }

    pub fn track_scroll(mut self, scroll_handle: ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle);
        self
    }

    pub fn start_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.start_children
    }

    pub fn start_child(mut self, start_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut()
            .push(start_child.into_element().into_any());
        self
    }

    pub fn start_children(
        mut self,
        start_children: impl IntoIterator<Item = impl IntoElement>,
    ) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut().extend(
            start_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }

    pub fn end_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.end_children
    }

    pub fn end_child(mut self, end_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut()
            .push(end_child.into_element().into_any());
        self
    }

    pub fn end_children(mut self, end_children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut().extend(
            end_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TabBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .flex_none()
            .w_full()
            .h(
                // TODO: This should scale with [UiDensity], however tabs,
                // and other tab bar tools need to scale dynamically first.
                rems_from_px(29.),
            )
            .bg(cx.theme().colors().tab_bar_background)
            .when(!self.start_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(Spacing::Small.rems(cx))
                        .px(Spacing::Medium.rems(cx))
                        .border_b_1()
                        .border_r_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.start_children),
                )
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .h_full()
                    .overflow_x_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .border_b_1()
                            .border_color(cx.theme().colors().border),
                    )
                    .child(
                        h_flex()
                            .id("tabs")
                            .flex_grow()
                            .overflow_x_scroll()
                            .when_some(self.scroll_handle, |cx, scroll_handle| {
                                cx.track_scroll(&scroll_handle)
                            })
                            .children(self.children),
                    ),
            )
            .when(!self.end_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(Spacing::Small.rems(cx))
                        .px(Spacing::Medium.rems(cx))
                        .border_b_1()
                        .border_l_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.end_children),
                )
            })
    }
}
