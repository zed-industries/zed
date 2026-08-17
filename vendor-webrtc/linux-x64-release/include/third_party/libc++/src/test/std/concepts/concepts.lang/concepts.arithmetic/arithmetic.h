//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_CONCEPTS_LANG_CONCEPTS_ARITHMETIC_H_
#define LIBCXX_TEST_CONCEPTS_LANG_CONCEPTS_ARITHMETIC_H_

#include <concepts>

// This overload should never be called. It exists solely to force subsumption.
template <std::integral I>
constexpr bool CheckSubsumption(I) {
  return false;
}

// clang-format off
template <std::integral I>
requires std::signed_integral<I> && (!std::unsigned_integral<I>)
constexpr bool CheckSubsumption(I) {
  return std::is_signed_v<I>;
}

template <std::integral I>
requires std::unsigned_integral<I> && (!std::signed_integral<I>)
constexpr bool CheckSubsumption(I) {
  return std::is_unsigned_v<I>;
}
// clang-format on

enum ClassicEnum { a, b, c };
enum class ScopedEnum { x, y, z };
struct EmptyStruct {};

#endif // LIBCXX_TEST_CONCEPTS_LANG_CONCEPTS_ARITHMETIC_H_
