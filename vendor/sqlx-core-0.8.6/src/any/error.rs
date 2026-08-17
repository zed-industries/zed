use std::any::type_name;

use crate::any::type_info::AnyTypeInfo;
use crate::any::Any;
use crate::error::BoxDynError;
use crate::type_info::TypeInfo;
use crate::types::Type;

pub(super) fn mismatched_types<T: Type<Any>>(ty: &AnyTypeInfo) -> BoxDynError {
    format!(
        "mismatched types; Rust type `{}` is not compatible with SQL type `{}`",
        type_name::<T>(),
        ty.name()
    )
    .into()
}
