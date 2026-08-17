//! Container for all SQL value types.

use std::borrow::Cow;

#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-json")]
use std::str::from_utf8;

#[cfg(feature = "with-chrono")]
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[cfg(feature = "with-time")]
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
use uuid::Uuid;

#[cfg(feature = "with-ipnetwork")]
use ipnetwork::IpNetwork;

#[cfg(feature = "with-ipnetwork")]
use std::net::IpAddr;

#[cfg(feature = "with-mac_address")]
use mac_address::MacAddress;

use crate::{ColumnType, CommonSqlQueryBuilder, QueryBuilder, StringLen};

/// [`Value`] types variant for Postgres array
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ArrayType {
    Bool,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    TinyUnsigned,
    SmallUnsigned,
    Unsigned,
    BigUnsigned,
    Float,
    Double,
    String,
    Char,
    Bytes,

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal,

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime,

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone,

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid,

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal,

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal,

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork,

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress,
}

/// Value variants
///
/// We want the inner Value to be exactly 1 pointer sized, so anything larger should be boxed.
///
/// If the `hashable-value` feature is enabled, NaN == NaN, which contradicts Rust's built-in
/// implementation of NaN != NaN.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "hashable-value"), derive(PartialEq))]
pub enum Value {
    Bool(Option<bool>),
    TinyInt(Option<i8>),
    SmallInt(Option<i16>),
    Int(Option<i32>),
    BigInt(Option<i64>),
    TinyUnsigned(Option<u8>),
    SmallUnsigned(Option<u16>),
    Unsigned(Option<u32>),
    BigUnsigned(Option<u64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<Box<String>>),
    Char(Option<char>),

    #[allow(clippy::box_collection)]
    Bytes(Option<Box<Vec<u8>>>),

    #[cfg(feature = "with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Option<Box<Json>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDate(Option<Box<NaiveDate>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoTime(Option<Box<NaiveTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTime(Option<Box<NaiveDateTime>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeUtc(Option<Box<DateTime<Utc>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeLocal(Option<Box<DateTime<Local>>>),

    #[cfg(feature = "with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    ChronoDateTimeWithTimeZone(Option<Box<DateTime<FixedOffset>>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDate(Option<Box<time::Date>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeTime(Option<Box<time::Time>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTime(Option<Box<PrimitiveDateTime>>),

    #[cfg(feature = "with-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
    TimeDateTimeWithTimeZone(Option<Box<OffsetDateTime>>),

    #[cfg(feature = "with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Option<Box<Uuid>>),

    #[cfg(feature = "with-rust_decimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
    Decimal(Option<Box<Decimal>>),

    #[cfg(feature = "with-bigdecimal")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
    BigDecimal(Option<Box<BigDecimal>>),

    #[cfg(feature = "postgres-array")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
    Array(ArrayType, Option<Box<Vec<Value>>>),

    #[cfg(feature = "postgres-vector")]
    #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
    Vector(Option<Box<pgvector::Vector>>),

    #[cfg(feature = "with-ipnetwork")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
    IpNetwork(Option<Box<IpNetwork>>),

    #[cfg(feature = "with-mac_address")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
    MacAddress(Option<Box<MacAddress>>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CommonSqlQueryBuilder.value_to_string(self))
    }
}

pub trait ValueType: Sized {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr>;

    fn unwrap(v: Value) -> Self {
        Self::try_from(v).unwrap()
    }

    fn expect(v: Value, msg: &str) -> Self {
        Self::try_from(v).expect(msg)
    }

    fn is_option() -> bool {
        false
    }

    fn type_name() -> String;

    fn array_type() -> ArrayType;

    fn column_type() -> ColumnType;

    fn enum_type_name() -> Option<&'static str> {
        None
    }
}

#[derive(Debug)]
pub struct ValueTypeErr;

impl std::error::Error for ValueTypeErr {}

impl std::fmt::Display for ValueTypeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value type mismatch")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hashable-value", derive(Hash, Eq))]
pub enum ValueTuple {
    One(Value),
    Two(Value, Value),
    Three(Value, Value, Value),
    Many(Vec<Value>),
}

pub trait IntoValueTuple {
    fn into_value_tuple(self) -> ValueTuple;
}

pub trait FromValueTuple: Sized {
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple;
}

