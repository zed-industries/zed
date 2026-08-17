//! YAML objects manipulation utilities.

#![allow(clippy::module_name_repetitions)]

use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{collections::BTreeMap, convert::TryFrom, mem, ops::Index, ops::IndexMut};

#[cfg(feature = "encoding")]
use encoding_rs::{Decoder, DecoderResult, Encoding};
use hashlink::LinkedHashMap;

use crate::parser::{Event, MarkedEventReceiver, Parser, Tag};
use crate::scanner::{Marker, ScanError, TScalarStyle};

/// A YAML node is stored as this `Yaml` enumeration, which provides an easy way to
/// access your YAML document.
///
/// # Examples
///
/// ```
/// use yaml_rust2::Yaml;
/// let foo = Yaml::from_str("-123"); // convert the string to the appropriate YAML type
/// assert_eq!(foo.as_i64().unwrap(), -123);
///
/// // iterate over an Array
/// let vec = Yaml::Array(vec![Yaml::Integer(1), Yaml::Integer(2)]);
/// for v in vec.as_vec().unwrap() {
///     assert!(v.as_i64().is_some());
/// }
/// ```
#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord, Hash)]
pub enum Yaml {
    /// Float types are stored as String and parsed on demand.
    /// Note that `f64` does NOT implement Eq trait and can NOT be stored in `BTreeMap`.
    Real(String),
    /// YAML int is stored as i64.
    Integer(i64),
    /// YAML scalar.
    String(String),
    /// YAML bool, e.g. `true` or `false`.
    Boolean(bool),
    /// YAML array, can be accessed as a `Vec`.
    Array(Array),
    /// YAML hash, can be accessed as a `LinkedHashMap`.
    ///
    /// Insertion order will match the order of insertion into the map.
    Hash(Hash),
    /// Alias, not fully supported yet.
    Alias(usize),
    /// YAML null, e.g. `null` or `~`.
    Null,
    /// Accessing a nonexistent node via the Index trait returns `BadValue`. This
    /// simplifies error handling in the calling code. Invalid type conversion also
    /// returns `BadValue`.
    BadValue,
}

/// The type contained in the `Yaml::Array` variant. This corresponds to YAML sequences.
pub type Array = Vec<Yaml>;
/// The type contained in the `Yaml::Hash` variant. This corresponds to YAML mappings.
pub type Hash = LinkedHashMap<Yaml, Yaml>;

// parse f64 as Core schema
// See: https://github.com/chyh1990/yaml-rust/issues/51
fn parse_f64(v: &str) -> Option<f64> {
    match v {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some(f64::INFINITY),
        "-.inf" | "-.Inf" | "-.INF" => Some(f64::NEG_INFINITY),
        ".nan" | "NaN" | ".NAN" => Some(f64::NAN),
        _ => v.parse::<f64>().ok(),
    }
}

/// Main structure for quickly parsing YAML.
///
/// See [`YamlLoader::load_from_str`].
#[derive(Default)]
pub struct YamlLoader {
    /// The different YAML documents that are loaded.
    docs: Vec<Yaml>,
    // states
    // (current node, anchor_id) tuple
    doc_stack: Vec<(Yaml, usize)>,
    key_stack: Vec<Yaml>,
    anchor_map: BTreeMap<usize, Yaml>,
    /// An error, if one was encountered.
    error: Option<ScanError>,
}

impl MarkedEventReceiver for YamlLoader {
    fn on_event(&mut self, ev: Event, mark: Marker) {
        if self.error.is_some() {
            return;
        }
        if let Err(e) = self.on_event_impl(ev, mark) {
            self.error = Some(e);
        }
    }
}

/// An error that happened when loading a YAML document.
#[derive(Debug)]
pub enum LoadError {
    /// An I/O error.
    IO(std::io::Error),
    /// An error within the scanner. This indicates a malformed YAML input.
    Scan(ScanError),
    /// A decoding error (e.g.: Invalid UTF-8).
    Decode(std::borrow::Cow<'static, str>),
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        LoadError::IO(error)
    }
}

