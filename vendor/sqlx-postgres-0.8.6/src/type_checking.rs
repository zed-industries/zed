use crate::Postgres;

// The paths used below will also be emitted by the macros so they have to match the final facade.
#[allow(unused_imports, dead_code)]
mod sqlx {
    pub use crate as postgres;
    pub use sqlx_core::*;
}

impl_type_checking!(
        Postgres {
        (),
        bool,
        String | &str,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        Vec<u8> | &[u8],

        sqlx::postgres::types::Oid,

        sqlx::postgres::types::PgInterval,

        sqlx::postgres::types::PgMoney,

        sqlx::postgres::types::PgLTree,

        sqlx::postgres::types::PgLQuery,

        sqlx::postgres::types::PgCube,

        sqlx::postgres::types::PgPoint,

        sqlx::postgres::types::PgLine,

        sqlx::postgres::types::PgLSeg,

        sqlx::postgres::types::PgBox,

        sqlx::postgres::types::PgPath,

        sqlx::postgres::types::PgPolygon,

        sqlx::postgres::types::PgCircle,

        #[cfg(feature = "uuid")]
        sqlx::types::Uuid,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDate,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDateTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc> | sqlx::types::chrono::DateTime<_>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::postgres::types::PgTimeTz<sqlx::types::chrono::NaiveTime, sqlx::types::chrono::FixedOffset>,

        #[cfg(feature = "time")]
        sqlx::types::time::Time,

        #[cfg(feature = "time")]
        sqlx::types::time::Date,

        #[cfg(feature = "time")]
        sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "time")]
        sqlx::postgres::types::PgTimeTz<sqlx::types::time::Time, sqlx::types::time::UtcOffset>,

        #[cfg(feature = "bigdecimal")]
        sqlx::types::BigDecimal,

        #[cfg(feature = "rust_decimal")]
        sqlx::types::Decimal,

        #[cfg(feature = "ipnetwork")]
        sqlx::types::ipnetwork::IpNetwork,

        #[cfg(feature = "ipnet")]
        sqlx::types::ipnet::IpNet,

        #[cfg(feature = "mac_address")]
        sqlx::types::mac_address::MacAddress,

        #[cfg(feature = "json")]
        sqlx::types::JsonValue,

        #[cfg(feature = "bit-vec")]
        sqlx::types::BitVec,

        sqlx::postgres::types::PgHstore,
        // Arrays

        Vec<bool> | &[bool],
        Vec<String> | &[String],
        Vec<Vec<u8>> | &[Vec<u8>],
        Vec<i8> | &[i8],
        Vec<i16> | &[i16],
        Vec<i32> | &[i32],
        Vec<i64> | &[i64],
        Vec<f32> | &[f32],
        Vec<f64> | &[f64],
        Vec<sqlx::postgres::types::Oid> | &[sqlx::postgres::types::Oid],
        Vec<sqlx::postgres::types::PgMoney> | &[sqlx::postgres::types::PgMoney],
        Vec<sqlx::postgres::types::PgInterval> | &[sqlx::postgres::types::PgInterval],

        #[cfg(feature = "uuid")]
        Vec<sqlx::types::Uuid> | &[sqlx::types::Uuid],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveTime> | &[sqlx::types::chrono::NaiveTime],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveDate> | &[sqlx::types::chrono::NaiveDate],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::NaiveDateTime> | &[sqlx::types::chrono::NaiveDateTime],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> | &[sqlx::types::chrono::DateTime<_>],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::Time> | &[sqlx::types::time::Time],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::Date> | &[sqlx::types::time::Date],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::PrimitiveDateTime> | &[sqlx::types::time::PrimitiveDateTime],

        #[cfg(feature = "time")]
        Vec<sqlx::types::time::OffsetDateTime> | &[sqlx::types::time::OffsetDateTime],

        #[cfg(feature = "bigdecimal")]
        Vec<sqlx::types::BigDecimal> | &[sqlx::types::BigDecimal],

        #[cfg(feature = "rust_decimal")]
        Vec<sqlx::types::Decimal> | &[sqlx::types::Decimal],

        #[cfg(feature = "ipnetwork")]
        Vec<sqlx::types::ipnetwork::IpNetwork> | &[sqlx::types::ipnetwork::IpNetwork],

        #[cfg(feature = "ipnet")]
        Vec<sqlx::types::ipnet::IpNet> | &[sqlx::types::ipnet::IpNet],

        #[cfg(feature = "mac_address")]
        Vec<sqlx::types::mac_address::MacAddress> | &[sqlx::types::mac_address::MacAddress],

        #[cfg(feature = "json")]
        Vec<sqlx::types::JsonValue> | &[sqlx::types::JsonValue],

        Vec<sqlx::postgres::types::PgHstore> | &[sqlx::postgres::types::PgHstore],

        // Ranges

        sqlx::postgres::types::PgRange<i32>,
        sqlx::postgres::types::PgRange<i64>,

        #[cfg(feature = "bigdecimal")]
        sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>,

        #[cfg(feature = "rust_decimal")]
        sqlx::postgres::types::PgRange<sqlx::types::Decimal>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>> |
            sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<_>>,

        #[cfg(feature = "time")]
        sqlx::postgres::types::PgRange<sqlx::types::time::Date>,

        #[cfg(feature = "time")]
        sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>,

        #[cfg(feature = "time")]
        sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>,

        // Range arrays

        Vec<sqlx::postgres::types::PgRange<i32>> | &[sqlx::postgres::types::PgRange<i32>],
        Vec<sqlx::postgres::types::PgRange<i64>> | &[sqlx::postgres::types::PgRange<i64>],

        #[cfg(feature = "bigdecimal")]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>],

        #[cfg(feature = "rust_decimal")]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::Decimal>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::Decimal>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<_>>],

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<_>>],

        #[cfg(feature = "time")]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::time::Date>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::time::Date>],

        #[cfg(feature = "time")]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>],

        #[cfg(feature = "time")]
        Vec<sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>> |
            &[sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>],
    },
    ParamChecking::Strong,
    feature-types: info => info.__type_feature_gate(),
);
