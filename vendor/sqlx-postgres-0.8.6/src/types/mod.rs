//! Conversions between Rust and **Postgres** types.
//!
//! # Types
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | BOOL                                                 |
//! | `i8`                                  | "CHAR"                                               |
//! | `i16`                                 | SMALLINT, SMALLSERIAL, INT2                          |
//! | `i32`                                 | INT, SERIAL, INT4                                    |
//! | `i64`                                 | BIGINT, BIGSERIAL, INT8                              |
//! | `f32`                                 | REAL, FLOAT4                                         |
//! | `f64`                                 | DOUBLE PRECISION, FLOAT8                             |
//! | `&str`, [`String`]                    | VARCHAR, CHAR(N), TEXT, NAME, CITEXT                 |
//! | `&[u8]`, `Vec<u8>`                    | BYTEA                                                |
//! | `()`                                  | VOID                                                 |
//! | [`PgInterval`]                        | INTERVAL                                             |
//! | [`PgRange<T>`](PgRange)               | INT8RANGE, INT4RANGE, TSRANGE, TSTZRANGE, DATERANGE, NUMRANGE |
//! | [`PgMoney`]                           | MONEY                                                |
//! | [`PgLTree`]                           | LTREE                                                |
//! | [`PgLQuery`]                          | LQUERY                                               |
//! | [`PgCiText`]                          | CITEXT<sup>1</sup>                                   |
//! | [`PgCube`]                            | CUBE                                                 |
//! | [`PgPoint`]                           | POINT                                                |
//! | [`PgLine`]                            | LINE                                                 |
//! | [`PgLSeg`]                            | LSEG                                                 |
//! | [`PgBox`]                             | BOX                                                  |
//! | [`PgPath`]                            | PATH                                                 |
//! | [`PgPolygon`]                         | POLYGON                                              |
//! | [`PgCircle`]                          | CIRCLE                                               |
//! | [`PgHstore`]                          | HSTORE                                               |
//!
//! <sup>1</sup> SQLx generally considers `CITEXT` to be compatible with `String`, `&str`, etc.,
//! but this wrapper type is available for edge cases, such as `CITEXT[]` which Postgres
//! does not consider to be compatible with `TEXT[]`.
//!
//! ### [`bigdecimal`](https://crates.io/crates/bigdecimal)
//! Requires the `bigdecimal` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                        |
//! |---------------------------------------|------------------------------------------------------|
//! | `bigdecimal::BigDecimal`              | NUMERIC                                              |
//!
#![doc=include_str!("bigdecimal-range.md")]
//!
//! ### [`rust_decimal`](https://crates.io/crates/rust_decimal)
//! Requires the `rust_decimal` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                        |
//! |---------------------------------------|------------------------------------------------------|
//! | `rust_decimal::Decimal`               | NUMERIC                                              |
//!
#![doc=include_str!("rust_decimal-range.md")]
//!
//! ### [`chrono`](https://crates.io/crates/chrono)
//!
//! Requires the `chrono` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `chrono::DateTime<Utc>`               | TIMESTAMPTZ                                          |
//! | `chrono::DateTime<Local>`             | TIMESTAMPTZ                                          |
//! | `chrono::NaiveDateTime`               | TIMESTAMP                                            |
//! | `chrono::NaiveDate`                   | DATE                                                 |
//! | `chrono::NaiveTime`                   | TIME                                                 |
//! | [`PgTimeTz`]                          | TIMETZ                                               |
//!
//! ### [`time`](https://crates.io/crates/time)
//!
//! Requires the `time` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `time::PrimitiveDateTime`             | TIMESTAMP                                            |
//! | `time::OffsetDateTime`                | TIMESTAMPTZ                                          |
//! | `time::Date`                          | DATE                                                 |
//! | `time::Time`                          | TIME                                                 |
//! | [`PgTimeTz`]                          | TIMETZ                                               |
//!
//! ### [`uuid`](https://crates.io/crates/uuid)
//!
//! Requires the `uuid` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `uuid::Uuid`                          | UUID                                                 |
//!
//! ### [`ipnetwork`](https://crates.io/crates/ipnetwork)
//!
//! Requires the `ipnetwork` Cargo feature flag (takes precedence over `ipnet` if both are used).
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `ipnetwork::IpNetwork`                | INET, CIDR                                           |
//! | `std::net::IpAddr`                    | INET, CIDR                                           |
//!
//! Note that because `IpAddr` does not support network prefixes, it is an error to attempt to decode
//! an `IpAddr` from a `INET` or `CIDR` value with a network prefix smaller than the address' full width:
//! `/32` for IPv4 addresses and `/128` for IPv6 addresses.
//!
//! `IpNetwork` does not have this limitation.
//!
//! ### [`ipnet`](https://crates.io/crates/ipnet)
//!
//! Requires the `ipnet` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `ipnet::IpNet`                        | INET, CIDR                                           |
//! | `std::net::IpAddr`                    | INET, CIDR                                           |
//!
//! The same `IpAddr` limitation for smaller network prefixes applies as with `ipnet`.
//!
//! ### [`mac_address`](https://crates.io/crates/mac_address)
//!
//! Requires the `mac_address` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `mac_address::MacAddress`             | MACADDR                                              |
//!
//! ### [`bit-vec`](https://crates.io/crates/bit-vec)
//!
//! Requires the `bit-vec` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | `bit_vec::BitVec`                     | BIT, VARBIT                                          |
//!
//! ### [`json`](https://crates.io/crates/serde_json)
//!
//! Requires the `json` Cargo feature flag.
//!
//! | Rust type                             | Postgres type(s)                                     |
//! |---------------------------------------|------------------------------------------------------|
//! | [`Json<T>`]                           | JSON, JSONB                                          |
//! | `serde_json::Value`                   | JSON, JSONB                                          |
//! | `&serde_json::value::RawValue`        | JSON, JSONB                                          |
//!
//! `Value` and `RawValue` from `serde_json` can be used for unstructured JSON data with
//! Postgres.
//!
//! [`Json<T>`](crate::types::Json) can be used for structured JSON data with Postgres.
//!
//! # [Composite types](https://www.postgresql.org/docs/current/rowtypes.html)
//!
//! User-defined composite types are supported through a derive for `Type`.
//!
//! ```text
//! CREATE TYPE inventory_item AS (
//!     name            text,
//!     supplier_id     integer,
//!     price           numeric
//! );
//! ```
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[sqlx(type_name = "inventory_item")]
//! struct InventoryItem {
//!     name: String,
//!     supplier_id: i32,
//!     price: BigDecimal,
//! }
//! ```
//!
//! Anonymous composite types are represented as tuples. Note that anonymous composites may only
//! be returned and not sent to Postgres (this is a limitation of postgres).
//!
//! # Arrays
//!
//! One-dimensional arrays are supported as `Vec<T>` or `&[T]` where `T` implements `Type`.
//!
//! # [Enumerations](https://www.postgresql.org/docs/current/datatype-enum.html)
//!
//! User-defined enumerations are supported through a derive for `Type`.
//!
//! ```text
//! CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
//! ```
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[sqlx(type_name = "mood", rename_all = "lowercase")]
//! enum Mood { Sad, Ok, Happy }
//! ```
//!
//! Rust enumerations may also be defined to be represented as an integer using `repr`.
//! The following type expects a SQL type of `INTEGER` or `INT4` and will convert to/from the
//! Rust enumeration.
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[repr(i32)]
//! enum Mood { Sad = 0, Ok = 1, Happy = 2 }
//! ```
//!
//! Rust enumerations may also be defined to be represented as a string using `type_name = "text"`.
//! The following type expects a SQL type of `TEXT` and will convert to/from the Rust enumeration.
//!
//! ```rust,ignore
//! #[derive(sqlx::Type)]
//! #[sqlx(type_name = "text")]
//! enum Mood { Sad, Ok, Happy }
//! ```
//!
//! Note that an error can occur if you attempt to decode a value not contained within the enum
//! definition.
//!

