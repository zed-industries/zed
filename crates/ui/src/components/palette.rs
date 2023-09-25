use std::marker::PhantomData;

use crate::theme::theme;
use crate::{h_stack, v_stack, Label, LabelColor};
use crate::{prelude::*, LabelSize};
use gpui2::elements::div::ScrollState;
use gpui2::style::{StyleHelpers, Styleable};
use gpui2::{elements::div, IntoElement};
use gpui2::{Element, ParentElement, ViewContext};

#[derive(Element)]
pub struct Palette<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    input_placeholder: &'static str,
    empty_string: &'static str,
    items: Vec<PaletteItem>,
    default_order: OrderMethod,
}

impl<V: 'static> Palette<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        Self {
            view_type: PhantomData,
            scroll_state,
            input_placeholder: "Find something...",
            empty_string: "No items found.",
            items: vec![],
            default_order: OrderMethod::default(),
        }
    }

    pub fn items(mut self, mut items: Vec<PaletteItem>) -> Self {
        items.sort_by_key(|item| item.label);
        self.items = items;
        self
    }

    pub fn placeholder(mut self, input_placeholder: &'static str) -> Self {
        self.input_placeholder = input_placeholder;
        self
    }

    pub fn empty_string(mut self, empty_string: &'static str) -> Self {
        self.empty_string = empty_string;
        self
    }

    // TODO: Hook up sort order
    pub fn default_order(mut self, default_order: OrderMethod) -> Self {
        self.default_order = default_order;
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        v_stack()
            .w_96()
            .rounded_lg()
            .fill(theme.lowest.base.default.background)
            .border()
            .border_color(theme.lowest.base.default.border)
            .child(
                v_stack()
                    .gap_px()
                    .child(v_stack().py_0p5().px_1().child(
                        div().px_2().py_0p5().child(
                            Label::new(self.input_placeholder).color(LabelColor::Placeholder),
                        ),
                    ))
                    .child(div().h_px().w_full().fill(theme.lowest.base.default.border))
                    .child(
                        v_stack()
                            .py_0p5()
                            .px_1()
                            .grow()
                            .max_h_96()
                            .overflow_y_scroll(self.scroll_state.clone())
                            .children(
                                vec![if self.items.is_empty() {
                                    Some(h_stack().justify_between().px_2().py_1().child(
                                        Label::new(self.empty_string).color(LabelColor::Muted),
                                    ))
                                } else {
                                    None
                                }]
                                .into_iter()
                                .flatten(),
                            )
                            .children(self.items.iter().map(|item| {
                                h_stack()
                                    .justify_between()
                                    .px_2()
                                    .py_0p5()
                                    .rounded_lg()
                                    .hover()
                                    .fill(theme.lowest.base.hovered.background)
                                    .active()
                                    .fill(theme.lowest.base.pressed.background)
                                    .child(palette_item(item.label, item.keybinding))
                            })),
                    ),
            )
    }
}

#[derive(Element)]
pub struct PaletteItem {
    pub label: &'static str,
    pub keybinding: Option<&'static str>,
}

pub fn palette_item(label: &'static str, keybinding: Option<&'static str>) -> PaletteItem {
    PaletteItem { label, keybinding }
}

impl PaletteItem {
    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub fn keybinding(mut self, keybinding: Option<&'static str>) -> Self {
        self.keybinding = keybinding;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let keybinding_label = match self.keybinding {
            Some(keybind) => Label::new(keybind)
                .color(LabelColor::Muted)
                .size(LabelSize::Small),
            None => Label::new(""),
        };

        div()
            .flex()
            .flex_row()
            .grow()
            .justify_between()
            .child(Label::new(self.label))
            .child(
                self.keybinding
                    .map(|_| {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_1()
                            .py_0()
                            .my_0p5()
                            .rounded_md()
                            .text_sm()
                            .fill(theme.lowest.on.default.background)
                            .child(keybinding_label)
                    })
                    .unwrap_or_else(|| div()),
            )
    }
}
