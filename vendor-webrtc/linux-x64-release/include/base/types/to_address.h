// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_TO_ADDRESS_H_
#define BASE_TYPES_TO_ADDRESS_H_

#include <memory>
#include <type_traits>

// SFINAE-compatible wrapper for `std::to_address()`.
//
// The standard does not require `std::to_address()` to be SFINAE-compatible
// when code attempts instantiation with non-pointer-like types, and libstdc++'s
// implementation hard errors. For the sake of templated code that wants simple,
// unified handling, Chromium instead uses this wrapper, which provides that
// guarantee. This allows code to use "`to_address()` would be valid here" as a
// constraint to detect pointer-like types.
namespace base {

// Note that calling `std::to_address()` with a function pointer renders the
// program ill-formed.
template <typename T>
  requires(!std::is_function_v<T>)
constexpr T* to_address(T* p) noexcept {
  return p;
}

// These constraints cover the cases where `std::to_address()`'s fancy pointer
// overload is well-specified.
template <typename P>
  requires requires(const P& p) { std::pointer_traits<P>::to_address(p); } ||
           requires(const P& p) { p.operator->(); }
constexpr auto to_address(const P& p) noexcept {
  return std::to_address(p);
}

}  // namespace base

#endif  // BASE_TYPES_TO_ADDRESS_H_
