use std::marker::PhantomData;
use std::sync::Arc;

use gpui3::{DefiniteLength, Hsla, Interactive, MouseButton, WindowContext};

use crate::prelude::*;
use crate::settings::user_settings;
use crate::{h_stack, Icon, IconColor, IconElement, Label, LabelColor};

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

pub type ClickHandler<S> = Arc<dyn Fn(&mut S, &mut ViewContext<S>) + 'static + Send + Sync>;

struct ButtonHandlers<S: 'static + Send + Sync> {
    click: Option<ClickHandler<S>>,
}

impl<S: 'static + Send + Sync> Default for ButtonHandlers<S> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
pub struct Button<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    label: SharedString,
    variant: ButtonVariant,
    state: InteractionState,
    icon: Option<Icon>,
    icon_position: Option<IconPosition>,
    width: Option<DefiniteLength>,
    handlers: ButtonHandlers<S>,
}

impl<S: 'static + Send + Sync + Clone> Button<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
            label: label.into(),
            variant: Default::default(),
            state: Default::default(),
            icon: None,
            icon_position: None,
            width: Default::default(),
            handlers: ButtonHandlers::default(),
        }
    }

    pub fn ghost(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
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

    pub fn on_click(mut self, handler: ClickHandler<S>) -> Self {
        self.handlers.click = Some(handler);
        self
    }

    fn background_color(&self, cx: &mut ViewContext<S>) -> Hsla {
        let color = ThemeColor::new(cx);

        match (self.variant, self.state) {
            (ButtonVariant::Ghost, InteractionState::Enabled) => color.ghost_element,
            (ButtonVariant::Ghost, InteractionState::Focused) => color.ghost_element,
            (ButtonVariant::Ghost, InteractionState::Hovered) => color.ghost_element_hover,
            (ButtonVariant::Ghost, InteractionState::Active) => color.ghost_element_active,
            (ButtonVariant::Ghost, InteractionState::Disabled) => color.filled_element_disabled,
            (ButtonVariant::Filled, InteractionState::Enabled) => color.filled_element,
            (ButtonVariant::Filled, InteractionState::Focused) => color.filled_element,
            (ButtonVariant::Filled, InteractionState::Hovered) => color.filled_element_hover,
            (ButtonVariant::Filled, InteractionState::Active) => color.filled_element_active,
            (ButtonVariant::Filled, InteractionState::Disabled) => color.filled_element_disabled,
        }
    }

    fn label_color(&self) -> LabelColor {
        match self.state {
            InteractionState::Disabled => LabelColor::Disabled,
            _ => Default::default(),
        }
    }

    fn icon_color(&self) -> IconColor {
        match self.state {
            InteractionState::Disabled => IconColor::Disabled,
            _ => Default::default(),
        }
    }

    fn border_color(&self, cx: &WindowContext) -> Hsla {
        let color = ThemeColor::new(cx);

        match self.state {
            InteractionState::Focused => color.border_focused,
            _ => color.border_transparent,
        }
    }

    fn render_label(&self) -> Label<S> {
        Label::new(self.label.clone()).color(self.label_color())
    }

    fn render_icon(&self, icon_color: IconColor) -> Option<IconElement<S>> {
        self.icon.map(|i| IconElement::new(i).color(icon_color))
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let icon_color = self.icon_color();
        let border_color = self.border_color(cx);
        let setting = user_settings();

        let mut el = h_stack()
            .p_1()
            .text_size(ui_size(1.))
            .rounded_md()
            .border()
            .border_color(border_color)
            .bg(self.background_color(cx))
            .hover(|style| {
                let color = ThemeColor::new(cx);

                style.bg(match self.variant {
                    ButtonVariant::Ghost => color.ghost_element_hover,
                    ButtonVariant::Filled => color.filled_element_hover,
                })
            });

        match (self.icon, self.icon_position) {
            (Some(_), Some(IconPosition::Left)) => {
                el = el
                    .gap_1()
                    .child(self.render_label())
                    .children(self.render_icon(icon_color))
            }
            (Some(_), Some(IconPosition::Right)) => {
                el = el
                    .gap_1()
                    .children(self.render_icon(icon_color))
                    .child(self.render_label())
            }
            (_, _) => el = el.child(self.render_label()),
        }

        if let Some(width) = self.width {
            el = el.w(width).justify_center();
        }

        if let Some(click_handler) = self.handlers.click.clone() {
            el = el.on_mouse_down(MouseButton::Left, move |state, event, cx| {
                click_handler(state, cx);
            });
        }

        el
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui3::rems;
    use strum::IntoEnumIterator;

    use crate::{h_stack, v_stack, LabelColor, Story};

    use super::*;

    #[derive(Element)]
    pub struct ButtonStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ButtonStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let states = InteractionState::iter();

            Story::container(cx)
                .child(Story::title_for::<_, Button<S>>(cx))
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
                                            Button::new("Label")
                                                .variant(ButtonVariant::Ghost)
                                                .state(state),
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
                                                .icon_position(IconPosition::Left)
                                                .state(state),
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
                                                .icon_position(IconPosition::Right)
                                                .state(state),
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
                                            Button::new("Label")
                                                .variant(ButtonVariant::Filled)
                                                .state(state),
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
                                                .icon_position(IconPosition::Left)
                                                .state(state),
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
                                                .icon_position(IconPosition::Right)
                                                .state(state),
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
                                                .state(state)
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
                                                .state(state)
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
                                                .state(state)
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
