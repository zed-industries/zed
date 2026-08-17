use crate::{Signature, Type};
use serde::de::{Deserialize, DeserializeSeed};
use std::marker::PhantomData;

/// Types with dynamic signatures.
///
/// Prefer implementing [Type] if possible, but if the actual signature of your type cannot be
/// determined until runtime, you can implement this type to support serialization.  You should
/// also implement [DynamicDeserialize] for deserialization.
pub trait DynamicType {
    /// The type signature for `self`.
    ///
    /// See [`Type::SIGNATURE`] for details.
    fn signature(&self) -> Signature;
}

/// Types that deserialize based on dynamic signatures.
///
/// Prefer implementing [Type] and [Deserialize] if possible, but if the actual signature of your
/// type cannot be determined until runtime, you should implement this type to support
/// deserialization given a signature.
pub trait DynamicDeserialize<'de>: DynamicType {
    /// A [DeserializeSeed] implementation for this type.
    type Deserializer: DeserializeSeed<'de, Value = Self> + DynamicType;

    /// Get a deserializer compatible with this parsed signature.
    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer>;
}

impl<T> DynamicType for T
where
    T: Type + ?Sized,
{
    fn signature(&self) -> Signature {
        <T as Type>::SIGNATURE.clone()
    }
}

impl<'de, T> DynamicDeserialize<'de> for T
where
    T: Type + Deserialize<'de>,
{
    type Deserializer = PhantomData<T>;

    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer> {
        let expected = <T as Type>::SIGNATURE;

        if expected != signature {
            match expected {
                Signature::Structure(fields)
                    if fields.len() == 1 && fields.iter().next().unwrap() == signature =>
                {
                    // This is likely a D-Bus message body containing a single type being
                    // deserialized as a single-field struct. No need to be super strict here.
                }
                _ => {
                    return Err(zvariant::Error::SignatureMismatch(
                        signature.clone(),
                        format!("`{expected}`"),
                    ));
                }
            }
        }

        Ok(PhantomData)
    }
}
