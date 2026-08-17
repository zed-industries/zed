use zvariant::{LE, Signature, Value, signature};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn signature() {
    let sig: Signature = signature!("yys");

    // Structure will always add () around the signature if it's a struct.
    basic_type_test!(LE, DBus, sig, 7, Signature, 1);

    #[cfg(feature = "gvariant")]
    basic_type_test!(LE, GVariant, sig, 6, Signature, 1);

    // As Value
    let v: Value<'_> = sig.into();
    assert_eq!(v.value_signature(), "g");
    let encoded = value_test!(LE, DBus, v, 10);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    assert_eq!(v, Value::Signature(signature!("yys")));

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let encoded = value_test!(LE, GVariant, v, 8);
        let v = encoded.deserialize::<Value<'_>>().unwrap().0;
        assert_eq!(v, Value::Signature(signature!("yys")));
    }
}