impl YamlLoader {
    fn on_event_impl(&mut self, ev: Event, mark: Marker) -> Result<(), ScanError> {
        // println!("EV {:?}", ev);
        match ev {
            Event::DocumentStart | Event::Nothing | Event::StreamStart | Event::StreamEnd => {
                // do nothing
            }
            Event::DocumentEnd => {
                match self.doc_stack.len() {
                    // empty document
                    0 => self.docs.push(Yaml::BadValue),
                    1 => self.docs.push(self.doc_stack.pop().unwrap().0),
                    _ => unreachable!(),
                }
            }
            Event::SequenceStart(aid, _) => {
                self.doc_stack.push((Yaml::Array(Vec::new()), aid));
            }
            Event::SequenceEnd => {
                let node = self.doc_stack.pop().unwrap();
                self.insert_new_node(node, mark)?;
            }
            Event::MappingStart(aid, _) => {
                self.doc_stack.push((Yaml::Hash(Hash::new()), aid));
                self.key_stack.push(Yaml::BadValue);
            }
            Event::MappingEnd => {
                self.key_stack.pop().unwrap();
                let node = self.doc_stack.pop().unwrap();
                self.insert_new_node(node, mark)?;
            }
            Event::Scalar(v, style, aid, tag) => {
                let node = if style != TScalarStyle::Plain {
                    Yaml::String(v)
                } else if let Some(Tag {
                    ref handle,
                    ref suffix,
                }) = tag
                {
                    if handle == "tag:yaml.org,2002:" {
                        match suffix.as_ref() {
                            "bool" => {
                                // "true" or "false"
                                match v.parse::<bool>() {
                                    Err(_) => Yaml::BadValue,
                                    Ok(v) => Yaml::Boolean(v),
                                }
                            }
                            "int" => match v.parse::<i64>() {
                                Err(_) => Yaml::BadValue,
                                Ok(v) => Yaml::Integer(v),
                            },
                            "float" => match parse_f64(&v) {
                                Some(_) => Yaml::Real(v),
                                None => Yaml::BadValue,
                            },
                            "null" => match v.as_ref() {
                                "~" | "null" => Yaml::Null,
                                _ => Yaml::BadValue,
                            },
                            _ => Yaml::String(v),
                        }
                    } else {
                        Yaml::String(v)
                    }
                } else {
                    // Datatype is not specified, or unrecognized
                    Yaml::from_str(&v)
                };

                self.insert_new_node((node, aid), mark)?;
            }
            Event::Alias(id) => {
                let n = match self.anchor_map.get(&id) {
                    Some(v) => v.clone(),
                    None => Yaml::BadValue,
                };
                self.insert_new_node((n, 0), mark)?;
            }
        }
        // println!("DOC {:?}", self.doc_stack);
        Ok(())
    }

