# Interior mutability

Everything in Objective-C is an object, and objects can be freely shared between each other. This is done especially in GUI code where e.g. the application keeps a reference to all the currently open windows, while views can also access their parent window.

In Rust, we generally dislike this, since it leads to designs that are more interlinked, rarely thread safe, harder for the compiler to optimize, and harder to understand, so instead mutability is restricted to `&mut` references, which ensures that the references is not [aliased].

The original design of `objc2` did actually use `&mut` references in certain circumstances, see [#563] and [#265], but we have since removed that functionality, since it is just too error prone and restrictive.

So what are we to do? Well, luckily, Rust lets us "opt-out" of the usual rules using [`UnsafeCell`], which tells the compiler that a type is actually mutable within a `&` reference, so that's what we do; every object is `UnsafeCell` internally.

[aliased]: https://doc.rust-lang.org/nomicon/aliasing.html
[#563]: https://github.com/madsmtm/objc2/issues/563
[#265]: https://github.com/madsmtm/objc2/issues/265
[`UnsafeCell`]: std::cell::UnsafeCell


## Interior mutability in practice

So what does this mean in practice?

Well, it means that when you define classes yourself, **you will have to use interior mutability helpers like [`Cell`] and [`RefCell`] before you can modify properties**. This is a bit annoying, and can be a bit confusing if you're not used to it.

Let's take an example: We define a class that contains an [`i32`] and a [`Vec`]. In usual Rust, both would just be in a `struct`, and you wouldn't have to do anything extra to access them, but since the class is only mutable from shared references like `&`, we have to wrap the integer in `Cell` and the vector in `RefCell` before we can safely mutate them:

```rust
use std::cell::{Cell, RefCell};
use objc2::{define_class, DefinedClass};
use objc2::runtime::NSObject;

// Usually, you would just do:

struct MyStruct {
    my_int: i32,
    my_vec: Vec<i32>,
}

impl MyStruct {
    fn add_next(&mut self) {
        self.my_int += 1;
        self.my_vec.push(self.my_int);
    }
}

// But when interfacing with Objective-C, you have to do:

struct Ivars {
    // `Copy` types that we want to mutate have to be wrapped in Cell
    my_int: Cell<i32>,
    // non-`Copy` types that we want to mutate have to be wrapped in RefCell
    my_vec: RefCell<Vec<i32>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = Ivars]
    struct MyClass;

    impl MyClass {
        #[unsafe(method(myMethod))]
        fn add_next(&self) {
            let ivars = self.ivars();

            // self.my_int += 1;
            ivars.my_int.set(ivars.my_int.get() + 1);

            // self.my_vec.push(self.my_int);
            ivars.my_vec.borrow_mut().push(ivars.my_int.get());
        }
    }
);
```

Note how this makes your class no longer [`Sync`], since there could be a race condition if we tried to call `add_next` from two threads at the same time. If you wanted to make it thread safe, you would have to use [`AtomicI32`], with all of the difficulties inherent in that.

[`Cell`]: std::cell::Cell
[`RefCell`]: std::cell::RefCell
[`Vec`]: std::vec::Vec
[`Sync`]: std::marker::Sync
[`AtomicI32`]: std::sync::atomic::AtomicI32


## Thread safety

This hints at a general observation: **very few Objective-C classes are thread safe**, because they allow mutation while aliased elsewhere. So that's the other major place where interior mutability is likely to affect you.

Even something as basic as `NSString` is not thread safe, because it may have come from a `NSMutableString` (see [the notes on deref][deref]), and as such could be mutated from a separate thread without the type-system knowing that.

See also the [`MainThreadMarker`] and [`MainThreadOnly`] for a similar, even harder restriction, where some types are only usable on the main thread because they access global statics.

[deref]: about_generated::deref
[`MainThreadMarker`]: crate::MainThreadMarker
[`MainThreadOnly`]: crate::MainThreadOnly
