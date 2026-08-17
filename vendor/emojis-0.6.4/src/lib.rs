//! ✨ Lookup emoji in *O(1)* time, access metadata and GitHub shortcodes,
//! iterate over all emoji.
//!
//! # Features
//!
//! - Lookup up emoji by Unicode value
//! - Lookup up emoji by GitHub shortcode ([gemoji] v4.1.0)
//! - Access emoji metadata: name, unicode version, group, skin tone, [gemoji] shortcodes
//! - Iterate over emojis in Unicode CLDR order
//! - Iterate over emojis in an emoji group, e.g. "Smileys & Emotion" or "Flags"
//! - Iterate over the skin tones for an emoji
//! - Select a specific skin tone for an emoji
//! - Uses [Unicode v16.0](https://unicode.org/emoji/charts-16.0/emoji-released.html) emoji specification
//!
//! [gemoji]: https://github.com/github/gemoji
//!
//! # Getting started
//!
//! First, add the `emojis` crate to your Cargo manifest.
//!
//! ```sh
//! cargo add emojis
//! ```
//!
//! Simply use the `get()` function to lookup emojis by Unicode value.
//! ```
//! let rocket = emojis::get("🚀").unwrap();
//! ```
//!
//! Or the `get_by_shortcode()` function to lookup emojis by [gemoji] shortcode.
//!
//! ```
//! let rocket = emojis::get_by_shortcode("rocket").unwrap();
//! ```
//!
//! These operations take *Ο(1)* time.
//!
//! # MSRV
//!
//! Currently the minimum supported Rust version is 1.60 due to the dependency
//! on [`phf`]. The policy of this crate is to only increase the MSRV in a
//! breaking release.
//!
//! # Breaking changes
//!
//! When [gemoji] or the Unicode version is upgraded this is not considered a
//! breaking change, instead you should make sure to use
//! [`unicode_version()`][Emoji::unicode_version] to filter out newer versions.
//!
//! # Examples
//!
//! See [examples/replace.rs] for an example that replaces `:gemoji:` names with
//! real emojis in text.
//!
//! ```sh
//! $ echo "launch :rocket:" | cargo run --example replace
//! launch 🚀
//! ```
//!
//! [`get()`][get] and [`get_by_shortcode()`][get_by_shortcode] return an
//! [`Emoji`] struct which contains various metadata regarding the emoji.
//! ```
//! let hand = emojis::get("🤌").unwrap();
//! assert_eq!(hand.as_str(), "\u{1f90c}");
//! assert_eq!(hand.as_bytes(), &[0xf0, 0x9f, 0xa4, 0x8c]);
//! assert_eq!(hand.name(), "pinched fingers");
//! assert_eq!(hand.unicode_version(), emojis::UnicodeVersion::new(13, 0));
//! assert_eq!(hand.group(), emojis::Group::PeopleAndBody);
//! assert_eq!(hand.skin_tone(), Some(emojis::SkinTone::Default));
//! assert_eq!(hand.shortcode(), Some("pinched_fingers"));
//! ```
//!
//! Use [`skin_tones()`][Emoji::skin_tones] to iterate over the skin tones of an
//! emoji.
//! ```
//! let raised_hands = emojis::get("🙌🏼").unwrap();
//! let skin_tones: Vec<_> = raised_hands.skin_tones().unwrap().map(|e| e.as_str()).collect();
//! assert_eq!(skin_tones, ["🙌", "🙌🏻", "🙌🏼", "🙌🏽", "🙌🏾", "🙌🏿"]);
//! ```
//!
//! You can use the [`iter()`] function to iterate over all emojis. This only
//! includes the default skin tone versions.
//! ```
//! let faces: Vec<_> = emojis::iter().map(|e| e.as_str()).take(5).collect();
//! assert_eq!(faces, ["😀", "😃", "😄", "😁", "😆"]);
//! ```
//!
//! It is recommended to filter the list by the maximum Unicode version that you
//! wish to support.
//! ```
//! let iter = emojis::iter().filter(|e| {
//!     e.unicode_version() < emojis::UnicodeVersion::new(13, 0)
//! });
//! ```
//!
//! Using the [`Group`] enum you can iterate over all emojis in a group.
//! ```
//! let fruit: Vec<_> = emojis::Group::FoodAndDrink.emojis().map(|e| e.as_str()).take(5).collect();
//! assert_eq!(fruit, ["🍇", "🍈", "🍉", "🍊", "🍋"]);
//! ```
//!
//! [examples/replace.rs]: https://github.com/rossmacarthur/emojis/blob/trunk/examples/replace.rs
//! [gemoji]: https://github.com/github/gemoji

