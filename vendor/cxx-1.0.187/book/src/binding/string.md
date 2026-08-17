{{#title rust::String — Rust ♡ C++}}
# rust::String

### Public API:

```cpp,hidelines=...
// rust/cxx.h
...
...#include <iosfwd>
...#include <string>
...
...namespace rust {

class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  // Throws std::invalid_argument if not UTF-8.
  String(const std::string &);
  String(const char *);
  String(const char *, size_t);
  String(const char8_t *);
  String(const char8_t *, size_t);

  // Replaces invalid UTF-8 data with the replacement character (U+FFFD).
  static String lossy(const std::string &) noexcept;
  static String lossy(const char *) noexcept;
  static String lossy(const char *, size_t) noexcept;

  // Throws std::invalid_argument if not UTF-16.
  String(const char16_t *);
  String(const char16_t *, size_t);

  // Replaces invalid UTF-16 data with the replacement character (U+FFFD).
  static String lossy(const char16_t *) noexcept;
  static String lossy(const char16_t *, size_t) noexcept;

  String &operator=(const String &) & noexcept;
  String &operator=(String &&) & noexcept;

  explicit operator std::string() const;

  // Note: no null terminator.
  const char *data() const noexcept;
  // Length in bytes.
  size_t size() const noexcept;
  // Length in bytes, same as size().
  size_t length() const noexcept;
  bool empty() const noexcept;

  const char *c_str() noexcept;

  size_t capacity() const noexcept;
  void reserve(size_t new_cap) noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  void swap(String &) noexcept;
};

std::ostream &operator<<(std::ostream &, const String &);
...
...} // namespace rust
```

### Restrictions:

None. Strings may be used as function arguments and function return values, by
value or by reference, as well as fields of shared structs.

## Example

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    struct ConcatRequest {
        fst: String,
        snd: String,
    }

    unsafe extern "C++" {
        include!("example/include/concat.h");
        fn concat(r: ConcatRequest) -> String;
    }
}

fn main() {
    let concatenated = ffi::concat(ffi::ConcatRequest {
        fst: "fearless".to_owned(),
        snd: "concurrency".to_owned(),
    });
    println!("concatenated: {:?}", concatenated);
}
```

```cpp
// include/concat.h

#pragma once
#include "example/src/main.rs.h"
#include "rust/cxx.h"

rust::String concat(ConcatRequest r);
```

```cpp
// src/concat.cc

#include "example/include/concat.h"

rust::String concat(ConcatRequest r) {
  // The full suite of operator overloads hasn't been added
  // yet on rust::String, but we can get it done like this:
  return std::string(r.fst) + std::string(r.snd);
}
```
