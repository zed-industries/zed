//! Contains Serde `Serializer` for XML [simple types] [as defined] in the XML Schema.
//!
//! [simple types]: https://www.w3schools.com/xml/el_simpletype.asp
//! [as defined]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition

use crate::errors::serialize::DeError;
use crate::escapei::_escape;
use crate::se::{Indent, QuoteLevel};
use serde::ser::{
    Impossible, Serialize, SerializeSeq, SerializeTuple, SerializeTupleStruct, Serializer,
};
use serde::serde_if_integer128;
use std::borrow::Cow;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteTarget {
    /// Escape data for a text content. No additional escape symbols
    Text,
    /// Escape data for a double-quoted attribute. `"` always escaped
    DoubleQAttr,
    /// Escape data for a single-quoted attribute. `'` always escaped
    SingleQAttr,
}

/// Escapes atomic value that could be part of a `xs:list`. All whitespace characters
/// additionally escaped
fn escape_item(value: &str, target: QuoteTarget, level: QuoteLevel) -> Cow<str> {
    use QuoteLevel::*;
    use QuoteTarget::*;

    match (target, level) {
        (_, Full) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items, cannot be used in the item
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' | b'>' | b'\'' | b'\"' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (Text, Partial) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items, cannot be used in the item
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            _ => false,
        }),
        (Text, Minimal) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items, cannot be used in the item
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (DoubleQAttr, Partial) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items, cannot be used in the item
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            // Double quoted attribute should escape quote
            b'"' => true,
            _ => false,
        }),
        (DoubleQAttr, Minimal) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items, cannot be used in the item
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' => true,
            // Double quoted attribute should escape quote
            b'"' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (SingleQAttr, Partial) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            // Single quoted attribute should escape quote
            b'\'' => true,
            _ => false,
        }),
        (SingleQAttr, Minimal) => _escape(value, |ch| match ch {
            // Spaces used as delimiters of list items
            b' ' | b'\r' | b'\n' | b'\t' => true,
            // Required characters to escape
            b'&' | b'<' => true,
            // Single quoted attribute should escape quote
            b'\'' => true,
            _ => false,
        }),
    }
}

/// Escapes XSD simple type value
fn escape_list(value: &str, target: QuoteTarget, level: QuoteLevel) -> Cow<str> {
    use QuoteLevel::*;
    use QuoteTarget::*;

    match (target, level) {
        (_, Full) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' | b'>' | b'\'' | b'\"' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (Text, Partial) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            _ => false,
        }),
        (Text, Minimal) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (DoubleQAttr, Partial) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            // Double quoted attribute should escape quote
            b'"' => true,
            _ => false,
        }),
        (DoubleQAttr, Minimal) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' => true,
            // Double quoted attribute should escape quote
            b'"' => true,
            _ => false,
        }),
        //----------------------------------------------------------------------
        (SingleQAttr, Partial) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' | b'>' => true,
            // Single quoted attribute should escape quote
            b'\'' => true,
            _ => false,
        }),
        (SingleQAttr, Minimal) => _escape(value, |ch| match ch {
            // Required characters to escape
            b'&' | b'<' => true,
            // Single quoted attribute should escape quote
            b'\'' => true,
            _ => false,
        }),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A serializer that handles ordinary [simple type definition][item] with
/// `{variety} = atomic`, or an ordinary [simple type] definition with
/// `{variety} = union` whose basic members are all atomic.
///
/// This serializer can serialize only primitive types:
/// - numbers
/// - booleans
/// - strings
/// - units
/// - options
/// - unit variants of enums
///
/// Identifiers represented as strings and serialized accordingly.
///
/// Serialization of all other types returns [`Unsupported`][DeError::Unsupported] error.
///
/// [item]: https://www.w3.org/TR/xmlschema11-1/#std-item_type_definition
/// [simple type]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition
pub struct AtomicSerializer<W: Write> {
    pub writer: W,
    pub target: QuoteTarget,
    /// Defines which XML characters need to be escaped
    pub level: QuoteLevel,
}

impl<W: Write> AtomicSerializer<W> {
    fn write_str(&mut self, value: &str) -> Result<(), DeError> {
        Ok(self
            .writer
            .write_str(&escape_item(value, self.target, self.level))?)
    }
}

impl<W: Write> Serializer for AtomicSerializer<W> {
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

