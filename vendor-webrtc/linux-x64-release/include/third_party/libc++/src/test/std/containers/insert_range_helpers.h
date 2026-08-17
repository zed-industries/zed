//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_INSERT_RANGE_HELPERS_H
#define SUPPORT_INSERT_RANGE_HELPERS_H

#include <algorithm>
#include <array>
#include <cassert>
#include <concepts>
#include <map>
#include <ranges>
#include <set>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "exception_safety_helpers.h"
#include "from_range_helpers.h"
#include "min_allocator.h"
#include "test_allocator.h"
#include "test_iterators.h"
#include "test_macros.h"
#include "type_algorithms.h"

// A simple literal-type container. It can be used as a `constexpr` global variable (which isn't supported by
// `std::vector`).
template <class T, std::size_t N = 32>
class Buffer {
public:
  constexpr Buffer() = default;

  constexpr Buffer(std::initializer_list<T> input) {
    assert(input.size() <= N);
    std::ranges::copy(input, data_);
    size_ = input.size();
  }

  // Makes initializing `Buffer<char>` nicer -- allows writing `buf = "abc"` instead of `buf = {'a', 'b', 'c'}`.
  // To make the two forms equivalent, omits the terminating null.
  template <std::size_t N2>
  constexpr Buffer(const char (&input)[N2])
    requires std::same_as<T, char>
  {
    static_assert(N2 <= N);
    std::ranges::copy(input, data_);
    // Omit the terminating null.
    size_ = input[N2 - 1] == '\0' ? N2 - 1 : N2;
  }

  constexpr const T* begin() const { return data_; }
  constexpr const T* end() const { return data_ + size_; }
  constexpr std::size_t size() const { return size_; }

private:
  std::size_t size_ = 0;
  T data_[N]        = {};
};

template <class T>
struct TestCase {
  Buffer<T> initial;
  std::size_t index = 0;
  Buffer<T> input;
  Buffer<T> expected;
};

template <class T, class PtrT, class Func>
constexpr void for_all_iterators_and_allocators(Func f) {
  using Iterators =
      types::type_list< cpp20_input_iterator<PtrT>,
                        forward_iterator<PtrT>,
                        bidirectional_iterator<PtrT>,
                        random_access_iterator<PtrT>,
                        contiguous_iterator<PtrT>,
                        PtrT >;

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

// Uses a shorter list of iterator types for use in `constexpr` mode for cases when running the full set in would take
// too long.
template <class T, class PtrT, class Func>
constexpr void for_all_iterators_and_allocators_constexpr(Func f) {
  using Iterators = types::type_list< cpp20_input_iterator<PtrT>, forward_iterator<PtrT>, PtrT >;

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

#endif // SUPPORT_INSERT_RANGE_HELPERS_H