    fn insert_new_node(&mut self, node: (Yaml, usize), mark: Marker) -> Result<(), ScanError> {
        // valid anchor id starts from 1
        if node.1 > 0 {
            self.anchor_map.insert(node.1, node.0.clone());
        }
        if self.doc_stack.is_empty() {
            self.doc_stack.push(node);
        } else {
            let parent = self.doc_stack.last_mut().unwrap();
            match *parent {
                (Yaml::Array(ref mut v), _) => v.push(node.0),
                (Yaml::Hash(ref mut h), _) => {
                    let cur_key = self.key_stack.last_mut().unwrap();
                    // current node is a key
                    if cur_key.is_badvalue() {
                        *cur_key = node.0;
                    // current node is a value
                    } else {
                        let mut newkey = Yaml::BadValue;
                        mem::swap(&mut newkey, cur_key);
                        if h.insert(newkey, node.0).is_some() {
                            let inserted_key = h.back().unwrap().0;
                            return Err(ScanError::new_string(
                                mark,
                                format!("{inserted_key:?}: duplicated key in mapping"),
                            ));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    /// Load the given string as a set of YAML documents.
    ///
    /// The `source` is interpreted as YAML documents and is parsed. Parsing succeeds if and only
    /// if all documents are parsed successfully. An error in a latter document prevents the former
    /// from being returned.
    /// # Errors
    /// Returns `ScanError` when loading fails.
    pub fn load_from_str(source: &str) -> Result<Vec<Yaml>, ScanError> {
        Self::load_from_iter(source.chars())
    }

    /// Load the contents of the given iterator as a set of YAML documents.
    ///
    /// The `source` is interpreted as YAML documents and is parsed. Parsing succeeds if and only
    /// if all documents are parsed successfully. An error in a latter document prevents the former
    /// from being returned.
    /// # Errors
    /// Returns `ScanError` when loading fails.
    pub fn load_from_iter<I: Iterator<Item = char>>(source: I) -> Result<Vec<Yaml>, ScanError> {
        let mut parser = Parser::new(source);
        Self::load_from_parser(&mut parser)
    }

    /// Load the contents from the specified Parser as a set of YAML documents.
    ///
    /// Parsing succeeds if and only if all documents are parsed successfully.
    /// An error in a latter document prevents the former from being returned.
    /// # Errors
    /// Returns `ScanError` when loading fails.
    pub fn load_from_parser<I: Iterator<Item = char>>(
        parser: &mut Parser<I>,
    ) -> Result<Vec<Yaml>, ScanError> {
        let mut loader = YamlLoader::default();
        parser.load(&mut loader, true)?;
        if let Some(e) = loader.error {
            Err(e)
        } else {
            Ok(loader.docs)
        }
    }

    /// Return a reference to the parsed Yaml documents.
    #[must_use]
    pub fn documents(&self) -> &[Yaml] {
        &self.docs
    }
}

/// The signature of the function to call when using [`YAMLDecodingTrap::Call`].
///
/// The arguments are as follows:
///  * `malformation_length`: The length of the sequence the decoder failed to decode.
///  * `bytes_read_after_malformation`: The number of lookahead bytes the decoder consumed after
///    the malformation.
///  * `input_at_malformation`: What the input buffer is at the malformation.
///    This is the buffer starting at the malformation. The first `malformation_length` bytes are
///    the problematic sequence. The following `bytes_read_after_malformation` are already stored
///    in the decoder and will not be re-fed.
///  * `output`: The output string.
///
/// The function must modify `output` as it feels is best. For instance, one could recreate the
/// behavior of [`YAMLDecodingTrap::Ignore`] with an empty function, [`YAMLDecodingTrap::Replace`]
/// by pushing a `\u{FFFD}` into `output` and [`YAMLDecodingTrap::Strict`] by returning
/// [`ControlFlow::Break`].
///
/// # Returns
/// The function must return [`ControlFlow::Continue`] if decoding may continue or
/// [`ControlFlow::Break`] if decoding must be aborted. An optional error string may be supplied.
#[cfg(feature = "encoding")]
pub type YAMLDecodingTrapFn = fn(
    malformation_length: u8,
    bytes_read_after_malformation: u8,
    input_at_malformation: &[u8],
    output: &mut String,
) -> ControlFlow<Cow<'static, str>>;

/// The behavior [`YamlDecoder`] must have when an decoding error occurs.
#[cfg(feature = "encoding")]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum YAMLDecodingTrap {
    /// Ignore the offending bytes, remove them from the output.
    Ignore,
    /// Error out.
    Strict,
    /// Replace them with the Unicode REPLACEMENT CHARACTER.
    Replace,
    /// Call the user-supplied function upon decoding malformation.
    Call(YAMLDecodingTrapFn),
}

/// `YamlDecoder` is a `YamlLoader` builder that allows you to supply your own encoding error trap.
/// For example, to read a YAML file while ignoring Unicode decoding errors you can set the
/// `encoding_trap` to `encoding::DecoderTrap::Ignore`.
/// ```rust
/// use yaml_rust2::yaml::{YamlDecoder, YAMLDecodingTrap};
///
/// let string = b"---
/// a\xa9: 1
/// b: 2.2
/// c: [1, 2]
/// ";
/// let out = YamlDecoder::read(string as &[u8])
///     .encoding_trap(YAMLDecodingTrap::Ignore)
///     .decode()
///     .unwrap();
/// ```
#[cfg(feature = "encoding")]
pub struct YamlDecoder<T: std::io::Read> {
    source: T,
    trap: YAMLDecodingTrap,
}

#[cfg(feature = "encoding")]
impl<T: std::io::Read> YamlDecoder<T> {
    /// Create a `YamlDecoder` decoding the given source.
    pub fn read(source: T) -> YamlDecoder<T> {
        YamlDecoder {
            source,
            trap: YAMLDecodingTrap::Strict,
        }
    }

    /// Set the behavior of the decoder when the encoding is invalid.
    pub fn encoding_trap(&mut self, trap: YAMLDecodingTrap) -> &mut Self {
        self.trap = trap;
        self
    }

    /// Run the decode operation with the source and trap the `YamlDecoder` was built with.
    ///
    /// # Errors
    /// Returns `LoadError` when decoding fails.
    pub fn decode(&mut self) -> Result<Vec<Yaml>, LoadError> {
        let mut buffer = Vec::new();
        self.source.read_to_end(&mut buffer)?;

        // Check if the `encoding` library can detect encoding from the BOM, otherwise use
        // `detect_utf16_endianness`.
        let (encoding, _) =
            Encoding::for_bom(&buffer).unwrap_or_else(|| (detect_utf16_endianness(&buffer), 2));
        let mut decoder = encoding.new_decoder();
        let mut output = String::new();

        // Decode the input buffer.
        decode_loop(&buffer, &mut output, &mut decoder, self.trap)?;

        YamlLoader::load_from_str(&output).map_err(LoadError::Scan)
    }
}

/// Perform a loop of [`Decoder::decode_to_string`], reallocating `output` if needed.
#[cfg(feature = "encoding")]
fn decode_loop(
    input: &[u8],
    output: &mut String,
    decoder: &mut Decoder,
    trap: YAMLDecodingTrap,
) -> Result<(), LoadError> {
    output.reserve(input.len());
    let mut total_bytes_read = 0;

    loop {
        match decoder.decode_to_string_without_replacement(&input[total_bytes_read..], output, true)
        {
            // If the input is empty, we processed the whole input.
            (DecoderResult::InputEmpty, _) => break Ok(()),
            // If the output is full, we must reallocate.
            (DecoderResult::OutputFull, bytes_read) => {
                total_bytes_read += bytes_read;
                // The output is already reserved to the size of the input. We slowly resize. Here,
                // we're expecting that 10% of bytes will double in size when converting to UTF-8.
                output.reserve(input.len() / 10);
            }
            (DecoderResult::Malformed(malformed_len, bytes_after_malformed), bytes_read) => {
                total_bytes_read += bytes_read;
                match trap {
                    // Ignore (skip over) malformed character.
                    YAMLDecodingTrap::Ignore => {}
                    // Replace them with the Unicode REPLACEMENT CHARACTER.
                    YAMLDecodingTrap::Replace => {
                        output.push('\u{FFFD}');
                    }
                    // Otherwise error, getting as much context as possible.
                    YAMLDecodingTrap::Strict => {
                        let malformed_len = malformed_len as usize;
                        let bytes_after_malformed = bytes_after_malformed as usize;
                        let byte_idx = total_bytes_read - (malformed_len + bytes_after_malformed);
                        let malformed_sequence = &input[byte_idx..byte_idx + malformed_len];

                        break Err(LoadError::Decode(Cow::Owned(format!(
                            "Invalid character sequence at {byte_idx}: {malformed_sequence:?}",
                        ))));
                    }
                    YAMLDecodingTrap::Call(callback) => {
                        let byte_idx =
                            total_bytes_read - ((malformed_len + bytes_after_malformed) as usize);
                        let malformed_sequence =
                            &input[byte_idx..byte_idx + malformed_len as usize];
                        if let ControlFlow::Break(error) = callback(
                            malformed_len,
                            bytes_after_malformed,
                            &input[byte_idx..],
                            output,
                        ) {
                            if error.is_empty() {
                                break Err(LoadError::Decode(Cow::Owned(format!(
                                    "Invalid character sequence at {byte_idx}: {malformed_sequence:?}",
                                ))));
                            }
                            break Err(LoadError::Decode(error));
                        }
                    }
                }
            }
        }
    }
}

/// The encoding crate knows how to tell apart UTF-8 from UTF-16LE and utf-16BE, when the
/// bytestream starts with BOM codepoint.
/// However, it doesn't even attempt to guess the UTF-16 endianness of the input bytestream since
/// in the general case the bytestream could start with a codepoint that uses both bytes.
///
/// The YAML-1.2 spec mandates that the first character of a YAML document is an ASCII character.
/// This allows the encoding to be deduced by the pattern of null (#x00) characters.
//
/// See spec at <https://yaml.org/spec/1.2/spec.html#id2771184>
#[cfg(feature = "encoding")]
fn detect_utf16_endianness(b: &[u8]) -> &'static Encoding {
    if b.len() > 1 && (b[0] != b[1]) {
        if b[0] == 0 {
            return encoding_rs::UTF_16BE;
        } else if b[1] == 0 {
            return encoding_rs::UTF_16LE;
        }
    }
    encoding_rs::UTF_8
}

macro_rules! define_as (
    ($name:ident, $t:ident, $yt:ident) => (
/// Get a copy of the inner object in the YAML enum if it is a `$t`.
///
/// # Return
/// If the variant of `self` is `Yaml::$yt`, return `Some($t)` with a copy of the `$t` contained.
/// Otherwise, return `None`.
#[must_use]
pub fn $name(&self) -> Option<$t> {
    match *self {
        Yaml::$yt(v) => Some(v),
        _ => None
    }
}
    );
);

macro_rules! define_as_ref (
    ($name:ident, $t:ty, $yt:ident) => (
/// Get a reference to the inner object in the YAML enum if it is a `$t`.
///
/// # Return
/// If the variant of `self` is `Yaml::$yt`, return `Some(&$t)` with the `$t` contained. Otherwise,
/// return `None`.
#[must_use]
pub fn $name(&self) -> Option<$t> {
    match *self {
        Yaml::$yt(ref v) => Some(v),
        _ => None
    }
}
    );
);

macro_rules! define_as_mut_ref (
    ($name:ident, $t:ty, $yt:ident) => (
/// Get a mutable reference to the inner object in the YAML enum if it is a `$t`.
///
/// # Return
/// If the variant of `self` is `Yaml::$yt`, return `Some(&mut $t)` with the `$t` contained.
/// Otherwise, return `None`.
#[must_use]
pub fn $name(&mut self) -> Option<$t> {
    match *self {
        Yaml::$yt(ref mut v) => Some(v),
        _ => None
    }
}
    );
);

macro_rules! define_into (
    ($name:ident, $t:ty, $yt:ident) => (
/// Get the inner object in the YAML enum if it is a `$t`.
///
/// # Return
/// If the variant of `self` is `Yaml::$yt`, return `Some($t)` with the `$t` contained. Otherwise,
/// return `None`.
#[must_use]
pub fn $name(self) -> Option<$t> {
    match self {
        Yaml::$yt(v) => Some(v),
        _ => None
    }
}
    );
);

impl Yaml {
    define_as!(as_bool, bool, Boolean);
    define_as!(as_i64, i64, Integer);

    define_as_ref!(as_str, &str, String);
    define_as_ref!(as_hash, &Hash, Hash);
    define_as_ref!(as_vec, &Array, Array);

    define_as_mut_ref!(as_mut_hash, &mut Hash, Hash);
    define_as_mut_ref!(as_mut_vec, &mut Array, Array);

    define_into!(into_bool, bool, Boolean);
    define_into!(into_i64, i64, Integer);
    define_into!(into_string, String, String);
    define_into!(into_hash, Hash, Hash);
    define_into!(into_vec, Array, Array);

    /// Return whether `self` is a [`Yaml::Null`] node.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(*self, Yaml::Null)
    }

    /// Return whether `self` is a [`Yaml::BadValue`] node.
    #[must_use]
    pub fn is_badvalue(&self) -> bool {
        matches!(*self, Yaml::BadValue)
    }

    /// Return whether `self` is a [`Yaml::Array`] node.
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(*self, Yaml::Array(_))
    }

    /// Return the `f64` value contained in this YAML node.
    ///
    /// If the node is not a [`Yaml::Real`] YAML node or its contents is not a valid `f64` string,
    /// `None` is returned.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        if let Yaml::Real(ref v) = self {
            parse_f64(v)
        } else {
            None
        }
    }

