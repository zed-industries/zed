use std::rc::Rc;

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString};
use heck::ToTitleCase as _;
use ui::{
    ButtonSize, ContextMenu, Disableable as _, DropdownMenu, DropdownStyle, FluentBuilder as _,
    IconPosition, px,
};

#[derive(IntoElement)]
pub struct EnumVariantDropdown {
    id: ElementId,
    selected_index: usize,
    labels: &'static [&'static str],
    should_do_title_case: bool,
    tab_index: Option<isize>,
    disabled: bool,
    aria_label: Option<SharedString>,
    aria_description: Option<SharedString>,
    on_change: Rc<dyn Fn(usize, &mut ui::Window, &mut App) + 'static>,
}

impl EnumVariantDropdown {
    pub fn new<T>(
        id: impl Into<ElementId>,
        current_value: T,
        variants: &'static [T],
        labels: &'static [&'static str],
        on_change: impl Fn(T, &mut ui::Window, &mut App) + 'static,
    ) -> Self
    where
        T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + Sync + 'static,
    {
        let selected_index = variants
            .iter()
            .position(|v| *v == current_value)
            .unwrap_or(0);
        Self::new_indexed(id, selected_index, labels, move |index, window, cx| {
            on_change(variants[index], window, cx)
        })
    }

    pub fn new_indexed(
        id: impl Into<ElementId>,
        selected_index: usize,
        labels: &'static [&'static str],
        on_change: impl Fn(usize, &mut ui::Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            selected_index,
            labels,
            should_do_title_case: true,
            tab_index: None,
            disabled: false,
            aria_label: None,
            aria_description: None,
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the label announced by assistive technology.
    /// Defaults to the currently selected value's label.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Sets the supplementary description announced by assistive technology
    /// after the combobox's name, role, and value (e.g. a setting subtitle).
    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.aria_description = Some(description.into());
        self
    }
}

impl RenderOnce for EnumVariantDropdown {
    fn render(self, window: &mut ui::Window, cx: &mut ui::App) -> impl gpui::IntoElement {
        let current_value_label = self.labels[self.selected_index];

        let context_menu = window.use_keyed_state(current_value_label, cx, |window, cx| {
            ContextMenu::new(window, cx, move |mut menu, _, _| {
                for (index, &label) in self.labels.iter().enumerate() {
                    let on_change = self.on_change.clone();
                    menu = menu.toggleable_entry(
                        if self.should_do_title_case {
                            label.to_title_case()
                        } else {
                            label.to_string()
                        },
                        index == self.selected_index,
                        IconPosition::End,
                        None,
                        move |window, cx| {
                            on_change(index, window, cx);
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
        .when_some(self.aria_label, |this, label| this.aria_label(label))
        .when_some(self.aria_description, |this, description| {
            this.aria_description(description)
        })
        .disabled(self.disabled)
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
