use crate::SqlxValues;
use sea_query::Value;

impl sqlx::IntoArguments<'_, sqlx::mysql::MySql> for SqlxValues {
    fn into_arguments(self) -> sqlx::mysql::MySqlArguments {
        let mut args = sqlx::mysql::MySqlArguments::default();
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
                    let _ = args.add(i);
                }
                Value::Float(f) => {
                    let _ = args.add(f);
                }
                Value::Double(d) => {
                    let _ = args.add(d);
                }
                Value::String(s) => {
                    let _ = args.add(s.as_deref());
                }
                Value::Char(c) => {
                    let _ = args.add(c.map(|c| c.to_string()));
                }
                Value::Bytes(b) => {
                    let _ = args.add(b.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDate(d) => {
                    let _ = args.add(d.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoTime(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTime(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeUtc(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeLocal(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-chrono")]
                Value::ChronoDateTimeWithTimeZone(t) => {
                    let _ = args
                        .add(Value::ChronoDateTimeWithTimeZone(t).chrono_as_naive_utc_in_string());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDate(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeTime(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTime(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-time")]
                Value::TimeDateTimeWithTimeZone(t) => {
                    let _ = args.add(t.as_deref());
                }
                #[cfg(feature = "with-uuid")]
                Value::Uuid(uuid) => {
                    let _ = args.add(uuid.as_deref());
                }
                #[cfg(feature = "with-rust_decimal")]
                Value::Decimal(d) => {
                    let _ = args.add(d.as_deref());
                }
                #[cfg(feature = "with-bigdecimal")]
                Value::BigDecimal(d) => {
                    let _ = args.add(d.as_deref());
                }
                #[cfg(feature = "with-json")]
                Value::Json(j) => {
                    let _ = args.add(j.as_deref());
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(_, _) => {
                    panic!("Mysql doesn't support array arguments");
                }
                #[cfg(feature = "postgres-vector")]
                Value::Vector(_) => {
                    panic!("Mysql doesn't support vector arguments");
                }
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(_) => {
                    panic!("Mysql doesn't support IpNetwork arguments");
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(_) => {
                    panic!("Mysql doesn't support MacAddress arguments");
                }
            }
        }
        args
    }
}
