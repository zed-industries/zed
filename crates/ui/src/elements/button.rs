use crate::{h_stack, icon, label, prelude::*, Icon, IconAsset, IconColor, Label, LabelColor};
use crate::{theme, LabelSize};
use gpui2::geometry::DefiniteLength;
use gpui2::style::StyleHelpers;
use gpui2::{Element, Hsla, IntoElement, ParentElement, ViewContext};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
}

#[derive(Element)]
pub struct Button {
    label: &'static str,
    variant: ButtonVariant,
    state: InteractionState,
    icon: Option<IconAsset>,
    icon_position: Option<IconPosition>,
    width: Option<DefiniteLength>,
}

pub fn button(label: &'static str) -> Button {
    Button {
        label,
        variant: Default::default(),
        state: Default::default(),
        icon: None,
        icon_position: None,
        width: Default::default(),
    }
}

impl Button {
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

    fn background_color<V>(&self, cx: &mut ViewContext<V>) -> Hsla {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        match (self.variant, self.state) {
            (_, InteractionState::Focused) => theme.lowest.accent.default.background,
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
        label(self.label.clone())
            .size(LabelSize::Small)
            .color(self.label_color())
    }

    fn render_icon(&self, icon_color: IconColor) -> Option<Icon> {
        self.icon.map(|i| icon(i).color(icon_color))
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let icon_color = self.icon_color();

        let mut el = h_stack()
            .h_6()
            .px_1()
            .items_center()
            .rounded_md()
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

        el
    }
}
