use gpui::{AnyElement, FocusHandle, Focusable, Stateful};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct TabBar {
    id: ElementId,
    focus_handle: FocusHandle,
    start_slot: SmallVec<[AnyElement; 2]>,
    end_slot: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>, focus_handle: FocusHandle) -> Self {
        Self {
            id: id.into(),
            focus_handle,
            start_slot: SmallVec::new(),
            end_slot: SmallVec::new(),
            children: SmallVec::new(),
        }
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl IntoIterator<Item = E>) -> Self {
        self.start_slot = element
            .into_iter()
            .map(IntoElement::into_any_element)
            .collect();
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl IntoIterator<Item = E>) -> Self {
        self.end_slot = element
            .into_iter()
            .map(IntoElement::into_any_element)
            .collect();
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

        div()
            .id(self.id)
            .group("tab_bar")
            .track_focus(&self.focus_handle)
            .w_full()
            .h(rems(HEIGHT_IN_REMS))
            .overflow_hidden()
            .flex()
            .flex_none()
            .bg(cx.theme().colors().tab_bar_background)
            .child(
                h_stack()
                    .flex_none()
                    .gap_1()
                    .px_1()
                    .border_b()
                    .border_r()
                    .border_color(cx.theme().colors().border)
                    .children(self.start_slot),
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
                    .child(h_stack().id("tabs").z_index(2).children(self.children)),
            )
            .child(
                h_stack()
                    .flex_none()
                    .gap_1()
                    .px_1()
                    .border_b()
                    .border_l()
                    .border_color(cx.theme().colors().border)
                    .children(self.end_slot),
            )
    }
}
