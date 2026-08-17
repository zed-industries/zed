use crate::{Error, Result, utils::define_name_type_impls};
use serde::Serialize;
use zvariant::{OwnedValue, Str, Type, Value};

/// String that identifies an [interface name][in] on the bus.
///
/// # Examples
///
/// ```
/// use zbus_names::InterfaceName;
///
/// // Valid interface names.
/// let name = InterfaceName::try_from("org.gnome.Interface_for_you").unwrap();
/// assert_eq!(name, "org.gnome.Interface_for_you");
/// let name = InterfaceName::try_from("a.very.loooooooooooooooooo_ooooooo_0000o0ng.Name").unwrap();
/// assert_eq!(name, "a.very.loooooooooooooooooo_ooooooo_0000o0ng.Name");
///
/// // Invalid interface names
/// InterfaceName::try_from("").unwrap_err();
/// InterfaceName::try_from(":start.with.a.colon").unwrap_err();
/// InterfaceName::try_from("double..dots").unwrap_err();
/// InterfaceName::try_from(".").unwrap_err();
/// InterfaceName::try_from(".start.with.dot").unwrap_err();
/// InterfaceName::try_from("no-dots").unwrap_err();
/// InterfaceName::try_from("1st.element.starts.with.digit").unwrap_err();
/// InterfaceName::try_from("the.2nd.element.starts.with.digit").unwrap_err();
/// InterfaceName::try_from("contains.dashes-in.the.name").unwrap_err();
/// ```
///
/// [in]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue,
)]
pub struct InterfaceName<'name>(Str<'name>);

/// Owned sibling of [`InterfaceName`].
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue)]
pub struct OwnedInterfaceName(#[serde(borrow)] InterfaceName<'static>);

define_name_type_impls! {
    name: InterfaceName,
    owned: OwnedInterfaceName,
    validate: validate,
}

fn validate(name: &str) -> Result<()> {
    validate_bytes(name.as_bytes()).map_err(|_| {
        Error::InvalidName(
            "Invalid interface name. See \
            https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface"
        )
    })
}

pub(crate) fn validate_bytes(bytes: &[u8]) -> std::result::Result<(), ()> {
    use winnow::{
        Parser,
        combinator::separated,
        stream::AsChar,
        token::{one_of, take_while},
    };
    // Rules
    //
    // * Only ASCII alphanumeric and `_`
    // * Must not begin with a `.`.
    // * Must contain at least one `.`.
    // * Each element must:
    //  * not begin with a digit.
    //  * be 1 character (so name must be minimum 3 characters long).
    // * <= 255 characters.
    //
    // Note: A `-` not allowed, which is why we can't use the same parser as for `WellKnownName`.
    let first_element_char = one_of((AsChar::is_alpha, b'_'));
    let subsequent_element_chars = take_while::<_, _, ()>(0.., (AsChar::is_alphanum, b'_'));
    let element = (first_element_char, subsequent_element_chars);
    let mut interface_name = separated(2.., element, b'.');

    interface_name
        .parse(bytes)
        .map_err(|_| ())
        .and_then(|_: ()| {
            // Least likely scenario so we check this last.
            if bytes.len() > 255 {
                return Err(());
            }

            Ok(())
        })
}
