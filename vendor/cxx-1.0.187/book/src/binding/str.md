{{#title rust::Str — Rust ♡ C++}}
# rust::Str

### Public API:

```cpp,hidelines=...
// rust/cxx.h
...
...#include <iosfwd>
...#include <string>
...
...namespace rust {

class Str final {
public:
  Str() noexcept;
  Str(const Str &) noexcept;
  Str(const String &) noexcept;

  // Throws std::invalid_argument if not utf-8.
  Str(const std::string &);
  Str(const char *);
  Str(const char *, size_t);

  Str &operator=(const Str &) & noexcept;

  explicit operator std::string() const;
#if __cplusplus >= 201703L
  explicit operator std::string_view() const;
#endif

  // Note: no null terminator.
  const char *data() const noexcept;
  // Length in bytes.
  size_t size() const noexcept;
  // Length in bytes, same as size().
  size_t length() const noexcept;
  bool empty() const noexcept;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;
};

std::ostream &operator<<(std::ostream &, const Str &);
...
...} // namespace rust
```

### Notes:

**Be aware that rust::Str behaves like &amp;str i.e. it is a borrow!**&ensp;C++
needs to be mindful of the lifetimes at play.

Just to reiterate: &amp;str is rust::Str. Do not try to write &amp;str as `const
rust::Str &`. A language-level C++ reference is not able to capture the fat
pointer nature of &amp;str.

### Restrictions:

Allowed as function argument or return value. Not supported in shared structs
yet. `&mut str` is not supported yet, but is also extremely obscure so this is
fine.

## Example

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn r(greeting: &str);
    }

    unsafe extern "C++" {
        include!("example/include/greeting.h");
        fn c(greeting: &str);
    }
}

fn r(greeting: &str) {
    println!("{}", greeting);
}

fn main() {
    ffi::c("hello from Rust");
}
```

```cpp
// include/greeting.h

#pragma once
#include "example/src/main.rs.h"
#include "rust/cxx.h"

void c(rust::Str greeting);
```

```cpp
// src/greeting.cc

#include "example/include/greeting.h"
#include <iostream>

void c(rust::Str greeting) {
  std::cout << greeting << std::endl;
  r("hello from C++");
}
```
