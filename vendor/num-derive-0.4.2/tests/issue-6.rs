#![deny(trivial_numeric_casts)]

#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive, ToPrimitive)]
pub enum SomeEnum {
    A = 1,
}

#[test]
fn test_trivial_numeric_casts() {
    use num::{FromPrimitive, ToPrimitive};
    assert!(SomeEnum::from_u64(1).is_some());
    assert!(SomeEnum::from_i64(-1).is_none());
    assert_eq!(SomeEnum::A.to_u64(), Some(1));
}
