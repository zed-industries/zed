use zvariant::{BE, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn unit() {
    let ctxt = Context::new_dbus(BE, 0);
    let encoded = to_bytes(ctxt, &()).unwrap();
    assert_eq!(encoded.len(), 0, "invalid encoding using `to_bytes`");
    let _: () = encoded
        .deserialize()
        .expect("invalid decoding using `from_slice`")
        .0;
}
