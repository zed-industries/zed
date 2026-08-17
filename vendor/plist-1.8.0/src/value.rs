use std::{
    borrow::Cow,
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, Write},
    path::Path,
};

use crate::{
    error::{self, Error, ErrorKind, EventKind},
    stream::{
        private, AsciiReader, BinaryWriter, Event, Events, Reader, Writer, XmlReader,
        XmlWriteOptions, XmlWriter,
    },
    u64_to_usize, Date, Dictionary, Integer, Uid,
};

/// Represents any plist value.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Value {
    Array(Vec<Value>),
    Dictionary(Dictionary),
    Boolean(bool),
    Data(Vec<u8>),
    Date(Date),
    Real(f64),
    Integer(Integer),
    String(String),
    Uid(Uid),
}

impl Value {
    /// Reads a `Value` from a plist file of any encoding.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
        let file = File::open(path).map_err(error::from_io_without_position)?;
        Value::from_reader(BufReader::new(file))
    }

    /// Reads a `Value` from a seekable byte stream containing a plist of any encoding.
    pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Value, Error> {
        let reader = Reader::new(reader);
        Value::from_events(reader)
    }

    /// Reads a `Value` from a byte stream containing an ASCII encoded plist.
    pub fn from_reader_ascii<R: Read>(reader: R) -> Result<Value, Error> {
        let reader = AsciiReader::new(reader);
        Value::from_events(reader)
    }

    /// Reads a `Value` from a byte stream containing an XML encoded plist.
    pub fn from_reader_xml<R: Read>(reader: R) -> Result<Value, Error> {
        let reader = XmlReader::new(BufReader::new(reader));
        Value::from_events(reader)
    }

    /// Serializes a `Value` to a file as a binary encoded plist.
    pub fn to_file_binary<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = File::create(path).map_err(error::from_io_without_position)?;
        self.to_writer_binary(BufWriter::new(&mut file))?;
        file.sync_all().map_err(error::from_io_without_position)?;
        Ok(())
    }

    /// Serializes a `Value` to a file as an XML encoded plist.
    pub fn to_file_xml<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = File::create(path).map_err(error::from_io_without_position)?;
        self.to_writer_xml(BufWriter::new(&mut file))?;
        file.sync_all().map_err(error::from_io_without_position)?;
        Ok(())
    }

    /// Serializes a `Value` to a byte stream as a binary encoded plist.
    pub fn to_writer_binary<W: Write>(&self, writer: W) -> Result<(), Error> {
        let mut writer = BinaryWriter::new(writer);
        self.to_writer_inner(&mut writer)
    }

    /// Serializes a `Value` to a byte stream as an XML encoded plist.
    pub fn to_writer_xml<W: Write>(&self, writer: W) -> Result<(), Error> {
        self.to_writer_xml_with_options(writer, &XmlWriteOptions::default())
    }

    /// Serializes a `Value` to a stream, using custom [`XmlWriteOptions`].
    ///
    /// If you need to serialize to a file, you must acquire an appropriate
    /// `Write` handle yourself.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::{BufWriter, Write};
    /// use std::fs::File;
    /// use plist::{Dictionary, Value, XmlWriteOptions};
    ///
    /// let value: Value = Dictionary::new().into();
    /// // .. add some keys & values
    /// let mut file = File::create("com.example.myPlist.plist").unwrap();
    /// let options = XmlWriteOptions::default().indent_string("  ");
    /// value.to_writer_xml_with_options(BufWriter::new(&mut file), &options).unwrap();
    /// file.sync_all().unwrap();
    /// ```
    pub fn to_writer_xml_with_options<W: Write>(
        &self,
        writer: W,
        options: &XmlWriteOptions,
    ) -> Result<(), Error> {
        let mut writer = XmlWriter::new_with_options(writer, options);
        self.to_writer_inner(&mut writer)
    }

    fn to_writer_inner(&self, writer: &mut dyn Writer) -> Result<(), Error> {
        let events = self.events();
        for event in events {
            writer.write(event)?;
        }
        Ok(())
    }

    /// Builds a single `Value` from an `Event` iterator.
    /// On success any excess `Event`s will remain in the iterator.
    #[cfg(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps")]
    pub fn from_events<'event, T>(events: T) -> Result<Value, Error>
    where
        T: IntoIterator<Item = Result<Event<'event>, Error>>,
    {
        Builder::build(events.into_iter())
    }

    /// Builds a single `Value` from an `Event` iterator.
    /// On success any excess `Event`s will remain in the iterator.
    #[cfg(not(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"))]
    pub(crate) fn from_events<'event, T>(events: T) -> Result<Value, Error>
    where
        T: IntoIterator<Item = Result<Event<'event>, Error>>,
    {
        Builder::build(events.into_iter())
    }

    /// Converts a `Value` into an `Event` iterator.
    #[cfg(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps")]
    #[doc(hidden)]
    #[deprecated(since = "1.2.0", note = "use Value::events instead")]
    pub fn into_events(&self) -> Events<'_> {
        self.events()
    }

    /// Creates an `Event` iterator for this `Value`.
    #[cfg(not(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps"))]
    pub(crate) fn events(&self) -> Events<'_> {
        Events::new(self)
    }

    /// Creates an `Event` iterator for this `Value`.
    #[cfg(feature = "enable_unstable_features_that_may_break_with_minor_version_bumps")]
    pub fn events(&self) -> Events<'_> {
        Events::new(self)
    }

    /// If the `Value` is a Array, returns the underlying `Vec`.
    ///
    /// Returns `None` otherwise.
    ///
    /// This method consumes the `Value`. To get a reference instead, use
    /// `as_array`.
    pub fn into_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(dict) => Some(dict),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated `Vec`.
    ///
    /// Returns `None` otherwise.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(array),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated mutable `Vec`.
    ///
    /// Returns `None` otherwise.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut array) => Some(array),
            _ => None,
        }
    }

    /// If the `Value` is a Dictionary, returns the associated `BTreeMap`.
    ///
    /// Returns `None` otherwise.
    ///
    /// This method consumes the `Value`. To get a reference instead, use
    /// `as_dictionary`.
    pub fn into_dictionary(self) -> Option<Dictionary> {
        match self {
            Value::Dictionary(dict) => Some(dict),
            _ => None,
        }
    }

    /// If the `Value` is a Dictionary, returns the associated `BTreeMap`.
    ///
    /// Returns `None` otherwise.
    pub fn as_dictionary(&self) -> Option<&Dictionary> {
        match *self {
            Value::Dictionary(ref dict) => Some(dict),
            _ => None,
        }
    }

    /// If the `Value` is a Dictionary, returns the associated mutable `BTreeMap`.
    ///
    /// Returns `None` otherwise.
    pub fn as_dictionary_mut(&mut self) -> Option<&mut Dictionary> {
        match *self {
            Value::Dictionary(ref mut dict) => Some(dict),
            _ => None,
        }
    }

    /// If the `Value` is a Boolean, returns the associated `bool`.
    ///
    /// Returns `None` otherwise.
    pub fn as_boolean(&self) -> Option<bool> {
        match *self {
            Value::Boolean(v) => Some(v),
            _ => None,
        }
    }

    /// If the `Value` is a Data, returns the underlying `Vec`.
    ///
    /// Returns `None` otherwise.
    ///
    /// This method consumes the `Value`. If this is not desired, please use
    /// `as_data` method.
    pub fn into_data(self) -> Option<Vec<u8>> {
        match self {
            Value::Data(data) => Some(data),
            _ => None,
        }
    }

    /// If the `Value` is a Data, returns the associated `Vec`.
    ///
    /// Returns `None` otherwise.
    pub fn as_data(&self) -> Option<&[u8]> {
        match *self {
            Value::Data(ref data) => Some(data),
            _ => None,
        }
    }

    /// If the `Value` is a Date, returns the associated `Date`.
    ///
    /// Returns `None` otherwise.
    pub fn as_date(&self) -> Option<Date> {
        match *self {
            Value::Date(date) => Some(date),
            _ => None,
        }
    }

    /// If the `Value` is a Real, returns the associated `f64`.
    ///
    /// Returns `None` otherwise.
    pub fn as_real(&self) -> Option<f64> {
        match *self {
            Value::Real(v) => Some(v),
            _ => None,
        }
    }

    /// If the `Value` is a signed Integer, returns the associated `i64`.
    ///
    /// Returns `None` otherwise.
    pub fn as_signed_integer(&self) -> Option<i64> {
        match *self {
            Value::Integer(v) => v.as_signed(),
            _ => None,
        }
    }

    /// If the `Value` is an unsigned Integer, returns the associated `u64`.
    ///
    /// Returns `None` otherwise.
    pub fn as_unsigned_integer(&self) -> Option<u64> {
        match *self {
            Value::Integer(v) => v.as_unsigned(),
            _ => None,
        }
    }

    /// If the `Value` is a String, returns the underlying `String`.
    ///
    /// Returns `None` otherwise.
    ///
    /// This method consumes the `Value`. If this is not desired, please use
    /// `as_string` method.
    pub fn into_string(self) -> Option<String> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    /// If the `Value` is a String, returns the associated `str`.
    ///
    /// Returns `None` otherwise.
    pub fn as_string(&self) -> Option<&str> {
        match *self {
            Value::String(ref v) => Some(v),
            _ => None,
        }
    }

    /// If the `Value` is a Uid, returns the underlying `Uid`.
    ///
    /// Returns `None` otherwise.
    ///
    /// This method consumes the `Value`. If this is not desired, please use
    /// `as_uid` method.
    pub fn into_uid(self) -> Option<Uid> {
        match self {
            Value::Uid(u) => Some(u),
            _ => None,
        }
    }

    /// If the `Value` is a Uid, returns the associated `Uid`.
    ///
    /// Returns `None` otherwise.
    pub fn as_uid(&self) -> Option<&Uid> {
        match *self {
            Value::Uid(ref u) => Some(u),
            _ => None,
        }
    }
}

