use gpui::{AbsoluteLength, Hsla, IntoElement, Pixels};
#[allow(missing_docs)]
use gpui::{Img, SharedString};
use strum::IntoEnumIterator;

use crate::IconName;

pub enum AvatarSource {
    Avatar(Img),
    AnonymousAvatar(AnonymousAvatarIcon),
    FallbackAvatar(SharedString),
    LoadingAvatar,
}

pub enum AvatarEffect {
    Grayscale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum AnonymousAvatarIcon {
    Crown,
    Cat,
    Dragon,
    Alien,
    Ghost,
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
}
