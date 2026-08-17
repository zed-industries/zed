//! `Hints` allow you to pass extra information to the server.
//!
//! Many of these are standardized by either:
//!
//! [galago-project spec](http://www.galago-project.org/specs/notification/0.9/x344.html) or
//! [gnome notification-spec](https://developer.gnome.org/notification-spec/#hints)
//!
//! Which of these are actually implemented depends strongly on the Notification server you talk to.
//! Usually the `get_capabilities()` gives some clues, but the standards usually mention much more
//! than is actually available.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, unused_imports)]


use super::{Hint, constants::*};
use crate ::Urgency;

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
use crate::image::*;

use std::collections::{HashMap, HashSet};
#[cfg(feature = "dbus")]
use dbus::arg::{messageitem::MessageItem, RefArg};

/// All currently implemented `Hints` that can be sent.
///
/// as found on <https://developer.gnome.org/notification-spec/>
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct HintMessage(Hint);

#[cfg(feature = "dbus")]
impl HintMessage {
    pub fn wrap_hint(hint: Hint) -> (MessageItem, MessageItem) {
        Self::from(hint).into()
    }
}

impl From<Hint> for HintMessage {
    fn from(hint: Hint) -> Self {
        HintMessage(hint)
    }
}

impl std::ops::Deref for HintMessage {
    type Target = Hint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "dbus")]
impl<'a, A: RefArg> From<(&'a String, &'a A)> for HintMessage {
    fn from(pair: (&String, &A)) -> Self {

        let (key, variant) = pair;
        match (key.as_ref(), variant.as_u64(), variant.as_i64(), variant.as_str().map(String::from)) {

            (ACTION_ICONS,   Some(1),  _,       _          ) => Hint::ActionIcons(true),
            (ACTION_ICONS,   _,        _,       _          ) => Hint::ActionIcons(false),
            (URGENCY,        level,    _,       _          ) => Hint::Urgency(level.into()),
            (CATEGORY,       _,        _,       Some(name) ) => Hint::Category(name),

            (DESKTOP_ENTRY,  _,        _,       Some(entry)) => Hint::DesktopEntry(entry),
            (IMAGE_PATH,     _,        _,       Some(path) ) => Hint::ImagePath(path),
            (RESIDENT,       Some(1),  _,       _          ) => Hint::Resident(true),
            (RESIDENT,       _,        _,       _          ) => Hint::Resident(false),

            (SOUND_FILE,     _,        _,       Some(path) ) => Hint::SoundFile(path),
            (SOUND_NAME,     _,        _,       Some(name) ) => Hint::SoundName(name),
            (SUPPRESS_SOUND, Some(1),  _,       _          ) => Hint::SuppressSound(true),
            (SUPPRESS_SOUND, _,        _,       _          ) => Hint::SuppressSound(false),
            (TRANSIENT,      Some(1),  _,       _          ) => Hint::Transient(true),
            (TRANSIENT,      _,        _,       _          ) => Hint::Transient(false),
            (X,              _,        Some(x), _          ) => Hint::X(x as i32),
            (Y,              _,        Some(y), _          ) => Hint::Y(y as i32),

            other => {
                eprintln!("Invalid Hint{:#?} ", other);
                Hint::Invalid
            }
        }.into()
    }
}

#[cfg(feature = "dbus")]
impl From<HintMessage> for (MessageItem, MessageItem) {
    fn from(hint: HintMessage) -> Self {

        let (key, value): (String, MessageItem) = match hint.0 {
            Hint::ActionIcons(value)       => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            Hint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            Hint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
            #[cfg(all(feature = "images_no_default_features", unix, not(target_os ="macos")))]
            Hint::ImageData(image)         => (image_spec(*crate::SPEC_VERSION), ImageMessage::from(image).into()),
            Hint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), MessageItem::Str(value.clone())),
            Hint::Resident(value)          => (RESIDENT       .to_owned(), MessageItem::Bool(value)), // bool
            Hint::SoundFile(ref value)     => (SOUND_FILE     .to_owned(), MessageItem::Str(value.clone())),
            Hint::SoundName(ref value)     => (SOUND_NAME     .to_owned(), MessageItem::Str(value.clone())),
            Hint::SuppressSound(value)     => (SUPPRESS_SOUND .to_owned(), MessageItem::Bool(value)),
            Hint::Transient(value)         => (TRANSIENT      .to_owned(), MessageItem::Bool(value)),
            Hint::X(value)                 => (X              .to_owned(), MessageItem::Int32(value)),
            Hint::Y(value)                 => (Y              .to_owned(), MessageItem::Int32(value)),
            Hint::Urgency(value)           => (URGENCY        .to_owned(), MessageItem::Byte(value as u8)),
            Hint::Custom(ref key, ref val) => (key            .to_owned(), MessageItem::Str(val.to_owned ())),
            Hint::CustomInt(ref key, val)  => (key            .to_owned(), MessageItem::Int32(val)),
            Hint::Invalid                  => ("invalid"      .to_owned(), MessageItem::Str("Invalid".to_owned()))
        };

        (MessageItem::Str(key), MessageItem::Variant(Box::new(value)))
    }
}


// TODO: deprecated, Prefer the DBus Arg and RefArg APIs
#[cfg(feature = "dbus")]
impl From<(&MessageItem, &MessageItem)> for HintMessage {
    fn from ((key, mut value): (&MessageItem, &MessageItem)) -> Self {
        use Hint as Hint;

        // If this is a variant, consider the thing inside it
        // If it's a nested variant, keep drilling down until we get a real value
        while let MessageItem::Variant(inner) = value {
            value = inner;
        }

        let is_stringy = value.inner::<&str>().is_ok();

        match key.inner::<&str>() {
            Ok(CATEGORY)        => value.inner::<&str>().map(String::from).map(Hint::Category),
            Ok(ACTION_ICONS)    => value.inner().map(Hint::ActionIcons),
            Ok(DESKTOP_ENTRY)   => value.inner::<&str>().map(String::from).map(Hint::DesktopEntry),
            Ok(IMAGE_PATH)      => value.inner::<&str>().map(String::from).map(Hint::ImagePath),
            Ok(RESIDENT)        => value.inner().map(Hint::Resident),
            Ok(SOUND_FILE)      => value.inner::<&str>().map(String::from).map(Hint::SoundFile),
            Ok(SOUND_NAME)      => value.inner::<&str>().map(String::from).map(Hint::SoundName),
            Ok(SUPPRESS_SOUND)  => value.inner().map(Hint::SuppressSound),
            Ok(TRANSIENT)       => value.inner().map(Hint::Transient),
            Ok(X)               => value.inner().map(Hint::X),
            Ok(Y)               => value.inner().map(Hint::Y),
            Ok(URGENCY)         => value.inner().map(|i| match i {
                0  => Urgency::Low,
                2  => Urgency::Critical,
                _  => Urgency::Normal
            }).map(Hint::Urgency),
            Ok(k) if is_stringy => value.inner::<&str>().map(|v| Hint::Custom(k.to_string(), v.to_string())),
            Ok(k)               => value.inner().map(|v| Hint::CustomInt(k.to_string(), v)),
            _ => Err(()),
        }.unwrap_or(Hint::Invalid)
        .into()
    }
}


#[allow(missing_docs)]
#[cfg(feature = "dbus")]
pub(crate) fn hints_from_variants<A: RefArg>(hints: &HashMap<String, A>) -> HashSet<HintMessage> {
    hints.iter().map(Into::into).collect()
}