    /// Return the `f64` value contained in this YAML node.
    ///
    /// If the node is not a [`Yaml::Real`] YAML node or its contents is not a valid `f64` string,
    /// `None` is returned.
    #[must_use]
    pub fn into_f64(self) -> Option<f64> {
        self.as_f64()
    }

    /// If a value is null or otherwise bad (see variants), consume it and
    /// replace it with a given value `other`. Otherwise, return self unchanged.
    ///
    /// ```
    /// use yaml_rust2::yaml::Yaml;
    ///
    /// assert_eq!(Yaml::BadValue.or(Yaml::Integer(3)),  Yaml::Integer(3));
    /// assert_eq!(Yaml::Integer(3).or(Yaml::BadValue),  Yaml::Integer(3));
    /// ```
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        match self {
            Yaml::BadValue | Yaml::Null => other,
            this => this,
        }
    }

    /// See `or` for behavior. This performs the same operations, but with
    /// borrowed values for less linear pipelines.
    #[must_use]
    pub fn borrowed_or<'a>(&'a self, other: &'a Self) -> &'a Self {
        match self {
            Yaml::BadValue | Yaml::Null => other,
            this => this,
        }
    }
}

#[allow(clippy::should_implement_trait)]
impl Yaml {
    /// Convert a string to a [`Yaml`] node.
    ///
    /// [`Yaml`] does not implement [`std::str::FromStr`] since conversion may not fail. This
    /// function falls back to [`Yaml::String`] if nothing else matches.
    ///
    /// # Examples
    /// ```
    /// # use yaml_rust2::yaml::Yaml;
    /// assert!(matches!(Yaml::from_str("42"), Yaml::Integer(42)));
    /// assert!(matches!(Yaml::from_str("0x2A"), Yaml::Integer(42)));
    /// assert!(matches!(Yaml::from_str("0o52"), Yaml::Integer(42)));
    /// assert!(matches!(Yaml::from_str("~"), Yaml::Null));
    /// assert!(matches!(Yaml::from_str("null"), Yaml::Null));
    /// assert!(matches!(Yaml::from_str("true"), Yaml::Boolean(true)));
    /// assert!(matches!(Yaml::from_str("3.14"), Yaml::Real(_)));
    /// assert!(matches!(Yaml::from_str("foo"), Yaml::String(_)));
    /// ```
    #[must_use]
    pub fn from_str(v: &str) -> Yaml {
        if let Some(number) = v.strip_prefix("0x") {
            if let Ok(i) = i64::from_str_radix(number, 16) {
                return Yaml::Integer(i);
            }
        } else if let Some(number) = v.strip_prefix("0o") {
            if let Ok(i) = i64::from_str_radix(number, 8) {
                return Yaml::Integer(i);
            }
        } else if let Some(number) = v.strip_prefix('+') {
            if let Ok(i) = number.parse::<i64>() {
                return Yaml::Integer(i);
            }
        }
        match v {
            "~" | "null" => Yaml::Null,
            "true" => Yaml::Boolean(true),
            "false" => Yaml::Boolean(false),
            _ => {
                if let Ok(integer) = v.parse::<i64>() {
                    Yaml::Integer(integer)
                } else if parse_f64(v).is_some() {
                    Yaml::Real(v.to_owned())
                } else {
                    Yaml::String(v.to_owned())
                }
            }
        }
    }
}

