#[test]
#[cfg(feature = "heapless")]
fn heapless_string_value() {
    use heapless::String;
    use zvariant::{LE, serialized::Context, to_bytes};

    #[macro_use]
    mod common {
        include!("common.rs");
    }

    let s = String::<32>::try_from("hello world!").unwrap();
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(encoded.len(), 17);
    let decoded: String<32> = encoded.deserialize().unwrap().0;
    assert_eq!(&decoded, "hello world!");
}