#[cfg(feature = "serde")]
pub mod serde_impls {
    use serde::{
        de,
        de::{EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
        ser,
    };

    use crate::{
        date::serde_impls::DATE_NEWTYPE_STRUCT_NAME, uid::serde_impls::UID_NEWTYPE_STRUCT_NAME,
        Dictionary, Value,
    };

    pub const VALUE_NEWTYPE_STRUCT_NAME: &str = "PLIST-VALUE";

    impl ser::Serialize for Value {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                Value::Array(ref v) => v.serialize(serializer),
                Value::Dictionary(ref m) => m.serialize(serializer),
                Value::Boolean(b) => serializer.serialize_bool(b),
                Value::Data(ref v) => serializer.serialize_bytes(v),
                Value::Date(d) => d.serialize(serializer),
                Value::Real(n) => serializer.serialize_f64(n),
                Value::Integer(n) => n.serialize(serializer),
                Value::String(ref s) => serializer.serialize_str(s),
                Value::Uid(ref u) => u.serialize(serializer),
            }
        }
    }

    impl<'de> de::Deserialize<'de> for Value {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct ValueVisitor;

            impl<'de> Visitor<'de> for ValueVisitor {
                type Value = Value;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("any supported plist value")
                }

                fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                    Ok(Value::Boolean(value))
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Value, E> {
                    Ok(Value::Data(v))
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<Value, E> {
                    Ok(Value::Data(v.to_vec()))
                }

                fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                    Ok(Value::Integer(value.into()))
                }

                fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                    Ok(Value::Integer(value.into()))
                }

                fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                    Ok(Value::Real(value))
                }

                fn visit_map<V>(self, mut map: V) -> Result<Value, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut values = Dictionary::new();
                    while let Some((k, v)) = map.next_entry()? {
                        values.insert(k, v);
                    }
                    Ok(Value::Dictionary(values))
                }

                fn visit_str<E>(self, value: &str) -> Result<Value, E> {
                    Ok(Value::String(value.to_owned()))
                }

                fn visit_string<E>(self, value: String) -> Result<Value, E> {
                    Ok(Value::String(value))
                }

                fn visit_newtype_struct<T>(self, deserializer: T) -> Result<Value, T::Error>
                where
                    T: de::Deserializer<'de>,
                {
                    deserializer.deserialize_any(self)
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                    while let Some(elem) = seq.next_element()? {
                        vec.push(elem);
                    }
                    Ok(Value::Array(vec))
                }

                fn visit_enum<A>(self, data: A) -> Result<Value, A::Error>
                where
                    A: EnumAccess<'de>,
                {
                    let (name, variant) = data.variant::<String>()?;
                    match &*name {
                        DATE_NEWTYPE_STRUCT_NAME => Ok(Value::Date(variant.newtype_variant()?)),
                        UID_NEWTYPE_STRUCT_NAME => Ok(Value::Uid(variant.newtype_variant()?)),
                        _ => Err(de::Error::unknown_variant(
                            &name,
                            &[DATE_NEWTYPE_STRUCT_NAME, UID_NEWTYPE_STRUCT_NAME],
                        )),
                    }
                }
            }

            // Serde serialisers are encouraged to treat newtype structs as insignificant
            // wrappers around the data they contain. That means not parsing anything other
            // than the contained value. Therefore, this should not prevent using `Value`
            // with other `Serializer`s.
            deserializer.deserialize_newtype_struct(VALUE_NEWTYPE_STRUCT_NAME, ValueVisitor)
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(from: Vec<Value>) -> Value {
        Value::Array(from)
    }
}

