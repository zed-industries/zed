use gpui::{AnyElement, ScrollHandle, Stateful};
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
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for TabBar {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        const HEIGHT_IN_REMS: f32 = 30. / 16.;

        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .flex_none()
            .w_full()
            .h(rems(HEIGHT_IN_REMS))
            .bg(cx.theme().colors().tab_bar_background)
            .when(!self.start_children.is_empty(), |this| {
                this.child(
                    h_stack()
                        .flex_none()
                        .gap_1()
                        .px_1()
                        .border_b()
                        .border_r()
                        .border_color(cx.theme().colors().border)
                        .children(self.start_children),
                )
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .h_full()
                    .overflow_hidden_x()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .z_index(1)
                            .size_full()
                            .border_b()
                            .border_color(cx.theme().colors().border),
                    )
                    .child(
                        h_stack()
                            .id("tabs")
                            .z_index(2)
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
                    h_stack()
                        .flex_none()
                        .gap_1()
                        .px_1()
                        .border_b()
                        .border_l()
                        .border_color(cx.theme().colors().border)
                        .children(self.end_children),
                )
            })
    }
}