pub trait Nullable {
    fn null() -> Value;
}

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }

    pub fn expect<T>(self, msg: &str) -> T
    where
        T: ValueType,
    {
        T::expect(self, msg)
    }

    /// Get the null variant of self
    ///
    /// ```
    /// use sea_query::Value;
    ///
    /// let v = Value::Int(Some(2));
    /// let n = v.as_null();
    ///
    /// assert_eq!(n, Value::Int(None));
    ///
    /// // one liner:
    /// assert_eq!(Into::<Value>::into(2.2).as_null(), Value::Double(None));
    /// ```
    pub fn as_null(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(None),
            Self::TinyInt(_) => Self::TinyInt(None),
            Self::SmallInt(_) => Self::SmallInt(None),
            Self::Int(_) => Self::Int(None),
            Self::BigInt(_) => Self::BigInt(None),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(None),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(None),
            Self::Unsigned(_) => Self::Unsigned(None),
            Self::BigUnsigned(_) => Self::BigUnsigned(None),
            Self::Float(_) => Self::Float(None),
            Self::Double(_) => Self::Double(None),
            Self::String(_) => Self::String(None),
            Self::Char(_) => Self::Char(None),
            Self::Bytes(_) => Self::Bytes(None),

            #[cfg(feature = "with-json")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
            Self::Json(_) => Self::Json(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDate(_) => Self::ChronoDate(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoTime(_) => Self::ChronoTime(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTime(_) => Self::ChronoDateTime(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(None),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeWithTimeZone(_) => Self::ChronoDateTimeWithTimeZone(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDate(_) => Self::TimeDate(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeTime(_) => Self::TimeTime(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTime(_) => Self::TimeDateTime(None),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTimeWithTimeZone(_) => Self::TimeDateTimeWithTimeZone(None),

            #[cfg(feature = "with-uuid")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
            Self::Uuid(_) => Self::Uuid(None),

            #[cfg(feature = "with-rust_decimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
            Self::Decimal(_) => Self::Decimal(None),

            #[cfg(feature = "with-bigdecimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
            Self::BigDecimal(_) => Self::BigDecimal(None),

            #[cfg(feature = "postgres-array")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
            Self::Array(ty, _) => Self::Array(ty.clone(), None),

            #[cfg(feature = "postgres-vector")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
            Self::Vector(_) => Self::Vector(None),

            #[cfg(feature = "with-ipnetwork")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
            Self::IpNetwork(_) => Self::IpNetwork(None),

            #[cfg(feature = "with-mac_address")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
            Self::MacAddress(_) => Self::MacAddress(None),
        }
    }

    /// Get a default value of self's type
    ///
    /// ```
    /// use sea_query::Value;
    ///
    /// let v = Value::Int(None);
    /// let n = v.dummy_value();
    /// assert_eq!(n, Value::Int(Some(0)));
    /// ```
    pub fn dummy_value(&self) -> Self {
        match self {
            Self::Bool(_) => Self::Bool(Some(Default::default())),
            Self::TinyInt(_) => Self::TinyInt(Some(Default::default())),
            Self::SmallInt(_) => Self::SmallInt(Some(Default::default())),
            Self::Int(_) => Self::Int(Some(Default::default())),
            Self::BigInt(_) => Self::BigInt(Some(Default::default())),
            Self::TinyUnsigned(_) => Self::TinyUnsigned(Some(Default::default())),
            Self::SmallUnsigned(_) => Self::SmallUnsigned(Some(Default::default())),
            Self::Unsigned(_) => Self::Unsigned(Some(Default::default())),
            Self::BigUnsigned(_) => Self::BigUnsigned(Some(Default::default())),
            Self::Float(_) => Self::Float(Some(Default::default())),
            Self::Double(_) => Self::Double(Some(Default::default())),
            Self::String(_) => Self::String(Some(Default::default())),
            Self::Char(_) => Self::Char(Some(Default::default())),
            Self::Bytes(_) => Self::Bytes(Some(Default::default())),

            #[cfg(feature = "with-json")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
            Self::Json(_) => Self::Json(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDate(_) => Self::ChronoDate(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoTime(_) => Self::ChronoTime(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTime(_) => Self::ChronoDateTime(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeUtc(_) => Self::ChronoDateTimeUtc(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeLocal(_) => Self::ChronoDateTimeLocal(Some(Default::default())),

            #[cfg(feature = "with-chrono")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
            Self::ChronoDateTimeWithTimeZone(_) => {
                Self::ChronoDateTimeWithTimeZone(Some(Default::default()))
            }

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDate(_) => Self::TimeDate(Some(time::Date::MIN.into())),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeTime(_) => Self::TimeTime(Some(time::Time::MIDNIGHT.into())),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTime(_) => Self::TimeDateTime(Some(PrimitiveDateTime::MIN.into())),

            #[cfg(feature = "with-time")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
            Self::TimeDateTimeWithTimeZone(_) => {
                Self::TimeDateTimeWithTimeZone(Some(OffsetDateTime::UNIX_EPOCH.into()))
            }

            #[cfg(feature = "with-uuid")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
            Self::Uuid(_) => Self::Uuid(Some(Default::default())),

            #[cfg(feature = "with-rust_decimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
            Self::Decimal(_) => Self::Decimal(Some(Default::default())),

            #[cfg(feature = "with-bigdecimal")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
            Self::BigDecimal(_) => Self::BigDecimal(Some(Default::default())),

            #[cfg(feature = "postgres-array")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
            Self::Array(ty, _) => Self::Array(ty.clone(), Some(Default::default())),

            #[cfg(feature = "postgres-vector")]
            #[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
            Self::Vector(_) => Self::Vector(Some(Box::new(vec![].into()))),

            #[cfg(feature = "with-ipnetwork")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
            Self::IpNetwork(_) => Self::IpNetwork(Some(Box::new("0.0.0.0".parse().unwrap()))),

            #[cfg(feature = "with-mac_address")]
            #[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
            Self::MacAddress(_) => Self::MacAddress(Some(Default::default())),
        }
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(x))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}

macro_rules! type_to_box_value {
    ( $type: ty, $name: ident, $col_type: expr ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Some(Box::new(x)))
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::$name(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::$name(Some(x)) => Ok(*x),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::$name
            }

            fn column_type() -> ColumnType {
                use ColumnType::*;
                $col_type
            }
        }
    };
}

type_to_value!(bool, Bool, Boolean);
type_to_value!(i8, TinyInt, TinyInteger);
type_to_value!(i16, SmallInt, SmallInteger);
type_to_value!(i32, Int, Integer);
type_to_value!(i64, BigInt, BigInteger);
type_to_value!(u8, TinyUnsigned, TinyUnsigned);
type_to_value!(u16, SmallUnsigned, SmallUnsigned);
type_to_value!(u32, Unsigned, Unsigned);
type_to_value!(u64, BigUnsigned, BigUnsigned);
type_to_value!(f32, Float, Float);
type_to_value!(f64, Double, Double);
type_to_value!(char, Char, Char(None));

impl From<&[u8]> for Value {
    fn from(x: &[u8]) -> Value {
        Value::Bytes(Some(Box::<Vec<u8>>::new(x.into())))
    }
}

impl From<&str> for Value {
    fn from(x: &str) -> Value {
        let string: String = x.into();
        Value::String(Some(Box::new(string)))
    }
}

impl From<&String> for Value {
    fn from(x: &String) -> Value {
        let string: String = x.into();
        Value::String(Some(Box::new(string)))
    }
}

impl Nullable for &str {
    fn null() -> Value {
        Value::String(None)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value> + Nullable,
{
    fn from(x: Option<T>) -> Value {
        match x {
            Some(v) => v.into(),
            None => T::null(),
        }
    }
}

impl<T> ValueType for Option<T>
where
    T: ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if v == T::null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from(v)?))
        }
    }

    fn is_option() -> bool {
        true
    }

    fn type_name() -> String {
        format!("Option<{}>", T::type_name())
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}

impl From<Cow<'_, str>> for Value {
    fn from(x: Cow<'_, str>) -> Value {
        x.into_owned().into()
    }
}

impl ValueType for Cow<'_, str> {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(x)) => Ok((*x).into()),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "Cow<str>".into()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::None)
    }
}

type_to_box_value!(Vec<u8>, Bytes, VarBinary(StringLen::None));
type_to_box_value!(String, String, String(StringLen::None));

#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
mod with_json {
    use super::*;

    type_to_box_value!(Json, Json, Json);
}

#[cfg(feature = "with-chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
mod with_chrono {
    use super::*;
    use chrono::{Local, Offset, Utc};

    type_to_box_value!(NaiveDate, ChronoDate, Date);
    type_to_box_value!(NaiveTime, ChronoTime, Time);
    type_to_box_value!(NaiveDateTime, ChronoDateTime, DateTime);

    impl From<DateTime<Utc>> for Value {
        fn from(v: DateTime<Utc>) -> Value {
            Value::ChronoDateTimeUtc(Some(Box::new(v)))
        }
    }

    impl From<DateTime<Local>> for Value {
        fn from(v: DateTime<Local>) -> Value {
            Value::ChronoDateTimeLocal(Some(Box::new(v)))
        }
    }

    impl From<DateTime<FixedOffset>> for Value {
        fn from(x: DateTime<FixedOffset>) -> Value {
            let v =
                DateTime::<FixedOffset>::from_naive_utc_and_offset(x.naive_utc(), x.offset().fix());
            Value::ChronoDateTimeWithTimeZone(Some(Box::new(v)))
        }
    }

    impl Nullable for DateTime<Utc> {
        fn null() -> Value {
            Value::ChronoDateTimeUtc(None)
        }
    }

    impl ValueType for DateTime<Utc> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeUtc(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<Utc>).to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::ChronoDateTimeUtc
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone
        }
    }

    impl Nullable for DateTime<Local> {
        fn null() -> Value {
            Value::ChronoDateTimeLocal(None)
        }
    }

    impl ValueType for DateTime<Local> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeLocal(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<Local>).to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::ChronoDateTimeLocal
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone
        }
    }

    impl Nullable for DateTime<FixedOffset> {
        fn null() -> Value {
            Value::ChronoDateTimeWithTimeZone(None)
        }
    }

    impl ValueType for DateTime<FixedOffset> {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::ChronoDateTimeWithTimeZone(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(DateTime<FixedOffset>).to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::ChronoDateTimeWithTimeZone
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone
        }
    }
}

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
pub mod time_format {
    use time::format_description::FormatItem;
    use time::macros::format_description;

