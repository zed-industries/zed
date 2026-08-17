#![allow(clippy::float_cmp, clippy::cast_sign_loss)]

use serde_json::{Map, Value};

use crate::{compiler, paths::Location, types::JsonType, ValidationError};

#[inline]
pub(crate) fn map_get_u64<'a>(
    m: &'a Map<String, Value>,
    ctx: &compiler::Context,
    type_name: &str,
) -> Option<Result<u64, ValidationError<'a>>> {
    let value = m.get(type_name)?;
    match value.as_u64() {
        Some(n) => Some(Ok(n)),
        None if value.is_i64() => Some(Err(ValidationError::minimum(
            Location::new(),
            ctx.location().clone(),
            value,
            0.into(),
        ))),
        None => {
            if let Some(value) = value.as_f64() {
                if value.trunc() == value {
                    // NOTE: Imprecise cast as big integers are not supported yet
                    #[allow(clippy::cast_possible_truncation)]
                    return Some(Ok(value as u64));
                }
            }
            Some(Err(ValidationError::single_type_error(
                Location::new(),
                ctx.location().clone(),
                value,
                JsonType::Integer,
            )))
        }
    }
}

/// Fail if the input value is not `u64`.
pub(crate) fn fail_on_non_positive_integer(
    value: &Value,
    instance_path: Location,
) -> ValidationError<'_> {
    if value.is_i64() {
        ValidationError::minimum(Location::new(), instance_path, value, 0.into())
    } else {
        ValidationError::single_type_error(Location::new(), instance_path, value, JsonType::Integer)
    }
}
