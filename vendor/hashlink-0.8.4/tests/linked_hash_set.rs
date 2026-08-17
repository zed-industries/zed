use hashbrown::hash_map::DefaultHashBuilder;
use hashlink::linked_hash_set::{self, LinkedHashSet};

#[allow(dead_code)]
fn assert_covariance() {
    fn set<'new>(v: LinkedHashSet<&'static str>) -> LinkedHashSet<&'new str> {
        v
    }

    fn iter<'a, 'new>(
        v: linked_hash_set::Iter<'a, &'static str>,
    ) -> linked_hash_set::Iter<'a, &'new str> {
        v
    }

    fn into_iter<'new>(
        v: linked_hash_set::IntoIter<&'static str>,
    ) -> linked_hash_set::IntoIter<&'new str> {
        v
    }

    fn difference<'a, 'new>(
        v: linked_hash_set::Difference<'a, &'static str, DefaultHashBuilder>,
    ) -> linked_hash_set::Difference<'a, &'new str, DefaultHashBuilder> {
        v
    }

    fn symmetric_difference<'a, 'new>(
        v: linked_hash_set::SymmetricDifference<'a, &'static str, DefaultHashBuilder>,
    ) -> linked_hash_set::SymmetricDifference<'a, &'new str, DefaultHashBuilder> {
        v
    }

    fn intersection<'a, 'new>(
        v: linked_hash_set::Intersection<'a, &'static str, DefaultHashBuilder>,
    ) -> linked_hash_set::Intersection<'a, &'new str, DefaultHashBuilder> {
        v
    }

    fn union<'a, 'new>(
        v: linked_hash_set::Union<'a, &'static str, DefaultHashBuilder>,
    ) -> linked_hash_set::Union<'a, &'new str, DefaultHashBuilder> {
        v
    }

    fn drain<'new>(
        d: linked_hash_set::Drain<'static, &'static str>,
    ) -> linked_hash_set::Drain<'new, &'new str> {
        d
    }
}

#[test]
fn test_zero_capacities() {
    type HS = LinkedHashSet<i32>;

    let s = HS::new();
    assert_eq!(s.capacity(), 0);

    let s = HS::default();
    assert_eq!(s.capacity(), 0);

    let s = HS::with_hasher(DefaultHashBuilder::default());
    assert_eq!(s.capacity(), 0);

    let s = HS::with_capacity(0);
    assert_eq!(s.capacity(), 0);

    let s = HS::with_capacity_and_hasher(0, DefaultHashBuilder::default());
    assert_eq!(s.capacity(), 0);

    let mut s = HS::new();
    s.insert(1);
    s.insert(2);
    s.remove(&1);
    s.remove(&2);
    s.shrink_to_fit();
    assert_eq!(s.capacity(), 0);

    let mut s = HS::new();
    s.reserve(0);
    assert_eq!(s.capacity(), 0);
}

#[test]
fn test_disjoint() {
    let mut xs = LinkedHashSet::new();
    let mut ys = LinkedHashSet::new();
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(xs.insert(5));
    assert!(ys.insert(11));
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(xs.insert(7));
    assert!(xs.insert(19));
    assert!(xs.insert(4));
    assert!(ys.insert(2));
    assert!(ys.insert(-11));
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(ys.insert(7));
    assert!(!xs.is_disjoint(&ys));
    assert!(!ys.is_disjoint(&xs));
}

#[test]
fn test_subset_and_superset() {
    let mut a = LinkedHashSet::new();
    assert!(a.insert(0));
    assert!(a.insert(5));
    assert!(a.insert(11));
    assert!(a.insert(7));

    let mut b = LinkedHashSet::new();
    assert!(b.insert(0));
    assert!(b.insert(7));
    assert!(b.insert(19));
    assert!(b.insert(250));
    assert!(b.insert(11));
    assert!(b.insert(200));

    assert!(!a.is_subset(&b));
    assert!(!a.is_superset(&b));
    assert!(!b.is_subset(&a));
    assert!(!b.is_superset(&a));

    assert!(b.insert(5));

    assert!(a.is_subset(&b));
    assert!(!a.is_superset(&b));
    assert!(!b.is_subset(&a));
    assert!(b.is_superset(&a));
}