    pub static FORMAT_DATE: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");
    pub static FORMAT_TIME: &[FormatItem<'static>] =
        format_description!("[hour]:[minute]:[second].[subsecond digits:6]");
    pub static FORMAT_DATETIME: &[FormatItem<'static>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");
    pub static FORMAT_DATETIME_TZ: &[FormatItem<'static>] = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6] [offset_hour sign:mandatory]:[offset_minute]"
    );
}

#[cfg(feature = "with-time")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-time")))]
mod with_time {
    use super::*;

    type_to_box_value!(time::Date, TimeDate, Date);
    type_to_box_value!(time::Time, TimeTime, Time);
    type_to_box_value!(PrimitiveDateTime, TimeDateTime, DateTime);

    impl From<OffsetDateTime> for Value {
        fn from(v: OffsetDateTime) -> Value {
            Value::TimeDateTimeWithTimeZone(Some(Box::new(v)))
        }
    }

    impl Nullable for OffsetDateTime {
        fn null() -> Value {
            Value::TimeDateTimeWithTimeZone(None)
        }
    }

    impl ValueType for OffsetDateTime {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::TimeDateTimeWithTimeZone(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(OffsetDateTime).to_owned()
        }

        fn array_type() -> ArrayType {
            ArrayType::TimeDateTimeWithTimeZone
        }

        fn column_type() -> ColumnType {
            ColumnType::TimestampWithTimeZone
        }
    }
}

#[cfg(feature = "with-rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-rust_decimal")))]
mod with_rust_decimal {
    use super::*;

    type_to_box_value!(Decimal, Decimal, Decimal(None));
}

#[cfg(feature = "with-bigdecimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-bigdecimal")))]
mod with_bigdecimal {
    use super::*;

    type_to_box_value!(BigDecimal, BigDecimal, Decimal(None));
}

#[cfg(feature = "with-uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
mod with_uuid {
    use super::*;

    type_to_box_value!(Uuid, Uuid, Uuid);

    macro_rules! fmt_uuid_to_box_value {
        ( $type: ty, $conversion_fn: ident ) => {
            impl From<$type> for Value {
                fn from(x: $type) -> Value {
                    Value::Uuid(Some(Box::new(x.into_uuid())))
                }
            }

            impl Nullable for $type {
                fn null() -> Value {
                    Value::Uuid(None)
                }
            }

            impl ValueType for $type {
                fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                    match v {
                        Value::Uuid(Some(x)) => Ok(x.$conversion_fn()),
                        _ => Err(ValueTypeErr),
                    }
                }

                fn type_name() -> String {
                    stringify!($type).to_owned()
                }

                fn array_type() -> ArrayType {
                    ArrayType::Uuid
                }

                fn column_type() -> ColumnType {
                    ColumnType::Uuid
                }
            }
        };
    }

    fmt_uuid_to_box_value!(uuid::fmt::Braced, braced);
    fmt_uuid_to_box_value!(uuid::fmt::Hyphenated, hyphenated);
    fmt_uuid_to_box_value!(uuid::fmt::Simple, simple);
    fmt_uuid_to_box_value!(uuid::fmt::Urn, urn);
}

#[cfg(feature = "with-ipnetwork")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-ipnetwork")))]
mod with_ipnetwork {
    use super::*;

    type_to_box_value!(IpNetwork, IpNetwork, Inet);
}

#[cfg(feature = "with-mac_address")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-mac_address")))]
mod with_mac_address {
    use super::*;

    type_to_box_value!(MacAddress, MacAddress, MacAddr);
}

#[cfg(feature = "postgres-array")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-array")))]
pub mod with_array {
    use super::*;
    use crate::RcOrArc;

    // We only implement conversion from Vec<T> to Array when T is not u8.
    // This is because for u8's case, there is already conversion to Byte defined above.
    // TODO When negative trait becomes a stable feature, following code can be much shorter.
    pub trait NotU8 {}

    impl NotU8 for bool {}
    impl NotU8 for i8 {}
    impl NotU8 for i16 {}
    impl NotU8 for i32 {}
    impl NotU8 for i64 {}
    impl NotU8 for u16 {}
    impl NotU8 for u32 {}
    impl NotU8 for u64 {}
    impl NotU8 for f32 {}
    impl NotU8 for f64 {}
    impl NotU8 for char {}
    impl NotU8 for String {}
    impl NotU8 for Vec<u8> {}

    // TODO impl<T: NotU8> NotU8 for Option<T> {}

    #[cfg(feature = "with-json")]
    impl NotU8 for Json {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveDate {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveTime {}

    #[cfg(feature = "with-chrono")]
    impl NotU8 for NaiveDateTime {}

    #[cfg(feature = "with-chrono")]
    impl<Tz> NotU8 for DateTime<Tz> where Tz: chrono::TimeZone {}

    #[cfg(feature = "with-time")]
    impl NotU8 for time::Date {}

    #[cfg(feature = "with-time")]
    impl NotU8 for time::Time {}

    #[cfg(feature = "with-time")]
    impl NotU8 for PrimitiveDateTime {}

    #[cfg(feature = "with-time")]
    impl NotU8 for OffsetDateTime {}

    #[cfg(feature = "with-rust_decimal")]
    impl NotU8 for Decimal {}

    #[cfg(feature = "with-bigdecimal")]
    impl NotU8 for BigDecimal {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for Uuid {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for uuid::fmt::Braced {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for uuid::fmt::Hyphenated {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for uuid::fmt::Simple {}

    #[cfg(feature = "with-uuid")]
    impl NotU8 for uuid::fmt::Urn {}

    #[cfg(feature = "with-ipnetwork")]
    impl NotU8 for IpNetwork {}

    #[cfg(feature = "with-mac_address")]
    impl NotU8 for MacAddress {}

    impl<T> From<Vec<T>> for Value
    where
        T: Into<Value> + NotU8 + ValueType,
    {
        fn from(x: Vec<T>) -> Value {
            Value::Array(
                T::array_type(),
                Some(Box::new(x.into_iter().map(|e| e.into()).collect())),
            )
        }
    }

