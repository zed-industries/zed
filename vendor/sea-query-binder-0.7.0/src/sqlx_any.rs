use crate::SqlxValues;
use sea_query::Value;

impl<'q> sqlx::IntoArguments<'q, sqlx::any::Any> for SqlxValues {
    fn into_arguments(self) -> sqlx::any::AnyArguments<'q> {
        let mut args = sqlx::any::AnyArguments::default();
        for arg in self.0.into_iter() {
            use sqlx::Arguments;
            match arg {
                Value::Bool(b) => {
                    let _ = args.add(b);
                }
                Value::TinyInt(i) => {
                    let _ = args.add(i.map(Into::<i32>::into));
                }
                Value::SmallInt(i) => {
                    let _ = args.add(i.map(Into::<i32>::into));
                }
                Value::Int(i) => {
                    let _ = args.add(i);
                }
                Value::BigInt(i) => {
                    let _ = args.add(i);
                }
                Value::TinyUnsigned(i) => {
                    let _ = args.add(i.map(Into::<i32>::into));
                }
                Value::SmallUnsigned(i) => {
                    let _ = args.add(i.map(Into::<i32>::into));
                }
                Value::Unsigned(i) => {
                    let _ = args.add(i.map(Into::<i64>::into));
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
                Value::ChronoDate(t) => {
                    let _ = args.add(Value::ChronoDate(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    let _ = args.add(Value::ChronoTime(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    let _ = args.add(Value::ChronoDateTime(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    let _ = args.add(Value::ChronoDateTimeUtc(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    let _ = args.add(Value::ChronoDateTimeLocal(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    let _ = args
                        .add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    let _ = args.add(Value::TimeDate(t).time_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    let _ = args.add(Value::TimeTime(t).time_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    let _ = args.add(Value::TimeDateTime(t).time_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    let _ =
                        args.add(Value::TimeDateTimeWithTimeZone(t).time_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(_) => {
                    panic!("UUID support not implemented for Any");
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(_) => {
                    panic!("Sqlite doesn't support decimal arguments");
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(_) => {
                    panic!("Sqlite doesn't support bigdecimal arguments");
                }
                #[cfg(feature = "with-json")]
                Value::Json(_) => {
                    panic!("Json support not implemented for Any");
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("SQLx doesn't support IpNetwork arguments for Any");
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("SQLx doesn't support MacAddress arguments for Any");
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("SQLx doesn't support array arguments for Any");
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("SQLx doesn't support vector arguments for Any");
                }
            }
        }
        args
    }
}
