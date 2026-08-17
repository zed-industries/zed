#![allow(clippy::float_cmp)]

use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};
use bson::oid::ObjectId;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct StructWithObjectId(i32, ObjectId, u8);

#[test]
fn test_object_id() {
    let obj = StructWithObjectId(
        123,
        ObjectId::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        33,
    );
    let serialized = to_vec(&obj).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(serialized);
    let deserialized: StructWithObjectId = from_slice(&serialized).unwrap();
    assert_eq!(obj, deserialized);
}
