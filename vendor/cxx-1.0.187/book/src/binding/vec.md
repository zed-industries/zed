{{#title rust::Vec<T> — Rust ♡ C++}}
# rust::Vec\<T\>

### Public API:

```cpp,hidelines=...
// rust/cxx.h
...
...#include <initializer_list>
...#include <iterator>
...#include <type_traits>
...
...namespace rust {

template <typename T>
class Vec final {
public:
  using value_type = T;

  Vec() noexcept;
  Vec(std::initializer_list<T>);
  Vec(const Vec &);
  Vec(Vec &&) noexcept;
  ~Vec() noexcept;

  Vec &operator=(Vec &&) & noexcept;
  Vec &operator=(const Vec &) &;

  size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  size_t capacity() const noexcept;

  const T &operator[](size_t n) const noexcept;
  const T &at(size_t n) const;
  const T &front() const;
  const T &back() const;

  T &operator[](size_t n) noexcept;
  T &at(size_t n);
  T &front();
  T &back();

  void reserve(size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&...args);
  void truncate(size_t len);
  void clear();

  class iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  class const_iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  void swap(Vec &) noexcept;
};
...
...template <typename T>
...class Vec<T>::iterator final {
...public:
...#if __cplusplus >= 202002L
...  using iterator_category = std::contiguous_iterator_tag;
...#else
...  using iterator_category = std::random_access_iterator_tag;
...#endif
...  using value_type = T;
...  using pointer = T *;
...  using reference = T &;
...
...  T &operator*() const noexcept;
...  T *operator->() const noexcept;
...  T &operator[](ptrdiff_t) const noexcept;
...
...  iterator &operator++() noexcept;
...  iterator operator++(int) noexcept;
...  iterator &operator--() noexcept;
...  iterator operator--(int) noexcept;
...
...  iterator &operator+=(ptrdiff_t) noexcept;
...  iterator &operator-=(ptrdiff_t) noexcept;
...  iterator operator+(ptrdiff_t) const noexcept;
...  iterator operator-(ptrdiff_t) const noexcept;
...  ptrdiff_t operator-(const iterator &) const noexcept;
...
...  bool operator==(const iterator &) const noexcept;
...  bool operator!=(const iterator &) const noexcept;
...  bool operator<(const iterator &) const noexcept;
...  bool operator<=(const iterator &) const noexcept;
...  bool operator>(const iterator &) const noexcept;
...  bool operator>=(const iterator &) const noexcept;
...};
...
...template <typename T>
...class Vec<T>::const_iterator final {
...public:
...#if __cplusplus >= 202002L
...  using iterator_category = std::contiguous_iterator_tag;
...#else
...  using iterator_category = std::random_access_iterator_tag;
...#endif
...  using value_type = const T;
...  using pointer = const T *;
...  using reference = const T &;
...
...  const T &operator*() const noexcept;
...  const T *operator->() const noexcept;
...  const T &operator[](ptrdiff_t) const noexcept;
...
...  const_iterator &operator++() noexcept;
...  const_iterator operator++(int) noexcept;
...  const_iterator &operator--() noexcept;
...  const_iterator operator--(int) noexcept;
...
...  const_iterator &operator+=(ptrdiff_t) noexcept;
...  const_iterator &operator-=(ptrdiff_t) noexcept;
...  const_iterator operator+(ptrdiff_t) const noexcept;
...  const_iterator operator-(ptrdiff_t) const noexcept;
...  ptrdiff_t operator-(const const_iterator &) const noexcept;
...
...  bool operator==(const const_iterator &) const noexcept;
...  bool operator!=(const const_iterator &) const noexcept;
...  bool operator<(const const_iterator &) const noexcept;
...  bool operator<=(const const_iterator &) const noexcept;
...  bool operator>(const const_iterator &) const noexcept;
...  bool operator>=(const const_iterator &) const noexcept;
...};
...
...} // namespace rust
```

### Restrictions:

Vec\<T\> does not support T being an opaque C++ type. You should use
CxxVector\<T\> (C++ std::vector\<T\>) instead for collections of opaque C++
types on the language boundary.

## Example

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    struct Shared {
        v: u32,
    }

    unsafe extern "C++" {
        include!("example/include/example.h");

        fn f(elements: Vec<Shared>);
    }
}

fn main() {
    let shared = |v| ffi::Shared { v };
    let elements = vec![shared(3), shared(2), shared(1)];
    ffi::f(elements);
}
```

```cpp
// include/example.h

#pragma once
#include "example/src/main.rs.h"
#include "rust/cxx.h"

void f(rust::Vec<Shared> elements);
```

```cpp
// src/example.cc

#include "example/include/example.h"
#include <algorithm>
#include <cassert>
#include <iostream>
#include <iterator>
#include <vector>

void f(rust::Vec<Shared> v) {
  for (auto shared : v) {
    std::cout << shared.v << std::endl;
  }

  // Copy the elements to a C++ std::vector using STL algorithm.
  std::vector<Shared> stdv;
  std::copy(v.begin(), v.end(), std::back_inserter(stdv));
  assert(v.size() == stdv.size());
}
```
