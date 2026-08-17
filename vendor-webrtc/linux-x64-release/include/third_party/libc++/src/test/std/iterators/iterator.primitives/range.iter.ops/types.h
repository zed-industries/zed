//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_ITERATORS_ITERATOR_PRIMITIVES_RANGE_ITER_OPS_TYPES_H
#define TEST_STD_ITERATORS_ITERATOR_PRIMITIVES_RANGE_ITER_OPS_TYPES_H

#include <cassert>
#include <cstddef>
#include <iterator>
#include <utility>

#include "test_iterators.h" // for the fallthrough base() function

class distance_apriori_sentinel {
public:
  distance_apriori_sentinel() = default;
  constexpr explicit distance_apriori_sentinel(std::ptrdiff_t const count) : count_(count) {}

  constexpr bool operator==(std::input_or_output_iterator auto const&) const {
    assert(false && "difference op should take precedence");
    return false;
  }

  friend constexpr std::ptrdiff_t operator-(std::input_or_output_iterator auto const&,
                                            distance_apriori_sentinel const y) {
    return -y.count_;
  }

  friend constexpr std::ptrdiff_t operator-(distance_apriori_sentinel const x,
                                            std::input_or_output_iterator auto const&) {
    return x.count_;
  }

private:
  std::ptrdiff_t count_ = 0;
};

// Sentinel type that can be assigned to an iterator. This is to test the cases where the
// various iterator operations use assignment instead of successive increments/decrements.
template <class It>
class assignable_sentinel {
public:
    explicit assignable_sentinel() = default;
    constexpr explicit assignable_sentinel(const It& it) : base_(base(it)) {}
    constexpr operator It() const { return It(base_); }
    constexpr bool operator==(const It& other) const { return base_ == base(other); }
    friend constexpr It base(const assignable_sentinel& s) { return It(s.base_); }
private:
    decltype(base(std::declval<It>())) base_;
};

template <class It>
assignable_sentinel(const It&) -> assignable_sentinel<It>;

#endif // TEST_STD_ITERATORS_ITERATOR_PRIMITIVES_RANGE_ITER_OPS_TYPES_H
