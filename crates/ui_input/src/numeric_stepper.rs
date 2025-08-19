use editor::Editor;
use gpui::{ClickEvent, Entity, Focusable};

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

#[derive(IntoElement, RegisterComponent)]
pub struct NumericStepper {
    id: ElementId,
    value: SharedString,
    style: NumericStepperStyle,
    input_field: Entity<Editor>,
    mode: Entity<NumericStepperMode>,
    set_value_to: Box<dyn Fn(usize, &mut App) + 'static>,
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
        set_value_to: impl Fn(usize, &mut App) + 'static,
        on_decrement: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        on_increment: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let id = id.into();
        let value = value.into();

        let (input_field, mode) = window.with_global_id(id.clone(), |global_id, window| {
            // todo! Make sure that using this api is inline and appropriate with the codebase
            window.with_element_state::<(Entity<Editor>, Entity<NumericStepperMode>), _>(
                global_id,
                |mut editor, window| {
                    let state = editor
                        .get_or_insert_with(|| {
                            let mode = cx.new(|_| NumericStepperMode::default());
                            let weak_mode = mode.downgrade();
                            let editor = cx.new(|cx| {
                                let editor = Editor::single_line(window, cx);

                                cx.on_focus_out(
                                    &editor.focus_handle(cx),
                                    window,
                                    move |this, _, window, cx| {
                                        this.clear(window, cx);

                                        weak_mode
                                            .update(cx, |mode, _| *mode = NumericStepperMode::Read)
                                            .ok();
                                    },
                                )
                                .detach();

                                editor
                            });

                            (editor, mode)
                        })
                        .clone();

                    (state.clone(), state)
                },
            )
        });

        Self {
            id,
            value,
            input_field,
            mode,
            set_value_to: Box::new(set_value_to),
            on_decrement: Box::new(on_decrement),
            on_increment: Box::new(on_increment),
            style: NumericStepperStyle::default(),
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
            .id(self.id.clone())
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
                    .child(if matches!(self.mode.read(cx), NumericStepperMode::Read) {
                        div()
                            .id(SharedString::new(format!(
                                "numeric_stepper_label{}",
                                &self.id,
                            )))
                            .child(Label::new(self.value).mx_3())
                            .on_click({
                                let mode = self.mode.downgrade();
                                let input_field_focus_handle = self.input_field.focus_handle(cx);

                                move |click, window, cx| {
                                    if click.click_count() == 2 {
                                        mode.update(cx, |mode, _| {
                                            *mode = NumericStepperMode::Edit;
                                        })
                                        .ok();

                                        window.focus(&input_field_focus_handle);
                                    }
                                }
                            })
                            .into_any_element()
                    } else {
                        div()
                            .child(self.input_field.clone())
                            .child("todo!(This should be removed. It's only here to get input_field to render correctly)")
                            .on_action::<menu::Confirm>({
                                let input_field = self.input_field.downgrade();
                                let mode = self.mode.downgrade();
                                let set_value = self.set_value_to;

                                move |_, _, cx| {
                                    input_field
                                        .update(cx, |input_field, cx| {
                                            if let Some(number) =
                                                input_field.text(cx).parse::<usize>().ok()
                                            {
                                                set_value(number, cx);

                                                mode.update(cx, |mode, _| {
                                                    *mode = NumericStepperMode::Read
                                                })
                                                .ok();
                                            }
                                        })
                                        .ok();
                                }
                            })
                            .w_full()
                            .mx_3()
                            .into_any_element()
                    })
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

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
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
                                move |_, _| {},
                                move |_, _, _| {},
                                move |_, _, _| {},
                                window,
                                cx,
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Outlined",
                            NumericStepper::new(
                                "numeric-stepper-with-border-component-preview",
                                "10",
                                move |_, _| {},
                                move |_, _, _| {},
                                move |_, _, _| {},
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
