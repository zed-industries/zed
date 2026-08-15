// Tests for the `map_lookup_then_insert` lint.

#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {}

// ------------ SHOULD FIRE -----------

fn hash_map_if_let(map: &mut HashMap<String, usize>, key: String) -> usize {
    if let Some(value) = map.get(&key) {
        *value
    } else {
        map.insert(key, 1);
        1
    }
}

fn hash_map_match(map: &mut HashMap<String, usize>, key: String) -> usize {
    match map.get(&key).copied() {
        Some(value) => value,
        None => {
            map.insert(key, 1);
            1
        }
    }
}

// Fix: `map.entry(key).and_modify(|value| *value += 1).or_insert(1)`.
fn hash_map_get_mut(map: &mut HashMap<String, usize>, key: String) {
    match map.get_mut(&key) {
        Some(value) => *value += 1,
        None => {
            map.insert(key, 1);
        }
    }
}

fn hash_map_get_key_value(map: &mut HashMap<String, usize>, key: String) -> usize {
    match map.get_key_value(&key) {
        Some((_, value)) => *value,
        None => {
            map.insert(key, 1);
            1
        }
    }
}

fn btree_map_match(map: &mut BTreeMap<String, usize>, key: String) -> usize {
    match map.get(&key) {
        Some(value) => *value,
        None => {
            map.insert(key, 1);
            1
        }
    }
}

fn btree_map_if_let(map: &mut BTreeMap<String, usize>, key: String) -> usize {
    if let Some(value) = map.get(&key) {
        *value
    } else {
        map.insert(key, 1);
        1
    }
}

// Fix: `map.entry(key).and_modify(|value| *value += 1).or_insert(1)`.
fn btree_map_get_mut(map: &mut BTreeMap<String, usize>, key: String) {
    match map.get_mut(&key) {
        Some(value) => *value += 1,
        None => {
            map.insert(key, 1);
        }
    }
}

fn btree_map_get_key_value(map: &mut BTreeMap<String, usize>, key: String) -> usize {
    match map.get_key_value(&key) {
        Some((_, value)) => *value,
        None => {
            map.insert(key, 1);
            1
        }
    }
}

fn btree_map_let_else(map: &mut BTreeMap<String, usize>, key: String) -> usize {
    let Some(value) = map.get(&key) else {
        map.insert(key, 1);
        return 1;
    };
    *value
}

// `btree_map::Entry` has no `insert_entry`; the fix here is
// `map.entry(key).and_modify(|value| *value = 2)`.
fn btree_map_overwrite_when_present(map: &mut BTreeMap<String, usize>, key: String) {
    if let Some(_current) = map.get(&key) {
        map.insert(key, 2);
    }
}

// Sets repeat the search too. They need no entry API: the fix is plain
// `set.insert(value)`, which returns `false` when the value was present.
fn hash_set_if_let(set: &mut HashSet<String>, value: String) -> bool {
    if let Some(existing) = set.get(&value) {
        existing.is_empty()
    } else {
        set.insert(value)
    }
}

fn btree_set_match(set: &mut BTreeSet<String>, value: String) -> bool {
    match set.get(&value) {
        Some(_) => false,
        None => set.insert(value),
    }
}

fn hash_map_let_else(map: &mut HashMap<String, usize>, key: String) -> usize {
    let Some(value) = map.get(&key) else {
        map.insert(key, 1);
        return 1;
    };
    *value
}

fn expensive() -> usize {
    42
}

// Fix: `map.entry(key).or_insert_with(expensive)`.
fn fix_with_or_insert_with(map: &mut HashMap<String, usize>, key: String) {
    match map.get(&key) {
        Some(_) => {}
        None => {
            map.insert(key, expensive());
        }
    }
}

// Fix: `map.entry(key).or_insert_with_key(|key| key.len())`.
fn fix_with_or_insert_with_key(map: &mut HashMap<String, usize>, key: String) {
    match map.get(&key) {
        Some(_) => {}
        None => {
            let value = key.len();
            map.insert(key, value);
        }
    }
}

// Fix: `map.entry(key).or_default()`.
fn fix_with_or_default(map: &mut HashMap<String, usize>, key: String) {
    match map.get(&key) {
        Some(_) => {}
        None => {
            map.insert(key, usize::default());
        }
    }
}

// Fix: `map.entry(key).insert_entry(2)`, keeping the occupancy check.
fn fix_with_insert_entry(map: &mut HashMap<String, usize>, key: String) {
    if let Some(_current) = map.get(&key) {
        map.insert(key, 2);
    }
}

// Hasher aliases like `FxHashMap` resolve to `HashMap`, so they are covered.
type FxLikeMap = HashMap<
    String,
    usize,
    std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
>;

fn custom_hasher_alias(map: &mut FxLikeMap, key: String) {
    match map.get(&key) {
        Some(_) => {}
        None => {
            map.insert(key, 1);
        }
    }
}

// A third-party map (`hashbrown`, `indexmap`, ...) that offers an inherent
// `entry` method is covered.
struct EntryMap;

impl EntryMap {
    fn get(&self, _key: &str) -> Option<&usize> {
        None
    }

    fn insert(&mut self, _key: String, _value: usize) {}

    fn entry(&mut self, _key: String) -> std::collections::hash_map::Entry<'_, String, usize> {
        unimplemented!()
    }
}

// Clippy's `unnecessary_get_then_check` only covers the std collections, so
// the `get(..).is_none()` form on a third-party map is still ours to catch.
fn entry_compatible_custom_map(map: &mut EntryMap, key: String) {
    if map.get(&key).is_none() {
        map.insert(key, 1);
    }
}

