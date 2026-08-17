use crate::database::Database;
use crate::decode::Decode;
use crate::type_info::TypeInfo;
use crate::value::Value;
use std::any::Any;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The type of query parameter checking done by a SQL database.
#[derive(PartialEq, Eq)]
pub enum ParamChecking {
    /// Parameter checking is weak or nonexistent (uses coercion or allows mismatches).
    Weak,
    /// Parameter checking is strong (types must match exactly).
    Strong,
}

/// Type-checking extensions for the `Database` trait.
///
/// Mostly supporting code for the macros, and for `Debug` impls.
pub trait TypeChecking: Database {
    /// Describes how the database in question typechecks query parameters.
    const PARAM_CHECKING: ParamChecking;

    /// Get the full path of the Rust type that corresponds to the given `TypeInfo`, if applicable.
    ///
    /// If the type has a borrowed equivalent suitable for query parameters,
    /// this is that borrowed type.
    fn param_type_for_id(id: &Self::TypeInfo) -> Option<&'static str>;

    /// Get the full path of the Rust type that corresponds to the given `TypeInfo`, if applicable.
    ///
    /// Always returns the owned version of the type, suitable for decoding from `Row`.
    fn return_type_for_id(id: &Self::TypeInfo) -> Option<&'static str>;

    /// Get the name of the Cargo feature gate that must be enabled to process the given `TypeInfo`,
    /// if applicable.
    fn get_feature_gate(info: &Self::TypeInfo) -> Option<&'static str>;

    /// If `value` is a well-known type, decode and format it using `Debug`.
    ///
    /// If `value` is not a well-known type or could not be decoded, the reason is printed instead.
    fn fmt_value_debug(value: &<Self as Database>::Value) -> FmtValue<'_, Self>;
}

/// An adapter for [`Value`] which attempts to decode the value and format it when printed using [`Debug`].
pub struct FmtValue<'v, DB>
where
    DB: Database,
{
    value: &'v <DB as Database>::Value,
    fmt: fn(&'v <DB as Database>::Value, &mut Formatter<'_>) -> fmt::Result,
}

impl<'v, DB> FmtValue<'v, DB>
where
    DB: Database,
{
    // This API can't take `ValueRef` directly as it would need to pass it to `Decode` by-value,
    // which means taking ownership of it. We cannot rely on a `Clone` impl because `SqliteValueRef` doesn't have one.
    /// When printed with [`Debug`], attempt to decode `value` as the given type `T` and format it using [`Debug`].
    ///
    /// If `value` could not be decoded as `T`, the reason is printed instead.
    pub fn debug<T>(value: &'v <DB as Database>::Value) -> Self
    where
        T: Decode<'v, DB> + Debug + Any,
    {
        Self {
            value,
            fmt: |value, f| {
                let info = value.type_info();

                match T::decode(value.as_ref()) {
                    Ok(value) => Debug::fmt(&value, f),
                    Err(e) => {
                        if e.is::<crate::error::UnexpectedNullError>() {
                            f.write_str("NULL")
                        } else {
                            f.write_fmt(format_args!(
                                "(error decoding SQL type {} as {}: {e:?})",
                                info.name(),
                                std::any::type_name::<T>()
                            ))
                        }
                    }
                }
            },
        }
    }

    /// If the type to be decoded is not known or not supported, print the SQL type instead,
    /// as well as any applicable SQLx feature that needs to be enabled.
    pub fn unknown(value: &'v <DB as Database>::Value) -> Self
    where
        DB: TypeChecking,
    {
        Self {
            value,
            fmt: |value, f| {
                let info = value.type_info();

                if let Some(feature_gate) = <DB as TypeChecking>::get_feature_gate(&info) {
                    return f.write_fmt(format_args!(
                        "(unknown SQL type {}: SQLx feature {feature_gate} not enabled)",
                        info.name()
                    ));
                }

                f.write_fmt(format_args!("(unknown SQL type {})", info.name()))
            },
        }
    }
}

impl<'v, DB> Debug for FmtValue<'v, DB>
where
    DB: Database,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self.fmt)(self.value, f)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! select_input_type {
    ($ty:ty, $input:ty) => {
        stringify!($input)
    };
    ($ty:ty) => {
        stringify!($ty)
    };
}

#[macro_export]
macro_rules! impl_type_checking {
    (
        $database:path {
            $($(#[$meta:meta])? $ty:ty $(| $input:ty)?),*$(,)?
        },
        ParamChecking::$param_checking:ident,
        feature-types: $ty_info:ident => $get_gate:expr,
    ) => {
        impl $crate::type_checking::TypeChecking for $database {
            const PARAM_CHECKING: $crate::type_checking::ParamChecking = $crate::type_checking::ParamChecking::$param_checking;

            fn param_type_for_id(info: &Self::TypeInfo) -> Option<&'static str> {
                match () {
                    $(
                        $(#[$meta])?
                        _ if <$ty as sqlx_core::types::Type<$database>>::type_info() == *info => Some($crate::select_input_type!($ty $(, $input)?)),
                    )*
                    $(
                        $(#[$meta])?
                        _ if <$ty as sqlx_core::types::Type<$database>>::compatible(info) => Some($crate::select_input_type!($ty $(, $input)?)),
                    )*
                    _ => None
                }
            }

            fn return_type_for_id(info: &Self::TypeInfo) -> Option<&'static str> {
                match () {
                    $(
                        $(#[$meta])?
                        _ if <$ty as sqlx_core::types::Type<$database>>::type_info() == *info => Some(stringify!($ty)),
                    )*
                    $(
                        $(#[$meta])?
                        _ if <$ty as sqlx_core::types::Type<$database>>::compatible(info) => Some(stringify!($ty)),
                    )*
                    _ => None
                }
            }

            fn get_feature_gate($ty_info: &Self::TypeInfo) -> Option<&'static str> {
                $get_gate
            }

            fn fmt_value_debug(value: &Self::Value) -> $crate::type_checking::FmtValue<Self> {
                use $crate::value::Value;

                let info = value.type_info();

                match () {
                    $(
                        $(#[$meta])?
                        _ if <$ty as sqlx_core::types::Type<$database>>::compatible(&info) => $crate::type_checking::FmtValue::debug::<$ty>(value),
                    )*
                    _ => $crate::type_checking::FmtValue::unknown(value),
                }
            }
        }
    };
}