    /// We cannot store anything, so the absence of a unit and presence of it
    /// does not differ, so serialization of unit returns `Err(Unsupported)`
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(DeError::Unsupported(
            "unit type `()` cannot be serialized as an `xs:list` item".into(),
        ))
    }

    /// We cannot store anything, so the absence of a unit and presence of it
    /// does not differ, so serialization of unit returns `Err(Unsupported)`
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "unit struct `{}` cannot be serialized as an `xs:list` item",
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
                "enum newtype variant `{}::{}` cannot be serialized as an `xs:list` item",
                name, variant
            )
            .into(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(DeError::Unsupported(
            "sequence cannot be serialized as an `xs:list` item".into(),
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(DeError::Unsupported(
            "tuple cannot be serialized as an `xs:list` item".into(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "tuple struct `{}` cannot be serialized as an `xs:list` item",
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
                "enum tuple variant `{}::{}` cannot be serialized as an `xs:list` item",
                name, variant
            )
            .into(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DeError::Unsupported(
            "map cannot be serialized as an `xs:list` item".into(),
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "struct `{}` cannot be serialized as an `xs:list` item",
                name
            )
            .into(),
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
                "enum struct variant `{}::{}` cannot be serialized as an `xs:list` item",
                name, variant
            )
            .into(),
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A serializer for a values representing XSD [simple types], which used in:
/// - attribute values (`<... ...="value" ...>`)
/// - text content (`<...>text</...>`)
/// - CDATA content (`<...><![CDATA[cdata]]></...>`)
///
/// [simple types]: https://www.w3.org/TR/xmlschema11-1/#Simple_Type_Definition
pub struct SimpleTypeSerializer<'i, W: Write> {
    /// Writer to which this serializer writes content
    pub writer: W,
    /// Target for which element is serializing. Affects additional characters to escape.
    pub target: QuoteTarget,
    /// Defines which XML characters need to be escaped
    pub level: QuoteLevel,
    /// Indent that should be written before the content if content is not an empty string
    pub(crate) indent: Indent<'i>,
}

impl<'i, W: Write> SimpleTypeSerializer<'i, W> {
    fn write_str(&mut self, value: &str) -> Result<(), DeError> {
        self.indent.write_indent(&mut self.writer)?;
        Ok(self
            .writer
            .write_str(&escape_list(value, self.target, self.level))?)
    }
}

impl<'i, W: Write> Serializer for SimpleTypeSerializer<'i, W> {
    type Ok = W;
    type Error = DeError;

    type SerializeSeq = SimpleSeq<'i, W>;
    type SerializeTuple = SimpleSeq<'i, W>;
    type SerializeTupleStruct = SimpleSeq<'i, W>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    write_primitive!();

    fn serialize_str(mut self, value: &str) -> Result<Self::Ok, Self::Error> {
        if value.is_empty() {
            self.indent = Indent::None;
        }
        self.write_str(value)?;
        Ok(self.writer)
    }

    /// Does not write anything
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer)
    }

    /// Does not write anything
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer)
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
            format!("enum newtype variant `{}::{}` cannot be serialized as an attribute or text content value", name, variant).into(),
        ))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SimpleSeq {
            writer: self.writer,
            target: self.target,
            level: self.level,
            first: true,
            indent: self.indent,
        })
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(None)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(DeError::Unsupported(
            format!("enum tuple variant `{}::{}` cannot be serialized as an attribute or text content value", name, variant).into(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DeError::Unsupported(
            "map cannot be serialized as an attribute or text content value".into(),
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(DeError::Unsupported(
            format!(
                "struct `{}` cannot be serialized as an attribute or text content value",
                name
            )
            .into(),
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
            format!("enum struct variant `{}::{}` cannot be serialized as an attribute or text content value", name, variant).into(),
        ))
    }
}

/// Serializer for a sequence of atomic values delimited by space
pub struct SimpleSeq<'i, W: Write> {
    writer: W,
    target: QuoteTarget,
    level: QuoteLevel,
    /// If `true`, nothing was written yet
    first: bool,
    /// Indent that should be written before the content if content is not an empty string
    indent: Indent<'i>,
}

impl<'i, W: Write> SerializeSeq for SimpleSeq<'i, W> {
    type Ok = W;
    type Error = DeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Write indent for the first element and delimiter for others
        //FIXME: sequence with only empty strings will be serialized as indent only + delimiters
        if self.first {
            self.indent.write_indent(&mut self.writer)?;
        } else {
            self.writer.write_char(' ')?;
        }
        self.first = false;
        value.serialize(AtomicSerializer {
            writer: &mut self.writer,
            target: self.target,
            level: self.level,
        })?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer)
    }
}

