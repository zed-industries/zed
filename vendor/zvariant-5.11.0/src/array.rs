#![allow(unknown_lints)]
use serde::{
    de::{DeserializeSeed, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};
use std::fmt::{Display, Write};

use crate::{
    DynamicDeserialize, DynamicType, Error, Result, Signature, Type, Value,
    value::{SignatureSeed, value_display_fmt},
};

/// A helper type to wrap arrays in a [`Value`].
///
/// API is provided to convert from, and to a [`Vec`].
///
/// [`Value`]: enum.Value.html#variant.Array
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Array<'a> {
    elements: Vec<Value<'a>>,
    signature: Signature,
}

impl<'a> Array<'a> {
    /// Create a new empty `Array`, given the signature of the elements.
    pub fn new(element_signature: &Signature) -> Array<'a> {
        let signature = Signature::array(element_signature.clone());

        Array {
            elements: vec![],
            signature,
        }
    }

    pub(crate) fn new_full_signature(signature: &Signature) -> Array<'a> {
        assert!(matches!(signature, Signature::Array(_)));

        Array {
            elements: vec![],
            signature: signature.clone(),
        }
    }

    /// Append `element`.
    ///
    /// # Errors
    ///
    /// if `element`'s signature doesn't match the element signature `self` was created for.
    pub fn append<'e: 'a>(&mut self, element: Value<'e>) -> Result<()> {
        match &self.signature {
            Signature::Array(child) if element.value_signature() != child.signature() => {
                return Err(Error::SignatureMismatch(
                    element.value_signature().clone(),
                    child.signature().clone().to_string(),
                ));
            }
            Signature::Array(_) => (),
            _ => unreachable!("Incorrect `Array` signature"),
        }

        self.elements.push(element);

        Ok(())
    }

    /// Get all the elements.
    pub fn inner(&self) -> &[Value<'a>] {
        &self.elements
    }

    /// Get the value at the given index.
    pub fn get<V>(&'a self, idx: usize) -> Result<Option<V>>
    where
        V: TryFrom<&'a Value<'a>>,
        <V as TryFrom<&'a Value<'a>>>::Error: Into<crate::Error>,
    {
        self.elements
            .get(idx)
            .map(|v| v.downcast_ref::<V>())
            .transpose()
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    /// The signature of the `Array`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Get the signature of the elements in the `Array`.
    pub fn element_signature(&self) -> &Signature {
        match &self.signature {
            Signature::Array(child) => child.signature(),
            _ => unreachable!("Incorrect `Array` signature"),
        }
    }

    pub(crate) fn try_to_owned(&self) -> Result<Array<'static>> {
        Ok(Array {
            elements: self
                .elements
                .iter()
                .map(|v| v.try_to_owned().map(Into::into))
                .collect::<Result<_>>()?,
            signature: self.signature.clone(),
        })
    }

    pub(crate) fn try_into_owned(self) -> Result<Array<'static>> {
        Ok(Array {
            elements: self
                .elements
                .into_iter()
                .map(|v| v.try_into_owned().map(Into::into))
                .collect::<Result<_>>()?,
            signature: self.signature.clone(),
        })
    }

    /// Tries to clone the `Array`.
    pub fn try_clone(&self) -> crate::Result<Self> {
        let elements = self
            .elements
            .iter()
            .map(|v| v.try_clone())
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(Self {
            elements,
            signature: self.signature.clone(),
        })
    }
}

impl Display for Array<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        array_display_fmt(self, f, true)
    }
}

