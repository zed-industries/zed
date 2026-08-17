//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SORTABLE_HELPERS_H
#define SORTABLE_HELPERS_H

#include <cstddef>
#include <type_traits>

#include "test_macros.h"

#if TEST_STD_VER > 17
#include <compare>
#include <iterator>
#include "test_iterators.h"
#endif

struct TrivialSortable {
    int value;
    TEST_CONSTEXPR TrivialSortable() : value(0) {}
    TEST_CONSTEXPR TrivialSortable(int v) : value(v) {}
    friend TEST_CONSTEXPR bool operator<(const TrivialSortable& a, const TrivialSortable& b) {
        return a.value / 10 < b.value / 10;
    }
    static TEST_CONSTEXPR bool less(const TrivialSortable& a, const TrivialSortable& b) {
        return a.value < b.value;
    }
};

struct NonTrivialSortable {
    int value;
    TEST_CONSTEXPR NonTrivialSortable() : value(0) {}
    TEST_CONSTEXPR NonTrivialSortable(int v) : value(v) {}
    TEST_CONSTEXPR NonTrivialSortable(const NonTrivialSortable& rhs) : value(rhs.value) {}
    TEST_CONSTEXPR_CXX14 NonTrivialSortable& operator=(const NonTrivialSortable& rhs) { value = rhs.value; return *this; }
    friend TEST_CONSTEXPR bool operator<(const NonTrivialSortable& a, const NonTrivialSortable& b) {
        return a.value / 10 < b.value / 10;
    }
    static TEST_CONSTEXPR bool less(const NonTrivialSortable& a, const NonTrivialSortable& b) {
        return a.value < b.value;
    }
};


struct TrivialSortableWithComp {
    int value;
    TEST_CONSTEXPR TrivialSortableWithComp() : value(0) {}
    TEST_CONSTEXPR TrivialSortableWithComp(int v) : value(v) {}
    struct Comparator {
        TEST_CONSTEXPR bool operator()(const TrivialSortableWithComp& a, const TrivialSortableWithComp& b) const {
            return a.value / 10 < b.value / 10;
        }
    };
    static TEST_CONSTEXPR bool less(const TrivialSortableWithComp& a, const TrivialSortableWithComp& b) {
        return a.value < b.value;
    }
};

struct NonTrivialSortableWithComp {
    int value;
    TEST_CONSTEXPR NonTrivialSortableWithComp() : value(0) {}
    TEST_CONSTEXPR NonTrivialSortableWithComp(int v) : value(v) {}
    TEST_CONSTEXPR NonTrivialSortableWithComp(const NonTrivialSortableWithComp& rhs) : value(rhs.value) {}
    TEST_CONSTEXPR_CXX14 NonTrivialSortableWithComp& operator=(const NonTrivialSortableWithComp& rhs) { value = rhs.value; return *this; }
    struct Comparator {
        TEST_CONSTEXPR bool operator()(const NonTrivialSortableWithComp& a, const NonTrivialSortableWithComp& b) const {
            return a.value / 10 < b.value / 10;
        }
    };
    static TEST_CONSTEXPR bool less(const NonTrivialSortableWithComp& a, const NonTrivialSortableWithComp& b) {
        return a.value < b.value;
    }
};

static_assert(std::is_trivially_copyable<TrivialSortable>::value, "");
static_assert(std::is_trivially_copyable<TrivialSortableWithComp>::value, "");
static_assert(!std::is_trivially_copyable<NonTrivialSortable>::value, "");
static_assert(!std::is_trivially_copyable<NonTrivialSortableWithComp>::value, "");

#if TEST_STD_VER > 17
struct TracedCopy {
  int copied = 0;
  int data   = 0;

  constexpr TracedCopy() = default;
  constexpr TracedCopy(int i) : data(i) {}
  constexpr TracedCopy(const TracedCopy& other) : copied(other.copied + 1), data(other.data) {}

  constexpr TracedCopy(TracedCopy&& other)            = delete;
  constexpr TracedCopy& operator=(TracedCopy&& other) = delete;

  constexpr TracedCopy& operator=(const TracedCopy& other) {
    copied = other.copied + 1;
    data   = other.data;
    return *this;
  }

  constexpr bool copiedOnce() const { return copied == 1; }

  constexpr bool operator==(const TracedCopy& o) const { return data == o.data; }
  constexpr auto operator<=>(const TracedCopy& o) const { return data <=> o.data; }
};

template <class Iter>
struct NonBorrowedRange {
  int* data_;
  std::size_t size_;

  // TODO: some algorithms calls std::__copy
  // std::__copy(contiguous_iterator<int*>, sentinel_wrapper<contiguous_iterator<int*>>, contiguous_iterator<int*>) doesn't seem to work.
  // It seems that it unwraps contiguous_iterator<int*> into int*, and then it failed because there is no == between int* and
  // sentinel_wrapper<contiguous_iterator<int*>>
  using Sent = std::conditional_t<std::contiguous_iterator<Iter>, Iter, sentinel_wrapper<Iter>>;

  constexpr NonBorrowedRange(int* d, std::size_t s) : data_{d}, size_{s} {}

  constexpr Iter begin() const { return Iter{data_}; };
  constexpr Sent end() const { return Sent{Iter{data_ + size_}}; };
};
#endif // TEST_STD_VER > 17

#endif // SORTABLE_HELPERS_H
