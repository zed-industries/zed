use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

use alloc::{string::{String, ToString}, vec::Vec, vec};
#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
struct CRecB {
    a: String,
    b: Vec<CRecB>,
}

#[test]
fn test_recursive_struct() {
    let one = CRecB {
        a: "one".to_string(),
        b: vec![],
    };
    let two = CRecB {
        a: "two".to_string(),
        b: vec![],
    };

    let three = CRecB {
        a: "three".to_string(),
        b: vec![one, two],
    };

    let data = to_vec(&three).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_three = from_slice::<CRecB>(&data).unwrap();
    assert_eq!(three, actual_three);
}
