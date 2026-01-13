use gpui::{AnyElement, Pixels, ScrollHandle};
use smallvec::SmallVec;

use crate::prelude::*;

/// A vertical tab bar component for displaying tabs on the side of a pane.
#[derive(IntoElement)]
pub struct SideTabBar {
    id: ElementId,
    header_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    footer_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
    width: Pixels,
}

impl SideTabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header_children: SmallVec::new(),
            children: SmallVec::new(),
            footer_children: SmallVec::new(),
            scroll_handle: None,
            width: px(200.),
        }
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    pub fn track_scroll(mut self, scroll_handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle.clone());
        self
    }

    pub fn header_child(mut self, child: impl IntoElement) -> Self {
        self.header_children.push(child.into_any_element());
        self
    }

    pub fn header_children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.header_children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    pub fn footer_child(mut self, child: impl IntoElement) -> Self {
        self.footer_children.push(child.into_any_element());
        self
    }

    pub fn footer_children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.footer_children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl ParentElement for SideTabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for SideTabBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .id(self.id)
            .group("side_tab_bar")
            .flex_none()
            .h_full()
            .w(self.width)
            .bg(cx.theme().colors().tab_bar_background)
            .border_l_1()
            .border_color(cx.theme().colors().border)
            // Header section (nav buttons, etc.)
            .when(!self.header_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .w_full()
                        .py(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.header_children),
                )
            })
            // Scrollable tab list
            .child(
                div()
                    .id("side_tab_bar_scroll")
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .when_some(self.scroll_handle, |this, handle| this.track_scroll(&handle))
                    .child(v_flex().w_full().children(self.children)),
            )
            // Footer section (action buttons)
            .when(!self.footer_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .w_full()
                        .py(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .children(self.footer_children),
                )
            })
    }
}
