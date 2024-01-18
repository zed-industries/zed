use std::sync::Arc;

use crate::{h_flex, prelude::*, Disclosure, Label};
use gpui::{AnyElement, ClickEvent};

#[derive(IntoElement)]
pub struct ListHeader {
    /// The label of the header.
    label: SharedString,
    /// A slot for content that appears before the label, like an icon or avatar.
    start_slot: Option<AnyElement>,
    /// A slot for content that appears after the label, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
    /// A slot for content that appears on hover after the label
    /// It will obscure the `end_slot` when visible.
    end_hover_slot: Option<AnyElement>,
    toggle: Option<bool>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    inset: bool,
    selected: bool,
}

impl ListHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            start_slot: None,
            end_slot: None,
            end_hover_slot: None,
            inset: false,
            toggle: None,
            on_toggle: None,
            selected: false,
        }
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(on_toggle));
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_hover_slot<E: IntoElement>(mut self, end_hover_slot: impl Into<Option<E>>) -> Self {
        self.end_hover_slot = end_hover_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }
}

impl Selectable for ListHeader {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for ListHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id(self.label.clone())
            .w_full()
            .relative()
            .group("list_header")
            .child(
                div()
                    .h_7()
                    .when(self.inset, |this| this.px_2())
                    .when(self.selected, |this| {
                        this.bg(cx.theme().colors().ghost_element_selected)
                    })
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .children(self.toggle.map(|is_open| {
                                Disclosure::new("toggle", is_open).on_toggle(self.on_toggle.clone())
                            }))
                            .child(
                                div()
                                    .id("label_container")
                                    .flex()
                                    .gap_1()
                                    .items_center()
                                    .children(self.start_slot)
                                    .child(Label::new(self.label.clone()).color(Color::Muted))
                                    .when_some(self.on_toggle, |this, on_toggle| {
                                        this.on_click(move |event, cx| on_toggle(event, cx))
                                    }),
                            ),
                    )
                    .child(h_flex().children(self.end_slot))
                    .when_some(self.end_hover_slot, |this, end_hover_slot| {
                        this.child(
                            div()
                                .absolute()
                                .right_0()
                                .visible_on_hover("list_header")
                                .child(end_hover_slot),
                        )
                    }),
            )
    }
}
