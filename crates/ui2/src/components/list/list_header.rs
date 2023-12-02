use std::rc::Rc;

use gpui::{AnyElement, ClickEvent, Div};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::{h_stack, Disclosure, Icon, IconElement, IconSize, Label};

#[derive(IntoElement)]
pub struct ListHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    meta: SmallVec<[AnyElement; 2]>,
    toggle: Option<bool>,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    inset: bool,
    selected: bool,
}

impl ListHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            left_icon: None,
            meta: SmallVec::new(),
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
        self.on_toggle = Some(Rc::new(on_toggle));
        self
    }

    pub fn left_icon(mut self, left_icon: impl Into<Option<Icon>>) -> Self {
        self.left_icon = left_icon.into();
        self
    }

    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta.push(meta.into_any_element());
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
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        h_stack().w_full().relative().child(
            div()
                .h_5()
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
                    h_stack()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .gap_1()
                                .items_center()
                                .children(self.left_icon.map(|i| {
                                    IconElement::new(i)
                                        .color(Color::Muted)
                                        .size(IconSize::Small)
                                }))
                                .child(Label::new(self.label.clone()).color(Color::Muted)),
                        )
                        .children(
                            self.toggle
                                .map(|is_open| Disclosure::new(is_open).on_toggle(self.on_toggle)),
                        ),
                )
                .child(h_stack().gap_2().items_center().children(self.meta)),
        )
    }
}
