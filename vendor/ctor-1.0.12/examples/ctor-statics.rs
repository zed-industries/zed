//! Demonstrate various forms of `#[ctor]` statics.
use std::mem::MaybeUninit;

use ctor::ctor;

#[derive(Debug)]
#[allow(dead_code)]
struct MyStatic(String);

impl MyStatic {
    fn new(s: impl AsRef<str>) -> Self {
        Self(s.as_ref().to_string())
    }
}

#[ctor(unsafe)]
static STATIC_CTOR: MyStatic = MyStatic::new("foo");

#[ctor(unsafe)]
static STATIC_CTOR_REF: &'static MyStatic = &MyStatic::new("foo");

/// This is not a recommended pattern - just demonstrating how to return static
/// refs.
#[ctor(unsafe)]
#[allow(static_mut_refs)]
static STATIC_CTOR_REF_GLOBAL: &'static MyStatic = {
    unsafe {
        static mut GLOBAL: MaybeUninit<MyStatic> = MaybeUninit::uninit();

        GLOBAL.write(MyStatic::new("foo"));
        GLOBAL.assume_init_ref()
    }
};

/// The above is a similar form to this:
pub static FOO: &'static (dyn ::core::fmt::Debug + Sync) = &"foo";

fn main() {
    println!("STATIC_CTOR: {:?}", STATIC_CTOR);
    println!("STATIC_CTOR_REF: {:?}", STATIC_CTOR_REF);
    println!("STATIC_CTOR_REF_GLOBAL: {:?}", STATIC_CTOR_REF_GLOBAL);
}
