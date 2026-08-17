#![allow(unknown_lints)]
use serde::{
    de::{DeserializeSeed, Deserializer, Error, SeqAccess, Visitor},
    ser::{Serialize, SerializeTupleStruct, Serializer},
};
use std::fmt::{Display, Write};

use crate::{
    DynamicDeserialize, DynamicType, OwnedValue, Signature, Value, value::SignatureSeed,
    value_display_fmt,
};

/// Use this to efficiently build a [`Structure`].
///
/// [`Structure`]: struct.Structure.html
#[derive(Debug, Default, PartialEq)]
pub struct StructureBuilder<'a>(Vec<Value<'a>>);

impl<'a> StructureBuilder<'a> {
    /// Create a new `StructureBuilder`.
    ///
    /// Same as `StructureBuilder::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `field` to `self`.
    ///
    /// This method returns `Self` so that you can use the builder pattern to create a complex
    /// structure.
    #[must_use]
    pub fn add_field<T>(self, field: T) -> Self
    where
        T: DynamicType + Into<Value<'a>>,
    {
        self.append_field(Value::new(field))
    }

    /// Append `field` to `self`.
    ///
    /// Identical to `add_field`, except the field must be in the form of a `Value`.
    #[must_use]
    pub fn append_field<'e: 'a>(mut self, field: Value<'e>) -> Self {
        self.0.push(field);

        self
    }

    /// Append `field` to `self`.
    ///
    /// Identical to `add_field`, except it makes changes in-place.
    pub fn push_field<T>(&mut self, field: T)
    where
        T: DynamicType + Into<Value<'a>>,
    {
        self.push_value(Value::new(field))
    }

    /// Append `field` to `self`.
    ///
    /// Identical to `append_field`, except it makes changes in-place.
    pub fn push_value<'e: 'a>(&mut self, field: Value<'e>) {
        self.0.push(field)
    }

    /// Build the `Structure`.
    ///
    /// [`Structure`]: struct.Structure.html
    pub fn build(self) -> crate::Result<Structure<'a>> {
        if self.0.is_empty() {
            return Err(crate::Error::EmptyStructure);
        }

        let fields_signatures: Box<[Signature]> =
            self.0.iter().map(Value::value_signature).cloned().collect();
        let signature = Signature::structure(fields_signatures);

        Ok(Structure {
            fields: self.0,
            signature,
        })
    }

    /// Same as `build` except Signature is provided.
    pub(crate) fn build_with_signature<'s: 'a>(self, signature: &Signature) -> Structure<'a> {
        Structure {
            fields: self.0,
            signature: signature.clone(),
        }
    }
}

/// Use this to deserialize a [`Structure`].
///
/// The lifetime `'a` is now redundant and kept only for backward compatibility. All instances now
/// has a `'static` lifetime. This will be removed in the next major release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureSeed<'a> {
    signature: Signature,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl StructureSeed<'static> {
    /// Create a new `StructureSeed`
    ///
    /// The given signature must be a valid structure signature.
    #[must_use]
    pub fn new_unchecked(signature: &Signature) -> Self {
        StructureSeed {
            signature: signature.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl TryFrom<Signature> for StructureSeed<'static> {
    type Error = zvariant::Error;

    fn try_from(signature: Signature) -> Result<Self, zvariant::Error> {
        if !matches!(signature, Signature::Structure(_)) {
            return Err(zvariant::Error::IncorrectType);
        }

        Ok(StructureSeed {
            signature,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'de> DeserializeSeed<'de> for StructureSeed<'_> {
    type Value = Structure<'de>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(StructureVisitor {
            signature: self.signature,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructureVisitor {
    signature: Signature,
}

impl<'de> Visitor<'de> for StructureVisitor {
    type Value = Structure<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a Structure value")
    }

    fn visit_seq<V>(self, visitor: V) -> Result<Structure<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        SignatureSeed {
            signature: &self.signature,
        }
        .visit_struct(visitor)
    }
}

/// A helper type to wrap structs in [`Value`].
///
/// API is provided to convert from, and to tuples.
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Structure<'a> {
    fields: Vec<Value<'a>>,
    signature: Signature,
}

impl<'a> Structure<'a> {
    /// Get a reference to all the fields of `self`.
    pub fn fields(&self) -> &[Value<'a>] {
        &self.fields
    }

    /// Converts `self` to a `Vec` containing all its fields.
    pub fn into_fields(self) -> Vec<Value<'a>> {
        self.fields
    }

    /// Get the signature of this `Structure`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub(crate) fn try_to_owned(&self) -> crate::Result<Structure<'static>> {
        Ok(Structure {
            fields: self
                .fields
                .iter()
                .map(|v| v.try_to_owned().map(Into::into))
                .collect::<crate::Result<_>>()?,
            signature: self.signature.to_owned(),
        })
    }

    pub(crate) fn try_into_owned(self) -> crate::Result<Structure<'static>> {
        Ok(Structure {
            fields: self
                .fields
                .into_iter()
                .map(|v| v.try_into_owned().map(Into::into))
                .collect::<crate::Result<_>>()?,
            signature: self.signature,
        })
    }

    /// Attempt to clone `self`.
    pub fn try_clone(&self) -> Result<Self, crate::Error> {
        let fields = self
            .fields
            .iter()
            .map(|v| v.try_clone())
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(Self {
            fields,
            signature: self.signature.clone(),
        })
    }
}

