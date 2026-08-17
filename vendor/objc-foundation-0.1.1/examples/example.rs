extern crate objc_foundation;

use objc_foundation::{NSArray, NSDictionary, NSObject, NSString,
    INSArray, INSCopying, INSDictionary, INSObject, INSString};

fn main() {
    // Create and compare NSObjects
    let obj = NSObject::new();
    println!("{:?} == {:?}? {:?}", obj, obj, obj == obj);

    let obj2 = NSObject::new();
    println!("{:?} == {:?}? {:?}", obj, obj2, obj == obj2);

    // Create an NSArray from a Vec
    let objs = vec![obj, obj2];
    let array = NSArray::from_vec(objs);
    for obj in array.object_enumerator() {
        println!("{:?}", obj);
    }
    println!("{}", array.count());

    // Turn the NSArray back into a Vec
    let mut objs = NSArray::into_vec(array);
    let obj = objs.pop().unwrap();

    // Create an NSString from a str slice
    let string = NSString::from_str("Hello, world!");
    println!("{}", string.as_str());
    let string2 = string.copy();
    println!("{}", string2.as_str());

    // Create a dictionary mapping strings to objects
    let keys = &[&*string];
    let vals = vec![obj];
    let dict = NSDictionary::from_keys_and_objects(keys, vals);
    println!("{:?}", dict.object_for(&string));
    println!("{}", dict.count());
}
