{{#title Result<T> — Rust ♡ C++}}
# Result\<T\>

Result\<T\> is allowed as the return type of an extern function in either
direction. Its behavior is to translate to/from C++ exceptions. If your codebase
does not use C++ exceptions, or prefers to represent fallibility using something
like outcome\<T\>, leaf::result\<T\>, StatusOr\<T\>, etc then you'll need to
handle the translation of those to Rust Result\<T\> using your own shims for
now. Better support for this is planned.

If an exception is thrown from an `extern "C++"` function that is *not* declared
by the CXX bridge to return Result, the program calls C++'s `std::terminate`.
The behavior is equivalent to the same exception being thrown through a
`noexcept` C++ function.

If a panic occurs in *any* `extern "Rust"` function, regardless of whether it is
declared by the CXX bridge to return Result, a message is logged and the program
calls Rust's `std::process::abort`.

## Returning Result from Rust to C++

An `extern "Rust"` function returning a Result turns into a `throw` in C++ if
the Rust side produces an error.

Note that the return type written inside of cxx::bridge must be written without
a second type parameter. Only the Ok type is specified for the purpose of the
FFI. The Rust *implementation* (outside of the bridge module) may pick any error
type as long as it has a std::fmt::Display impl.

```rust,noplayground
# use std::io;
#
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn fallible1(depth: usize) -> Result<String>;
        fn fallible2() -> Result<()>;
    }
}

fn fallible1(depth: usize) -> anyhow::Result<String> {
    if depth == 0 {
        return Err(anyhow::Error::msg("fallible1 requires depth > 0"));
    }
    ...
}

fn fallible2() -> Result<(), io::Error> {
    ...
    Ok(())
}
```

The exception that gets thrown by CXX on the C++ side is always of type
`rust::Error` and has the following C++ public API. The `what()` member function
gives the error message according to the Rust error's std::fmt::Display impl.

```cpp,hidelines=...
// rust/cxx.h
...
...namespace rust {

class Error final : public std::exception {
public:
  Error(const Error &);
  Error(Error &&) noexcept;
  ~Error() noexcept;

  Error &operator=(const Error &) &;
  Error &operator=(Error &&) & noexcept;

  const char *what() const noexcept override;
};
...
...} // namespace rust
```

## Returning Result from C++ to Rust

An `extern "C++"` function returning a Result turns into a `catch` in C++ that
converts the exception into an Err for Rust.

Note that the return type written inside of cxx::bridge must be written without
a second type parameter. Only the Ok type is specified for the purpose of the
FFI. The resulting error type created by CXX when an `extern "C++"` function
throws will always be of type **[`cxx::Exception`]**.

[`cxx::Exception`]: https://docs.rs/cxx/*/cxx/struct.Exception.html

```rust,noplayground
# use std::process;
#
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/example.h");
        fn fallible1(depth: usize) -> Result<String>;
        fn fallible2() -> Result<()>;
    }
}

fn main() {
    if let Err(err) = ffi::fallible1(99) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
```

The specific set of caught exceptions and the conversion to error message are
both customizable. The way you do this is by defining a template function
`rust::behavior::trycatch` with a suitable signature inside any one of the
headers `include!`'d by your cxx::bridge.

The template signature is required to be:

```cpp
namespace rust {
namespace behavior {

template <typename Try, typename Fail>
static void trycatch(Try &&func, Fail &&fail) noexcept;

} // namespace behavior
} // namespace rust
```

The default `trycatch` used by CXX if you have not provided your own is the
following. You must follow the same pattern: invoke `func` with no arguments,
catch whatever exception(s) you want, and invoke `fail` with the error message
you'd like for the Rust error to have.

```cpp,hidelines=...
...#include <exception>
...
...namespace rust {
...namespace behavior {
...
template <typename Try, typename Fail>
static void trycatch(Try &&func, Fail &&fail) noexcept try {
  func();
} catch (const std::exception &e) {
  fail(e.what());
}
...
...} // namespace behavior
...} // namespace rust
```
