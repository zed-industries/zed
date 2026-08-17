// SPDX-License-Identifier: Apache-2.0

//! A dynamic CBOR value

mod canonical;
mod integer;

mod de;
mod error;
mod ser;

pub use canonical::CanonicalValue;
pub use error::Error;
pub use integer::Integer;

use alloc::{boxed::Box, string::String, vec::Vec};

/// A representation of a dynamic CBOR value that can handled dynamically
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    /// An integer
    Integer(Integer),

    /// Bytes
    Bytes(Vec<u8>),

    /// A float
    Float(f64),

    /// A string
    Text(String),

    /// A boolean
    Bool(bool),

    /// Null
    Null,

    /// Tag
    Tag(u64, Box<Value>),

    /// An array
    Array(Vec<Value>),

    /// A map
    Map(Vec<(Value, Value)>),
}

impl Value {
    /// Returns true if the `Value` is an `Integer`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Integer(17.into());
    ///
    /// assert!(value.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    /// If the `Value` is a `Integer`, returns a reference to the associated `Integer` data.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Integer(17.into());
    ///
    /// // We can read the number
    /// assert_eq!(17, value.as_integer().unwrap().try_into().unwrap());
    /// ```
    pub fn as_integer(&self) -> Option<Integer> {
        match self {
            Value::Integer(int) => Some(*int),
            _ => None,
        }
    }

