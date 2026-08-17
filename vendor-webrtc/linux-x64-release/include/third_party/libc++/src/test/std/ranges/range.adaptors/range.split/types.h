//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_SPLIT_TYPES_H
#define TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_SPLIT_TYPES_H

#include <functional>
#include <ranges>

#include "test_macros.h"
#include "test_iterators.h"
#include "test_range.h"

template <class Derived>
struct ForwardIterBase {
  using iterator_concept = std::forward_iterator_tag;
  using value_type       = int;
  using difference_type  = std::intptr_t;

  constexpr int operator*() const { return 5; }

  constexpr Derived& operator++() { return static_cast<Derived&>(*this); }
  constexpr Derived operator++(int) { return {}; }

  friend constexpr bool operator==(const ForwardIterBase&, const ForwardIterBase&) { return true; };
};

#endif // TEST_STD_RANGES_RANGE_ADAPTORS_RANGE_SPLIT_TYPES_H
