#![deny(unused_qualifications)]

#[macro_use]
extern crate num_derive;
use num::FromPrimitive;
use num::ToPrimitive;

#[derive(FromPrimitive, ToPrimitive)]
pub enum SomeEnum {
    A = 1,
}

#[test]
fn test_unused_qualifications() {
    assert!(SomeEnum::from_u64(1).is_some());
    assert!(SomeEnum::from_i64(-1).is_none());
    assert!(SomeEnum::A.to_i64().is_some());
}
