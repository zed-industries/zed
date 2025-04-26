use crate::prelude::*;

use documented::Documented;
use gpui::{AnyElement, Hsla, ImageSource, Img, IntoElement, Styled, img};

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// use ui::{Avatar, AvatarShape};
///
/// Avatar::new("path/to/image.png")
///     .shape(AvatarShape::Circle)
///     .grayscale(true)
///     .border_color(gpui::red());
/// ```
#[derive(IntoElement, Documented, RegisterComponent)]
pub struct Avatar {
    image: Img,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
}

impl Avatar {
    /// Creates a new avatar element with the specified image source.
    pub fn new(src: impl Into<ImageSource>) -> Self {
        Avatar {
            image: img(src),
            size: None,
            border_color: None,
            indicator: None,
        }
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::{Avatar, AvatarShape};
    ///
    /// let avatar = Avatar::new("path/to/image.png").grayscale(true);
    /// ```
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.image = self.image.grayscale(grayscale);
        self
    }

    /// Sets the border color of the avatar.
    ///
    /// This might be used to match the border to the background color of
    /// the parent element to create the illusion of cropping another
    /// shape underneath (for example in face piles.)
    pub fn border_color(mut self, color: impl Into<Hsla>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    /// Size overrides the avatar size. By default they are 1rem.
    pub fn size<L: Into<AbsoluteLength>>(mut self, size: impl Into<Option<L>>) -> Self {
        self.size = size.into().map(Into::into);
        self
    }

    /// Sets the current indicator to be displayed on the avatar, if any.
    pub fn indicator<E: IntoElement>(mut self, indicator: impl Into<Option<E>>) -> Self {
        self.indicator = indicator.into().map(IntoElement::into_any_element);
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_width = if self.border_color.is_some() {
            px(2.)
        } else {
            px(0.)
        };

        let image_size = self.size.unwrap_or_else(|| rems(1.).into());
        let container_size = image_size.to_pixels(window.rem_size()) + border_width * 2.;

        div()
            .size(container_size)
            .rounded_full()
            .when_some(self.border_color, |this, color| {
                this.border(border_width).border_color(color)
            })
            .child(
                self.image
                    .size(image_size)
                    .rounded_full()
                    .bg(cx.theme().colors().ghost_element_background),
            )
            .children(self.indicator.map(|indicator| div().child(indicator)))
    }
}

use gpui::AnyView;

/// The audio status of an player, for use in representing
/// their status visually on their avatar.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AudioStatus {
    /// The player's microphone is muted.
    Muted,
    /// The player's microphone is muted, and collaboration audio is disabled.
    Deafened,
}

/// An indicator that shows the audio status of a player.
#[derive(IntoElement)]
pub struct AvatarAudioStatusIndicator {
    audio_status: AudioStatus,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
}

impl AvatarAudioStatusIndicator {
    /// Creates a new `AvatarAudioStatusIndicator`
    pub fn new(audio_status: AudioStatus) -> Self {
        Self {
            audio_status,
            tooltip: None,
        }
    }

    /// Sets the tooltip for the indicator.
    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for AvatarAudioStatusIndicator {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_size = IconSize::Indicator;

        let width_in_px = icon_size.rems() * window.rem_size();
        let padding_x = px(4.);

        div()
            .absolute()
            .bottom(rems_from_px(-3.))
            .right(rems_from_px(-6.))
            .w(width_in_px + padding_x)
            .h(icon_size.rems())
            .child(
                h_flex()
                    .id("muted-indicator")
                    .justify_center()
                    .px(padding_x)
                    .py(px(2.))
                    .bg(cx.theme().status().error_background)
                    .rounded_sm()
                    .child(
                        Icon::new(match self.audio_status {
                            AudioStatus::Muted => IconName::MicMute,
                            AudioStatus::Deafened => IconName::AudioOff,
                        })
                        .size(icon_size)
                        .color(Color::Error),
                    )
                    .when_some(self.tooltip, |this, tooltip| {
                        this.tooltip(move |window, cx| tooltip(window, cx))
                    }),
            )
    }
}

/// Represents the availability status of a collaborator.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum CollaboratorAvailability {
    Free,
    Busy,
}

/// Represents the availability and presence status of a collaborator.
#[derive(IntoElement)]
pub struct AvatarAvailabilityIndicator {
    availability: CollaboratorAvailability,
    avatar_size: Option<Pixels>,
}

impl AvatarAvailabilityIndicator {
    /// Creates a new indicator
    pub fn new(availability: CollaboratorAvailability) -> Self {
        Self {
            availability,
            avatar_size: None,
        }
    }

    /// Sets the size of the [`Avatar`](crate::Avatar) this indicator appears on.
    pub fn avatar_size(mut self, size: impl Into<Option<Pixels>>) -> Self {
        self.avatar_size = size.into();
        self
    }
}

impl RenderOnce for AvatarAvailabilityIndicator {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let avatar_size = self.avatar_size.unwrap_or_else(|| window.rem_size());

        // HACK: non-integer sizes result in oval indicators.
        let indicator_size = (avatar_size * 0.4).round();

        div()
            .absolute()
            .bottom_0()
            .right_0()
            .size(indicator_size)
            .rounded(indicator_size)
            .bg(match self.availability {
                CollaboratorAvailability::Free => cx.theme().status().created,
                CollaboratorAvailability::Busy => cx.theme().status().deleted,
            })
    }
}

// View this component preview using `workspace: open component-preview`
impl Component for Avatar {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn description() -> Option<&'static str> {
        Some(Avatar::DOCS)
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let example_avatar = "https://avatars.githubusercontent.com/u/1714999?v=4";

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group(vec![
                        single_example("Default", Avatar::new(example_avatar).into_any_element()),
                        single_example(
                            "Grayscale",
                            Avatar::new(example_avatar)
                                .grayscale(true)
                                .into_any_element(),
                        ),
                        single_example(
                            "Border",
                            Avatar::new(example_avatar)
                                .border_color(cx.theme().colors().border)
                                .into_any_element(),
                        ).description("Can be used to create visual space by setting the border color to match the background, which creates the appearance of a gap around the avatar."),
                    ]),
                    example_group_with_title(
                        "Indicator Styles",
                        vec![
                            single_example(
                                "Muted",
                                Avatar::new(example_avatar)
                                    .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Muted))
                                    .into_any_element(),
                            ).description("Indicates the collaborator's mic is muted."),
                            single_example(
                                "Deafened",
                                Avatar::new(example_avatar)
                                    .indicator(AvatarAudioStatusIndicator::new(
                                        AudioStatus::Deafened,
                                    ))
                                    .into_any_element(),
                            ).description("Indicates that both the collaborator's mic and audio are muted."),
                            single_example(
                                "Availability: Free",
                                Avatar::new(example_avatar)
                                    .indicator(AvatarAvailabilityIndicator::new(
                                        CollaboratorAvailability::Free,
                                    ))
                                    .into_any_element(),
                            ).description("Indicates that the person is free, usually meaning they are not in a call."),
                            single_example(
                                "Availability: Busy",
                                Avatar::new(example_avatar)
                                    .indicator(AvatarAvailabilityIndicator::new(
                                        CollaboratorAvailability::Busy,
                                    ))
                                    .into_any_element(),
                            ).description("Indicates that the person is busy, usually meaning they are in a channel or direct call."),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
