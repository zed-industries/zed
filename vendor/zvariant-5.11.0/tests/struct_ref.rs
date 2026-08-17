use zvariant::{LE, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn struct_ref() {
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &(&1u32, &2u32)).unwrap();
    let decoded: [u32; 2] = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, [1u32, 2u32]);
}
