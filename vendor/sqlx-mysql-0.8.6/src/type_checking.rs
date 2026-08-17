// Type mappings used by the macros and `Debug` impls.

#[allow(unused_imports)]
use sqlx_core as sqlx;

use crate::MySql;

impl_type_checking!(
    MySql {
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,

        // ordering is important here as otherwise we might infer strings to be binary
        // CHAR, VAR_CHAR, TEXT
        String,

        // BINARY, VAR_BINARY, BLOB
        Vec<u8>,

        // Types from third-party crates need to be referenced at a known path
        // for the macros to work, but we don't want to require the user to add extra dependencies.
        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDate,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::NaiveDateTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,

        #[cfg(feature = "time")]
        sqlx::types::time::Time,

        #[cfg(feature = "time")]
        sqlx::types::time::Date,

        #[cfg(feature = "time")]
        sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "bigdecimal")]
        sqlx::types::BigDecimal,

        #[cfg(feature = "rust_decimal")]
        sqlx::types::Decimal,

        #[cfg(feature = "json")]
        sqlx::types::JsonValue,
    },
    ParamChecking::Weak,
    feature-types: info => info.__type_feature_gate(),
);
