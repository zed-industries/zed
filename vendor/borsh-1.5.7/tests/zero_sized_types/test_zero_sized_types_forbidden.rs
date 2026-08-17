use alloc::{string::ToString, vec, vec::Vec};

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

use alloc::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque};

use borsh::from_slice;
use borsh::to_vec;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

use borsh::error::ERROR_ZST_FORBIDDEN;
#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
struct A();

#[test]
fn test_deserialize_vec_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<Vec<A>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_vec_of_zst() {
    let v = vec![A()];
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_vec_of_unit_type() {
    let v = vec![(), (), ()];
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_vec_of_vec_of_unit_type() {
    let v: Vec<Vec<()>> = vec![vec![(), (), ()]];
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_deserialize_vec_deque_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<VecDeque<A>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_vec_deque_of_zst() {
    let v: VecDeque<A> = vec![A()].into();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_deserialize_linked_list_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<LinkedList<A>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_linked_list_of_zst() {
    let v: LinkedList<A> = vec![A()].into_iter().collect();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_deserialize_btreeset_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<BTreeSet<A>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_btreeset_of_zst() {
    let v: BTreeSet<A> = vec![A()].into_iter().collect();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[cfg(hash_collections)]
#[test]
fn test_deserialize_hashset_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<HashSet<A>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[cfg(hash_collections)]
#[test]
fn test_serialize_hashset_of_zst() {
    let v: HashSet<A> = vec![A()].into_iter().collect();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_deserialize_btreemap_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<BTreeMap<A, u64>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[test]
fn test_serialize_btreemap_of_zst() {
    let v: BTreeMap<A, u64> = vec![(A(), 42u64)].into_iter().collect();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[cfg(hash_collections)]
#[test]
fn test_deserialize_hashmap_of_zst() {
    let v = [0u8, 0u8, 0u8, 64u8];
    let res = from_slice::<HashMap<A, u64>>(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[cfg(hash_collections)]
#[test]
fn test_serialize_hashmap_of_zst() {
    let v: HashMap<A, u64> = vec![(A(), 42u64)].into_iter().collect();
    let res = to_vec(&v);
    assert_eq!(res.unwrap_err().to_string(), ERROR_ZST_FORBIDDEN);
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct B(u32);
#[test]
fn test_deserialize_non_zst() {
    let v = [1, 0, 0, 0, 64, 0, 0, 0];
    let res = Vec::<B>::try_from_slice(&v);
    assert!(res.is_ok());
}

#[test]
fn test_serialize_non_zst() {
    let v = vec![B(1)];
    let res = to_vec(&v);
    assert!(res.is_ok());
}
