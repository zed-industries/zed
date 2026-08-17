{{#title extern "Rust" — Rust ♡ C++}}
# extern "Rust"

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "Rust" {

    }
}
```

The `extern "Rust"` section of a CXX bridge declares Rust types and signatures
to be made available to C++.

The CXX code generator uses your extern "Rust" section(s) to produce a C++
header file containing the corresponding C++ declarations. The generated header
has the same path as the Rust source file containing the bridge, except with a
`.rs.h` file extension.

A bridge module may contain zero or more extern "Rust" blocks.

## Opaque Rust types

Types defined in Rust that are made available to C++, but only behind an
indirection.

```rust,noplayground
# #[cxx::bridge]
# mod ffi {
    extern "Rust" {
        type MyType;
        type MyOtherType;
        type OneMoreType<'a>;
    }
# }
```

For example in the ***[Tutorial](tutorial.md)*** we saw `MultiBuf` used in this
way. Rust code created the `MultiBuf`, passed a `&mut MultiBuf` to C++, and C++
later passed a `&mut MultiBuf` back across the bridge to Rust.

Another example is the one on the ***[Box\<T\>](binding/box.md)*** page, which
exposes the Rust standard library's `std::fs::File` to C++ as an opaque type in
a similar way but with Box as the indirection rather than &mut.

The types named as opaque types (`MyType` etc) refer to types in the `super`
module, the parent module of the CXX bridge. You can think of an opaque type `T`
as being like a re-export `use super::T` made available to C++ via the generated
header.

Opaque types are currently required to be [`Sized`] and [`Unpin`]. In
particular, a trait object `dyn MyTrait` or slice `[T]` may not be used for an
opaque Rust type. These restrictions may be lifted in the future.

[`Sized`]: https://doc.rust-lang.org/std/marker/trait.Sized.html
[`Unpin`]: https://doc.rust-lang.org/std/marker/trait.Unpin.html

For now, types used as extern Rust types are required to be defined by the same
crate that contains the bridge using them. This restriction may be lifted in the
future.

The bridge's parent module will contain the appropriate imports or definitions
for these types.

```rust,noplayground
use path::to::MyType;

pub struct MyOtherType {
    ...
}
#
# #[cxx::bridge]
# mod ffi {
#     extern "Rust" {
#         type MyType;
#         type MyOtherType;
#     }
# }
```

## Functions

Rust functions made callable to C++.

Just like for opaque types, these functions refer implicitly to something in
scope in the `super` module, whether defined there or imported by some `use`
statement.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyType;
        fn f() -> Box<MyType>;
    }
}

struct MyType(i32);

fn f() -> Box<MyType> {
    return Box::new(MyType(1));
}
```

Extern Rust function signature may consist of types defined in the bridge,
primitives, and [any of these additional bindings](bindings.md).

## Methods

Any signature with a `self` parameter is interpreted as a Rust method and
exposed to C++ as a non-static member function.

```rust,noplayground
# #[cxx::bridge]
# mod ffi {
    extern "Rust" {
        type MyType;
        fn f(&self) -> usize;
    }
# }
```

The `self` parameter may be a shared reference `&self`, an exclusive reference
`&mut self`, or a pinned reference `self: Pin<&mut Self>`. A by-value `self` is
not currently supported.

If the surrounding `extern "Rust"` block contains exactly one extern type, that
type is implicitly the receiver for a `&self` or `&mut self` method. If the
surrounding block contains *more than one* extern type, a receiver type must be
provided explicitly for the self parameter, or you can consider splitting into
multiple extern blocks.

```rust,noplayground
# #[cxx::bridge]
# mod ffi {
    extern "Rust" {
        type First;
        type Second;
        fn bar(self: &First);
        fn foo(self: &mut Second);
    }
# }
```

## Associated functions

A function with a `Self` attribute is interpreted as a Rust associated function
and exposed to C++ as a static member function. These must not have a `self`
argument.

In the following example, the `builder` associated function is callable as
`MyType::builder()` from both Rust and C++.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyType;
        type MyTypeBuilder;

        #[Self = "MyType"]
        fn builder() -> Box<MyTypeBuilder>;
    }
}

pub struct MyType;
pub struct MyTypeBuilder;

impl MyType {
    pub fn builder() -> Box<MyTypeBuilder> {
        ...
    }
}
```

## Functions with explicit lifetimes

An extern Rust function signature is allowed to contain explicit lifetimes but
in this case the function must be declared unsafe-to-call. This is pretty
meaningless given we're talking about calls from C++, but at least it draws some
extra attention from the caller that they may be responsible for upholding some
atypical lifetime relationship.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyType;
        unsafe fn f<'a>(&'a self, s: &str) -> &'a str;
    }
}
```

Bounds on a lifetime (like `<'a, 'b: 'a>`) are not currently supported. Nor are
type parameters or where-clauses.
