{{#title rust::Box<T> — Rust ♡ C++}}
# rust::Box\<T\>

### Public API:

```cpp,hidelines=...
// rust/cxx.h
...
...#include <type_traits>
...
...namespace rust {

template <typename T>
class Box final {
public:
  using element_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(Box &&) & noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  void swap(Box &) noexcept;

  // Important: requires that `raw` came from an into_raw call. Do not
  // pass a pointer from `new` or any other source.
  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;
};
...
...} // namespace rust
```

### Restrictions:

Box\<T\> does not support T being an opaque C++ type. You should use
[UniquePtr\<T\>](uniqueptr.md) or [SharedPtr\<T\>](sharedptr.md) instead for
transferring ownership of opaque C++ types on the language boundary.

If T is an opaque Rust type, the Rust type is required to be [Sized] i.e. size
known at compile time. In the future we may introduce support for dynamically
sized opaque Rust types.

[Sized]: https://doc.rust-lang.org/std/marker/trait.Sized.html

## Example

This program uses a Box to pass ownership of some opaque piece of Rust state
over to C++ and then back to a Rust callback, which is a useful pattern for
implementing [async functions over FFI](../async.md).

```rust,noplayground
// src/main.rs

use std::io::Write;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type File;
    }

    unsafe extern "C++" {
        include!("example/include/example.h");

        fn f(
            callback: fn(Box<File>, fst: &str, snd: &str),
            out: Box<File>,
        );
    }
}

pub struct File(std::fs::File);

fn main() {
    let out = std::fs::File::create("example.log").unwrap();

    ffi::f(
        |mut out, fst, snd| { let _ = write!(out.0, "{}{}\n", fst, snd); },
        Box::new(File(out)),
    );
}
```

```cpp
// include/example.h

#pragma once
#include "example/src/main.rs.h"
#include "rust/cxx.h"

void f(rust::Fn<void(rust::Box<File>, rust::Str, rust::Str)> callback,
       rust::Box<File> out);
```

```cpp
// include/example.cc

#include "example/include/example.h"

void f(rust::Fn<void(rust::Box<File>, rust::Str, rust::Str)> callback,
       rust::Box<File> out) {
  callback(std::move(out), "fearless", "concurrency");
}
```
