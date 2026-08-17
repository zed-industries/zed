//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_EXCEPTION_SAFETY_HELPERS_H
#define SUPPORT_EXCEPTION_SAFETY_HELPERS_H

#include <cassert>
#include <cstddef>
#include <functional>
#include <utility>
#include "test_macros.h"

#if !defined(TEST_HAS_NO_EXCEPTIONS)
template <int N>
struct ThrowingCopy {
  static bool throwing_enabled;
  static int created_by_copying;
  static int destroyed;
  int x = 0; // Allows distinguishing between different instances.

  ThrowingCopy() = default;
  ThrowingCopy(int value) : x(value) {}
  ~ThrowingCopy() { ++destroyed; }

  ThrowingCopy(const ThrowingCopy& other) : x(other.x) {
    ++created_by_copying;
    if (throwing_enabled && created_by_copying == N) {
      throw -1;
    }
  }

  // Defined to silence GCC warnings. For test purposes, only copy construction is considered `created_by_copying`.
  ThrowingCopy& operator=(const ThrowingCopy& other) {
    x = other.x;
    return *this;
  }

  friend bool operator==(const ThrowingCopy& lhs, const ThrowingCopy& rhs) { return lhs.x == rhs.x; }
  friend bool operator<(const ThrowingCopy& lhs, const ThrowingCopy& rhs) { return lhs.x < rhs.x; }

  static void reset() { created_by_copying = destroyed = 0; }
};

template <int N>
bool ThrowingCopy<N>::throwing_enabled = true;
template <int N>
int ThrowingCopy<N>::created_by_copying = 0;
template <int N>
int ThrowingCopy<N>::destroyed = 0;

template <int N>
struct std::hash<ThrowingCopy<N>> {
  std::size_t operator()(const ThrowingCopy<N>& value) const { return value.x; }
};

template <int ThrowOn, int Size, class Func>
void test_exception_safety_throwing_copy(Func&& func) {
  using T = ThrowingCopy<ThrowOn>;
  T::reset();
  T in[Size];

  try {
    func(in, in + Size);
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(T::created_by_copying == ThrowOn);
    assert(T::destroyed == ThrowOn - 1); // No destructor call for the partially-constructed element.
  }
}

// Destroys the container outside the user callback to avoid destroying extra elements upon throwing (which would
// complicate asserting that the expected number of elements was destroyed).
template <class Container, int ThrowOn, int Size, class Func>
void test_exception_safety_throwing_copy_container(Func&& func) {
  using T             = ThrowingCopy<ThrowOn>;
  T::throwing_enabled = false;
  T in[Size];
  Container c(in, in + Size);
  T::throwing_enabled = true;
  T::reset();

  try {
    func(std::move(c));
    assert(false); // The function call above should throw.

  } catch (int) {
    assert(T::created_by_copying == ThrowOn);
    assert(T::destroyed == ThrowOn - 1); // No destructor call for the partially-constructed element.
  }
}

#endif // !defined(TEST_HAS_NO_EXCEPTIONS)

#endif // SUPPORT_EXCEPTION_SAFETY_HELPERS_H
