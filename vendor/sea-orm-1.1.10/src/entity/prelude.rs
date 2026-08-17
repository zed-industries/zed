pub use crate::{
    error::*,
    sea_query::{DynIden, Expr, RcOrArc, SeaRc, StringLen},
    ActiveEnum, ActiveModelBehavior, ActiveModelTrait, ColumnDef, ColumnTrait, ColumnType,
    ColumnTypeTrait, ConnectionTrait, CursorTrait, DatabaseConnection, DbConn, EntityName,
    EntityTrait, EnumIter, ForeignKeyAction, Iden, IdenStatic, Linked, LoaderTrait, ModelTrait,
    PaginatorTrait, PrimaryKeyArity, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter, QueryResult,
    Related, RelationDef, RelationTrait, Select, Value,
};

#[cfg(feature = "macros")]
pub use crate::{
    DeriveActiveEnum, DeriveActiveModel, DeriveActiveModelBehavior, DeriveColumn,
    DeriveCustomColumn, DeriveDisplay, DeriveEntity, DeriveEntityModel, DeriveIden,
    DeriveIntoActiveModel, DeriveModel, DerivePartialModel, DerivePrimaryKey, DeriveRelatedEntity,
    DeriveRelation, DeriveValueType,
};

pub use async_trait;

#[cfg(feature = "with-json")]
pub use serde_json::Value as Json;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveDate as Date;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveTime as Time;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveDateTime as DateTime;

/// Date time with fixed offset
#[cfg(feature = "with-chrono")]
pub type DateTimeWithTimeZone = chrono::DateTime<chrono::FixedOffset>;

/// Date time represented in UTC
#[cfg(feature = "with-chrono")]
pub type DateTimeUtc = chrono::DateTime<chrono::Utc>;

/// Date time represented in local time
#[cfg(feature = "with-chrono")]
pub type DateTimeLocal = chrono::DateTime<chrono::Local>;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveDate as ChronoDate;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveTime as ChronoTime;

#[cfg(feature = "with-chrono")]
pub use chrono::NaiveDateTime as ChronoDateTime;

/// Date time with fixed offset
#[cfg(feature = "with-chrono")]
pub type ChronoDateTimeWithTimeZone = chrono::DateTime<chrono::FixedOffset>;

/// Date time represented in UTC
#[cfg(feature = "with-chrono")]
pub type ChronoDateTimeUtc = chrono::DateTime<chrono::Utc>;

/// Date time represented in local time
#[cfg(feature = "with-chrono")]
pub type ChronoDateTimeLocal = chrono::DateTime<chrono::Local>;

#[cfg(feature = "with-time")]
pub use time::Date as TimeDate;

#[cfg(feature = "with-time")]
pub use time::Time as TimeTime;

#[cfg(feature = "with-time")]
pub use time::PrimitiveDateTime as TimeDateTime;

#[cfg(feature = "with-time")]
pub use time::OffsetDateTime as TimeDateTimeWithTimeZone;

#[cfg(feature = "with-rust_decimal")]
pub use rust_decimal::Decimal;

#[cfg(feature = "with-bigdecimal")]
pub use bigdecimal::BigDecimal;

#[cfg(feature = "with-uuid")]
pub use uuid::Uuid;

#[cfg(feature = "postgres-vector")]
pub use pgvector::Vector as PgVector;

#[cfg(feature = "with-ipnetwork")]
pub use ipnetwork::IpNetwork;
