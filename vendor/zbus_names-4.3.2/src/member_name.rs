use crate::{Error, Result, utils::define_name_type_impls};
use serde::Serialize;
use zvariant::{OwnedValue, Str, Type, Value};

/// String that identifies an [member (method or signal) name][in] on the bus.
///
/// # Examples
///
/// ```
/// use zbus_names::MemberName;
///
/// // Valid member names.
/// let name = MemberName::try_from("Member_for_you").unwrap();
/// assert_eq!(name, "Member_for_you");
/// let name = MemberName::try_from("CamelCase101").unwrap();
/// assert_eq!(name, "CamelCase101");
/// let name = MemberName::try_from("a_very_loooooooooooooooooo_ooooooo_0000o0ngName").unwrap();
/// assert_eq!(name, "a_very_loooooooooooooooooo_ooooooo_0000o0ngName");
///
/// // Invalid member names
/// MemberName::try_from("").unwrap_err();
/// MemberName::try_from(".").unwrap_err();
/// MemberName::try_from("1startWith_a_Digit").unwrap_err();
/// MemberName::try_from("contains.dots_in_the_name").unwrap_err();
/// MemberName::try_from("contains-dashes-in_the_name").unwrap_err();
/// ```
///
/// [in]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member
#[derive(
    Clone, Debug, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue,
)]
pub struct MemberName<'name>(Str<'name>);

/// Owned sibling of [`MemberName`].
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Type, Value, PartialOrd, Ord, OwnedValue)]
pub struct OwnedMemberName(#[serde(borrow)] MemberName<'static>);

define_name_type_impls! {
    name: MemberName,
    owned: OwnedMemberName,
    validate: validate,
}

fn validate(name: &str) -> Result<()> {
    validate_bytes(name.as_bytes()).map_err(|_| {
        Error::InvalidName(
            "Invalid member name. See \
            https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-member",
        )
    })
}

pub(crate) fn validate_bytes(bytes: &[u8]) -> std::result::Result<(), ()> {
    use winnow::{
        Parser,
        stream::AsChar,
        token::{one_of, take_while},
    };
    // Rules
    //
    // * Only ASCII alphanumeric or `_`.
    // * Must not begin with a digit.
    // * Must contain at least 1 character.
    // * <= 255 characters.
    let first_element_char = one_of((AsChar::is_alpha, b'_'));
    let subsequent_element_chars = take_while::<_, _, ()>(0.., (AsChar::is_alphanum, b'_'));
    let mut member_name = (first_element_char, subsequent_element_chars);

    member_name.parse(bytes).map_err(|_| ()).and_then(|_| {
        // Least likely scenario so we check this last.
        if bytes.len() > 255 {
            return Err(());
        }

        Ok(())
    })
}
