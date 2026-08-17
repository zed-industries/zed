//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures
//! into TOML documents (as strings). Note that some top-level functions here
//! are also provided at the top of the crate.
//!
//! Note that the TOML format has a restriction that if a table itself contains
//! tables, all keys with non-table values must be emitted first. This is
//! typically easy to ensure happens when you're defining a `struct` as you can
//! reorder the fields manually, but when working with maps (such as `BTreeMap`
//! or `HashMap`) this can lead to serialization errors. In those situations you
//! may use the `tables_last` function in this module like so:
//!
//! ```rust
//! # use serde_derive::Serialize;
//! # use std::collections::HashMap;
//! #[derive(Serialize)]
//! struct Manifest {
//!     package: Package,
//!     #[serde(serialize_with = "toml::ser::tables_last")]
//!     dependencies: HashMap<String, Dependency>,
//! }
//! # type Package = String;
//! # type Dependency = String;
//! # fn main() {}
//! ```

use std::cell::Cell;
use std::error;
use std::fmt::{self, Write};
use std::marker;
use std::rc::Rc;

use crate::datetime;
use serde::ser;

/// Serialize the given data structure as a TOML byte vector.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ser::Serialize,
{
    to_string(value).map(|e| e.into_bytes())
}

/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
///
/// # Examples
///
/// ```
/// use serde_derive::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Serialize)]
/// struct Database {
///     ip: String,
///     port: Vec<u16>,
///     connection_max: u32,
///     enabled: bool,
/// }
///
/// let config = Config {
///     database: Database {
///         ip: "192.168.1.1".to_string(),
///         port: vec![8001, 8002, 8003],
///         connection_max: 5000,
///         enabled: false,
///     },
/// };
///
/// let toml = toml::to_string(&config).unwrap();
/// println!("{}", toml)
/// ```
pub fn to_string<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: ser::Serialize,
{
    let mut dst = String::with_capacity(128);
    value.serialize(&mut Serializer::new(&mut dst))?;
    Ok(dst)
}

/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `Serializer::pretty` for more details.
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: ser::Serialize,
{
    let mut dst = String::with_capacity(128);
    value.serialize(&mut Serializer::pretty(&mut dst))?;
    Ok(dst)
}

/// Errors that can occur when serializing a type.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Indicates that a Rust type was requested to be serialized but it was not
    /// supported.
    ///
    /// Currently the TOML format does not support serializing types such as
    /// enums, tuples and tuple structs.
    UnsupportedType,

    /// The key of all TOML maps must be strings, but serialization was
    /// attempted where the key of a map was not a string.
    KeyNotString,

    /// An error that we never omit but keep for backwards compatibility
    #[doc(hidden)]
    KeyNewline,

    /// An array had to be homogeneous, but now it is allowed to be heterogeneous.
    #[doc(hidden)]
    ArrayMixedType,

    /// All values in a TOML table must be emitted before further tables are
    /// emitted. If a value is emitted *after* a table then this error is
    /// generated.
    ValueAfterTable,

    /// A serialized date was invalid.
    DateInvalid,

    /// A serialized number was invalid.
    NumberInvalid,

    /// None was attempted to be serialized, but it's not supported.
    UnsupportedNone,

    /// A custom error which could be generated when serializing a particular
    /// type.
    Custom(String),
}

#[derive(Debug, Default, Clone)]
/// Internal place for holding array settings
struct ArraySettings {
    indent: usize,
    trailing_comma: bool,
}

