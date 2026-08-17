use crate::Signature;

impl crate::Type for serde_bytes::Bytes {
    const SIGNATURE: &'static Signature = &Signature::static_array(&Signature::U8);
}

impl crate::Type for serde_bytes::ByteBuf {
    const SIGNATURE: &'static Signature = &Signature::static_array(&Signature::U8);
}
