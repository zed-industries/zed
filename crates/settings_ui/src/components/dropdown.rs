use std::rc::Rc;

use gpui::{App, ElementId, IntoElement, RenderOnce};
use heck::ToTitleCase as _;
use ui::{
    ButtonSize, ContextMenu, ContextMenuEntry, DocumentationSide, DropdownMenu, DropdownStyle,
    FluentBuilder as _, IconPosition, Label, px,
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
    descriptions: Vec<Option<&'static str>>,
    should_do_title_case: bool,
    tab_index: Option<isize>,
    on_change: Rc<dyn Fn(T, &mut ui::Window, &mut App) + 'static>,
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
        on_change: impl Fn(T, &mut ui::Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            current_value,
            variants,
            labels,
            descriptions: Vec::new(),
            should_do_title_case: true,
            tab_index: None,
            on_change: Rc::new(on_change),
        }
    }

    pub fn descriptions(mut self, descriptions: Vec<Option<&'static str>>) -> Self {
        self.descriptions = descriptions;
        self
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
                for (index, (&value, &label)) in
                    std::iter::zip(self.variants, self.labels).enumerate()
                {
                    let on_change = self.on_change.clone();
                    let current_value = self.current_value;
                    let display_label = if self.should_do_title_case {
                        label.to_title_case()
                    } else {
                        label.to_string()
                    };

                    let description = self.descriptions.get(index).copied().flatten();

                    let entry = ContextMenuEntry::new(display_label)
                        .toggleable(IconPosition::End, value == current_value)
                        .handler(move |window, cx| {
                            on_change(value, window, cx);
                        });

                    let entry = if let Some(description) = description {
                        entry.documentation_aside(DocumentationSide::Left, move |_| {
                            Label::new(description).into_any_element()
                        })
                    } else {
                        entry
                    };

                    menu = menu.item(entry);
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
