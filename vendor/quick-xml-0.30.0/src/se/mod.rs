//! Module to handle custom serde `Serializer`

/// Implements writing primitives to the underlying writer.
/// Implementor must provide `write_str(self, &str) -> Result<(), DeError>` method
macro_rules! write_primitive {
    ($method:ident ( $ty:ty )) => {
        fn $method(mut self, value: $ty) -> Result<Self::Ok, Self::Error> {
            self.write_str(&value.to_string())?;
            Ok(self.writer)
        }
    };
    () => {
        fn serialize_bool(mut self, value: bool) -> Result<Self::Ok, Self::Error> {
            self.write_str(if value { "true" } else { "false" })?;
            Ok(self.writer)
        }

        write_primitive!(serialize_i8(i8));
        write_primitive!(serialize_i16(i16));
        write_primitive!(serialize_i32(i32));
        write_primitive!(serialize_i64(i64));

        write_primitive!(serialize_u8(u8));
        write_primitive!(serialize_u16(u16));
        write_primitive!(serialize_u32(u32));
        write_primitive!(serialize_u64(u64));

        serde_if_integer128! {
            write_primitive!(serialize_i128(i128));
            write_primitive!(serialize_u128(u128));
        }

        write_primitive!(serialize_f32(f32));
        write_primitive!(serialize_f64(f64));

        fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(&value.to_string())
        }

        fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
            //TODO: customization point - allow user to decide how to encode bytes
            Err(DeError::Unsupported(
                "`serialize_bytes` not supported yet".into(),
            ))
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Ok(self.writer)
        }

        fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
            value.serialize(self)
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(variant)
        }

        fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error> {
            value.serialize(self)
        }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

mod content;
mod element;
pub(crate) mod key;
pub(crate) mod simple_type;

use self::content::ContentSerializer;
use self::element::ElementSerializer;
use crate::errors::serialize::DeError;
use crate::writer::Indentation;
use serde::ser::{self, Serialize};
use serde::serde_if_integer128;
use std::fmt::Write;
use std::str::from_utf8;

/// Serialize struct into a `Write`r.
///
/// # Examples
///
/// ```
/// # use quick_xml::se::to_writer;
/// # use serde::Serialize;
/// # use pretty_assertions::assert_eq;
/// #[derive(Serialize)]
/// struct Root<'a> {
///     #[serde(rename = "@attribute")]
///     attribute: &'a str,
///     element: &'a str,
///     #[serde(rename = "$text")]
///     text: &'a str,
/// }
///
/// let data = Root {
///     attribute: "attribute content",
///     element: "element content",
///     text: "text content",
/// };
///
/// let mut buffer = String::new();
/// to_writer(&mut buffer, &data).unwrap();
/// assert_eq!(
///     buffer,
///     // The root tag name is automatically deduced from the struct name
///     // This will not work for other types or struct with #[serde(flatten)] fields
///     "<Root attribute=\"attribute content\">\
///         <element>element content</element>\
///         text content\
///     </Root>"
/// );
/// ```
pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<(), DeError>
where
    W: Write,
    T: ?Sized + Serialize,
{
    value.serialize(Serializer::new(&mut writer))
}

/// Serialize struct into a `String`.
///
/// # Examples
///
/// ```
/// # use quick_xml::se::to_string;
/// # use serde::Serialize;
/// # use pretty_assertions::assert_eq;
/// #[derive(Serialize)]
/// struct Root<'a> {
///     #[serde(rename = "@attribute")]
///     attribute: &'a str,
///     element: &'a str,
///     #[serde(rename = "$text")]
///     text: &'a str,
/// }
///
/// let data = Root {
///     attribute: "attribute content",
///     element: "element content",
///     text: "text content",
/// };
///
/// assert_eq!(
///     to_string(&data).unwrap(),
///     // The root tag name is automatically deduced from the struct name
///     // This will not work for other types or struct with #[serde(flatten)] fields
///     "<Root attribute=\"attribute content\">\
///         <element>element content</element>\
///         text content\
///     </Root>"
/// );
/// ```
pub fn to_string<T>(value: &T) -> Result<String, DeError>
where
    T: ?Sized + Serialize,
{
    let mut buffer = String::new();
    to_writer(&mut buffer, value)?;
    Ok(buffer)
}

