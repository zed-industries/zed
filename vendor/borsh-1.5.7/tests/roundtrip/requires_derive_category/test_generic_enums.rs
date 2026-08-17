use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum B<W, G> {
    X { f: Vec<W> },
    Y(G),
}

#[test]
fn test_generic_enum() {
    let b: B<String, u64> = B::X {
        f: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };
    let c: B<String, u64> = B::Y(656556u64);

    let list = vec![b, c];
    let data = to_vec(&list).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_list = from_slice::<Vec<B<String, u64>>>(&data).unwrap();

    assert_eq!(list, actual_list);
}
