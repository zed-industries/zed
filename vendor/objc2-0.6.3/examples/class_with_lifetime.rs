//! Note: We can't use the `define_class!` macro for this, it doesn't support
//! such use-cases (yet). Instead, we'll create it manually.
#![deny(unsafe_op_in_unsafe_fn)]
use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::Once;

use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, NSObject};
use objc2::{msg_send, ClassType, Encoding, Message, RefEncode};

/// The type of the instance variable that we want to store
type Ivar<'a> = &'a Cell<u8>;

/// Struct that represents our custom object.
#[repr(C)]
struct MyObject<'a> {
    /// Required to give MyObject the proper layout
    ///
    /// Beware: `retain`-ing this field directly is dangerous, since it'd make
    /// it possible to extend the lifetime of the object beyond lifetime `'a`.
    inner: AnyObject,
    /// For auto traits and variance
    p: PhantomData<Ivar<'a>>,
}

unsafe impl RefEncode for MyObject<'_> {
    const ENCODING_REF: Encoding = NSObject::ENCODING_REF;
}

unsafe impl Message for MyObject<'_> {}

impl<'a> MyObject<'a> {
    fn class() -> &'static AnyClass {
        // NOTE: Use std::lazy::LazyCell if in MSRV
        static REGISTER_CLASS: Once = Once::new();

        REGISTER_CLASS.call_once(|| {
            let superclass = NSObject::class();
            let mut builder = ClassBuilder::new(c"MyObject", superclass).unwrap();

            builder.add_ivar::<Ivar<'_>>(c"number");

            let _cls = builder.register();
        });
        AnyClass::get(c"MyObject").unwrap()
    }

    fn new(number: &'a mut u8) -> Retained<Self> {
        // SAFETY: The instance variable is initialized below.
        let this: Retained<Self> = unsafe { msg_send![Self::class(), new] };

        // It is generally very hard to use `mut` in Objective-C, so let's use
        // interior mutability instead.
        let number = Cell::from_mut(number);

        let ivar = Self::class().instance_variable(c"number").unwrap();
        // SAFETY: The ivar is added with the same type below, and the
        // lifetime of the reference is properly bound to the class.
        unsafe { ivar.load_ptr::<Ivar<'_>>(&this.inner).write(number) };
        this
    }

    fn get(&self) -> u8 {
        let ivar = Self::class().instance_variable(c"number").unwrap();
        // SAFETY: The ivar is added with the same type below, and is initialized in `new`
        unsafe { ivar.load::<Ivar<'_>>(&self.inner).get() }
    }

    fn set(&self, number: u8) {
        let ivar = Self::class().instance_variable(c"number").unwrap();
        // SAFETY: The ivar is added with the same type below, and is initialized in `new`
        unsafe { ivar.load::<Ivar<'_>>(&self.inner).set(number) };
    }
}

fn main() {
    let mut number = 54;

    let obj = MyObject::new(&mut number);
    assert_eq!(obj.get(), 54);

    // We can now mutate the referenced `number`
    obj.set(7);
    assert_eq!(obj.get(), 7);

    // And we can clone the object, since we use interior mutability.
    let clone = obj.clone();
    clone.set(42);
    assert_eq!(obj.get(), 42);
    drop(clone);

    // It is not possible to convert to `Retained<NSObject>`, since that would
    // loose the lifetime information that `MyObject` stores.
    //
    // let obj = obj.into_super();
    //
    // Neither is it not possible to access `number` any more, since `obj`
    // holds a mutable reference to it.
    //
    // assert_eq!(number, 42);

    drop(obj);
    // And now that we've dropped `obj`, we can access `number` again
    assert_eq!(number, 42);
}
