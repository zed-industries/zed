// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_EXPECTED_INTERNAL_H_
#define BASE_TYPES_EXPECTED_INTERNAL_H_

// IWYU pragma: private, include "base/types/expected.h"
#include <concepts>
#include <functional>
#include <type_traits>
#include <utility>
#include <variant>

#include "base/check.h"

// This header defines type traits and aliases used for the implementation of
// base::expected.
namespace base {

template <typename T>
class ok;

template <typename E>
class unexpected;

struct unexpect_t {
  explicit unexpect_t() = default;
};

// in-place construction of unexpected values
inline constexpr unexpect_t unexpect{};

template <typename T, typename E>
class expected;

namespace internal {

template <typename T>
inline constexpr bool UnderlyingIsOk = false;
template <typename T>
inline constexpr bool UnderlyingIsOk<ok<T>> = true;
template <typename T>
inline constexpr bool IsOk = UnderlyingIsOk<std::remove_cvref_t<T>>;

template <typename T>
inline constexpr bool UnderlyingIsUnexpected = false;
template <typename E>
inline constexpr bool UnderlyingIsUnexpected<unexpected<E>> = true;
template <typename T>
inline constexpr bool IsUnexpected =
    UnderlyingIsUnexpected<std::remove_cvref_t<T>>;

template <typename T>
inline constexpr bool UnderlyingIsExpected = false;
template <typename T, typename E>
inline constexpr bool UnderlyingIsExpected<expected<T, E>> = true;
template <typename T>
inline constexpr bool IsExpected = UnderlyingIsExpected<std::remove_cvref_t<T>>;

template <typename T, typename U>
inline constexpr bool IsConstructibleOrConvertible =
    std::is_constructible_v<T, U> || std::is_convertible_v<U, T>;

template <typename T, typename U>
inline constexpr bool IsAnyConstructibleOrConvertible =
    IsConstructibleOrConvertible<T, U&> ||
    IsConstructibleOrConvertible<T, U&&> ||
    IsConstructibleOrConvertible<T, const U&> ||
    IsConstructibleOrConvertible<T, const U&&>;

// Checks whether a given expected<U, G> can be converted into another
// expected<T, E>. Used inside expected's conversion constructors. UF and GF are
// the forwarded versions of U and G, e.g. UF is const U& for the converting
// copy constructor and U for the converting move constructor. Similarly for GF.
// ExUG is used for convenience, and not expected to be passed explicitly.
// See https://eel.is/c++draft/expected#lib:expected,constructor___
template <typename T,
          typename E,
          typename UF,
          typename GF,
          typename ExUG =
              expected<std::remove_cvref_t<UF>, std::remove_cvref_t<GF>>>
inline constexpr bool IsValidConversion =
    std::is_constructible_v<T, UF> && std::is_constructible_v<E, GF> &&
    !IsAnyConstructibleOrConvertible<T, ExUG> &&
    !IsAnyConstructibleOrConvertible<unexpected<E>, ExUG>;

// Checks whether a given expected<U, G> can be converted into another
// expected<T, E> when T is a void type. Used inside expected<void>'s conversion
// constructors. GF is the forwarded versions of G, e.g. GF is const G& for the
// converting copy constructor and G for the converting move constructor. ExUG
// is used for convenience, and not expected to be passed explicitly. See
// https://eel.is/c++draft/expected#lib:expected%3cvoid%3e,constructor___
template <typename E,
          typename U,
          typename GF,
          typename ExUG = expected<U, std::remove_cvref_t<GF>>>
inline constexpr bool IsValidVoidConversion =
    std::is_void_v<U> && std::is_constructible_v<E, GF> &&
    !IsAnyConstructibleOrConvertible<unexpected<E>, ExUG>;

// Checks whether expected<T, E> can be constructed from a value of type U.
template <typename T, typename E, typename U>
inline constexpr bool IsValidValueConstruction =
    std::is_constructible_v<T, U> &&
    !std::is_same_v<std::remove_cvref_t<U>, std::in_place_t> &&
    !std::is_same_v<std::remove_cvref_t<U>, expected<T, E>> && !IsOk<U> &&
    !IsUnexpected<U>;

template <typename T, typename U>
inline constexpr bool IsOkValueConstruction =
    !std::is_same_v<std::remove_cvref_t<U>, ok<T>> &&
    !std::is_same_v<std::remove_cvref_t<U>, std::in_place_t> &&
    std::is_constructible_v<T, U>;

template <typename T, typename U>
inline constexpr bool IsUnexpectedValueConstruction =
    !std::is_same_v<std::remove_cvref_t<U>, unexpected<T>> &&
    !std::is_same_v<std::remove_cvref_t<U>, std::in_place_t> &&
    std::is_constructible_v<T, U>;

template <typename T, typename E, typename U>
inline constexpr bool IsValueAssignment =
    !std::is_same_v<expected<T, E>, std::remove_cvref_t<U>> && !IsOk<U> &&
    !IsUnexpected<U> && std::is_constructible_v<T, U> &&
    std::is_assignable_v<T&, U>;

template <typename T, typename E>
class ExpectedImpl {
 public:
  static constexpr size_t kValIdx = 1;
  static constexpr size_t kErrIdx = 2;
  static constexpr std::in_place_index_t<1> kValTag{};
  static constexpr std::in_place_index_t<2> kErrTag{};