    impl<T> Nullable for Vec<T>
    where
        T: Into<Value> + NotU8 + ValueType,
    {
        fn null() -> Value {
            Value::Array(T::array_type(), None)
        }
    }

    impl<T> ValueType for Vec<T>
    where
        T: NotU8 + ValueType,
    {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::Array(ty, Some(v)) if T::array_type() == ty => {
                    Ok(v.into_iter().map(|e| e.unwrap()).collect())
                }
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(Vec<T>).to_owned()
        }

        fn array_type() -> ArrayType {
            T::array_type()
        }

        fn column_type() -> ColumnType {
            use ColumnType::*;
            Array(RcOrArc::new(T::column_type()))
        }
    }
}

#[cfg(feature = "postgres-vector")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres-vector")))]
pub mod with_vector {
    use super::*;

    impl From<pgvector::Vector> for Value {
        fn from(x: pgvector::Vector) -> Value {
            Value::Vector(Some(Box::new(x)))
        }
    }

    impl Nullable for pgvector::Vector {
        fn null() -> Value {
            Value::Vector(None)
        }
    }

    impl ValueType for pgvector::Vector {
        fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
            match v {
                Value::Vector(Some(x)) => Ok(*x),
                _ => Err(ValueTypeErr),
            }
        }

        fn type_name() -> String {
            stringify!(Vector).to_owned()
        }

        fn array_type() -> ArrayType {
            unimplemented!("Vector does not have array type")
        }

