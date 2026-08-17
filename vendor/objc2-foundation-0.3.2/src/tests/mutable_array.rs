#![cfg(feature = "NSArray")]
use objc2::rc::Allocated;
use objc2::runtime::AnyObject;
use objc2::AnyThread;

use crate::{NSMutableArray, NSObject};

#[test]
#[cfg(feature = "NSValue")]
fn test_creation() {
    use crate::NSNumber;

    let _ = <NSMutableArray<NSNumber>>::from_retained_slice(&[]);
    let _ = NSMutableArray::from_retained_slice(&[NSNumber::new_u8(4), NSNumber::new_u8(2)]);

    let _ = <NSMutableArray<NSNumber>>::from_slice(&[]);
    let _ = NSMutableArray::from_slice(&[&*NSNumber::new_u8(4), &*NSNumber::new_u8(2)]);
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_containing_another_array() {
    let array = NSMutableArray::from_retained_slice(&[NSMutableArray::<AnyObject>::new()]);
    let _ = array.objectAtIndex(0);
    let _ = array.firstObject().unwrap();
    let _ = array.lastObject().unwrap();
}

#[test]
#[cfg(feature = "NSString")]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_allowed_mutation_while_iterating() {
    use crate::{NSMutableString, NSString};

    let array =
        NSMutableArray::from_retained_slice(&[NSMutableString::new(), NSMutableString::new()]);
    let to_add = NSString::from_str("test");

    for s in &array {
        s.appendString(&to_add);
    }

    assert_eq!(array.objectAtIndex(0), to_add);
    assert_eq!(array.objectAtIndex(1), to_add);
}

#[test]
#[should_panic = "mutation detected during enumeration"]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_iter_mutation_detection() {
    let array = NSMutableArray::from_retained_slice(&[NSObject::new(), NSObject::new()]);

    for item in &array {
        array.removeObject(&item);
    }
}

#[test]
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
#[cfg(feature = "std")]
fn test_threaded() {
    std::thread::scope(|s| {
        s.spawn(|| {
            let _ = NSMutableArray::from_retained_slice(&[NSObject::new(), NSObject::new()]);
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
#[cfg_attr(
    feature = "gnustep-1-7",
    ignore = "thread safety issues regarding initialization"
)]
fn test_to_vec() {
    let array = NSMutableArray::from_retained_slice(&[crate::NSString::new()]);

    let vec = array.to_vec();
    assert_eq!(vec.len(), 1);
}

#[test]
#[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
fn test_sort() {
    use crate::ns_string;
    use alloc::string::ToString;

    let strings = [ns_string!("hello"), ns_string!("hi")];
    let strings = NSMutableArray::from_slice(&strings);

    strings.sort_by(|s1, s2| s1.len().cmp(&s2.len()));
    assert_eq!(strings.objectAtIndex(0).to_string(), "hi");
    assert_eq!(strings.objectAtIndex(1).to_string(), "hello");
}

#[test]
#[cfg(feature = "NSValue")]
#[cfg_attr(
    all(target_os = "macos", target_arch = "x86"),
    should_panic = "mutation detected during enumeration"
)]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "x86")),
    should_panic = "`itemsPtr` was NULL, likely due to mutation during iteration"
)]
#[cfg_attr(feature = "gnustep-1-7", ignore = "errors differently on GNUStep")]
fn null_itemsptr() {
    use crate::NSNumber;
    // Found while fuzzing

    let array = NSMutableArray::new();
    let mut iter = array.iter();
    array.addObject(&*NSNumber::new_i32(0));
    array.addObject(&*NSNumber::new_i32(0));
    array.removeObjectAtIndex(0);
    array.addObject(&*NSNumber::new_i32(0));
    let _ = iter.next();
    array.removeAllObjects();
    let _ = iter.next();
}
