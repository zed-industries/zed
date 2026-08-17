use hashlink::{linked_hash_map, LinkedHashMap};

#[allow(dead_code)]
fn assert_covariance() {
    fn set<'new>(v: LinkedHashMap<&'static str, ()>) -> LinkedHashMap<&'new str, ()> {
        v
    }

    fn iter<'a, 'new>(
        v: linked_hash_map::Iter<'a, &'static str, &'static str>,
    ) -> linked_hash_map::Iter<'a, &'new str, &'new str> {
        v
    }

    fn iter_mut<'a, 'new>(
        v: linked_hash_map::Iter<'a, &'static str, ()>,
    ) -> linked_hash_map::Iter<'a, &'new str, ()> {
        v
    }

    fn into_iter<'new>(
        v: linked_hash_map::IntoIter<&'static str, &'static str>,
    ) -> linked_hash_map::IntoIter<&'new str, &'new str> {
        v
    }

    fn drain<'new>(
        d: linked_hash_map::Drain<'static, &'static str, &'static str>,
    ) -> linked_hash_map::Drain<'new, &'new str, &'new str> {
        d
    }

    fn raw_entry_builder<'a, 'new>(
        v: linked_hash_map::RawEntryBuilder<'a, &'static str, &'static str, ()>,
    ) -> linked_hash_map::RawEntryBuilder<'a, &'new str, &'new str, ()> {
        v
    }
}

#[test]
fn test_index() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(10, map[&1]);
    map[&2] = 22;
    assert_eq!(22, map[&2]);
}

#[test]
fn test_insert_and_get() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.get(&1), Some(&10));
    assert_eq!(map.get(&2), Some(&20));
    assert_eq!(map.len(), 2);
}

#[test]
fn test_insert_update() {
    let mut map = LinkedHashMap::new();
    map.insert("1".to_string(), vec![10, 10]);
    map.insert("1".to_string(), vec![10, 19]);
    assert_eq!(map.get(&"1".to_string()), Some(&vec![10, 19]));
    assert_eq!(map.len(), 1);
}

#[test]
fn test_entry_insert_vacant() {
    let mut map = LinkedHashMap::new();
    match map.entry("1".to_string()) {
        linked_hash_map::Entry::Vacant(e) => {
            assert_eq!(*e.insert(vec![10, 10]), vec![10, 10]);
        }
        _ => panic!("fail"),
    }
    assert!(map.contains_key("1"));
    assert_eq!(map["1"], vec![10, 10]);

    match map.entry("1".to_string()) {
        linked_hash_map::Entry::Occupied(mut e) => {
            assert_eq!(*e.get(), vec![10, 10]);
            assert_eq!(e.insert(vec![10, 16]), vec![10, 10]);
        }
        _ => panic!("fail"),
    }

    assert!(map.contains_key("1"));
    assert_eq!(map["1"], vec![10, 16]);

    match map.entry("1".to_string()) {
        linked_hash_map::Entry::Occupied(e) => {
            assert_eq!(e.remove(), vec![10, 16]);
        }
        _ => panic!("fail"),
    }
}

#[test]
fn test_remove() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    map.remove(&3);
    map.remove(&4);
    assert!(map.get(&3).is_none());
    assert!(map.get(&4).is_none());
    map.insert(6, 60);
    map.insert(7, 70);
    map.insert(8, 80);
    assert_eq!(map.get(&6), Some(&60));
    assert_eq!(map.get(&7), Some(&70));
    assert_eq!(map.get(&8), Some(&80));
}

#[test]
fn test_pop() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);
    assert_eq!(map.pop_front(), Some((1, 10)));
    assert!(map.get(&1).is_none());
    assert_eq!(map.pop_back(), Some((5, 50)));
    assert!(map.get(&5).is_none());
    map.insert(6, 60);
    map.insert(7, 70);
    map.insert(8, 80);
    assert_eq!(map.pop_front(), Some((2, 20)));
    assert!(map.get(&2).is_none());
    assert_eq!(map.pop_back(), Some((8, 80)));
    assert!(map.get(&8).is_none());
    map.insert(3, 30);
    assert_eq!(map.pop_front(), Some((4, 40)));
    assert!(map.get(&4).is_none());
    assert_eq!(map.pop_back(), Some((3, 30)));
    assert!(map.get(&3).is_none());
}

