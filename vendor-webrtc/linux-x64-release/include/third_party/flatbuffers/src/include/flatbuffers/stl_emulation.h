/*
 * Copyright 2017 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_STL_EMULATION_H_
#define FLATBUFFERS_STL_EMULATION_H_

// clang-format off
#include "flatbuffers/base.h"

#include <string>
#include <type_traits>
#include <vector>
#include <memory>
#include <limits>

#ifndef FLATBUFFERS_USE_STD_OPTIONAL
  // Detect C++17 compatible compiler.
  // __cplusplus >= 201703L - a compiler has support of 'static inline' variables.
  #if (defined(__cplusplus) && __cplusplus >= 201703L) \
      || (defined(_MSVC_LANG) && _MSVC_LANG >= 201703L)
    #define FLATBUFFERS_USE_STD_OPTIONAL 1
  #else
    #define FLATBUFFERS_USE_STD_OPTIONAL 0
  #endif // (defined(__cplusplus) && __cplusplus >= 201703L) ...
#endif // FLATBUFFERS_USE_STD_OPTIONAL

#if FLATBUFFERS_USE_STD_OPTIONAL
  #include <optional>
#endif

#ifndef FLATBUFFERS_USE_STD_SPAN
  // Testing __cpp_lib_span requires including either <version> or <span>,
  // both of which were added in C++20.
  // See: https://en.cppreference.com/w/cpp/utility/feature_test
  #if defined(__cplusplus) && __cplusplus >= 202002L \
      || (defined(_MSVC_LANG) && _MSVC_LANG >= 202002L)
    #define FLATBUFFERS_USE_STD_SPAN 1
  #endif
#endif // FLATBUFFERS_USE_STD_SPAN

#if defined(FLATBUFFERS_USE_STD_SPAN)
  #include <array>
  #include <span>
#else
  // Disable non-trivial ctors if FLATBUFFERS_SPAN_MINIMAL defined.
  #if !defined(FLATBUFFERS_TEMPLATES_ALIASES)
    #define FLATBUFFERS_SPAN_MINIMAL
  #else
    // Enable implicit construction of a span<T,N> from a std::array<T,N>.
    #include <array>
  #endif
#endif // defined(FLATBUFFERS_USE_STD_SPAN)

// This header provides backwards compatibility for older versions of the STL.
namespace flatbuffers {

#if defined(FLATBUFFERS_TEMPLATES_ALIASES)
  template <typename T>
  using numeric_limits = std::numeric_limits<T>;
#else
  template <typename T> class numeric_limits :
    public std::numeric_limits<T> {};
#endif  // defined(FLATBUFFERS_TEMPLATES_ALIASES)

#if defined(FLATBUFFERS_TEMPLATES_ALIASES)
  template <typename T> using is_scalar = std::is_scalar<T>;
  template <typename T, typename U> using is_same = std::is_same<T,U>;
  template <typename T> using is_floating_point = std::is_floating_point<T>;
  template <typename T> using is_unsigned = std::is_unsigned<T>;
  template <typename T> using is_enum = std::is_enum<T>;
  template <typename T> using make_unsigned = std::make_unsigned<T>;
  template<bool B, class T, class F>
  using conditional = std::conditional<B, T, F>;
  template<class T, T v>
  using integral_constant = std::integral_constant<T, v>;
  template <bool B>
  using bool_constant = integral_constant<bool, B>;
  using true_type  = std::true_type;
  using false_type = std::false_type;
#else
  // MSVC 2010 doesn't support C++11 aliases.
  template <typename T> struct is_scalar : public std::is_scalar<T> {};
  template <typename T, typename U> struct is_same : public std::is_same<T,U> {};
  template <typename T> struct is_floating_point :
        public std::is_floating_point<T> {};
  template <typename T> struct is_unsigned : public std::is_unsigned<T> {};
  template <typename T> struct is_enum : public std::is_enum<T> {};
  template <typename T> struct make_unsigned : public std::make_unsigned<T> {};
  template<bool B, class T, class F>
  struct conditional : public std::conditional<B, T, F> {};
  template<class T, T v>
  struct integral_constant : public std::integral_constant<T, v> {};
  template <bool B>
  struct bool_constant : public integral_constant<bool, B> {};
  typedef bool_constant<true>  true_type;
  typedef bool_constant<false> false_type;
#endif  // defined(FLATBUFFERS_TEMPLATES_ALIASES)

#if defined(FLATBUFFERS_TEMPLATES_ALIASES)
  template <class T> using unique_ptr = std::unique_ptr<T>;
#else
  // MSVC 2010 doesn't support C++11 aliases.
  // We're manually "aliasing" the class here as we want to bring unique_ptr
  // into the flatbuffers namespace.  We have unique_ptr in the flatbuffers
  // namespace we have a completely independent implementation (see below)
  // for C++98 STL implementations.
  template <class T> class unique_ptr : public std::unique_ptr<T> {
    public:
    unique_ptr() {}
    explicit unique_ptr(T* p) : std::unique_ptr<T>(p) {}
    unique_ptr(std::unique_ptr<T>&& u) { *this = std::move(u); }
    unique_ptr(unique_ptr&& u) { *this = std::move(u); }
    unique_ptr& operator=(std::unique_ptr<T>&& u) {
      std::unique_ptr<T>::reset(u.release());
      return *this;
    }
    unique_ptr& operator=(unique_ptr&& u) {
      std::unique_ptr<T>::reset(u.release());
      return *this;
    }
    unique_ptr& operator=(T* p) {
      return std::unique_ptr<T>::operator=(p);
    }
  };
#endif  // defined(FLATBUFFERS_TEMPLATES_ALIASES)

#if FLATBUFFERS_USE_STD_OPTIONAL
template<class T>
using Optional = std::optional<T>;
using nullopt_t = std::nullopt_t;
inline constexpr nullopt_t nullopt = std::nullopt;

#else
// Limited implementation of Optional<T> type for a scalar T.
// This implementation limited by trivial types compatible with
// std::is_arithmetic<T> or std::is_enum<T> type traits.

// A tag to indicate an empty flatbuffers::optional<T>.
struct nullopt_t {
  explicit FLATBUFFERS_CONSTEXPR_CPP11 nullopt_t(int) {}
};

#if defined(FLATBUFFERS_CONSTEXPR_DEFINED)
  namespace internal {
    template <class> struct nullopt_holder {
      static constexpr nullopt_t instance_ = nullopt_t(0);
    };
    template<class Dummy>
    constexpr nullopt_t nullopt_holder<Dummy>::instance_;
  }
  static constexpr const nullopt_t &nullopt = internal::nullopt_holder<void>::instance_;

#else
  namespace internal {
    template <class> struct nullopt_holder {
      static const nullopt_t instance_;
    };
    template<class Dummy>
    const nullopt_t nullopt_holder<Dummy>::instance_  = nullopt_t(0);
  }
  static const nullopt_t &nullopt = internal::nullopt_holder<void>::instance_;

#endif

template<class T>
class Optional FLATBUFFERS_FINAL_CLASS {
  // Non-scalar 'T' would extremely complicated Optional<T>.
  // Use is_scalar<T> checking because flatbuffers flatbuffers::is_arithmetic<T>
  // isn't implemented.
  static_assert(flatbuffers::is_scalar<T>::value, "unexpected type T");

 public:
  ~Optional() {}

  FLATBUFFERS_CONSTEXPR_CPP11 Optional() FLATBUFFERS_NOEXCEPT
    : value_(), has_value_(false) {}

  FLATBUFFERS_CONSTEXPR_CPP11 Optional(nullopt_t) FLATBUFFERS_NOEXCEPT
    : value_(), has_value_(false) {}

  FLATBUFFERS_CONSTEXPR_CPP11 Optional(T val) FLATBUFFERS_NOEXCEPT
    : value_(val), has_value_(true) {}

  FLATBUFFERS_CONSTEXPR_CPP11 Optional(const Optional &other) FLATBUFFERS_NOEXCEPT
    : value_(other.value_), has_value_(other.has_value_) {}

  FLATBUFFERS_CONSTEXPR_CPP14 Optional &operator=(const Optional &other) FLATBUFFERS_NOEXCEPT {
    value_ = other.value_;
    has_value_ = other.has_value_;
    return *this;
  }

  FLATBUFFERS_CONSTEXPR_CPP14 Optional &operator=(nullopt_t) FLATBUFFERS_NOEXCEPT {
    value_ = T();
    has_value_ = false;
    return *this;
  }

  FLATBUFFERS_CONSTEXPR_CPP14 Optional &operator=(T val) FLATBUFFERS_NOEXCEPT {
    value_ = val;
    has_value_ = true;
    return *this;
  }

  void reset() FLATBUFFERS_NOEXCEPT {
    *this = nullopt;
  }

  void swap(Optional &other) FLATBUFFERS_NOEXCEPT {
    std::swap(value_, other.value_);
    std::swap(has_value_, other.has_value_);
  }

  FLATBUFFERS_CONSTEXPR_CPP11 FLATBUFFERS_EXPLICIT_CPP11 operator bool() const FLATBUFFERS_NOEXCEPT {
    return has_value_;
  }

  FLATBUFFERS_CONSTEXPR_CPP11 bool has_value() const FLATBUFFERS_NOEXCEPT {
    return has_value_;
  }

  FLATBUFFERS_CONSTEXPR_CPP11 const T& operator*() const FLATBUFFERS_NOEXCEPT {
    return value_;
  }

  const T& value() const {
    FLATBUFFERS_ASSERT(has_value());
    return value_;
  }

  T value_or(T default_value) const FLATBUFFERS_NOEXCEPT {
    return has_value() ? value_ : default_value;
  }

 private:
  T value_;
  bool has_value_;
};

template<class T>
FLATBUFFERS_CONSTEXPR_CPP11 bool operator==(const Optional<T>& opt, nullopt_t) FLATBUFFERS_NOEXCEPT {
  return !opt;
}
template<class T>
FLATBUFFERS_CONSTEXPR_CPP11 bool operator==(nullopt_t, const Optional<T>& opt) FLATBUFFERS_NOEXCEPT {
  return !opt;
}

template<class T, class U>
FLATBUFFERS_CONSTEXPR_CPP11 bool operator==(const Optional<T>& lhs, const U& rhs) FLATBUFFERS_NOEXCEPT {
  return static_cast<bool>(lhs) && (*lhs == rhs);
}

template<class T, class U>
FLATBUFFERS_CONSTEXPR_CPP11 bool operator==(const T& lhs, const Optional<U>& rhs) FLATBUFFERS_NOEXCEPT {
  return static_cast<bool>(rhs) && (lhs == *rhs);
}

template<class T, class U>
FLATBUFFERS_CONSTEXPR_CPP11 bool operator==(const Optional<T>& lhs, const Optional<U>& rhs) FLATBUFFERS_NOEXCEPT {
  return static_cast<bool>(lhs) != static_cast<bool>(rhs)
              ? false
              : !static_cast<bool>(lhs) ? true : (*lhs == *rhs);
}
#endif // FLATBUFFERS_USE_STD_OPTIONAL


// Very limited and naive partial implementation of C++20 std::span<T,Extent>.
#if defined(FLATBUFFERS_USE_STD_SPAN)
  inline constexpr std::size_t dynamic_extent = std::dynamic_extent;
  template<class T, std::size_t Extent = std::dynamic_extent>
  using span = std::span<T, Extent>;

#else // !defined(FLATBUFFERS_USE_STD_SPAN)
FLATBUFFERS_CONSTEXPR std::size_t dynamic_extent = static_cast<std::size_t>(-1);

// Exclude this code if MSVC2010 or non-STL Android is active.
// The non-STL Android doesn't have `std::is_convertible` required for SFINAE.
#if !defined(FLATBUFFERS_SPAN_MINIMAL)
namespace internal {
  // This is SFINAE helper class for checking of a common condition:
  // > This overload only participates in overload resolution
  // > Check whether a pointer to an array of From can be converted
  // > to a pointer to an array of To.
  // This helper is used for checking of 'From -> const From'.
  template<class To, std::size_t Extent, class From, std::size_t N>
  struct is_span_convertible {
    using type =
      typename std::conditional<std::is_convertible<From (*)[], To (*)[]>::value
                                && (Extent == dynamic_extent || N == Extent),
                                int, void>::type;
  };

  template<typename T>
  struct SpanIterator {
    // TODO: upgrade to std::random_access_iterator_tag.
    using iterator_category = std::forward_iterator_tag;
    using difference_type  = std::ptrdiff_t;
    using value_type = typename std::remove_cv<T>::type;
    using reference = T&;
    using pointer   = T*;

    // Convince MSVC compiler that this iterator is trusted (it is verified).
    #ifdef _MSC_VER
      using _Unchecked_type = pointer;
    #endif // _MSC_VER

    SpanIterator(pointer ptr) : ptr_(ptr) {}
    reference operator*() const { return *ptr_; }
    pointer operator->() { return ptr_; }
    SpanIterator& operator++() { ptr_++; return *this; }  
    SpanIterator  operator++(int) { auto tmp = *this; ++(*this); return tmp; }

    friend bool operator== (const SpanIterator& lhs, const SpanIterator& rhs) { return lhs.ptr_ == rhs.ptr_; }
    friend bool operator!= (const SpanIterator& lhs, const SpanIterator& rhs) { return lhs.ptr_ != rhs.ptr_; }

   private:
    pointer ptr_;
  };
}  // namespace internal
#endif  // !defined(FLATBUFFERS_SPAN_MINIMAL)

// T - element type; must be a complete type that is not an abstract
// class type.
// Extent - the number of elements in the sequence, or dynamic.
template<class T, std::size_t Extent = dynamic_extent>
class span FLATBUFFERS_FINAL_CLASS {
 public:
  typedef T element_type;
  typedef T& reference;
  typedef const T& const_reference;
  typedef T* pointer;
  typedef const T* const_pointer;
  typedef std::size_t size_type;

  static FLATBUFFERS_CONSTEXPR size_type extent = Extent;

  // Returns the number of elements in the span.
  FLATBUFFERS_CONSTEXPR_CPP11 size_type size() const FLATBUFFERS_NOEXCEPT {
    return count_;
  }

  // Returns the size of the sequence in bytes.
  FLATBUFFERS_CONSTEXPR_CPP11
  size_type size_bytes() const FLATBUFFERS_NOEXCEPT {
    return size() * sizeof(element_type);
  }

  // Checks if the span is empty.
  FLATBUFFERS_CONSTEXPR_CPP11 bool empty() const FLATBUFFERS_NOEXCEPT {
    return size() == 0;
  }

  // Returns a pointer to the beginning of the sequence.
  FLATBUFFERS_CONSTEXPR_CPP11 pointer data() const FLATBUFFERS_NOEXCEPT {
    return data_;
  }

  #if !defined(FLATBUFFERS_SPAN_MINIMAL)
    using Iterator = internal::SpanIterator<T>;

    Iterator begin() const { return Iterator(data()); }
    Iterator end() const   { return Iterator(data() + size()); }
  #endif

  // Returns a reference to the idx-th element of the sequence.
  // The behavior is undefined if the idx is greater than or equal to size().
  FLATBUFFERS_CONSTEXPR_CPP11 reference operator[](size_type idx) const {
    return data()[idx];
  }

  FLATBUFFERS_CONSTEXPR_CPP11 span(const span &other) FLATBUFFERS_NOEXCEPT
      : data_(other.data_), count_(other.count_) {}

  FLATBUFFERS_CONSTEXPR_CPP14 span &operator=(const span &other)
      FLATBUFFERS_NOEXCEPT {
    data_ = other.data_;
    count_ = other.count_;
  }

  // Limited implementation of
  // `template <class It> constexpr std::span(It first, size_type count);`.
  //
  // Constructs a span that is a view over the range [first, first + count);
  // the resulting span has: data() == first and size() == count.
  // The behavior is undefined if [first, first + count) is not a valid range,
  // or if (extent != flatbuffers::dynamic_extent && count != extent).
  FLATBUFFERS_CONSTEXPR_CPP11
  explicit span(pointer first, size_type count) FLATBUFFERS_NOEXCEPT
    : data_ (Extent == dynamic_extent ? first : (Extent == count ? first : nullptr)),
      count_(Extent == dynamic_extent ? count : (Extent == count ? Extent : 0)) {
      // Make span empty if the count argument is incompatible with span<T,N>.
  }

  // Exclude this code if MSVC2010 is active. The MSVC2010 isn't C++11
  // compliant, it doesn't support default template arguments for functions.
  #if defined(FLATBUFFERS_SPAN_MINIMAL)
  FLATBUFFERS_CONSTEXPR_CPP11 span() FLATBUFFERS_NOEXCEPT : data_(nullptr),
                                                            count_(0) {
    static_assert(extent == 0 || extent == dynamic_extent, "invalid span");
  }

  #else
  // Constructs an empty span whose data() == nullptr and size() == 0.
  // This overload only participates in overload resolution if
  // extent == 0 || extent == flatbuffers::dynamic_extent.
  // A dummy template argument N is need dependency for SFINAE.
  template<std::size_t N = 0,
    typename internal::is_span_convertible<element_type, Extent, element_type, (N - N)>::type = 0>
  FLATBUFFERS_CONSTEXPR_CPP11 span() FLATBUFFERS_NOEXCEPT : data_(nullptr),
                                                            count_(0) {
    static_assert(extent == 0 || extent == dynamic_extent, "invalid span");
  }

  // Constructs a span that is a view over the array arr; the resulting span
  // has size() == N and data() == std::data(arr). These overloads only
  // participate in overload resolution if
  // extent == std::dynamic_extent || N == extent is true and
  // std::remove_pointer_t<decltype(std::data(arr))>(*)[]
  // is convertible to element_type (*)[].
  template<std::size_t N,
    typename internal::is_span_convertible<element_type, Extent, element_type, N>::type = 0>
  FLATBUFFERS_CONSTEXPR_CPP11 span(element_type (&arr)[N]) FLATBUFFERS_NOEXCEPT
      : data_(arr), count_(N) {}

  template<class U, std::size_t N,
    typename internal::is_span_convertible<element_type, Extent, U, N>::type = 0>
  FLATBUFFERS_CONSTEXPR_CPP11 span(std::array<U, N> &arr) FLATBUFFERS_NOEXCEPT
     : data_(arr.data()), count_(N) {}

  //template<class U, std::size_t N,
  //  int = 0>
  //FLATBUFFERS_CONSTEXPR_CPP11 span(std::array<U, N> &arr) FLATBUFFERS_NOEXCEPT
  //   : data_(arr.data()), count_(N) {}

  template<class U, std::size_t N,
    typename internal::is_span_convertible<element_type, Extent, U, N>::type = 0>
  FLATBUFFERS_CONSTEXPR_CPP11 span(const std::array<U, N> &arr) FLATBUFFERS_NOEXCEPT
    : data_(arr.data()), count_(N) {}

  // Converting constructor from another span s;
  // the resulting span has size() == s.size() and data() == s.data().
  // This overload only participates in overload resolution
  // if extent == std::dynamic_extent || N == extent is true and U (*)[]
  // is convertible to element_type (*)[].
  template<class U, std::size_t N,
    typename internal::is_span_convertible<element_type, Extent, U, N>::type = 0>
  FLATBUFFERS_CONSTEXPR_CPP11 span(const flatbuffers::span<U, N> &s) FLATBUFFERS_NOEXCEPT
      : span(s.data(), s.size()) {
  }

  #endif  // !defined(FLATBUFFERS_SPAN_MINIMAL)

 private:
  // This is a naive implementation with 'count_' member even if (Extent != dynamic_extent).
  pointer const data_;
  size_type count_;
};
#endif  // defined(FLATBUFFERS_USE_STD_SPAN)

#if !defined(FLATBUFFERS_SPAN_MINIMAL)
template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<ElementType, Extent> make_span(ElementType(&arr)[Extent]) FLATBUFFERS_NOEXCEPT {
  return span<ElementType, Extent>(arr);
}

template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<const ElementType, Extent> make_span(const ElementType(&arr)[Extent]) FLATBUFFERS_NOEXCEPT {
  return span<const ElementType, Extent>(arr);
}

template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<ElementType, Extent> make_span(std::array<ElementType, Extent> &arr) FLATBUFFERS_NOEXCEPT {
  return span<ElementType, Extent>(arr);
}

template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<const ElementType, Extent> make_span(const std::array<ElementType, Extent> &arr) FLATBUFFERS_NOEXCEPT {
  return span<const ElementType, Extent>(arr);
}

template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<ElementType, dynamic_extent> make_span(ElementType *first, std::size_t count) FLATBUFFERS_NOEXCEPT {
  return span<ElementType, dynamic_extent>(first, count);
}

template<class ElementType, std::size_t Extent>
FLATBUFFERS_CONSTEXPR_CPP11
flatbuffers::span<const ElementType, dynamic_extent> make_span(const ElementType *first, std::size_t count) FLATBUFFERS_NOEXCEPT {
  return span<const ElementType, dynamic_extent>(first, count);
}
#endif // !defined(FLATBUFFERS_SPAN_MINIMAL)

}  // namespace flatbuffers

#endif  // FLATBUFFERS_STL_EMULATION_H_
