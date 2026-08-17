use crate::prelude::*;

/// Support deserializing from flattened and non-flattened representation
///
/// When working with different serialization formats, sometimes it is more idiomatic to flatten
/// fields, while other formats prefer nesting. Using `#[serde(flatten)]` only the flattened form
/// is supported.
///
/// This helper creates a function, which support deserializing from either the flattened or the
/// nested form. It gives an error, when both forms are provided. The `flatten` attribute is
/// required on the field such that the helper works. The serialization format will always be
/// flattened.
///
/// # Examples
///
/// ```rust
/// # use serde::Deserialize;
/// #
/// // Setup the types
/// #[derive(Deserialize, Debug)]
/// struct S {
///     #[serde(flatten, deserialize_with = "deserialize_t")]
///     t: T,
/// }
///
/// #[derive(Deserialize, Debug)]
/// struct T {
///     i: i32,
/// }
///
/// // The macro creates custom deserialization code.
/// // You need to specify a function name and the field name of the flattened field.
/// serde_with::flattened_maybe!(deserialize_t, "t");
///
/// # fn main() {
/// // Supports both flattened
/// let j = r#" {"i":1} "#;
/// assert!(serde_json::from_str::<S>(j).is_ok());
/// # // Ensure the t field is not dead code
/// # assert_eq!(serde_json::from_str::<S>(j).unwrap().t.i, 1);
///
/// // and non-flattened versions.
/// let j = r#" {"t":{"i":1}} "#;
/// assert!(serde_json::from_str::<S>(j).is_ok());
///
/// // Ensure that the value is given
/// let j = r#" {} "#;
/// assert!(serde_json::from_str::<S>(j).is_err());
///
/// // and only occurs once, not multiple times.
/// let j = r#" {"i":1,"t":{"i":1}} "#;
/// assert!(serde_json::from_str::<S>(j).is_err());
/// # }
/// ```
#[macro_export]
macro_rules! flattened_maybe {
    ($fn:ident, $field:tt) => {
        fn $fn<'de, T, D>(deserializer: D) -> $crate::__private__::Result<T, D::Error>
        where
            T: $crate::__private__::Deserialize<'de>,
            D: $crate::__private__::Deserializer<'de>,
        {
            $crate::__private__::DeserializeSeed::deserialize(
                $crate::flatten_maybe::FlattenedMaybe($field, $crate::__private__::PhantomData),
                deserializer,
            )
        }
    };
}

/// Helper struct for the deserialization of the flattened maybe field.
///
/// Takes as first value the field name of the non-flattened field.
pub struct FlattenedMaybe<T>(pub &'static str, pub PhantomData<T>);

impl<'de, T> DeserializeSeed<'de> for FlattenedMaybe<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field<'de> {
            // Marked for the non-flattened field
            field_not_flat,
            // Rest, buffered to be deserialized later
            other(content::de::Content<'de>),
        }

        struct FieldVisitor<'a> {
            fieldname: &'a str,
        }

        impl<'a, 'de> Visitor<'de> for FieldVisitor<'a> {
            type Value = Field<'de>;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Formatter::write_str(formatter, "field identifier")
            }
            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::Bool(value)))
            }
            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::I8(value)))
            }
            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::I16(value)))
            }
            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::I32(value)))
            }
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::I64(value)))
            }
            fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::I128(value)))
            }
            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::U8(value)))
            }
            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::U16(value)))
            }
            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::U32(value)))
            }
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::U64(value)))
            }
            fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::U128(value)))
            }
            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::F32(value)))
            }
            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::F64(value)))
            }
            fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::Char(value)))
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(Field::other(content::de::Content::Unit))
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value == self.fieldname {
                    Ok(Field::field_not_flat)
                } else {
                    let value = content::de::Content::String(ToString::to_string(value));
                    Ok(Field::other(value))
                }
            }
            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value == self.fieldname.as_bytes() {
                    Ok(Field::field_not_flat)
                } else {
                    let value = content::de::Content::ByteBuf(value.to_vec());
                    Ok(Field::other(value))
                }
            }
            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value == self.fieldname {
                    Ok(Field::field_not_flat)
                } else {
                    let value = content::de::Content::Str(value);
                    Ok(Field::other(value))
                }
            }
            fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if value == self.fieldname.as_bytes() {
                    Ok(Field::field_not_flat)
                } else {
                    let value = content::de::Content::Bytes(value);
                    Ok(Field::other(value))
                }
            }
        }

        impl<'de> DeserializeSeed<'de> for FieldVisitor<'_> {
            type Value = Field<'de>;

            #[inline]
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserializer::deserialize_identifier(deserializer, self)
            }
        }

        struct FlattenedMaybeVisitor<T> {
            is_human_readable: bool,
            fieldname: &'static str,
            marker: PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for FlattenedMaybeVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_fmt(format_args!(
                    "a structure with a maybe flattened field `{}`",
                    self.fieldname,
                ))
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // Set to Some if field is present
                let mut value_not_flat: Option<Option<T>> = None;
                // Collect all other fields or the flattened fields
                let mut collect =
                    Vec::<Option<(content::de::Content<'_>, content::de::Content<'_>)>>::new();

                // Iterate over the map
                while let Some(key) = MapAccess::next_key_seed(
                    &mut map,
                    FieldVisitor {
                        fieldname: self.fieldname,
                    },
                )? {
                    match key {
                        Field::field_not_flat => {
                            if Option::is_some(&value_not_flat) {
                                return Err(<A::Error as DeError>::duplicate_field(self.fieldname));
                            }
                            value_not_flat = Some(MapAccess::next_value::<Option<T>>(&mut map)?);
                        }
                        Field::other(name) => {
                            collect.push(Some((name, MapAccess::next_value(&mut map)?)));
                        }
                    }
                }

                // Map is done, now check what we got
                let value_not_flat = value_not_flat.flatten();
                // Try to reconstruct the flattened structure
                let value_flat: Option<T> =
                    Deserialize::deserialize(content::de::FlatMapDeserializer(
                        &mut collect,
                        PhantomData,
                        self.is_human_readable,
                    ))?;

                // Check that exactly one of the two options is set
                match (value_flat, value_not_flat) {
                    (Some(t), None) | (None, Some(t)) => Ok(t),
                    (None, None) => Err(DeError::missing_field(self.fieldname)),
                    (Some(_), Some(_)) => Err(DeError::custom(format_args!(
                        "`{}` is both flattened and not",
                        self.fieldname,
                    ))),
                }
            }
        }

        let is_human_readable = deserializer.is_human_readable();
        Deserializer::deserialize_map(
            deserializer,
            FlattenedMaybeVisitor {
                is_human_readable,
                fieldname: self.0,
                marker: PhantomData,
            },
        )
    }
}
