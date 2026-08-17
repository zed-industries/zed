use serde::{Deserialize, Serialize};
use zvariant::{LE, Str, Structure, Type, Value, as_value, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn struct_value() {
    // Struct->Value
    let s: Value<'_> = ("a", "b", (1, 2)).into();

    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(dbg!(encoded.len()), 40);
    let decoded: Value<'_> = encoded.deserialize().unwrap().0;
    let s = <Structure<'_>>::try_from(decoded).unwrap();
    let outer = <(Str<'_>, Str<'_>, Structure<'_>)>::try_from(s).unwrap();
    assert_eq!(outer.0, "a");
    assert_eq!(outer.1, "b");

    let inner = <(i32, i32)>::try_from(outer.2).unwrap();
    assert_eq!(inner.0, 1);
    assert_eq!(inner.1, 2);

    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    struct Foo {
        val: u32,
    }

    let foo = Foo { val: 99 };
    let v = as_value::Serialize(&foo);
    let encoded = to_bytes(ctxt, &v).unwrap();
    let decoded: as_value::Deserialize<'_, Foo> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.0, foo);

    // Unit struct should be treated as a 0-sized tuple (the same as unit type)
    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    struct Unit;

    assert_eq!(Unit::SIGNATURE, "");
    let encoded = to_bytes(ctxt, &Unit).unwrap();
    assert_eq!(encoded.len(), 0);
    let _decoded: Unit = encoded.deserialize().unwrap().0;

    // Structs w/o fields should be treated as a unit struct.
    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    struct NoFields {}

    assert_eq!(NoFields::SIGNATURE, "y");
    let encoded = to_bytes(ctxt, &NoFields {}).unwrap();
    assert_eq!(encoded.len(), 1);
    let _decoded: NoFields = encoded.deserialize().unwrap().0;

    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let encoded = to_bytes(ctxt, &NoFields {}).unwrap();
        assert_eq!(encoded.len(), 1);
        let _decoded: NoFields = encoded.deserialize().unwrap().0;
    }
}
