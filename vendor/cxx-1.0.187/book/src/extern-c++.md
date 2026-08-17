{{#title extern "C++" — Rust ♡ C++}}
# extern "C++"

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("path/to/header.h");
        include!("path/to/another.h");

        ...
    }
}
```

The `extern "C++"` section of a CXX bridge declares C++ types and signatures to
be made available to Rust, and gives the paths of the header(s) which contain
the corresponding C++ declarations.

A bridge module may contain zero or more extern "C++" blocks.

## Opaque C++ types

Type defined in C++ that are made available to Rust, but only behind an
indirection.

```rust,noplayground
# #[cxx::bridge]
# mod ffi {
    extern "C++" {
        # include!("path/to/header.h");
        #
        type MyType;
        type MyOtherType;
    }
# }
```

For example in the ***[Tutorial](tutorial.md)*** we saw `BlobstoreClient`
implemented as an opaque C++ type. The blobstore client was created in C++ and
returned to Rust by way of a UniquePtr.

**Mutability:** Unlike extern Rust types and shared types, an extern C++ type is
not permitted to be passed by plain mutable reference `&mut MyType` across the
FFI bridge. For mutation support, the bridge is required to use `Pin<&mut
MyType>`. This is to safeguard against things like mem::swap-ing the contents of
two mutable references, given that Rust doesn't have information about the size
of the underlying object and couldn't invoke an appropriate C++ move constructor
anyway.

**Thread safety:** Be aware that CXX does not assume anything about the thread
safety of your extern C++ types. In other words the `MyType` etc bindings which
CXX produces for you in Rust *do not* come with `Send` and `Sync` impls. If you
are sure that your C++ type satisfies the requirements of `Send` and/or `Sync`
and need to leverage that fact from Rust, you must provide your own unsafe
marker trait impls.

```rust,noplayground
# #[cxx::bridge]
# mod ffi {
#     extern "C++" {
#         include!("path/to/header.h");
#
#         type MyType;
#     }
# }
#
/// The C++ implementation of MyType is thread safe.
unsafe impl Send for ffi::MyType {}
unsafe impl Sync for ffi::MyType {}
```

Take care in doing this because thread safety in C++ can be extremely tricky to
assess if you are coming from a Rust background. For example the
`BlobstoreClient` type in the tutorial is *not thread safe* despite doing only
completely innocuous things in its implementation. Concurrent calls to the `tag`
member function trigger a data race on the `blobs` map.

## Functions and member functions

This largely follows the same principles as ***[extern
"Rust"](extern-rust.md)*** functions and methods. In particular, any signature
with a `self` parameter is interpreted as a C++ non-static member function and
exposed to Rust as a method; any signature with a `#[Self = "…"]` attribute is
interpreted as a C++ static member function and exposed to Rust as an associated
function.

The programmer **does not** need to promise that the signatures they have typed
in are accurate; that would be unreasonable. CXX performs static assertions that
the signatures exactly correspond with what is declared in C++. Rather, the
programmer is only on the hook for things that C++'s static information is not
precise enough to capture, i.e. things that would only be represented at most by
comments in the C++ code unintelligible to a static assertion: namely whether
the C++ function is safe or unsafe to be called from Rust.

**Safety:** the extern "C++" block is responsible for deciding whether to expose
each signature inside as safe-to-call or unsafe-to-call. If an extern block
contains at least one safe-to-call signature, it must be written as an `unsafe
extern` block, which serves as an item level unsafe block to indicate that an
unchecked safety claim is being made about the contents of the block.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        # include!("path/to/header.h");
        #
        fn f();  // safe to call
    }

    extern "C++" {
        unsafe fn g();  // unsafe to call
    }
}
```

## Lifetimes

C++ types holding borrowed data may be described naturally in Rust by an extern
type with a generic lifetime parameter. For example in the case of the following
pair of types:

```cpp
// header.h

class Resource;

class TypeContainingBorrow {
  TypeContainingBorrow(const Resource &res) : res(res) {}
  const Resource &res;
};

std::shared_ptr<TypeContainingBorrow> create(const Resource &res);
```

we'd want to expose this to Rust as:

```rust,noplayground
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        # include!("path/to/header.h");
        #
        type Resource;
        type TypeContainingBorrow<'a>;

        fn create<'a>(res: &'a Resource) -> SharedPtr<TypeContainingBorrow<'a>>;

        // or with lifetime elision:
        fn create(res: &Resource) -> SharedPtr<TypeContainingBorrow>;
    }
}
```

## Reusing existing binding types

Extern C++ types support a syntax for declaring that a Rust binding of the
correct C++ type already exists outside of the current bridge module. This
avoids generating a fresh new binding which Rust's type system would consider
non-interchangeable with the first.

```rust,noplayground
#[cxx::bridge(namespace = "path::to")]
mod ffi {
    extern "C++" {
        type MyType = crate::existing::MyType;
    }

