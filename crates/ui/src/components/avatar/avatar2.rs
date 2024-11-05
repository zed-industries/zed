use std::time::Duration;

use crate::prelude::*;
use gpui::{
    hsla, img, pulsating_between, AbsoluteLength, Animation, AnimationExt, AnyElement, FontWeight,
    Hsla, ImageSource, IntoElement, SharedString,
};
use strum::IntoEnumIterator;

const DEFAULT_AVATAR_SIZE: f32 = 20.0;

/// A collection of types of content that can be used for the avatar
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarSource {
    /// The avatar's content is an image
    Avatar(ImageSource),
    /// The avatar's content is a random icon
    AnonymousAvatar(AnonymousAvatarIcon),
    /// The avatar's content is a string (user's initials)
    FallbackAvatar(SharedString),
    /// The avatar's content is loading
    LoadingAvatar,
}

/// A collection of effects that can be applied to the avatar's content
pub enum AvatarEffect {
    /// The avatar's content is rendered in grayscale
    Grayscale,
}

/// A collection of random icons to be used as the anonymous avatars content
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum AnonymousAvatarIcon {
    /// A crown icon
    Crown,
    /// A cat icon
    Cat,
    /// A dragon icon
    Dragon,
    /// An alian icon
    Alien,
    /// A ghost icon
    Ghost,
    /// A crab icon
    #[default]
    Crab,
    /// An alternative alien icon
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

/// An element that renders a user avatar with customizable appearance options.
///
/// # Examples
///
/// ```
/// use ui::Avatar;
///
/// Avatar::new("path/to/image.png")
///     .grayscale(true)
///     .border_color(gpui::red());
/// ```
#[derive(IntoElement)]
pub struct Avatar2 {
    source: AvatarSource,
    size: Option<AbsoluteLength>,
    border_color: Option<Hsla>,
    indicator: Option<AnyElement>,
    grayscale: bool,
}

impl Avatar2 {
    /// Creates a new avatar with image set to option for allowing forcing initials or anonymous icon rendering
    pub fn new(image: impl Into<ImageSource>) -> Self {
        Avatar2 {
            source: AvatarSource::Avatar(image.into()),
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    /// Creates an avatar that can have image empty but filled by a fallback option
    pub fn new_fallback() -> Self {
        Avatar2 {
            source: AvatarSource::LoadingAvatar,
            size: None,
            border_color: None,
            indicator: None,
            grayscale: false,
        }
    }

    /// Uses the user name's first letter as the fallback avatar
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::Avatar;
    ///
    /// div().children(
    ///    PLAYER_HANDLES.iter().enumerate().map(|(ix, handle)| {
    ///        Avatar::new_fallback()
    ///            .fallback_initials(handle.to_string())
    ///            .fallback_anonymous(ix as u32)
    ///    }),
    ///
    /// ```
    pub fn fallback_initials(mut self, initials: impl Into<SharedString>) -> Self {
        let initials = initials.into();
        self.source = AvatarSource::FallbackAvatar(if initials.is_empty() {
            "?".into()
        } else {
            initials
        });
        self
    }

    /// Iterates over a set of random icons as a fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::Avatar;
    ///
    /// div().children((0..=5).map(|ix| {
    ///    Avatar::new_fallback()
    ///        .fallback_anonymous(ix)
    /// })))
    ///
    /// ```
    pub fn fallback_anonymous(mut self, index: u32) -> Self {
        // Only set anonymous avatar if there's no initials
        if !matches!(self.source, AvatarSource::FallbackAvatar(_)) {
            self.source =
                AvatarSource::AnonymousAvatar(AnonymousAvatarIcon::from_index(index as usize));
        }
        self
    }

    /// Uses a pulsating background animation to indicate the loading state
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::Avatar;
    ///
    /// let avatar = Avatar::new("path/to/image.png").loading(true);
    /// ```
    pub fn loading(mut self, is_loading: bool) -> Self {
        if is_loading {
            self.source = AvatarSource::LoadingAvatar;
        }
        self
    }

    /// Applies a grayscale filter to the avatar image.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::Avatar;
    ///
    /// let avatar = Avatar::new("path/to/image.png").grayscale(true);
    /// ```
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
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

    fn base_avatar_style(&self, size: Pixels) -> Div {
        div()
            .size(size)
            .rounded_full()
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center()
    }

    fn render_content(&self, content_size: Pixels, cx: &WindowContext) -> AnyElement {
        match &self.source {
            AvatarSource::Avatar(image) => self.render_image(image, content_size),
            AvatarSource::AnonymousAvatar(icon) => {
                self.render_anonymous_avatar(*icon, content_size, cx)
            }
            AvatarSource::FallbackAvatar(initials) => {
                self.render_fallback_avatar(initials, content_size, cx)
            }
            AvatarSource::LoadingAvatar => self.render_loading_avatar(content_size, cx),
        }
    }

    fn render_image(&self, image: &ImageSource, content_size: Pixels) -> AnyElement {
        self.base_avatar_style(content_size)
            .child(
                img(image.clone())
                    .size(content_size)
                    .rounded_full()
                    .when(self.grayscale, |img| img.grayscale(true)),
            )
            .into_any_element()
    }

    fn render_anonymous_avatar(
        &self,
        icon: AnonymousAvatarIcon,
        content_size: Pixels,
        cx: &WindowContext,
    ) -> AnyElement {
        let color = self.color(cx);

        let bg_color = color.opacity(0.2);

        self.base_avatar_style(content_size)
            .bg(bg_color)
            .child(
                Icon::new(icon.into())
                    .size(IconSize::Indicator)
                    .color(Color::Custom(color)),
            )
            .into_any_element()
    }

    fn render_fallback_avatar(
        &self,
        initials: &str,
        content_size: Pixels,
        cx: &WindowContext,
    ) -> AnyElement {
        let color = self.color(cx);
        let bg_color = color.opacity(0.2);
        let first_letter = initials
            .chars()
            .next()
            .unwrap_or('?')
            .to_string()
            .to_uppercase();

        self.base_avatar_style(content_size)
            .bg(bg_color)
            .child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(color)
                    .text_size(px(10.))
                    .line_height(relative(1.))
                    .font_weight(FontWeight::BOLD)
                    .child(first_letter),
            )
            .into_any_element()
    }

    fn render_loading_avatar(&self, content_size: Pixels, cx: &WindowContext) -> AnyElement {
        let color = self.color(cx);

        self.base_avatar_style(content_size)
            .bg(cx.theme().colors().element_background)
            .with_animation(
                "pulsating-bg",
                Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(pulsating_between(0.3, 0.7)),
                move |this, delta| this.bg(color.opacity(0.8 - delta)),
            )
            .into_any_element()
    }

    fn color(&self, cx: &WindowContext) -> Hsla {
        match &self.source {
            AvatarSource::AnonymousAvatar(icon) => {
                let base_color = cx
                    .theme()
                    .players()
                    .color_for_participant((*icon as u8).into())
                    .cursor;
                if !self.grayscale {
                    base_color
                } else {
                    hsla(0.0, 0.0, 0.5, 1.0)
                }
            }
            AvatarSource::FallbackAvatar(initials) => {
                let index = initials.chars().next().map(|c| c as u8).unwrap_or(0);
                let base_color = cx
                    .theme()
                    .players()
                    .color_for_participant(index.into())
                    .cursor;
                if !self.grayscale {
                    base_color
                } else {
                    hsla(0.0, 0.0, 0.5, 1.0)
                }
            }
            _ => cx.theme().colors().text,
        }
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

        div()
            .id("avatar")
            .size(container_size)
            .rounded_full()
            .when_some(self.border_color, |this, color| {
                this.border(border_width).border_color(color)
            })
            .child(self.render_content(content_size, cx))
            .when_some(self.indicator, |this, indicator| {
                this.child(div().absolute().bottom_0().right_0().child(indicator))
            })
    }
}
