use super::detail::{Entry, IndexMap};

type TestMap = IndexMap<char, i32>;

fn assert_get(map: &mut TestMap, key: char, expected: impl Into<Option<(usize, i32)>>) {
    let mut expected = expected.into();
    assert_eq!(map.contains_key(&key), expected.is_some());
    assert_eq!(
        map.get(&key),
        expected.as_ref().map(|(_index, value)| value)
    );
    assert_eq!(
        map.get_key_value(&key),
        expected.as_ref().map(|(_index, value)| (&key, value))
    );
    assert_eq!(
        map.get_full(&key),
        expected
            .as_ref()
            .map(|(index, value)| (*index, &key, value))
    );
    assert_eq!(
        map.get_index_of(&key),
        expected.map(|(index, _value)| index)
    );
    if let Some((index, _value)) = expected {
        assert_eq!(
            map.get_index(index),
            expected.as_ref().map(|(_index, value)| (&key, value))
        );
        assert_eq!(
            map.get_index_mut(index),
            expected.as_mut().map(|(_index, value)| (&key, value))
        );
    }
}

#[test]
fn new_works() {
    let mut map = <IndexMap<char, i32>>::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert!(map.iter().eq([]));
    assert!(map.keys().eq([].iter()));
    assert!(map.values().eq([].iter()));
    assert!(map.values_mut().eq([].iter()));
    assert!(map.iter_mut().eq([]));
    assert!(map.into_iter().eq([]));
}

#[test]
fn insert_works() {
    let mut map = <IndexMap<char, i32>>::new();
    let (k0, None) = map.insert_full('a', 10) else {
        panic!()
    };
    assert_get(&mut map, 'a', (k0, 10));
    let (k1, None) = map.insert_full('b', 20) else {
        panic!()
    };
    assert_get(&mut map, 'b', (k1, 20));
    assert_eq!(map.insert('a', 30), Some(10));
    assert_get(&mut map, 'a', (k0, 30));
    assert_eq!(map.len(), 2);
    assert!(!map.is_empty());
}

#[test]
fn extend_works() {
    let mut map = <IndexMap<char, i32>>::new();
    let mut values = [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)];
    map.extend(values);
    assert!(!map.is_empty());
    assert_eq!(map.len(), values.len());
    for (index, (key, value)) in values.into_iter().enumerate() {
        assert_get(&mut map, key, (index, value));
    }
    assert!(map.iter().eq(values.iter().map(|(k, v)| (k, v))));
    assert!(map.iter_mut().eq(values.iter_mut().map(|(k, v)| (&*k, v))));
    assert!(map.keys().eq(values.iter().map(|(k, _v)| k)));
    assert!(map.values().eq(values.iter().map(|(_k, v)| v)));
    assert!(map.values_mut().eq(values.iter_mut().map(|(_k, v)| v)));
    assert!(map.into_iter().eq(values));
}

#[test]
fn clear_works() {
    let mut map = <IndexMap<char, i32>>::new();
    map.extend([('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)]);
    map.clear();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert!(map.iter().eq([]));
    assert!(map.keys().eq([].iter()));
    assert!(map.values().eq([].iter()));
    assert!(map.values_mut().eq([].iter()));
    assert!(map.iter_mut().eq([]));
    assert!(map.into_iter().eq([]));
}

#[test]
fn swap_remove_works_ascending() {
    let mut map = <IndexMap<char, i32>>::new();
    let values = [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)];
    map.extend(values);
    assert_eq!(map.swap_remove_full(&'a'), Some((0, 'a', 0))); // moves 'f' to 0
    assert_eq!(map.swap_remove(&'a'), None);
    assert_eq!(map.swap_remove_full(&'b'), Some((1, 'b', 1))); // moves 'e' to 1
    assert_eq!(map.swap_remove(&'b'), None);
    assert_eq!(map.swap_remove_full(&'c'), Some((2, 'c', 2))); // moves 'd' to 2
    assert_eq!(map.swap_remove(&'c'), None);
    assert_eq!(map.swap_remove_full(&'d'), Some((2, 'd', 3)));
    assert_eq!(map.swap_remove(&'d'), None);
    assert_eq!(map.swap_remove_full(&'e'), Some((1, 'e', 4)));
    assert_eq!(map.swap_remove(&'e'), None);
    assert_eq!(map.swap_remove_full(&'f'), Some((0, 'f', 5)));
    assert_eq!(map.swap_remove(&'f'), None);
    for (key, _value) in values {
        assert_get(&mut map, key, None);
    }
}

#[test]
fn swap_remove_works_descending() {
    let mut map = <IndexMap<char, i32>>::new();
    let values = [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)];
    map.extend(values);
    assert_eq!(map.swap_remove_full(&'f'), Some((5, 'f', 5)));
    assert_eq!(map.swap_remove(&'f'), None);
    assert_eq!(map.swap_remove_full(&'e'), Some((4, 'e', 4)));
    assert_eq!(map.swap_remove(&'e'), None);
    assert_eq!(map.swap_remove_full(&'d'), Some((3, 'd', 3)));
    assert_eq!(map.swap_remove(&'d'), None);
    assert_eq!(map.swap_remove_full(&'c'), Some((2, 'c', 2)));
    assert_eq!(map.swap_remove(&'c'), None);
    assert_eq!(map.swap_remove_full(&'b'), Some((1, 'b', 1)));
    assert_eq!(map.swap_remove(&'b'), None);
    assert_eq!(map.swap_remove_full(&'a'), Some((0, 'a', 0)));
    assert_eq!(map.swap_remove(&'a'), None);
    for (key, _value) in values {
        assert_get(&mut map, key, None);
    }
}

#[test]
fn entry_works_occupied() {
    let mut map = <IndexMap<char, i32>>::new();
    let values = [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)];
    map.extend(values);
    for (key, mut value) in values {
        match map.entry(key) {
            Entry::Occupied(mut entry) => {
                assert_eq!(entry.get(), &value);
                assert_eq!(entry.get_mut(), &mut value);
                assert_eq!(entry.key(), &key);
                let new_value = value + 10;
                assert_eq!(entry.insert(new_value), value);
                assert_eq!(entry.get(), &new_value);
            }
            Entry::Vacant(_) => panic!(),
        }
    }
}

#[test]
fn entry_works_vacant() {
    let mut map = <IndexMap<char, i32>>::new();
    let values = [('a', 0), ('b', 1), ('c', 2), ('d', 3), ('e', 4), ('f', 5)];
    for (key, mut value) in values {
        match map.entry(key) {
            Entry::Occupied(_) => panic!(),
            Entry::Vacant(entry) => {
                assert_eq!(entry.key(), &key);
                assert_eq!(entry.insert(value), &mut value);
            }
        }
    }
    assert!(!map.is_empty());
    assert_eq!(map.len(), values.len());
    for (index, (key, value)) in values.into_iter().enumerate() {
        assert_get(&mut map, key, (index, value));
    }
}
