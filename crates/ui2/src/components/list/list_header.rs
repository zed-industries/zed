use std::rc::Rc;

use gpui::{ClickEvent, Div};

use crate::prelude::*;
use crate::{h_stack, Disclosure, Icon, IconButton, IconElement, IconSize, Label};

pub enum ListHeaderMeta {
    Tools(Vec<IconButton>),
    // TODO: This should be a button
    Button(Label),
    Text(Label),
}

#[derive(IntoElement)]
pub struct ListHeader {
    label: SharedString,
    left_icon: Option<Icon>,
    meta: Option<ListHeaderMeta>,
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
            meta: None,
            inset: false,
            toggle: None,
            on_toggle: None,
            selected: false,
        }
    }

    pub fn toggle(mut self, toggle: Option<bool>) -> Self {
        self.toggle = toggle;
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(on_toggle));
        self
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    pub fn right_button(self, button: IconButton) -> Self {
        self.meta(Some(ListHeaderMeta::Tools(vec![button])))
    }

    pub fn meta(mut self, meta: Option<ListHeaderMeta>) -> Self {
        self.meta = meta;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for ListHeader {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let meta = match self.meta {
            Some(ListHeaderMeta::Tools(icons)) => div().child(
                h_stack()
                    .gap_2()
                    .items_center()
                    .children(icons.into_iter().map(|i| i.color(Color::Muted))),
            ),
            Some(ListHeaderMeta::Button(label)) => div().child(label),
            Some(ListHeaderMeta::Text(label)) => div().child(label),
            None => div(),
        };

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
                .child(meta),
        )
    }
}
