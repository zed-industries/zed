//! Deserializing TOML into Rust structures.
//!
//! This module contains all the Serde support for deserializing TOML documents
//! into Rust structures. Note that some top-level functions here are also
//! provided at the top of the crate.

/// Deserializes a string into a type.
///
/// This function will attempt to interpret `s` as a TOML document and
/// deserialize `T` from the document.
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: Owner,
/// }
///
/// #[derive(Deserialize)]
/// struct Owner {
///     name: String,
/// }
///
/// let config: Config = toml::from_str(r#"
///     title = 'TOML Example'
///
///     [owner]
///     name = 'Lisa'
/// "#).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub fn from_str<T>(s: &'_ str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(Deserializer::new(s))
}

/// Errors that can occur when deserializing a type.
#[derive(PartialEq, Eq, Clone)]
pub struct Error {
    inner: crate::edit::de::Error,
}

impl Error {
    fn new(inner: crate::edit::de::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn add_key(&mut self, key: String) {
        self.inner.add_key(key);
    }

    /// What went wrong
    pub fn message(&self) -> &str {
        self.inner.message()
    }

    /// The start/end index into the original document where the error occurred
    #[cfg(feature = "parse")]
    pub fn span(&self) -> Option<std::ops::Range<usize>> {
        self.inner.span()
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(crate::edit::de::Error::custom(msg))
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

/// Deserialization TOML document
///
/// To deserializes TOML values, instead of documents, see [`ValueDeserializer`].
#[cfg(feature = "parse")]
pub struct Deserializer<'a> {
    input: &'a str,
}

#[cfg(feature = "parse")]
impl<'a> Deserializer<'a> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::Deserializer<'de> for Deserializer<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner.deserialize_option(visitor).map_err(Error::new)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_newtype_struct(name, visitor)
            .map_err(Error::new)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_struct(name, fields, visitor)
            .map_err(Error::new)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = toml_edit::de::Deserializer::parse(self.input).map_err(Error::new)?;
        inner
            .deserialize_enum(name, variants, visitor)
            .map_err(Error::new)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}

/// Deserialization TOML [value][crate::Value]
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     title: String,
///     owner: Owner,
/// }
///
/// #[derive(Deserialize)]
/// struct Owner {
///     name: String,
/// }
///
/// let config = Config::deserialize(toml::de::ValueDeserializer::new(
///     r#"{ title = 'TOML Example', owner = { name = 'Lisa' } }"#
/// )).unwrap();
///
/// assert_eq!(config.title, "TOML Example");
/// assert_eq!(config.owner.name, "Lisa");
/// ```
#[cfg(feature = "parse")]
pub struct ValueDeserializer<'a> {
    input: &'a str,
}

#[cfg(feature = "parse")]
impl<'a> ValueDeserializer<'a> {
    /// Deserialization implementation for TOML.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

#[cfg(feature = "parse")]
impl<'de> serde::Deserializer<'de> for ValueDeserializer<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_any(visitor).map_err(Error::new)
    }

    // `None` is interpreted as a missing field so be sure to implement `Some`
    // as a present field.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner.deserialize_option(visitor).map_err(Error::new)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner
            .deserialize_newtype_struct(name, visitor)
            .map_err(Error::new)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner
            .deserialize_struct(name, fields, visitor)
            .map_err(Error::new)
    }

    // Called when the type to deserialize is an enum, as opposed to a field in the type.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let inner = self
            .input
            .parse::<toml_edit::de::ValueDeserializer>()
            .map_err(Error::new)?;
        inner
            .deserialize_enum(name, variants, visitor)
            .map_err(Error::new)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map unit
        ignored_any unit_struct tuple_struct tuple identifier
    }
}