/// Serialize struct into a `Write`r using specified root tag name.
/// `root_tag` should be valid [XML name], otherwise error is returned.
///
/// # Examples
///
/// ```
/// # use quick_xml::se::to_writer_with_root;
/// # use serde::Serialize;
/// # use pretty_assertions::assert_eq;
/// #[derive(Serialize)]
/// struct Root<'a> {
///     #[serde(rename = "@attribute")]
///     attribute: &'a str,
///     element: &'a str,
///     #[serde(rename = "$text")]
///     text: &'a str,
/// }
///
/// let data = Root {
///     attribute: "attribute content",
///     element: "element content",
///     text: "text content",
/// };
///
/// let mut buffer = String::new();
/// to_writer_with_root(&mut buffer, "top-level", &data).unwrap();
/// assert_eq!(
///     buffer,
///     "<top-level attribute=\"attribute content\">\
///         <element>element content</element>\
///         text content\
///     </top-level>"
/// );
/// ```
///
/// [XML name]: https://www.w3.org/TR/xml11/#NT-Name
pub fn to_writer_with_root<W, T>(mut writer: W, root_tag: &str, value: &T) -> Result<(), DeError>
where
    W: Write,
    T: ?Sized + Serialize,
{
    value.serialize(Serializer::with_root(&mut writer, Some(root_tag))?)
}