static BAD_VALUE: Yaml = Yaml::BadValue;
impl<'a> Index<&'a str> for Yaml {
    type Output = Yaml;

    fn index(&self, idx: &'a str) -> &Yaml {
        let key = Yaml::String(idx.to_owned());
        match self.as_hash() {
            Some(h) => h.get(&key).unwrap_or(&BAD_VALUE),
            None => &BAD_VALUE,
        }
    }
}

impl<'a> IndexMut<&'a str> for Yaml {
    fn index_mut(&mut self, idx: &'a str) -> &mut Yaml {
        let key = Yaml::String(idx.to_owned());
        match self.as_mut_hash() {
            Some(h) => h.get_mut(&key).unwrap(),
            None => panic!("Not a hash type"),
        }
    }
}

impl Index<usize> for Yaml {
    type Output = Yaml;

    fn index(&self, idx: usize) -> &Yaml {
        if let Some(v) = self.as_vec() {
            v.get(idx).unwrap_or(&BAD_VALUE)
        } else if let Some(v) = self.as_hash() {
            let key = Yaml::Integer(i64::try_from(idx).unwrap());
            v.get(&key).unwrap_or(&BAD_VALUE)
        } else {
            &BAD_VALUE
        }
    }
}

impl IndexMut<usize> for Yaml {
    /// Perform indexing if `self` is a sequence or a mapping.
    ///
    /// # Panics
    /// This function panics if the index given is out of range (as per [`IndexMut`]). If `self` i
    /// a [`Yaml::Array`], this is when the index is bigger or equal to the length of the
    /// underlying `Vec`. If `self` is a [`Yaml::Hash`], this is when the mapping sequence does no
    /// contain [`Yaml::Integer`]`(idx)` as a key.
    ///
    /// This function also panics if `self` is not a [`Yaml::Array`] nor a [`Yaml::Hash`].
    fn index_mut(&mut self, idx: usize) -> &mut Yaml {
        match self {
            Yaml::Array(sequence) => sequence.index_mut(idx),
            Yaml::Hash(mapping) => {
                let key = Yaml::Integer(i64::try_from(idx).unwrap());
                mapping.get_mut(&key).unwrap()
            }
            _ => panic!("Attempting to index but `self` is not a sequence nor a mapping"),
        }
    }
}

