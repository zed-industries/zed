use gpui::{BorderStyle, ClickEvent, Corners, Edges, Hsla, Role, Toggled, canvas, quad};

use crate::{Checkbox, ToggleState, prelude::*};

enum ChoiceCardKind {
    Radio,
    Checkbox,
}

/// A full-width, clickable card representing a single choice, with a leading radio or checkbox
/// indicator, a label, and an optional description.
///
/// The card owns its focus, click handling, and accessibility semantics. Place related cards in
/// a container with the `RadioGroup` (for radio cards) or `Group` role so assistive technology
/// announces them together.
#[derive(IntoElement, RegisterComponent)]
pub struct ChoiceCard {
    id: ElementId,
    kind: ChoiceCardKind,
    label: SharedString,
    description: Option<SharedString>,
    is_selected: bool,
    invalid: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ChoiceCard {
    /// A card for one of several mutually exclusive choices, with a radio indicator.
    /// Place sibling cards in a `Role::RadioGroup` container.
    pub fn radio(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        is_selected: bool,
    ) -> Self {
        Self::new(id, ChoiceCardKind::Radio, label, is_selected)
    }

    /// A card for an independently toggled choice, with a checkbox indicator.
    /// Place sibling cards in a `Role::Group` container.
    pub fn checkbox(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        is_selected: bool,
    ) -> Self {
        Self::new(id, ChoiceCardKind::Checkbox, label, is_selected)
    }

    fn new(
        id: impl Into<ElementId>,
        kind: ChoiceCardKind,
        label: impl Into<SharedString>,
        is_selected: bool,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            label: label.into(),
            description: None,
            is_selected,
            invalid: false,
            on_click: None,
        }
    }

    /// Secondary text shown below the label and announced by assistive technology.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Draws the card border, and the radio indicator if any, in the error color.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// The card is only focusable and shows a pointer cursor once a click handler is set.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn render_radio_indicator(is_selected: bool, border_color: Hsla, background: Hsla) -> Div {
        // Sharing the checkbox slot size keeps radio and checkbox labels aligned.
        div()
            .size(Checkbox::container_size())
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div().size_3().flex_none().child(
                    canvas(
                        |_, _, _| {},
                        move |bounds, _, window, cx| {
                            let bounds = window.pixel_snap_bounds(bounds);
                            let radius = bounds.size.width.min(bounds.size.height) / 2.;
                            window.paint_quad(quad(
                                bounds,
                                Corners::all(radius),
                                background,
                                Edges::all(px(1.)),
                                border_color,
                                BorderStyle::Solid,
                            ));

                            if is_selected {
                                // Equal whole-device-pixel insets keep both circles concentric
                                // when their independently rounded diameters would have different parity.
                                let scale_factor = window.scale_factor();
                                let inset =
                                    px((f32::from(radius) * scale_factor / 2.).round()
                                        / scale_factor);
                                let dot_bounds = bounds.dilate(-inset);
                                window.paint_quad(quad(
                                    dot_bounds,
                                    Corners::all(
                                        dot_bounds.size.width.min(dot_bounds.size.height) / 2.,
                                    ),
                                    Color::Accent.color(cx),
                                    Edges::default(),
                                    Hsla::transparent_black(),
                                    BorderStyle::Solid,
                                ));
                            }
                        },
                    )
                    .size_full(),
                ),
            )
    }
}

