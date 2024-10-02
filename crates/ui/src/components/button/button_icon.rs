use crate::{prelude::*, Icon, IconName, IconSize};

/// An icon that appears within a button.
///
/// Can be used as either an icon alongside a label, like in [`Button`](crate::Button),
/// or as a standalone icon, like in [`IconButton`](crate::IconButton).
#[derive(IntoElement)]
pub(super) struct ButtonIcon {
    icon: IconName,
    size: IconSize,
    color: Color,
    disabled: bool,
    selected: bool,
    selected_icon: Option<IconName>,
    selected_icon_color: Option<Color>,
    selected_style: Option<ButtonStyle>,
}

impl ButtonIcon {
    pub fn new(icon: IconName) -> Self {
        Self {
            icon,
            size: IconSize::default(),
            color: Color::default(),
            disabled: false,
            selected: false,
            selected_icon: None,
            selected_icon_color: None,
            selected_style: None,
        }
    }

    pub fn size(mut self, size: impl Into<Option<IconSize>>) -> Self {
        if let Some(size) = size.into() {
            self.size = size;
        }

        self
    }

    pub fn color(mut self, color: impl Into<Option<Color>>) -> Self {
        if let Some(color) = color.into() {
            self.color = color;
        }

        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.selected_icon = icon.into();
        self
    }

    pub fn selected_icon_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.selected_icon_color = color.into();
        self
    }
}

impl Disableable for ButtonIcon {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for ButtonIcon {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonIcon {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl RenderOnce for ButtonIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let icon = self
            .selected_icon
            .filter(|_| self.selected)
            .unwrap_or(self.icon);

        let icon_color = if self.disabled {
            Color::Disabled
        } else if self.selected_style.is_some() && self.selected {
            self.selected_style.unwrap().into()
        } else if self.selected {
            self.selected_icon_color.unwrap_or(Color::Selected)
        } else {
            self.color
        };

        Icon::new(icon).size(self.size).color(icon_color)
    }
}
