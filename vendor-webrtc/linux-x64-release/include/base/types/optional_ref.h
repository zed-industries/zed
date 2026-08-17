// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_OPTIONAL_REF_H_
#define BASE_TYPES_OPTIONAL_REF_H_

#include <concepts>
#include <memory>
#include <optional>
#include <type_traits>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"

namespace base {

// `optional_ref<T>` is similar to `std::optional<T>`, except it does not own
// the underlying value.
//
// When passing an optional parameter, prefer `optional_ref` to `const
// std::optional<T>&` as the latter often results in hidden copies due to
// implicit conversions, e.g. given the function:
//
//   void TakesOptionalString(const std::optional<std::string>& str);
//
// And a call to that looks like:
//
//   std::string s = "Hello world!";
//   TakesOptionalString(s);
//
// This copies `s` into a temporary `std::optional<std::string>` in order to
// call `TakesOptionalString()`.
//
// The C++ style guide recommends using `const T*` instead of `const
// std::optional<T>&` when `T` would normally be passed by reference. However
// `const T*` is not always a good substitute because:
//
// - `const T*` disallows the use of temporaries, since it is not possible to
//   take the address of a temporary.
// - additional boilerplate (e.g. `OptionalToPtr`) is required to pass an
//   `std::optional<T>` to a `const T*` function parameter.
//
// Like `span<T>`, mutability of `optional_ref<T>` is controlled by the template
// argument `T`; e.g. `optional_ref<const int>` only allows const access to the
// referenced `int` value.
//
// Thus, `optional_ref<const T>` can be constructed from:
// - `std::nullopt`
// - `const T*` or `T*`
// - `const T&` or `T&`
// ` `const std::optional<T>&` or `std::optional<T>&`
//
// While `optional_ref<T>` can only be constructed from:
// - `std::nullopt`
// - `T*`
// - `T&`
// - `std::optional<T>&`
//
// Implicit conversions are disallowed, e.g. this will not compile:
//
//   [](base::optional_ref<std::string> s) {}("Hello world!");
//
// This restriction may be relaxed in the future if it proves too onerous.
//
// `optional_ref<T>` is lightweight and should be passed by value. It is copy
// constructible but not copy assignable, to reduce the risk of lifetime bugs.
template <typename T>
class optional_ref {
 private:
  // Disallowed because `std::optional` does not allow its template argument to
  // be a reference type.
  static_assert(!std::is_reference_v<T>,
                "T must not be a reference type (use a pointer?)");

  // Both checks are important here, as:
  // - optional_ref does not allow silent implicit conversions between types,
  //   so the decayed types must match exactly.
  // - unless the types differ only in const qualification, and T is at least as
  //   const-qualified as the incoming type U.
  template <typename U>
  static constexpr bool IsCompatibleV =
      std::is_same_v<std::decay_t<T>, std::decay_t<U>> &&
      std::is_convertible_v<U*, T*>;

 public:
  using value_type = T;

  // Constructs an empty `optional_ref`.
  constexpr optional_ref() = default;
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(std::nullopt_t) {}

