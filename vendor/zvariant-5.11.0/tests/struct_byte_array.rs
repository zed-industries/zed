use std::collections::HashMap;
use zvariant::{Value, serialized::Context};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn struct_byte_array() {
    let ctxt = Context::new_dbus(zvariant::LE, 0);
    let value: (Vec<u8>, HashMap<String, Value<'_>>) = (Vec::new(), HashMap::new());
    let value = zvariant::to_bytes(ctxt, &value).unwrap();
    #[cfg(feature = "serde_bytes")]
    let (bytes, map): (&serde_bytes::Bytes, HashMap<&str, Value<'_>>) = value
        .deserialize()
        .expect("Could not deserialize serde_bytes::Bytes in struct.")
        .0;
    #[cfg(not(feature = "serde_bytes"))]
    let (bytes, map): (&[u8], HashMap<&str, Value<'_>>) = value
        .deserialize()
        .expect("Could not deserialize u8 slice in struct")
        .0;

    assert!(bytes.is_empty());
    assert!(map.is_empty());
}