#![no_std]

#[cfg(test)]
extern crate alloc;

mod gen;

use core::cmp;
use core::convert;
use core::fmt;
use core::hash;

pub use crate::gen::Group;

/// Represents an emoji.
///
/// See [Unicode.org](https://unicode.org/emoji/charts/full-emoji-list.html) for
/// more information.
#[derive(Debug)]
pub struct Emoji {
    emoji: &'static str,
    name: &'static str,
    unicode_version: UnicodeVersion,
    group: Group,

    // Stores the id of the emoji with the default skin tone, the number of
    // skin tones and then the skin tone of the current emoji.
    //
    //     (<id>, <n>, <skin_tone>)
    //
    skin_tone: Option<(u16, u8, SkinTone)>,

    aliases: Option<&'static [&'static str]>,
}

/// A Unicode version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnicodeVersion {
    major: u32,
    minor: u32,
}

/// The skin tone of an emoji.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
    LightAndMediumLight,
    LightAndMedium,
    LightAndMediumDark,
    LightAndDark,
    MediumLightAndLight,
    MediumLightAndMedium,
    MediumLightAndMediumDark,
    MediumLightAndDark,
    MediumAndLight,
    MediumAndMediumLight,
    MediumAndMediumDark,
    MediumAndDark,
    MediumDarkAndLight,
    MediumDarkAndMediumLight,
    MediumDarkAndMedium,
    MediumDarkAndDark,
    DarkAndLight,
    DarkAndMediumLight,
    DarkAndMedium,
    DarkAndMediumDark,
}

impl UnicodeVersion {
    /// Construct a new version.
    #[inline]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    #[inline]
    pub const fn major(self) -> u32 {
        self.major
    }

    #[inline]
    pub const fn minor(self) -> u32 {
        self.minor
    }
}

impl Emoji {
    /// Returns this emoji as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let rocket = emojis::get("🚀").unwrap();
    /// assert_eq!(rocket.as_str(), "🚀")
    /// ```
    #[inline]
    pub const fn as_str(&self) -> &str {
        self.emoji
    }

