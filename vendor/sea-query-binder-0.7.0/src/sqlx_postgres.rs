#[cfg(feature = "with-bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "with-chrono")]
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
#[cfg(feature = "with-ipnetwork")]
use ipnetwork::IpNetwork;
#[cfg(feature = "with-mac_address")]
use mac_address::MacAddress;
#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;
#[cfg(feature = "with-json")]
use serde_json::Value as Json;
#[cfg(feature = "with-uuid")]
use uuid::Uuid;

use sea_query::{ArrayType, Value};

use crate::SqlxValues;

impl sqlx::IntoArguments<'_, sqlx::postgres::Postgres> for SqlxValues {
    fn into_arguments(self) -> sqlx::postgres::PgArguments {
        let mut args = sqlx::postgres::PgArguments::default();
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
                    let _ = args.add(i.map(|i| i as i16));
                }
                Value::SmallUnsigned(i) => {
                    let _ = args.add(i.map(|i| i as i32));
                }
                Value::Unsigned(i) => {
                    let _ = args.add(i.map(|i| i as i64));
                }
                Value::BigUnsigned(i) => {
                    let _ = args.add(i.map(|i| <i64 as TryFrom<u64>>::try_from(i).unwrap()));
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
                    let _ = args.add(t.as_deref());
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
                #[cfg(feature = "with-ipnetwork")]
                Value::IpNetwork(ip) => {
                    let _ = args.add(ip.as_deref());
                }
                #[cfg(feature = "with-mac_address")]
                Value::MacAddress(mac) => {
                    let _ = args.add(mac.as_deref());
                }
                #[cfg(feature = "postgres-array")]
                Value::Array(ty, v) => match ty {
                    ArrayType::Bool => {
                        let value: Option<Vec<bool>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Bool");
                        let _ = args.add(value);
                    }
                    ArrayType::TinyInt => {
                        let value: Option<Vec<i8>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TinyInt");
                        let _ = args.add(value);
                    }
                    ArrayType::SmallInt => {
                        let value: Option<Vec<i16>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::SmallInt");
                        let _ = args.add(value);
                    }
                    ArrayType::Int => {
                        let value: Option<Vec<i32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Int");
                        let _ = args.add(value);
                    }
                    ArrayType::BigInt => {
                        let value: Option<Vec<i64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigInt");
                        let _ = args.add(value);
                    }
                    ArrayType::TinyUnsigned => {
                        let value: Option<Vec<u8>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TinyUnsigned");
                        let value: Option<Vec<i16>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i16).collect());
                        let _ = args.add(value);
                    }
                    ArrayType::SmallUnsigned => {
                        let value: Option<Vec<u16>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::SmallUnsigned");
                        let value: Option<Vec<i32>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i32).collect());
                        let _ = args.add(value);
                    }
                    ArrayType::Unsigned => {
                        let value: Option<Vec<u32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Unsigned");
                        let value: Option<Vec<i64>> =
                            value.map(|vec| vec.into_iter().map(|i| i as i64).collect());
                        let _ = args.add(value);
                    }
                    ArrayType::BigUnsigned => {
                        let value: Option<Vec<u64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigUnsigned");
                        let value: Option<Vec<i64>> = value.map(|vec| {
                            vec.into_iter()
                                .map(|i| <i64 as TryFrom<u64>>::try_from(i).unwrap())
                                .collect()
                        });
                        let _ = args.add(value);
                    }
                    ArrayType::Float => {
                        let value: Option<Vec<f32>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Float");
                        let _ = args.add(value);
                    }
                    ArrayType::Double => {
                        let value: Option<Vec<f64>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Double");
                        let _ = args.add(value);
                    }
                    ArrayType::String => {
                        let value: Option<Vec<String>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::String");
                        let _ = args.add(value);
                    }
                    ArrayType::Char => {
                        let value: Option<Vec<char>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Char");
                        let value: Option<Vec<String>> =
                            value.map(|vec| vec.into_iter().map(|c| c.to_string()).collect());
                        let _ = args.add(value);
                    }
                    ArrayType::Bytes => {
                        let value: Option<Vec<Vec<u8>>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Bytes");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDate => {
                        let value: Option<Vec<NaiveDate>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDate");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoTime => {
                        let value: Option<Vec<NaiveTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoTime");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTime => {
                        let value: Option<Vec<NaiveDateTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDateTime");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeUtc => {
                        let value: Option<Vec<DateTime<Utc>>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::ChronoDateTimeUtc");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeLocal => {
                        let value: Option<Vec<DateTime<Local>>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::ChronoDateTimeLocal",
                        );
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-chrono")]
                    ArrayType::ChronoDateTimeWithTimeZone => {
                        let value: Option<Vec<DateTime<Local>>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::ChronoDateTimeWithTimeZone",
                        );
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDate => {
                        let value: Option<Vec<time::Date>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeDate");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeTime => {
                        let value: Option<Vec<time::Time>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeTime");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDateTime => {
                        let value: Option<Vec<time::PrimitiveDateTime>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::TimeDateTime");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-time")]
                    ArrayType::TimeDateTimeWithTimeZone => {
                        let value: Option<Vec<time::OffsetDateTime>> = Value::Array(ty, v).expect(
                            "This Value::Array should consist of Value::TimeDateTimeWithTimeZone",
                        );
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-uuid")]
                    ArrayType::Uuid => {
                        let value: Option<Vec<Uuid>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Uuid");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-rust_decimal")]
                    ArrayType::Decimal => {
                        let value: Option<Vec<Decimal>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Decimal");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-bigdecimal")]
                    ArrayType::BigDecimal => {
                        let value: Option<Vec<BigDecimal>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::BigDecimal");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-json")]
                    ArrayType::Json => {
                        let value: Option<Vec<Json>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::Json");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-ipnetwork")]
                    ArrayType::IpNetwork => {
                        let value: Option<Vec<IpNetwork>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::IpNetwork");
                        let _ = args.add(value);
                    }
                    #[cfg(feature = "with-mac_address")]
                    ArrayType::MacAddress => {
                        let value: Option<Vec<MacAddress>> = Value::Array(ty, v)
                            .expect("This Value::Array should consist of Value::MacAddress");
                        let _ = args.add(value);
                    }
                },
                #[cfg(feature = "postgres-vector")]
                Value::Vector(v) => {
                    let _ = args.add(v.as_deref());
                }
            }
        }
        args
    }
}
