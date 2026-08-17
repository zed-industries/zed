//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_STD_EXPERIMENTAL_SIMD_TEST_UTILS_H
#define LIBCXX_TEST_STD_EXPERIMENTAL_SIMD_TEST_UTILS_H

#include <array>
#include <cassert>
#include <cstddef>
#include <experimental/simd>
#include <type_traits>
#include <utility>

#include "type_algorithms.h"

namespace ex = std::experimental::parallelism_v2;

constexpr std::size_t max_simd_size = 32;

template <template <class T, std::size_t N> class F>
struct TestAllSimdAbiFunctor {
  template <class T, std::size_t N>
  using sized_abis = types::type_list<ex::simd_abi::fixed_size<N>, ex::simd_abi::deduce_t<T, N>>;

  template <class T, std::size_t... Ns>
  void instantiate_with_n(std::index_sequence<Ns...>) {
    (types::for_each(sized_abis<T, Ns>{}, F<T, Ns>{}), ...);
  }

  template <class T>
  void operator()() {
    using abis = types::type_list<ex::simd_abi::scalar, ex::simd_abi::native<T>, ex::simd_abi::compatible<T>>;
    types::for_each(abis{}, F<T, 1>());

    instantiate_with_n<T>(
        std::index_sequence<1, 2, 3, 4, 8, 16, max_simd_size - 2, max_simd_size - 1, max_simd_size>{});
  }
};

// TODO: Support long double (12 bytes) for 32-bits x86
#ifdef __i386__
using arithmetic_no_bool_types = types::concatenate_t<types::integer_types, types::type_list<float, double>>;
#else
using arithmetic_no_bool_types = types::concatenate_t<types::integer_types, types::floating_point_types>;
#endif

// For interfaces with vectorizable type template parameters, we only use some common or boundary types
// as template parameters for testing to ensure that the compilation time of a single test does not exceed.
using simd_test_integer_types =
    types::type_list<char,
                     unsigned,
                     int
#ifndef TEST_HAS_NO_INT128
                     ,
                     __int128_t
#endif
                     >;
using simd_test_types = types::concatenate_t<simd_test_integer_types, types::type_list<float, double>>;

template <template <class T, std::size_t N> class Func>
void test_all_simd_abi() {
  types::for_each(arithmetic_no_bool_types(), TestAllSimdAbiFunctor<Func>());
}

constexpr size_t bit_ceil(size_t val) {
  size_t pow = 1;
  while (pow < val)
    pow <<= 1;
  return pow;
}

template <class From, class To, class = void>
inline constexpr bool is_non_narrowing_convertible_v = false;

template <class From, class To>
inline constexpr bool is_non_narrowing_convertible_v<From, To, std::void_t<decltype(To{std::declval<From>()})>> = true;

template <std::size_t ArraySize, class SimdAbi, class T, class U = T>
void assert_simd_values_equal(const ex::simd<T, SimdAbi>& origin_simd, const std::array<U, ArraySize>& expected_value) {
  for (std::size_t i = 0; i < origin_simd.size(); ++i)
    assert(origin_simd[i] == static_cast<T>(expected_value[i]));
}

template <std::size_t ArraySize, class T, class SimdAbi>
void assert_simd_mask_values_equal(const ex::simd_mask<T, SimdAbi>& origin_mask,
                                   const std::array<bool, ArraySize>& expected_value) {
  for (std::size_t i = 0; i < origin_mask.size(); ++i)
    assert(origin_mask[i] == expected_value[i]);
}

template <class SimdAbi, class T, class U = T>
void assert_simd_values_equal(const ex::simd<T, SimdAbi>& origin_simd, U* expected_value) {
  for (size_t i = 0; i < origin_simd.size(); ++i)
    assert(origin_simd[i] == static_cast<T>(expected_value[i]));
}

template <class SimdAbi, class T>
void assert_simd_mask_values_equal(const ex::simd_mask<T, SimdAbi>& origin_mask, bool* expected_value) {
  for (size_t i = 0; i < origin_mask.size(); ++i)
    assert(origin_mask[i] == expected_value[i]);
}

#endif // LIBCXX_TEST_STD_EXPERIMENTAL_SIMD_TEST_UTILS_H