    /// Returns this emoji as slice of UTF-8 encoded bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let rocket = emojis::get("🚀").unwrap();
    /// assert_eq!(rocket.as_bytes(), &[0xf0, 0x9f, 0x9a, 0x80]);
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.emoji.as_bytes()
    }

    /// Returns the CLDR name for this emoji.
    ///
    /// # Examples
    ///
    /// ```
    /// let cool = emojis::get("😎").unwrap();
    /// assert_eq!(cool.name(), "smiling face with sunglasses");
    /// ```
    #[inline]
    pub const fn name(&self) -> &str {
        self.name
    }

    /// Returns the Unicode version this emoji first appeared in.
    ///
    /// # Examples
    ///
    /// ```
    /// use emojis::UnicodeVersion;
    ///
    /// let villain = emojis::get("🦹").unwrap();
    /// assert_eq!(villain.unicode_version(), UnicodeVersion::new(11, 0));
    /// ```
    #[inline]
    pub const fn unicode_version(&self) -> UnicodeVersion {
        self.unicode_version
    }

    /// Returns the group this emoji belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use emojis::Group;
    ///
    /// let flag = emojis::get("🇿🇦").unwrap();
    /// assert_eq!(flag.group(), Group::Flags);
    /// ```
    #[inline]
    pub const fn group(&self) -> Group {
        self.group
    }

    /// Returns the skin tone of this emoji.
    ///
    /// # Examples
    ///
    /// ```
    /// use emojis::SkinTone;
    ///
    /// let peace = emojis::get("✌️").unwrap();
    /// assert_eq!(peace.skin_tone(), Some(SkinTone::Default));
    ///
    /// let peace = emojis::get("✌🏽").unwrap();
    /// assert_eq!(peace.skin_tone(), Some(SkinTone::Medium));
    /// ```
    ///
    /// For emojis where skin tones are not applicable this will be `None`.
    ///
    /// ```
    /// let cool = emojis::get("😎").unwrap();
    /// assert!(cool.skin_tone().is_none());
    /// ```
    #[inline]
    pub fn skin_tone(&self) -> Option<SkinTone> {
        self.skin_tone.map(|(_, _, v)| v)
    }

    /// Returns an iterator over the emoji and all the related skin tone emojis.
    ///
    /// # Examples
    ///
    /// ```
    /// use emojis::Emoji;
    ///
    /// let luck = emojis::get("🤞🏼").unwrap();
    /// let skin_tones: Vec<_> = luck.skin_tones().unwrap().map(Emoji::as_str).collect();
    /// assert_eq!(skin_tones, ["🤞", "🤞🏻", "🤞🏼", "🤞🏽", "🤞🏾", "🤞🏿"]);
    /// ```
    ///
    /// Some emojis have 26 skin tones!
    ///
    /// ```
    /// use emojis::SkinTone;
    ///
    /// let couple = emojis::get("👩🏿‍❤️‍👨🏼").unwrap();
    /// let skin_tones = couple.skin_tones().unwrap().count();
    /// assert_eq!(skin_tones, 26);
    /// ```
    ///
    /// For emojis where skin tones are not applicable this will return `None`.
    ///
    /// ```
    /// let cool = emojis::get("😎").unwrap();
    /// assert!(cool.skin_tones().is_none());
    /// ```
    #[inline]
    pub fn skin_tones(&self) -> Option<impl Iterator<Item = &Self> + Clone> {
        let (i, n, _) = self.skin_tone?;
        Some(crate::gen::EMOJIS[i as usize..].iter().take(n as usize))
    }

    /// Returns a version of this emoji that has the given skin tone.
    ///
    /// # Examples
    ///
    /// ```
    /// use emojis::SkinTone;
    ///
    /// let raised_hands = emojis::get("🙌🏼")
    ///     .unwrap()
    ///     .with_skin_tone(SkinTone::MediumDark)
    ///     .unwrap();
    /// assert_eq!(raised_hands, emojis::get("🙌🏾").unwrap());
    /// ```
    ///
    /// ```
    /// use emojis::SkinTone;
    ///
    /// let couple = emojis::get("👩‍❤️‍👨")
    ///     .unwrap()
    ///     .with_skin_tone(SkinTone::DarkAndMediumLight)
    ///     .unwrap();
    /// assert_eq!(couple, emojis::get("👩🏿‍❤️‍👨🏼").unwrap());
    /// ```
    ///
    /// For emojis where the skin tone is not applicable this will return
    /// `None`.
    ///
    /// ```
    /// use emojis::SkinTone;
    ///
    /// let cool = emojis::get("😎").unwrap();
    /// assert!(cool.with_skin_tone(SkinTone::Medium).is_none());
    /// ```
    #[inline]
    pub fn with_skin_tone(&self, skin_tone: SkinTone) -> Option<&Self> {
        self.skin_tones()?
            .find(|emoji| emoji.skin_tone().unwrap() == skin_tone)
    }

    /// Returns the first GitHub shortcode for this emoji.
    ///
    /// Most emojis only have zero or one shortcode but for a few there are
    /// multiple. Use the [`shortcodes()`][Emoji::shortcodes] method to return
    /// all the shortcodes. See [gemoji] for more information.
    ///
    /// For emojis that have zero shortcodes this will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let thinking = emojis::get("🤔").unwrap();
    /// assert_eq!(thinking.shortcode().unwrap(), "thinking");
    /// ```
    ///
    /// [gemoji]: https://github.com/github/gemoji
    #[inline]
    pub fn shortcode(&self) -> Option<&str> {
        self.aliases.and_then(|aliases| aliases.first().copied())
    }

    /// Returns an iterator over the GitHub shortcodes for this emoji.
    ///
    /// Most emojis only have zero or one shortcode but for a few there are
    /// multiple. Use the [`shortcode()`][Emoji::shortcode] method to return the
    /// first shortcode. See [gemoji] for more information.
    ///
    /// For emojis that have zero shortcodes this will return an empty iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// let laughing = emojis::get("😆").unwrap();
    /// assert_eq!(
    ///     laughing.shortcodes().collect::<Vec<_>>(),
    ///     vec!["laughing", "satisfied"]
    /// );
    /// ```
    ///
    /// [gemoji]: https://github.com/github/gemoji
    #[inline]
    pub fn shortcodes(&self) -> impl Iterator<Item = &str> + Clone {
        self.aliases.into_iter().flatten().copied()
    }
}

