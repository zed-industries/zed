//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef CHECK_CONSECUTIVE_H
#define CHECK_CONSECUTIVE_H

// <unordered_multiset>
// <unordered_multimap>

#include <cassert>
#include <set>
#include <stddef.h>

// Check consecutive equal values in an unordered_multiset iterator
template <typename Iter>
void CheckConsecutiveValues(Iter pos, Iter end, typename Iter::value_type value, std::size_t count) {
  for (std::size_t i = 0; i < count; ++i) {
    assert(pos != end);
    assert(*pos == value);
    ++pos;
  }
  assert(pos == end || *pos != value);
}

// Check consecutive equal keys in an unordered_multimap iterator
template <typename Iter>
void CheckConsecutiveKeys(Iter pos,
                          Iter end,
                          typename Iter::value_type::first_type key,
                          std::multiset<typename Iter::value_type::second_type>& values) {
  while (!values.empty()) {
    assert(pos != end);
    assert(pos->first == key);
    assert(values.find(pos->second) != values.end());
    values.erase(values.find(pos->second));
    ++pos;
  }
  assert(pos == end || pos->first != key);
}

#endif