#[test]
fn test_iterate() {
    let mut a = LinkedHashSet::new();
    for i in 0..32 {
        assert!(a.insert(i));
    }
    let mut observed: u32 = 0;
    for k in &a {
        observed |= 1 << *k;
    }
    assert_eq!(observed, 0xFFFF_FFFF);
}

#[test]
fn test_intersection() {
    let mut a = LinkedHashSet::new();
    let mut b = LinkedHashSet::new();

    assert!(a.insert(11));
    assert!(a.insert(1));
    assert!(a.insert(3));
    assert!(a.insert(77));
    assert!(a.insert(103));
    assert!(a.insert(5));
    assert!(a.insert(-5));

    assert!(b.insert(2));
    assert!(b.insert(11));
    assert!(b.insert(77));
    assert!(b.insert(-9));
    assert!(b.insert(-42));
    assert!(b.insert(5));
    assert!(b.insert(3));

    let mut i = 0;
    let expected = [3, 5, 11, 77];
    for x in a.intersection(&b) {
        assert!(expected.contains(x));
        i += 1
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_difference() {
    let mut a = LinkedHashSet::new();
    let mut b = LinkedHashSet::new();

    assert!(a.insert(1));
    assert!(a.insert(3));
    assert!(a.insert(5));
    assert!(a.insert(9));
    assert!(a.insert(11));

    assert!(b.insert(3));
    assert!(b.insert(9));

    let mut i = 0;
    let expected = [1, 5, 11];
    for x in a.difference(&b) {
        assert!(expected.contains(x));
        i += 1
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference() {
    let mut a = LinkedHashSet::new();
    let mut b = LinkedHashSet::new();

    assert!(a.insert(1));
    assert!(a.insert(3));
    assert!(a.insert(5));
    assert!(a.insert(9));
    assert!(a.insert(11));

    assert!(b.insert(-2));
    assert!(b.insert(3));
    assert!(b.insert(9));
    assert!(b.insert(14));
    assert!(b.insert(22));

    let mut i = 0;
    let expected = [-2, 1, 5, 11, 14, 22];
    for x in a.symmetric_difference(&b) {
        assert!(expected.contains(x));
        i += 1
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_union() {
    let mut a = LinkedHashSet::new();
    let mut b = LinkedHashSet::new();

    assert!(a.insert(1));
    assert!(a.insert(3));
    assert!(a.insert(5));
    assert!(a.insert(9));
    assert!(a.insert(11));
    assert!(a.insert(16));
    assert!(a.insert(19));
    assert!(a.insert(24));

    assert!(b.insert(-2));
    assert!(b.insert(1));
    assert!(b.insert(5));
    assert!(b.insert(9));
    assert!(b.insert(13));
    assert!(b.insert(19));

    let mut i = 0;
    let expected = [-2, 1, 3, 5, 9, 11, 13, 16, 19, 24];
    for x in a.union(&b) {
        assert!(expected.contains(x));
        i += 1
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_from_iter() {
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let set: LinkedHashSet<_> = xs.iter().cloned().collect();

    for x in &xs {
        assert!(set.contains(x));
    }
}

#[test]
fn test_move_iter() {
    let hs = {
        let mut hs = LinkedHashSet::new();

        hs.insert('a');
        hs.insert('b');

        hs
    };

    let v = hs.into_iter().collect::<Vec<char>>();
    assert!(v == ['a', 'b'] || v == ['b', 'a']);
}

#[test]
fn test_eq() {
    let mut s1 = LinkedHashSet::new();

    s1.insert(1);
    s1.insert(2);
    s1.insert(3);

    let mut s2 = LinkedHashSet::new();

    s2.insert(1);
    s2.insert(2);

    assert!(s1 != s2);

    s2.insert(3);

    assert_eq!(s1, s2);
}

#[test]
fn test_show() {
    let mut set = LinkedHashSet::new();
    let empty = LinkedHashSet::<i32>::new();

    set.insert(1);
    set.insert(2);

    let set_str = format!("{:?}", set);

    assert!(set_str == "{1, 2}" || set_str == "{2, 1}");
    assert_eq!(format!("{:?}", empty), "{}");
}

#[test]
fn test_trivial_drain() {
    let mut s = LinkedHashSet::<i32>::new();
    for _ in s.drain() {}
    assert!(s.is_empty());
    drop(s);

    let mut s = LinkedHashSet::<i32>::new();
    drop(s.drain());
    assert!(s.is_empty());
}

#[test]
fn test_drain() {
    let mut s: LinkedHashSet<_> = (1..100).collect();

    for _ in 0..20 {
        assert_eq!(s.len(), 99);

        {
            let mut last_i = 0;
            let mut d = s.drain();
            for (i, x) in d.by_ref().take(50).enumerate() {
                last_i = i;
                assert!(x != 0);
            }
            assert_eq!(last_i, 49);
        }

        for _ in &s {
            panic!("s should be empty!");
        }

        s.extend(1..100);
    }
}

#[test]
fn test_replace() {
    use core::hash;

    #[derive(Debug)]
    struct Foo(&'static str, i32);

    impl PartialEq for Foo {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Foo {}

    impl hash::Hash for Foo {
        fn hash<H: hash::Hasher>(&self, h: &mut H) {
            self.0.hash(h);
        }
    }

    let mut s = LinkedHashSet::new();
    assert_eq!(s.replace(Foo("a", 1)), None);
    assert_eq!(s.len(), 1);
    assert_eq!(s.replace(Foo("a", 2)), Some(Foo("a", 1)));
    assert_eq!(s.len(), 1);

    let mut it = s.iter();
    assert_eq!(it.next(), Some(&Foo("a", 2)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_extend_ref() {
    let mut a = LinkedHashSet::new();
    a.insert(1);

    a.extend(&[2, 3, 4]);

    assert_eq!(a.len(), 4);
    assert!(a.contains(&1));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(a.contains(&4));

    let mut b = LinkedHashSet::new();
    b.insert(5);
    b.insert(6);

    a.extend(&b);

    assert_eq!(a.len(), 6);
    assert!(a.contains(&1));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(a.contains(&4));
    assert!(a.contains(&5));
    assert!(a.contains(&6));
}

#[test]
fn test_retain() {
    let xs = [1, 2, 3, 4, 5, 6];
    let mut set: LinkedHashSet<i32> = xs.iter().cloned().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 3);
    assert!(set.contains(&2));
    assert!(set.contains(&4));
    assert!(set.contains(&6));
}

#[test]
fn test_retain_with_order() {
    let xs = [1, 2, 3, 4, 5, 6];
    let mut set: LinkedHashSet<i32> = xs.iter().cloned().collect();
    let mut vec = Vec::new();
    set.retain_with_order(|&k| {
        if k % 2 == 0 {
            true
        } else {
            vec.push(k);
            false
        }
    });
    assert_eq!(vec![1, 3, 5], vec);
}

#[test]
fn insert_order() {
    let mut set = LinkedHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    assert_eq!(
        set.clone().into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
}

#[test]
fn front_back() {
    let mut set = LinkedHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    assert_eq!(set.front(), Some(&1));
    assert_eq!(set.back(), Some(&4));
    assert_eq!(set.pop_back(), Some(4));
    assert_eq!(set.back(), Some(&3));
    assert_eq!(set.pop_front(), Some(1));
    assert_eq!(set.front(), Some(&2));
}

#[test]
fn double_ended_iter() {
    let mut set = LinkedHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next_back(), Some(&4));
    assert_eq!(iter.next_back(), Some(&3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    drop(iter);

    let mut iter = set.drain();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    drop(iter);

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    let mut iter = set.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(4));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn to_back_front_order() {
    let mut set = LinkedHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);

    assert_eq!(set.back().copied(), Some(4));
    assert_eq!(set.front().copied(), Some(1));
    set.to_back(&2);
    assert_eq!(set.back().copied(), Some(2));
    set.to_front(&3);
    assert_eq!(set.front().copied(), Some(3));
}

#[test]
fn test_order_equality() {
    let xs = [1, 2, 3, 4, 5, 6];
    let mut set1: LinkedHashSet<i32> = xs.iter().copied().collect();
    let mut set2: LinkedHashSet<i32> = xs.iter().copied().collect();

    assert_eq!(set1, set2);

    set1.to_front(&4);
    assert_ne!(set1, set2);

    set2.to_front(&4);
    assert_eq!(set1, set2);
}
