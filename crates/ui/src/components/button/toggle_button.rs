use gpui::{AnyView, ClickEvent};

use crate::{prelude::*, ButtonLike, ButtonLikeRounding, ElevationIndex};

/// The position of a [`ToggleButton`] within a group of buttons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToggleButtonPosition {
    /// The toggle button is first in the group.
    First,

    /// The toggle button is in the middle of the group (i.e., it is not the first or last toggle button).
    Middle,

    /// The toggle button is last in the group.
    Last,
}

#[derive(IntoElement)]
pub struct ToggleButton {
    base: ButtonLike,
    position_in_group: Option<ToggleButtonPosition>,
    label: SharedString,
    label_color: Option<Color>,
}

impl ToggleButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            position_in_group: None,
            label: label.into(),
            label_color: None,
        }
    }

    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }

    pub fn position_in_group(mut self, position: ToggleButtonPosition) -> Self {
        self.position_in_group = Some(position);
        self
    }

    pub fn first(self) -> Self {
        self.position_in_group(ToggleButtonPosition::First)
    }

    pub fn middle(self) -> Self {
        self.position_in_group(ToggleButtonPosition::Middle)
    }

    pub fn last(self) -> Self {
        self.position_in_group(ToggleButtonPosition::Last)
    }
}

impl Selectable for ToggleButton {
    fn selected(mut self, selected: bool) -> Self {
        self.base = self.base.selected(selected);
        self
    }
}

impl SelectableButton for ToggleButton {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base.selected_style = Some(style);
        self
    }
}

impl Disableable for ToggleButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for ToggleButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl ButtonCommon for ToggleButton {
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

impl RenderOnce for ToggleButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            Color::Selected
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base
            .when_some(self.position_in_group, |this, position| match position {
                ToggleButtonPosition::First => this.rounding(ButtonLikeRounding::Left),
                ToggleButtonPosition::Middle => this.rounding(None),
                ToggleButtonPosition::Last => this.rounding(ButtonLikeRounding::Right),
            })
            .child(
                Label::new(self.label)
                    .color(label_color)
                    .line_height_style(LineHeightStyle::UiLabel),
            )
    }
}