impl RenderOnce for ChoiceCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let border_color = if self.invalid {
            Color::Error.color(cx)
        } else {
            colors.border.opacity(0.8)
        };
        let focused_border_color = colors.border_focused;
        let editor_background = colors.editor_background;
        let (background, hover_background) = if self.is_selected {
            let accent = Color::Accent.color(cx);
            (
                editor_background.blend(accent.opacity(0.08)),
                editor_background.blend(accent.opacity(0.1)),
            )
        } else {
            (
                editor_background,
                colors
                    .element_background
                    .blend(colors.editor_foreground.opacity(0.025)),
            )
        };

        let (role, indicator) = match self.kind {
            ChoiceCardKind::Radio => (
                Role::RadioButton,
                Self::render_radio_indicator(self.is_selected, border_color, editor_background)
                    .into_any_element(),
            ),
            ChoiceCardKind::Checkbox => {
                let checkbox_state = if self.is_selected {
                    ToggleState::Selected
                } else {
                    ToggleState::Unselected
                };
                (
                    Role::CheckBox,
                    div()
                        .child(Checkbox::new((self.id.clone(), "checkbox"), checkbox_state))
                        .into_any_element(),
                )
            }
        };

        h_flex()
            .id(self.id)
            .role(role)
            .aria_label(self.label.clone())
            .when_some(self.description.clone(), |this, description| {
                this.aria_description(description)
            })
            .aria_toggled(if self.is_selected {
                Toggled::True
            } else {
                Toggled::False
            })
            .w_full()
            .min_h(rems_from_px(28_f32))
            .items_start()
            .gap_1p5()
            .rounded_sm()
            .border_1()
            .border_color(border_color.opacity(0.5))
            .bg(background)
            .px_2()
            .py_1()
            .hover(move |this| this.bg(hover_background))
            .focus_visible(move |this| this.border_color(focused_border_color))
            .when_some(self.on_click, |this, on_click| {
                this.tab_index(0).cursor_pointer().on_click(on_click)
            })
            .child(indicator)
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .gap_0p5()
                    .child(Label::new(self.label).size(LabelSize::Small))
                    .when_some(self.description, |this, description| {
                        this.child(
                            Label::new(description)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
    }
}

impl Component for ChoiceCard {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        "A clickable card for a single choice, with a radio or checkbox indicator, a label, and an optional description."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_6()
            .child(example_group_with_title(
                "Radio",
                vec![
                    single_example(
                        "Unselected",
                        Self::radio("choice-radio-unselected", "Unselected option", false)
                            .into_any_element(),
                    ),
                    single_example(
                        "Selected",
                        Self::radio("choice-radio-selected", "Selected option", true)
                            .into_any_element(),
                    ),
                    single_example(
                        "With Description",
                        Self::radio("choice-radio-description", "Option", true)
                            .description("A longer explanation of what this option does.")
                            .into_any_element(),
                    ),
                    single_example(
                        "Invalid",
                        Self::radio("choice-radio-invalid", "Invalid option", false)
                            .invalid(true)
                            .into_any_element(),
                    ),
                    single_example(
                        "Selected and Invalid",
                        Self::radio("choice-radio-selected-invalid", "Invalid option", true)
                            .invalid(true)
                            .into_any_element(),
                    ),
                ],
            ))
            .child(example_group_with_title(
                "Checkbox",
                vec![
                    single_example(
                        "Unselected",
                        Self::checkbox("choice-checkbox-unselected", "Unselected option", false)
                            .into_any_element(),
                    ),
                    single_example(
                        "Selected",
                        Self::checkbox("choice-checkbox-selected", "Selected option", true)
                            .into_any_element(),
                    ),
                    single_example(
                        "With Description",
                        Self::checkbox("choice-checkbox-description", "Option", true)
                            .description("A longer explanation of what this option does.")
                            .into_any_element(),
                    ),
                    single_example(
                        "Invalid",
                        Self::checkbox("choice-checkbox-invalid", "Invalid option", false)
                            .invalid(true)
                            .into_any_element(),
                    ),
                    single_example(
                        "Selected and Invalid",
                        Self::checkbox("choice-checkbox-selected-invalid", "Invalid option", true)
                            .invalid(true)
                            .into_any_element(),
                    ),
                ],
            ))
            .child(example_group_with_title(
                "Long Text",
                vec![
                    single_example(
                        "Radio",
                        Self::radio(
                            "choice-radio-long-text",
                            "Allow the agent to read and modify every file in this project, including files ignored by version control",
                            true,
                        )
                        .description(
                            "The agent will be able to create, edit, and delete files without asking for confirmation each time. You can revoke this permission at any point from the agent settings.",
                        )
                        .into_any_element(),
                    ),
                    single_example(
                        "Checkbox",
                        Self::checkbox(
                            "choice-checkbox-long-text",
                            "Include terminal output, diagnostics, and recently opened buffers as additional context",
                            false,
                        )
                        .description(
                            "Additional context can improve results for larger tasks, but it increases the size of each request and may take longer to process.",
                        )
                        .into_any_element(),
                    )
                ],
            ))
            .into_any_element()
    }
}
