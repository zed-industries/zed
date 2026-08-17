//! Conversions between Rust and standard **SQL** types.
//!
//! # Types
//!
//! | Rust type                             | SQL type(s)                                          |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | BOOLEAN                                              |
//! | `i16`                                 | SMALLINT                                             |
//! | `i32`                                 | INT                                                  |
//! | `i64`                                 | BIGINT                                               |
//! | `f32`                                 | FLOAT                                                |
//! | `f64`                                 | DOUBLE                                               |
//! | `&str`, [`String`]                    | VARCHAR, CHAR, TEXT                                  |
//!
//! # Nullable
//!
//! In addition, `Option<T>` is supported where `T` implements `Type`. An `Option<T>` represents
//! a potentially `NULL` value from SQL.

mod blob;
mod bool;
mod float;
mod int;
mod str;

#[test]
fn test_type_impls() {
    use crate::any::Any;
    use crate::decode::Decode;
    use crate::encode::Encode;
    use crate::types::Type;

    fn has_type<T>()
    where
        T: Type<Any>,
        for<'a> T: Encode<'a, Any>,
        for<'a> T: Decode<'a, Any>,
    {
    }

    has_type::<bool>();

    has_type::<i16>();
    has_type::<i32>();
    has_type::<i64>();

    has_type::<f32>();
    has_type::<f64>();

    // These imply that there are also impls for the equivalent slice types.
    has_type::<Vec<u8>>();
    has_type::<String>();
}
