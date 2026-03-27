//! Tests for the VecMap collection.
//!
//! This is in a sibling module so that the tests are guaranteed to only cover
//! states that can be created by the public API.

use crate::vecmap::*;

#[test]
fn test_entry_vacant_or_insert() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    let value = map.entry("a").or_insert(1);
    assert_eq!(*value, 1);
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &1)]);
}

#[test]
fn test_entry_occupied_or_insert_keeps_existing() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    map.entry("a").or_insert(1);
    let value = map.entry("a").or_insert(99);
    assert_eq!(*value, 1);
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &1)]);
}

#[test]
fn test_entry_or_insert_with() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    map.entry("a").or_insert_with(|| 42);
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &42)]);
}

#[test]
fn test_entry_or_insert_with_not_called_when_occupied() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    map.entry("a").or_insert(1);
    map.entry("a")
        .or_insert_with(|| panic!("should not be called"));
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &1)]);
}

#[test]
fn test_entry_or_insert_with_key() {
    let mut map: VecMap<&str, String> = VecMap::new();
    map.entry("hello").or_insert_with_key(|k| k.to_uppercase());
    assert_eq!(
        map.iter().collect::<Vec<_>>(),
        vec![(&"hello", &"HELLO".to_string())]
    );
}

#[test]
fn test_entry_or_insert_default() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    map.entry("a").or_insert_default();
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &0)]);
}

#[test]
fn test_entry_key() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    assert_eq!(*map.entry("a").key(), "a");
    map.entry("a").or_insert(1);
    assert_eq!(*map.entry("a").key(), "a");
}

#[test]
fn test_entry_mut_ref_can_be_updated() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    let value = map.entry("a").or_insert(0);
    *value = 5;
    assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&"a", &5)]);
}

#[test]
fn test_insertion_order_preserved() {
    let mut map: VecMap<&str, i32> = VecMap::new();
    map.entry("b").or_insert(2);
    map.entry("a").or_insert(1);
    map.entry("c").or_insert(3);
    assert_eq!(
        map.iter().collect::<Vec<_>>(),
        vec![(&"b", &2), (&"a", &1), (&"c", &3)]
    );
}

#[test]
fn test_multiple_entries_independent() {
    let mut map: VecMap<i32, i32> = VecMap::new();
    map.entry(1).or_insert(10);
    map.entry(2).or_insert(20);
    map.entry(3).or_insert(30);
    assert_eq!(map.iter().count(), 3);
    // Re-inserting does not duplicate keys
    map.entry(1).or_insert(99);
    assert_eq!(map.iter().count(), 3);
}
