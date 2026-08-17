{{#title std::shared_ptr<T> — Rust ♡ C++}}
# std::shared\_ptr\<T\>

The Rust binding of std::shared\_ptr\<T\> is called **[`SharedPtr<T>`]**. See
the link for documentation of the Rust API.

[`SharedPtr<T>`]: https://docs.rs/cxx/*/cxx/struct.SharedPtr.html

### Restrictions:

SharedPtr\<T\> does not support T being an opaque Rust type. You should use a
Box\<T\> (C++ [rust::Box\<T\>](box.md)) instead for transferring ownership of
opaque Rust types on the language boundary.

## Example

```rust,noplayground
// src/main.rs

use std::ops::Deref;
use std::ptr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/example.h");

        type Object;

        fn create_shared_ptr() -> SharedPtr<Object>;
    }
}

fn main() {
    let ptr1 = ffi::create_shared_ptr();

    {
        // Create a second shared_ptr holding shared ownership of the same
        // object. There is still only one Object but two SharedPtr<Object>.
        // Both pointers point to the same object on the heap.
        let ptr2 = ptr1.clone();
        assert!(ptr::eq(ptr1.deref(), ptr2.deref()));

        // ptr2 goes out of scope, but Object is not destroyed yet.
    }

    println!("say goodbye to Object");

    // ptr1 goes out of scope and Object is destroyed.
}
```

```cpp
// include/example.h

#pragma once
#include <memory>

class Object {
public:
  Object();
  ~Object();
};

std::shared_ptr<Object> create_shared_ptr();
```

```cpp
// src/example.cc

#include "example/include/example.h"
#include <iostream>

Object::Object() { std::cout << "construct Object" << std::endl; }
Object::~Object() { std::cout << "~Object" << std::endl; }

std::shared_ptr<Object> create_shared_ptr() {
  return std::make_shared<Object>();
}
```
