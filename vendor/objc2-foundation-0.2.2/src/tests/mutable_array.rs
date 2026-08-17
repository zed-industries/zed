#![cfg(feature = "NSArray")]
use alloc::vec;

use objc2::rc::{autoreleasepool, Allocated};
use objc2::{msg_send, ClassType};

#[cfg(feature = "NSValue")]
use crate::Foundation::NSNumber;
use crate::Foundation::{self, NSMutableArray, NSObject};

#[test]
#[cfg(feature = "NSValue")]
fn test_creation() {
    let _ = <NSMutableArray<NSNumber>>::from_vec(vec![]);
    let _ = NSMutableArray::from_vec(vec![NSNumber::new_u8(4), NSNumber::new_u8(2)]);

    let _ = <NSMutableArray<NSNumber>>::from_id_slice(&[]);
    let _ = NSMutableArray::from_id_slice(&[NSNumber::new_u8(4), NSNumber::new_u8(2)]);

    let _ = <NSMutableArray<NSNumber>>::from_slice(&[]);
    let _ = NSMutableArray::from_slice(&[&*NSNumber::new_u8(4), &*NSNumber::new_u8(2)]);
}

#[test]
#[cfg(feature = "NSString")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_containing_mutable_objects() {
    use Foundation::NSMutableString;

    let mut array = NSMutableArray::from_vec(vec![NSMutableString::new()]);
    let _: &mut NSMutableString = &mut array[0];
    let _: &mut NSMutableString = array.get_mut(0).unwrap();
    let _: &mut NSMutableString = array.first_mut().unwrap();
    let _: &mut NSMutableString = array.last_mut().unwrap();
}

#[test]
#[cfg(feature = "NSString")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_allowed_mutation_while_iterating() {
    use Foundation::{NSMutableString, NSString};

    let mut array = NSMutableArray::from_vec(vec![NSMutableString::new(), NSMutableString::new()]);
    let to_add = NSString::from_str("test");

    for s in &mut array {
        s.appendString(&to_add);
    }

    assert_eq!(&array[0], &*to_add);
    assert_eq!(&array[1], &*to_add);
}

#[test]
#[should_panic = "mutation detected during enumeration"]
#[cfg_attr(
    not(debug_assertions),
    ignore = "enumeration mutation only detected with debug assertions on"
)]
#[cfg_attr(
    all(debug_assertions, feature = "gnustep-1-7"),
    ignore = "thread safety issues regarding initialization"
)]
fn test_iter_mutation_detection() {
    let array = NSMutableArray::from_id_slice(&[NSObject::new(), NSObject::new()]);

    for item in &array {
        let item: &NSObject = item;
        let _: () = unsafe { msg_send![&array, removeObject: item] };
    }
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_threaded() {
    std::thread::scope(|s| {
        s.spawn(|| {
            let _ = NSMutableArray::from_vec(vec![NSObject::new(), NSObject::new()]);
        });

        s.spawn(|| {
            let array = <NSMutableArray<NSObject>>::alloc();
            let ptr = Allocated::as_ptr(&array);
            assert!(!ptr.is_null());
        });
    });
}

#[test]
#[cfg(feature = "NSString")]
#[cfg(feature = "NSRange")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_into_vec() {
    let array = NSMutableArray::from_id_slice(&[Foundation::NSString::new()]);

    let vec = NSMutableArray::into_vec(array);
    assert_eq!(vec.len(), 1);
}

#[test]
#[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
fn test_sort() {
    use Foundation::NSString;

    let strings = vec![NSString::from_str("hello"), NSString::from_str("hi")];
    let mut strings = NSMutableArray::from_vec(strings);

    autoreleasepool(|pool| {
        strings.sort_by(|s1, s2| s1.as_str(pool).len().cmp(&s2.as_str(pool).len()));
        assert_eq!(strings[0].as_str(pool), "hi");
        assert_eq!(strings[1].as_str(pool), "hello");
    });
}
