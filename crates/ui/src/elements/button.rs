use std::rc::Rc;

use gpui2::geometry::DefiniteLength;
use gpui2::platform::MouseButton;
use gpui2::{Element, EventContext, Hsla, Interactive, IntoElement, ParentElement, ViewContext};

use crate::prelude::*;
use crate::{h_stack, theme, Icon, IconAsset, IconColor, Label, LabelColor, LabelSize};

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

struct ButtonHandlers<V> {
    click: Option<Rc<dyn Fn(&mut V, &mut EventContext<V>)>>,
}

impl<V> Default for ButtonHandlers<V> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
pub struct Button<V: 'static> {
    label: String,
    variant: ButtonVariant,
    state: InteractionState,
    icon: Option<IconAsset>,
    icon_position: Option<IconPosition>,
    width: Option<DefiniteLength>,
    handlers: ButtonHandlers<V>,
}

impl<V: 'static> Button<V> {
    pub fn new<L>(label: L) -> Self
    where
        L: Into<String>,
    {
        Self {
            label: label.into(),
            variant: Default::default(),
            state: Default::default(),
            icon: None,
            icon_position: None,
            width: Default::default(),
            handlers: ButtonHandlers::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn icon(mut self, icon: IconAsset) -> Self {
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

    pub fn on_click(mut self, handler: impl Fn(&mut V, &mut EventContext<V>) + 'static) -> Self {
        self.handlers.click = Some(Rc::new(handler));
        self
    }

    fn background_color(&self, cx: &mut ViewContext<V>) -> Hsla {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        match (self.variant, self.state) {
            (ButtonVariant::Ghost, InteractionState::Hovered) => {
                theme.lowest.base.hovered.background
            }
            (ButtonVariant::Ghost, InteractionState::Active) => {
                theme.lowest.base.pressed.background
            }
            (ButtonVariant::Filled, InteractionState::Enabled) => {
                theme.lowest.on.default.background
            }
            (ButtonVariant::Filled, InteractionState::Hovered) => {
                theme.lowest.on.hovered.background
            }
            (ButtonVariant::Filled, InteractionState::Active) => theme.lowest.on.pressed.background,
            (ButtonVariant::Filled, InteractionState::Disabled) => {
                theme.lowest.on.disabled.background
            }
            _ => system_color.transparent,
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

    fn render_label(&self) -> Label {
        Label::new(self.label.clone())
            .size(LabelSize::Small)
            .color(self.label_color())
    }

    fn render_icon(&self, icon_color: IconColor) -> Option<Icon> {
        self.icon.map(|i| Icon::new(i).color(icon_color))
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let icon_color = self.icon_color();
        let system_color = SystemColor::new();
        let border_color: Hsla;

        match self.state {
            InteractionState::Focused => {
                border_color = theme.lowest.accent.default.border;
            }
            _ => {
                border_color = system_color.transparent;
            }
        }

        let mut el = h_stack()
            .h_6()
            .px_1()
            .items_center()
            .rounded_md()
            .border()
            .border_color(border_color)
            .fill(self.background_color(cx));

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
            el = el.on_mouse_down(MouseButton::Left, move |view, event, cx| {
                click_handler(view, cx);
            });
        }

        el
    }
}
