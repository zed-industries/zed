use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

use editor::Editor;
use gpui::{ClickEvent, Entity, FocusHandle, Focusable, Modifiers};

use ui::{IconButtonShape, prelude::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericStepperStyle {
    Outlined,
    #[default]
    Ghost,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericStepperMode {
    #[default]
    Read,
    Edit,
}

pub trait NumericStepperType:
    Display + Add<Output = Self> + Sub<Output = Self> + Copy + Clone + Sized + FromStr + 'static
{
    fn default_format(value: &Self) -> String {
        format!("{}", value)
    }
    fn default_step() -> Self;
    fn large_step() -> Self;
    fn small_step() -> Self;
}

macro_rules! impl_numeric_stepper_int {
    ($type:ident) => {
        impl NumericStepperType for $type {
            fn default_step() -> Self {
                1
            }

            fn large_step() -> Self {
                10
            }

            fn small_step() -> Self {
                1
            }
        }
    };
}

macro_rules! impl_numeric_stepper_float {
    ($type:ident) => {
        impl NumericStepperType for $type {
            fn default_format(value: &Self) -> String {
                format!("{:.2}", value)
            }

            fn default_step() -> Self {
                1.0
            }

            fn large_step() -> Self {
                10.0
            }

            fn small_step() -> Self {
                0.1
            }
        }
    };
}

impl_numeric_stepper_float!(f32);
impl_numeric_stepper_float!(f64);
impl_numeric_stepper_int!(isize);
impl_numeric_stepper_int!(usize);
impl_numeric_stepper_int!(i32);
impl_numeric_stepper_int!(u32);
impl_numeric_stepper_int!(i64);
impl_numeric_stepper_int!(u64);

// TODO: Add a new register component macro to support a specific type when using generics
pub struct NumericStepper<T> {
    id: ElementId,
    value: Entity<T>,
    style: NumericStepperStyle,
    focus_handle: FocusHandle,
    mode: Entity<NumericStepperMode>,
    format: Box<dyn FnOnce(&T) -> String>,
    large_step: T,
    small_step: T,
    step: T,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    tab_index: Option<isize>,
}

impl<T: NumericStepperType> NumericStepper<T> {
    pub fn new(
        id: impl Into<ElementId>,
        value: Entity<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let id = id.into();
        let mode = window.use_state(cx, |_, _| NumericStepperMode::default());

        Self {
            id,
            focus_handle: cx.focus_handle(),
            mode,
            value,
            style: NumericStepperStyle::default(),
            format: Box::new(T::default_format),
            large_step: T::large_step(),
            step: T::default_step(),
            small_step: T::small_step(),
            on_reset: None,
            tab_index: None,
        }
    }

    pub fn format(mut self, format: impl FnOnce(&T) -> String + 'static) -> Self {
        self.format = Box::new(format);
        self
    }

    pub fn small_step(mut self, step: T) -> Self {
        self.small_step = step;
        self
    }

    pub fn normal_step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    pub fn large_step(mut self, step: T) -> Self {
        self.large_step = step;
        self
    }

    pub fn style(mut self, style: NumericStepperStyle) -> Self {
        self.style = style;
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

impl<T: NumericStepperType> IntoElement for NumericStepper<T> {
    type Element = gpui::Component<Self>;

    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

impl<T: NumericStepperType> RenderOnce for NumericStepper<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let shape = IconButtonShape::Square;
        let icon_size = IconSize::Small;

        let is_outlined = matches!(self.style, NumericStepperStyle::Outlined);
        let mut tab_index = self.tab_index;

        let get_step = {
            let large_step = self.large_step;
            let step = self.step;
            let small_step = self.small_step;
            move |modifiers: Modifiers| -> T {
                if modifiers.shift {
                    large_step
                } else if modifiers.alt {
                    small_step
                } else {
                    step
                }
            }
        };

        h_flex()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .gap_1()
            .when_some(self.on_reset, |this, on_reset| {
                this.child(
                    IconButton::new("reset", IconName::RotateCcw)
                        .shape(shape)
                        .icon_size(icon_size)
                        .when_some(tab_index.as_mut(), |this, tab_index| {
                            *tab_index += 1;
                            this.tab_index(*tab_index - 1)
                        })
                        .on_click(on_reset),
                )
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
                        let decrement_handler = {
                            let value = self.value.clone();
                            move |click: &ClickEvent, _: &mut Window, cx: &mut App| {
                                let step = get_step(click.modifiers());
                                let current_value = *value.read(cx);
                                value.write(cx, current_value - step);
                            }
                        };

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
                                    .on_click(decrement_handler),
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
                                    .on_click(decrement_handler),
                            )
                        }
                    })
                    .child(match *self.mode.read(cx) {
                        NumericStepperMode::Read => div()
                            .id("numeric_stepper_label")
                            .child(Label::new((self.format)(self.value.read(cx))).mx_3())
                            .on_click({
                                let mode = self.mode.clone();

                                move |click, _, cx| {
                                    if click.click_count() == 2 {
                                        mode.write(cx, NumericStepperMode::Edit);
                                    }
                                }
                            })
                            .into_any_element(),
                        NumericStepperMode::Edit => div()
                            .child(window.use_state(cx, {
                                |window, cx| {
                                    let mut editor = Editor::single_line(window, cx);
                                    editor.set_text(format!("{}", self.value.read(cx)), window, cx);
                                    cx.on_focus_out(&editor.focus_handle(cx), window, {
                                        let mode = self.mode.clone();
                                        let value = self.value.clone();
                                        move |this, _, _window, cx| {
                                            if let Ok(new_value) = this.text(cx).parse::<T>() {
                                                value.write(cx, new_value);
                                            };
                                            mode.write(cx, NumericStepperMode::Read);
                                        }
                                    })
                                    .detach();

                                    window.focus(&editor.focus_handle(cx));

                                    editor
                                }
                            }))
                            .on_action::<menu::Confirm>({
                                let focus = self.focus_handle.clone();
                                move |_, window, _| {
                                    window.focus(&focus);
                                }
                            })
                            .w_full()
                            .mx_3()
                            .into_any_element(),
                    })
                    .map(|increment| {
                        let increment_handler = {
                            let value = self.value.clone();
                            move |click: &ClickEvent, _: &mut Window, cx: &mut App| {
                                let step = get_step(click.modifiers());
                                let current_value = *value.read(cx);
                                value.write(cx, current_value + step);
                            }
                        };

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
                                    .on_click(increment_handler),
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
                                    .on_click(increment_handler),
                            )
                        }
                    }),
            )
    }
}

impl<T: NumericStepperType> Component for NumericStepper<T> {
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

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let first_stepper = window.use_state(cx, |_, _| 10usize);
        let second_stepper = window.use_state(cx, |_, _| 10.0);
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
                                first_stepper,
                                window,
                                cx,
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Outlined",
                            NumericStepper::new(
                                "numeric-stepper-with-border-component-preview",
                                second_stepper,
                                window,
                                cx,
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
