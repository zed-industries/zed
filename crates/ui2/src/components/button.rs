use std::rc::Rc;

use gpui::{
    DefiniteLength, Div, Hsla, MouseButton, MouseDownEvent, RenderOnce, StatefulInteractiveElement,
    WindowContext,
};

use crate::prelude::*;
use crate::{h_stack, Icon, IconButton, IconElement, Label, LineHeightStyle, TextColor};

/// Provides the flexibility to use either a standard
/// button or an icon button in a given context.
pub enum ButtonOrIconButton {
    Button(Button),
    IconButton(IconButton),
}

impl From<Button> for ButtonOrIconButton {
    fn from(value: Button) -> Self {
        Self::Button(value)
    }
}

impl From<IconButton> for ButtonOrIconButton {
    fn from(value: IconButton) -> Self {
        Self::IconButton(value)
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Ghost,
    Filled,
}

impl ButtonVariant {
    pub fn bg_color(&self, cx: &mut WindowContext) -> Hsla {
        match self {
            ButtonVariant::Ghost => cx.theme().colors().ghost_element_background,
            ButtonVariant::Filled => cx.theme().colors().element_background,
        }
    }

    pub fn bg_color_hover(&self, cx: &mut WindowContext) -> Hsla {
        match self {
            ButtonVariant::Ghost => cx.theme().colors().ghost_element_hover,
            ButtonVariant::Filled => cx.theme().colors().element_hover,
        }
    }

    pub fn bg_color_active(&self, cx: &mut WindowContext) -> Hsla {
        match self {
            ButtonVariant::Ghost => cx.theme().colors().ghost_element_active,
            ButtonVariant::Filled => cx.theme().colors().element_active,
        }
    }
}

#[derive(RenderOnce)]
pub struct Button {
    disabled: bool,
    click_handler: Option<Rc<dyn Fn(&MouseDownEvent, &mut WindowContext)>>,
    icon: Option<Icon>,
    icon_position: Option<IconPosition>,
    label: SharedString,
    variant: ButtonVariant,
    width: Option<DefiniteLength>,
    color: Option<TextColor>,
}

impl Component for Button {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let (icon_color, label_color) = match (self.disabled, self.color) {
            (true, _) => (TextColor::Disabled, TextColor::Disabled),
            (_, None) => (TextColor::Default, TextColor::Default),
            (_, Some(color)) => (TextColor::from(color), color),
        };

        let mut button = h_stack()
            .id(SharedString::from(format!("{}", self.label)))
            .relative()
            .p_1()
            .text_ui()
            .rounded_md()
            .bg(self.variant.bg_color(cx))
            .cursor_pointer()
            .hover(|style| style.bg(self.variant.bg_color_hover(cx)))
            .active(|style| style.bg(self.variant.bg_color_active(cx)));

        match (self.icon, self.icon_position) {
            (Some(_), Some(IconPosition::Left)) => {
                button = button
                    .gap_1()
                    .child(self.render_label(label_color))
                    .children(self.render_icon(icon_color))
            }
            (Some(_), Some(IconPosition::Right)) => {
                button = button
                    .gap_1()
                    .children(self.render_icon(icon_color))
                    .child(self.render_label(label_color))
            }
            (_, _) => button = button.child(self.render_label(label_color)),
        }

        if let Some(width) = self.width {
            button = button.w(width).justify_center();
        }

        if let Some(click_handler) = self.click_handler.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |event, cx| {
                click_handler(event, cx);
            });
        }

        button
    }
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            disabled: false,
            click_handler: None,
            icon: None,
            icon_position: None,
            label: label.into(),
            variant: Default::default(),
            width: Default::default(),
            color: None,
        }
    }

    pub fn ghost(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_position(mut self, icon_position: IconPosition) -> Self {
        if self.icon.is_none() {
            panic!("An icon must be present if an icon_position is provided.");
        }
        self.icon_position = Some(icon_position);
        self
    }

    pub fn width(mut self, width: Option<DefiniteLength>) -> Self {
        self.width = width;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.click_handler = Some(Rc::new(handler));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn color(mut self, color: Option<TextColor>) -> Self {
        self.color = color;
        self
    }

    pub fn label_color(&self, color: Option<TextColor>) -> TextColor {
        if self.disabled {
            TextColor::Disabled
        } else if let Some(color) = color {
            color
        } else {
            Default::default()
        }
    }

    fn render_label(&self, color: TextColor) -> Label {
        Label::new(self.label.clone())
            .color(color)
            .line_height_style(LineHeightStyle::UILabel)
    }

    fn render_icon(&self, icon_color: TextColor) -> Option<IconElement> {
        self.icon.map(|i| IconElement::new(i).color(icon_color))
    }
}

#[derive(RenderOnce)]
pub struct ButtonGroup {
    buttons: Vec<Button>,
}

impl Component for ButtonGroup {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let mut group = h_stack();

        for button in self.buttons.into_iter() {
            group = group.child(button.render(cx));
        }

        group
    }
}

impl ButtonGroup {
    pub fn new(buttons: Vec<Button>) -> Self {
        Self { buttons }
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{h_stack, v_stack, Story, TextColor};
    use gpui::{rems, Div, Render};
    use strum::IntoEnumIterator;

    pub struct ButtonStory;

    impl Render for ButtonStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let states = InteractionState::iter();

            Story::container(cx)
                .child(Story::title_for::<Button>(cx))
                .child(
                    div()
                        .flex()
                        .gap_8()
                        .child(
                            div()
                                .child(Story::label(cx, "Ghost (Default)"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label").variant(ButtonVariant::Ghost), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Ghost – Left Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Ghost)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Ghost – Right Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Ghost)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right), // .state(state),
                                        )
                                }))),
                        )
                        .child(
                            div()
                                .child(Story::label(cx, "Filled"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label").variant(ButtonVariant::Filled), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Filled – Left Button"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left), // .state(state),
                                        )
                                })))
                                .child(Story::label(cx, "Filled – Right Button"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right), // .state(state),
                                        )
                                }))),
                        )
                        .child(
                            div()
                                .child(Story::label(cx, "Fixed With"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .width(Some(rems(6.).into())),
                                        )
                                })))
                                .child(Story::label(cx, "Fixed With – Left Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Left)
                                                .width(Some(rems(6.).into())),
                                        )
                                })))
                                .child(Story::label(cx, "Fixed With – Right Icon"))
                                .child(h_stack().gap_2().children(states.clone().map(|state| {
                                    v_stack()
                                        .gap_1()
                                        .child(
                                            Label::new(state.to_string()).color(TextColor::Muted),
                                        )
                                        .child(
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                // .state(state)
                                                .icon(Icon::Plus)
                                                .icon_position(IconPosition::Right)
                                                .width(Some(rems(6.).into())),
                                        )
                                }))),
                        ),
                )
                .child(Story::label(cx, "Button with `on_click`"))
                .child(
                    Button::new("Label")
                        .variant(ButtonVariant::Ghost)
                        .on_click(|_, cx| println!("Button clicked.")),
                )
        }
    }
}
