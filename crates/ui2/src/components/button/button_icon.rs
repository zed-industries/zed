use crate::{prelude::*, Icon, IconElement, IconSize};

/// An icon that appears within a button.
///
/// Can be used as either an icon alongside a label, like in [`Button`](crate::Button),
/// or as a standalone icon, like in [`IconButton`](crate::IconButton).
#[derive(IntoElement)]
pub(super) struct ButtonIcon {
    icon: Icon,
    size: IconSize,
    color: Color,
    disabled: bool,
    selected: bool,
    selected_icon: Option<Icon>,
}

impl ButtonIcon {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            size: IconSize::default(),
            color: Color::default(),
            disabled: false,
            selected: false,
            selected_icon: None,
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

    pub fn selected_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.selected_icon = icon.into();
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

impl RenderOnce for ButtonIcon {
    type Output = IconElement;

    fn render(self, _cx: &mut WindowContext) -> Self::Output {
        let icon = self
            .selected_icon
            .filter(|_| self.selected)
            .unwrap_or(self.icon);

        let icon_color = if self.disabled {
            Color::Disabled
        } else if self.selected {
            Color::Selected
        } else {
            self.color
        };

        IconElement::new(icon).size(self.size).color(icon_color)
    }
}
