#![allow(missing_docs)]
use super::button_like::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle};
use crate::internal::prelude::*;
use crate::{ElevationIndex, SelectableButton, Tooltip};
use crate::{IconName, IconSize};
use gpui::{AnyView, DefiniteLength};

use super::button_icon::ButtonIcon;

register_components!(button, [IconButton]);

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

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
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
                    let size = self.icon_size.square(cx);
                    this.width(size.into()).height(size.into())
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

impl ComponentPreview for IconButton {
    fn description() -> impl Into<Option<&'static str>> {
        "An IconButton is a button that displays only an icon. It's used for actions that can be represented by a single icon."
    }

    fn examples() -> Vec<ComponentExampleGroup<Self>> {
        vec![
            example_group_with_title(
                "Basic",
                vec![
                    single_example("Default", IconButton::new("default", IconName::Check)),
                    single_example(
                        "Selected",
                        IconButton::new("selected", IconName::Check).selected(true),
                    ),
                    single_example(
                        "Disabled",
                        IconButton::new("disabled", IconName::Check).disabled(true),
                    ),
                ],
            ),
            example_group_with_title(
                "Shapes",
                vec![
                    single_example(
                        "Square",
                        IconButton::new("square", IconName::Check).shape(IconButtonShape::Square),
                    ),
                    single_example(
                        "Wide",
                        IconButton::new("wide", IconName::Check).shape(IconButtonShape::Wide),
                    ),
                ],
            ),
            example_group_with_title(
                "Sizes",
                vec![
                    single_example(
                        "XSmall",
                        IconButton::new("xsmall", IconName::Check).icon_size(IconSize::XSmall),
                    ),
                    single_example(
                        "Small",
                        IconButton::new("small", IconName::Check).icon_size(IconSize::Small),
                    ),
                    single_example(
                        "Medium",
                        IconButton::new("medium", IconName::Check).icon_size(IconSize::Medium),
                    ),
                ],
            ),
            example_group_with_title(
                "Icon Color",
                vec![
                    single_example("Default", IconButton::new("default_color", IconName::Check)),
                    single_example(
                        "Custom",
                        IconButton::new("custom_color", IconName::Check).icon_color(Color::Success),
                    ),
                ],
            ),
            example_group_with_title(
                "With Tooltip",
                vec![single_example(
                    "Tooltip",
                    IconButton::new("tooltip", IconName::Check)
                        .tooltip(|cx| Tooltip::text("This is a tooltip", cx)),
                )],
            ),
        ]
    }
}
