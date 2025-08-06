use gpui::ClickEvent;

use crate::{IconButtonShape, prelude::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericStepperStyle {
    Outlined,
    #[default]
    Ghost,
}

#[derive(IntoElement, RegisterComponent)]
pub struct NumericStepper {
    id: ElementId,
    value: SharedString,
    style: NumericStepperStyle,
    on_decrement: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    on_increment: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    /// Whether to reserve space for the reset button.
    reserve_space_for_reset: bool,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    tab_index: Option<isize>,
}

impl NumericStepper {
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        on_decrement: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_increment: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            style: NumericStepperStyle::default(),
            on_decrement: Box::new(on_decrement),
            on_increment: Box::new(on_increment),
            reserve_space_for_reset: false,
            on_reset: None,
            tab_index: None,
        }
    }

    pub fn style(mut self, style: NumericStepperStyle) -> Self {
        self.style = style;
        self
    }

    pub fn reserve_space_for_reset(mut self, reserve_space_for_reset: bool) -> Self {
        self.reserve_space_for_reset = reserve_space_for_reset;
        self
    }

    pub fn on_reset(
        mut self,
        on_reset: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(on_reset));
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }
}

impl RenderOnce for NumericStepper {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        let is_outlined = matches!(self.style, NumericStepperStyle::Outlined);
        let mut tab_index = self.tab_index;

        h_flex()
            .id(self.id)
            .gap_1()
            .map(|element| {
                if let Some(on_reset) = self.on_reset {
                    element.child(
                        IconButton::new("reset", IconName::RotateCcw)
                            .shape(shape)
                            .icon_size(icon_size)
                            .when_some(tab_index.as_mut(), |this, tab_index| {
                                *tab_index += 1;
                                this.tab_index(*tab_index - 1)
                            })
                            .on_click(on_reset),
                    )
                } else if self.reserve_space_for_reset {
                    element.child(
                        h_flex()
                            .size(icon_size.square(window, cx))
                            .flex_none()
                            .into_any_element(),
                    )
                } else {
                    element
                }
            })
            .child(
                h_flex()
                    .gap_1()
                    .rounded_sm()
                    .map(|this| {
                        if is_outlined {
                            this.overflow_hidden()
                                .bg(cx.theme().colors().surface_background)
                                .border_1()
                                .border_color(cx.theme().colors().border_variant)
                        } else {
                            this.px_1().bg(cx.theme().colors().editor_background)
                        }
                    })
                    .map(|decrement| {
                        if is_outlined {
                            decrement.child(
                                h_flex()
                                    .id("decrement_button")
                                    .p_1p5()
                                    .size_full()
                                    .justify_center()
                                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                                    .border_r_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(Icon::new(IconName::Dash).size(IconSize::Small))
                                    .when_some(tab_index.as_mut(), |this, tab_index| {
                                        *tab_index += 1;
                                        this.tab_index(*tab_index - 1).focus(|style| {
                                            style.bg(cx.theme().colors().element_hover)
                                        })
                                    })
                                    .on_click(self.on_decrement),
                            )
                        } else {
                            decrement.child(
                                IconButton::new("decrement", IconName::Dash)
                                    .shape(shape)
                                    .icon_size(icon_size)
                                    .when_some(tab_index.as_mut(), |this, tab_index| {
                                        *tab_index += 1;
                                        this.tab_index(*tab_index - 1)
                                    })
                                    .on_click(self.on_decrement),
                            )
                        }
                    })
                    .child(Label::new(self.value).mx_3())
                    .map(|increment| {
                        if is_outlined {
                            increment.child(
                                h_flex()
                                    .id("increment_button")
                                    .p_1p5()
                                    .size_full()
                                    .justify_center()
                                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                                    .border_l_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(Icon::new(IconName::Plus).size(IconSize::Small))
                                    .when_some(tab_index.as_mut(), |this, tab_index| {
                                        *tab_index += 1;
                                        this.tab_index(*tab_index - 1).focus(|style| {
                                            style.bg(cx.theme().colors().element_hover)
                                        })
                                    })
                                    .on_click(self.on_increment),
                            )
                        } else {
                            increment.child(
                                IconButton::new("increment", IconName::Dash)
                                    .shape(shape)
                                    .icon_size(icon_size)
                                    .when_some(tab_index.as_mut(), |this, tab_index| {
                                        *tab_index += 1;
                                        this.tab_index(*tab_index - 1)
                                    })
                                    .on_click(self.on_increment),
                            )
                        }
                    }),
            )
    }
}

impl Component for NumericStepper {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn name() -> &'static str {
        "Numeric Stepper"
    }

    fn sort_name() -> &'static str {
        Self::name()
    }

    fn description() -> Option<&'static str> {
        Some("A button used to increment or decrement a numeric value.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Styles",
                    vec![
                        single_example(
                            "Default",
                            NumericStepper::new(
                                "numeric-stepper-component-preview",
                                "10",
                                move |_, _, _| {},
                                move |_, _, _| {},
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Outlined",
                            NumericStepper::new(
                                "numeric-stepper-with-border-component-preview",
                                "10",
                                move |_, _, _| {},
                                move |_, _, _| {},
                            )
                            .style(NumericStepperStyle::Outlined)
                            .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
