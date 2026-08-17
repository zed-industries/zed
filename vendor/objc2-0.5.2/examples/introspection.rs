use objc2::runtime::{AnyClass, NSObject};
use objc2::{sel, ClassType, Encode};

fn main() {
    // Get the class representing `NSObject`
    let cls = NSObject::class();

    // Inspect various properties of the class
    println!("NSObject superclass: {:?}", cls.superclass());
    println!("NSObject size: {}", cls.instance_size());
    println!(
        "-[NSObject alloc] would work: {}",
        cls.responds_to(sel!(alloc))
    );
    println!(
        "+[NSObject alloc] would work: {}",
        cls.metaclass().responds_to(sel!(alloc))
    );

    // Inspect an instance variable on the class
    //
    // Note: You should not rely on the `isa` ivar being available,
    // this is only for demonstration.
    let ivar = cls
        .instance_variable("isa")
        .expect("No ivar with name 'isa' found on NSObject");
    println!(
        "Instance variable {} has type encoding {:?}",
        ivar.name(),
        ivar.type_encoding()
    );
    assert!(<*const AnyClass>::ENCODING.equivalent_to_str(ivar.type_encoding()));

    // Inspect a method of the class
    let method = cls.instance_method(sel!(hash)).unwrap();
    println!(
        "-[NSObject hash] takes {} parameters",
        method.arguments_count()
    );
    let hash_return = method.return_type();
    println!("-[NSObject hash] return type: {hash_return:?}");
    assert!(usize::ENCODING.equivalent_to_str(&hash_return));

    // Create an instance
    let obj = NSObject::new();

    println!("NSObject address: {obj:p}");

    // Read an ivar on the object
    let isa: *const AnyClass = unsafe { *ivar.load(&obj) };
    println!("NSObject isa: {isa:?}");
}