impl Display for Structure<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        structure_display_fmt(self, f, true)
    }
}

pub(crate) fn structure_display_fmt(
    structure: &Structure<'_>,
    f: &mut std::fmt::Formatter<'_>,
    type_annotate: bool,
) -> std::fmt::Result {
    f.write_char('(')?;

    let fields = structure.fields();

    match fields.len() {
        0 => {}
        1 => {
            value_display_fmt(&fields[0], f, type_annotate)?;
            f.write_char(',')?;
        }
        _ => {
            for (i, field) in fields.iter().enumerate() {
                value_display_fmt(field, f, type_annotate)?;

                if i + 1 < fields.len() {
                    f.write_str(", ")?;
                }
            }
        }
    }

    f.write_char(')')
}

impl DynamicType for Structure<'_> {
    fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

impl DynamicType for StructureSeed<'_> {
    fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

impl<'a> DynamicDeserialize<'a> for Structure<'a> {
    type Deserializer = StructureSeed<'static>;

    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer> {
        let signature = match signature {
            Signature::Structure(_) => signature.clone(),
            s => Signature::structure([s.clone()]),
        };

        Ok(StructureSeed {
            signature,
            phantom: std::marker::PhantomData,
        })
    }
}

impl Serialize for Structure<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut structure = serializer.serialize_tuple_struct("Structure", self.fields.len())?;
        for field in &self.fields {
            field.serialize_value_as_tuple_struct_field(&mut structure)?;
        }
        structure.end()
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<'a, $($name),+> From<($($name),+,)> for Structure<'a>
            where
                $($name: DynamicType + Into<Value<'a>>,)+
            {
                #[inline]
                fn from(value: ($($name),+,)) -> Self {
                    StructureBuilder::new()
                    $(
                        .add_field(value. $n)
                    )+
                    .build().unwrap()
                }
            }

            impl<'a, E, $($name),+> TryFrom<Structure<'a>> for ($($name),+,)
            where
                $($name: TryFrom<Value<'a>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(mut s: Structure<'a>) -> core::result::Result<Self, Self::Error> {
                    Ok((
                    $(
                         $name::try_from(s.fields.remove(0))?,
                    )+
                    ))
                }
            }

            impl<'a, E, $($name),+> TryFrom<&Value<'a>> for ($($name),+,)
            where
                $($name: TryFrom<Value<'a>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(v: &Value<'a>) -> core::result::Result<Self, Self::Error> {
                    Self::try_from(Structure::try_from(v)?)
                }
            }

            impl<'a, E, $($name),+> TryFrom<Value<'a>> for ($($name),+,)
            where
                $($name: TryFrom<Value<'a>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(v: Value<'a>) -> core::result::Result<Self, Self::Error> {
                    Self::try_from(Structure::try_from(v)?)
                }
            }

            impl<E, $($name),+> TryFrom<OwnedValue> for ($($name),+,)
            where
                $($name: TryFrom<Value<'static>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(v: OwnedValue) -> core::result::Result<Self, Self::Error> {
                    Self::try_from(Value::from(v))
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

/// Owned [`Structure`]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedStructure(pub Structure<'static>);

/// Use this to deserialize an [`OwnedStructure`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedStructureSeed(Signature);

impl DynamicType for OwnedStructure {
    fn signature(&self) -> Signature {
        self.0.signature().clone()
    }
}

impl DynamicType for OwnedStructureSeed {
    fn signature(&self) -> Signature {
        self.0.clone()
    }
}

impl DynamicDeserialize<'_> for OwnedStructure {
    type Deserializer = OwnedStructureSeed;

    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer> {
        Structure::deserializer_for_signature(signature)
            .map(|StructureSeed { signature, .. }| OwnedStructureSeed(signature))
    }
}

impl<'de> DeserializeSeed<'de> for OwnedStructureSeed {
    type Value = OwnedStructure;
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer
            .deserialize_seq(StructureVisitor { signature: self.0 })
            .and_then(|s| match s.try_to_owned() {
                Ok(s) => Ok(OwnedStructure(s)),
                Err(e) => Err(D::Error::custom(e)),
            })
    }
}

impl Serialize for OwnedStructure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
