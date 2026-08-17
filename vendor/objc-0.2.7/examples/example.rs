#[macro_use]
extern crate objc;

use objc::Encode;
use objc::rc::StrongPtr;
use objc::runtime::{Class, Object};

fn main() {
    // Get a class
    let cls = class!(NSObject);
    println!("NSObject size: {}", cls.instance_size());

    // Inspect its ivars
    println!("NSObject ivars:");
    for ivar in cls.instance_variables().iter() {
        println!("{}", ivar.name());
    }

    // Allocate an instance
    let obj = unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, init];
        StrongPtr::new(obj)
    };
    println!("NSObject address: {:p}", obj);

    // Access an ivar of the object
    let isa: *const Class = unsafe {
        *(**obj).get_ivar("isa")
    };
    println!("NSObject isa: {:?}", isa);

    // Inspect a method of the class
    let hash_sel = sel!(hash);
    let hash_method = cls.instance_method(hash_sel).unwrap();
    let hash_return = hash_method.return_type();
    println!("-[NSObject hash] return type: {:?}", hash_return);
    assert!(hash_return == usize::encode());

    // Invoke a method on the object
    let hash: usize = unsafe {
        msg_send![*obj, hash]
    };
    println!("NSObject hash: {}", hash);
}
