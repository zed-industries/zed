{{#title std::unique_ptr<T> — Rust ♡ C++}}
# std::unique\_ptr\<T\>

The Rust binding of std::unique\_ptr\<T\> is called **[`UniquePtr<T>`]**. See
the link for documentation of the Rust API.

[`UniquePtr<T>`]: https://docs.rs/cxx/*/cxx/struct.UniquePtr.html

### Restrictions:

Only `std::unique_ptr<T, std::default_delete<T>>` is currently supported. Custom
deleters may be supported in the future.

UniquePtr\<T\> does not support T being an opaque Rust type. You should use a
Box\<T\> (C++ [rust::Box\<T\>](box.md)) instead for transferring ownership of
opaque Rust types on the language boundary.

## Example

UniquePtr is commonly useful for returning opaque C++ objects to Rust. This use
case was featured in the [*blobstore tutorial*](../tutorial.md).

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        // ...
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
    // ...
}
```

```cpp
// include/blobstore.h

#pragma once
#include <memory>

class BlobstoreClient;

std::unique_ptr<BlobstoreClient> new_blobstore_client();
```

```cpp
// src/blobstore.cc

#include "example/include/blobstore.h"

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::make_unique<BlobstoreClient>();
}
```
