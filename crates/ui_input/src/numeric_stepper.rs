use std::{
    fmt::Display,
    ops::{Add, Sub},
    rc::Rc,
    str::FromStr,
};

use editor::{Editor, EditorStyle};
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
    Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Copy
    + Clone
    + Sized
    + PartialOrd
    + FromStr
    + 'static
{
    fn default_format(value: &Self) -> String {
        format!("{}", value)
    }
    fn default_step() -> Self;
    fn large_step() -> Self;
    fn small_step() -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
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

            fn min_value() -> Self {
                <$type>::MIN
            }

            fn max_value() -> Self {
                <$type>::MAX
            }
        }
    };
}

macro_rules! impl_numeric_stepper_float {
    ($type:ident) => {
        impl NumericStepperType for $type {
            fn default_format(value: &Self) -> String {
                format!("{:^4}", value)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
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

            fn min_value() -> Self {
                <$type>::MIN
            }

            fn max_value() -> Self {
                <$type>::MAX
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

#[derive(RegisterComponent)]
pub struct NumericStepper<T = usize> {
    id: ElementId,
    value: T,
    style: NumericStepperStyle,
    focus_handle: FocusHandle,
    mode: Entity<NumericStepperMode>,
    format: Box<dyn FnOnce(&T) -> String>,
    large_step: T,
    small_step: T,
    step: T,
    min_value: T,
    max_value: T,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_change: Rc<dyn Fn(&T, &mut Window, &mut App) + 'static>,
    tab_index: Option<isize>,
}

impl<T: NumericStepperType> NumericStepper<T> {
    pub fn new(id: impl Into<ElementId>, value: T, window: &mut Window, cx: &mut App) -> Self {
        let id = id.into();

        let (mode, focus_handle) = window.with_id(id.clone(), |window| {
            let mode = window.use_state(cx, |_, _| NumericStepperMode::default());
            let focus_handle = window.use_state(cx, |_, cx| cx.focus_handle());
            (mode, focus_handle)
        });

        Self {
            id,
            mode,
            value,
            focus_handle: focus_handle.read(cx).clone(),
            style: NumericStepperStyle::default(),
            format: Box::new(T::default_format),
            large_step: T::large_step(),
            step: T::default_step(),
            small_step: T::small_step(),
            min_value: T::min_value(),
            max_value: T::max_value(),
            on_reset: None,
            on_change: Rc::new(|_, _, _| {}),
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

    pub fn min(mut self, min: T) -> Self {
        self.min_value = min;
        self
    }

    pub fn max(mut self, max: T) -> Self {
        self.max_value = max;
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

    pub fn on_change(mut self, on_change: impl Fn(&T, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Rc::new(on_change);
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
                            let value = self.value;
                            let on_change = self.on_change.clone();
                            let min = self.min_value;
                            move |click: &ClickEvent, window: &mut Window, cx: &mut App| {
                                let step = get_step(click.modifiers());
                                let new_value = value - step;
                                let new_value = if new_value < min { min } else { new_value };
                                on_change(&new_value, window, cx);
                                window.focus_prev();
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
                    .child(
                        h_flex()
                            .h_8()
                            .min_w_16()
                            .w_full()
                            .border_1()
                            .border_color(cx.theme().colors().border_transparent)
                            .in_focus(|this| this.border_color(cx.theme().colors().border_focused))
                            .child(match *self.mode.read(cx) {
                                NumericStepperMode::Read => h_flex()
                                    .id("numeric_stepper_label")
                                    .flex_1()
                                    .justify_center()
                                    .child(Label::new((self.format)(&self.value)).mx_3())
                                    .when_some(tab_index.as_mut(), |this, tab_index| {
                                        *tab_index += 1;
                                        this.tab_index(*tab_index - 1).focus(|style| {
                                            style.bg(cx.theme().colors().element_hover)
                                        })
                                    })
                                    .on_click({
                                        let _mode = self.mode.clone();
                                        move |click, _, _cx| {
                                            if click.click_count() == 2 || click.is_keyboard() {
                                                // Edit mode is disabled until we implement center text alignment for editor
                                                // mode.write(cx, NumericStepperMode::Edit);
                                            }
                                        }
                                    })
                                    .into_any_element(),
                                NumericStepperMode::Edit => h_flex()
                                    .flex_1()
                                    .child(window.use_state(cx, {
                                        |window, cx| {
                                            let previous_focus_handle = window.focused(cx);
                                            let mut editor = Editor::single_line(window, cx);
                                            let mut style = EditorStyle::default();
                                            style.text.text_align = gpui::TextAlign::Right;
                                            editor.set_style(style, window, cx);

                                            editor.set_text(format!("{}", self.value), window, cx);
                                            cx.on_focus_out(&editor.focus_handle(cx), window, {
                                                let mode = self.mode.clone();
                                                let min = self.min_value;
                                                let max = self.max_value;
                                                let on_change = self.on_change.clone();
                                                move |this, _, window, cx| {
                                                    if let Ok(new_value) =
                                                        this.text(cx).parse::<T>()
                                                    {
                                                        let new_value = if new_value < min {
                                                            min
                                                        } else if new_value > max {
                                                            max
                                                        } else {
                                                            new_value
                                                        };

                                                        if let Some(previous) =
                                                            previous_focus_handle.as_ref()
                                                        {
                                                            window.focus(previous);
                                                        }
                                                        on_change(&new_value, window, cx);
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
                                        move |_, window, _| {
                                            window.blur();
                                        }
                                    })
                                    .into_any_element(),
                            }),
                    )
                    .map(|increment| {
                        let increment_handler = {
                            let value = self.value;
                            let on_change = self.on_change.clone();
                            let max = self.max_value;
                            move |click: &ClickEvent, window: &mut Window, cx: &mut App| {
                                let step = get_step(click.modifiers());
                                let new_value = value + step;
                                let new_value = if new_value > max { max } else { new_value };
                                on_change(&new_value, window, cx);
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
                                IconButton::new("increment", IconName::Plus)
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

impl Component for NumericStepper<usize> {
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
        let first_stepper = window.use_state(cx, |_, _| 100usize);
        let second_stepper = window.use_state(cx, |_, _| 100.0);
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
                                *first_stepper.read(cx),
                                window,
                                cx,
                            )
                            .on_change({
                                let first_stepper = first_stepper.clone();
                                move |value, _, cx| first_stepper.write(cx, *value)
                            })
                            .into_any_element(),
                        ),
                        single_example(
                            "Outlined",
                            NumericStepper::new(
                                "numeric-stepper-with-border-component-preview",
                                *second_stepper.read(cx),
                                window,
                                cx,
                            )
                            .on_change({
                                let second_stepper = second_stepper.clone();
                                move |value, _, cx| second_stepper.write(cx, *value)
                            })
                            .min(1.0)
                            .max(100.0)
                            .style(NumericStepperStyle::Outlined)
                            .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