impl From<Dictionary> for Value {
    fn from(from: Dictionary) -> Value {
        Value::Dictionary(from)
    }
}

impl From<bool> for Value {
    fn from(from: bool) -> Value {
        Value::Boolean(from)
    }
}

impl<'a> From<&'a bool> for Value {
    fn from(from: &'a bool) -> Value {
        Value::Boolean(*from)
    }
}

impl From<Date> for Value {
    fn from(from: Date) -> Value {
        Value::Date(from)
    }
}

impl<'a> From<&'a Date> for Value {
    fn from(from: &'a Date) -> Value {
        Value::Date(*from)
    }
}

impl From<f64> for Value {
    fn from(from: f64) -> Value {
        Value::Real(from)
    }
}

impl From<f32> for Value {
    fn from(from: f32) -> Value {
        Value::Real(from.into())
    }
}

impl From<i64> for Value {
    fn from(from: i64) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<i32> for Value {
    fn from(from: i32) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<i16> for Value {
    fn from(from: i16) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<i8> for Value {
    fn from(from: i8) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<u64> for Value {
    fn from(from: u64) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<u32> for Value {
    fn from(from: u32) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<u16> for Value {
    fn from(from: u16) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl From<u8> for Value {
    fn from(from: u8) -> Value {
        Value::Integer(Integer::from(from))
    }
}

impl<'a> From<&'a f64> for Value {
    fn from(from: &'a f64) -> Value {
        Value::Real(*from)
    }
}

impl<'a> From<&'a f32> for Value {
    fn from(from: &'a f32) -> Value {
        Value::Real((*from).into())
    }
}

impl<'a> From<&'a i64> for Value {
    fn from(from: &'a i64) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a i32> for Value {
    fn from(from: &'a i32) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a i16> for Value {
    fn from(from: &'a i16) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a i8> for Value {
    fn from(from: &'a i8) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a u64> for Value {
    fn from(from: &'a u64) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a u32> for Value {
    fn from(from: &'a u32) -> Value {
        Value::Integer(Integer::from(*from))
    }
}

impl<'a> From<&'a u16> for Value {
    fn from(from: &'a u16) -> Value {
        Value::Integer((*from).into())
    }
}

impl<'a> From<&'a u8> for Value {
    fn from(from: &'a u8) -> Value {
        Value::Integer((*from).into())
    }
}

impl From<String> for Value {
    fn from(from: String) -> Value {
        Value::String(from)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(from: &'a str) -> Value {
        Value::String(from.into())
    }
}

enum StackItem {
    Root(Value),
    Array(Vec<Value>),
    Dict(Dictionary),
    DictAndKey(Dictionary, String),
}

#[derive(Default)]
pub struct Builder {
    stack: Vec<StackItem>,
}

impl Builder {
    fn build<'event, T>(stream: T) -> Result<Value, Error>
    where
        T: Iterator<Item = Result<Event<'event>, Error>>,
    {
        let mut builder = Self::default();
        for event in stream {
            builder.write(event?)?;
        }
        builder.finish()
    }

    fn write_value(&mut self, value: Value) -> Result<(), Error> {
        match (self.stack.pop(), value) {
            (None, value) => self.stack.push(StackItem::Root(value)),
            (Some(StackItem::Root(_)), value) => {
                return Err(ErrorKind::ExpectedEndOfEventStream {
                    found: EventKind::of_value(&value),
                }
                .without_position())
            }
            (Some(StackItem::Array(mut array)), value) => {
                array.push(value);
                self.stack.push(StackItem::Array(array));
            }
            (Some(StackItem::Dict(dict)), Value::String(key)) => {
                self.stack.push(StackItem::DictAndKey(dict, key));
            }
            (Some(StackItem::Dict(_)), value) => {
                return Err(ErrorKind::UnexpectedEventType {
                    expected: EventKind::DictionaryKeyOrEndCollection,
                    found: EventKind::of_value(&value),
                }
                .without_position())
            }
            (Some(StackItem::DictAndKey(mut dict, key)), value) => {
                dict.insert(key, value);
                self.stack.push(StackItem::Dict(dict));
            }
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<Value, Error> {
        match self.stack.pop() {
            Some(StackItem::Root(value)) => Ok(value),
            _ => Err(ErrorKind::UnexpectedEndOfEventStream.without_position()),
        }
    }
}

impl Writer for Builder {
    fn write_start_array(&mut self, len: Option<u64>) -> Result<(), Error> {
        let len = len.and_then(u64_to_usize).unwrap_or(0);
        self.stack.push(StackItem::Array(Vec::with_capacity(len)));
        Ok(())
    }

    fn write_start_dictionary(&mut self, _: Option<u64>) -> Result<(), Error> {
        self.stack.push(StackItem::Dict(Dictionary::new()));
        Ok(())
    }

    fn write_end_collection(&mut self) -> Result<(), Error> {
        let value = match self.stack.pop() {
            Some(StackItem::Root(_)) => {
                return Err(ErrorKind::ExpectedEndOfEventStream {
                    found: EventKind::EndCollection,
                }
                .without_position())
            }
            Some(StackItem::Array(array)) => Value::Array(array),
            Some(StackItem::Dict(dict)) => Value::Dictionary(dict),
            Some(StackItem::DictAndKey(_, _)) | None => {
                return Err(ErrorKind::UnexpectedEventType {
                    expected: EventKind::ValueOrStartCollection,
                    found: EventKind::EndCollection,
                }
                .without_position())
            }
        };
        self.write_value(value)
    }

    fn write_boolean(&mut self, value: bool) -> Result<(), Error> {
        self.write_value(Value::Boolean(value))
    }

    fn write_data(&mut self, value: Cow<[u8]>) -> Result<(), Error> {
        self.write_value(Value::Data(value.into_owned()))
    }

    fn write_date(&mut self, value: Date) -> Result<(), Error> {
        self.write_value(Value::Date(value))
    }

    fn write_integer(&mut self, value: Integer) -> Result<(), Error> {
        self.write_value(Value::Integer(value))
    }

    fn write_real(&mut self, value: f64) -> Result<(), Error> {
        self.write_value(Value::Real(value))
    }

    fn write_string(&mut self, value: Cow<str>) -> Result<(), Error> {
        self.write_value(Value::String(value.into_owned()))
    }

    fn write_uid(&mut self, value: Uid) -> Result<(), Error> {
        self.write_value(Value::Uid(value))
    }
}

impl private::Sealed for Builder {}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use crate::{stream::Event::*, Date};

    #[test]
    fn value_accessors() {
        let vec = vec![Value::Real(0.0)];
        let mut array = Value::Array(vec.clone());
        assert_eq!(array.as_array(), Some(&vec.clone()));
        assert_eq!(array.as_array_mut(), Some(&mut vec.clone()));

        let mut map = Dictionary::new();
        map.insert("key1".to_owned(), Value::String("value1".to_owned()));
        let mut dict = Value::Dictionary(map.clone());
        assert_eq!(dict.as_dictionary(), Some(&map.clone()));
        assert_eq!(dict.as_dictionary_mut(), Some(&mut map.clone()));

        assert_eq!(Value::Boolean(true).as_boolean(), Some(true));

        let slice: &[u8] = &[1, 2, 3];
        assert_eq!(Value::Data(slice.to_vec()).as_data(), Some(slice));
        assert_eq!(
            Value::Data(slice.to_vec()).into_data(),
            Some(slice.to_vec())
        );

        let date: Date = SystemTime::now().into();
        assert_eq!(Value::Date(date).as_date(), Some(date));

        assert_eq!(Value::Real(0.0).as_real(), Some(0.0));
        assert_eq!(Value::Integer(1.into()).as_signed_integer(), Some(1));
        assert_eq!(Value::Integer(1.into()).as_unsigned_integer(), Some(1));
        assert_eq!(Value::Integer((-1).into()).as_unsigned_integer(), None);
        assert_eq!(
            Value::Integer((i64::MAX as u64 + 1).into()).as_signed_integer(),
            None
        );
        assert_eq!(Value::String("2".to_owned()).as_string(), Some("2"));
        assert_eq!(
            Value::String("t".to_owned()).into_string(),
            Some("t".to_owned())
        );
    }

    #[test]
    fn builder() {
        // Input
        let events = vec![
            StartDictionary(None),
            String("Author".into()),
            String("William Shakespeare".into()),
            String("Lines".into()),
            StartArray(None),
            String("It is a tale told by an idiot,".into()),
            String("Full of sound and fury, signifying nothing.".into()),
            EndCollection,
            String("Birthdate".into()),
            Integer(1564.into()),
            String("Height".into()),
            Real(1.60),
            EndCollection,
        ];

        let value = Builder::build(events.into_iter().map(Ok));

        // Expected output
        let lines = vec![
            Value::String("It is a tale told by an idiot,".to_owned()),
            Value::String("Full of sound and fury, signifying nothing.".to_owned()),
        ];

        let mut dict = Dictionary::new();
        dict.insert(
            "Author".to_owned(),
            Value::String("William Shakespeare".to_owned()),
        );
        dict.insert("Lines".to_owned(), Value::Array(lines));
        dict.insert("Birthdate".to_owned(), Value::Integer(1564.into()));
        dict.insert("Height".to_owned(), Value::Real(1.60));

        assert_eq!(value.unwrap(), Value::Dictionary(dict));
    }

    #[test]
    fn builder_fails_if_all_events_have_not_been_read() {
        let events = vec![String("Item 1".into()), String("Item 2".into())];

        let value = Builder::build(events.into_iter().map(Ok));

        assert!(value.is_err());
    }
}
