use zvariant::{LE, ObjectPath, Value};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn object_path_value() {
    let o = ObjectPath::try_from("/hello/world").unwrap();
    basic_type_test!(LE, DBus, o, 17, ObjectPath<'_>, 4);

    #[cfg(feature = "gvariant")]
    basic_type_test!(LE, GVariant, o, 13, ObjectPath<'_>, 1);

    // As Value
    let v: Value<'_> = o.into();
    assert_eq!(v.value_signature(), "o");
    let encoded = value_test!(LE, DBus, v, 21);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    assert_eq!(
        v,
        Value::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
    );

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let encoded = value_test!(LE, GVariant, v, 15);
        let v = encoded.deserialize::<Value<'_>>().unwrap().0;
        assert_eq!(
            v,
            Value::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
        );
    }
}
