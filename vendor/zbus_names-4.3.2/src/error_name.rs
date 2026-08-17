use crate::{Error, Result, utils::define_name_type_impls};
use serde::Serialize;
use zvariant::{OwnedValue, Str, Type, Value};

/// String that identifies an [error name][en] on the bus.
///
/// Error names have same constraints as interface names.
///
/// # Examples
///
/// ```
/// use zbus_names::ErrorName;
///
/// // Valid error names.
/// let name = ErrorName::try_from("org.gnome.Error_for_you").unwrap();
/// assert_eq!(name, "org.gnome.Error_for_you");
/// let name = ErrorName::try_from("a.very.loooooooooooooooooo_ooooooo_0000o0ng.ErrorName").unwrap();
/// assert_eq!(name, "a.very.loooooooooooooooooo_ooooooo_0000o0ng.ErrorName");
///
/// // Invalid error names
/// ErrorName::try_from("").unwrap_err();
/// ErrorName::try_from(":start.with.a.colon").unwrap_err();
/// ErrorName::try_from("double..dots").unwrap_err();
/// ErrorName::try_from(".").unwrap_err();
/// ErrorName::try_from(".start.with.dot").unwrap_err();
/// ErrorName::try_from("no-dots").unwrap_err();
/// ErrorName::try_from("1st.element.starts.with.digit").unwrap_err();
/// ErrorName::try_from("the.2nd.element.starts.with.digit").unwrap_err();
/// ErrorName::try_from("contains.dashes-in.the.name").unwrap_err();
/// ```
///
/// [en]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue,
)]
pub struct ErrorName<'name>(Str<'name>);

/// Owned sibling of [`ErrorName`].
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue)]
pub struct OwnedErrorName(#[serde(borrow)] ErrorName<'static>);

define_name_type_impls! {
    name: ErrorName,
    owned: OwnedErrorName,
    validate: validate,
}

fn validate(name: &str) -> Result<()> {
    // Error names follow the same rules as interface names.
    crate::interface_name::validate_bytes(name.as_bytes()).map_err(|_| {
        Error::InvalidName(
            "Invalid error name. See \
            https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-error",
        )
    })
}