impl ArraySettings {
    fn pretty() -> ArraySettings {
        ArraySettings {
            indent: 4,
            trailing_comma: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
/// String settings
struct StringSettings {
    /// Whether to use literal strings when possible
    literal: bool,
}

impl StringSettings {
    fn pretty() -> StringSettings {
        StringSettings { literal: true }
    }
}

#[derive(Debug, Default, Clone)]
/// Internal struct for holding serialization settings
struct Settings {
    array: Option<ArraySettings>,
    string: Option<StringSettings>,
}

/// Serialization implementation for TOML.
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
pub struct Serializer<'a> {
    dst: &'a mut String,
    state: State<'a>,
    settings: Rc<Settings>,
}

#[derive(Debug, Copy, Clone)]
enum ArrayState {
    Started,
    StartedAsATable,
}

#[derive(Debug, Clone)]
enum State<'a> {
    Table {
        key: &'a str,
        parent: &'a State<'a>,
        first: &'a Cell<bool>,
        table_emitted: &'a Cell<bool>,
    },
    Array {
        parent: &'a State<'a>,
        first: &'a Cell<bool>,
        type_: &'a Cell<Option<ArrayState>>,
        len: Option<usize>,
    },
    End,
}

#[doc(hidden)]
pub struct SerializeSeq<'a, 'b> {
    ser: &'b mut Serializer<'a>,
    first: Cell<bool>,
    type_: Cell<Option<ArrayState>>,
    len: Option<usize>,
}

#[doc(hidden)]
pub enum SerializeTable<'a, 'b> {
    Datetime(&'b mut Serializer<'a>),
    Table {
        ser: &'b mut Serializer<'a>,
        key: String,
        first: Cell<bool>,
        table_emitted: Cell<bool>,
    },
}

impl<'a> Serializer<'a> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: &'a mut String) -> Serializer<'a> {
        Serializer {
            dst,
            state: State::End,
            settings: Rc::new(Settings::default()),
        }
    }

