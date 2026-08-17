// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines some bit utilities.

#ifndef BASE_BITS_H_
#define BASE_BITS_H_

#include <stddef.h>
#include <stdint.h>

#include <bit>
#include <concepts>
#include <type_traits>

#include "base/check.h"

namespace base::bits {

// Bit functions in <bit> are restricted to a specific set of types of unsigned
// integer; restrict functions in this file that are related to those in that
// header to match for consistency.
template <typename T>
concept UnsignedInteger =
    std::unsigned_integral<T> && !std::same_as<T, bool> &&
    !std::same_as<T, char> && !std::same_as<T, char8_t> &&
    !std::same_as<T, char16_t> && !std::same_as<T, char32_t> &&
    !std::same_as<T, wchar_t>;

// We want to migrate all users of these functions to use the unsigned type
// versions of the functions, but until they are all moved over, create a
// concept that captures all the types that must be supported for compatibility
// but that we want to remove.
//
// TODO(crbug.com/40256225): Switch uses to supported functions and
// remove.
template <typename T>
concept SignedIntegerDeprecatedDoNotUse =
    std::integral<T> && !UnsignedInteger<T>;

// Round down |size| to a multiple of alignment, which must be a power of two.
template <typename T>
  requires UnsignedInteger<T>
inline constexpr T AlignDown(T size, T alignment) {
  DCHECK(std::has_single_bit(alignment));
  return size & ~(alignment - 1);
}

// Round down |size| to a multiple of alignment, which must be a power of two.
// DEPRECATED; use the UnsignedInteger version.
//
// TODO(crbug.com/40256225): Switch uses and remove.
template <typename T>
inline constexpr auto AlignDownDeprecatedDoNotUse(T size, T alignment) {
  using U = std::make_unsigned_t<T>;
  DCHECK(std::has_single_bit(static_cast<U>(alignment)));
  return static_cast<U>(size) & ~static_cast<U>(alignment - 1);
}

// Move |ptr| back to the previous multiple of alignment, which must be a power
// of two. Defined for types where sizeof(T) is one byte.
template <typename T>
  requires(sizeof(T) == 1)
inline T* AlignDown(T* ptr, uintptr_t alignment) {
  return reinterpret_cast<T*>(
      AlignDown(reinterpret_cast<uintptr_t>(ptr), alignment));
}

// Round up |size| to a multiple of alignment, which must be a power of two.
template <typename T>
  requires UnsignedInteger<T>
inline constexpr T AlignUp(T size, T alignment) {
  DCHECK(std::has_single_bit(alignment));
  return (size + alignment - 1) & ~(alignment - 1);
}

// Round up |size| to a multiple of alignment, which must be a power of two.
// DEPRECATED; use the UnsignedInteger version.
//
// TODO(crbug.com/40256225): Switch uses and remove.
template <typename T>
  requires SignedIntegerDeprecatedDoNotUse<T>
inline constexpr T AlignUpDeprecatedDoNotUse(T size, T alignment) {
  using U = std::make_unsigned_t<T>;
  DCHECK(std::has_single_bit(static_cast<U>(alignment)));
  return static_cast<U>(size + alignment - 1) & ~static_cast<U>(alignment - 1);
}

// Advance |ptr| to the next multiple of alignment, which must be a power of
// two. Defined for types where sizeof(T) is one byte.
template <typename T>
  requires(sizeof(T) == 1)
inline T* AlignUp(T* ptr, uintptr_t alignment) {
  return reinterpret_cast<T*>(
      AlignUp(reinterpret_cast<uintptr_t>(ptr), alignment));
}

// Returns the integer i such as 2^i <= n < 2^(i+1).
//
// A common use for this function is to measure the number of bits required to
// contain a value; for that case use std::bit_width().
//
// A common use for this function is to take its result and use it to left-shift
// a bit; instead of doing so, use std::bit_floor().
constexpr int Log2Floor(uint32_t n) {
  return 31 - std::countl_zero(n);
}

// Returns the integer i such as 2^(i-1) < n <= 2^i.
//
// A common use for this function is to measure the number of bits required to
// contain a value; for that case use std::bit_width().
//
// A common use for this function is to take its result and use it to left-shift
// a bit; instead of doing so, use std::bit_ceil().
constexpr int Log2Ceiling(uint32_t n) {
  // When n == 0, we want the function to return -1.
  // When n == 0, (n - 1) will underflow to 0xFFFFFFFF, which is
  // why the statement below starts with (n ? 32 : -1).
  return (n ? 32 : -1) - std::countl_zero(n - 1);
}

// Returns a value of type T with a single bit set in the left-most position.
// Can be used instead of manually shifting a 1 to the left. Unlike the other
// functions in this file, usable for any integral type.
template <typename T>
  requires std::integral<T>
constexpr T LeftmostBit() {
  T one(1u);
  return one << (8 * sizeof(T) - 1);
}

}  // namespace base::bits

#endif  // BASE_BITS_H_