pub(crate) fn array_display_fmt(
    array: &Array<'_>,
    f: &mut std::fmt::Formatter<'_>,
    type_annotate: bool,
) -> std::fmt::Result {
    // Print as string if it is a bytestring (i.e., first nul character is the last byte)
    if let [leading @ .., Value::U8(b'\0')] = array.as_ref() {
        if !leading.contains(&Value::U8(b'\0')) {
            let bytes = leading
                .iter()
                .map(|v| {
                    v.downcast_ref::<u8>()
                        .expect("item must have a signature of a byte")
                })
                .collect::<Vec<_>>();

            let string = String::from_utf8_lossy(&bytes);
            write!(f, "b{:?}", string.as_ref())?;

            return Ok(());
        }
    }

    if array.is_empty() {
        if type_annotate {
            write!(f, "@{} ", array.signature())?;
        }
        f.write_str("[]")?;
    } else {
        f.write_char('[')?;

        // Annotate only the first item as the rest will be of the same type.
        let mut type_annotate = type_annotate;

        for (i, item) in array.iter().enumerate() {
            value_display_fmt(item, f, type_annotate)?;
            type_annotate = false;

            if i + 1 < array.len() {
                f.write_str(", ")?;
            }
        }

        f.write_char(']')?;
    }

    Ok(())
}

/// Use this to deserialize an [Array].
pub struct ArraySeed {
    signature: Signature,
    phantom: std::marker::PhantomData<()>,
}

impl ArraySeed {
    fn new(signature: &Signature) -> ArraySeed {
        ArraySeed {
            signature: signature.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl DynamicType for Array<'_> {
    fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

impl DynamicType for ArraySeed {
    fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

impl<'a> DynamicDeserialize<'a> for Array<'a> {
    type Deserializer = ArraySeed;

    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer> {
        if !matches!(signature, Signature::Array(_)) {
            return Err(zvariant::Error::SignatureMismatch(
                signature.clone(),
                "an array signature".to_owned(),
            ));
        };

        Ok(ArraySeed::new(signature))
    }
}

impl<'a> std::ops::Deref for Array<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<'a, T> From<Vec<T>> for Array<'a>
where
    T: Type + Into<Value<'a>>,
{
    fn from(values: Vec<T>) -> Self {
        let element_signature = T::SIGNATURE.clone();
        let elements = values.into_iter().map(Value::new).collect();
        let signature = Signature::array(element_signature);

        Self {
            elements,
            signature,
        }
    }
}

impl<'a, T> From<&[T]> for Array<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(values: &[T]) -> Self {
        let element_signature = T::SIGNATURE.clone();
        let elements = values
            .iter()
            .map(|value| Value::new(value.clone()))
            .collect();
        let signature = Signature::array(element_signature);

        Self {
            elements,
            signature,
        }
    }
}

impl<'a, T> From<&Vec<T>> for Array<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(values: &Vec<T>) -> Self {
        Self::from(&values[..])
    }
}

impl<'a, T> TryFrom<Array<'a>> for Vec<T>
where
    T: TryFrom<Value<'a>>,
    T::Error: Into<crate::Error>,
{
    type Error = Error;

    fn try_from(v: Array<'a>) -> core::result::Result<Self, Self::Error> {
        // there is no try_map yet..
        let mut res = vec![];
        for e in v.elements.into_iter() {
            let value = if let Value::Value(v) = e {
                T::try_from(*v)
            } else {
                T::try_from(e)
            }
            .map_err(Into::into)?;

            res.push(value);
        }
        Ok(res)
    }
}

// TODO: this could be useful
// impl<'a, 'b, T> TryFrom<&'a Array<'b>> for Vec<T>

impl Serialize for Array<'_> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.elements.len()))?;
        for element in &self.elements {
            element.serialize_value_as_seq_element(&mut seq)?;
        }

        seq.end()
    }
}

impl<'de> DeserializeSeed<'de> for ArraySeed {
    type Value = Array<'de>;
    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ArrayVisitor {
            signature: self.signature,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArrayVisitor {
    signature: Signature,
}

impl<'de> Visitor<'de> for ArrayVisitor {
    type Value = Array<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an Array value")
    }

    fn visit_seq<V>(self, visitor: V) -> std::result::Result<Array<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        SignatureSeed {
            signature: &self.signature,
        }
        .visit_array(visitor)
    }
}
