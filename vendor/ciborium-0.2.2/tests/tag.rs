// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use ciborium::{de::from_reader, ser::into_writer, tag::*, value::Value};
use rstest::rstest;
use serde::{de::DeserializeOwned, Serialize};

use core::fmt::Debug;

#[rstest(item, bytes, value, encode, success,
    case(Captured(Some(6), true), "c6f5", Value::Tag(6, Value::Bool(true).into()), true, true),
    case(Captured(None, true), "f5", Value::Bool(true), true, true),

    case(Required::<_, 6>(true), "c6f5", Value::Tag(6, Value::Bool(true).into()), true, true),
    case(Required::<_, 6>(true), "c7f5", Value::Tag(7, Value::Bool(true).into()), false, false),
    case(Required::<_, 6>(true), "f5", Value::Bool(true), false, false),

    case(Accepted::<_, 6>(true), "c6f5", Value::Tag(6, Value::Bool(true).into()), true, true),
    case(Accepted::<_, 6>(true), "c7f5", Value::Tag(7, Value::Bool(true).into()), false, false),
    case(Accepted::<_, 6>(true), "f5", Value::Bool(true), false, true),
)]
fn test<T: Serialize + DeserializeOwned + Debug + Eq>(
    item: T,
    bytes: &str,
    value: Value,
    encode: bool,
    success: bool,
) {
    let bytes = hex::decode(bytes).unwrap();

    if encode {
        // Encode into bytes
        let mut encoded = Vec::new();
        into_writer(&item, &mut encoded).unwrap();
        assert_eq!(bytes, encoded);

        // Encode into value
        assert_eq!(value, Value::serialized(&item).unwrap());
    }

    // Decode from bytes
    match from_reader(&bytes[..]) {
        Ok(x) if success => assert_eq!(item, x),
        Ok(..) => panic!("unexpected success"),
        Err(e) if success => Err(e).unwrap(),
        Err(..) => (),
    }

    // Decode from value
    match value.deserialized() {
        Ok(x) if success => assert_eq!(item, x),
        Ok(..) => panic!("unexpected success"),
        Err(e) if success => Err(e).unwrap(),
        Err(..) => (),
    }
}
