//! Algorithm parameters.

use crate::errors::InvalidValue;
use crate::{
    value::{Decimal, Value},
    Encoding, Error, Ident, Result,
};
use core::{
    fmt::{self, Debug, Write},
    iter::FromIterator,
    str::{self, FromStr},
};

/// Individual parameter name/value pair.
pub type Pair<'a> = (Ident<'a>, Value<'a>);

/// Delimiter character between name/value pairs.
pub(crate) const PAIR_DELIMITER: char = '=';

/// Delimiter character between parameters.
pub(crate) const PARAMS_DELIMITER: char = ',';

/// Maximum number of supported parameters.
const MAX_LENGTH: usize = 127;

/// Error message used with `expect` for when internal invariants are violated
/// (i.e. the contents of a [`ParamsString`] should always be valid)
const INVARIANT_VIOLATED_MSG: &str = "PHC params invariant violated";

/// Algorithm parameter string.
///
/// The [PHC string format specification][1] defines a set of optional
/// algorithm-specific name/value pairs which can be encoded into a
/// PHC-formatted parameter string as follows:
///
/// ```text
/// $<param>=<value>(,<param>=<value>)*
/// ```
///
/// This type represents that set of parameters.
///
/// [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification
#[derive(Clone, Default, Eq, PartialEq)]
pub struct ParamsString(Buffer);

impl ParamsString {
    /// Create new empty [`ParamsString`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Add the given byte value to the [`ParamsString`], encoding it as "B64".
    pub fn add_b64_bytes<'a>(&mut self, name: impl TryInto<Ident<'a>>, bytes: &[u8]) -> Result<()> {
        if !self.is_empty() {
            self.0
                .write_char(PARAMS_DELIMITER)
                .map_err(|_| Error::ParamsMaxExceeded)?
        }

        let name = name.try_into().map_err(|_| Error::ParamNameInvalid)?;

        // Add param name
        let offset = self.0.length;
        if write!(self.0, "{}=", name).is_err() {
            self.0.length = offset;
            return Err(Error::ParamsMaxExceeded);
        }

        // Encode B64 value
        let offset = self.0.length as usize;
        let written = Encoding::B64
            .encode(bytes, &mut self.0.bytes[offset..])?
            .len();

        self.0.length += written as u8;
        Ok(())
    }

    /// Add a key/value pair with a decimal value to the [`ParamsString`].
    pub fn add_decimal<'a>(&mut self, name: impl TryInto<Ident<'a>>, value: Decimal) -> Result<()> {
        let name = name.try_into().map_err(|_| Error::ParamNameInvalid)?;
        self.add(name, value)
    }

    /// Add a key/value pair with a string value to the [`ParamsString`].
    pub fn add_str<'a>(
        &mut self,
        name: impl TryInto<Ident<'a>>,
        value: impl TryInto<Value<'a>>,
    ) -> Result<()> {
        let name = name.try_into().map_err(|_| Error::ParamNameInvalid)?;

        let value = value
            .try_into()
            .map_err(|_| Error::ParamValueInvalid(InvalidValue::InvalidFormat))?;

        self.add(name, value)
    }

    /// Borrow the contents of this [`ParamsString`] as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }

    /// Borrow the contents of this [`ParamsString`] as a `str`.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Get the count of the number ASCII characters in this [`ParamsString`].
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Is this set of parameters empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over the parameters.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.as_str())
    }

    /// Get a parameter [`Value`] by name.
    pub fn get<'a>(&self, name: impl TryInto<Ident<'a>>) -> Option<Value<'_>> {
        let name = name.try_into().ok()?;

        for (n, v) in self.iter() {
            if name == n {
                return Some(v);
            }
        }

        None
    }

    /// Get a parameter as a `str`.
    pub fn get_str<'a>(&self, name: impl TryInto<Ident<'a>>) -> Option<&str> {
        self.get(name).map(|value| value.as_str())
    }

    /// Get a parameter as a [`Decimal`].
    ///
    /// See [`Value::decimal`] for format information.
    pub fn get_decimal<'a>(&self, name: impl TryInto<Ident<'a>>) -> Option<Decimal> {
        self.get(name).and_then(|value| value.decimal().ok())
    }

    /// Add a value to this [`ParamsString`] using the provided callback.
    fn add(&mut self, name: Ident<'_>, value: impl fmt::Display) -> Result<()> {
        if self.get(name).is_some() {
            return Err(Error::ParamNameDuplicated);
        }

        let orig_len = self.0.length;

        if !self.is_empty() {
            self.0
                .write_char(PARAMS_DELIMITER)
                .map_err(|_| Error::ParamsMaxExceeded)?
        }

        if write!(self.0, "{}={}", name, value).is_err() {
            self.0.length = orig_len;
            return Err(Error::ParamsMaxExceeded);
        }

        Ok(())
    }
}

