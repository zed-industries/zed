use serde::ser::{SerializeStruct, Serializer};

use crate::Type;

/// A wrapper to serialize `T: Type + serde::Serialize` as a value.
///
/// When the type of a value is well-known, you may avoid the cost and complexity of wrapping to a
/// generic [`enum@crate::Value`] and instead use this wrapper.
///
/// ```
/// # use zvariant::{to_bytes, serialized::Context, as_value::Serialize, LE};
/// #
/// # let ctxt = Context::new_dbus(LE, 0);
/// let _ = to_bytes(ctxt, &Serialize(&[0, 1, 2])).unwrap();
/// ```
pub struct Serialize<'a, T: Type + serde::Serialize>(pub &'a T);

impl<T: Type + serde::Serialize> serde::Serialize for Serialize<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializer implementation needs to ensure padding isn't added for Value.
        let mut structure = serializer.serialize_struct("Variant", 2)?;

        structure.serialize_field("signature", T::SIGNATURE)?;
        structure.serialize_field("value", self.0)?;

        structure.end()
    }
}

impl<T: Type + serde::Serialize> Type for Serialize<'_, T> {
    const SIGNATURE: &'static crate::Signature = &crate::Signature::Variant;
}

/// Serialize a value as a [`enum@zvariant::Value`].
pub fn serialize<T, S>(value: &T, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Type + serde::Serialize,
{
    use serde::Serialize as _;

    Serialize(value).serialize(ser)
}

/// Serialize an optional value as a [`enum@zvariant::Value`].
pub fn serialize_optional<T, S>(value: &Option<T>, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Type + serde::Serialize,
{
    super::serialize(value.as_ref().unwrap(), ser)
}
