use crate::{Error, Result, utils::define_name_type_impls};
use serde::Serialize;
use zvariant::{OwnedValue, Str, Type, Value};

/// String that identifies a [unique bus name][ubn].
///
/// # Examples
///
/// ```
/// use zbus_names::UniqueName;
///
/// // Valid unique names.
/// let name = UniqueName::try_from(":org.gnome.Service-for_you").unwrap();
/// assert_eq!(name, ":org.gnome.Service-for_you");
/// let name = UniqueName::try_from(":a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name").unwrap();
/// assert_eq!(name, ":a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name");
///
/// // Invalid unique names
/// UniqueName::try_from("").unwrap_err();
/// UniqueName::try_from("dont.start.with.a.colon").unwrap_err();
/// UniqueName::try_from(":double..dots").unwrap_err();
/// UniqueName::try_from(".").unwrap_err();
/// UniqueName::try_from(".start.with.dot").unwrap_err();
/// UniqueName::try_from(":no-dots").unwrap_err();
/// ```
///
/// [ubn]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue,
)]
pub struct UniqueName<'name>(pub(crate) Str<'name>);

/// Owned sibling of [`UniqueName`].
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue)]
pub struct OwnedUniqueName(#[serde(borrow)] UniqueName<'static>);

define_name_type_impls! {
    name: UniqueName,
    owned: OwnedUniqueName,
    validate: validate,
}

fn validate(name: &str) -> Result<()> {
    validate_bytes(name.as_bytes()).map_err(|_| {
        Error::InvalidName(
            "Invalid unique name. \
            See https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus"
        )
    })
}

pub(crate) fn validate_bytes(bytes: &[u8]) -> std::result::Result<(), ()> {
    use winnow::{
        Parser,
        combinator::{alt, separated},
        stream::AsChar,
        token::take_while,
    };
    // Rules
    //
    // * Only ASCII alphanumeric, `_` or '-'
    // * Must begin with a `:`.
    // * Must contain at least one `.`.
    // * Each element must be 1 character (so name must be minimum 4 characters long).
    // * <= 255 characters.
    let element = take_while::<_, _, ()>(1.., (AsChar::is_alphanum, b'_', b'-'));
    let peer_name = (b':', (separated(2.., element, b'.'))).map(|_: (_, ())| ());
    let bus_name = b"org.freedesktop.DBus".map(|_| ());
    let mut unique_name = alt((bus_name, peer_name));

    unique_name.parse(bytes).map_err(|_| ()).and_then(|_: ()| {
        // Least likely scenario so we check this last.
        if bytes.len() > 255 {
            return Err(());
        }

        Ok(())
    })
}