  template <typename U, typename G>
  friend class ExpectedImpl;

  constexpr ExpectedImpl() noexcept
    requires(std::default_initializable<T>)
      : data_(kValTag) {}
  constexpr ExpectedImpl(const ExpectedImpl& rhs) noexcept : data_(rhs.data_) {
    CHECK(!rhs.is_moved_from());
  }
  constexpr ExpectedImpl(ExpectedImpl&& rhs) noexcept
      : data_(std::move(rhs.data_)) {
    CHECK(!rhs.is_moved_from());
    rhs.set_is_moved_from();
  }

  template <typename U, typename G>
  constexpr explicit ExpectedImpl(const ExpectedImpl<U, G>& rhs) noexcept {
    if (rhs.has_value()) {
      emplace_value(rhs.value());
    } else {
      emplace_error(rhs.error());
    }
  }

  template <typename U, typename G>
  constexpr explicit ExpectedImpl(ExpectedImpl<U, G>&& rhs) noexcept {
    if (rhs.has_value()) {
      emplace_value(std::move(rhs.value()));
    } else {
      emplace_error(std::move(rhs.error()));
    }
    rhs.set_is_moved_from();
  }

  template <typename... Args>
  constexpr explicit ExpectedImpl(decltype(kValTag), Args&&... args) noexcept
      : data_(kValTag, std::forward<Args>(args)...) {}

  template <typename U, typename... Args>
  constexpr explicit ExpectedImpl(decltype(kValTag),
                                  std::initializer_list<U> il,
                                  Args&&... args) noexcept
      : data_(kValTag, il, std::forward<Args>(args)...) {}

  template <typename... Args>
  constexpr explicit ExpectedImpl(decltype(kErrTag), Args&&... args) noexcept
      : data_(kErrTag, std::forward<Args>(args)...) {}

  template <typename U, typename... Args>
  constexpr explicit ExpectedImpl(decltype(kErrTag),
                                  std::initializer_list<U> il,
                                  Args&&... args) noexcept
      : data_(kErrTag, il, std::forward<Args>(args)...) {}

  constexpr ExpectedImpl& operator=(const ExpectedImpl& rhs) noexcept {
    CHECK(!rhs.is_moved_from());
    data_ = rhs.data_;
    return *this;
  }

  constexpr ExpectedImpl& operator=(ExpectedImpl&& rhs) noexcept {
    CHECK(!rhs.is_moved_from());
    data_ = std::move(rhs.data_);
    rhs.set_is_moved_from();
    return *this;
  }

  template <typename... Args>
  constexpr T& emplace_value(Args&&... args) noexcept {
    return data_.template emplace<kValIdx>(std::forward<Args>(args)...);
  }

  template <typename U, typename... Args>
  constexpr T& emplace_value(std::initializer_list<U> il,
                             Args&&... args) noexcept {
    return data_.template emplace<kValIdx>(il, std::forward<Args>(args)...);
  }

  template <typename... Args>
  constexpr E& emplace_error(Args&&... args) noexcept {
    return data_.template emplace<kErrIdx>(std::forward<Args>(args)...);
  }

  template <typename U, typename... Args>
  constexpr E& emplace_error(std::initializer_list<U> il,
                             Args&&... args) noexcept {
    return data_.template emplace<kErrIdx>(il, std::forward<Args>(args)...);
  }

  void swap(ExpectedImpl& rhs) noexcept {
    CHECK(!is_moved_from());
    CHECK(!rhs.is_moved_from());
    data_.swap(rhs.data_);
  }

  constexpr bool has_value() const noexcept {
    CHECK(!is_moved_from());
    return data_.index() == kValIdx;
  }

  // Note: No `CHECK()` here and below, since std::get already checks that
  // the passed in index is active.
  constexpr T& value() noexcept { return std::get<kValIdx>(data_); }
  constexpr const T& value() const noexcept { return std::get<kValIdx>(data_); }

  constexpr E& error() noexcept { return std::get<kErrIdx>(data_); }
  constexpr const E& error() const noexcept { return std::get<kErrIdx>(data_); }

 private:
  static constexpr size_t kNulIdx = 0;
  static_assert(kNulIdx != kValIdx);
  static_assert(kNulIdx != kErrIdx);

  constexpr bool is_moved_from() const noexcept {
    return data_.index() == kNulIdx;
  }

  constexpr void set_is_moved_from() noexcept {
    data_.template emplace<kNulIdx>();
  }