impl cmp::PartialEq<Emoji> for Emoji {
    #[inline]
    fn eq(&self, other: &Emoji) -> bool {
        self.emoji == other.emoji
    }
}

impl cmp::PartialEq<str> for Emoji {
    #[inline]
    fn eq(&self, s: &str) -> bool {
        self.as_str() == s
    }
}

impl cmp::PartialEq<&str> for Emoji {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        self.as_str() == *s
    }
}

impl cmp::Eq for Emoji {}

impl hash::Hash for Emoji {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.emoji.hash(state);
    }
}

impl convert::AsRef<str> for Emoji {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl convert::AsRef<[u8]> for Emoji {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Display for Emoji {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for &'static Emoji {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for &'static Emoji {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = &'static Emoji;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a string representing an emoji")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                crate::get(value).ok_or_else(|| E::custom("invalid emoji"))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl Group {
    /// Returns an iterator over all groups.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut iter = emojis::Group::iter();
    /// assert_eq!(iter.next().unwrap(), emojis::Group::SmileysAndEmotion);
    /// assert_eq!(iter.next().unwrap(), emojis::Group::PeopleAndBody);
    /// ```
    #[inline]
    pub fn iter() -> impl Iterator<Item = Group> + Clone {
        [
            Self::SmileysAndEmotion,
            Self::PeopleAndBody,
            Self::AnimalsAndNature,
            Self::FoodAndDrink,
            Self::TravelAndPlaces,
            Self::Activities,
            Self::Objects,
            Self::Symbols,
            Self::Flags,
        ]
        .iter()
        .copied()
    }

    /// Returns an iterator over all emojis in this group.
    ///
    /// # Examples
    ///
    /// ```
    /// let flags: Vec<_> = emojis::Group::Flags.emojis().map(|e| e.as_str()).take(5).collect();
    /// assert_eq!(flags, ["🏁", "🚩", "🎌", "🏴", "🏳️"]);
    /// ```
    #[inline]
    pub fn emojis(&self) -> impl Iterator<Item = &'static Emoji> {
        let group = *self;
        iter()
            .skip_while(move |emoji| emoji.group != group)
            .take_while(move |emoji| emoji.group == group)
    }
}

/// Returns an iterator over all emojis.
///
/// - Ordered by Unicode CLDR data.
/// - Excludes non-default skin tones.
///
/// # Examples
///
/// ```
/// let faces: Vec<_> = emojis::iter().map(|e| e.as_str()).take(5).collect();
/// assert_eq!(faces, ["😀", "😃", "😄", "😁", "😆"]);
/// ```
#[inline]
pub fn iter() -> impl Iterator<Item = &'static Emoji> + Clone {
    crate::gen::EMOJIS
        .iter()
        .filter(|emoji| matches!(emoji.skin_tone(), Some(SkinTone::Default) | None))
}

/// Lookup an emoji by Unicode value.
///
/// This take *Ο(1)* time.
///
/// # Note
///
/// If passed a minimally qualified or unqualified emoji this will return the
/// emoji struct containing the fully qualified version.
///
/// # Examples
///
/// In the ordinary case.
///
/// ```
/// let emoji = "🚀";
/// let rocket = emojis::get(emoji).unwrap();
/// assert!(rocket.as_str() == emoji);
/// assert_eq!(rocket.shortcode().unwrap(), "rocket");
/// ```
///
/// For a minimally qualified or unqualified emoji.
///
/// ```
/// let unqualified = "\u{1f43f}";
/// let fully_qualified = "\u{1f43f}\u{fe0f}";
/// let chipmunk = emojis::get(unqualified).unwrap();
/// assert_eq!(chipmunk.as_str(), fully_qualified);
/// assert_eq!(chipmunk.shortcode().unwrap(), "chipmunk");
/// ```
#[inline]
pub fn get(s: &str) -> Option<&'static Emoji> {
    crate::gen::unicode::MAP
        .get(s)
        .map(|&i| &crate::gen::EMOJIS[i])
}

/// Lookup an emoji by GitHub shortcode.
///
/// This take *Ο(1)* time.
///
/// # Examples
///
/// ```
/// let rocket = emojis::get_by_shortcode("rocket").unwrap();
/// assert_eq!(rocket, "🚀");
/// ```
#[inline]
pub fn get_by_shortcode(s: &str) -> Option<&'static Emoji> {
    crate::gen::shortcode::MAP
        .get(s)
        .map(|&i| &crate::gen::EMOJIS[i])
}
