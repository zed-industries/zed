use serde::{Deserialize, Serialize};
use serde_json::json;
use zvariant::{BE, LE, OwnedValue, Value, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn value_value() {
    let ctxt = Context::new_dbus(BE, 0);
    let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
    assert_eq!(encoded.len(), 8);
    assert_eq!(LE.read_u64(&encoded), 0xBAAB_BAAB_BAAB_BAAB_u64);
    let decoded: u64 = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, 0xABBA_ABBA_ABBA_ABBA);

    // Lie about there being bytes before
    let ctxt = Context::new_dbus(LE, 2);
    let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
    assert_eq!(encoded.len(), 14);
    let decoded: u64 = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, 0xABBA_ABBA_ABBA_ABBA_u64);
    let ctxt = Context::new_dbus(LE, 0);

    // As Value
    let v: Value<'_> = 0xFEFE_u64.into();
    assert_eq!(v.value_signature(), "t");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 16);
    let v = encoded.deserialize().unwrap().0;
    assert_eq!(v, Value::U64(0xFEFE));

    // And now as Value in a Value
    let v = Value::Value(Box::new(v));
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 16);
    let v = encoded.deserialize().unwrap().0;
    if let Value::Value(v) = v {
        assert_eq!(v.value_signature(), "t");
        assert_eq!(*v, Value::U64(0xFEFE));
    } else {
        panic!();
    }

    // Ensure Value works with other Serializer & Deserializer
    let v: Value<'_> = 0xFEFE_u64.into();
    let encoded = serde_json::to_string(&v).unwrap();
    let v = serde_json::from_str::<Value<'_>>(&encoded).unwrap();
    assert_eq!(v, Value::U64(0xFEFE));

    // Now a test case for https://github.com/z-galaxy/zbus/issues/549
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Data {
        inner: OwnedValue,
    }

    let value = zvariant::Value::new("variant-value");
    let inner = zvariant::StructureBuilder::new()
        .add_field("value1".to_string())
        .add_field("value2")
        .append_field(zvariant::Value::new(value)) // let's try to get a variant
        .build()
        .unwrap()
        .try_into()
        .unwrap();

    let data = Data { inner };
    let as_json = serde_json::to_value(&data).unwrap();
    let expected_json = json!(
        {
            "inner": {
                "signature": "(ssv)",
                "value": [
                    "value1",
                    "value2",
                    {
                        "signature": "s",
                        "value": "variant-value"
                    }
                ]
            }
        }
    );
    assert_eq!(expected_json, as_json);
    let data_again: Data = serde_json::from_str(&as_json.to_string()).unwrap();
    assert_eq!(data, data_again);
}
