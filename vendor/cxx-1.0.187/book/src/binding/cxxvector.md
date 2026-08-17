{{#title std::vector<T> — Rust ♡ C++}}
# std::vector\<T\>

The Rust binding of std::vector\<T\> is called **[`CxxVector<T>`]**. See the
link for documentation of the Rust API.

[`CxxVector<T>`]: https://docs.rs/cxx/*/cxx/struct.CxxVector.html

### Restrictions:

Rust code can never obtain a CxxVector by value. Instead in Rust code we will
only ever look at a vector behind a reference or smart pointer, as in
&CxxVector\<T\> or UniquePtr\<CxxVector\<T\>\>.

CxxVector\<T\> does not support T being an opaque Rust type. You should use a
Vec\<T\> (C++ rust::Vec\<T\>) instead for collections of opaque Rust types on
the language boundary.

## Example

This program involves Rust code converting a `CxxVector<CxxString>` (i.e.
`std::vector<std::string>`) into a Rust `Vec<String>`.

```rust,noplayground
// src/main.rs

#![no_main] // main defined in C++ by main.cc

use cxx::{CxxString, CxxVector};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn f(vec: &CxxVector<CxxString>);
    }
}

fn f(vec: &CxxVector<CxxString>) {
    let vec: Vec<String> = vec
        .iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    g(&vec);
}

fn g(vec: &[String]) {
    println!("{:?}", vec);
}
```

```cpp
// src/main.cc

#include "example/src/main.rs.h"
#include <string>
#include <vector>

int main() {
  std::vector<std::string> vec{"fearless", "concurrency"};
  f(vec);
}
```