use crate::type_info::PgTypeKind;
use crate::{PgTypeInfo, Postgres};

pub(crate) use sqlx_core::types::{Json, Type};

mod array;
mod bool;
mod bytes;
mod citext;
mod float;
mod hstore;
mod int;
mod interval;
mod lquery;
mod ltree;
// Not behind a Cargo feature because we require JSON in the driver implementation.
mod json;
mod money;
mod oid;
mod range;
mod record;
mod str;
mod text;
mod tuple;
mod void;

#[cfg(any(feature = "chrono", feature = "time"))]
mod time_tz;

#[cfg(feature = "bigdecimal")]
mod bigdecimal;

mod cube;

mod geometry;

#[cfg(any(feature = "bigdecimal", feature = "rust_decimal"))]
mod numeric;

#[cfg(feature = "rust_decimal")]
mod rust_decimal;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "time")]
mod time;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(feature = "ipnet")]
mod ipnet;

#[cfg(feature = "ipnetwork")]
mod ipnetwork;

#[cfg(feature = "mac_address")]
mod mac_address;

#[cfg(feature = "bit-vec")]
mod bit_vec;

pub use array::PgHasArrayType;
pub use citext::PgCiText;
pub use cube::PgCube;
pub use geometry::circle::PgCircle;
pub use geometry::line::PgLine;
pub use geometry::line_segment::PgLSeg;
pub use geometry::path::PgPath;
pub use geometry::point::PgPoint;
pub use geometry::polygon::PgPolygon;
pub use geometry::r#box::PgBox;
pub use hstore::PgHstore;
pub use interval::PgInterval;
pub use lquery::PgLQuery;
pub use lquery::PgLQueryLevel;
pub use lquery::PgLQueryVariant;
pub use lquery::PgLQueryVariantFlag;
pub use ltree::PgLTree;
pub use ltree::PgLTreeLabel;
pub use ltree::PgLTreeParseError;
pub use money::PgMoney;
pub use oid::Oid;
pub use range::PgRange;

#[cfg(any(feature = "chrono", feature = "time"))]
pub use time_tz::PgTimeTz;

// used in derive(Type) for `struct`
// but the interface is not considered part of the public API
#[doc(hidden)]
pub use record::{PgRecordDecoder, PgRecordEncoder};

// Type::compatible impl appropriate for arrays
fn array_compatible<E: Type<Postgres> + ?Sized>(ty: &PgTypeInfo) -> bool {
    // we require the declared type to be an _array_ with an
    // element type that is acceptable
    if let PgTypeKind::Array(element) = &ty.kind() {
        return E::compatible(element);
    }

    false
}
