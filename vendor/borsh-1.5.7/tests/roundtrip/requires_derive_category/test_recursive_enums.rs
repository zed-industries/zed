use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum ERecD {
    B { x: String, y: i32 },
    C(u8, Vec<ERecD>),
}

#[test]
fn test_recursive_enum() {
    let one = ERecD::B {
        x: "one".to_string(),
        y: 3213123,
    };
    let two = ERecD::C(10, vec![]);

    let three = ERecD::C(11, vec![one, two]);
    let data = to_vec(&three).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_three = from_slice::<ERecD>(&data).unwrap();
    assert_eq!(three, actual_three);
}
