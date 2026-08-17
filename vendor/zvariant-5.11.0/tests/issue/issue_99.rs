use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zvariant::{LE, Type, Value, serialized::Context, to_bytes_for_signature};

#[test]
fn issue_99() {
    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct ZVStruct<'s>(#[serde(borrow)] HashMap<&'s str, Value<'s>>);

    let mut dict = HashMap::new();
    dict.insert("hi", Value::from("hello"));
    dict.insert("bye", Value::from("then"));

    let element = ZVStruct(dict);

    let ctxt = Context::new_gvariant(LE, 0);
    let signature = ZVStruct::SIGNATURE;

    let encoded = to_bytes_for_signature(ctxt, signature, &element).unwrap();
    let _: ZVStruct<'_> = encoded.deserialize_for_signature(signature).unwrap().0;
}
