#![cfg(feature = "NSSet")]
#![cfg(feature = "NSString")]
use alloc::vec;

use crate::Foundation::{self, ns_string, NSMutableSet, NSString};

#[test]
fn test_insert() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    assert!(set.insert_id(NSString::from_str("one")));
    assert!(!set.insert_id(NSString::from_str("one")));
    assert!(set.insert_id(NSString::from_str("two")));
}

#[test]
fn test_remove() {
    let strs = ["one", "two", "three"].map(NSString::from_str);
    let mut set = NSMutableSet::from_id_slice(&strs);

    assert!(set.remove(ns_string!("one")));
    assert!(!set.remove(ns_string!("one")));
}

#[test]
fn test_clear() {
    let strs = ["one", "two", "three"].map(NSString::from_str);
    let mut set = NSMutableSet::from_id_slice(&strs);
    assert_eq!(set.len(), 3);

    set.removeAllObjects();
    assert!(set.is_empty());
}

#[test]
#[cfg(feature = "NSString")]
fn test_into_vec() {
    let strs = vec![
        Foundation::NSMutableString::from_str("one"),
        Foundation::NSMutableString::from_str("two"),
        Foundation::NSMutableString::from_str("three"),
    ];
    let set = NSMutableSet::from_vec(strs);

    let mut vec = NSMutableSet::into_vec(set);
    for str in vec.iter_mut() {
        str.appendString(ns_string!(" times zero is zero"));
    }

    assert_eq!(vec.len(), 3);
    let suffix = ns_string!("zero");
    assert!(vec.iter().all(|str| str.hasSuffix(suffix)));
}

#[test]
fn test_extend() {
    let mut set = NSMutableSet::new();
    assert!(set.is_empty());

    set.extend(["one", "two", "three"].map(NSString::from_str));
    assert_eq!(set.len(), 3);
}

#[test]
#[cfg(feature = "NSObject")]
fn test_mutable_copy() {
    use Foundation::{NSMutableCopying, NSSet};

    let set1 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
    let mut set2 = set1.mutableCopy();
    set2.insert_id(NSString::from_str("four"));

    assert!(set1.is_subset(&set2));
    assert_ne!(set1.mutableCopy(), set2);
}