impl FromStr for ParamsString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.as_bytes().len() > MAX_LENGTH {
            return Err(Error::ParamsMaxExceeded);
        }

        if s.is_empty() {
            return Ok(ParamsString::new());
        }

        // Validate the string is well-formed
        for mut param in s.split(PARAMS_DELIMITER).map(|p| p.split(PAIR_DELIMITER)) {
            // Validate name
            param
                .next()
                .ok_or(Error::ParamNameInvalid)
                .and_then(Ident::try_from)?;

            // Validate value
            param
                .next()
                .ok_or(Error::ParamValueInvalid(InvalidValue::Malformed))
                .and_then(Value::try_from)?;

            if param.next().is_some() {
                return Err(Error::ParamValueInvalid(InvalidValue::Malformed));
            }
        }

        let mut bytes = [0u8; MAX_LENGTH];
        bytes[..s.as_bytes().len()].copy_from_slice(s.as_bytes());

        Ok(Self(Buffer {
            bytes,
            length: s.as_bytes().len() as u8,
        }))
    }
}

impl<'a> FromIterator<Pair<'a>> for ParamsString {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Pair<'a>>,
    {
        let mut params = ParamsString::new();

        for pair in iter {
            params.add_str(pair.0, pair.1).expect("PHC params error");
        }

        params
    }
}

impl fmt::Display for ParamsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for ParamsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Iterator over algorithm parameters stored in a [`ParamsString`] struct.
pub struct Iter<'a> {
    inner: Option<str::Split<'a, char>>,
}

impl<'a> Iter<'a> {
    /// Create a new [`Iter`].
    fn new(s: &'a str) -> Self {
        if s.is_empty() {
            Self { inner: None }
        } else {
            Self {
                inner: Some(s.split(PARAMS_DELIMITER)),
            }
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Pair<'a>;

    fn next(&mut self) -> Option<Pair<'a>> {
        let mut param = self.inner.as_mut()?.next()?.split(PAIR_DELIMITER);

        let name = param
            .next()
            .and_then(|id| Ident::try_from(id).ok())
            .expect(INVARIANT_VIOLATED_MSG);

        let value = param
            .next()
            .and_then(|value| Value::try_from(value).ok())
            .expect(INVARIANT_VIOLATED_MSG);

        debug_assert_eq!(param.next(), None);
        Some((name, value))
    }
}

/// Parameter buffer.
#[derive(Clone, Debug, Eq)]
struct Buffer {
    /// Byte array containing an ASCII-encoded string.
    bytes: [u8; MAX_LENGTH],

    /// Length of the string in ASCII characters (i.e. bytes).
    length: u8,
}

impl AsRef<str> for Buffer {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.bytes[..(self.length as usize)]).expect(INVARIANT_VIOLATED_MSG)
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            bytes: [0u8; MAX_LENGTH],
            length: 0,
        }
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        // Ensure comparisons always honor the initialized portion of the buffer
        self.as_ref().eq(other.as_ref())
    }
}