#[test]
fn test_move() {
    let to_back = |map: &mut LinkedHashMap<_, _>, key| match map.entry(key) {
        linked_hash_map::Entry::Occupied(mut occupied) => occupied.to_back(),
        linked_hash_map::Entry::Vacant(_) => panic!(),
    };

    let to_front = |map: &mut LinkedHashMap<_, _>, key| match map.entry(key) {
        linked_hash_map::Entry::Occupied(mut occupied) => occupied.to_front(),
        linked_hash_map::Entry::Vacant(_) => panic!(),
    };

    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    map.insert(5, 50);

    to_back(&mut map, 1);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![2, 3, 4, 5, 1]);

    to_front(&mut map, 4);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![4, 2, 3, 5, 1]);

    to_back(&mut map, 3);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![4, 2, 5, 1, 3]);

    to_front(&mut map, 2);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![2, 4, 5, 1, 3]);

    to_back(&mut map, 3);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![2, 4, 5, 1, 3]);

    to_front(&mut map, 2);
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), vec![2, 4, 5, 1, 3]);
}

#[test]
fn test_clear() {
    let mut map = LinkedHashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.clear();
    assert!(map.get(&1).is_none());
    assert!(map.get(&2).is_none());
    assert!(map.is_empty());
}

#[test]
fn test_iter() {
    let mut map = LinkedHashMap::new();

    // empty iter
    assert_eq!(None, map.iter().next());

    map.insert("a", 10);
    map.insert("b", 20);
    map.insert("c", 30);

    // regular iter
    let mut iter = map.iter();
    assert_eq!((&"a", &10), iter.next().unwrap());
    assert_eq!((&"b", &20), iter.next().unwrap());
    assert_eq!((&"c", &30), iter.next().unwrap());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());

    let mut iter = map.iter();
    assert_eq!((&"a", &10), iter.next().unwrap());
    let mut iclone = iter.clone();
    assert_eq!((&"b", &20), iter.next().unwrap());
    assert_eq!((&"b", &20), iclone.next().unwrap());
    assert_eq!((&"c", &30), iter.next().unwrap());
    assert_eq!((&"c", &30), iclone.next().unwrap());

    // reversed iter
    let mut rev_iter = map.iter().rev();
    assert_eq!((&"c", &30), rev_iter.next().unwrap());
    assert_eq!((&"b", &20), rev_iter.next().unwrap());
    assert_eq!((&"a", &10), rev_iter.next().unwrap());
    assert_eq!(None, rev_iter.next());
    assert_eq!(None, rev_iter.next());

    // mixed
    let mut mixed_iter = map.iter();
    assert_eq!((&"a", &10), mixed_iter.next().unwrap());
    assert_eq!((&"c", &30), mixed_iter.next_back().unwrap());
    assert_eq!((&"b", &20), mixed_iter.next().unwrap());
    assert_eq!(None, mixed_iter.next());
    assert_eq!(None, mixed_iter.next_back());
}

#[test]
fn test_borrow() {
    #[derive(PartialEq, Eq, Hash)]
    struct Foo(Bar);
    #[derive(PartialEq, Eq, Hash)]
    struct Bar(i32);

    impl ::std::borrow::Borrow<Bar> for Foo {
        fn borrow(&self) -> &Bar {
            &self.0
        }
    }

    let mut map = LinkedHashMap::new();
    map.insert(Foo(Bar(1)), "a");
    map.insert(Foo(Bar(2)), "b");

    assert!(map.contains_key(&Bar(1)));
    assert!(map.contains_key(&Bar(2)));
    assert!(map.contains_key(&Foo(Bar(1))));
    assert!(map.contains_key(&Foo(Bar(2))));

    assert_eq!(map.get(&Bar(1)), Some(&"a"));
    assert_eq!(map.get(&Bar(2)), Some(&"b"));
    assert_eq!(map.get(&Foo(Bar(1))), Some(&"a"));
    assert_eq!(map.get(&Foo(Bar(2))), Some(&"b"));

    assert_eq!(map.get_mut(&Bar(1)), Some(&mut "a"));
    assert_eq!(map.get_mut(&Bar(2)), Some(&mut "b"));
    assert_eq!(map.get_mut(&Foo(Bar(1))), Some(&mut "a"));
    assert_eq!(map.get_mut(&Foo(Bar(2))), Some(&mut "b"));

    assert_eq!(map[&Bar(1)], "a");
    assert_eq!(map[&Bar(2)], "b");
    assert_eq!(map[&Foo(Bar(1))], "a");
    assert_eq!(map[&Foo(Bar(2))], "b");

    assert_eq!(map.remove(&Bar(1)), Some("a"));
    assert_eq!(map.remove(&Bar(2)), Some("b"));
    assert_eq!(map.remove(&Foo(Bar(1))), None);
    assert_eq!(map.remove(&Foo(Bar(2))), None);
}

