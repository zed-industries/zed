use std::sync::Arc;

use gpui::{div, DefiniteLength, Hsla, MouseButton, StatefulInteractiveComponent, WindowContext};

use crate::{
    h_stack, prelude::*, Icon, IconButton, IconColor, IconElement, Label, LabelColor,
    LineHeightStyle,
};

/// Provides the flexibility to use either a standard
/// button or an icon button in a given context.
pub enum ButtonOrIconButton<V: 'static> {
    Button(Button<V>),
    IconButton(IconButton<V>),
}

impl<V: 'static> From<Button<V>> for ButtonOrIconButton<V> {
    fn from(value: Button<V>) -> Self {
        Self::Button(value)
    }
}

impl<V: 'static> From<IconButton<V>> for ButtonOrIconButton<V> {
    fn from(value: IconButton<V>) -> Self {
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

pub type ClickHandler<V> = Arc<dyn Fn(&mut V, &mut ViewContext<V>) + Send + Sync>;

struct ButtonHandlers<V: 'static> {
    click: Option<ClickHandler<V>>,
}

unsafe impl<S> Send for ButtonHandlers<S> {}
unsafe impl<S> Sync for ButtonHandlers<S> {}

impl<V: 'static> Default for ButtonHandlers<V> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Component)]
pub struct Button<V: 'static> {
    disabled: bool,
    handlers: ButtonHandlers<V>,
    icon: Option<Icon>,
    icon_position: Option<IconPosition>,
    label: SharedString,
    variant: ButtonVariant,
    width: Option<DefiniteLength>,
    color: Option<LabelColor>,
}

impl<V: 'static> Button<V> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            disabled: false,
            handlers: ButtonHandlers::default(),
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

    pub fn on_click(mut self, handler: ClickHandler<V>) -> Self {
        self.handlers.click = Some(handler);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn color(mut self, color: Option<LabelColor>) -> Self {
        self.color = color;
        self
    }

    pub fn label_color(&self, color: Option<LabelColor>) -> LabelColor {
        if self.disabled {
            LabelColor::Disabled
        } else if let Some(color) = color {
            color
        } else {
            Default::default()
        }
    }

    fn render_label(&self, color: LabelColor) -> Label {
        Label::new(self.label.clone())
            .color(color)
            .line_height_style(LineHeightStyle::UILabel)
    }

    fn render_icon(&self, icon_color: IconColor) -> Option<IconElement> {
        self.icon.map(|i| IconElement::new(i).color(icon_color))
    }

    pub fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let (icon_color, label_color) = match (self.disabled, self.color) {
            (true, _) => (IconColor::Disabled, LabelColor::Disabled),
            (_, None) => (IconColor::Default, LabelColor::Default),
            (_, Some(color)) => (IconColor::from(color), color),
        };

        let mut button = h_stack()
            .id(SharedString::from(format!("{}", self.label)))
            .relative()
            .p_1()
            .text_ui()
            .rounded_md()
            .bg(self.variant.bg_color(cx))
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

        if let Some(click_handler) = self.handlers.click.clone() {
            button = button.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        button
    }
}

#[derive(Component)]
pub struct ButtonGroup<V: 'static> {
    buttons: Vec<Button<V>>,
}

impl<V: 'static> ButtonGroup<V> {
    pub fn new(buttons: Vec<Button<V>>) -> Self {
        Self { buttons }
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let mut el = h_stack().text_ui();

        for button in self.buttons {
            el = el.child(button.render(_view, cx));
        }

        el
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{h_stack, v_stack, LabelColor, Story};
    use gpui::{rems, Div, Render};
    use strum::IntoEnumIterator;

    pub struct ButtonStory;

    impl Render for ButtonStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let states = InteractionState::iter();

            Story::container(cx)
                .child(Story::title_for::<_, Button<Self>>(cx))
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                                            Label::new(state.to_string()).color(LabelColor::Muted),
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
                        .on_click(Arc::new(|_view, _cx| println!("Button clicked."))),
                )
        }
    }
}