        fn column_type() -> ColumnType {
            ColumnType::Vector(None)
        }
    }
}

#[allow(unused_macros)]
macro_rules! box_to_opt_ref {
    ( $v: expr ) => {
        match $v {
            Some(v) => Some(v.as_ref()),
            None => None,
        }
    };
}

#[cfg(feature = "with-json")]
impl Value {
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub fn as_ref_json(&self) -> Option<&Json> {
        match self {
            Self::Json(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Json"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date(&self) -> bool {
        matches!(self, Self::ChronoDate(_))
    }

    pub fn as_ref_chrono_date(&self) -> Option<&NaiveDate> {
        match self {
            Self::ChronoDate(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDate"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date(&self) -> bool {
        matches!(self, Self::TimeDate(_))
    }

    pub fn as_ref_time_date(&self) -> Option<&time::Date> {
        match self {
            Self::TimeDate(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDate"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_time(&self) -> bool {
        matches!(self, Self::ChronoTime(_))
    }

    pub fn as_ref_chrono_time(&self) -> Option<&NaiveTime> {
        match self {
            Self::ChronoTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoTime"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_time(&self) -> bool {
        matches!(self, Self::TimeTime(_))
    }

    pub fn as_ref_time_time(&self) -> Option<&time::Time> {
        match self {
            Self::TimeTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeTime"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time(&self) -> bool {
        matches!(self, Self::ChronoDateTime(_))
    }

    pub fn as_ref_chrono_date_time(&self) -> Option<&NaiveDateTime> {
        match self {
            Self::ChronoDateTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTime"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date_time(&self) -> bool {
        matches!(self, Self::TimeDateTime(_))
    }

    pub fn as_ref_time_date_time(&self) -> Option<&PrimitiveDateTime> {
        match self {
            Self::TimeDateTime(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDateTime"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_utc(&self) -> bool {
        matches!(self, Self::ChronoDateTimeUtc(_))
    }

    pub fn as_ref_chrono_date_time_utc(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::ChronoDateTimeUtc(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeUtc"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_local(&self) -> bool {
        matches!(self, Self::ChronoDateTimeLocal(_))
    }

    pub fn as_ref_chrono_date_time_local(&self) -> Option<&DateTime<Local>> {
        match self {
            Self::ChronoDateTimeLocal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeLocal"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn is_chrono_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::ChronoDateTimeWithTimeZone(_))
    }

    pub fn as_ref_chrono_date_time_with_time_zone(&self) -> Option<&DateTime<FixedOffset>> {
        match self {
            Self::ChronoDateTimeWithTimeZone(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::ChronoDateTimeWithTimeZone"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn is_time_date_time_with_time_zone(&self) -> bool {
        matches!(self, Self::TimeDateTimeWithTimeZone(_))
    }

    pub fn as_ref_time_date_time_with_time_zone(&self) -> Option<&OffsetDateTime> {
        match self {
            Self::TimeDateTimeWithTimeZone(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::TimeDateTimeWithTimeZone"),
        }
    }
}

#[cfg(feature = "with-chrono")]
impl Value {
    pub fn chrono_as_naive_utc_in_string(&self) -> Option<String> {
        match self {
            Self::ChronoDate(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoDateTime(v) => v.as_ref().map(|v| v.to_string()),
            Self::ChronoDateTimeUtc(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            Self::ChronoDateTimeLocal(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            Self::ChronoDateTimeWithTimeZone(v) => v.as_ref().map(|v| v.naive_utc().to_string()),
            _ => panic!("not chrono Value"),
        }
    }
}

#[cfg(feature = "with-time")]
impl Value {
    pub fn time_as_naive_utc_in_string(&self) -> Option<String> {
        match self {
            Self::TimeDate(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_DATE).ok()),
            Self::TimeTime(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_TIME).ok()),
            Self::TimeDateTime(v) => v
                .as_ref()
                .and_then(|v| v.format(time_format::FORMAT_DATETIME).ok()),
            Self::TimeDateTimeWithTimeZone(v) => v.as_ref().and_then(|v| {
                v.to_offset(time::macros::offset!(UTC))
                    .format(time_format::FORMAT_DATETIME_TZ)
                    .ok()
            }),
            _ => panic!("not time Value"),
        }
    }
}

#[cfg(feature = "with-rust_decimal")]
impl Value {
    pub fn is_decimal(&self) -> bool {
        matches!(self, Self::Decimal(_))
    }

    pub fn as_ref_decimal(&self) -> Option<&Decimal> {
        match self {
            Self::Decimal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Decimal"),
        }
    }

    pub fn decimal_to_f64(&self) -> Option<f64> {
        use rust_decimal::prelude::ToPrimitive;

        self.as_ref_decimal().map(|d| d.to_f64().unwrap())
    }
}

#[cfg(feature = "with-bigdecimal")]
impl Value {
    pub fn is_big_decimal(&self) -> bool {
        matches!(self, Self::BigDecimal(_))
    }

    pub fn as_ref_big_decimal(&self) -> Option<&BigDecimal> {
        match self {
            Self::BigDecimal(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::BigDecimal"),
        }
    }

    pub fn big_decimal_to_f64(&self) -> Option<f64> {
        use bigdecimal::ToPrimitive;
        self.as_ref_big_decimal().map(|d| d.to_f64().unwrap())
    }
}

#[cfg(feature = "with-uuid")]
impl Value {
    pub fn is_uuid(&self) -> bool {
        matches!(self, Self::Uuid(_))
    }
    pub fn as_ref_uuid(&self) -> Option<&Uuid> {
        match self {
            Self::Uuid(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Uuid"),
        }
    }
}

#[cfg(feature = "postgres-array")]
impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn as_ref_array(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Array(_, v) => box_to_opt_ref!(v),
            _ => panic!("not Value::Array"),
        }
    }
}

#[cfg(feature = "with-ipnetwork")]
impl Value {
    pub fn is_ipnetwork(&self) -> bool {
        matches!(self, Self::IpNetwork(_))
    }

    pub fn as_ref_ipnetwork(&self) -> Option<&IpNetwork> {
        match self {
            Self::IpNetwork(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::IpNetwork"),
        }
    }

    pub fn as_ipaddr(&self) -> Option<IpAddr> {
        match self {
            Self::IpNetwork(v) => v.clone().map(|v| v.network()),
            _ => panic!("not Value::IpNetwork"),
        }
    }
}

#[cfg(feature = "with-mac_address")]
impl Value {
    pub fn is_mac_address(&self) -> bool {
        matches!(self, Self::MacAddress(_))
    }

    pub fn as_ref_mac_address(&self) -> Option<&MacAddress> {
        match self {
            Self::MacAddress(v) => box_to_opt_ref!(v),
            _ => panic!("not Value::MacAddress"),
        }
    }
}

impl IntoIterator for ValueTuple {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ValueTuple::One(v) => vec![v].into_iter(),
            ValueTuple::Two(v, w) => vec![v, w].into_iter(),
            ValueTuple::Three(u, v, w) => vec![u, v, w].into_iter(),
            ValueTuple::Many(vec) => vec.into_iter(),
        }
    }
}

impl IntoValueTuple for ValueTuple {
    fn into_value_tuple(self) -> ValueTuple {
        self
    }
}

impl<V> IntoValueTuple for V
where
    V: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::One(self.into())
    }
}

impl<V, W> IntoValueTuple for (V, W)
where
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Two(self.0.into(), self.1.into())
    }
}

impl<U, V, W> IntoValueTuple for (U, V, W)
where
    U: Into<Value>,
    V: Into<Value>,
    W: Into<Value>,
{
    fn into_value_tuple(self) -> ValueTuple {
        ValueTuple::Three(self.0.into(), self.1.into(), self.2.into())
    }
}

macro_rules! impl_into_value_tuple {
    ( $($idx:tt : $T:ident),+ $(,)? ) => {
        impl< $($T),+ > IntoValueTuple for ( $($T),+ )
        where
            $($T: Into<Value>),+
        {
            fn into_value_tuple(self) -> ValueTuple {
                ValueTuple::Many(vec![
                    $(self.$idx.into()),+
                ])
            }
        }
    };
}

#[rustfmt::skip]
mod impl_into_value_tuple {
    use super::*;

    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9, 10:T10);
    impl_into_value_tuple!(0:T0, 1:T1, 2:T2, 3:T3, 4:T4, 5:T5, 6:T6, 7:T7, 8:T8, 9:T9, 10:T10, 11:T11);
}

impl<V> FromValueTuple for V
where
    V: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::One(u) => u.unwrap(),
            _ => panic!("not ValueTuple::One"),
        }
    }
}

impl<V, W> FromValueTuple for (V, W)
where
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Two(v, w) => (v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Two"),
        }
    }
}

impl<U, V, W> FromValueTuple for (U, V, W)
where
    U: Into<Value> + ValueType,
    V: Into<Value> + ValueType,
    W: Into<Value> + ValueType,
{
    fn from_value_tuple<I>(i: I) -> Self
    where
        I: IntoValueTuple,
    {
        match i.into_value_tuple() {
            ValueTuple::Three(u, v, w) => (u.unwrap(), v.unwrap(), w.unwrap()),
            _ => panic!("not ValueTuple::Three"),
        }
    }
}

macro_rules! impl_from_value_tuple {
    ( $len:expr, $($T:ident),+ $(,)? ) => {
        impl< $($T),+ > FromValueTuple for ( $($T),+ )
        where
            $($T: Into<Value> + ValueType),+
        {
            fn from_value_tuple<Z>(i: Z) -> Self
            where
                Z: IntoValueTuple,
            {
                match i.into_value_tuple() {
                    ValueTuple::Many(vec) if vec.len() == $len => {
                        let mut iter = vec.into_iter();
                        (
                            $(<$T as ValueType>::unwrap(iter.next().unwrap())),+
                        )
                    }
                    _ => panic!("not ValueTuple::Many with length of {}", $len),
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impl_from_value_tuple {
    use super::*;

    impl_from_value_tuple!( 4, T0, T1, T2, T3);
    impl_from_value_tuple!( 5, T0, T1, T2, T3, T4);
    impl_from_value_tuple!( 6, T0, T1, T2, T3, T4, T5);
    impl_from_value_tuple!( 7, T0, T1, T2, T3, T4, T5, T6);
    impl_from_value_tuple!( 8, T0, T1, T2, T3, T4, T5, T6, T7);
    impl_from_value_tuple!( 9, T0, T1, T2, T3, T4, T5, T6, T7, T8);
    impl_from_value_tuple!(10, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    impl_from_value_tuple!(11, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    impl_from_value_tuple!(12, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature = "with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn sea_value_to_json_value(value: &Value) -> Json {
    match value {
        Value::Bool(None)
        | Value::TinyInt(None)
        | Value::SmallInt(None)
        | Value::Int(None)
        | Value::BigInt(None)
        | Value::TinyUnsigned(None)
        | Value::SmallUnsigned(None)
        | Value::Unsigned(None)
        | Value::BigUnsigned(None)
        | Value::Float(None)
        | Value::Double(None)
        | Value::String(None)
        | Value::Char(None)
        | Value::Bytes(None)
        | Value::Json(None) => Json::Null,
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(None) => Json::Null,
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(None) => Json::Null,
        #[cfg(feature = "with-uuid")]
        Value::Uuid(None) => Json::Null,
        #[cfg(feature = "postgres-array")]
        Value::Array(_, None) => Json::Null,
        #[cfg(feature = "postgres-vector")]
        Value::Vector(None) => Json::Null,
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(None) => Json::Null,
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(None) => Json::Null,
        Value::Bool(Some(b)) => Json::Bool(*b),
        Value::TinyInt(Some(v)) => (*v).into(),
        Value::SmallInt(Some(v)) => (*v).into(),
        Value::Int(Some(v)) => (*v).into(),
        Value::BigInt(Some(v)) => (*v).into(),
        Value::TinyUnsigned(Some(v)) => (*v).into(),
        Value::SmallUnsigned(Some(v)) => (*v).into(),
        Value::Unsigned(Some(v)) => (*v).into(),
        Value::BigUnsigned(Some(v)) => (*v).into(),
        Value::Float(Some(v)) => (*v).into(),
        Value::Double(Some(v)) => (*v).into(),
        Value::String(Some(s)) => Json::String(s.as_ref().clone()),
        Value::Char(Some(v)) => Json::String(v.to_string()),
        Value::Bytes(Some(s)) => Json::String(from_utf8(s).unwrap().to_string()),
        Value::Json(Some(v)) => v.as_ref().clone(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeUtc(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-chrono")]
        Value::ChronoDateTimeLocal(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDate(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTime(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-time")]
        Value::TimeDateTimeWithTimeZone(_) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-rust_decimal")]
        Value::Decimal(Some(v)) => {
            use rust_decimal::prelude::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-bigdecimal")]
        Value::BigDecimal(Some(v)) => {
            use bigdecimal::ToPrimitive;
            v.as_ref().to_f64().unwrap().into()
        }
        #[cfg(feature = "with-uuid")]
        Value::Uuid(Some(v)) => Json::String(v.to_string()),
        #[cfg(feature = "postgres-array")]
        Value::Array(_, Some(v)) => {
            Json::Array(v.as_ref().iter().map(sea_value_to_json_value).collect())
        }
        #[cfg(feature = "postgres-vector")]
        Value::Vector(Some(v)) => Json::Array(v.as_slice().iter().map(|&v| v.into()).collect()),
        #[cfg(feature = "with-ipnetwork")]
        Value::IpNetwork(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
        #[cfg(feature = "with-mac_address")]
        Value::MacAddress(Some(_)) => CommonSqlQueryBuilder.value_to_string(value).into(),
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_value() {
        macro_rules! test_value {
            ( $type: ty, $val: literal ) => {
                let val: $type = $val;
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val);
            };
        }

        test_value!(u8, 255);
        test_value!(u16, 65535);
        test_value!(i8, 127);
        test_value!(i16, 32767);
        test_value!(i32, 1073741824);
        test_value!(i64, 8589934592);
    }

    #[test]
    fn test_option_value() {
        macro_rules! test_some_value {
            ( $type: ty, $val: literal ) => {
                let val: Option<$type> = Some($val);
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val.unwrap());
            };
        }

        macro_rules! test_none {
            ( $type: ty, $name: ident ) => {
                let val: Option<$type> = None;
                let v: Value = val.into();
                assert_eq!(v, Value::$name(None));
            };
        }

        test_some_value!(u8, 255);
        test_some_value!(u16, 65535);
        test_some_value!(i8, 127);
        test_some_value!(i16, 32767);
        test_some_value!(i32, 1073741824);
        test_some_value!(i64, 8589934592);

        test_none!(u8, TinyUnsigned);
        test_none!(u16, SmallUnsigned);
        test_none!(i8, TinyInt);
        test_none!(i16, SmallInt);
        test_none!(i32, Int);
        test_none!(i64, BigInt);
    }

    #[test]
    fn test_cow_value() {
        let val: Cow<str> = "hello".into();
        let val2 = val.clone();
        let v: Value = val.into();
        let out: Cow<str> = v.unwrap();
        assert_eq!(out, val2);
    }

    #[test]
    fn test_box_value() {
        let val: String = "hello".to_owned();
        let v: Value = val.clone().into();
        let out: String = v.unwrap();
        assert_eq!(out, val);
    }

    #[test]
    fn test_value_tuple() {
        assert_eq!(
            1i32.into_value_tuple(),
            ValueTuple::One(Value::Int(Some(1)))
        );
        assert_eq!(
            "b".into_value_tuple(),
            ValueTuple::One(Value::String(Some(Box::new("b".to_owned()))))
        );
        assert_eq!(
            (1i32, "b").into_value_tuple(),
            ValueTuple::Two(
                Value::Int(Some(1)),
                Value::String(Some(Box::new("b".to_owned())))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b").into_value_tuple(),
            ValueTuple::Three(
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned())))
            )
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8).into_value_tuple(),
            ValueTuple::Many(vec![
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123))
            ])
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8, 456u16).into_value_tuple(),
            ValueTuple::Many(vec![
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123)),
                Value::SmallUnsigned(Some(456))
            ])
        );
        assert_eq!(
            (1i32, 2.4f64, "b", 123u8, 456u16, 789u32).into_value_tuple(),
            ValueTuple::Many(vec![
                Value::Int(Some(1)),
                Value::Double(Some(2.4)),
                Value::String(Some(Box::new("b".to_owned()))),
                Value::TinyUnsigned(Some(123)),
                Value::SmallUnsigned(Some(456)),
                Value::Unsigned(Some(789))
            ])
        );
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_from_value_tuple() {
        let mut val = 1i32;
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = "b".to_owned();
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, "b".to_owned());
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned());
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);

        let mut val = (1i32, 2.4f64, "b".to_owned(), 123u8, 456u16, 789u32);
        let original = val.clone();
        val = FromValueTuple::from_value_tuple(val);
        assert_eq!(val, original);
    }

    #[test]
    fn test_value_tuple_iter() {
        let mut iter = (1i32).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b").into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8).into_value_tuple().into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16)
            .into_value_tuple()
            .into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
        assert_eq!(iter.next(), None);

        let mut iter = (1i32, 2.4f64, "b", 123u8, 456u16, 789u32)
            .into_value_tuple()
            .into_iter();
        assert_eq!(iter.next().unwrap(), Value::Int(Some(1)));
        assert_eq!(iter.next().unwrap(), Value::Double(Some(2.4)));
        assert_eq!(
            iter.next().unwrap(),
            Value::String(Some(Box::new("b".to_owned())))
        );
        assert_eq!(iter.next().unwrap(), Value::TinyUnsigned(Some(123)));
        assert_eq!(iter.next().unwrap(), Value::SmallUnsigned(Some(456)));
        assert_eq!(iter.next().unwrap(), Value::Unsigned(Some(789)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[cfg(feature = "with-json")]
    fn test_json_value() {
        let json = serde_json::json! {{
            "a": 25.0,
            "b": "hello",
        }};
        let value: Value = json.clone().into();
        let out: Json = value.unwrap();
        assert_eq!(out, json);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_value() {
        let timestamp = NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(2, 2, 2)
            .unwrap();
        let value: Value = timestamp.into();
        let out: NaiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_utc_value() {
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(3, 4, 5)
                .unwrap(),
            Utc,
        );
        let value: Value = timestamp.into();
        let out: DateTime<Utc> = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_local_value() {
        let timestamp_utc = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2022, 1, 2)
                .unwrap()
                .and_hms_opt(3, 4, 5)
                .unwrap(),
            Utc,
        );
        let timestamp_local: DateTime<Local> = timestamp_utc.into();
        let value: Value = timestamp_local.into();
        let out: DateTime<Local> = value.unwrap();
        assert_eq!(out, timestamp_local);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_timezone_value() {
        let timestamp = DateTime::parse_from_rfc3339("2020-01-01T02:02:02+08:00").unwrap();
        let value: Value = timestamp.into();
        let out: DateTime<FixedOffset> = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn test_chrono_query() {
        use crate::*;

        let string = "2020-01-01T02:02:02+08:00";
        let timestamp = DateTime::parse_from_rfc3339(string).unwrap();

        let query = Query::select().expr(timestamp).to_owned();

        let formatted = "2020-01-01 02:02:02.000000 +08:00";

        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            format!("SELECT '{formatted}'")
        );
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            format!("SELECT '{formatted}'")
        );
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            format!("SELECT '{formatted}'")
        );
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_value() {
        use time::macros::{date, time};
        let timestamp = date!(2020 - 01 - 01).with_time(time!(2:2:2));
        let value: Value = timestamp.into();
        let out: PrimitiveDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_utc_value() {
        use time::macros::{date, time};
        let timestamp = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let value: Value = timestamp.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_local_value() {
        use time::macros::{date, offset, time};
        let timestamp_utc = date!(2022 - 01 - 02).with_time(time!(3:04:05)).assume_utc();
        let timestamp_local: OffsetDateTime = timestamp_utc.to_offset(offset!(+3));
        let value: Value = timestamp_local.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp_local);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_timezone_value() {
        use time::macros::{date, offset, time};
        let timestamp = date!(2022 - 01 - 02)
            .with_time(time!(3:04:05))
            .assume_offset(offset!(+8));
        let value: Value = timestamp.into();
        let out: OffsetDateTime = value.unwrap();
        assert_eq!(out, timestamp);
    }

    #[test]
    #[cfg(feature = "with-time")]
    fn test_time_query() {
        use crate::*;
        use time::macros::datetime;

        let timestamp = datetime!(2020-01-01 02:02:02 +8);
        let query = Query::select().expr(timestamp).to_owned();
        let formatted = "2020-01-01 02:02:02.000000 +08:00";

        assert_eq!(
            query.to_string(MysqlQueryBuilder),
            format!("SELECT '{formatted}'")
        );
        assert_eq!(
            query.to_string(PostgresQueryBuilder),
            format!("SELECT '{formatted}'")
        );
        assert_eq!(
            query.to_string(SqliteQueryBuilder),
            format!("SELECT '{formatted}'")
        );
    }

    #[test]
    #[cfg(feature = "with-uuid")]
    fn test_uuid_value() {
        let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
        let value: Value = uuid.into();
        let out: Uuid = value.unwrap();
        assert_eq!(out, uuid);

        let uuid_braced = uuid.braced();
        let value: Value = uuid_braced.into();
        let out: Uuid = value.unwrap();
        assert_eq!(out, uuid);

        let uuid_hyphenated = uuid.hyphenated();
        let value: Value = uuid_hyphenated.into();
        let out: Uuid = value.unwrap();
        assert_eq!(out, uuid);

        let uuid_simple = uuid.simple();
        let value: Value = uuid_simple.into();
        let out: Uuid = value.unwrap();
        assert_eq!(out, uuid);

        let uuid_urn = uuid.urn();
        let value: Value = uuid_urn.into();
        let out: Uuid = value.unwrap();
        assert_eq!(out, uuid);
    }

    #[test]
    #[cfg(feature = "with-rust_decimal")]
    fn test_decimal_value() {
        use std::str::FromStr;

        let num = "2.02";
        let val = Decimal::from_str(num).unwrap();
        let v: Value = val.into();
        let out: Decimal = v.unwrap();
        assert_eq!(out.to_string(), num);
    }

    #[test]
    #[cfg(feature = "postgres-array")]
    fn test_array_value() {
        let array = vec![1, 2, 3, 4, 5];
        let v: Value = array.into();
        let out: Vec<i32> = v.unwrap();
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    #[cfg(feature = "postgres-array")]
    fn test_option_array_value() {
        let v: Value = Value::Array(ArrayType::Int, None);
        let out: Option<Vec<i32>> = v.unwrap();
        assert_eq!(out, None);
    }
}

#[cfg(feature = "hashable-value")]
mod hashable_value {
    use super::*;
    use ordered_float::OrderedFloat;
    use std::{
        hash::{Hash, Hasher},
        mem,
    };

    impl PartialEq for Value {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Bool(l), Self::Bool(r)) => l == r,
                (Self::TinyInt(l), Self::TinyInt(r)) => l == r,
                (Self::SmallInt(l), Self::SmallInt(r)) => l == r,
                (Self::Int(l), Self::Int(r)) => l == r,
                (Self::BigInt(l), Self::BigInt(r)) => l == r,
                (Self::TinyUnsigned(l), Self::TinyUnsigned(r)) => l == r,
                (Self::SmallUnsigned(l), Self::SmallUnsigned(r)) => l == r,
                (Self::Unsigned(l), Self::Unsigned(r)) => l == r,
                (Self::BigUnsigned(l), Self::BigUnsigned(r)) => l == r,
                (Self::Float(l), Self::Float(r)) => cmp_f32(l, r),
                (Self::Double(l), Self::Double(r)) => cmp_f64(l, r),
                (Self::String(l), Self::String(r)) => l == r,
                (Self::Char(l), Self::Char(r)) => l == r,
                (Self::Bytes(l), Self::Bytes(r)) => l == r,

                #[cfg(feature = "with-json")]
                (Self::Json(l), Self::Json(r)) => cmp_json(l, r),

                #[cfg(feature = "with-chrono")]
                (Self::ChronoDate(l), Self::ChronoDate(r)) => l == r,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoTime(l), Self::ChronoTime(r)) => l == r,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTime(l), Self::ChronoDateTime(r)) => l == r,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeUtc(l), Self::ChronoDateTimeUtc(r)) => l == r,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeLocal(l), Self::ChronoDateTimeLocal(r)) => l == r,
                #[cfg(feature = "with-chrono")]
                (Self::ChronoDateTimeWithTimeZone(l), Self::ChronoDateTimeWithTimeZone(r)) => {
                    l == r
                }

                #[cfg(feature = "with-time")]
                (Self::TimeDate(l), Self::TimeDate(r)) => l == r,
                #[cfg(feature = "with-time")]
                (Self::TimeTime(l), Self::TimeTime(r)) => l == r,
                #[cfg(feature = "with-time")]
                (Self::TimeDateTime(l), Self::TimeDateTime(r)) => l == r,
                #[cfg(feature = "with-time")]
                (Self::TimeDateTimeWithTimeZone(l), Self::TimeDateTimeWithTimeZone(r)) => l == r,

                #[cfg(feature = "with-uuid")]
                (Self::Uuid(l), Self::Uuid(r)) => l == r,

                #[cfg(feature = "with-rust_decimal")]
                (Self::Decimal(l), Self::Decimal(r)) => l == r,

                #[cfg(feature = "with-bigdecimal")]
                (Self::BigDecimal(l), Self::BigDecimal(r)) => l == r,

                #[cfg(feature = "postgres-array")]
                (Self::Array(ty_l, values_l), Self::Array(ty_r, values_r)) => {
                    ty_l == ty_r && values_l == values_r
                }

                #[cfg(feature = "postgres-vector")]
                (Self::Vector(l), Self::Vector(r)) => cmp_vector(l, r),

                #[cfg(feature = "with-ipnetwork")]
                (Self::IpNetwork(l), Self::IpNetwork(r)) => l == r,

                #[cfg(feature = "with-mac_address")]
                (Self::MacAddress(l), Self::MacAddress(r)) => l == r,

                _ => false,
            }
        }
    }

    impl Eq for Value {}

    impl Hash for Value {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            mem::discriminant(self).hash(state);
            match self {
                Value::Bool(v) => v.hash(state),
                Value::TinyInt(v) => v.hash(state),
                Value::SmallInt(v) => v.hash(state),
                Value::Int(v) => v.hash(state),
                Value::BigInt(v) => v.hash(state),
                Value::TinyUnsigned(v) => v.hash(state),
                Value::SmallUnsigned(v) => v.hash(state),
                Value::Unsigned(v) => v.hash(state),
                Value::BigUnsigned(v) => v.hash(state),
                Value::Float(v) => hash_f32(v, state),
                Value::Double(v) => hash_f64(v, state),
                Value::String(v) => v.hash(state),
                Value::Char(v) => v.hash(state),
                Value::Bytes(v) => v.hash(state),

                #[cfg(feature = "with-json")]
                Value::Json(value) => hash_json(value, state),

                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(naive_date) => naive_date.hash(state),
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(naive_time) => naive_time.hash(state),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(naive_date_time) => naive_date_time.hash(state),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(date_time) => date_time.hash(state),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(date_time) => date_time.hash(state),
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(date_time) => date_time.hash(state),

                #[cfg(feature = "with-time")]
                Value::TimeDate(date) => date.hash(state),
                #[cfg(feature = "with-time")]
                Value::TimeTime(time) => time.hash(state),
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(primitive_date_time) => primitive_date_time.hash(state),
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(offset_date_time) => offset_date_time.hash(state),

                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => uuid.hash(state),

                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(decimal) => decimal.hash(state),

                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(big_decimal) => big_decimal.hash(state),

                #[cfg(feature = "postgres-array")]
                Value::Array(array_type, vec) => {
                    array_type.hash(state);
                    vec.hash(state);
                }

                #[cfg(feature = "postgres-vector")]
                Value::Vector(vector) => hash_vector(vector, state),

                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip_network) => ip_network.hash(state),

                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac_address) => mac_address.hash(state),
            }
        }
    }

    fn hash_f32<H: Hasher>(v: &Option<f32>, state: &mut H) {
        match v {
            Some(v) => OrderedFloat(*v).hash(state),
            None => "null".hash(state),
        }
    }

    fn hash_f64<H: Hasher>(v: &Option<f64>, state: &mut H) {
        match v {
            Some(v) => OrderedFloat(*v).hash(state),
            None => "null".hash(state),
        }
    }

    fn cmp_f32(l: &Option<f32>, r: &Option<f32>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => OrderedFloat(*l).eq(&OrderedFloat(*r)),
            (None, None) => true,
            _ => false,
        }
    }

    fn cmp_f64(l: &Option<f64>, r: &Option<f64>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => OrderedFloat(*l).eq(&OrderedFloat(*r)),
            (None, None) => true,
            _ => false,
        }
    }

    #[cfg(feature = "with-json")]
    fn hash_json<H: Hasher>(v: &Option<Box<Json>>, state: &mut H) {
        match v {
            Some(v) => serde_json::to_string(v).unwrap().hash(state),
            None => "null".hash(state),
        }
    }

    #[cfg(feature = "with-json")]
    fn cmp_json(l: &Option<Box<Json>>, r: &Option<Box<Json>>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => serde_json::to_string(l)
                .unwrap()
                .eq(&serde_json::to_string(r).unwrap()),
            (None, None) => true,
            _ => false,
        }
    }

    #[cfg(feature = "postgres-vector")]
    fn hash_vector<H: Hasher>(v: &Option<Box<pgvector::Vector>>, state: &mut H) {
        match v {
            Some(v) => {
                for &value in v.as_slice().iter() {
                    hash_f32(&Some(value), state);
                }
            }
            None => "null".hash(state),
        }
    }

    #[cfg(feature = "postgres-vector")]
    fn cmp_vector(l: &Option<Box<pgvector::Vector>>, r: &Option<Box<pgvector::Vector>>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => {
                let (l, r) = (l.as_slice(), r.as_slice());
                if l.len() != r.len() {
                    return false;
                }
                for (l, r) in l.iter().zip(r.iter()) {
                    if !cmp_f32(&Some(*l), &Some(*r)) {
                        return false;
                    }
                }
                true
            }
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn test_hash_value_0() {
        let hash_set: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::Int(None),
            Value::BigInt(None),
            Value::BigInt(None),
            Value::Float(None),
            Value::Float(None),                // Null is not NaN
            Value::Float(Some(std::f32::NAN)), // NaN considered equal
            Value::Float(Some(std::f32::NAN)),
            Value::Double(None),
            Value::Double(None),
            Value::Double(Some(std::f64::NAN)),
            Value::Double(Some(std::f64::NAN)),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::BigInt(None),
            Value::Float(None),
            Value::Double(None),
            Value::Float(Some(std::f32::NAN)),
            Value::Double(Some(std::f64::NAN)),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }

    #[test]
    fn test_hash_value_1() {
        let hash_set: std::collections::HashSet<Value> = [
            Value::Int(None),
            Value::Int(Some(1)),
            Value::Int(Some(1)),
            Value::BigInt(Some(2)),
            Value::BigInt(Some(2)),
            Value::Float(Some(3.0)),
            Value::Float(Some(3.0)),
            Value::Double(Some(3.0)),
            Value::Double(Some(3.0)),
            Value::BigInt(Some(5)),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Value::BigInt(Some(5)),
            Value::Double(Some(3.0)),
            Value::Float(Some(3.0)),
            Value::BigInt(Some(2)),
            Value::Int(Some(1)),
            Value::Int(None),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }

    #[cfg(feature = "postgres-array")]
    #[test]
    fn test_hash_value_array() {
        assert_eq!(
            Into::<Value>::into(vec![0i32, 1, 2]),
            Value::Array(
                ArrayType::Int,
                Some(Box::new(vec![
                    Value::Int(Some(0)),
                    Value::Int(Some(1)),
                    Value::Int(Some(2))
                ]))
            )
        );

        assert_eq!(
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Value::Array(
                ArrayType::Float,
                Some(Box::new(vec![
                    Value::Float(Some(0f32)),
                    Value::Float(Some(1.0)),
                    Value::Float(Some(2.0))
                ]))
            )
        );

        let hash_set: std::collections::HashSet<Value> = [
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![3f32, 2.0, 1.0]),
        ]
        .into_iter()
        .collect();

        let unique: std::collections::HashSet<Value> = [
            Into::<Value>::into(vec![0i32, 1, 2]),
            Into::<Value>::into(vec![0f32, 1.0, 2.0]),
            Into::<Value>::into(vec![3f32, 2.0, 1.0]),
        ]
        .into_iter()
        .collect();

        assert_eq!(hash_set, unique);
    }
}
