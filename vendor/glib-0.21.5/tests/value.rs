use std::ops::Deref;

use glib::{prelude::*, BoxedValue, Value};

// FIXME all .get::<i32>() should be replaced with .get(); the compiler is totally able to infer the type itself.
// But somehow without some tests are failing on Windows because the type inference doesn't work or something.

// Test that `ToValue` (and conversely, `FromValue`) uphold the promised invariants
#[test]
pub fn to_value_invariants() {
    // Inverse
    assert_eq!(0i32, 0i32.to_value().get::<i32>().unwrap());
    assert_eq!(0i32, 0i32.to_value().get::<i32>().unwrap());

    // Idempotence
    assert_eq!(
        &0i32.to_value().type_(),
        &0i32.to_value().to_value().type_()
    );
    assert_eq!(0i32, 0i32.to_value().to_value().get::<i32>().unwrap());
    assert_eq!(
        0i32,
        0i32.to_value()
            .get::<Value>()
            .unwrap()
            .get::<i32>()
            .unwrap()
    );
    assert_eq!(
        0i32,
        0i32.to_value()
            .get::<Value>()
            .unwrap()
            .get::<i32>()
            .unwrap()
    );
}

// Test that `ToValue` and `FromValue` handle nested boxed values correctly (as per the documentation)
#[test]
pub fn to_value_boxed() {
    let x = 0i32.to_value();
    let boxed = BoxedValue(x);
    assert_eq!(
        0i32,
        boxed
            .to_value()
            .to_value()
            .get::<BoxedValue>()
            .unwrap()
            .deref()
            .get::<i32>()
            .unwrap()
    );
    assert_eq!(
        0i32,
        boxed
            .to_value()
            .get::<Value>()
            .unwrap()
            .get::<BoxedValue>()
            .unwrap()
            .deref()
            .get::<i32>()
            .unwrap()
    );
}
