#![cfg_attr(rustfmt, rustfmt_skip)]

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
use zbus::zvariant;

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) mod message;

#[cfg(all(feature = "images_no_default_features", any(feature = "dbus", feature = "zbus"), unix, not(target_os = "macos")))]
use crate::image::Image;

#[cfg(all(feature = "images_no_default_features", feature = "zbus", unix, not(target_os = "macos")))]
use crate::image::image_spec_str;
use crate::Urgency;

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))] use crate::notification::Notification;
#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))] use std::collections::HashMap;

mod constants;

#[cfg(all(unix, not(target_os = "macos")))]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) enum CustomHintType {
    Int,
    String,
}

/// Hints allow you to pass extra information to the notification server.
///
/// Many of these are standardized by:
///
/// * <https://specifications.freedesktop.org/notification-spec/latest/hints.html>
///
/// Which hints are actually implemented depends strongly on the notification server you talk to.
/// Usually [`get_capabilities()`](`crate::get_capabilities`) gives some clues, but the standard usually mentions much more
/// than is actually available.
///
/// Pass these to [`Notification::hint`].
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Hint {
    /// If true, server may interpret action identifiers as named icons and display those.
    ActionIcons(bool),

    /// Check out:
    ///
    /// * <https://specifications.freedesktop.org/notification-spec/latest/hints.html>
    Category(String),

    /// Name of the `DesktopEntry` representing the calling application.
    /// In case of `"firefox.desktop"` use `"firefox"`. May be used to retrieve the correct icon.
    DesktopEntry(String),

    /// Image as raw data
    #[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
    ImageData(Image),

    /// Display the image at this path.
    ImagePath(String),

    /// This does not work on all servers, however timeout=0 will do the job
    Resident(bool),

    /// Play the sound at this path.
    SoundFile(String),

    /// A themeable named sound from the freedesktop.org [sound naming specification](http://0pointer.de/public/sound-naming-spec.html) to play when the notification pops up.
    /// Similar to icon-name, but for sounds. An example would be `"message-new-instant"`.
    SoundName(String),

    /// Suppress the notification sound.
    SuppressSound(bool),

    /// When set, the server will treat the notification as transient and bypass the server's persistence capability, if it exists.
    Transient(bool),

    /// Lets the notification point to a certain 'x' position on the screen.
    /// Requires `Y`.
    X(i32),

    /// Lets the notification point to a certain 'y' position on the screen.
    /// Requires `X`.
    Y(i32),

    /// Pass a [`Urgency`](crate::Urgency), either `Low`, `Normal`, or `Critical`.
    Urgency(Urgency),

    /// If you want to pass something entirely different.
    Custom(String, String),

    /// A custom numerical (integer) hint
    CustomInt(String, i32),

    /// Only used by this `NotificationServer` implementation.
    Invalid // TODO find a better solution to this
}

