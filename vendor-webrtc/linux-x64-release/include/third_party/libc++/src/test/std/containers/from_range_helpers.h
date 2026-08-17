//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_FROM_RANGE_HELPERS_H
#define SUPPORT_FROM_RANGE_HELPERS_H

#include <array>
#include <cstddef>
#include <iterator>
#include <type_traits>
#include <vector>

#include "min_allocator.h"
#include "test_allocator.h"
#include "test_iterators.h"
#include "test_macros.h"
#include "type_algorithms.h"

struct Empty {};

template <class T>
struct InputRange {
  cpp20_input_iterator<T*> begin();
  sentinel_wrapper<cpp20_input_iterator<T*>> end();
};

template <class Iter, class Sent, std::ranges::input_range Range>
constexpr auto wrap_input(Range&& input) {
  auto b = Iter(std::ranges::begin(input));
  auto e = Sent(Iter(std::ranges::end(input)));
  return std::ranges::subrange(std::move(b), std::move(e));
}

template <class Iter, class Sent, class T, std::size_t N>
constexpr auto wrap_input(std::array<T, N>& input) {
  auto b = Iter(input.data());
  auto e = Sent(Iter(input.data() + input.size()));
  return std::ranges::subrange(std::move(b), std::move(e));
}

template <class Iter, class Sent, class T>
constexpr auto wrap_input(std::vector<T>& input) {
  auto b = Iter(input.data());
  auto e = Sent(Iter(input.data() + input.size()));
  return std::ranges::subrange(std::move(b), std::move(e));
}

struct KeyValue {
  int key;    // Only the key is considered for equality comparison.
  char value; // Allows distinguishing equivalent instances.

  bool operator<(const KeyValue& other) const { return key < other.key; }
  bool operator==(const KeyValue& other) const { return key == other.key; }
};

template <>
struct std::hash<KeyValue> {
  std::size_t operator()(const KeyValue& kv) const { return kv.key; }
};

#if !defined(TEST_HAS_NO_EXCEPTIONS)

template <class T>
struct ThrowingAllocator {
  using value_type      = T;
  using char_type       = T;
  using is_always_equal = std::false_type;

  ThrowingAllocator() = default;

  template <class U>
  ThrowingAllocator(const ThrowingAllocator<U>&) {}

  T* allocate(std::size_t) { throw 1; }
  void deallocate(T*, std::size_t) {}

  template <class U>
  friend bool operator==(const ThrowingAllocator&, const ThrowingAllocator<U>&) {
    return true;
  }
};
#endif

template <class T, class Func>
constexpr void for_all_iterators_and_allocators(Func f) {
  using Iterators =
      types::type_list< cpp20_input_iterator<T*>,
                        forward_iterator<T*>,
                        bidirectional_iterator<T*>,
                        random_access_iterator<T*>,
                        contiguous_iterator<T*>,
                        T* >;

  types::for_each(Iterators{}, [=]<class Iter>() {
    f.template operator()<Iter, sentinel_wrapper<Iter>, std::allocator<T>>();
    f.template operator()<Iter, sentinel_wrapper<Iter>, test_allocator<T>>();
    f.template operator()<Iter, sentinel_wrapper<Iter>, min_allocator<T>>();
    f.template operator()<Iter, sentinel_wrapper<Iter>, safe_allocator<T>>();

    if constexpr (std::sentinel_for<Iter, Iter>) {
      f.template operator()<Iter, Iter, std::allocator<T>>();
      f.template operator()<Iter, Iter, test_allocator<T>>();
      f.template operator()<Iter, Iter, min_allocator<T>>();
      f.template operator()<Iter, Iter, safe_allocator<T>>();
    }
  });
}

#endif // SUPPORT_FROM_RANGE_HELPERS_H
