mod decorated_icon;
mod icon_decoration;

use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use decorated_icon::*;
use gpui::{AnimationElement, AnyElement, Hsla, IntoElement, Rems, Transformation, img, svg};
pub use icon_decoration::*;
pub use icons::*;

use crate::{Indicator, prelude::*};

#[derive(IntoElement)]
pub enum AnyIcon {
    Icon(Icon),
    AnimatedIcon(AnimationElement<Icon>),
}

impl AnyIcon {
    /// Returns a new [`AnyIcon`] after applying the given mapping function
    /// to the contained [`Icon`].
    pub fn map(self, f: impl FnOnce(Icon) -> Icon) -> Self {
        match self {
            Self::Icon(icon) => Self::Icon(f(icon)),
            Self::AnimatedIcon(animated_icon) => Self::AnimatedIcon(animated_icon.map_element(f)),
        }
    }
}

impl From<Icon> for AnyIcon {
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}

impl From<AnimationElement<Icon>> for AnyIcon {
    fn from(value: AnimationElement<Icon>) -> Self {
        Self::AnimatedIcon(value)
    }
}

impl RenderOnce for AnyIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self {
            Self::Icon(icon) => icon.into_any_element(),
            Self::AnimatedIcon(animated_icon) => animated_icon.into_any_element(),
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    /// 10px
    Indicator,
    /// 12px
    XSmall,
    /// 14px
    Small,
    #[default]
    /// 16px
    Medium,
    /// 48px
    XLarge,
    Custom(Rems),
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::Indicator => rems_from_px(10.),
            IconSize::XSmall => rems_from_px(12.),
            IconSize::Small => rems_from_px(14.),
            IconSize::Medium => rems_from_px(16.),
            IconSize::XLarge => rems_from_px(48.),
            IconSize::Custom(size) => size,
        }
    }

    /// Returns the individual components of the square that contains this [`IconSize`].
    ///
    /// The returned tuple contains:
    ///   1. The length of one side of the square
    ///   2. The padding of one side of the square
    pub fn square_components(&self, window: &mut Window, cx: &mut App) -> (Pixels, Pixels) {
        let icon_size = self.rems() * window.rem_size();
        let padding = match self {
            IconSize::Indicator => DynamicSpacing::Base00.px(cx),
            IconSize::XSmall => DynamicSpacing::Base02.px(cx),
            IconSize::Small => DynamicSpacing::Base02.px(cx),
            IconSize::Medium => DynamicSpacing::Base02.px(cx),
            IconSize::XLarge => DynamicSpacing::Base02.px(cx),
            // TODO: Wire into dynamic spacing
            IconSize::Custom(size) => size.to_pixels(window.rem_size()),
        };

        (icon_size, padding)
    }

    /// Returns the length of a side of the square that contains this [`IconSize`], with padding.
    pub fn square(&self, window: &mut Window, cx: &mut App) -> Pixels {
        let (icon_size, padding) = self.square_components(window, cx);

        icon_size + padding * 2.
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::new(icon)
    }
}

/// The source of an icon.
enum IconSource {
    /// An SVG embedded in the Zed binary.
    Svg(SharedString),
    /// An image file located at the specified path.
    ///
    /// Currently our SVG renderer is missing support for the following features:
    /// 1. Loading SVGs from external files.
    /// 2. Rendering polychrome SVGs.
    ///
    /// In order to support icon themes, we render the icons as images instead.
    Image(Arc<Path>),
}

impl IconSource {
    fn from_path(path: impl Into<SharedString>) -> Self {
        let path = path.into();
        if path.starts_with("icons/") {
            Self::Svg(path)
        } else {
            Self::Image(Arc::from(PathBuf::from(path.as_ref())))
        }
    }
}

#[derive(IntoElement, RegisterComponent)]
pub struct Icon {
    source: IconSource,
    color: Color,
    size: Rems,
    transformation: Transformation,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Self {
            source: IconSource::Svg(icon.path().into()),
            color: Color::default(),
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            source: IconSource::from_path(path),
            color: Color::default(),
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size.rems();
        self
    }

    /// Sets a custom size for the icon, in [`Rems`].
    ///
    /// Not to be exposed outside of the `ui` crate.
    pub(crate) fn custom_size(mut self, size: Rems) -> Self {
        self.size = size;
        self
    }

    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.transformation = transformation;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.source {
            IconSource::Svg(path) => svg()
                .with_transformation(self.transformation)
                .size(self.size)
                .flex_none()
                .path(path)
                .text_color(self.color.color(cx))
                .into_any_element(),
            IconSource::Image(path) => img(path)
                .size(self.size)
                .flex_none()
                .text_color(self.color.color(cx))
                .into_any_element(),
        }
    }
}

#[derive(IntoElement)]
pub struct IconWithIndicator {
    icon: Icon,
    indicator: Option<Indicator>,
    indicator_border_color: Option<Hsla>,
}

impl IconWithIndicator {
    pub fn new(icon: Icon, indicator: Option<Indicator>) -> Self {
        Self {
            icon,
            indicator,
            indicator_border_color: None,
        }
    }

    pub fn indicator(mut self, indicator: Option<Indicator>) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn indicator_color(mut self, color: Color) -> Self {
        if let Some(indicator) = self.indicator.as_mut() {
            indicator.color = color;
        }
        self
    }

    pub fn indicator_border_color(mut self, color: Option<Hsla>) -> Self {
        self.indicator_border_color = color;
        self
    }
}

impl RenderOnce for IconWithIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let indicator_border_color = self
            .indicator_border_color
            .unwrap_or_else(|| cx.theme().colors().elevated_surface_background);

        div()
            .relative()
            .child(self.icon)
            .when_some(self.indicator, |this, indicator| {
                this.child(
                    div()
                        .absolute()
                        .size_2p5()
                        .border_2()
                        .border_color(indicator_border_color)
                        .rounded_full()
                        .bottom_neg_0p5()
                        .right_neg_0p5()
                        .child(indicator),
                )
            })
    }
}

impl Component for Icon {
    fn scope() -> ComponentScope {
        ComponentScope::Images
    }

    fn description() -> Option<&'static str> {
        Some(
            "A versatile icon component that supports SVG and image-based icons with customizable size, color, and transformations.",
        )
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Sizes",
                        vec![
                            single_example("Default", Icon::new(IconName::Star).into_any_element()),
                            single_example(
                                "Small",
                                Icon::new(IconName::Star)
                                    .size(IconSize::Small)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Large",
                                Icon::new(IconName::Star)
                                    .size(IconSize::XLarge)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Colors",
                        vec![
                            single_example("Default", Icon::new(IconName::Bell).into_any_element()),
                            single_example(
                                "Custom Color",
                                Icon::new(IconName::Bell)
                                    .color(Color::Error)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