impl Hint {
    /// Get the `bool` representation of this hint.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            | Hint::ActionIcons(inner)
            | Hint::Resident(inner)
            | Hint::SuppressSound(inner)
            | Hint::Transient(inner) => Some(inner),
            _ => None
        }
    }

    /// Get the `i32` representation of this hint.
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Hint::X(inner) | Hint::Y(inner) => Some(inner),
            _ => None
        }
    }

    /// Get the `&str` representation of this hint.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Hint::DesktopEntry(ref inner) |
            Hint::ImagePath(ref inner)    |
            Hint::SoundFile(ref inner)    |
            Hint::SoundName(ref inner)    => Some(inner),
            _ => None
        }
    }

    /// Convenience function for converting a name and value into a hint.
    pub fn from_key_val(name: &str, value: &str) -> Result<Hint, String> {
        match (name,value){
            (constants::ACTION_ICONS,val)    => val.parse::<bool>().map(Hint::ActionIcons).map_err(|e|e.to_string()),
            (constants::CATEGORY, val)       => Ok(Hint::Category(val.to_owned())),
            (constants::DESKTOP_ENTRY, val)  => Ok(Hint::DesktopEntry(val.to_owned())),
            (constants::IMAGE_PATH, val)     => Ok(Hint::ImagePath(val.to_owned())),
            (constants::RESIDENT, val)       => val.parse::<bool>().map(Hint::Resident).map_err(|e|e.to_string()),
            (constants::SOUND_FILE, val)     => Ok(Hint::SoundFile(val.to_owned())),
            (constants::SOUND_NAME, val)     => Ok(Hint::SoundName(val.to_owned())),
            (constants::SUPPRESS_SOUND, val) => val.parse::<bool>().map(Hint::SuppressSound).map_err(|e|e.to_string()),
            (constants::TRANSIENT, val)      => val.parse::<bool>().map(Hint::Transient).map_err(|e|e.to_string()),
            (constants::X, val)              => val.parse::<i32>().map(Hint::X).map_err(|e|e.to_string()),
            (constants::Y, val)              => val.parse::<i32>().map(Hint::Y).map_err(|e|e.to_string()),
            _                                => Err(String::from("unknown name"))
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl Hint {}

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
#[test]
fn test_hints_to_map() {

    // custom value should only be there once if the names are identical

    let n1 = Notification::new()
        .hint(Hint::Custom("foo".into(), "bar1".into()))
        .hint(Hint::Custom("foo".into(), "bar2".into()))
        .hint(Hint::Custom("f00".into(), "bar3".into()))
        .finalize();

     assert_eq!(hints_to_map(&n1), maplit::hashmap!{
         "foo" => zvariant::Value::Str("bar2".into()),
         "f00" => zvariant::Value::Str("bar3".into())
     });
}

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
pub(crate) fn hints_to_map(notification: &Notification) -> HashMap::<&str, zvariant::Value<'_>> {
    notification
        .get_hints()
        .map(Into::into)
        .collect()
}

#[cfg(all(feature = "zbus", unix, not(target_os = "macos")))]
impl<'a> From<&'a Hint> for (&'a str, zvariant::Value<'a>) {
    fn from(val: &'a Hint) -> Self {
        use self::constants::*;
        match val {
            Hint::ActionIcons(value)       => (ACTION_ICONS   , zvariant::Value::Bool(*value)), // bool
            Hint::Category(value)          => (CATEGORY       , zvariant::Value::Str(value.as_str().into())),
            Hint::DesktopEntry(value)      => (DESKTOP_ENTRY  , zvariant::Value::Str(value.as_str().into())),

            #[cfg(all(feature = "zbus", feature = "images_no_default_features", unix, not(target_os = "macos")))]
            //Hint::ImageData(image)         => (image_spec(*crate::SPEC_VERSION).as_str(), ImagePayload::from(*image).into()),
            Hint::ImageData(image)         => (
                image_spec_str(*crate::SPEC_VERSION),
                zvariant::Value::Structure(
                    image.to_tuple().into()
                )
            ),


            Hint::ImagePath(value)         => (IMAGE_PATH     , zvariant::Value::Str(value.as_str().into())),
            Hint::Resident(value)          => (RESIDENT       , zvariant::Value::Bool(*value)), // bool
            Hint::SoundFile(value)         => (SOUND_FILE     , zvariant::Value::Str(value.as_str().into())),
            Hint::SoundName(value)         => (SOUND_NAME     , zvariant::Value::Str(value.as_str().into())),
            Hint::SuppressSound(value)     => (SUPPRESS_SOUND , zvariant::Value::Bool(*value)),
            Hint::Transient(value)         => (TRANSIENT      , zvariant::Value::Bool(*value)),
            Hint::X(value)                 => (X              , zvariant::Value::I32(*value)),
            Hint::Y(value)                 => (Y              , zvariant::Value::I32(*value)),
            Hint::Urgency(value)           => (URGENCY        , zvariant::Value::U8(*value as u8)),
            Hint::Custom(key, val)         => (key.as_str()   , zvariant::Value::Str(val.as_str().into())),
            Hint::CustomInt(key, val)      => (key.as_str()   , zvariant::Value::I32(*val)),
            Hint::Invalid                  => (INVALID        , zvariant::Value::Str(INVALID.into()))
        }
    }
}


#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl<'a, A: dbus::arg::RefArg> From<(&'a String, &'a A)> for Hint {
    fn from(pair: (&String, &A)) -> Self {

        let (key, variant) = pair;
        match (key.as_ref(), variant.as_u64(), variant.as_i64(), variant.as_str().map(String::from)) {

            (constants::ACTION_ICONS,   Some(1),  _,       _          ) => Hint::ActionIcons(true),
            (constants::ACTION_ICONS,   _,        _,       _          ) => Hint::ActionIcons(false),
            (constants::URGENCY,        level,    _,       _          ) => Hint::Urgency(level.into()),
            (constants::CATEGORY,       _,        _,       Some(name) ) => Hint::Category(name),

            (constants::DESKTOP_ENTRY,  _,        _,       Some(entry)) => Hint::DesktopEntry(entry),
            (constants::IMAGE_PATH,     _,        _,       Some(path) ) => Hint::ImagePath(path),
            (constants::RESIDENT,       Some(1),  _,       _          ) => Hint::Resident(true),
            (constants::RESIDENT,       _,        _,       _          ) => Hint::Resident(false),

            (constants::SOUND_FILE,     _,        _,       Some(path) ) => Hint::SoundFile(path),
            (constants::SOUND_NAME,     _,        _,       Some(name) ) => Hint::SoundName(name),
            (constants::SUPPRESS_SOUND, Some(1),  _,       _          ) => Hint::SuppressSound(true),
            (constants::SUPPRESS_SOUND, _,        _,       _          ) => Hint::SuppressSound(false),
            (constants::TRANSIENT,      Some(1),  _,       _          ) => Hint::Transient(true),
            (constants::TRANSIENT,      _,        _,       _          ) => Hint::Transient(false),
            (constants::X,              _,        Some(x), _          ) => Hint::X(x as i32),
            (constants::Y,              _,        Some(y), _          ) => Hint::Y(y as i32),

            other => {
                eprintln!("Invalid Hint {:#?} ", other);
                Hint::Invalid
            }
        }
    }
}