    extern "Rust" {
        fn f(x: &MyType) -> usize;
    }
}
```

In this case rather than producing a unique new Rust type `ffi::MyType` for the
Rust binding of C++'s `::path::to::MyType`, CXX will reuse the already existing
binding at `crate::existing::MyType` in expressing the signature of `f` and any
other uses of `MyType` within the bridge module.

CXX safely validates that `crate::existing::MyType` is in fact a binding for the
right C++ type `::path::to::MyType` by generating a static assertion based on
`crate::existing::MyType`'s implementation of [`ExternType`], which is a trait
automatically implemented by CXX for bindings that it generates but can also be
manually implemented as described below.

[`ExternType`]: https://docs.rs/cxx/*/cxx/trait.ExternType.html

`ExternType` serves the following two related use cases.

#### Safely unifying occurrences of an extern type across bridges

In the following snippet, two #\[cxx::bridge\] invocations in different files
(possibly different crates) both contain function signatures involving the same
C++ type `example::Demo`. If both were written just containing `type Demo;`,
then both macro expansions would produce their own separate Rust type called
`Demo` and thus the compiler wouldn't allow us to take the `Demo` returned by
`file1::ffi::create_demo` and pass it as the `Demo` argument accepted by
`file2::ffi::take_ref_demo`. Instead, one of the two `Demo`s has been defined as
an extern type alias of the other, making them the same type in Rust.

```rust,noplayground
// file1.rs
#[cxx::bridge(namespace = "example")]
pub mod ffi {
    unsafe extern "C++" {
        type Demo;

        fn create_demo() -> UniquePtr<Demo>;
    }
}
```

```rust,noplayground
// file2.rs
#[cxx::bridge(namespace = "example")]
pub mod ffi {
    unsafe extern "C++" {
        type Demo = crate::file1::ffi::Demo;

        fn take_ref_demo(demo: &Demo);
    }
}
```

#### Integrating with bindgen-generated or handwritten unsafe bindings

Handwritten `ExternType` impls make it possible to plug in a data structure
emitted by bindgen as the definition of a C++ type emitted by CXX.

By writing the unsafe `ExternType` impl, the programmer asserts that the C++
namespace and type name given in the type id refers to a C++ type that is
equivalent to Rust type that is the `Self` type of the impl.

```rust,noplayground
mod folly_sys;  // the bindgen-generated bindings

use cxx::{type_id, ExternType};

unsafe impl ExternType for folly_sys::StringPiece {
    type Id = type_id!("folly::StringPiece");
    type Kind = cxx::kind::Opaque;
}

#[cxx::bridge(namespace = "folly")]
pub mod ffi {
    unsafe extern "C++" {
        include!("rust_cxx_bindings.h");

        type StringPiece = crate::folly_sys::StringPiece;

        fn print_string_piece(s: &StringPiece);
    }
}

// Now if we construct a StringPiece or obtain one through one
// of the bindgen-generated signatures, we are able to pass it
// along to ffi::print_string_piece.
```

The `ExternType::Id` associated type encodes a type-level representation of the
type's C++ namespace and type name. It will always be defined using the
`type_id!` macro exposed in the cxx crate.

The `ExternType::Kind` associated type will always be either
[`cxx::kind::Opaque`] or [`cxx::kind::Trivial`] identifying whether a C++ type
is soundly relocatable by Rust's move semantics. A C++ type is only okay to hold
and pass around by value in Rust if its [move constructor is trivial] and it has
no destructor. In CXX, these are called Trivial extern C++ types, while types
with nontrivial move behavior or a destructor must be considered Opaque and
handled by Rust only behind an indirection, such as a reference or UniquePtr.

[`cxx::kind::Opaque`]: https://docs.rs/cxx/*/cxx/kind/enum.Opaque.html
[`cxx::kind::Trivial`]: https://docs.rs/cxx/*/cxx/kind/enum.Trivial.html
[move constructor is trivial]: https://en.cppreference.com/w/cpp/types/is_move_constructible

If you believe your C++ type reflected by the ExternType impl is indeed fine to
hold by value and move in Rust, you can specify:

```rust,noplayground
# unsafe impl cxx::ExternType for TypeName {
#     type Id = cxx::type_id!("name::space::of::TypeName");
    type Kind = cxx::kind::Trivial;
# }
```

which will enable you to pass it into C++ functions by value, return it by
value, and include it in `struct`s that you have declared to `cxx::bridge`. Your
claim about the triviality of the C++ type will be checked by a `static_assert`
in the generated C++ side of the binding.

## Explicit shim trait impls

This is a somewhat niche feature, but important when you need it.

CXX's support for C++'s std::unique\_ptr and std::vector is built on a set of
internal trait impls connecting the Rust API of UniquePtr and CxxVector to
underlying template instantiations performed by the C++ compiler.

When reusing a binding type across multiple bridge modules as described in the
previous section, you may find that your code needs some trait impls which CXX
hasn't decided to generate.

```rust,noplayground
#[cxx::bridge]
mod ffi1 {
    extern "C++" {
        include!("path/to/header.h");

        type A;
        type B;

        // Okay: CXX sees UniquePtr<B> using a type B defined within the same
        // bridge, and automatically emits the right template instantiations
        // corresponding to std::unique_ptr<B>.
        fn get_b() -> UniquePtr<B>;
    }
}

#[cxx::bridge]
mod ffi2 {
    extern "C++" {
        type A = crate::ffi1::A;

        // Rust trait error: CXX processing this module has no visibility into
        // whether template instantiations corresponding to std::unique_ptr<A>
        // have already been emitted by the upstream library, so it does not
        // emit them here. If the upstream library does not have any signatures
        // involving UniquePtr<A>, an explicit instantiation of the template
        // needs to be requested in one module or the other.
        fn get_a() -> UniquePtr<A>;
    }
}
```

You can request a specific template instantiation at a particular location in
the Rust crate hierarchy by writing `impl UniquePtr<A> {}` inside of the bridge
module which defines `A` but does not otherwise contain any use of
`UniquePtr<A>`.

```rust,noplayground
#[cxx::bridge]
mod ffi1 {
    extern "C++" {
        include!("path/to/header.h");

        type A;
        type B;

        fn get_b() -> UniquePtr<B>;
    }

    impl UniquePtr<A> {}  // explicit instantiation
}
```