impl Write for Buffer {
    fn write_str(&mut self, input: &str) -> fmt::Result {
        let bytes = input.as_bytes();
        let length = self.length as usize;

        if length + bytes.len() > MAX_LENGTH {
            return Err(fmt::Error);
        }

        self.bytes[length..(length + bytes.len())].copy_from_slice(bytes);
        self.length += bytes.len() as u8;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, FromIterator, Ident, ParamsString, Value};

    #[cfg(feature = "alloc")]
    use alloc::string::ToString;
    use core::str::FromStr;

    #[test]
    fn add() {
        let mut params = ParamsString::new();
        params.add_str("a", "1").unwrap();
        params.add_decimal("b", 2).unwrap();
        params.add_str("c", "3").unwrap();

        assert_eq!(params.iter().count(), 3);
        assert_eq!(params.get_decimal("a").unwrap(), 1);
        assert_eq!(params.get_decimal("b").unwrap(), 2);
        assert_eq!(params.get_decimal("c").unwrap(), 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn add_b64_bytes() {
        let mut params = ParamsString::new();
        params.add_b64_bytes("a", &[1]).unwrap();
        params.add_b64_bytes("b", &[2, 3]).unwrap();
        params.add_b64_bytes("c", &[4, 5, 6]).unwrap();
        assert_eq!(params.to_string(), "a=AQ,b=AgM,c=BAUG");
    }

    #[test]
    fn duplicate_names() {
        let name = Ident::new("a").unwrap();
        let mut params = ParamsString::new();
        params.add_decimal(name, 1).unwrap();

        let err = params.add_decimal(name, 2u32.into()).err().unwrap();
        assert_eq!(err, Error::ParamNameDuplicated);
    }

    #[test]
    fn from_iter() {
        let params = ParamsString::from_iter(
            [
                (Ident::new("a").unwrap(), Value::try_from("1").unwrap()),
                (Ident::new("b").unwrap(), Value::try_from("2").unwrap()),
                (Ident::new("c").unwrap(), Value::try_from("3").unwrap()),
            ]
            .iter()
            .cloned(),
        );

        assert_eq!(params.iter().count(), 3);
        assert_eq!(params.get_decimal("a").unwrap(), 1);
        assert_eq!(params.get_decimal("b").unwrap(), 2);
        assert_eq!(params.get_decimal("c").unwrap(), 3);
    }

    #[test]
    fn iter() {
        let mut params = ParamsString::new();
        params.add_str("a", "1").unwrap();
        params.add_str("b", "2").unwrap();
        params.add_str("c", "3").unwrap();

        let mut i = params.iter();

        for (name, value) in &[("a", "1"), ("b", "2"), ("c", "3")] {
            let name = Ident::new(name).unwrap();
            let value = Value::try_from(*value).unwrap();
            assert_eq!(i.next(), Some((name, value)));
        }

        assert_eq!(i.next(), None);
    }

    //
    // `FromStr` tests
    //

    #[test]
    fn parse_empty() {
        let params = ParamsString::from_str("").unwrap();
        assert!(params.is_empty());
    }

    #[test]
    fn parse_one() {
        let params = ParamsString::from_str("a=1").unwrap();
        assert_eq!(params.iter().count(), 1);
        assert_eq!(params.get("a").unwrap().decimal().unwrap(), 1);
    }

    #[test]
    fn parse_many() {
        let params = ParamsString::from_str("a=1,b=2,c=3").unwrap();
        assert_eq!(params.iter().count(), 3);
        assert_eq!(params.get_decimal("a").unwrap(), 1);
        assert_eq!(params.get_decimal("b").unwrap(), 2);
        assert_eq!(params.get_decimal("c").unwrap(), 3);
    }

    //
    // `Display` tests
    //

    #[test]
    #[cfg(feature = "alloc")]
    fn display_empty() {
        let params = ParamsString::new();
        assert_eq!(params.to_string(), "");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn display_one() {
        let params = ParamsString::from_str("a=1").unwrap();
        assert_eq!(params.to_string(), "a=1");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn display_many() {
        let params = ParamsString::from_str("a=1,b=2,c=3").unwrap();
        assert_eq!(params.to_string(), "a=1,b=2,c=3");
    }
}
