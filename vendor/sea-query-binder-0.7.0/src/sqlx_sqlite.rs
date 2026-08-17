use crate::SqlxValues;
use sea_query::Value;

impl<'q> sqlx::IntoArguments<'q, sqlx::sqlite::Sqlite> for SqlxValues {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments<'q> {
        let mut args = sqlx::sqlite::SqliteArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    let _ = args.add(b);
                }
                Value::TinyInt(i) => {
                    let _ = args.add(i);
                }
                Value::SmallInt(i) => {
                    let _ = args.add(i);
                }
                Value::Int(i) => {
                    let _ = args.add(i);
                }
                Value::BigInt(i) => {
                    let _ = args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    let _ = args.add(i);
                }
                Value::SmallUnsigned(i) => {
                    let _ = args.add(i);
                }
                Value::Unsigned(i) => {
                    let _ = args.add(i);
                }
                Value::BigUnsigned(i) => {
                    let _ = args
                        .add(i.map(|i| <i64 as std::convert::TryFrom<u64>>::try_from(i).unwrap()));
                }
                Value::Float(f) => {
                    let _ = args.add(f);
                }
                Value::Double(d) => {
                    let _ = args.add(d);
                }
                Value::String(s) => {
                    let _ = args.add(s.map(|s| *s));
                }
                Value::Char(c) => {
                    let _ = args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    let _ = args.add(b.map(|b| *b));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    let _ = args.add(d.map(|d| *d));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    let _ = args.add(t.map(|t| *t));
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    let _ = args.add(uuid.map(|uuid| *uuid));
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(decimal) => {
                    use rust_decimal::prelude::ToPrimitive;
                    let _ = args.add(decimal.map(|d| d.to_string()));
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(big_decimal) => {
                    use bigdecimal::ToPrimitive;
                    let _ = args.add(big_decimal.map(|d| d.to_string()));
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    let _ = args.add(j.map(|j| *j));
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("Sqlite doesn't support IpNetwork arguments");
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("Sqlite doesn't support MacAddress arguments");
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("Sqlite doesn't support array arguments");
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("Sqlite doesn't support vector arguments");
                }
            }
        }
        args
    }
}
