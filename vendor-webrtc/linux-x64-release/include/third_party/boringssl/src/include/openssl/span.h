// Copyright 2017 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_SSL_SPAN_H
#define OPENSSL_HEADER_SSL_SPAN_H

#include <openssl/base.h>   // IWYU pragma: export

#if !defined(BORINGSSL_NO_CXX)

extern "C++" {

#include <stdlib.h>

#include <algorithm>
#include <string_view>
#include <type_traits>

#if __has_include(<version>)
#include <version>
#endif

#if defined(__cpp_lib_ranges) && __cpp_lib_ranges >= 201911L
#include <ranges>
BSSL_NAMESPACE_BEGIN
template <typename T>
class Span;
BSSL_NAMESPACE_END

// Mark `Span` as satisfying the `view` and `borrowed_range` concepts. This
// should be done before the definition of `Span`, so that any inlined calls to
// range functionality use the correct specializations.
template <typename T>
inline constexpr bool std::ranges::enable_view<bssl::Span<T>> = true;
template <typename T>
inline constexpr bool std::ranges::enable_borrowed_range<bssl::Span<T>> = true;
#endif

BSSL_NAMESPACE_BEGIN

template <typename T>
class Span;

namespace internal {
template <typename T>
class SpanBase {
  // Put comparison operator implementations into a base class with const T, so
  // they can be used with any type that implicitly converts into a Span.
  static_assert(std::is_const<T>::value,
                "Span<T> must be derived from SpanBase<const T>");

  friend bool operator==(Span<T> lhs, Span<T> rhs) {
    return std::equal(lhs.begin(), lhs.end(), rhs.begin(), rhs.end());
  }

  friend bool operator!=(Span<T> lhs, Span<T> rhs) { return !(lhs == rhs); }
};

// Heuristically test whether C is a container type that can be converted into
// a Span<T> by checking for data() and size() member functions.
template <typename C, typename T>
using EnableIfContainer = std::enable_if_t<
    std::is_convertible_v<decltype(std::declval<C>().data()), T *> &&
    std::is_integral_v<decltype(std::declval<C>().size())>>;

}  // namespace internal

// A Span<T> is a non-owning reference to a contiguous array of objects of type
// |T|. Conceptually, a Span is a simple a pointer to |T| and a count of
// elements accessible via that pointer. The elements referenced by the Span can
// be mutated if |T| is mutable.
//
// A Span can be constructed from container types implementing |data()| and
// |size()| methods. If |T| is constant, construction from a container type is
// implicit. This allows writing methods that accept data from some unspecified
// container type:
//
// // Foo views data referenced by v.
// void Foo(bssl::Span<const uint8_t> v) { ... }
//
// std::vector<uint8_t> vec;
// Foo(vec);
//
// For mutable Spans, conversion is explicit:
//
// // FooMutate mutates data referenced by v.
// void FooMutate(bssl::Span<uint8_t> v) { ... }
//
// FooMutate(bssl::Span<uint8_t>(vec));
//
// You can also use C++17 class template argument deduction to construct Spans
// in order to deduce the type of the Span automatically.
//
// FooMutate(bssl::Span(vec));
//
// Note that Spans have value type sematics. They are cheap to construct and
// copy, and should be passed by value whenever a method would otherwise accept
// a reference or pointer to a container or array.
template <typename T>
class Span : private internal::SpanBase<const T> {
 public:
  static const size_t npos = static_cast<size_t>(-1);

  using element_type = T;
  using value_type = std::remove_cv_t<T>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using pointer = T *;
  using const_pointer = const T *;
  using reference = T &;
  using const_reference = const T &;
  using iterator = T *;
  using const_iterator = const T *;

  constexpr Span() : Span(nullptr, 0) {}
  constexpr Span(T *ptr, size_t len) : data_(ptr), size_(len) {}

  template <size_t N>
  constexpr Span(T (&array)[N]) : Span(array, N) {}

  template <typename C, typename = internal::EnableIfContainer<C, T>,
            typename = std::enable_if_t<std::is_const<T>::value, C>>
  constexpr Span(const C &container)
      : data_(container.data()), size_(container.size()) {}

  template <typename C, typename = internal::EnableIfContainer<C, T>,
            typename = std::enable_if_t<!std::is_const<T>::value, C>>
  constexpr explicit Span(C &container)
      : data_(container.data()), size_(container.size()) {}

  constexpr T *data() const { return data_; }
  constexpr size_t size() const { return size_; }
  constexpr bool empty() const { return size_ == 0; }

  constexpr iterator begin() const { return data_; }
  constexpr const_iterator cbegin() const { return data_; }
  constexpr iterator end() const { return data_ + size_; }
  constexpr const_iterator cend() const { return end(); }

  constexpr T &front() const {
    if (size_ == 0) {
      abort();
    }
    return data_[0];
  }
  constexpr T &back() const {
    if (size_ == 0) {
      abort();
    }
    return data_[size_ - 1];
  }

  constexpr T &operator[](size_t i) const {
    if (i >= size_) {
      abort();
    }
    return data_[i];
  }
  T &at(size_t i) const { return (*this)[i]; }

  constexpr Span subspan(size_t pos = 0, size_t len = npos) const {
    if (pos > size_) {
      // absl::Span throws an exception here. Note std::span and Chromium
      // base::span additionally forbid pos + len being out of range, with a
      // special case at npos/dynamic_extent, while absl::Span::subspan clips
      // the span. For now, we align with absl::Span in case we switch to it in
      // the future.
      abort();
    }
    return Span(data_ + pos, std::min(size_ - pos, len));
  }

  constexpr Span first(size_t len) const {
    if (len > size_) {
      abort();
    }
    return Span(data_, len);
  }

  constexpr Span last(size_t len) const {
    if (len > size_) {
      abort();
    }
    return Span(data_ + size_ - len, len);
  }

 private:
  T *data_;
  size_t size_;
};

template <typename T>
const size_t Span<T>::npos;

template <typename T>
Span(T *, size_t) -> Span<T>;
template <typename T, size_t size>
Span(T (&array)[size]) -> Span<T>;
template <
    typename C,
    typename T = std::remove_pointer_t<decltype(std::declval<C>().data())>,
    typename = internal::EnableIfContainer<C, T>>
Span(C &) -> Span<T>;

template <typename T>
constexpr Span<T> MakeSpan(T *ptr, size_t size) {
  return Span<T>(ptr, size);
}

template <typename C>
constexpr auto MakeSpan(C &c) -> decltype(MakeSpan(c.data(), c.size())) {
  return MakeSpan(c.data(), c.size());
}

template <typename T, size_t N>
constexpr Span<T> MakeSpan(T (&array)[N]) {
  return Span<T>(array, N);
}

template <typename T>
constexpr Span<const T> MakeConstSpan(T *ptr, size_t size) {
  return Span<const T>(ptr, size);
}

template <typename C>
constexpr auto MakeConstSpan(const C &c)
    -> decltype(MakeConstSpan(c.data(), c.size())) {
  return MakeConstSpan(c.data(), c.size());
}

template <typename T, size_t size>
constexpr Span<const T> MakeConstSpan(T (&array)[size]) {
  return array;
}

inline Span<const uint8_t> StringAsBytes(std::string_view s) {
  return MakeConstSpan(reinterpret_cast<const uint8_t *>(s.data()), s.size());
}

inline std::string_view BytesAsStringView(bssl::Span<const uint8_t> b) {
  return std::string_view(reinterpret_cast<const char *>(b.data()), b.size());
}

BSSL_NAMESPACE_END

}  // extern C++

#endif  // !defined(BORINGSSL_NO_CXX)

#endif  // OPENSSL_HEADER_SSL_SPAN_H
