// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_OPTIONAL_UTIL_H_
#define BASE_TYPES_OPTIONAL_UTIL_H_

#include <concepts>
#include <functional>
#include <optional>
#include <utility>

#include "base/compiler_specific.h"
#include "base/types/expected.h"

namespace base {

// Helper for converting an `std::optional<T>` to a pointer suitable for
// passing as a function argument (alternatively, consider using
// `base::optional_ref`):
//
// void MaybeProcessData(const std::string* optional_data);
//
// class Example {
//  public:
//   void DoSomething() {
//     MaybeProcessData(base::OptionalToPtr(data_));
//   }
//
//  private:
//   std::optional<std::string> data_;
// };
//
// Rationale: per the C++ style guide, if `T` would normally be passed by
// reference, the optional version should be passed as `T*`, and *not* as
// `const std::optional<T>&`. Passing as `const std::optional<T>&` leads to
// implicit constructions and copies, e.g.:
//
// // BAD: a caller passing a `std::string` implicitly copies the entire string
// // to construct a temporary `std::optional<std::string>` to use for the
// // function argument.
// void BadMaybeProcessData(const std::optional<std::string>& optional_data);
//
// For more background, see https://abseil.io/tips/163. Also see
// `base/types/optional_ref.h` for an alternative approach to
// `const std::optional<T>&` that does not require the use of raw pointers.
template <class T>
const T* OptionalToPtr(const std::optional<T>& optional LIFETIME_BOUND) {
  return optional.has_value() ? &optional.value() : nullptr;
}

template <class T>
T* OptionalToPtr(std::optional<T>& optional LIFETIME_BOUND) {
  return optional.has_value() ? &optional.value() : nullptr;
}

// Helper for creating an `std::optional<T>` from a `T*` which may be null.
//
// This copies `T` into the `std::optional`. When you have control over the
// function that accepts the optional, and it currently expects a
// `std::optional<T>&` or `const std::optional<T>&`, consider changing it to
// accept a `base::optional_ref<T>` / `base::optional_ref<const T>` instead,
// which can be constructed from `T*` without copying.
template <class T>
std::optional<T> OptionalFromPtr(const T* value) {
  return value ? std::optional<T>(*value) : std::nullopt;
}

// Helper for creating a `base::expected<U, F>` from an `std::optional<T>` and
// an error of type E, where T is convertible to U and E is convertible to F. If
// `opt` contains a value, this copies it into the `base::expected`, otherwise
// it moves `err` in.
template <class T, class E, class U = T, class F = E>
base::expected<U, F> OptionalToExpected(const std::optional<T>& opt, E&& err)
  requires(std::convertible_to<T, U> && std::copyable<T> &&
           std::convertible_to<E, F> && std::movable<E>)
{
  if (opt.has_value()) {
    return base::ok(opt.value());
  }
  return base::unexpected(std::move(err));
}

// As above, but copies `err` into the `base:expected` if `opt` doesn't contain
// a value.
template <class T, class E, class U = T, class F = E>
base::expected<U, F> OptionalToExpected(const std::optional<T>& opt,
                                        const E& err)
  requires(std::convertible_to<T, U> && std::copyable<T> &&
           std::convertible_to<E, F> && std::copyable<E>)
{
  if (opt.has_value()) {
    return base::ok(opt.value());
  }
  return base::unexpected(err);
}

// Helper for creating an `std::optional<U>` from a `base::expected<T, E>`,
// where T is convertible to U. If `exp` contains a value, this copies it into
// the `std::optional`, otherwise it returns std::nullopt.
template <class T, class E, class U = T>
std::optional<U> OptionalFromExpected(const base::expected<T, E>& exp)
  requires(std::convertible_to<T, U>)
{
  if (exp.has_value()) {
    return std::optional(exp.value());
  }
  return std::nullopt;
}

// Unwrap a `std::optional<T>` if it holds a value, and set `out` to the value
// in the optional. Returns true if the optional held a value which means `out`
// was assigned to. Returns false if the optional was empty, in which casse
// `out` will be unchanged (and the `proj` function will not be called).
//
// If a `proj` function is provided, it can modify the value in the optional,
// and `out` will instead be set to the value returned from the `proj` function.
// The `proj` may return any value or type, as long as its type matches or is
// assignable to `out`.
//
// If the optional is moved to this function as an rvalue, the unwrapped value
// will also be moved to assign to `out` as an rvalue (or moved into the `proj`
// function argument).
//
// # Examples
// Simple usage that unwraps the inner value into a variable or early outs:
// ```
// void maybe_do_stuff(std::optional<int> o) {
//   int val = 0;
//   if (!OptionalUnwrapTo(o, val)) {
//     return;
//   }
//   do_stuff(val);
// }
// ```
//
// Unwraps the inner value and converts it to a different type:
// ```
// void maybe_do_stuff(std::optional<int> o) {
//   MyType val;
//   if (!OptionalUnwrapTo(o, val, [](int i) { return MyType::FromId(i); })) {
//     return;
//   }
//   do_stuff(val);
// }
// ```
template <class T, class O, class P = std::identity>
  requires(std::invocable<P, const T&>) &&
          requires(O& out, std::invoke_result_t<P, const T&> val) { out = val; }
bool OptionalUnwrapTo(const std::optional<T>& optional, O& out, P proj = {}) {
  if (optional) {
    // Note: `proj` is received by value not by ref (unlike similar std
    // functions) as we see a large binary size impact from the latter. So we
    // unconditionally move it here while invoking.
    out = std::invoke(std::move(proj), optional.value());
    return true;
  }
  return false;
}

template <class T, class O, class P = std::identity>
  requires(std::invocable<P, T &&>) &&
          requires(O& out, std::invoke_result_t<P, T&&> val) { out = val; }
bool OptionalUnwrapTo(std::optional<T>&& optional, O& out, P proj = {}) {
  if (optional) {
    // Note: `proj` is received by value not by ref (unlike similar std
    // functions) as we see a large binary size impact from the latter. So we
    // unconditionally move it here while invoking.
    out = std::invoke(std::move(proj), std::move(optional).value());
    return true;
  }
  return false;
}

}  // namespace base

#endif  // BASE_TYPES_OPTIONAL_UTIL_H_