impl<'i, W: Write> SerializeTuple for SimpleSeq<'i, W> {
    type Ok = W;
    type Error = DeError;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl<'i, W: Write> SerializeTupleStruct for SimpleSeq<'i, W> {
    type Ok = W;
    type Error = DeError;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Bytes;
    use serde::Serialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Serialize, PartialEq)]
    struct Unit;

    #[derive(Debug, Serialize, PartialEq)]
    struct Newtype(usize);

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
        Newtype(usize),
        Tuple(&'static str, usize),
        Struct {
            key: &'static str,
            val: usize,
        },
    }

    mod escape_item {
        use super::*;

        mod full {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_item("text<\"'&> \t\n\rtext", QuoteTarget::Text, QuoteLevel::Full),
                    "text&lt;&quot;&apos;&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Full
                    ),
                    "text&lt;&quot;&apos;&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Full
                    ),
                    "text&lt;&quot;&apos;&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }
        }

        mod partial {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::Text,
                        QuoteLevel::Partial
                    ),
                    "text&lt;\"'&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Partial
                    ),
                    "text&lt;&quot;'&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Partial
                    ),
                    "text&lt;\"&apos;&amp;&gt;&#32;&#9;&#10;&#13;text"
                );
            }
        }

        mod minimal {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::Text,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;\"'&amp;>&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;&quot;'&amp;>&#32;&#9;&#10;&#13;text"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_item(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;\"&apos;&amp;>&#32;&#9;&#10;&#13;text"
                );
            }
        }
    }

    mod escape_list {
        use super::*;

        mod full {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_list("text<\"'&> \t\n\rtext", QuoteTarget::Text, QuoteLevel::Full),
                    "text&lt;&quot;&apos;&amp;&gt; \t\n\rtext"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Full
                    ),
                    "text&lt;&quot;&apos;&amp;&gt; \t\n\rtext"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Full
                    ),
                    "text&lt;&quot;&apos;&amp;&gt; \t\n\rtext"
                );
            }
        }

        mod partial {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::Text,
                        QuoteLevel::Partial
                    ),
                    "text&lt;\"'&amp;&gt; \t\n\rtext"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Partial
                    ),
                    "text&lt;&quot;'&amp;&gt; \t\n\rtext"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Partial
                    ),
                    "text&lt;\"&apos;&amp;&gt; \t\n\rtext"
                );
            }
        }

        mod minimal {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn text() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::Text,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;\"'&amp;> \t\n\rtext"
                );
            }

            #[test]
            fn double_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::DoubleQAttr,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;&quot;'&amp;> \t\n\rtext"
                );
            }

            #[test]
            fn single_quote_attr() {
                assert_eq!(
                    escape_list(
                        "text<\"'&> \t\n\rtext",
                        QuoteTarget::SingleQAttr,
                        QuoteLevel::Minimal
                    ),
                    "text&lt;\"&apos;&amp;> \t\n\rtext"
                );
            }
        }
    }

    /// Tests for serialize atomic and union values, as defined in XSD specification
    mod atomic {
        use super::*;
        use pretty_assertions::assert_eq;

        /// Checks that given `$data` successfully serialized as `$expected`
        macro_rules! serialize_as {
            ($name:ident: $data:expr => $expected:literal) => {
                #[test]
                fn $name() {
                    let ser = AtomicSerializer {
                        writer: String::new(),
                        target: QuoteTarget::Text,
                        level: QuoteLevel::Full,
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
                    let ser = AtomicSerializer {
                        writer: &mut buffer,
                        target: QuoteTarget::Text,
                        level: QuoteLevel::Full,
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
        serialize_as!(char_lt:   '<' => "&lt;");
        serialize_as!(char_gt:   '>' => "&gt;");
        serialize_as!(char_amp:  '&' => "&amp;");
        serialize_as!(char_apos: '\'' => "&apos;");
        serialize_as!(char_quot: '"' => "&quot;");

        serialize_as!(str_non_escaped: "non-escaped-string" => "non-escaped-string");
        serialize_as!(str_escaped: "<\"escaped & string'>" => "&lt;&quot;escaped&#32;&amp;&#32;string&apos;&gt;");

        err!(bytes: Bytes(b"<\"escaped & bytes'>")
            => Unsupported("`serialize_bytes` not supported yet"));

        serialize_as!(option_none: Option::<&str>::None => "");
        serialize_as!(option_some: Some("non-escaped-string") => "non-escaped-string");

        err!(unit: ()
            => Unsupported("unit type `()` cannot be serialized as an `xs:list` item"));
        err!(unit_struct: Unit
            => Unsupported("unit struct `Unit` cannot be serialized as an `xs:list` item"));

        serialize_as!(enum_unit: Enum::Unit => "Unit");
        serialize_as!(enum_unit_escaped: Enum::UnitEscaped => "&lt;&quot;&amp;&apos;&gt;");

        serialize_as!(newtype: Newtype(42) => "42");
        err!(enum_newtype: Enum::Newtype(42)
            => Unsupported("enum newtype variant `Enum::Newtype` cannot be serialized as an `xs:list` item"));

        err!(seq: vec![1, 2, 3]
            => Unsupported("sequence cannot be serialized as an `xs:list` item"));
        err!(tuple: ("<\"&'>", "with\t\n\r spaces", 3usize)
            => Unsupported("tuple cannot be serialized as an `xs:list` item"));
        err!(tuple_struct: Tuple("first", 42)
            => Unsupported("tuple struct `Tuple` cannot be serialized as an `xs:list` item"));
        err!(enum_tuple: Enum::Tuple("first", 42)
            => Unsupported("enum tuple variant `Enum::Tuple` cannot be serialized as an `xs:list` item"));

        err!(map: BTreeMap::from([(1, 2), (3, 4)])
            => Unsupported("map cannot be serialized as an `xs:list` item"));
        err!(struct_: Struct { key: "answer", val: 42 }
            => Unsupported("struct `Struct` cannot be serialized as an `xs:list` item"));
        err!(enum_struct: Enum::Struct { key: "answer", val: 42 }
            => Unsupported("enum struct variant `Enum::Struct` cannot be serialized as an `xs:list` item"));
    }

    mod simple_type {
        use super::*;
        use pretty_assertions::assert_eq;

        /// Checks that given `$data` successfully serialized as `$expected`
        macro_rules! serialize_as {
            ($name:ident: $data:expr => $expected:literal) => {
                #[test]
                fn $name() {
                    let ser = SimpleTypeSerializer {
                        writer: String::new(),
                        target: QuoteTarget::Text,
                        level: QuoteLevel::Full,
                        indent: Indent::None,
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
                    let ser = SimpleTypeSerializer {
                        writer: &mut buffer,
                        target: QuoteTarget::Text,
                        level: QuoteLevel::Full,
                        indent: Indent::None,
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
        serialize_as!(char_lt:   '<' => "&lt;");
        serialize_as!(char_gt:   '>' => "&gt;");
        serialize_as!(char_amp:  '&' => "&amp;");
        serialize_as!(char_apos: '\'' => "&apos;");
        serialize_as!(char_quot: '"' => "&quot;");

        serialize_as!(str_non_escaped: "non-escaped string" => "non-escaped string");
        serialize_as!(str_escaped: "<\"escaped & string'>" => "&lt;&quot;escaped &amp; string&apos;&gt;");

        err!(bytes: Bytes(b"<\"escaped & bytes'>")
            => Unsupported("`serialize_bytes` not supported yet"));

        serialize_as!(option_none: Option::<&str>::None => "");
        serialize_as!(option_some: Some("non-escaped string") => "non-escaped string");

        serialize_as!(unit: () => "");
        serialize_as!(unit_struct: Unit => "");

        serialize_as!(enum_unit: Enum::Unit => "Unit");
        serialize_as!(enum_unit_escaped: Enum::UnitEscaped => "&lt;&quot;&amp;&apos;&gt;");

        serialize_as!(newtype: Newtype(42) => "42");
        err!(enum_newtype: Enum::Newtype(42)
            => Unsupported("enum newtype variant `Enum::Newtype` cannot be serialized as an attribute or text content value"));

        serialize_as!(seq: vec![1, 2, 3] => "1 2 3");
        serialize_as!(seq_empty: Vec::<usize>::new() => "");
        serialize_as!(seq_with_1_empty_str: vec![""] => "");
        serialize_as!(seq_with_2_empty_strs: vec!["", ""] => " ");
        serialize_as!(tuple: ("<\"&'>", "with\t\n\r spaces", 3usize)
            => "&lt;&quot;&amp;&apos;&gt; with&#9;&#10;&#13;&#32;spaces 3");
        serialize_as!(tuple_struct: Tuple("first", 42) => "first 42");
        err!(enum_tuple: Enum::Tuple("first", 42)
            => Unsupported("enum tuple variant `Enum::Tuple` cannot be serialized as an attribute or text content value"));

        err!(map: BTreeMap::from([(1, 2), (3, 4)])
            => Unsupported("map cannot be serialized as an attribute or text content value"));
        err!(struct_: Struct { key: "answer", val: 42 }
            => Unsupported("struct `Struct` cannot be serialized as an attribute or text content value"));
        err!(enum_struct: Enum::Struct { key: "answer", val: 42 }
            => Unsupported("enum struct variant `Enum::Struct` cannot be serialized as an attribute or text content value"));
    }
}