  std::variant<std::monostate, T, E> data_;
};

template <typename Exp, typename F>
constexpr auto AndThen(Exp&& exp, F&& f) noexcept {
  using T = std::remove_cvref_t<decltype(exp.value())>;
  using E = std::remove_cvref_t<decltype(exp.error())>;

  auto invoke_f = [&]() -> decltype(auto) {
    if constexpr (!std::is_void_v<T>) {
      return std::invoke(std::forward<F>(f), std::forward<Exp>(exp).value());
    } else {
      return std::invoke(std::forward<F>(f));
    }
  };

  using U = decltype(invoke_f());
  static_assert(internal::IsExpected<U>,
                "expected<T, E>::and_then: Result of f() must be a "
                "specialization of expected");
  static_assert(
      std::is_same_v<typename U::error_type, E>,
      "expected<T, E>::and_then: Result of f() must have E as error_type");

  return exp.has_value() ? invoke_f()
                         : U(unexpect, std::forward<Exp>(exp).error());
}

template <typename Exp, typename F>
constexpr auto OrElse(Exp&& exp, F&& f) noexcept {
  using T = std::remove_cvref_t<decltype(exp.value())>;
  using G = std::invoke_result_t<F, decltype(std::forward<Exp>(exp).error())>;

  static_assert(internal::IsExpected<G>,
                "expected<T, E>::or_else: Result of f() must be a "
                "specialization of expected");
  static_assert(
      std::is_same_v<typename G::value_type, T>,
      "expected<T, E>::or_else: Result of f() must have T as value_type");

  if (!exp.has_value()) {
    return std::invoke(std::forward<F>(f), std::forward<Exp>(exp).error());
  }

  if constexpr (!std::is_void_v<T>) {
    return G(std::in_place, std::forward<Exp>(exp).value());
  } else {
    return G();
  }
}

template <typename Exp, typename F>
constexpr auto Transform(Exp&& exp, F&& f) noexcept {
  using T = std::remove_cvref_t<decltype(exp.value())>;
  using E = std::remove_cvref_t<decltype(exp.error())>;

  auto invoke_f = [&]() -> decltype(auto) {
    if constexpr (!std::is_void_v<T>) {
      return std::invoke(std::forward<F>(f), std::forward<Exp>(exp).value());
    } else {
      return std::invoke(std::forward<F>(f));
    }
  };

  using U = std::remove_cv_t<decltype(invoke_f())>;
  if constexpr (!std::is_void_v<U>) {
    static_assert(!std::is_array_v<U>,
                  "expected<T, E>::transform: Result of f() should "
                  "not be an Array");
    static_assert(!std::is_same_v<U, std::in_place_t>,
                  "expected<T, E>::transform: Result of f() should "
                  "not be std::in_place_t");
    static_assert(!std::is_same_v<U, unexpect_t>,
                  "expected<T, E>::transform: Result of f() should "
                  "not be unexpect_t");
    static_assert(!internal::IsOk<U>,
                  "expected<T, E>::transform: Result of f() should "
                  "not be a specialization of ok");
    static_assert(!internal::IsUnexpected<U>,
                  "expected<T, E>::transform: Result of f() should "
                  "not be a specialization of unexpected");
    static_assert(std::is_object_v<U>,
                  "expected<T, E>::transform: Result of f() should be "
                  "an object type");
  }

  if (!exp.has_value()) {
    return expected<U, E>(unexpect, std::forward<Exp>(exp).error());
  }

  if constexpr (!std::is_void_v<U>) {
    return expected<U, E>(std::in_place, invoke_f());
  } else {
    invoke_f();
    return expected<U, E>();
  }
}

template <typename Exp, typename F>
constexpr auto TransformError(Exp&& exp, F&& f) noexcept {
  using T = std::remove_cvref_t<decltype(exp.value())>;
  using G = std::remove_cv_t<
      std::invoke_result_t<F, decltype(std::forward<Exp>(exp).error())>>;

  static_assert(
      !std::is_array_v<G>,
      "expected<T, E>::transform_error: Result of f() should not be an Array");
  static_assert(!std::is_same_v<G, std::in_place_t>,
                "expected<T, E>::transform_error: Result of f() should not be "
                "std::in_place_t");
  static_assert(!std::is_same_v<G, unexpect_t>,
                "expected<T, E>::transform_error: Result of f() should not be "
                "unexpect_t");
  static_assert(!internal::IsOk<G>,
                "expected<T, E>::transform_error: Result of f() should not be "
                "a specialization of ok");
  static_assert(!internal::IsUnexpected<G>,
                "expected<T, E>::transform_error: Result of f() should not be "
                "a specialization of unexpected");
  static_assert(std::is_object_v<G>,
                "expected<T, E>::transform_error: Result of f() should be an "
                "object type");

  if (!exp.has_value()) {
    return expected<T, G>(
        unexpect,
        std::invoke(std::forward<F>(f), std::forward<Exp>(exp).error()));
  }

  if constexpr (std::is_void_v<T>) {
    return expected<T, G>();
  } else {
    return expected<T, G>(std::in_place, std::forward<Exp>(exp).value());
  }
}

}  // namespace internal

}  // namespace base

#endif  // BASE_TYPES_EXPECTED_INTERNAL_H_