/// Serialize struct into a `String` using specified root tag name.
/// `root_tag` should be valid [XML name], otherwise error is returned.
///
/// # Examples
///
/// ```
/// # use quick_xml::se::to_string_with_root;
/// # use serde::Serialize;
/// # use pretty_assertions::assert_eq;
/// #[derive(Serialize)]
/// struct Root<'a> {
///     #[serde(rename = "@attribute")]
///     attribute: &'a str,
///     element: &'a str,
///     #[serde(rename = "$text")]
///     text: &'a str,
/// }
///
/// let data = Root {
///     attribute: "attribute content",
///     element: "element content",
///     text: "text content",
/// };
///
/// assert_eq!(
///     to_string_with_root("top-level", &data).unwrap(),
///     "<top-level attribute=\"attribute content\">\
///         <element>element content</element>\
///         text content\
///     </top-level>"
/// );
/// ```
///
/// [XML name]: https://www.w3.org/TR/xml11/#NT-Name
pub fn to_string_with_root<T>(root_tag: &str, value: &T) -> Result<String, DeError>
where
    T: ?Sized + Serialize,
{
    let mut buffer = String::new();
    to_writer_with_root(&mut buffer, root_tag, value)?;
    Ok(buffer)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Defines which characters would be escaped in [`Text`] events and attribute
/// values.
///
/// [`Text`]: crate::events::Event::Text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteLevel {
    /// Performs escaping, escape all characters that could have special meaning
    /// in the XML. This mode is compatible with SGML specification.
    ///
    /// Characters that will be replaced:
    ///
    /// Original | Replacement
    /// ---------|------------
    /// `<`      | `&lt;`
    /// `>`      | `&gt;`
    /// `&`      | `&amp;`
    /// `"`      | `&quot;`
    /// `'`      | `&apos;`
    Full,
    /// Performs escaping that is compatible with SGML specification.
    ///
    /// This level adds escaping of `>` to the `Minimal` level, which is [required]
    /// for compatibility with SGML.
    ///
    /// Characters that will be replaced:
    ///
    /// Original | Replacement
    /// ---------|------------
    /// `<`      | `&lt;`
    /// `>`      | `&gt;`
    /// `&`      | `&amp;`
    ///
    /// [required]: https://www.w3.org/TR/xml11/#syntax
    Partial,
    /// Performs the minimal possible escaping, escape only strictly necessary
    /// characters.
    ///
    /// Characters that will be replaced:
    ///
    /// Original | Replacement
    /// ---------|------------
    /// `<`      | `&lt;`
    /// `&`      | `&amp;`
    Minimal,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Implements serialization method by forwarding it to the serializer created by
/// the helper method [`Serializer::ser`].
macro_rules! forward {
    ($name:ident($ty:ty)) => {
        fn $name(self, value: $ty) -> Result<Self::Ok, Self::Error> {
            self.ser(&concat!("`", stringify!($ty), "`"))?.$name(value)
        }
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Almost all characters can form a name. Citation from <https://www.w3.org/TR/xml11/#sec-xml11>:
///
/// > The overall philosophy of names has changed since XML 1.0. Whereas XML 1.0
/// > provided a rigid definition of names, wherein everything that was not permitted
/// > was forbidden, XML 1.1 names are designed so that everything that is not
/// > forbidden (for a specific reason) is permitted. Since Unicode will continue
/// > to grow past version 4.0, further changes to XML can be avoided by allowing
/// > almost any character, including those not yet assigned, in names.
///
/// <https://www.w3.org/TR/xml11/#NT-NameStartChar>
const fn is_xml11_name_start_char(ch: char) -> bool {
    match ch {
        ':'
        | 'A'..='Z'
        | '_'
        | 'a'..='z'
        | '\u{00C0}'..='\u{00D6}'
        | '\u{00D8}'..='\u{00F6}'
        | '\u{00F8}'..='\u{02FF}'
        | '\u{0370}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}
/// <https://www.w3.org/TR/xml11/#NT-NameChar>
const fn is_xml11_name_char(ch: char) -> bool {
    match ch {
        '-' | '.' | '0'..='9' | '\u{00B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}' => {
            true
        }
        _ => is_xml11_name_start_char(ch),
    }
}

/// Helper struct to self-defense from errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub(self) struct XmlName<'n>(&'n str);

impl<'n> XmlName<'n> {
    /// Checks correctness of the XML name according to [XML 1.1 specification]
    ///
    /// [XML 1.1 specification]: https://www.w3.org/TR/xml11/#NT-Name
    pub fn try_from(name: &'n str) -> Result<XmlName<'n>, DeError> {
        //TODO: Customization point: allow user to decide if he want to reject or encode the name
        match name.chars().next() {
            Some(ch) if !is_xml11_name_start_char(ch) => Err(DeError::Unsupported(
                format!("character `{ch}` is not allowed at the start of an XML name `{name}`")
                    .into(),
            )),
            _ => match name.matches(|ch| !is_xml11_name_char(ch)).next() {
                Some(s) => Err(DeError::Unsupported(
                    format!("character `{s}` is not allowed in an XML name `{name}`").into(),
                )),
                None => Ok(XmlName(name)),
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) enum Indent<'i> {
    None,
    Owned(Indentation),
    Borrow(&'i mut Indentation),
}

impl<'i> Indent<'i> {
    pub fn borrow(&mut self) -> Indent {
        match self {
            Self::None => Indent::None,
            Self::Owned(ref mut i) => Indent::Borrow(i),
            Self::Borrow(i) => Indent::Borrow(i),
        }
    }

    pub fn increase(&mut self) {
        match self {
            Self::None => {}
            Self::Owned(i) => i.grow(),
            Self::Borrow(i) => i.grow(),
        }
    }

    pub fn decrease(&mut self) {
        match self {
            Self::None => {}
            Self::Owned(i) => i.shrink(),
            Self::Borrow(i) => i.shrink(),
        }
    }

    pub fn write_indent<W: std::fmt::Write>(&mut self, mut writer: W) -> Result<(), DeError> {
        match self {
            Self::None => {}
            Self::Owned(i) => {
                writer.write_char('\n')?;
                writer.write_str(from_utf8(i.current())?)?;
            }
            Self::Borrow(i) => {
                writer.write_char('\n')?;
                writer.write_str(from_utf8(i.current())?)?;
            }
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A Serializer
pub struct Serializer<'w, 'r, W: Write> {
    ser: ContentSerializer<'w, 'r, W>,
    /// Name of the root tag. If not specified, deduced from the structure name
    root_tag: Option<XmlName<'r>>,
}

impl<'w, 'r, W: Write> Serializer<'w, 'r, W> {
    /// Creates a new `Serializer` that uses struct name as a root tag name.
    ///
    /// Note, that attempt to serialize a non-struct (including unit structs
    /// and newtype structs) will end up to an error. Use `with_root` to create
    /// serializer with explicitly defined root element name
    pub fn new(writer: &'w mut W) -> Self {
        Self {
            ser: ContentSerializer {
                writer,
                level: QuoteLevel::Full,
                indent: Indent::None,
                write_indent: false,
                expand_empty_elements: false,
            },
            root_tag: None,
        }
    }

    /// Creates a new `Serializer` that uses specified root tag name. `name` should
    /// be valid [XML name], otherwise error is returned.
    ///
    /// # Examples
    ///
    /// When serializing a primitive type, only its representation will be written:
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// # use serde::Serialize;
    /// # use quick_xml::se::Serializer;
    ///
    /// let mut buffer = String::new();
    /// let ser = Serializer::with_root(&mut buffer, Some("root")).unwrap();
    ///
    /// "node".serialize(ser).unwrap();
    /// assert_eq!(buffer, "<root>node</root>");
    /// ```
    ///
    /// When serializing a struct, newtype struct, unit struct or tuple `root_tag`
    /// is used as tag name of root(s) element(s):
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// # use serde::Serialize;
    /// # use quick_xml::se::Serializer;
    ///
    /// #[derive(Debug, PartialEq, Serialize)]
    /// struct Struct {
    ///     question: String,
    ///     answer: u32,
    /// }
    ///
    /// let mut buffer = String::new();
    /// let ser = Serializer::with_root(&mut buffer, Some("root")).unwrap();
    ///
    /// let data = Struct {
    ///     question: "The Ultimate Question of Life, the Universe, and Everything".into(),
    ///     answer: 42,
    /// };
    ///
    /// data.serialize(ser).unwrap();
    /// assert_eq!(
    ///     buffer,
    ///     "<root>\
    ///         <question>The Ultimate Question of Life, the Universe, and Everything</question>\
    ///         <answer>42</answer>\
    ///      </root>"
    /// );
    /// ```
    ///
    /// [XML name]: https://www.w3.org/TR/xml11/#NT-Name
    pub fn with_root(writer: &'w mut W, root_tag: Option<&'r str>) -> Result<Self, DeError> {
        Ok(Self {
            ser: ContentSerializer {
                writer,
                level: QuoteLevel::Full,
                indent: Indent::None,
                write_indent: false,
                expand_empty_elements: false,
            },
            root_tag: root_tag.map(|tag| XmlName::try_from(tag)).transpose()?,
        })
    }

    /// Enable or disable expansion of empty elements. Defaults to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// # use serde::Serialize;
    /// # use quick_xml::se::Serializer;
    ///
    /// #[derive(Debug, PartialEq, Serialize)]
    /// struct Struct {
    ///     question: Option<String>,
    /// }
    ///
    /// let mut buffer = String::new();
    /// let mut ser = Serializer::new(&mut buffer);
    /// ser.expand_empty_elements(true);
    ///
    /// let data = Struct {
    ///   question: None,
    /// };
    ///
    /// data.serialize(ser).unwrap();
    /// assert_eq!(
    ///     buffer,
    ///     "<Struct><question></question></Struct>"
    /// );
    /// ```
    pub fn expand_empty_elements(&mut self, expand: bool) -> &mut Self {
        self.ser.expand_empty_elements = expand;
        self
    }

    /// Configure indent for a serializer
    pub fn indent(&mut self, indent_char: char, indent_size: usize) -> &mut Self {
        self.ser.indent = Indent::Owned(Indentation::new(indent_char as u8, indent_size));
        self
    }

    /// Set the indent object for a serializer
    pub(crate) fn set_indent(&mut self, indent: Indent<'r>) -> &mut Self {
        self.ser.indent = indent;
        self
    }

    /// Creates actual serializer or returns an error if root tag is not defined.
    /// In that case `err` contains the name of type that cannot be serialized.
    fn ser(self, err: &str) -> Result<ElementSerializer<'w, 'r, W>, DeError> {
        if let Some(key) = self.root_tag {
            Ok(ElementSerializer { ser: self.ser, key })
        } else {
            Err(DeError::Unsupported(
                format!("cannot serialize {} without defined root tag", err).into(),
            ))
        }
    }

    /// Creates actual serializer using root tag or a specified `key` if root tag
    /// is not defined. Returns an error if root tag is not defined and a `key`
    /// does not conform [XML rules](XmlName::try_from) for names.
    fn ser_name(self, key: &'static str) -> Result<ElementSerializer<'w, 'r, W>, DeError> {
        Ok(ElementSerializer {
            ser: self.ser,
            key: match self.root_tag {
                Some(key) => key,
                None => XmlName::try_from(key)?,
            },
        })
    }
}

impl<'w, 'r, W: Write> ser::Serializer for Serializer<'w, 'r, W> {
    type Ok = ();
    type Error = DeError;

    type SerializeSeq = <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeSeq;
    type SerializeTuple = <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeTuple;
    type SerializeTupleStruct =
        <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant =
        <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeTupleVariant;
    type SerializeMap = <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeMap;
    type SerializeStruct = <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeStruct;
    type SerializeStructVariant =
        <ElementSerializer<'w, 'r, W> as ser::Serializer>::SerializeStructVariant;

    forward!(serialize_bool(bool));

    forward!(serialize_i8(i8));
    forward!(serialize_i16(i16));
    forward!(serialize_i32(i32));
    forward!(serialize_i64(i64));

    forward!(serialize_u8(u8));
    forward!(serialize_u16(u16));
    forward!(serialize_u32(u32));
    forward!(serialize_u64(u64));

    serde_if_integer128! {
        forward!(serialize_i128(i128));
        forward!(serialize_u128(u128));
    }

    forward!(serialize_f32(f32));
    forward!(serialize_f64(f64));

    forward!(serialize_char(char));
    forward!(serialize_str(&str));
    forward!(serialize_bytes(&[u8]));

    fn serialize_none(self) -> Result<Self::Ok, DeError> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, DeError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, DeError> {
        self.ser("`()`")?.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, DeError> {
        self.ser_name(name)?.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, DeError> {
        self.ser_name(name)?
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, DeError> {
        self.ser_name(name)?.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, DeError> {
        self.ser_name(name)?
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, DeError> {
        self.ser("sequence")?.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, DeError> {
        self.ser("unnamed tuple")?.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, DeError> {
        self.ser_name(name)?.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, DeError> {
        self.ser_name(name)?
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, DeError> {
        self.ser("map")?.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, DeError> {
        self.ser_name(name)?.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, DeError> {
        self.ser_name(name)?
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}
