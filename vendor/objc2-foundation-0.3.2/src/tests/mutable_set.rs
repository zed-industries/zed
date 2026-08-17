#![cfg(feature = "NSSet")]
#![cfg(feature = "NSString")]
use crate::{ns_string, NSMutableSet, NSMutableString};

#[test]
fn test_insert() {
    let set = NSMutableSet::new();
    assert!(set.is_empty());
    set.addObject(ns_string!("one"));
    set.addObject(ns_string!("one"));
    assert_eq!(set.count(), 1);
    set.addObject(ns_string!("two"));
    assert_eq!(set.count(), 2);
}

#[test]
#[cfg(feature = "NSValue")]
fn test_insert_number() {
    use crate::NSNumber;

    let set = NSMutableSet::new();
    set.addObject(&*NSNumber::new_u32(42));
    set.addObject(&*NSNumber::new_u32(42));
    assert_eq!(set.count(), 1);
}

#[test]
fn test_remove() {
    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let set = NSMutableSet::from_slice(&strs);

    assert!(set.containsObject(ns_string!("one")));
    set.removeObject(ns_string!("one"));
    assert!(!set.containsObject(ns_string!("one")));
    assert_eq!(set.count(), 2);

    set.removeObject(ns_string!("one"));
    assert_eq!(set.count(), 2);
}

#[test]
fn test_clear() {
    let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
    let set = NSMutableSet::from_slice(&strs);
    assert_eq!(set.len(), 3);

    set.removeAllObjects();
    assert!(set.is_empty());
}

#[test]
fn test_to_vec() {
    let strs = [
        NSMutableString::from_str("one"),
        NSMutableString::from_str("two"),
        NSMutableString::from_str("three"),
    ];
    let set = NSMutableSet::from_retained_slice(&strs);

    let vec = set.to_vec();
    for str in &vec {
        str.appendString(ns_string!(" times zero is zero"));
    }

    assert_eq!(vec.len(), 3);
    assert!(vec.iter().all(|str| str.hasSuffix(ns_string!("zero"))));
}

#[test]
fn test_extend() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    set.extend([ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    assert_eq!(set.len(), 3);
}

#[test]
#[cfg(feature = "NSObject")]
fn test_mutable_copy() {
    use crate::{NSMutableCopying, NSSet};

    let set1 = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    let set2 = set1.mutableCopy();
    set2.addObject(ns_string!("four"));

    assert!(set1.isSubsetOfSet(&set2));
    assert_ne!(set1.mutableCopy(), set2);
}
