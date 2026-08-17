Const `TypeId` and non-'static `TypeId`
=======================================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/typeid-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/typeid)
[<img alt="crates.io" src="https://img.shields.io/crates/v/typeid.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/typeid)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-typeid-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/typeid)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/typeid/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/typeid/actions?query=branch%3Amaster)

[`ConstTypeId`]: https://docs.rs/typeid/1/typeid/struct.ConstTypeId.html
[`typeid::of`]: https://docs.rs/typeid/1/typeid/fn.of.html

### Const `TypeId`

This crate provides [`ConstTypeId`], which is like [`core::any::TypeId`] but is
constructible in const in stable Rust. (The standard library's TypeId's is
nightly-only to construct in const; the tracking issue for this is
[rust#77125].)

[`core::any::TypeId`]: https://doc.rust-lang.org/core/any/struct.TypeId.html
[rust#77125]: https://github.com/rust-lang/rust/issues/77125

Being able to construct `ConstTypeId` in const makes it suitable for use cases
that rely on static promotion:

```rust
use std::fmt::{self, Debug, Display};
use std::ptr;
use typeid::ConstTypeId;

pub struct ObjectVTable {
    type_id: ConstTypeId,
    drop_in_place: unsafe fn(*mut ()),
    display: unsafe fn(*const (), &mut fmt::Formatter) -> fmt::Result,
    debug: unsafe fn(*const (), &mut fmt::Formatter) -> fmt::Result,
}

impl ObjectVTable {
    pub const fn new<T: Display + Debug>() -> &'static Self {
        &ObjectVTable {
            type_id: const { ConstTypeId::of::<T>() },
            drop_in_place: |ptr| unsafe { ptr::drop_in_place(ptr.cast::<T>()) },
            display: |ptr, f| unsafe { Display::fmt(&*ptr.cast::<T>(), f) },
            debug: |ptr, f| unsafe { Debug::fmt(&*ptr.cast::<T>(), f) },
        }
    }
}
```

and in associated constants:

```rust
use typeid::ConstTypeId;

pub trait GetTypeId {
    const TYPEID: ConstTypeId;
}

impl<T: 'static> GetTypeId for T {
    const TYPEID: ConstTypeId = ConstTypeId::of::<Self>();
}
```

<br>

### Non-'static `TypeId`

This crate provides [`typeid::of`], which takes an arbitrary non-'static type
`T` and produces the `TypeId` for the type obtained by replacing all lifetimes
in `T` by `'static`, other than higher-rank lifetimes found in trait objects.

For example if `T` is `&'b dyn for<'a> Trait<'a, 'c>`, then `typeid::of::<T>()`
produces the TypeId of `&'static dyn for<'a> Trait<'a, 'static>`.

It should be obvious that unlike with the standard library's TypeId,
`typeid::of::<A>() == typeid::of::<B>()` does **not** mean that `A` and `B` are
the same type. However, there is a common special case where this behavior is
exactly what is needed. If:

- `A` is an arbitrary non-'static type parameter, _and_
- `B` is 'static, _and_
- all types with the same id as `B` are also 'static

then `typeid::of::<A>() == typeid::of::<B>()` guarantees that `A` and `B` are
the same type.

```rust
use core::any::TypeId;
use core::slice;

pub fn example<T>(slice: &[T]) {
    // T is arbitrary and non-'static.

    if typeid::of::<T>() == TypeId::of::<u8>() {
        // T is definitely u8
        let bytes = unsafe { slice::from_raw_parts(slice.as_ptr().cast(), slice.len()) };
        process_bytes(bytes);
    } else {
        for t in slice {
            process(t);
        }
    }
}

fn process<T>(_: &T) {/* ... */}
fn process_bytes(_: &[u8]) {/* ... */}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
