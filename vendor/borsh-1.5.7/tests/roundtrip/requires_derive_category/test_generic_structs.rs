use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};
use core::marker::PhantomData;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(hash_collections)]
use core::{cmp::Eq, hash::Hash};

#[cfg(feature = "std")]
use std::collections::HashMap;

use alloc::{
    string::{ToString, String},
    vec,
    vec::Vec,
};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum B<W, G> {
    X { f: Vec<W> },
    Y(G),
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A<T, F, G> {
    x: Vec<T>,
    y: String,
    b: B<F, G>,
    pd: PhantomData<T>,
    c: Result<T, G>,
    d: [u64; 5],
}

#[test]
fn test_generic_struct() {
    let a = A::<String, u64, String> {
        x: vec!["foo".to_string(), "bar".to_string()],
        pd: Default::default(),
        y: "world".to_string(),
        b: B::X { f: vec![1, 2] },
        c: Err("error".to_string()),
        d: [0, 1, 2, 3, 4],
    };
    let data = to_vec(&a).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_a = from_slice::<A<String, u64, String>>(&data).unwrap();
    assert_eq!(a, actual_a);
}

trait TraitName {
    type Associated;
    #[allow(unused)]
    fn method(&self);
}

impl TraitName for u32 {
    type Associated = String;
    fn method(&self) {}
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct Parametrized<T, V>
where
    T: TraitName,
{
    field: T::Associated,
    another: V,
}

#[test]
fn test_generic_associated_type_field() {
    let a = Parametrized::<u32, String> {
        field: "value".to_string(),
        another: "field".to_string(),
    };
    let data = to_vec(&a).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_a = from_slice::<Parametrized<u32, String>>(&data).unwrap();
    assert_eq!(a, actual_a);
}

/// `T: PartialOrd` bound is required for `BorshSerialize` derive to be successful
/// `T: Hash + Eq` bound is required for `BorshDeserialize` derive to be successful
#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize)]
struct C<T: Ord + Hash + Eq, U> {
    a: String,
    b: HashMap<T, U>,
}

#[cfg(hash_collections)]
#[test]
fn test_generic_struct_hashmap() {
    let mut hashmap = HashMap::new();
    hashmap.insert(34, "another".to_string());
    hashmap.insert(14, "value".to_string());
    let a = C::<u32, String> {
        a: "field".to_string(),
        b: hashmap,
    };
    let data = to_vec(&a).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(data);
    let actual_a = from_slice::<C<u32, String>>(&data).unwrap();
    assert_eq!(actual_a.b.get(&14), Some("value".to_string()).as_ref());
    assert_eq!(actual_a.b.get(&34), Some("another".to_string()).as_ref());
}