    /// Instantiate a "pretty" formatter
    ///
    /// By default this will use:
    ///
    /// - pretty strings: strings with newlines will use the `'''` syntax. See
    ///   `Serializer::pretty_string`
    /// - pretty arrays: each item in arrays will be on a newline, have an indentation of 4 and
    ///   have a trailing comma. See `Serializer::pretty_array`
    pub fn pretty(dst: &'a mut String) -> Serializer<'a> {
        Serializer {
            dst,
            state: State::End,
            settings: Rc::new(Settings {
                array: Some(ArraySettings::pretty()),
                string: Some(StringSettings::pretty()),
            }),
        }
    }

    /// Enable or Disable pretty strings
    ///
    /// If enabled, literal strings will be used when possible and strings with
    /// one or more newlines will use triple quotes (i.e.: `'''` or `"""`)
    ///
    /// # Examples
    ///
    /// Instead of:
    ///
    /// ```toml,ignore
    /// single = "no newlines"
    /// text = "\nfoo\nbar\n"
    /// ```
    ///
    /// You will have:
    ///
    /// ```toml,ignore
    /// single = 'no newlines'
    /// text = '''
    /// foo
    /// bar
    /// '''
    /// ```
    pub fn pretty_string(&mut self, value: bool) -> &mut Self {
        Rc::get_mut(&mut self.settings).unwrap().string = if value {
            Some(StringSettings::pretty())
        } else {
            None
        };
        self
    }

    /// Enable or Disable Literal strings for pretty strings
    ///
    /// If enabled, literal strings will be used when possible and strings with
    /// one or more newlines will use triple quotes (i.e.: `'''` or `"""`)
    ///
    /// If disabled, literal strings will NEVER be used and strings with one or
    /// more newlines will use `"""`
    ///
    /// # Examples
    ///
    /// Instead of:
    ///
    /// ```toml,ignore
    /// single = "no newlines"
    /// text = "\nfoo\nbar\n"
    /// ```
    ///
    /// You will have:
    ///
    /// ```toml,ignore
    /// single = "no newlines"
    /// text = """
    /// foo
    /// bar
    /// """
    /// ```
    pub fn pretty_string_literal(&mut self, value: bool) -> &mut Self {
        let use_default = if let Some(ref mut s) = Rc::get_mut(&mut self.settings).unwrap().string {
            s.literal = value;
            false
        } else {
            true
        };

        if use_default {
            let mut string = StringSettings::pretty();
            string.literal = value;
            Rc::get_mut(&mut self.settings).unwrap().string = Some(string);
        }
        self
    }

    /// Enable or Disable pretty arrays
    ///
    /// If enabled, arrays will always have each item on their own line.
    ///
    /// Some specific features can be controlled via other builder methods:
    ///
    /// - `Serializer::pretty_array_indent`: set the indent to a value other
    ///   than 4.
    /// - `Serializer::pretty_array_trailing_comma`: enable/disable the trailing
    ///   comma on the last item.
    ///
    /// # Examples
    ///
    /// Instead of:
    ///
    /// ```toml,ignore
    /// array = ["foo", "bar"]
    /// ```
    ///
    /// You will have:
    ///
    /// ```toml,ignore
    /// array = [
    ///     "foo",
    ///     "bar",
    /// ]
    /// ```
    pub fn pretty_array(&mut self, value: bool) -> &mut Self {
        Rc::get_mut(&mut self.settings).unwrap().array = if value {
            Some(ArraySettings::pretty())
        } else {
            None
        };
        self
    }

    /// Set the indent for pretty arrays
    ///
    /// See `Serializer::pretty_array` for more details.
    pub fn pretty_array_indent(&mut self, value: usize) -> &mut Self {
        let use_default = if let Some(ref mut a) = Rc::get_mut(&mut self.settings).unwrap().array {
            a.indent = value;
            false
        } else {
            true
        };

        if use_default {
            let mut array = ArraySettings::pretty();
            array.indent = value;
            Rc::get_mut(&mut self.settings).unwrap().array = Some(array);
        }
        self
    }

    /// Specify whether to use a trailing comma when serializing pretty arrays
    ///
    /// See `Serializer::pretty_array` for more details.
    pub fn pretty_array_trailing_comma(&mut self, value: bool) -> &mut Self {
        let use_default = if let Some(ref mut a) = Rc::get_mut(&mut self.settings).unwrap().array {
            a.trailing_comma = value;
            false
        } else {
            true
        };

        if use_default {
            let mut array = ArraySettings::pretty();
            array.trailing_comma = value;
            Rc::get_mut(&mut self.settings).unwrap().array = Some(array);
        }
        self
    }

    fn display<T: fmt::Display>(&mut self, t: T, type_: ArrayState) -> Result<(), Error> {
        self.emit_key(type_)?;
        write!(self.dst, "{}", t).map_err(ser::Error::custom)?;
        if let State::Table { .. } = self.state {
            self.dst.push('\n');
        }
        Ok(())
    }

    fn emit_key(&mut self, type_: ArrayState) -> Result<(), Error> {
        self.array_type(type_)?;
        let state = self.state.clone();
        self._emit_key(&state)
    }

    // recursive implementation of `emit_key` above
    fn _emit_key(&mut self, state: &State<'_>) -> Result<(), Error> {
        match *state {
            State::End => Ok(()),
            State::Array {
                parent,
                first,
                type_,
                len,
            } => {
                assert!(type_.get().is_some());
                if first.get() {
                    self._emit_key(parent)?;
                }
                self.emit_array(first, len)
            }
            State::Table {
                parent,
                first,
                table_emitted,
                key,
            } => {
                if table_emitted.get() {
                    return Err(Error::ValueAfterTable);
                }
                if first.get() {
                    self.emit_table_header(parent)?;
                    first.set(false);
                }
                self.escape_key(key)?;
                self.dst.push_str(" = ");
                Ok(())
            }
        }
    }

    fn emit_array(&mut self, first: &Cell<bool>, len: Option<usize>) -> Result<(), Error> {
        match (len, &self.settings.array) {
            (Some(0..=1), _) | (_, &None) => {
                if first.get() {
                    self.dst.push('[')
                } else {
                    self.dst.push_str(", ")
                }
            }
            (_, &Some(ref a)) => {
                if first.get() {
                    self.dst.push_str("[\n")
                } else {
                    self.dst.push_str(",\n")
                }
                for _ in 0..a.indent {
                    self.dst.push(' ');
                }
            }
        }
        Ok(())
    }

    fn array_type(&mut self, type_: ArrayState) -> Result<(), Error> {
        let prev = match self.state {
            State::Array { type_, .. } => type_,
            _ => return Ok(()),
        };
        if prev.get().is_none() {
            prev.set(Some(type_));
        }
        Ok(())
    }

    fn escape_key(&mut self, key: &str) -> Result<(), Error> {
        let ok = !key.is_empty()
            && key
                .chars()
                .all(|c| matches!(c,'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'));
        if ok {
            write!(self.dst, "{}", key).map_err(ser::Error::custom)?;
        } else {
            self.emit_str(key, true)?;
        }
        Ok(())
    }

    fn emit_str(&mut self, value: &str, is_key: bool) -> Result<(), Error> {
        #[derive(PartialEq)]
        enum Type {
            NewlineTripple,
            OnelineTripple,
            OnelineSingle,
        }

        enum Repr {
            /// represent as a literal string (using '')
            Literal(String, Type),
            /// represent the std way (using "")
            Std(Type),
        }

        fn do_pretty(value: &str) -> Repr {
            // For doing pretty prints we store in a new String
            // because there are too many cases where pretty cannot
            // work. We need to determine:
            // - if we are a "multi-line" pretty (if there are \n)
            // - if ['''] appears if multi or ['] if single
            // - if there are any invalid control characters
            //
            // Doing it any other way would require multiple passes
            // to determine if a pretty string works or not.
            let mut out = String::with_capacity(value.len() * 2);
            let mut ty = Type::OnelineSingle;
            // found consecutive single quotes
            let mut max_found_singles = 0;
            let mut found_singles = 0;
            let mut can_be_pretty = true;

            for ch in value.chars() {
                if can_be_pretty {
                    if ch == '\'' {
                        found_singles += 1;
                        if found_singles >= 3 {
                            can_be_pretty = false;
                        }
                    } else {
                        if found_singles > max_found_singles {
                            max_found_singles = found_singles;
                        }
                        found_singles = 0
                    }
                    match ch {
                        '\t' => {}
                        '\n' => ty = Type::NewlineTripple,
                        // Escape codes are needed if any ascii control
                        // characters are present, including \b \f \r.
                        c if c <= '\u{1f}' || c == '\u{7f}' => can_be_pretty = false,
                        _ => {}
                    }
                    out.push(ch);
                } else {
                    // the string cannot be represented as pretty,
                    // still check if it should be multiline
                    if ch == '\n' {
                        ty = Type::NewlineTripple;
                    }
                }
            }
            if can_be_pretty && found_singles > 0 && value.ends_with('\'') {
                // We cannot escape the ending quote so we must use """
                can_be_pretty = false;
            }
            if !can_be_pretty {
                debug_assert!(ty != Type::OnelineTripple);
                return Repr::Std(ty);
            }
            if found_singles > max_found_singles {
                max_found_singles = found_singles;
            }
            debug_assert!(max_found_singles < 3);
            if ty == Type::OnelineSingle && max_found_singles >= 1 {
                // no newlines, but must use ''' because it has ' in it
                ty = Type::OnelineTripple;
            }
            Repr::Literal(out, ty)
        }

        let repr = if !is_key && self.settings.string.is_some() {
            match (&self.settings.string, do_pretty(value)) {
                (&Some(StringSettings { literal: false, .. }), Repr::Literal(_, ty)) => {
                    Repr::Std(ty)
                }
                (_, r) => r,
            }
        } else {
            Repr::Std(Type::OnelineSingle)
        };
        match repr {
            Repr::Literal(literal, ty) => {
                // A pretty string
                match ty {
                    Type::NewlineTripple => self.dst.push_str("'''\n"),
                    Type::OnelineTripple => self.dst.push_str("'''"),
                    Type::OnelineSingle => self.dst.push('\''),
                }
                self.dst.push_str(&literal);
                match ty {
                    Type::OnelineSingle => self.dst.push('\''),
                    _ => self.dst.push_str("'''"),
                }
            }
            Repr::Std(ty) => {
                match ty {
                    Type::NewlineTripple => self.dst.push_str("\"\"\"\n"),
                    // note: OnelineTripple can happen if do_pretty wants to do
                    // '''it's one line'''
                    // but settings.string.literal == false
                    Type::OnelineSingle | Type::OnelineTripple => self.dst.push('"'),
                }
                for ch in value.chars() {
                    match ch {
                        '\u{8}' => self.dst.push_str("\\b"),
                        '\u{9}' => self.dst.push_str("\\t"),
                        '\u{a}' => match ty {
                            Type::NewlineTripple => self.dst.push('\n'),
                            Type::OnelineSingle => self.dst.push_str("\\n"),
                            _ => unreachable!(),
                        },
                        '\u{c}' => self.dst.push_str("\\f"),
                        '\u{d}' => self.dst.push_str("\\r"),
                        '\u{22}' => self.dst.push_str("\\\""),
                        '\u{5c}' => self.dst.push_str("\\\\"),
                        c if c <= '\u{1f}' || c == '\u{7f}' => {
                            write!(self.dst, "\\u{:04X}", ch as u32).map_err(ser::Error::custom)?;
                        }
                        ch => self.dst.push(ch),
                    }
                }
                match ty {
                    Type::NewlineTripple => self.dst.push_str("\"\"\""),
                    Type::OnelineSingle | Type::OnelineTripple => self.dst.push('"'),
                }
            }
        }
        Ok(())
    }

    fn emit_table_header(&mut self, state: &State<'_>) -> Result<(), Error> {
        let array_of_tables = match *state {
            State::End => return Ok(()),
            State::Array { .. } => true,
            _ => false,
        };

        // Unlike [..]s, we can't omit [[..]] ancestors, so be sure to emit table
        // headers for them.
        let mut p = state;
        if let State::Array { first, parent, .. } = *state {
            if first.get() {
                p = parent;
            }
        }
        while let State::Table { first, parent, .. } = *p {
            p = parent;
            if !first.get() {
                break;
            }
            if let State::Array {
                parent: &State::Table { .. },
                ..
            } = *parent
            {
                self.emit_table_header(parent)?;
                break;
            }
        }

        match *state {
            State::Table { first, .. } => {
                if !first.get() {
                    // Newline if we are a table that is not the first
                    // table in the document.
                    self.dst.push('\n');
                }
            }
            State::Array { parent, first, .. } => {
                if !first.get() {
                    // Always newline if we are not the first item in the
                    // table-array
                    self.dst.push('\n');
                } else if let State::Table { first, .. } = *parent {
                    if !first.get() {
                        // Newline if we are not the first item in the document
                        self.dst.push('\n');
                    }
                }
            }
            _ => {}
        }
        self.dst.push('[');
        if array_of_tables {
            self.dst.push('[');
        }
        self.emit_key_part(state)?;
        if array_of_tables {
            self.dst.push(']');
        }
        self.dst.push_str("]\n");
        Ok(())
    }

    fn emit_key_part(&mut self, key: &State<'_>) -> Result<bool, Error> {
        match *key {
            State::Array { parent, .. } => self.emit_key_part(parent),
            State::End => Ok(true),
            State::Table {
                key,
                parent,
                table_emitted,
                ..
            } => {
                table_emitted.set(true);
                let first = self.emit_key_part(parent)?;
                if !first {
                    self.dst.push('.');
                }
                self.escape_key(key)?;
                Ok(false)
            }
        }
    }
}

macro_rules! serialize_float {
    ($this:expr, $v:expr) => {{
        $this.emit_key(ArrayState::Started)?;
        match ($v.is_sign_negative(), $v.is_nan(), $v == 0.0) {
            (true, true, _) => write!($this.dst, "-nan"),
            (false, true, _) => write!($this.dst, "nan"),
            (true, false, true) => write!($this.dst, "-0.0"),
            (false, false, true) => write!($this.dst, "0.0"),
            (_, false, false) => write!($this.dst, "{}", $v).and_then(|_| {
                if $v % 1.0 == 0.0 {
                    write!($this.dst, ".0")
                } else {
                    Ok(())
                }
            }),
        }
        .map_err(ser::Error::custom)?;

        if let State::Table { .. } = $this.state {
            $this.dst.push_str("\n");
        }
        return Ok(());
    }};
}

impl<'a, 'b> ser::Serializer for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'a, 'b>;
    type SerializeTuple = SerializeSeq<'a, 'b>;
    type SerializeTupleStruct = SerializeSeq<'a, 'b>;
    type SerializeTupleVariant = SerializeSeq<'a, 'b>;
    type SerializeMap = SerializeTable<'a, 'b>;
    type SerializeStruct = SerializeTable<'a, 'b>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        self.display(v, ArrayState::Started)
    }

    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        serialize_float!(self, v)
    }

    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        serialize_float!(self, v)
    }

    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, value: &str) -> Result<(), Self::Error> {
        self.emit_key(ArrayState::Started)?;
        self.emit_str(value, false)?;
        if let State::Table { .. } = self.state {
            self.dst.push('\n');
        }
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<(), Self::Error> {
        use serde::ser::Serialize;
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::UnsupportedNone)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.array_type(ArrayState::Started)?;
        Ok(SerializeSeq {
            ser: self,
            first: Cell::new(true),
            type_: Cell::new(None),
            len,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.array_type(ArrayState::StartedAsATable)?;
        Ok(SerializeTable::Table {
            ser: self,
            key: String::new(),
            first: Cell::new(true),
            table_emitted: Cell::new(false),
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if name == datetime::NAME {
            self.array_type(ArrayState::Started)?;
            Ok(SerializeTable::Datetime(self))
        } else {
            self.array_type(ArrayState::StartedAsATable)?;
            Ok(SerializeTable::Table {
                ser: self,
                key: String::new(),
                first: Cell::new(true),
                table_emitted: Cell::new(false),
            })
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }
}

impl<'a, 'b> ser::SerializeSeq for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        value.serialize(&mut Serializer {
            dst: &mut *self.ser.dst,
            state: State::Array {
                parent: &self.ser.state,
                first: &self.first,
                type_: &self.type_,
                len: self.len,
            },
            settings: self.ser.settings.clone(),
        })?;
        self.first.set(false);
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        match self.type_.get() {
            Some(ArrayState::StartedAsATable) => return Ok(()),
            Some(ArrayState::Started) => match (self.len, &self.ser.settings.array) {
                (Some(0..=1), _) | (_, &None) => {
                    self.ser.dst.push(']');
                }
                (_, &Some(ref a)) => {
                    if a.trailing_comma {
                        self.ser.dst.push(',');
                    }
                    self.ser.dst.push_str("\n]");
                }
            },
            None => {
                assert!(self.first.get());
                self.ser.emit_key(ArrayState::Started)?;
                self.ser.dst.push_str("[]")
            }
        }
        if let State::Table { .. } = self.ser.state {
            self.ser.dst.push('\n');
        }
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTuple for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, 'b> ser::SerializeMap for SerializeTable<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        match *self {
            SerializeTable::Datetime(_) => panic!(), // shouldn't be possible
            SerializeTable::Table { ref mut key, .. } => {
                key.truncate(0);
                *key = input.serialize(StringExtractor)?;
            }
        }
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        match *self {
            SerializeTable::Datetime(_) => panic!(), // shouldn't be possible
            SerializeTable::Table {
                ref mut ser,
                ref key,
                ref first,
                ref table_emitted,
                ..
            } => {
                let res = value.serialize(&mut Serializer {
                    dst: &mut *ser.dst,
                    state: State::Table {
                        key,
                        parent: &ser.state,
                        first,
                        table_emitted,
                    },
                    settings: ser.settings.clone(),
                });
                match res {
                    Ok(()) => first.set(false),
                    Err(Error::UnsupportedNone) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        match self {
            SerializeTable::Datetime(_) => panic!(), // shouldn't be possible
            SerializeTable::Table { ser, first, .. } => {
                if first.get() {
                    let state = ser.state.clone();
                    ser.emit_table_header(&state)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeStruct for SerializeTable<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ser::Serialize,
    {
        match *self {
            SerializeTable::Datetime(ref mut ser) => {
                if key == datetime::FIELD {
                    value.serialize(DateStrEmitter(*ser))?;
                } else {
                    return Err(Error::DateInvalid);
                }
            }
            SerializeTable::Table {
                ref mut ser,
                ref first,
                ref table_emitted,
                ..
            } => {
                let res = value.serialize(&mut Serializer {
                    dst: &mut *ser.dst,
                    state: State::Table {
                        key,
                        parent: &ser.state,
                        first,
                        table_emitted,
                    },
                    settings: ser.settings.clone(),
                });
                match res {
                    Ok(()) => first.set(false),
                    Err(Error::UnsupportedNone) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        match self {
            SerializeTable::Datetime(_) => {}
            SerializeTable::Table { ser, first, .. } => {
                if first.get() {
                    let state = ser.state.clone();
                    ser.emit_table_header(&state)?;
                }
            }
        }
        Ok(())
    }
}

struct DateStrEmitter<'a, 'b>(&'b mut Serializer<'a>);

impl<'a, 'b> ser::Serializer for DateStrEmitter<'a, 'b> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_i8(self, _v: i8) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_i16(self, _v: i16) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_i32(self, _v: i32) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_i64(self, _v: i64) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_u8(self, _v: u8) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_u16(self, _v: u16) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_u32(self, _v: u32) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_u64(self, _v: u64) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_f64(self, _v: f64) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_char(self, _v: char) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_str(self, value: &str) -> Result<(), Self::Error> {
        self.0.display(value, ArrayState::Started)?;
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::DateInvalid)
    }

    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::DateInvalid)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::DateInvalid)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::DateInvalid)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::DateInvalid)
    }
}

struct StringExtractor;

impl ser::Serializer for StringExtractor {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = ser::Impossible<String, Error>;
    type SerializeTuple = ser::Impossible<String, Error>;
    type SerializeTupleStruct = ser::Impossible<String, Error>;
    type SerializeTupleVariant = ser::Impossible<String, Error>;
    type SerializeMap = ser::Impossible<String, Error>;
    type SerializeStruct = ser::Impossible<String, Error>;
    type SerializeStructVariant = ser::Impossible<String, Error>;

    fn serialize_bool(self, _v: bool) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_i8(self, _v: i8) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_i16(self, _v: i16) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_i32(self, _v: i32) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_i64(self, _v: i64) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_u8(self, _v: u8) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_u16(self, _v: u16) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_u32(self, _v: u32) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_u64(self, _v: u64) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_f32(self, _v: f32) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_f64(self, _v: f64) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_char(self, _v: char) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_str(self, value: &str) -> Result<String, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_none(self) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<String, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::KeyNotString)
    }

    fn serialize_unit(self) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<String, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::KeyNotString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::KeyNotString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::KeyNotString)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnsupportedType => "unsupported Rust type".fmt(f),
            Error::KeyNotString => "map key was not a string".fmt(f),
            Error::ValueAfterTable => "values must be emitted before tables".fmt(f),
            Error::DateInvalid => "a serialized date was invalid".fmt(f),
            Error::NumberInvalid => "a serialized number was invalid".fmt(f),
            Error::UnsupportedNone => "unsupported None value".fmt(f),
            Error::Custom(ref s) => s.fmt(f),
            Error::KeyNewline => unreachable!(),
            Error::ArrayMixedType => unreachable!(),
        }
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

enum Category {
    Primitive,
    Array,
    Table,
}

/// Convenience function to serialize items in a map in an order valid with
/// TOML.
///
/// TOML carries the restriction that keys in a table must be serialized last if
/// their value is a table itself. This isn't always easy to guarantee, so this
/// helper can be used like so:
///
/// ```rust
/// # use serde_derive::Serialize;
/// # use std::collections::HashMap;
/// #[derive(Serialize)]
/// struct Manifest {
///     package: Package,
///     #[serde(serialize_with = "toml::ser::tables_last")]
///     dependencies: HashMap<String, Dependency>,
/// }
/// # type Package = String;
/// # type Dependency = String;
/// # fn main() {}
/// ```
pub fn tables_last<'a, I, K, V, S>(data: &'a I, serializer: S) -> Result<S::Ok, S::Error>
where
    &'a I: IntoIterator<Item = (K, V)>,
    K: ser::Serialize,
    V: ser::Serialize,
    S: ser::Serializer,
{
    use serde::ser::SerializeMap;

    let mut map = serializer.serialize_map(None)?;
    for (k, v) in data {
        if let Category::Primitive = v.serialize(Categorize::new())? {
            map.serialize_entry(&k, &v)?;
        }
    }
    for (k, v) in data {
        if let Category::Array = v.serialize(Categorize::new())? {
            map.serialize_entry(&k, &v)?;
        }
    }
    for (k, v) in data {
        if let Category::Table = v.serialize(Categorize::new())? {
            map.serialize_entry(&k, &v)?;
        }
    }
    map.end()
}

struct Categorize<E>(marker::PhantomData<E>);

impl<E> Categorize<E> {
    fn new() -> Self {
        Categorize(marker::PhantomData)
    }
}

impl<E: ser::Error> ser::Serializer for Categorize<E> {
    type Ok = Category;
    type Error = E;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<Category, E>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Primitive)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Array)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, v: &T) -> Result<Self::Ok, Self::Error> {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error> {
        v.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("unsupported"))
    }
}

impl<E: ser::Error> ser::SerializeSeq for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Array)
    }
}

impl<E: ser::Error> ser::SerializeTuple for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Array)
    }
}

impl<E: ser::Error> ser::SerializeTupleVariant for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Array)
    }
}

impl<E: ser::Error> ser::SerializeTupleStruct for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Array)
    }
}

impl<E: ser::Error> ser::SerializeMap for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Table)
    }
}

impl<E: ser::Error> ser::SerializeStruct for Categorize<E> {
    type Ok = Category;
    type Error = E;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Category::Table)
    }
}
