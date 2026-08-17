use crate::de;
use crate::error::{self, Error, ErrorImpl};
use serde::de::{Unexpected, Visitor};
use serde::{forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

/// Represents a YAML number, whether integer or floating point.
#[derive(Clone, PartialEq, PartialOrd)]
pub struct Number {
    n: N,
}

// "N" is a prefix of "NegInt"... this is a false positive.
// https://github.com/Manishearth/rust-clippy/issues/1241
#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone)]
enum N {
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// May be infinite or NaN.
    Float(f64),
}

impl Number {
    /// Returns true if the `Number` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Number on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let big = i64::MAX as u64 + 10;
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 64
    /// b: 9223372036854775817
    /// c: 256.0
    /// "#)?;
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::PosInt(v) => v <= i64::max_value() as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` is an integer between zero and `u64::MAX`.
    ///
    /// For any Number on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 64
    /// b: -64
    /// c: 256.0
    /// "#)?;
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::PosInt(_) => true,
            N::NegInt(_) | N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` can be represented by f64.
    ///
    /// For any Number on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 256.0
    /// b: 64
    /// c: -64
    /// "#)?;
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::PosInt(_) | N::NegInt(_) => false,
        }
    }

    /// If the `Number` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let big = i64::MAX as u64 + 10;
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 64
    /// b: 9223372036854775817
    /// c: 256.0
    /// "#)?;
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::PosInt(n) => {
                if n <= i64::max_value() as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::NegInt(n) => Some(n),
            N::Float(_) => None,
        }
    }

    /// If the `Number` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 64
    /// b: -64
    /// c: 256.0
    /// "#)?;
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::PosInt(n) => Some(n),
            N::NegInt(_) | N::Float(_) => None,
        }
    }

    /// Represents the number as f64 if possible. Returns None otherwise.
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let v: serde_yaml::Value = serde_yaml::from_str(r#"
    /// a: 256.0
    /// b: 64
    /// c: -64
    /// "#)?;
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # fn main() -> serde_yaml::Result<()> {
    /// let v: serde_yaml::Value = serde_yaml::from_str(".inf")?;
    /// assert_eq!(v.as_f64(), Some(f64::INFINITY));
    ///
    /// let v: serde_yaml::Value = serde_yaml::from_str("-.inf")?;
    /// assert_eq!(v.as_f64(), Some(f64::NEG_INFINITY));
    ///
    /// let v: serde_yaml::Value = serde_yaml::from_str(".nan")?;
    /// assert!(v.as_f64().unwrap().is_nan());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }

    /// Returns true if this value is NaN and false otherwise.
    ///
    /// ```
    /// # use serde_yaml::Number;
    /// #
    /// assert!(!Number::from(256.0).is_nan());
    ///
    /// assert!(Number::from(f64::NAN).is_nan());
    ///
    /// assert!(!Number::from(f64::INFINITY).is_nan());
    ///
    /// assert!(!Number::from(f64::NEG_INFINITY).is_nan());
    ///
    /// assert!(!Number::from(1).is_nan());
    /// ```
    #[inline]
    pub fn is_nan(&self) -> bool {
        match self.n {
            N::PosInt(_) | N::NegInt(_) => false,
            N::Float(f) => f.is_nan(),
        }
    }

    /// Returns true if this value is positive infinity or negative infinity and
    /// false otherwise.
    ///
    /// ```
    /// # use serde_yaml::Number;
    /// #
    /// assert!(!Number::from(256.0).is_infinite());
    ///
    /// assert!(!Number::from(f64::NAN).is_infinite());
    ///
    /// assert!(Number::from(f64::INFINITY).is_infinite());
    ///
    /// assert!(Number::from(f64::NEG_INFINITY).is_infinite());
    ///
    /// assert!(!Number::from(1).is_infinite());
    /// ```
    #[inline]
    pub fn is_infinite(&self) -> bool {
        match self.n {
            N::PosInt(_) | N::NegInt(_) => false,
            N::Float(f) => f.is_infinite(),
        }
    }

    /// Returns true if this number is neither infinite nor NaN.
    ///
    /// ```
    /// # use serde_yaml::Number;
    /// #
    /// assert!(Number::from(256.0).is_finite());
    ///
    /// assert!(!Number::from(f64::NAN).is_finite());
    ///
    /// assert!(!Number::from(f64::INFINITY).is_finite());
    ///
    /// assert!(!Number::from(f64::NEG_INFINITY).is_finite());
    ///
    /// assert!(Number::from(1).is_finite());
    /// ```
    #[inline]
    pub fn is_finite(&self) -> bool {
        match self.n {
            N::PosInt(_) | N::NegInt(_) => true,
            N::Float(f) => f.is_finite(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::PosInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            N::NegInt(i) => formatter.write_str(itoa::Buffer::new().format(i)),
            N::Float(f) if f.is_nan() => formatter.write_str(".nan"),
            N::Float(f) if f.is_infinite() => {
                if f.is_sign_negative() {
                    formatter.write_str("-.inf")
                } else {
                    formatter.write_str(".inf")
                }
            }
            N::Float(f) => formatter.write_str(ryu::Buffer::new().format_finite(f)),
        }
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(repr: &str) -> Result<Self, Self::Err> {
        if let Ok(result) = de::visit_int(NumberVisitor, repr) {
            return result;
        }
        if !de::digits_but_not_number(repr) {
            if let Some(float) = de::parse_f64(repr) {
                return Ok(float.into());
            }
        }
        Err(error::new(ErrorImpl::FailedToParseNumber))
    }
}

impl PartialEq for N {
    fn eq(&self, other: &N) -> bool {
        match (*self, *other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    // YAML only has one NaN;
                    // the bit representation isn't preserved
                    true
                } else {
                    a == b
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    // YAML only has one NaN
                    Some(Ordering::Equal)
                } else {
                    a.partial_cmp(&b)
                }
            }
            _ => Some(self.total_cmp(other)),
        }
    }
}

