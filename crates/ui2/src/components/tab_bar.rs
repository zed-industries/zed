use gpui::{AnyElement, FocusHandle, Focusable, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    focus_handle: FocusHandle,
    start_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    end_children: SmallVec<[AnyElement; 2]>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>, focus_handle: FocusHandle) -> Self {
        Self {
            id: id.into(),
            focus_handle,
            start_children: SmallVec::new(),
            children: SmallVec::new(),
            end_children: SmallVec::new(),
        }
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
    type Rendered = Focusable<Stateful<Div>>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        const HEIGHT_IN_REMS: f32 = 30. / 16.;

        h_stack()
            .id(self.id)
            .group("tab_bar")
            .track_focus(&self.focus_handle)
            .w_full()
            .h(rems(HEIGHT_IN_REMS))
            .bg(cx.theme().colors().tab_bar_background)
            .child(
                h_stack()
                    .flex_none()
                    .gap_1()
                    .px_1()
                    .border_b()
                    .border_r()
                    .border_color(cx.theme().colors().border)
                    .children(self.start_children),
            )
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
                            .overflow_x_scroll()
                            .children(self.children),
                    ),
            )
            .child(
                h_stack()
                    .flex_none()
                    .gap_1()
                    .px_1()
                    .border_b()
                    .border_l()
                    .border_color(cx.theme().colors().border)
                    .children(self.end_children),
            )
    }
}