    /// If the `Value` is a `Integer`, returns a the associated `Integer` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::{Value, value::Integer};
    /// #
    /// let value = Value::Integer(17.into());
    /// assert_eq!(value.into_integer(), Ok(Integer::from(17)));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_integer(), Err(Value::Bool(true)));
    /// ```
    pub fn into_integer(self) -> Result<Integer, Self> {
        match self {
            Value::Integer(int) => Ok(int),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a `Bytes`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bytes(vec![104, 101, 108, 108, 111]);
    ///
    /// assert!(value.is_bytes());
    /// ```
    pub fn is_bytes(&self) -> bool {
        self.as_bytes().is_some()
    }

    /// If the `Value` is a `Bytes`, returns a reference to the associated bytes vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bytes(vec![104, 101, 108, 108, 111]);
    ///
    /// assert_eq!(std::str::from_utf8(value.as_bytes().unwrap()).unwrap(), "hello");
    /// ```
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match *self {
            Value::Bytes(ref bytes) => Some(bytes),
            _ => None,
        }
    }

    /// If the `Value` is a `Bytes`, returns a mutable reference to the associated bytes vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Bytes(vec![104, 101, 108, 108, 111]);
    /// value.as_bytes_mut().unwrap().clear();
    ///
    /// assert_eq!(value, Value::Bytes(vec![]));
    /// ```
    pub fn as_bytes_mut(&mut self) -> Option<&mut Vec<u8>> {
        match *self {
            Value::Bytes(ref mut bytes) => Some(bytes),
            _ => None,
        }
    }

    /// If the `Value` is a `Bytes`, returns a the associated `Vec<u8>` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bytes(vec![104, 101, 108, 108, 111]);
    /// assert_eq!(value.into_bytes(), Ok(vec![104, 101, 108, 108, 111]));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_bytes(), Err(Value::Bool(true)));
    /// ```
    pub fn into_bytes(self) -> Result<Vec<u8>, Self> {
        match self {
            Value::Bytes(vec) => Ok(vec),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a `Float`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Float(17.0.into());
    ///
    /// assert!(value.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// If the `Value` is a `Float`, returns a reference to the associated float data.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Float(17.0.into());
    ///
    /// // We can read the float number
    /// assert_eq!(value.as_float().unwrap(), 17.0_f64);
    /// ```
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Value::Float(f) => Some(f),
            _ => None,
        }
    }

    /// If the `Value` is a `Float`, returns a the associated `f64` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Float(17.);
    /// assert_eq!(value.into_float(), Ok(17.));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_float(), Err(Value::Bool(true)));
    /// ```
    pub fn into_float(self) -> Result<f64, Self> {
        match self {
            Value::Float(f) => Ok(f),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a `Text`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Text(String::from("hello"));
    ///
    /// assert!(value.is_text());
    /// ```
    pub fn is_text(&self) -> bool {
        self.as_text().is_some()
    }

    /// If the `Value` is a `Text`, returns a reference to the associated `String` data.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Text(String::from("hello"));
    ///
    /// // We can read the String
    /// assert_eq!(value.as_text().unwrap(), "hello");
    /// ```
    pub fn as_text(&self) -> Option<&str> {
        match *self {
            Value::Text(ref s) => Some(s),
            _ => None,
        }
    }

    /// If the `Value` is a `Text`, returns a mutable reference to the associated `String` data.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Text(String::from("hello"));
    /// value.as_text_mut().unwrap().clear();
    ///
    /// assert_eq!(value.as_text().unwrap(), &String::from(""));
    /// ```
    pub fn as_text_mut(&mut self) -> Option<&mut String> {
        match *self {
            Value::Text(ref mut s) => Some(s),
            _ => None,
        }
    }

    /// If the `Value` is a `String`, returns a the associated `String` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Text(String::from("hello"));
    /// assert_eq!(value.into_text().as_deref(), Ok("hello"));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_text(), Err(Value::Bool(true)));
    /// ```
    pub fn into_text(self) -> Result<String, Self> {
        match self {
            Value::Text(s) => Ok(s),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a `Bool`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bool(false);
    ///
    /// assert!(value.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `Value` is a `Bool`, returns a copy of the associated boolean value. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bool(false);
    ///
    /// assert_eq!(value.as_bool().unwrap(), false);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// If the `Value` is a `Bool`, returns a the associated `bool` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Bool(false);
    /// assert_eq!(value.into_bool(), Ok(false));
    ///
    /// let value = Value::Float(17.);
    /// assert_eq!(value.into_bool(), Err(Value::Float(17.)));
    /// ```
    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            Value::Bool(b) => Ok(b),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a `Null`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Null;
    ///
    /// assert!(value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if the `Value` is a `Tag`. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Tag(61, Box::from(Value::Null));
    ///
    /// assert!(value.is_tag());
    /// ```
    pub fn is_tag(&self) -> bool {
        self.as_tag().is_some()
    }

    /// If the `Value` is a `Tag`, returns the associated tag value and a reference to the tag `Value`.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Tag(61, Box::from(Value::Bytes(vec![104, 101, 108, 108, 111])));
    ///
    /// let (tag, data) = value.as_tag().unwrap();
    /// assert_eq!(tag, 61);
    /// assert_eq!(data, &Value::Bytes(vec![104, 101, 108, 108, 111]));
    /// ```
    pub fn as_tag(&self) -> Option<(u64, &Value)> {
        match self {
            Value::Tag(tag, data) => Some((*tag, data)),
            _ => None,
        }
    }

    /// If the `Value` is a `Tag`, returns the associated tag value and a mutable reference
    /// to the tag `Value`. Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Tag(61, Box::from(Value::Bytes(vec![104, 101, 108, 108, 111])));
    ///
    /// let (tag, mut data) = value.as_tag_mut().unwrap();
    /// data.as_bytes_mut().unwrap().clear();
    /// assert_eq!(tag, &61);
    /// assert_eq!(data, &Value::Bytes(vec![]));
    /// ```
    pub fn as_tag_mut(&mut self) -> Option<(&mut u64, &mut Value)> {
        match self {
            Value::Tag(tag, data) => Some((tag, data.as_mut())),
            _ => None,
        }
    }

    /// If the `Value` is a `Tag`, returns a the associated pair of `u64` and `Box<value>` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Tag(7, Box::new(Value::Float(12.)));
    /// assert_eq!(value.into_tag(), Ok((7, Box::new(Value::Float(12.)))));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_tag(), Err(Value::Bool(true)));
    /// ```
    pub fn into_tag(self) -> Result<(u64, Box<Value>), Self> {
        match self {
            Value::Tag(tag, value) => Ok((tag, value)),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Array(
    ///     vec![
    ///         Value::Text(String::from("foo")),
    ///         Value::Text(String::from("bar"))
    ///     ]
    /// );
    ///
    /// assert!(value.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the `Value` is an Array, returns a reference to the associated vector. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Array(
    ///     vec![
    ///         Value::Text(String::from("foo")),
    ///         Value::Text(String::from("bar"))
    ///     ]
    /// );
    ///
    /// // The length of `value` is 2 elements.
    /// assert_eq!(value.as_array().unwrap().len(), 2);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(array),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns a mutable reference to the associated vector.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Array(
    ///     vec![
    ///         Value::Text(String::from("foo")),
    ///         Value::Text(String::from("bar"))
    ///     ]
    /// );
    ///
    /// value.as_array_mut().unwrap().clear();
    /// assert_eq!(value, Value::Array(vec![]));
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut list) => Some(list),
            _ => None,
        }
    }

    /// If the `Value` is a `Array`, returns a the associated `Vec<Value>` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::{Value, value::Integer};
    /// #
    /// let mut value = Value::Array(
    ///     vec![
    ///         Value::Integer(17.into()),
    ///         Value::Float(18.),
    ///     ]
    /// );
    /// assert_eq!(value.into_array(), Ok(vec![Value::Integer(17.into()), Value::Float(18.)]));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_array(), Err(Value::Bool(true)));
    /// ```
    pub fn into_array(self) -> Result<Vec<Value>, Self> {
        match self {
            Value::Array(vec) => Ok(vec),
            other => Err(other),
        }
    }

    /// Returns true if the `Value` is a Map. Returns false otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Map(
    ///     vec![
    ///         (Value::Text(String::from("foo")), Value::Text(String::from("bar")))
    ///     ]
    /// );
    ///
    /// assert!(value.is_map());
    /// ```
    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    /// If the `Value` is a Map, returns a reference to the associated Map data. Returns None
    /// otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let value = Value::Map(
    ///     vec![
    ///         (Value::Text(String::from("foo")), Value::Text(String::from("bar")))
    ///     ]
    /// );
    ///
    /// // The length of data is 1 entry (1 key/value pair).
    /// assert_eq!(value.as_map().unwrap().len(), 1);
    ///
    /// // The content of the first element is what we expect
    /// assert_eq!(
    ///     value.as_map().unwrap().get(0).unwrap(),
    ///     &(Value::Text(String::from("foo")), Value::Text(String::from("bar")))
    /// );
    /// ```
    pub fn as_map(&self) -> Option<&Vec<(Value, Value)>> {
        match *self {
            Value::Map(ref map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is a Map, returns a mutable reference to the associated Map Data.
    /// Returns None otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Map(
    ///     vec![
    ///         (Value::Text(String::from("foo")), Value::Text(String::from("bar")))
    ///     ]
    /// );
    ///
    /// value.as_map_mut().unwrap().clear();
    /// assert_eq!(value, Value::Map(vec![]));
    /// assert_eq!(value.as_map().unwrap().len(), 0);
    /// ```
    pub fn as_map_mut(&mut self) -> Option<&mut Vec<(Value, Value)>> {
        match *self {
            Value::Map(ref mut map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is a `Map`, returns a the associated `Vec<(Value, Value)>` data as `Ok`.
    /// Returns `Err(Self)` otherwise.
    ///
    /// ```
    /// # use ciborium::Value;
    /// #
    /// let mut value = Value::Map(
    ///     vec![
    ///         (Value::Text(String::from("key")), Value::Float(18.)),
    ///     ]
    /// );
    /// assert_eq!(value.into_map(), Ok(vec![(Value::Text(String::from("key")), Value::Float(18.))]));
    ///
    /// let value = Value::Bool(true);
    /// assert_eq!(value.into_map(), Err(Value::Bool(true)));
    /// ```
    pub fn into_map(self) -> Result<Vec<(Value, Value)>, Self> {
        match self {
            Value::Map(map) => Ok(map),
            other => Err(other),
        }
    }
}

macro_rules! implfrom {
    ($($v:ident($t:ty)),+ $(,)?) => {
        $(
            impl From<$t> for Value {
                #[inline]
                fn from(value: $t) -> Self {
                    Self::$v(value.into())
                }
            }
        )+
    };
}

implfrom! {
    Integer(Integer),
    Integer(u64),
    Integer(i64),
    Integer(u32),
    Integer(i32),
    Integer(u16),
    Integer(i16),
    Integer(u8),
    Integer(i8),

    Bytes(Vec<u8>),
    Bytes(&[u8]),

    Float(f64),
    Float(f32),

    Text(String),
    Text(&str),

    Bool(bool),

    Array(&[Value]),
    Array(Vec<Value>),

    Map(&[(Value, Value)]),
    Map(Vec<(Value, Value)>),
}

impl From<u128> for Value {
    #[inline]
    fn from(value: u128) -> Self {
        if let Ok(x) = Integer::try_from(value) {
            return Value::Integer(x);
        }

        let mut bytes = &value.to_be_bytes()[..];
        while let Some(0) = bytes.first() {
            bytes = &bytes[1..];
        }

        Value::Tag(ciborium_ll::tag::BIGPOS, Value::Bytes(bytes.into()).into())
    }
}

impl From<i128> for Value {
    #[inline]
    fn from(value: i128) -> Self {
        if let Ok(x) = Integer::try_from(value) {
            return Value::Integer(x);
        }

        let (tag, raw) = match value.is_negative() {
            true => (ciborium_ll::tag::BIGNEG, value as u128 ^ !0),
            false => (ciborium_ll::tag::BIGPOS, value as u128),
        };

        let mut bytes = &raw.to_be_bytes()[..];
        while let Some(0) = bytes.first() {
            bytes = &bytes[1..];
        }

        Value::Tag(tag, Value::Bytes(bytes.into()).into())
    }
}

impl From<char> for Value {
    #[inline]
    fn from(value: char) -> Self {
        let mut v = String::with_capacity(1);
        v.push(value);
        Value::Text(v)
    }
}