impl N {
    fn total_cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (N::PosInt(a), N::PosInt(b)) => a.cmp(&b),
            (N::NegInt(a), N::NegInt(b)) => a.cmp(&b),
            // negint is always less than zero
            (N::NegInt(_), N::PosInt(_)) => Ordering::Less,
            (N::PosInt(_), N::NegInt(_)) => Ordering::Greater,
            (N::Float(a), N::Float(b)) => a.partial_cmp(&b).unwrap_or_else(|| {
                // arbitrarily sort the NaN last
                if !a.is_nan() {
                    Ordering::Less
                } else if !b.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }),
            // arbitrarily sort integers below floats
            // FIXME: maybe something more sensible?
            (_, N::Float(_)) => Ordering::Less,
            (N::Float(_), _) => Ordering::Greater,
        }
    }
}

impl Number {
    pub(crate) fn total_cmp(&self, other: &Self) -> Ordering {
        self.n.total_cmp(&other.n)
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(i) => serializer.serialize_u64(i),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

struct NumberVisitor;

impl<'de> Visitor<'de> for NumberVisitor {
    type Value = Number;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number")
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
        Ok(value.into())
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
        Ok(value.into())
    }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
        Ok(value.into())
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserializer<'de> for Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::PosInt(i) => visitor.visit_u64(i),
            N::NegInt(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a> Deserializer<'de> for &'a Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::PosInt(i) => visitor.visit_u64(i),
            N::NegInt(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

macro_rules! from_signed {
    ($($signed_ty:ident)*) => {
        $(
            impl From<$signed_ty> for Number {
                #[inline]
                #[allow(clippy::cast_sign_loss)]
                fn from(i: $signed_ty) -> Self {
                    if i < 0 {
                        Number { n: N::NegInt(i as i64) }
                    } else {
                        Number { n: N::PosInt(i as u64) }
                    }
                }
            }
        )*
    };
}

macro_rules! from_unsigned {
    ($($unsigned_ty:ident)*) => {
        $(
            impl From<$unsigned_ty> for Number {
                #[inline]
                fn from(u: $unsigned_ty) -> Self {
                    Number { n: N::PosInt(u as u64) }
                }
            }
        )*
    };
}

from_signed!(i8 i16 i32 i64 isize);
from_unsigned!(u8 u16 u32 u64 usize);

impl From<f32> for Number {
    fn from(f: f32) -> Self {
        Number::from(f as f64)
    }
}

impl From<f64> for Number {
    fn from(mut f: f64) -> Self {
        if f.is_nan() {
            // Destroy NaN sign, signaling, and payload. YAML only has one NaN.
            f = f64::NAN.copysign(1.0);
        }
        Number { n: N::Float(f) }
    }
}

// This is fine, because we don't _really_ implement hash for floats
// all other hash functions should work as expected
#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.n {
            N::Float(_) => {
                // you should feel bad for using f64 as a map key
                3.hash(state);
            }
            N::PosInt(u) => u.hash(state),
            N::NegInt(i) => i.hash(state),
        }
    }
}

pub(crate) fn unexpected(number: &Number) -> Unexpected {
    match number.n {
        N::PosInt(u) => Unexpected::Unsigned(u),
        N::NegInt(i) => Unexpected::Signed(i),
        N::Float(f) => Unexpected::Float(f),
    }
}
