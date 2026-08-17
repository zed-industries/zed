use crate::errors::serialize::DeError;
use serde::ser::{Impossible, Serialize, Serializer};
use serde::serde_if_integer128;
use std::fmt::Write;

/// A serializer, that ensures, that only plain types can be serialized,
/// so result can be used as an XML tag or attribute name.
///
/// This serializer does not check that name does not contain characters that
/// [not allowed] in XML names, because in some cases it should pass names
/// that would be filtered on higher level.
///
/// [not allowed]: https://www.w3.org/TR/xml11/#sec-common-syn
pub struct QNameSerializer<W: Write> {
    /// Writer to which this serializer writes content
    pub writer: W,
}

impl<W: Write> QNameSerializer<W> {
    #[inline]
    fn write_str(&mut self, value: &str) -> Result<(), DeError> {
        Ok(self.writer.write_str(value)?)
    }
}

impl<W: Write> Serializer for QNameSerializer<W> {
    type Ok = W;
    type Error = DeError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    write_primitive!();

    fn serialize_str(mut self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.write_str(value)?;
        Ok(self.writer)
    }

    /// Because unit type can be represented only by empty string which is not
    /// a valid XML name, serialization of unit returns `Err(Unsupported)`
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(DeError::Unsupported(
            "unit type `()` cannot be serialized as an XML tag name".into(),
        ))
    }

    /// Because unit struct can be represented only by empty string which is not
    /// a valid XML name, serialization of unit struct returns `Err(Unsupported)`
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "unit struct `{}` cannot be serialized as an XML tag name",
                name
            )
            .into(),
        ))
    }

    /// We cannot store both a variant discriminant and a variant value,
    /// so serialization of enum newtype variant returns `Err(Unsupported)`
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, DeError> {
        Err(DeError::Unsupported(
            format!(
                "enum newtype variant `{}::{}` cannot be serialized as an XML tag name",
                name, variant
            )
            .into(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(DeError::Unsupported(
            "sequence cannot be serialized as an XML tag name".into(),
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(DeError::Unsupported(
            "tuple cannot be serialized as an XML tag name".into(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "tuple struct `{}` cannot be serialized as an XML tag name",
                name
            )
            .into(),
        ))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "enum tuple variant `{}::{}` cannot be serialized as an XML tag name",
                name, variant
            )
            .into(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DeError::Unsupported(
            "map cannot be serialized as an XML tag name".into(),
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(DeError::Unsupported(
            format!("struct `{}` cannot be serialized as an XML tag name", name).into(),
        ))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "enum struct variant `{}::{}` cannot be serialized as an XML tag name",
                name, variant
            )
            .into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Bytes;
    use pretty_assertions::assert_eq;
    use serde::Serialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Serialize, PartialEq)]
    struct Unit;

    #[derive(Debug, Serialize, PartialEq)]
    struct Newtype(bool);

    #[derive(Debug, Serialize, PartialEq)]
    struct Tuple(&'static str, usize);

    #[derive(Debug, Serialize, PartialEq)]
    struct Struct {
        key: &'static str,
        val: usize,
    }

    #[derive(Debug, Serialize, PartialEq)]
    enum Enum {
        Unit,
        #[serde(rename = "<\"&'>")]
        UnitEscaped,
        Newtype(bool),
        Tuple(&'static str, usize),
        Struct {
            key: &'static str,
            val: usize,
        },
    }

    /// Checks that given `$data` successfully serialized as `$expected`
    macro_rules! serialize_as {
        ($name:ident: $data:expr => $expected:literal) => {
            #[test]
            fn $name() {
                let ser = QNameSerializer {
                    writer: String::new(),
                };

                let buffer = $data.serialize(ser).unwrap();
                assert_eq!(buffer, $expected);
            }
        };
    }

    /// Checks that attempt to serialize given `$data` results to a
    /// serialization error `$kind` with `$reason`
    macro_rules! err {
        ($name:ident: $data:expr => $kind:ident($reason:literal)) => {
            #[test]
            fn $name() {
                let mut buffer = String::new();
                let ser = QNameSerializer {
                    writer: &mut buffer,
                };

                match $data.serialize(ser).unwrap_err() {
                    DeError::$kind(e) => assert_eq!(e, $reason),
                    e => panic!(
                        "Expected `{}({})`, found `{:?}`",
                        stringify!($kind),
                        $reason,
                        e
                    ),
                }
                assert_eq!(buffer, "");
            }
        };
    }

    serialize_as!(false_: false => "false");
    serialize_as!(true_:  true  => "true");

    serialize_as!(i8_:    -42i8                => "-42");
    serialize_as!(i16_:   -4200i16             => "-4200");
    serialize_as!(i32_:   -42000000i32         => "-42000000");
    serialize_as!(i64_:   -42000000000000i64   => "-42000000000000");
    serialize_as!(isize_: -42000000000000isize => "-42000000000000");

    serialize_as!(u8_:    42u8                => "42");
    serialize_as!(u16_:   4200u16             => "4200");
    serialize_as!(u32_:   42000000u32         => "42000000");
    serialize_as!(u64_:   42000000000000u64   => "42000000000000");
    serialize_as!(usize_: 42000000000000usize => "42000000000000");

    serde_if_integer128! {
        serialize_as!(i128_: -420000000000000000000000000000i128 => "-420000000000000000000000000000");
        serialize_as!(u128_:  420000000000000000000000000000u128 => "420000000000000000000000000000");
    }

    serialize_as!(f32_: 4.2f32 => "4.2");
    serialize_as!(f64_: 4.2f64 => "4.2");

    serialize_as!(char_non_escaped: 'h' => "h");
    serialize_as!(char_lt:   '<' => "<");
    serialize_as!(char_gt:   '>' => ">");
    serialize_as!(char_amp:  '&' => "&");
    serialize_as!(char_apos: '\'' => "'");
    serialize_as!(char_quot: '"' => "\"");

    serialize_as!(str_valid_name: "valid-name" => "valid-name");
    serialize_as!(str_space: "string with spaces" => "string with spaces");
    serialize_as!(str_lt:   "string<" => "string<");
    serialize_as!(str_gt:   "string>" => "string>");
    serialize_as!(str_amp:  "string&" => "string&");
    serialize_as!(str_apos: "string'" => "string'");
    serialize_as!(str_quot: "string\"" => "string\"");

    err!(bytes: Bytes(b"<\"escaped & bytes'>")
        => Unsupported("`serialize_bytes` not supported yet"));

    serialize_as!(option_none: Option::<&str>::None => "");
    serialize_as!(option_some: Some("non-escaped-string") => "non-escaped-string");

    err!(unit: ()
        => Unsupported("unit type `()` cannot be serialized as an XML tag name"));
    err!(unit_struct: Unit
        => Unsupported("unit struct `Unit` cannot be serialized as an XML tag name"));

    serialize_as!(enum_unit: Enum::Unit => "Unit");
    serialize_as!(enum_unit_escaped: Enum::UnitEscaped => "<\"&'>");

    serialize_as!(newtype: Newtype(true) => "true");
    err!(enum_newtype: Enum::Newtype(false)
        => Unsupported("enum newtype variant `Enum::Newtype` cannot be serialized as an XML tag name"));

    err!(seq: vec![1, 2, 3]
        => Unsupported("sequence cannot be serialized as an XML tag name"));
    err!(tuple: ("<\"&'>", "with\t\r\n spaces", 3usize)
        => Unsupported("tuple cannot be serialized as an XML tag name"));
    err!(tuple_struct: Tuple("first", 42)
        => Unsupported("tuple struct `Tuple` cannot be serialized as an XML tag name"));
    err!(enum_tuple: Enum::Tuple("first", 42)
        => Unsupported("enum tuple variant `Enum::Tuple` cannot be serialized as an XML tag name"));

    err!(map: BTreeMap::from([("_1", 2), ("_3", 4)])
        => Unsupported("map cannot be serialized as an XML tag name"));
    err!(struct_: Struct { key: "answer", val: 42 }
        => Unsupported("struct `Struct` cannot be serialized as an XML tag name"));
    err!(enum_struct: Enum::Struct { key: "answer", val: 42 }
        => Unsupported("enum struct variant `Enum::Struct` cannot be serialized as an XML tag name"));
}
