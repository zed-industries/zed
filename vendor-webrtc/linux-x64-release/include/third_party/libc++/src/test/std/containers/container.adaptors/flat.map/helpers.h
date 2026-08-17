//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FLAT_MAP_HELPERS_H
#define SUPPORT_FLAT_MAP_HELPERS_H

#include <algorithm>
#include <cassert>
#include <string>
#include <vector>
#include <flat_map>

#include "../flat_helpers.h"
#include "test_allocator.h"
#include "test_macros.h"

template <class... Args>
void check_invariant(const std::flat_map<Args...>& m) {
  assert(m.keys().size() == m.values().size());
  const auto& keys = m.keys();
  assert(std::is_sorted(keys.begin(), keys.end(), m.key_comp()));
  auto key_equal = [&](const auto& x, const auto& y) {
    const auto& c = m.key_comp();
    return !c(x, y) && !c(y, x);
  };
  assert(std::adjacent_find(keys.begin(), keys.end(), key_equal) == keys.end());
}

template <class F>
void test_emplace_exception_guarantee([[maybe_unused]] F&& emplace_function) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  using C = TransparentComparator;
  {
    // Throw on emplace the key, and underlying has strong exception guarantee
    using KeyContainer = std::vector<int, test_allocator<int>>;
    using M            = std::flat_map<int, int, C, KeyContainer>;

    LIBCPP_STATIC_ASSERT(std::__container_traits<KeyContainer>::__emplacement_has_strong_exception_safety_guarantee);

    test_allocator_statistics stats;

    KeyContainer a({1, 2, 3, 4}, test_allocator<int>{&stats});
    std::vector<int> b                    = {5, 6, 7, 8};
    [[maybe_unused]] auto expected_keys   = a;
    [[maybe_unused]] auto expected_values = b;
    M m(std::sorted_unique, std::move(a), std::move(b));

    stats.throw_after = 1;
    try {
      emplace_function(m, 0, 0);
      assert(false);
    } catch (const std::bad_alloc&) {
      check_invariant(m);
      // In libc++, the flat_map is unchanged
      LIBCPP_ASSERT(m.size() == 4);
      LIBCPP_ASSERT(m.keys() == expected_keys);
      LIBCPP_ASSERT(m.values() == expected_values);
    }
  }
  {
    // Throw on emplace the key, and underlying has no strong exception guarantee
    using KeyContainer = EmplaceUnsafeContainer<int>;
    using M            = std::flat_map<int, int, C, KeyContainer>;

    LIBCPP_STATIC_ASSERT(!std::__container_traits<KeyContainer>::__emplacement_has_strong_exception_safety_guarantee);
    KeyContainer a     = {1, 2, 3, 4};
    std::vector<int> b = {5, 6, 7, 8};
    M m(std::sorted_unique, std::move(a), std::move(b));
    try {
      emplace_function(m, 0, 0);
      assert(false);
    } catch (int) {
      check_invariant(m);
      // In libc++, the flat_map is cleared
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
  {
    // Throw on emplace the value, and underlying has strong exception guarantee
    using ValueContainer = std::vector<int, test_allocator<int>>;
    ;
    using M = std::flat_map<int, int, C, std::vector<int>, ValueContainer>;

    LIBCPP_STATIC_ASSERT(std::__container_traits<ValueContainer>::__emplacement_has_strong_exception_safety_guarantee);

    std::vector<int> a = {1, 2, 3, 4};
    test_allocator_statistics stats;
    ValueContainer b({1, 2, 3, 4}, test_allocator<int>{&stats});

    [[maybe_unused]] auto expected_keys   = a;
    [[maybe_unused]] auto expected_values = b;
    M m(std::sorted_unique, std::move(a), std::move(b));

    stats.throw_after = 1;
    try {
      emplace_function(m, 0, 0);
      assert(false);
    } catch (const std::bad_alloc&) {
      check_invariant(m);
      // In libc++, the emplaced key is erased and the flat_map is unchanged
      LIBCPP_ASSERT(m.size() == 4);
      LIBCPP_ASSERT(m.keys() == expected_keys);
      LIBCPP_ASSERT(m.values() == expected_values);
    }
  }
  {
    // Throw on emplace the value, and underlying has no strong exception guarantee
    using ValueContainer = EmplaceUnsafeContainer<int>;
    using M              = std::flat_map<int, int, C, std::vector<int>, ValueContainer>;

    LIBCPP_STATIC_ASSERT(!std::__container_traits<ValueContainer>::__emplacement_has_strong_exception_safety_guarantee);
    std::vector<int> a = {1, 2, 3, 4};
    ValueContainer b   = {1, 2, 3, 4};

    M m(std::sorted_unique, std::move(a), std::move(b));

    try {
      emplace_function(m, 0, 0);
      assert(false);
    } catch (int) {
      check_invariant(m);
      // In libc++, the flat_map is cleared
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
  {
    // Throw on emplace the value, then throw again on erasing the key
    using KeyContainer   = ThrowOnEraseContainer<int>;
    using ValueContainer = std::vector<int, test_allocator<int>>;
    using M              = std::flat_map<int, int, C, KeyContainer, ValueContainer>;

    LIBCPP_STATIC_ASSERT(std::__container_traits<ValueContainer>::__emplacement_has_strong_exception_safety_guarantee);

    KeyContainer a = {1, 2, 3, 4};
    test_allocator_statistics stats;
    ValueContainer b({1, 2, 3, 4}, test_allocator<int>{&stats});

    M m(std::sorted_unique, std::move(a), std::move(b));
    stats.throw_after = 1;
    try {
      emplace_function(m, 0, 0);
      assert(false);
    } catch (const std::bad_alloc&) {
      check_invariant(m);
      // In libc++, we try to erase the key after value emplacement failure.
      // and after erasure failure, we clear the flat_map
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
#endif
}

template <class F>
void test_insert_range_exception_guarantee([[maybe_unused]] F&& insert_function) {
#ifndef TEST_HAS_NO_EXCEPTIONS
  using KeyContainer   = EmplaceUnsafeContainer<int>;
  using ValueContainer = std::vector<int>;
  using M              = std::flat_map<int, int, std::ranges::less, KeyContainer, ValueContainer>;
  test_allocator_statistics stats;
  KeyContainer a{1, 2, 3, 4};
  ValueContainer b{1, 2, 3, 4};
  M m(std::sorted_unique, std::move(a), std::move(b));

  std::vector<std::pair<int, int>> newValues = {{0, 0}, {1, 1}, {5, 5}, {6, 6}, {7, 7}, {8, 8}};
  stats.throw_after                          = 1;
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
    using KeyContainer   = ThrowOnEraseContainer<int>;
    using ValueContainer = std::vector<int>;
    using M              = std::flat_map<int, int, TransparentComparator, KeyContainer, ValueContainer>;

    KeyContainer a{1, 2, 3, 4};
    ValueContainer b{1, 2, 3, 4};
    M m(std::sorted_unique, std::move(a), std::move(b));
    try {
      erase_function(m, 3);
      assert(false);
    } catch (int) {
      check_invariant(m);
      // In libc++, we clear if anything goes wrong when erasing
      LIBCPP_ASSERT(m.size() == 0);
    }
  }
  {
    // key erase throws
    using KeyContainer   = std::vector<int>;
    using ValueContainer = ThrowOnEraseContainer<int>;
    using M              = std::flat_map<int, int, TransparentComparator, KeyContainer, ValueContainer>;

    KeyContainer a{1, 2, 3, 4};
    ValueContainer b{1, 2, 3, 4};
    M m(std::sorted_unique, std::move(a), std::move(b));
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

#endif // SUPPORT_FLAT_MAP_HELPERS_H
