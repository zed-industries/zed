//! An abstraction of a plist file as a stream of events. Used to support multiple encodings.

mod binary_reader;
pub use self::binary_reader::BinaryReader;

mod binary_writer;
pub use self::binary_writer::BinaryWriter;

mod xml_reader;
pub use self::xml_reader::XmlReader;

mod xml_writer;
pub use self::xml_writer::XmlWriter;
#[cfg(feature = "serde")]
pub(crate) use xml_writer::encode_data_base64 as xml_encode_data_base64;

mod ascii_reader;
pub use self::ascii_reader::AsciiReader;

use std::{
    borrow::Cow,
    io::{self, BufReader, Read, Seek},
    vec,
};

use crate::{
    dictionary,
    error::{Error, ErrorKind},
    Date, Integer, Uid, Value,
};

/// An encoding of a plist as a flat structure.
///
/// Output by the event readers.
///
/// Dictionary keys and values are represented as pairs of values e.g.:
///
/// ```ignore rust
/// StartDictionary
/// String("Height") // Key
/// Real(181.2)      // Value
/// String("Age")    // Key
/// Integer(28)      // Value
/// EndDictionary
/// ```
///
/// ## Lifetimes
///
/// This type has a lifetime parameter; during serialization, data is borrowed
/// from a [`Value`], and the lifetime of the event is the lifetime of the
/// [`Value`] being serialized.
///
/// During deserialization, data is always copied anyway, and this lifetime
/// is always `'static`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Event<'a> {
    // While the length of an array or dict cannot be feasably greater than max(usize) this better
    // conveys the concept of an effectively unbounded event stream.
    StartArray(Option<u64>),
    StartDictionary(Option<u64>),
    EndCollection,

    Boolean(bool),
    Data(Cow<'a, [u8]>),
    Date(Date),
    Integer(Integer),
    Real(f64),
    String(Cow<'a, str>),
    Uid(Uid),
}

/// An owned [`Event`].
///
/// During deserialization, events are always owned; this type alias helps
/// keep that code a bit clearer.
pub type OwnedEvent = Event<'static>;

/// An `Event` stream returned by `Value::into_events`.
pub struct Events<'a> {
    stack: Vec<StackItem<'a>>,
}

