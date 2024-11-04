#![allow(missing_docs)]

use std::time::Duration;

use crate::prelude::*;
use gpui::{
    hsla, img, pulsating_between, AbsoluteLength, Animation, AnimationExt, AnyElement, Hsla,
    ImageSource, IntoElement, SharedString,
};
use strum::IntoEnumIterator;

const DEFAULT_AVATAR_SIZE: f32 = 16.0;

#[derive(Debug, Clone, PartialEq)]
pub enum AvatarSource {
    Avatar(ImageSource),
    AnonymousAvatar(AnonymousAvatarIcon),
    FallbackAvatar(SharedString),
    LoadingAvatar,
}

pub enum AvatarEffect {
    Grayscale,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum AnonymousAvatarIcon {
    Crown,
    Cat,
    Dragon,
    Alien,
    Ghost,
    #[default]
    Crab,
    Invader,
}

impl Into<IconName> for AnonymousAvatarIcon {
    fn into(self) -> IconName {
        match self {
            AnonymousAvatarIcon::Crown => IconName::AnonymousCrown,
            AnonymousAvatarIcon::Cat => IconName::AnonymousCat,
            AnonymousAvatarIcon::Dragon => IconName::AnonymousDragon,
            AnonymousAvatarIcon::Alien => IconName::AnonymousAlien,
            AnonymousAvatarIcon::Ghost => IconName::AnonymousGhost,
            AnonymousAvatarIcon::Crab => IconName::AnonymousCrab,
            AnonymousAvatarIcon::Invader => IconName::AnonymousInvader,
        }
    }
}

impl TryFrom<IconName> for AnonymousAvatarIcon {
    type Error = String;

    fn try_from(icon: IconName) -> Result<Self, Self::Error> {
        match icon {
            IconName::AnonymousCrown => Ok(AnonymousAvatarIcon::Crown),
            IconName::AnonymousCat => Ok(AnonymousAvatarIcon::Cat),
            IconName::AnonymousDragon => Ok(AnonymousAvatarIcon::Dragon),
            IconName::AnonymousAlien => Ok(AnonymousAvatarIcon::Alien),
            IconName::AnonymousGhost => Ok(AnonymousAvatarIcon::Ghost),
            IconName::AnonymousCrab => Ok(AnonymousAvatarIcon::Crab),
            IconName::AnonymousInvader => Ok(AnonymousAvatarIcon::Invader),
            _ => Err("Icon can't be turned into an AnonymousAvatarIcon.".to_string()),
        }
    }
}

impl AnonymousAvatarIcon {
    /// Returns an anonymous avatar icon based on the provided index.
    pub fn from_index(index: usize) -> Self {
        let variants = Self::iter().collect::<Vec<_>>();
        variants[index % variants.len()]
    }
}

#[derive(IntoElement)]
pub struct Avatar2 {
    source: AvatarSource,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
    grayscale: bool,
}

impl Avatar2 {
    /// Creates a new avatar element with the specified image source.
    pub fn new() -> Self {
        let source = AvatarSource::LoadingAvatar;

        Avatar2 {
            source,
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    /// Creates a new avatar element with an anonymous avatar icon based on the provided index.
    pub fn new_anonymous(index: usize) -> Self {
        let source = AvatarSource::AnonymousAvatar(AnonymousAvatarIcon::from_index(index));

        Avatar2 {
            source,
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    pub fn new_fallback(label: impl Into<SharedString>) -> Self {
        let initials: String = label
            .into()
            .to_uppercase()
            .split_whitespace()
            .filter_map(|s| s.chars().next())
            .take(2)
            .collect();

        let source = match initials.len() {
            0 => AvatarSource::AnonymousAvatar(AnonymousAvatarIcon::default()),
            _ => AvatarSource::FallbackAvatar(initials.into()),
        };

        Avatar2 {
            source,
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    /// Creates a new avatar element with an image source.
    pub fn from_image(src: impl Into<ImageSource>) -> Self {
        let source: AvatarSource = AvatarSource::Avatar(src.into());

        Avatar2 {
            source,
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    /// Applies a grayscale filter to the avatar image.
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    /// Sets the border color of the avatar.
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

impl RenderOnce for Avatar2 {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let rem_size = cx.rem_size();
        let base_size = self.size.unwrap_or_else(|| px(DEFAULT_AVATAR_SIZE).into());
        let content_size = base_size.to_pixels(rem_size);
        let border_width = if self.border_color.is_some() {
            px(2.0)
        } else {
            px(0.0)
        };
        let container_size = content_size + (border_width * 2.0);

        let avatar_bg = cx.theme().colors().element_background;

        let content = match self.source {
            AvatarSource::Avatar(img_source) => img(img_source)
                .size(content_size)
                .rounded_full()
                .when(self.grayscale, |img| img.grayscale(true))
                .into_any_element(),
            AvatarSource::AnonymousAvatar(icon) => Icon::new(icon.into())
                .size(IconSize::XSmall)
                .into_any_element(),
            AvatarSource::FallbackAvatar(initials) => {
                div().text_ui_xs(cx).child(initials).into_any_element()
            }
            AvatarSource::LoadingAvatar => div()
                .size(content_size)
                .bg(cx.theme().colors().ghost_element_background)
                .with_animation(
                    "pulsating-label",
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    move |this, delta| this.bg(avatar_bg.opacity(1.0 - delta)),
                )
                .into_any_element(),
        };

        div()
            .id("avatar")
            .size(container_size)
            .rounded_full()
            .overflow_hidden()
            .when_some(self.border_color, |this, color| {
                this.border(border_width).border_color(color)
            })
            .child(
                div()
                    .size(content_size)
                    .rounded_full()
                    .bg(cx.theme().colors().ghost_element_background)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(content),
            )
            .when_some(self.indicator, |this, indicator| {
                this.child(div().absolute().bottom_0().right_0().child(indicator))
            })
    }
}
