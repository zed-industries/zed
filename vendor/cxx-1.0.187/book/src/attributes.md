{{#title Attributes — Rust ♡ C++}}
# Attributes

## namespace

The top-level cxx::bridge attribute macro takes an optional `namespace` argument
to control the C++ namespace into which to emit extern Rust items and the
namespace in which to expect to find the extern C++ items.

```rust,noplayground
#[cxx::bridge(namespace = "path::of::my::company")]
mod ffi {
    extern "Rust" {
        type MyType;  // emitted to path::of::my::company::MyType
    }

    extern "C++" {
        type TheirType;  // refers to path::of::my::company::TheirType
    }
}
```

Additionally, a `#[namespace = "..."]` attribute may be used inside the bridge
module on any extern block or individual item. An item will inherit the
namespace specified on its surrounding extern block if any, otherwise the
namespace specified with the top level cxx::bridge attribute if any, otherwise
the global namespace.

```rust,noplayground
#[cxx::bridge(namespace = "third_priority")]
mod ffi {
    #[namespace = "second_priority"]
    extern "Rust" {
        fn f();

        #[namespace = "first_priority"]
        fn g();
    }

    extern "Rust" {
        fn h();
    }
}
```

The above would result in functions `::second_priority::f`,
`::first_priority::g`, `::third_priority::h`.

## rust\_name, cxx\_name

Sometimes you want the Rust name of a function or type to differ from its C++
name. Importantly, this enables binding multiple overloads of the same C++
function name using distinct Rust names.

```rust,noplayground
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        #[rust_name = "i32_overloaded_function"]
        fn cOverloadedFunction(x: i32) -> String;
        #[rust_name = "str_overloaded_function"]
        fn cOverloadedFunction(x: &str) -> String;
    }
}
```

The `#[rust_name = "..."]` attribute replaces the name that Rust should use for
this function, and an analogous `#[cxx_name = "..."]` attribute replaces the
name that C++ should use.

Either of the two attributes may be used on extern "Rust" as well as extern
"C++" functions, according to which one you find clearer in context.

The same attribute works for renaming functions, opaque types, shared
structs and enums, and enum variants.

## Self

Indicates the name of the type in which to place a [Rust associated function] or
[C++ static member function].

[Rust associated function]: extern-rust.md#associated-functions
[C++ static member function]: extern-c++.md#functions-and-member-functions

```rust,noplayground
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type RustType;

        #[Self = "RustType"]
        fn member();  // callable from C++ as `RustType::member()`
    }

    unsafe extern "C++" {
        type CppType;

        #[Self = "CppType"]
        fn member();  // callable from Rust as `CppType::member()`
    }
}
```
