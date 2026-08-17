#[cfg(feature = "arrayvec")]
#[test]
fn array_string_value() {
    use arrayvec::ArrayString;
    use std::str::FromStr;
    use zvariant::{LE, serialized::Context, to_bytes};

    #[macro_use]
    mod common {
        include!("common.rs");
    }
    let s = ArrayString::<32>::from_str("hello world!").unwrap();
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(encoded.len(), 17);
    let decoded: ArrayString<32> = encoded.deserialize().unwrap().0;
    assert_eq!(&decoded, "hello world!");
}