  // Constructs an `optional_ref` from an `std::optional`; the resulting
  // `optional_ref` is empty iff `o` is empty.
  //
  // Note: when constructing from a const reference, `optional_ref`'s template
  // argument must be const-qualified as well.
  // Note 2: avoiding direct use of `T` prevents implicit conversions.
  template <typename U>
    requires(std::is_const_v<T> && IsCompatibleV<U>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(const std::optional<U>& o LIFETIME_BOUND)
      : ptr_(o ? &*o : nullptr) {}
  template <typename U>
    requires(IsCompatibleV<U>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(std::optional<U>& o LIFETIME_BOUND)
      : ptr_(o ? &*o : nullptr) {}

  // Constructs an `optional_ref` from a pointer; the resulting `optional_ref`
  // is empty iff `p` is null.
  //
  // Note: when constructing from a const pointer, `optional_ref`'s template
  // argument must be const-qualified as well.
  // Note 2: avoiding direct use of `T` prevents implicit conversions.
  template <typename U>
    requires(IsCompatibleV<U>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(U* p LIFETIME_BOUND) : ptr_(p) {}

  // Constructs an `optional_ref` from a reference; the resulting `optional_ref`
  // is never empty.
  //
  // Note: when constructing from a const reference, `optional_ref`'s template
  // argument must be const-qualified as well.
  // Note 2: avoiding direct use of `T` prevents implicit conversions.
  template <typename U>
    requires(IsCompatibleV<const U>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(const U& r LIFETIME_BOUND) : ptr_(std::addressof(r)) {}
  template <typename U>
    requires(IsCompatibleV<U>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr optional_ref(U& r LIFETIME_BOUND) : ptr_(std::addressof(r)) {}

  // An empty `optional_ref` must be constructed with `std::nullopt`, not
  // `nullptr`. Otherwise, `optional_ref<T*>` constructed with `nullptr` would
  // be ambiguous: is it empty or is it engaged with a value of `nullptr`?
  constexpr optional_ref(std::nullptr_t) = delete;

  // Constructs a `optional_ref<const T>` from a `optional_ref<T>`. Conversions
  // in the reverse direction are disallowed.
  // NOLINTNEXTLINE(google-explicit-constructor)
  template <typename U = std::remove_const<T>>
    requires(std::is_const_v<T>)
  constexpr optional_ref(optional_ref<U> rhs) : ptr_(rhs.as_ptr()) {}

  // Copy construction is allowed to make it possible to pass `optional_ref`s to
  // another call. However, assignment is disallowed, as it makes it easy to
  // violate lifetime bounds. Use `CopyAsOptional()` if an `optional_ref` needs
  // to be persisted beyond the scope of a function call.
  constexpr optional_ref(const optional_ref&) = default;
  optional_ref& operator=(const optional_ref&) = delete;

  // CHECKs if the `optional_ref` is empty.
  constexpr T* operator->() const {
    CHECK(ptr_);
    return ptr_;
  }

  // CHECKs if the `optional_ref` is empty.
  constexpr T& operator*() const {
    CHECK(ptr_);
    return *ptr_;
  }

  // Returns `true` iff the `optional_ref` is non-empty.
  constexpr bool has_value() const { return ptr_; }
  constexpr explicit operator bool() const { return has_value(); }

  // CHECKs if the `optional_ref` is empty.
  constexpr T& value() const {
    CHECK(ptr_);
    return *ptr_;
  }

  // Convenience method for turning an `optional_ref` into a pointer.
  constexpr T* as_ptr() const { return ptr_; }

  // Convenience method for turning a non-owning `optional_ref` into an owning
  // `std::optional`. Incurs a copy; useful when saving an `optional_ref`
  // function parameter as a field, et cetera.
  template <typename U = std::decay_t<T>>
    requires(std::constructible_from<U, T>)
  constexpr std::optional<U> CopyAsOptional() const {
    return ptr_ ? std::optional<U>(*ptr_) : std::nullopt;
  }

  // Equality comparison operator against `optional_ref<U>`.
  template <typename U>
    requires std::equality_comparable_with<T, U>
  constexpr bool operator==(optional_ref<U> u) const {
    return (!has_value() && !u.has_value()) ||
           (has_value() && u.has_value() && value() == u.value());
  }

  // Equality comparison operator against `T`.
  constexpr bool operator==(const T& t) const
    requires(std::equality_comparable<T>)
  {
    return has_value() && value() == t;
  }

 private:
  raw_ptr<T> const ptr_ = nullptr;
};

template <typename T>
optional_ref(const T&) -> optional_ref<const T>;
template <typename T>
optional_ref(T&) -> optional_ref<T>;

template <typename T>
optional_ref(const std::optional<T>&) -> optional_ref<const T>;
template <typename T>
optional_ref(std::optional<T>&) -> optional_ref<T>;

template <typename T>
optional_ref(T*) -> optional_ref<T>;

template <typename T>
constexpr bool operator==(std::nullopt_t, optional_ref<T> x) {
  return !x.has_value();
}

template <typename T>
constexpr bool operator==(optional_ref<T> x, std::nullopt_t) {
  return !x.has_value();
}

}  // namespace base

#endif  // BASE_TYPES_OPTIONAL_REF_H_
