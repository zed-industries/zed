#![allow(clippy::float_cmp)]

use borsh::{from_slice, to_vec};
#[cfg(feature = "derive")]
use borsh::{BorshDeserialize, BorshSerialize};

use alloc::string::{String, ToString};

macro_rules! test_array {
    ($v: expr, $t: ty, $len: expr) => {
        let buf = borsh::to_vec(&$v).unwrap();
        #[cfg(feature = "std")]
        insta::assert_debug_snapshot!(buf);
        let actual_v: [$t; $len] = from_slice(&buf).expect("failed to deserialize");
        assert_eq!($v.len(), actual_v.len());
        #[allow(clippy::reversed_empty_ranges)]
        for i in 0..$len {
            assert_eq!($v[i], actual_v[i]);
        }
    };
}

macro_rules! test_arrays {
    ($test_name: ident, $el: expr, $t: ty) => {
        #[test]
        fn $test_name() {
            test_array!([$el; 0], $t, 0);
            test_array!([$el; 1], $t, 1);
            test_array!([$el; 2], $t, 2);
            test_array!([$el; 3], $t, 3);
            test_array!([$el; 4], $t, 4);
            test_array!([$el; 8], $t, 8);
            test_array!([$el; 16], $t, 16);
            test_array!([$el; 32], $t, 32);
            test_array!([$el; 64], $t, 64);
            test_array!([$el; 65], $t, 65);
        }
    };
}

test_arrays!(test_array_u8, 100u8, u8);
test_arrays!(test_array_i8, 100i8, i8);
test_arrays!(test_array_u32, 1000000000u32, u32);
test_arrays!(test_array_u64, 1000000000000000000u64, u64);
test_arrays!(
    test_array_u128,
    1000000000000000000000000000000000000u128,
    u128
);
test_arrays!(test_array_f32, 1000000000.0f32, f32);
test_arrays!(test_array_array_u8, [100u8; 32], [u8; 32]);
test_arrays!(test_array_zst, (), ());

#[cfg(feature = "derive")]
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct CustomStruct(u8);

#[cfg(feature = "derive")]
#[test]
fn test_custom_struct_array() {
    let arr = [CustomStruct(0), CustomStruct(1), CustomStruct(2)];
    let serialized = to_vec(&arr).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(serialized);
    let deserialized: [CustomStruct; 3] = from_slice(&serialized).unwrap();
    assert_eq!(arr, deserialized);
}

#[test]
fn test_string_array() {
    let arr = ["0".to_string(), "1".to_string(), "2".to_string()];
    let serialized = to_vec(&arr).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(serialized);
    let deserialized: [String; 3] = from_slice(&serialized).unwrap();
    assert_eq!(arr, deserialized);
}
