#![cfg(feature = "NSDictionary")]
#![cfg(feature = "NSValue")]
#![cfg(feature = "NSObject")]
use objc2::rc::Retained;

use crate::{NSMutableDictionary, NSNumber, NSObject};

fn sample_dict() -> Retained<NSMutableDictionary<NSNumber, NSObject>> {
    NSMutableDictionary::from_retained_objects(
        &[
            &*NSNumber::new_i32(1),
            &*NSNumber::new_i32(2),
            &*NSNumber::new_i32(3),
        ],
        &[NSObject::new(), NSObject::new(), NSObject::new()],
    )
}

fn sample_dict_mut() -> Retained<NSMutableDictionary<NSNumber, NSMutableDictionary>> {
    NSMutableDictionary::from_retained_objects(
        &[
            &*NSNumber::new_i32(1),
            &*NSNumber::new_i32(2),
            &*NSNumber::new_i32(3),
        ],
        &[
            NSMutableDictionary::new(),
            NSMutableDictionary::new(),
            NSMutableDictionary::new(),
        ],
    )
}

#[test]
#[cfg(feature = "NSString")]
fn dict_from_mutable() {
    use crate::{NSMutableString, NSString};

    let _: Retained<NSMutableDictionary<NSString, NSString>> = NSMutableDictionary::from_slices(
        &[&*NSMutableString::from_str("a")],
        &[&**NSMutableString::from_str("b")],
    );
}

#[test]
fn test_new() {
    let dict = NSMutableDictionary::<NSObject, NSObject>::new();
    assert!(dict.is_empty());
}

#[test]
fn test_get() {
    let dict = sample_dict_mut();

    assert!(dict.objectForKey(&NSNumber::new_i32(1)).is_some());
    assert!(dict.objectForKey(&NSNumber::new_i32(2)).is_some());
    assert!(dict.objectForKey(&NSNumber::new_i32(4)).is_none());
}

#[test]
fn test_to_vecs() {
    let dict = sample_dict_mut();
    let (keys, objects) = dict.to_vecs();
    assert_eq!(keys.len(), 3);
    assert_eq!(objects.len(), 3);
}

#[test]
fn test_insert() {
    let dict = <NSMutableDictionary<NSNumber, NSObject>>::new();
    dict.insert(&*NSNumber::new_i32(1), &NSObject::new());
    dict.insert(&*NSNumber::new_i32(2), &NSObject::new());
    dict.insert(&*NSNumber::new_i32(3), &NSObject::new());
    dict.insert(&*NSNumber::new_i32(1), &NSObject::new());
    assert_eq!(dict.len(), 3);
}

#[test]
fn test_remove() {
    let dict = sample_dict();
    assert_eq!(dict.len(), 3);
    dict.removeObjectForKey(&NSNumber::new_i32(1));
    dict.removeObjectForKey(&NSNumber::new_i32(2));
    dict.removeObjectForKey(&NSNumber::new_i32(1));
    dict.removeObjectForKey(&NSNumber::new_i32(4));
    assert_eq!(dict.len(), 1);
}

#[test]
fn test_clear() {
    let dict = sample_dict();
    assert_eq!(dict.len(), 3);

    dict.removeAllObjects();
    assert!(dict.is_empty());
}

#[test]
#[cfg(feature = "NSArray")]
fn test_to_array() {
    let dict = sample_dict();
    let array = dict.allValues();
    assert_eq!(array.len(), 3);
}

#[test]
#[should_panic = "mutation detected during enumeration"]
fn test_iter_mutation_detection() {
    let dict = sample_dict();

    let mut iter = dict.keys();
    let _ = iter.next();

    dict.insert(&*NSNumber::new_usize(1), &NSObject::new());

    let _ = iter.next();
}
