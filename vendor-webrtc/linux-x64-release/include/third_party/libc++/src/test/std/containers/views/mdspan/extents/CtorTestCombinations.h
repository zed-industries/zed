//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// UNSUPPORTED: c++03, c++11, c++14, c++17, c++20

// <mdspan>

#include <array>
#include <cassert>
#include <cstddef>
#include <mdspan>
#include <span>

#include "../ConvertibleToIntegral.h"
#include "test_macros.h"

// Helper file to implement combinatorial testing of extents constructor
//
// std::extents can be constructed from just indices, a std::array, or a std::span
// In each of those cases one can either provide all extents, or just the dynamic ones
// If constructed from std::span, the span needs to have a static extent
// Furthermore, the indices/array/span can have integer types other than index_type

template <class E, class AllExtents>
constexpr void test_runtime_observers(E ext, AllExtents expected) {
  for (typename E::rank_type r = 0; r < ext.rank(); r++) {
    ASSERT_SAME_TYPE(decltype(ext.extent(0)), typename E::index_type);
    ASSERT_NOEXCEPT(ext.extent(0));
    assert(ext.extent(r) == static_cast<typename E::index_type>(expected[r]));
  }
}

template <class E, class AllExtents>
constexpr void test_implicit_construction_call(E e, AllExtents all_ext) {
  test_runtime_observers(e, all_ext);
}

template <class E, class Test, class AllExtents>
constexpr void test_construction(AllExtents all_ext) {
  // test construction from all extents
  Test::template test_construction<E>(all_ext, all_ext, std::make_index_sequence<E::rank()>());

  // test construction from just dynamic extents
  // create an array of just the extents corresponding to dynamic values
  std::array<typename AllExtents::value_type, E::rank_dynamic()> dyn_ext{};
  size_t dynamic_idx = 0;
  for (size_t r = 0; r < E::rank(); r++) {
    if (E::static_extent(r) == std::dynamic_extent) {
      dyn_ext[dynamic_idx] = all_ext[r];
      dynamic_idx++;
    }
  }
  Test::template test_construction<E>(all_ext, dyn_ext, std::make_index_sequence<E::rank_dynamic()>());
}

template <class T, class TArg, class Test>
constexpr void test() {
  constexpr size_t D = std::dynamic_extent;

  test_construction<std::extents<T>, Test>(std::array<TArg, 0>{});

  test_construction<std::extents<T, 3>, Test>(std::array<TArg, 1>{3});
  test_construction<std::extents<T, D>, Test>(std::array<TArg, 1>{3});

  test_construction<std::extents<T, 3, 7>, Test>(std::array<TArg, 2>{3, 7});
  test_construction<std::extents<T, 3, D>, Test>(std::array<TArg, 2>{3, 7});
  test_construction<std::extents<T, D, 7>, Test>(std::array<TArg, 2>{3, 7});
  test_construction<std::extents<T, D, D>, Test>(std::array<TArg, 2>{3, 7});

  test_construction<std::extents<T, 3, 7, 9>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, 3, 7, D>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, 3, D, D>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, D, 7, D>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, D, D, D>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, 3, D, 9>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, D, D, 9>, Test>(std::array<TArg, 3>{3, 7, 9});
  test_construction<std::extents<T, D, 7, 9>, Test>(std::array<TArg, 3>{3, 7, 9});

  test_construction<std::extents<T, 1, 2, 3, 4, 5, 6, 7, 8, 9>, Test>(std::array<TArg, 9>{1, 2, 3, 4, 5, 6, 7, 8, 9});
  test_construction<std::extents<T, D, 2, 3, D, 5, D, 7, D, 9>, Test>(std::array<TArg, 9>{1, 2, 3, 4, 5, 6, 7, 8, 9});
  test_construction<std::extents<T, D, D, D, D, D, D, D, D, D>, Test>(std::array<TArg, 9>{1, 2, 3, 4, 5, 6, 7, 8, 9});
}

template <class Test>
constexpr bool test_index_type_combo() {
  test<int, int, Test>();
  test<int, size_t, Test>();
  test<unsigned, int, Test>();
  test<signed char, size_t, Test>();
  test<long long, unsigned, Test>();
  test<size_t, int, Test>();
  test<size_t, size_t, Test>();
  test<int, IntType, Test>();
  test<signed char, IntType, Test>();
  return true;
}
