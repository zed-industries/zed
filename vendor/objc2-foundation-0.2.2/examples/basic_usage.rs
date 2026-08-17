use objc2::rc::autoreleasepool;
use objc2_foundation::{ns_string, NSArray, NSDictionary, NSObject};

fn main() {
    // Create and compare NSObjects
    let obj = NSObject::new();
    #[allow(clippy::eq_op)]
    {
        println!("{obj:?} == {obj:?}? {:?}", obj == obj);
    }

    let obj2 = NSObject::new();
    println!("{obj:?} == {obj2:?}? {:?}", obj == obj2);

    // Create an NSArray from a Vec
    let objs = vec![obj, obj2];
    let array = NSArray::from_vec(objs);
    for obj in array.iter() {
        println!("{obj:?}");
    }
    println!("{}", array.len());

    // Turn the NSArray back into a Vec
    let mut objs = array.to_vec_retained();
    let obj = objs.pop().unwrap();

    // Create a static NSString
    let string = ns_string!("Hello, world!");
    // Use an autoreleasepool to get the `str` contents of the NSString
    autoreleasepool(|pool| {
        println!("{}", string.as_str(pool));
    });
    // Or use the `Display` implementation
    let _s = string.to_string(); // Using ToString
    println!("{string}"); // Or Display directly

    // Create a dictionary mapping strings to objects
    let keys = &[string];
    let objects = &[obj];
    let dict = NSDictionary::from_id_slice(keys, objects);
    println!("{:?}", dict.get(string));
    println!("{}", dict.len());
}
