use crate::any::{Any, AnyArgumentBuffer, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind, AnyValueRef};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;

impl Type<Any> for f32 {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Real,
        }
    }
}

impl<'q> Encode<'q, Any> for f32 {
    fn encode_by_ref(&self, buf: &mut AnyArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Real(*self));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for f32 {
    fn decode(value: AnyValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Real(r) => Ok(r),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for f64 {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Double,
        }
    }
}

impl<'q> Encode<'q, Any> for f64 {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Double(*self));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for f64 {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            // Widening is safe
            AnyValueKind::Real(r) => Ok(r as f64),
            AnyValueKind::Double(d) => Ok(d),
            other => other.unexpected(),
        }
    }
}