enum StackItem<'a> {
    Root(&'a Value),
    Array(std::slice::Iter<'a, Value>),
    Dict(dictionary::Iter<'a>),
    DictValue(&'a Value),
}

/// Options for customizing serialization of XML plists.
#[derive(Clone, Debug)]
pub struct XmlWriteOptions {
    root_element: bool,
    indent_char: u8,
    indent_count: usize,
}

impl XmlWriteOptions {
    /// Specify the sequence of characters used for indentation.
    ///
    /// This may be either an `&'static str` or an owned `String`.
    ///
    /// The default is `\t`.
    ///
    /// Since replacing `xml-rs` with `quick-xml`, the indent string has to consist of a single
    /// repeating ascii character. This is a backwards compatibility function, prefer using
    /// [`XmlWriteOptions::indent`].
    #[deprecated(since = "1.4.0", note = "please use `indent` instead")]
    pub fn indent_string(self, indent_str: impl Into<Cow<'static, str>>) -> Self {
        let indent_str = indent_str.into();
        let indent_str = indent_str.as_ref();

        if indent_str.is_empty() {
            return self.indent(0, 0);
        }

        assert!(
            indent_str.is_ascii(),
            "indent str must be ascii"
        );
        let indent_str = indent_str.as_bytes();
        assert!(
            indent_str.iter().all(|chr| chr == &indent_str[0]),
            "indent str must consist of a single repeating character"
        );

        self.indent(indent_str[0], indent_str.len())
    }

    /// Specifies the character and amount used for indentation.
    ///
    /// `indent_char` must be a valid UTF8 character.
    ///
    /// The default is indenting with a single tab.
    pub fn indent(mut self, indent_char: u8, indent_count: usize) -> Self {
        self.indent_char = indent_char;
        self.indent_count = indent_count;
        self
    }

    /// Selects whether to write the XML prologue, plist document type and root element.
    ///
    /// In other words the following:
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    /// <plist version="1.0">
    /// ...
    /// </plist>
    /// ```
    ///
    /// The default is `true`.
    pub fn root_element(mut self, write_root: bool) -> Self {
        self.root_element = write_root;
        self
    }
}

impl Default for XmlWriteOptions {
    fn default() -> Self {
        XmlWriteOptions {
            indent_char: b'\t',
            indent_count: 1,
            root_element: true,
        }
    }
}

impl<'a> Events<'a> {
    pub(crate) fn new(value: &'a Value) -> Events<'a> {
        Events {
            stack: vec![StackItem::Root(value)],
        }
    }
}

impl<'a> Iterator for Events<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        fn handle_value<'c, 'b: 'c>(
            value: &'b Value,
            stack: &'c mut Vec<StackItem<'b>>,
        ) -> Event<'b> {
            match value {
                Value::Array(array) => {
                    let len = array.len();
                    let iter = array.iter();
                    stack.push(StackItem::Array(iter));
                    Event::StartArray(Some(len as u64))
                }
                Value::Dictionary(dict) => {
                    let len = dict.len();
                    let iter = dict.into_iter();
                    stack.push(StackItem::Dict(iter));
                    Event::StartDictionary(Some(len as u64))
                }
                Value::Boolean(value) => Event::Boolean(*value),
                Value::Data(value) => Event::Data(Cow::Borrowed(value)),
                Value::Date(value) => Event::Date(*value),
                Value::Real(value) => Event::Real(*value),
                Value::Integer(value) => Event::Integer(*value),
                Value::String(value) => Event::String(Cow::Borrowed(value.as_str())),
                Value::Uid(value) => Event::Uid(*value),
            }
        }

        Some(match self.stack.pop()? {
            StackItem::Root(value)
            | StackItem::DictValue(value) => handle_value(value, &mut self.stack),
            StackItem::Array(mut array) => {
                if let Some(value) = array.next() {
                    // There might still be more items in the array so return it to the stack.
                    self.stack.push(StackItem::Array(array));
                    handle_value(value, &mut self.stack)
                } else {
                    Event::EndCollection
                }
            }
            StackItem::Dict(mut dict) => {
                if let Some((key, value)) = dict.next() {
                    // There might still be more items in the dictionary so return it to the stack.
                    self.stack.push(StackItem::Dict(dict));
                    // The next event to be returned must be the dictionary value.
                    self.stack.push(StackItem::DictValue(value));
                    // Return the key event now.
                    Event::String(Cow::Borrowed(key))
                } else {
                    Event::EndCollection
                }
            }
        })
    }
}

pub struct Reader<R: Read + Seek>(ReaderInner<R>);

enum ReaderInner<R: Read + Seek> {
    Uninitialized(Option<R>),
    Binary(BinaryReader<R>),
    Xml(XmlReader<BufReader<R>>),
    Ascii(AsciiReader<BufReader<R>>),
}

impl<R: Read + Seek> Reader<R> {
    pub fn new(reader: R) -> Reader<R> {
        Reader(ReaderInner::Uninitialized(Some(reader)))
    }

    fn init(&mut self, mut reader: R) -> Result<Option<OwnedEvent>, Error> {
        // Rewind reader back to the start.
        if let Err(err) = reader.rewind().map_err(from_io_offset_0) {
            self.0 = ReaderInner::Uninitialized(Some(reader));
            return Err(err);
        }

        // A plist is binary if it starts with magic bytes.
        match Reader::is_binary(&mut reader) {
            Ok(true) => {
                self.0 = ReaderInner::Binary(BinaryReader::new(reader));
                return self.next().transpose();
            }
            Ok(false) => (),
            Err(err) => {
                self.0 = ReaderInner::Uninitialized(Some(reader));
                return Err(err);
            }
        }

        // If a plist is not binary, try to parse as XML.
        // Use a `BufReader` for XML and ASCII plists as it is required by `quick-xml` and will
        // definitely speed up ASCII parsing as well.
        let mut xml_reader = XmlReader::new(BufReader::new(reader));
        let mut reader = match xml_reader.next() {
            res @ (Some(Ok(_)) | None) => {
                self.0 = ReaderInner::Xml(xml_reader);
                return res.transpose();
            }
            Some(Err(err)) if xml_reader.xml_doc_started() => {
                self.0 = ReaderInner::Uninitialized(Some(xml_reader.into_inner().into_inner()));
                return Err(err);
            }
            Some(Err(_)) => xml_reader.into_inner(),
        };

        // Rewind reader back to the start.
        if let Err(err) = reader.rewind().map_err(from_io_offset_0) {
            self.0 = ReaderInner::Uninitialized(Some(reader.into_inner()));
            return Err(err);
        }

        // If no valid XML markup is found, try to parse as ASCII.
        let mut ascii_reader = AsciiReader::new(reader);
        match ascii_reader.next() {
            res @ (Some(Ok(_)) | None) => {
                self.0 = ReaderInner::Ascii(ascii_reader);
                res.transpose()
            }
            Some(Err(err)) => {
                self.0 = ReaderInner::Uninitialized(Some(ascii_reader.into_inner().into_inner()));
                Err(err)
            }
        }
    }

    fn is_binary(reader: &mut R) -> Result<bool, Error> {
        let mut magic = [0; 8];
        reader.read_exact(&mut magic).map_err(from_io_offset_0)?;
        reader.rewind().map_err(from_io_offset_0)?;

        Ok(&magic == b"bplist00")
    }
}

impl<R: Read + Seek> Iterator for Reader<R> {
    type Item = Result<OwnedEvent, Error>;

    fn next(&mut self) -> Option<Result<OwnedEvent, Error>> {
        match self.0 {
            ReaderInner::Xml(ref mut parser) => parser.next(),
            ReaderInner::Binary(ref mut parser) => parser.next(),
            ReaderInner::Ascii(ref mut parser) => parser.next(),
            ReaderInner::Uninitialized(ref mut reader) => {
                let reader = reader.take().unwrap();
                self.init(reader).transpose()
            }
        }
    }
}

fn from_io_offset_0(err: io::Error) -> Error {
    ErrorKind::Io(err).with_byte_offset(0)
}

/// Supports writing event streams in different plist encodings.
pub trait Writer: private::Sealed {
    fn write(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::StartArray(len) => self.write_start_array(len),
            Event::StartDictionary(len) => self.write_start_dictionary(len),
            Event::EndCollection => self.write_end_collection(),
            Event::Boolean(value) => self.write_boolean(value),
            Event::Data(value) => self.write_data(value),
            Event::Date(value) => self.write_date(value),
            Event::Integer(value) => self.write_integer(value),
            Event::Real(value) => self.write_real(value),
            Event::String(value) => self.write_string(value),
            Event::Uid(value) => self.write_uid(value),
        }
    }

    fn write_start_array(&mut self, len: Option<u64>) -> Result<(), Error>;
    fn write_start_dictionary(&mut self, len: Option<u64>) -> Result<(), Error>;
    fn write_end_collection(&mut self) -> Result<(), Error>;

    fn write_boolean(&mut self, value: bool) -> Result<(), Error>;
    fn write_data(&mut self, value: Cow<[u8]>) -> Result<(), Error>;
    fn write_date(&mut self, value: Date) -> Result<(), Error>;
    fn write_integer(&mut self, value: Integer) -> Result<(), Error>;
    fn write_real(&mut self, value: f64) -> Result<(), Error>;
    fn write_string(&mut self, value: Cow<str>) -> Result<(), Error>;
    fn write_uid(&mut self, value: Uid) -> Result<(), Error>;
}