// ------------- SHOULD NOT FIRE -------------

// `contains_key` followed by `insert` is Clippy's `map_entry` lint.
fn clippy_handles_contains_key(map: &mut HashMap<String, usize>, key: String) {
    if !map.contains_key(&key) {
        map.insert(key, 1);
    }
}

// `get(..).is_none()`/`is_some()` on std collections is Clippy's chain:
// `unnecessary_get_then_check` rewrites the condition to
// `contains_key`/`contains`, which `map_entry`/`set_contains_or_insert`
// rewrite again.
fn clippy_handles_hash_map_get_is_none(map: &mut HashMap<String, usize>, key: String) {
    if map.get(&key).is_none() {
        map.insert(key, 1);
    }
}

fn clippy_handles_hash_map_get_is_some(map: &mut HashMap<String, usize>, key: String) {
    if map.get(&key).is_some() {
        map.insert(key, 2);
    }
}

fn clippy_handles_btree_map_get_is_none(map: &mut BTreeMap<String, usize>, key: String) {
    if map.get(&key).is_none() {
        map.insert(key, 1);
    }
}

fn clippy_handles_hash_set_get_is_none(set: &mut HashSet<String>, value: String) {
    if set.get(&value).is_none() {
        set.insert(value);
    }
}

fn clippy_handles_btree_set_get_is_none(set: &mut BTreeSet<String>, value: String) {
    if set.get(&value).is_none() {
        set.insert(value);
    }
}

fn already_uses_entry_or_insert(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).or_insert(1);
}

fn already_uses_entry_or_insert_with(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).or_insert_with(expensive);
}

fn already_uses_entry_or_insert_with_key(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).or_insert_with_key(|key| key.len());
}

fn already_uses_entry_or_default(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).or_default();
}

fn already_uses_entry_and_modify(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).and_modify(|value| *value += 1).or_insert(1);
}

fn already_uses_entry_insert_entry(map: &mut HashMap<String, usize>, key: String) {
    map.entry(key).insert_entry(2);
}

fn already_uses_entry_key(map: &mut HashMap<String, usize>, key: String) -> usize {
    map.entry(key).key().len()
}

// `btree_map::Entry` offers every method that `hash_map::Entry` does except
// `insert_entry`, so there is no BTreeMap mirror of that case.
fn btree_already_uses_entry_or_insert(map: &mut BTreeMap<String, usize>, key: String) {
    map.entry(key).or_insert(1);
}

fn btree_already_uses_entry_or_insert_with(map: &mut BTreeMap<String, usize>, key: String) {
    map.entry(key).or_insert_with(expensive);
}

fn btree_already_uses_entry_or_insert_with_key(map: &mut BTreeMap<String, usize>, key: String) {
    map.entry(key).or_insert_with_key(|key| key.len());
}

fn btree_already_uses_entry_or_default(map: &mut BTreeMap<String, usize>, key: String) {
    map.entry(key).or_default();
}

fn btree_already_uses_entry_and_modify(map: &mut BTreeMap<String, usize>, key: String) {
    map.entry(key).and_modify(|value| *value += 1).or_insert(1);
}

fn btree_already_uses_entry_key(map: &mut BTreeMap<String, usize>, key: String) -> usize {
    map.entry(key).key().len()
}

// `contains` followed by `insert` is Clippy's `set_contains_or_insert` lint.
fn clippy_handles_set_contains(set: &mut HashSet<String>, value: String) {
    if !set.contains(&value) {
        set.insert(value);
    }
}

// A bare `insert` already searches only once.
fn set_insert_alone(set: &mut HashSet<String>, value: String) -> bool {
    set.insert(value)
}

fn btree_set_insert_alone(set: &mut BTreeSet<String>, value: String) -> bool {
    set.insert(value)
}

fn different_key(map: &mut HashMap<String, usize>, first: String, second: String) {
    match map.get(&first) {
        Some(_) => {}
        None => {
            map.insert(second, 1);
        }
    }
}

fn different_map(
    first: &HashMap<String, usize>,
    second: &mut HashMap<String, usize>,
    key: String,
) {
    match first.get(&key) {
        Some(_) => {}
        None => {
            second.insert(key, 1);
        }
    }
}

fn lookup_without_insert(map: &HashMap<String, usize>, key: &str) -> Option<usize> {
    map.get(key).copied()
}

fn insertion_in_deferred_closure(map: &mut HashMap<String, usize>, key: String) {
    match map.get(&key).copied() {
        Some(_) => {}
        None => {
            let _insert_later = || {
                map.insert(key, 1);
            };
        }
    }
}

// `CustomMap` has no `entry` method, so there is no entry API to suggest.
struct CustomMap;

impl CustomMap {
    fn get(&self, _key: &str) -> Option<&usize> {
        None
    }

    fn insert(&mut self, _key: String, _value: usize) {}
}

fn non_standard_map(map: &mut CustomMap, key: String) {
    if map.get(&key).is_none() {
        map.insert(key, 1);
    }
}

fn next_key() -> String {
    String::new()
}

fn side_effecting_key(map: &mut HashMap<String, usize>) {
    match map.get(&next_key()) {
        Some(_) => {}
        None => {
            map.insert(next_key(), 1);
        }
    }
}

fn global_map() -> &'static mut HashMap<String, usize> {
    unimplemented!()
}

fn side_effecting_map(key: String) {
    match global_map().get(&key) {
        Some(_) => {}
        None => {
            global_map().insert(key, 1);
        }
    }
}
