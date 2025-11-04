use std::rc::Rc;

use gpui::{App, ElementId, IntoElement, RenderOnce};
use heck::ToTitleCase as _;
use ui::{
    ButtonSize, ContextMenu, DropdownMenu, DropdownStyle, FluentBuilder as _, IconPosition, px,
};

#[derive(IntoElement)]
pub struct EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    id: ElementId,
    current_value: T,
    variants: &'static [T],
    labels: &'static [&'static str],
    should_do_title_case: bool,
    tab_index: Option<isize>,
    on_change: Rc<dyn Fn(T, &mut App) + 'static>,
}

impl<T> EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    pub fn new(
        id: impl Into<ElementId>,
        current_value: T,
        variants: &'static [T],
        labels: &'static [&'static str],
        on_change: impl Fn(T, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            current_value,
            variants,
            labels,
            should_do_title_case: true,
            tab_index: None,
            on_change: Rc::new(on_change),
        }
    }

    pub fn title_case(mut self, title_case: bool) -> Self {
        self.should_do_title_case = title_case;
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }
}

impl<T> RenderOnce for EnumVariantDropdown<T>
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
{
    fn render(self, window: &mut ui::Window, cx: &mut ui::App) -> impl gpui::IntoElement {
        let current_value_label = self.labels[self
            .variants
            .iter()
            .position(|v| *v == self.current_value)
            .unwrap()];

        let context_menu = window.use_keyed_state(current_value_label, cx, |window, cx| {
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (&value, &label) in std::iter::zip(self.variants, self.labels) {
                    let on_change = self.on_change.clone();
                    let current_value = self.current_value;
                    menu = menu.toggleable_entry(
                        if self.should_do_title_case {
                            label.to_title_case()
                        } else {
                            label.to_string()
                        },
                        value == current_value,
                        IconPosition::End,
                        None,
                        move |_, cx| {
                            on_change(value, cx);
                        },
                    );
                }
                menu
            })
        });

        DropdownMenu::new(
            self.id,
            if self.should_do_title_case {
                current_value_label.to_title_case()
            } else {
                current_value_label.to_string()
            },
            context_menu,
        )
        .when_some(self.tab_index, |elem, tab_index| elem.tab_index(tab_index))
        .trigger_size(ButtonSize::Medium)
        .style(DropdownStyle::Outlined)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(2.0),
        })
        .into_any_element()
    }
}