pub(crate) mod private {
    use std::io::Write;

    pub trait Sealed {}

    impl<W: Write> Sealed for super::BinaryWriter<W> {}
    impl<W: Write> Sealed for super::XmlWriter<W> {}
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::{Event::*, *};

    const ANIMALS_PLIST_EVENTS: &[Event] = &[
        StartDictionary(None),
        String(Cow::Borrowed("AnimalColors")),
        StartDictionary(None),
        String(Cow::Borrowed("lamb")), // key
        String(Cow::Borrowed("black")),
        String(Cow::Borrowed("pig")), // key
        String(Cow::Borrowed("pink")),
        String(Cow::Borrowed("worm")), // key
        String(Cow::Borrowed("pink")),
        EndCollection,
        String(Cow::Borrowed("AnimalSmells")),
        StartDictionary(None),
        String(Cow::Borrowed("lamb")), // key
        String(Cow::Borrowed("lambish")),
        String(Cow::Borrowed("pig")), // key
        String(Cow::Borrowed("piggish")),
        String(Cow::Borrowed("worm")), // key
        String(Cow::Borrowed("wormy")),
        EndCollection,
        String(Cow::Borrowed("AnimalSounds")),
        StartDictionary(None),
        String(Cow::Borrowed("Lisa")), // key
        String(Cow::Borrowed("Why is the worm talking like a lamb?")),
        String(Cow::Borrowed("lamb")), // key
        String(Cow::Borrowed("baa")),
        String(Cow::Borrowed("pig")), // key
        String(Cow::Borrowed("oink")),
        String(Cow::Borrowed("worm")), // key
        String(Cow::Borrowed("baa")),
        EndCollection,
        EndCollection,
    ];

    #[test]
    fn autodetect_binary() {
        let reader = File::open("./tests/data/binary.plist").unwrap();
        let mut streaming_parser = Reader::new(reader);
        let events: Result<Vec<_>, _> = streaming_parser.by_ref().collect();

        assert!(matches!(streaming_parser.0, ReaderInner::Binary(_)));
        // The contents of this plist are tested for elsewhere.
        assert!(events.is_ok());
    }

    #[test]
    fn autodetect_xml() {
        let reader = File::open("./tests/data/xml-animals.plist").unwrap();
        let mut streaming_parser = Reader::new(reader);
        let events: Result<Vec<_>, _> = streaming_parser.by_ref().collect();

        assert!(matches!(streaming_parser.0, ReaderInner::Xml(_)));
        assert_eq!(events.unwrap(), ANIMALS_PLIST_EVENTS);
    }

    #[test]
    fn autodetect_ascii() {
        let reader = File::open("./tests/data/ascii-animals.plist").unwrap();
        let mut streaming_parser = Reader::new(reader);
        let events: Result<Vec<_>, _> = streaming_parser.by_ref().collect();

        assert!(matches!(streaming_parser.0, ReaderInner::Ascii(_)));
        assert_eq!(events.unwrap(), ANIMALS_PLIST_EVENTS);
    }
}
