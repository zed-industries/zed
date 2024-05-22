use gpui::{AnyView, DefiniteLength};

use crate::{prelude::*, ElevationIndex, SelectableButton, Spacing};
use crate::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle, IconName, IconSize};

use super::button_icon::ButtonIcon;

/// The shape of an [`IconButton`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IconButtonShape {
    Square,
    Wide,
}

#[derive(IntoElement)]
pub struct IconButton {
    base: ButtonLike,
    shape: IconButtonShape,
    icon: IconName,
    icon_size: IconSize,
    icon_color: Color,
    selected_icon: Option<IconName>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        let mut this = Self {
            base: ButtonLike::new(id),
            shape: IconButtonShape::Wide,
            icon,
            icon_size: IconSize::default(),
            icon_color: Color::Default,
            selected_icon: None,
        };
        this.base.base = this.base.base.debug_selector(|| format!("ICON-{:?}", icon));
        this
    }

    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn icon_color(mut self, icon_color: Color) -> Self {
        self.icon_color = icon_color;
        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.selected_icon = icon.into();
        self
    }
}

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Selectable for IconButton {
    fn selected(mut self, selected: bool) -> Self {
        self.base = self.base.selected(selected);
        self
    }
}

impl SelectableButton for IconButton {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.selected_style(style);
        self
    }
}

impl Clickable for IconButton {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl FixedWidth for IconButton {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.base = self.base.width(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for IconButton {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.base = self.base.layer(elevation);
        self
    }
}

impl VisibleOnHover for IconButton {
    fn visible_on_hover(mut self, group_name: impl Into<SharedString>) -> Self {
        self.base = self.base.visible_on_hover(group_name);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;
        let selected_style = self.base.selected_style;

        self.base
            .map(|this| match self.shape {
                IconButtonShape::Square => {
                    let icon_size = self.icon_size.rems() * cx.rem_size();
                    let padding = match self.icon_size {
                        IconSize::Indicator => Spacing::None.px(cx),
                        IconSize::XSmall => Spacing::None.px(cx),
                        IconSize::Small => Spacing::XSmall.px(cx),
                        IconSize::Medium => Spacing::XSmall.px(cx),
                    };

                    this.width((icon_size + padding * 2.).into())
                        .height((icon_size + padding * 2.).into())
                }
                IconButtonShape::Wide => this,
            })
            .child(
                ButtonIcon::new(self.icon)
                    .disabled(is_disabled)
                    .selected(is_selected)
                    .selected_icon(self.selected_icon)
                    .when_some(selected_style, |this, style| this.selected_style(style))
                    .size(self.icon_size)
                    .color(self.icon_color),
            )
    }
}
