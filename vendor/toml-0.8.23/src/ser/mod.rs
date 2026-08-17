//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures
//! into TOML documents (as strings). Note that some top-level functions here
//! are also provided at the top of the crate.

#[cfg(feature = "display")]
mod array;
#[cfg(feature = "display")]
mod map;
#[cfg(feature = "display")]
mod ser_value;

#[cfg(feature = "display")]
pub use ser_value::ValueSerializer;

/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
///
/// # Examples
///
/// ```
/// use serde::Serialize;
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
#[cfg(feature = "display")]
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut output = String::new();
    let serializer = Serializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}

/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `Serializer::pretty` for more details.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
///
/// For greater customization, instead serialize to a
/// [`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html).
#[cfg(feature = "display")]
pub fn to_string_pretty<T>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut output = String::new();
    let serializer = Serializer::pretty(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}

/// Errors that can occur when serializing a type.
#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) inner: crate::edit::ser::Error,
}

impl Error {
    pub(crate) fn new(inner: impl std::fmt::Display) -> Self {
        Self {
            inner: crate::edit::ser::Error::Custom(inner.to_string()),
        }
    }

    #[cfg(feature = "display")]
    pub(crate) fn wrap(inner: crate::edit::ser::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedType(t),
        }
    }

    pub(crate) fn unsupported_none() -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedNone,
        }
    }

    pub(crate) fn key_not_string() -> Self {
        Self {
            inner: crate::edit::ser::Error::KeyNotString,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

/// Serialization for TOML documents.
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
#[cfg(feature = "display")]
pub struct Serializer<'d> {
    dst: &'d mut String,
    settings: crate::fmt::DocumentFormatter,
}

#[cfg(feature = "display")]
impl<'d> Serializer<'d> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: &'d mut String) -> Self {
        Self {
            dst,
            settings: Default::default(),
        }
    }

    /// Apply a default "pretty" policy to the document
    ///
    /// For greater customization, instead serialize to a
    /// [`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html).
    pub fn pretty(dst: &'d mut String) -> Self {
        let mut ser = Serializer::new(dst);
        ser.settings.multiline_array = true;
        ser
    }
}

#[cfg(feature = "display")]
impl<'d> serde::ser::Serializer for Serializer<'d> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = array::SerializeDocumentArray<'d>;
    type SerializeTuple = array::SerializeDocumentArray<'d>;
    type SerializeTupleStruct = array::SerializeDocumentArray<'d>;
    type SerializeTupleVariant = array::SerializeDocumentTupleVariant<'d>;
    type SerializeMap = map::SerializeDocumentTable<'d>;
    type SerializeStruct = map::SerializeDocumentTable<'d>;
    type SerializeStructVariant = map::SerializeDocumentStructVariant<'d>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bool(v),
        )
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i8(v),
        )
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i16(v),
        )
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i32(v),
        )
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i64(v),
        )
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u8(v),
        )
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u16(v),
        )
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u32(v),
        )
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u64(v),
        )
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f32(v),
        )
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f64(v),
        )
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_char(v),
        )
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_str(v),
        )
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bytes(v),
        )
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_none(),
        )
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_some(v),
        )
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit(),
        )
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit_struct(name),
        )
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit_variant(
                name,
                variant_index,
                variant,
            ),
        )
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_struct(name, v),
        )
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_variant(
                name,
                variant_index,
                variant,
                value,
            ),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_seq(len)
            .map_err(Error::wrap)?;
        let ser = array::SerializeDocumentArray::new(self, ser);
        Ok(ser)
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map_err(Error::wrap)?;
        let ser = array::SerializeDocumentTupleVariant::new(self, ser);
        Ok(ser)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_map(len)
            .map_err(Error::wrap)?;
        let ser = map::SerializeDocumentTable::new(self, ser);
        Ok(ser)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_struct_variant(name, variant_index, variant, len)
            .map_err(Error::wrap)?;
        let ser = map::SerializeDocumentStructVariant::new(self, ser);
        Ok(ser)
    }
}

#[cfg(feature = "display")]
pub(crate) fn write_document(
    dst: &mut String,
    mut settings: crate::fmt::DocumentFormatter,
    value: Result<toml_edit::Value, crate::edit::ser::Error>,
) -> Result<(), Error> {
    use std::fmt::Write;
    use toml_edit::visit_mut::VisitMut as _;

    let value = value.map_err(Error::wrap)?;
    let mut table = match toml_edit::Item::Value(value).into_table() {
        Ok(i) => i,
        Err(_) => {
            return Err(Error::unsupported_type(None));
        }
    };

    settings.visit_table_mut(&mut table);

    let doc: toml_edit::DocumentMut = table.into();
    write!(dst, "{doc}").unwrap();

    Ok(())
}
