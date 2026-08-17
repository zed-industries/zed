//! Constructors beyond `ctor-basic` / `ctor-example`: anonymous `#[ctor]`, priorities, nested modules,
//! inherent `#[ctor]` methods on generic `impl`s, and ctors under a module-level `const`.
use ctor::ctor;
use libc_print::*;
use std::collections::HashMap;

/// A global hashmap (allocation before main).
#[ctor(unsafe)]
pub static SHOWCASE_GLOBAL: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    _ = m.insert(0, "foo");
    _ = m.insert(1, "bar");
    _ = m.insert(2, "baz");
    libc_println!("SHOWCASE_GLOBAL");
    m
};

/// Anonymous ctor (#1 of 2 with the same Rust name).
#[ctor(unsafe, anonymous)]
fn anonymous_ctor() {
    libc_println!("ctor_anonymous (#1)");
    let _f = anonymous_ctor;
}

/// Anonymous ctor (#2 of 2 with the same Rust name).
#[ctor(unsafe, anonymous)]
fn anonymous_ctor() {
    libc_println!("ctor_anonymous (#2)");
}

const _: () = {
    /// Anonymous ctor inside a `const` scope (#3).
    #[ctor(unsafe)]
    fn anonymous_ctor() {
        libc_println!("ctor_anonymous (#3)");
        let _f = anonymous_ctor;
    }
};

/// Regular `#[ctor]` function.
#[ctor(unsafe)]
fn ctor() {
    libc_println!("ctor");
}

/// Priority 1 `#[ctor]` function.
#[ctor(unsafe, priority = 1)]
fn ctor_priority_one() {
    libc_println!("ctor_priority_one");
}

/// A nested module with a `static` item.
pub mod module {
    use ctor::*;
    use libc_print::*;

    /// A `static` item in a nested module.
    #[ctor(unsafe)]
    pub(crate) static STATIC_CTOR: u8 = {
        libc_println!("module::STATIC_CTOR");
        42
    };
}

/// A generic `impl` with a `#[ctor]` method.
#[derive(Default)]
struct Foo<T> {
    _t: ::std::marker::PhantomData<T>,
}

impl<T: Default> Foo<T> {
    /// Drop the default value of the generic type.
    fn generic(self) {
        drop(T::default());
    }

    /// A `#[ctor]` method in a generic `impl`.
    #[ctor(unsafe)]
    fn ctor() {
        libc_println!("Foo::ctor");
    }
}

fn main() {
    libc_println!("main!");
    libc_println!("SHOWCASE_GLOBAL = {:?}", *SHOWCASE_GLOBAL);
    libc_println!("module::STATIC_CTOR = {:?}", *module::STATIC_CTOR);

    // Only one ctor call runs across monomorphizations; generics are unavailable in ctor bodies.
    Foo::<u32>::default().generic();
    Foo::<u64>::default().generic();
}
