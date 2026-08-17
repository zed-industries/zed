//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_TYPES_H
#define TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_TYPES_H

#include <concepts>

struct EqualityComparable {
  constexpr EqualityComparable(int value) : value_{value} {};

  friend constexpr bool operator==(const EqualityComparable&, const EqualityComparable&) noexcept = default;

  int value_;
};

static_assert(std::equality_comparable<EqualityComparable>);
static_assert(EqualityComparable{94} == EqualityComparable{94});
static_assert(EqualityComparable{94} != EqualityComparable{82});

struct NonComparable {};

static_assert(!std::three_way_comparable<NonComparable>);

#endif // TEST_STD_FUNCTIONOBJECTS_REFWRAP_HELPER_TYPES_H
