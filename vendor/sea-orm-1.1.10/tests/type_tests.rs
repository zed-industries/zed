#![allow(unused_imports, dead_code)]

pub mod common;

use sea_orm::{IntoActiveValue, TryFromU64, TryGetable, Value};

/*

When supporting a new type in SeaORM we should implement the following traits for it:
  - `IntoActiveValue`, given that it implemented `Into<Value>` already
  - `TryGetable`
  - `TryFromU64`

Also, we need to update `impl FromQueryResult for JsonValue` at `src/query/json.rs`
to correctly serialize the type as `serde_json::Value`.

*/

pub fn it_impl_into_active_value<T: IntoActiveValue<V>, V: Into<Value>>() {}

pub fn it_impl_try_getable<T: TryGetable>() {}

pub fn it_impl_try_from_u64<T: TryFromU64>() {}

#[allow(unused_macros)]
macro_rules! it_impl_traits {
    ( $ty: ty ) => {
        it_impl_into_active_value::<$ty, $ty>();
        it_impl_into_active_value::<Option<$ty>, Option<$ty>>();
        it_impl_into_active_value::<Option<Option<$ty>>, Option<$ty>>();

        it_impl_try_getable::<$ty>();
        it_impl_try_getable::<Option<$ty>>();

        it_impl_try_from_u64::<$ty>();
    };
}

#[sea_orm_macros::test]
#[cfg(feature = "sqlx-dep")]
fn main() {
    it_impl_traits!(i8);
    it_impl_traits!(i16);
    it_impl_traits!(i32);
    it_impl_traits!(i64);
    it_impl_traits!(u8);
    it_impl_traits!(u16);
    it_impl_traits!(u32);
    it_impl_traits!(u64);
    it_impl_traits!(bool);
    it_impl_traits!(f32);
    it_impl_traits!(f64);
    it_impl_traits!(Vec<u8>);
    it_impl_traits!(String);
    it_impl_traits!(serde_json::Value);
    it_impl_traits!(chrono::NaiveDate);
    it_impl_traits!(chrono::NaiveTime);
    it_impl_traits!(chrono::NaiveDateTime);
    it_impl_traits!(chrono::DateTime<chrono::FixedOffset>);
    it_impl_traits!(chrono::DateTime<chrono::Utc>);
    it_impl_traits!(chrono::DateTime<chrono::Local>);
    it_impl_traits!(time::Date);
    it_impl_traits!(time::Time);
    it_impl_traits!(time::PrimitiveDateTime);
    it_impl_traits!(time::OffsetDateTime);
    it_impl_traits!(rust_decimal::Decimal);
    it_impl_traits!(uuid::Uuid);
    it_impl_traits!(ipnetwork::IpNetwork);
}