impl IntoIterator for Yaml {
    type Item = Yaml;
    type IntoIter = YamlIter;

    fn into_iter(self) -> Self::IntoIter {
        YamlIter {
            yaml: self.into_vec().unwrap_or_default().into_iter(),
        }
    }
}

/// An iterator over a [`Yaml`] node.
pub struct YamlIter {
    yaml: std::vec::IntoIter<Yaml>,
}

impl Iterator for YamlIter {
    type Item = Yaml;

    fn next(&mut self) -> Option<Yaml> {
        self.yaml.next()
    }
}

#[cfg(test)]
mod test {
    use super::{YAMLDecodingTrap, Yaml, YamlDecoder};

    #[test]
    fn test_read_bom() {
        let s = b"\xef\xbb\xbf---
a: 1
b: 2.2
c: [1, 2]
";
        let out = YamlDecoder::read(s as &[u8]).decode().unwrap();
        let doc = &out[0];
        assert_eq!(doc["a"].as_i64().unwrap(), 1i64);
        assert!((doc["b"].as_f64().unwrap() - 2.2f64).abs() <= f64::EPSILON);
        assert_eq!(doc["c"][1].as_i64().unwrap(), 2i64);
        assert!(doc["d"][0].is_badvalue());
    }

    #[test]
    fn test_read_utf16le() {
        let s = b"\xff\xfe-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
\x00";
        let out = YamlDecoder::read(s as &[u8]).decode().unwrap();
        let doc = &out[0];
        println!("GOT: {doc:?}");
        assert_eq!(doc["a"].as_i64().unwrap(), 1i64);
        assert!((doc["b"].as_f64().unwrap() - 2.2f64) <= f64::EPSILON);
        assert_eq!(doc["c"][1].as_i64().unwrap(), 2i64);
        assert!(doc["d"][0].is_badvalue());
    }

