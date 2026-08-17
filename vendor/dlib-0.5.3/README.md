[![crates.io](https://img.shields.io/crates/v/dlib.svg)](https://crates.io/crates/dlib)
[![docs.rs](https://docs.rs/dlib/badge.svg)](https://docs.rs/dlib)

# dlib

dlib is a small crate providing macros to make easy the use of external system libraries that
can or cannot be optionally loaded at runtime, depending on whether a certain feature is enabled.

### Usage

dlib defines the `external_library!` macro, which can be invoked in this way:

```rust
external_library!(feature="dlopen-foo", Foo, "foo",
    statics:
        me: c_int,
        you: c_float,
    functions:
        fn foo() -> c_int,
        fn bar(c_int, c_float) -> (),
        fn baz(*const c_int) -> c_int,
    varargs:
        fn blah(c_int, c_int ...) -> *const c_void,
        fn bleh(c_int ...) -> (),
);
```

As you can see, it is required to separate static values from functions and from function
having variadic arguments. Each of these 3 categories is optional, but the ones used must appear
in this order. Return types of the functions must all be explicit (hence `-> ()` for void functions).

If the feature named by the `feature` argument (in this example, `dlopen-foo`) is absent on your crate,
this macro will expand to an extern block defining each of the items, using the third argument
of the macro as a link name:

```rust
#[link(name = "foo")]
extern "C" {
    pub static me: c_int;
    pub static you: c_float;
    pub fn foo() -> c_int;
    pub fn bar(_: c_int, _: c_float) -> ();
    pub fn baz(_: *const c_int) -> c_int;
    pub fn blah(_: c_int, _: c_int, ...) -> *const c_void;
    pub fn bleh(_: c_int, ...) -> ();
}

```

If the feature named by the `feature` argument is present on your crate, it will expand to a
`struct` named by the second argument of the macro, with one field for each of the symbols defined;
and a method `open`, which tries to load the library from the name or path given as an argument.

```rust
pub struct Foo {
    pub me: &'static c_int,
    pub you: &'static c_float,
    pub foo: unsafe extern "C" fn() -> c_int,
    pub bar: unsafe extern "C" fn(c_int, c_float) -> (),
    pub baz: unsafe extern "C" fn(*const c_int) -> c_int,
    pub blah: unsafe extern "C" fn(c_int, c_int, ...) -> *const c_void,
    pub bleh: unsafe extern "C" fn(c_int, ...) -> (),
}


impl Foo {
    pub unsafe fn open(name: &str) -> Result<Foo, DlError> { /* ... */ }
}
```

This method returns `Ok(..)` if the loading was successful. It contains an instance of the defined struct
with all of its fields pointing to the appropriate symbol.

If the library specified by `name` could not be openened, it returns `Err(DlError::CantOpen(e))`, with
`e` the error reported by `libloading` (see [LibLoadingError]);

It will also fail on the first missing symbol, with `Err(DlError::MissingSymbol(symb))` where `symb`
is a `&str` containing the missing symbol name.

Note that this method is unsafe, as loading (and unloading on drop) an external C library can run arbitrary
code. As such, you need to ensure that the specific library you want to load is safe to load in the context
you want to load it.

### Remaining generic in your crate

If you want your crate to remain generic over dlopen vs. linking, simply add a feature to your `Cargo.toml`:

```toml
[dependencies]
dlib = "0.5"

[features]
dlopen-foo = []
```

Then give the name of that feature as the `feature` argument to dlib's macros:

```rust
external_library!(feature="dlopen-foo", Foo, "foo",
    functions:
        fn foo() -> c_int,
);
```

`dlib` provides helper macros to dispatch the access to foreign symbols:

```rust
ffi_dispatch!(feature="dlopen-foo", Foo, function, arg1, arg2);
ffi_dispatch_static!(feature="dlopen-foo", Foo, my_static_var);
```

These will expand to the appropriate value or function call depending on the presence or absence of the
`dlopen-foo` feature on your crate.

You must still ensure that the functions/statics or the wrapper struct `Foo` are in scope. For example,
you could use the [`lazy_static`](https://crates.io/crates/lazy_static) crate to do the initialization,
and store the wrapper struct in a static variable that you import wherever needed:

```rust
#[cfg(feature = "dlopen-foo")]
lazy_static::lazy_static! {
    pub static ref FOO_STATIC: Foo =
        Foo::open("libfoo.so").ok().expect("could not find libfoo");
}
```

Then, it can become as simple as putting this on top of all modules using the FFI:

```rust
#[cfg(feature = "dlopen-foo")]
use ffi::FOO_STATIC;
#[cfg(not(feature = "dlopen-foo"))]
use ffi::*;
```

License: MIT
