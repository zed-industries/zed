//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_CONTAINER_ADAPTORS_FLAT_SET_HELPERS_H
#define TEST_STD_CONTAINERS_CONTAINER_ADAPTORS_FLAT_SET_HELPERS_H

#include <algorithm>
#include <cassert>
#include <string>
#include <vector>
#include <flat_set>

#include "../flat_helpers.h"
#include "test_allocator.h"
#include "test_macros.h"

template <class... Args>
void check_invariant(const std::flat_set<Args...>& m) {
  assert(std::is_sorted(m.begin(), m.end(), m.key_comp()));
  auto key_equal = [&](const auto& x, const auto& y) {
    const auto& c = m.key_comp();
    return !c(x, y) && !c(y, x);
  };
  assert(std::adjacent_find(m.begin(), m.end(), key_equal) == m.end());
}

template <class F>
void test_emplace_exception_guarantee([[maybe_unused]] F&& emplace_function) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  using C = TransparentComparator;
  {
    // Throw on emplace the key, and underlying has strong exception guarantee
    using KeyContainer = std::vector<int, test_allocator<int>>;
    using M            = std::flat_set<int, C, KeyContainer>;

    LIBCPP_STATIC_ASSERT(std::__container_traits<KeyContainer>::__emplacement_has_strong_exception_safety_guarantee);

    test_allocator_statistics stats;

    KeyContainer a({1, 2, 3, 4}, test_allocator<int>{&stats});
    [[maybe_unused]] auto expected_keys = a;
    M m(std::sorted_unique, std::move(a));

    stats.throw_after = 1;
    try {
      emplace_function(m, 0);
      assert(false);
    } catch (const std::bad_alloc&) {
      check_invariant(m);
      // In libc++, the flat_set is unchanged
      LIBCPP_ASSERT(m.size() == 4);
      LIBCPP_ASSERT(std::ranges::equal(m, expected_keys));
    }
  }
  {
    // Throw on emplace the key, and underlying has no strong exception guarantee
    using KeyContainer = EmplaceUnsafeContainer<int>;
    using M            = std::flat_set<int, C, KeyContainer>;

    LIBCPP_STATIC_ASSERT(!std::__container_traits<KeyContainer>::__emplacement_has_strong_exception_safety_guarantee);
    KeyContainer a = {1, 2, 3, 4};
    M m(std::sorted_unique, std::move(a));
    try {
      emplace_function(m, 0);
      assert(false);
    } catch (int) {
      check_invariant(m);
      // In libc++, the flat_set is cleared
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
#endif
}

template <class F>
void test_insert_range_exception_guarantee([[maybe_unused]] F&& insert_function) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  using KeyContainer = EmplaceUnsafeContainer<int>;
  using M            = std::flat_set<int, std::ranges::less, KeyContainer>;
  test_allocator_statistics stats;
  KeyContainer a{1, 2, 3, 4};
  M m(std::sorted_unique, std::move(a));

  std::vector<int> newValues = {0, 1, 5, 6, 7, 8};
  stats.throw_after          = 1;
  try {
    insert_function(m, newValues);
    assert(false);
  } catch (int) {
    check_invariant(m);
    // In libc++, we clear if anything goes wrong when inserting a range
    LIBCPP_ASSERT(m.size() == 0);
  }
#endif
}

template <class F>
void test_erase_exception_guarantee([[maybe_unused]] F&& erase_function) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  {
    // key erase throws
    using KeyContainer = ThrowOnEraseContainer<int>;
    using M            = std::flat_set<int, TransparentComparator, KeyContainer>;

    KeyContainer a{1, 2, 3, 4};
    M m(std::sorted_unique, std::move(a));
    try {
      erase_function(m, 3);
      assert(false);
    } catch (int) {
      check_invariant(m);
      // In libc++, we clear if anything goes wrong when erasing
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
#endif
}

#endif // TEST_STD_CONTAINERS_CONTAINER_ADAPTORS_FLAT_SET_HELPERS_H