    #[test]
    fn test_read_utf16be() {
        let s = b"\xfe\xff\x00-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
";
        let out = YamlDecoder::read(s as &[u8]).decode().unwrap();
        let doc = &out[0];
        println!("GOT: {doc:?}");
        assert_eq!(doc["a"].as_i64().unwrap(), 1i64);
        assert!((doc["b"].as_f64().unwrap() - 2.2f64).abs() <= f64::EPSILON);
        assert_eq!(doc["c"][1].as_i64().unwrap(), 2i64);
        assert!(doc["d"][0].is_badvalue());
    }

    #[test]
    fn test_read_utf16le_nobom() {
        let s = b"-\x00-\x00-\x00
\x00a\x00:\x00 \x001\x00
\x00b\x00:\x00 \x002\x00.\x002\x00
\x00c\x00:\x00 \x00[\x001\x00,\x00 \x002\x00]\x00
\x00";
        let out = YamlDecoder::read(s as &[u8]).decode().unwrap();
        let doc = &out[0];
        println!("GOT: {doc:?}");
        assert_eq!(doc["a"].as_i64().unwrap(), 1i64);
        assert!((doc["b"].as_f64().unwrap() - 2.2f64).abs() <= f64::EPSILON);
        assert_eq!(doc["c"][1].as_i64().unwrap(), 2i64);
        assert!(doc["d"][0].is_badvalue());
    }

    #[test]
    fn test_read_trap() {
        let s = b"---
a\xa9: 1
b: 2.2
c: [1, 2]
";
        let out = YamlDecoder::read(s as &[u8])
            .encoding_trap(YAMLDecodingTrap::Ignore)
            .decode()
            .unwrap();
        let doc = &out[0];
        println!("GOT: {doc:?}");
        assert_eq!(doc["a"].as_i64().unwrap(), 1i64);
        assert!((doc["b"].as_f64().unwrap() - 2.2f64).abs() <= f64::EPSILON);
        assert_eq!(doc["c"][1].as_i64().unwrap(), 2i64);
        assert!(doc["d"][0].is_badvalue());
    }

    #[test]
    fn test_or() {
        assert_eq!(Yaml::Null.or(Yaml::Integer(3)), Yaml::Integer(3));
        assert_eq!(Yaml::Integer(3).or(Yaml::Integer(7)), Yaml::Integer(3));
    }
}
