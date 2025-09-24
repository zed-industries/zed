use std::fmt::Debug;

use pretty_assertions::assert_eq;
use proptest::prelude::*;
use serde_json::Value as JsonValue;

pub fn assert_roundtrips<T, U>(left: T)
where
    T: Clone + Debug + PartialEq + TryFrom<U>,
    U: TryFrom<T>,
    <T as TryFrom<U>>::Error: Debug,
    <U as TryFrom<T>>::Error: Debug,
{
    let right = T::try_from(U::try_from(left.clone()).unwrap()).unwrap();
    assert_eq!(left, right);
}

pub fn arb_json() -> impl Strategy<Value = JsonValue> {
    let leaf = prop_oneof![
        Just(JsonValue::Null),
        any::<bool>().prop_map(JsonValue::Bool),
        any::<i64>().prop_map(|v| JsonValue::Number(v.into())),
        ".*".prop_map(JsonValue::String),
    ];
    leaf.prop_recursive(
        4,   // 4 levels deep
        128, // Shoot for maximum size of 128 nodes
        5,   // We put up to 5 items per collection
        |inner| {
            prop_oneof![
                // Take the inner strategy and make the two recursive cases.
                prop::collection::vec(inner.clone(), 0..5).prop_map(JsonValue::Array),
                prop::collection::hash_map(".*", inner, 0..5)
                    .prop_map(|m| JsonValue::Object(m.into_iter().collect())),
            ]
        },
    )
}
