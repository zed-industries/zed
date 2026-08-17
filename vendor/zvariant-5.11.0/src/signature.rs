use crate::{Basic, Type};

pub use zvariant_utils::signature::*;

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error::SignatureParse(e)
    }
}

impl Type for Signature {
    const SIGNATURE: &'static Signature = &Signature::Signature;
}

impl Basic for Signature {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
}

impl From<Signature> for crate::Value<'static> {
    fn from(value: Signature) -> Self {
        crate::Value::Signature(value)
    }
}
