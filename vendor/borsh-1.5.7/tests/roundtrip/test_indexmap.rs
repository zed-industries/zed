use borsh::BorshDeserialize;
use indexmap::{IndexMap, IndexSet};

#[test]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L100
// license: MIT OR Apache-2.0
fn test_indexmap_roundtrip() {
    let original_map: IndexMap<i32, i32> = {
        let mut map = IndexMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        map.insert(5, 6);
        map
    };
    let serialized_map = borsh::to_vec(&original_map).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(serialized_map);

    let deserialized_map: IndexMap<i32, i32> =
        BorshDeserialize::try_from_slice(&serialized_map).unwrap();
    assert_eq!(original_map, deserialized_map);
}

#[test]
// Taken from https://github.com/indexmap-rs/indexmap/blob/dd06e5773e4f91748396c67d00c83637f5c0dd49/src/borsh.rs#L115
// license: MIT OR Apache-2.0
fn test_indexset_roundtrip() {
    let mut original_set = IndexSet::new();
    [1, 2, 3, 4, 5, 6].iter().for_each(|&i| {
        original_set.insert(i);
    });

    let serialized_set = borsh::to_vec(&original_set).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(serialized_set);

    let deserialized_set: IndexSet<i32> =
        BorshDeserialize::try_from_slice(&serialized_set).unwrap();
    assert_eq!(original_set, deserialized_set);
}
