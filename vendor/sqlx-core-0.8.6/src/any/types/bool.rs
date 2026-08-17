use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;

impl Type<Any> for bool {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Bool,
        }
    }
}

impl<'q> Encode<'q, Any> for bool {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Bool(*self));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for bool {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Bool(b) => Ok(b),
            other => other.unexpected(),
        }
    }
}
