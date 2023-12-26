use gpui::{AnyView, DefiniteLength};

use crate::{prelude::*, IconPosition};
use crate::{
    ButtonCommon, ButtonLike, ButtonSize, ButtonStyle, Icon, IconSize, Label, LineHeightStyle,
};

use super::button_icon::ButtonIcon;

#[derive(IntoElement)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    label_color: Option<Color>,
    label_size: Option<LabelSize>,
    selected_label: Option<SharedString>,
    icon: Option<Icon>,
    icon_position: Option<IconPosition>,
    icon_size: Option<IconSize>,
    icon_color: Option<Color>,
    selected_icon: Option<Icon>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            label: label.into(),
            label_color: None,
            label_size: None,
            selected_label: None,
            icon: None,
            icon_position: None,
            icon_size: None,
            icon_color: None,
            selected_icon: None,
        }
    }

    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }

    pub fn label_size(mut self, label_size: impl Into<Option<LabelSize>>) -> Self {
        self.label_size = label_size.into();
        self
    }

    pub fn selected_label<L: Into<SharedString>>(mut self, label: impl Into<Option<L>>) -> Self {
        self.selected_label = label.into().map(Into::into);
        self
    }

    pub fn icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn icon_position(mut self, icon_position: impl Into<Option<IconPosition>>) -> Self {
        self.icon_position = icon_position.into();
        self
    }

    pub fn icon_size(mut self, icon_size: impl Into<Option<IconSize>>) -> Self {
        self.icon_size = icon_size.into();
        self
    }

    pub fn icon_color(mut self, icon_color: impl Into<Option<Color>>) -> Self {
        self.icon_color = icon_color.into();
        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.selected_icon = icon.into();
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.base = self.base.selected(selected);
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for Button {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl FixedWidth for Button {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.base = self.base.width(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for Button {
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
}

impl RenderOnce for Button {
    type Rendered = ButtonLike;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label = self
            .selected_label
            .filter(|_| is_selected)
            .unwrap_or(self.label);

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            Color::Selected
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base.child(
            h_stack()
                .gap_1()
                .when(self.icon_position.is_some(), |this| {
                    this.children(self.icon.map(|icon| {
                        ButtonIcon::new(icon)
                            .disabled(is_disabled)
                            .selected(is_selected)
                            .selected_icon(self.selected_icon)
                            .size(self.icon_size)
                            .color(self.icon_color)
                    }))
                })
                .child(
                    Label::new(label)
                        .color(label_color)
                        .size(self.label_size.unwrap_or_default())
                        .line_height_style(LineHeightStyle::UiLabel),
                )
                .when(!self.icon_position.is_some(), |this| {
                    this.children(self.icon.map(|icon| {
                        ButtonIcon::new(icon)
                            .disabled(is_disabled)
                            .selected(is_selected)
                            .selected_icon(self.selected_icon)
                            .size(self.icon_size)
                            .color(self.icon_color)
                    }))
                }),
        )
    }
}