#[test]
fn test_iter_mut() {
    let mut map = LinkedHashMap::new();
    map.insert("a", 10);
    map.insert("c", 30);
    map.insert("b", 20);

    {
        let mut iter = map.iter_mut();
        let entry = iter.next().unwrap();
        assert_eq!("a", *entry.0);
        *entry.1 = 17;

        assert_eq!(format!("{:?}", iter), "[(\"c\", 30), (\"b\", 20)]");

        // reverse iterator
        let mut iter = iter.rev();
        let entry = iter.next().unwrap();
        assert_eq!("b", *entry.0);
        *entry.1 = 23;

        let entry = iter.next().unwrap();
        assert_eq!("c", *entry.0);
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    assert_eq!(17, map[&"a"]);
    assert_eq!(23, map[&"b"]);
}

#[test]
fn test_consuming_iter() {
    let map = {
        let mut map = LinkedHashMap::new();
        map.insert("a", 10);
        map.insert("c", 30);
        map.insert("b", 20);
        map
    };

    let mut iter = map.into_iter();
    assert_eq!(Some(("a", 10)), iter.next());
    assert_eq!(Some(("b", 20)), iter.next_back());
    assert_eq!(iter.len(), 1);
    assert_eq!(format!("{:?}", iter), "[(\"c\", 30)]");
    assert_eq!(Some(("c", 30)), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_consuming_iter_empty() {
    let map = LinkedHashMap::<&str, i32>::new();
    let mut iter = map.into_iter();
    assert_eq!(None, iter.next());
}

#[test]
fn test_consuming_iter_with_free_list() {
    let mut map = LinkedHashMap::new();
    map.insert("a", 10);
    map.insert("c", 30);
    map.insert("b", 20);
    map.remove("a");
    map.remove("b");

    let mut iter = map.into_iter();
    assert_eq!(Some(("c", 30)), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_into_iter_drop() {
    struct Counter<'a>(&'a mut usize);

    impl<'a> Drop for Counter<'a> {
        fn drop(&mut self) {
            *self.0 += 1;
        }
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    {
        let mut map = LinkedHashMap::new();
        map.insert("a", Counter(&mut a));
        map.insert("b", Counter(&mut b));
        map.insert("c", Counter(&mut c));

        let mut iter = map.into_iter();
        assert_eq!(iter.next().map(|p| p.0), Some("a"));
        assert_eq!(iter.next_back().map(|p| p.0), Some("c"));
    }

    assert_eq!(a, 1);
    assert_eq!(b, 1);
    assert_eq!(c, 1);
}

#[test]
fn test_drain() {
    use std::{cell::Cell, rc::Rc};

    struct Counter(Rc<Cell<u32>>);

    impl<'a> Drop for Counter {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let mut map = LinkedHashMap::new();

    let a = Rc::new(Cell::new(0));
    let b = Rc::new(Cell::new(0));
    let c = Rc::new(Cell::new(0));

    map.insert("a", Counter(a.clone()));
    map.insert("b", Counter(b.clone()));
    map.insert("c", Counter(c.clone()));

    let mut iter = map.drain();
    assert_eq!(iter.next().map(|p| p.0), Some("a"));
    assert_eq!(iter.next_back().map(|p| p.0), Some("c"));
    assert_eq!(iter.next_back().map(|p| p.0), Some("b"));
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());

    drop(iter);
    assert_eq!(map.len(), 0);

    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 1);
    assert_eq!(c.get(), 1);

    map.insert("a", Counter(a.clone()));
    map.insert("b", Counter(b.clone()));
    map.insert("c", Counter(c.clone()));

    let mut iter = map.drain();
    assert_eq!(iter.next().map(|p| p.0), Some("a"));
    assert_eq!(iter.next().map(|p| p.0), Some("b"));
    assert_eq!(iter.next_back().map(|p| p.0), Some("c"));
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());

    drop(iter);
    assert_eq!(map.len(), 0);

    assert_eq!(a.get(), 2);
    assert_eq!(b.get(), 2);
    assert_eq!(c.get(), 2);

    map.insert("a", Counter(a.clone()));
    map.insert("b", Counter(b.clone()));
    map.insert("c", Counter(c.clone()));

    map.drain();
    assert_eq!(map.len(), 0);

    assert_eq!(a.get(), 3);
    assert_eq!(b.get(), 3);
    assert_eq!(c.get(), 3);
}

#[test]
fn test_send_sync() {
    fn is_send_sync<T: Send + Sync>() {}

    is_send_sync::<LinkedHashMap<u32, i32>>();
    is_send_sync::<linked_hash_map::Entry<u32, i32, ()>>();
    is_send_sync::<linked_hash_map::RawEntryBuilder<u32, i32, ()>>();
    is_send_sync::<linked_hash_map::RawEntryBuilderMut<u32, i32, ()>>();
    is_send_sync::<linked_hash_map::RawEntryMut<u32, i32, ()>>();
    is_send_sync::<linked_hash_map::Iter<u32, i32>>();
    is_send_sync::<linked_hash_map::IterMut<u32, i32>>();
    is_send_sync::<linked_hash_map::Drain<u32, i32>>();
    is_send_sync::<linked_hash_map::Keys<u32, i32>>();
    is_send_sync::<linked_hash_map::Values<u32, i32>>();
}

#[test]
fn test_retain() {
    use std::{cell::Cell, rc::Rc};

    let xs = [1, 2, 3, 4, 5, 6];
    let mut map: LinkedHashMap<String, i32> = xs.iter().map(|i| (i.to_string(), *i)).collect();
    map.retain(|_, v| *v % 2 == 0);
    assert_eq!(map.len(), 3);
    assert!(map.contains_key("2"));
    assert!(map.contains_key("4"));
    assert!(map.contains_key("6"));

    struct Counter(Rc<Cell<u32>>);

    impl<'a> Drop for Counter {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let c = Rc::new(Cell::new(0));

    let mut map = LinkedHashMap::new();
    map.insert(1, Counter(Rc::clone(&c)));
    map.insert(2, Counter(Rc::clone(&c)));
    map.insert(3, Counter(Rc::clone(&c)));
    map.insert(4, Counter(Rc::clone(&c)));

    map.retain(|k, _| *k % 2 == 0);

    assert!(c.get() == 2);
    drop(map);
    assert!(c.get() == 4);
}

#[test]
fn test_order_equality() {
    let xs = [1, 2, 3, 4, 5, 6];
    let mut map1: LinkedHashMap<String, i32> = xs.iter().map(|i| (i.to_string(), *i)).collect();
    let mut map2: LinkedHashMap<String, i32> = xs.iter().map(|i| (i.to_string(), *i)).collect();

    assert_eq!(map1, map2);

    map1.to_front("4");
    assert_ne!(map1, map2);

    map2.to_front("4");
    assert_eq!(map1, map2);
}

#[test]
fn test_replace() {
    let mut map = LinkedHashMap::new();

    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.insert(4, 4);

    assert!(map
        .iter()
        .map(|(k, v)| (*k, *v))
        .eq([(1, 1), (2, 2), (3, 3), (4, 4)].iter().copied()));

    map.insert(3, 5);

    assert!(map
        .iter()
        .map(|(k, v)| (*k, *v))
        .eq([(1, 1), (2, 2), (4, 4), (3, 5)].iter().copied()));

    map.replace(2, 6);

    assert!(map
        .iter()
        .map(|(k, v)| (*k, *v))
        .eq([(1, 1), (2, 6), (4, 4), (3, 5)].iter().copied()));
}

#[test]
fn test_shrink_to_fit_resize() {
    let mut map = LinkedHashMap::new();
    map.shrink_to_fit();

    for i in 0..100 {
        map.insert(i, i);
    }
    map.shrink_to_fit();

    for _ in 0..50 {
        map.pop_front();
        map.shrink_to_fit();
    }

    assert_eq!(map.len(), 50);
    for i in 50..100 {
        assert_eq!(map.get(&i).unwrap(), &i);
    }
}
