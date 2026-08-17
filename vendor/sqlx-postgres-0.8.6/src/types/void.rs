use crate::decode::Decode;
use crate::error::BoxDynError;
use crate::types::Type;
use crate::{PgTypeInfo, PgValueRef, Postgres};

impl Type<Postgres> for () {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::VOID
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        // RECORD is here so we can support the empty tuple
        *ty == PgTypeInfo::VOID || *ty == PgTypeInfo::RECORD
    }
}

impl<'r> Decode<'r, Postgres> for () {
    fn decode(_value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(())
    }
}
