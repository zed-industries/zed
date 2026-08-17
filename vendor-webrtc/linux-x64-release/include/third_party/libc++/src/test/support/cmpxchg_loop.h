//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include <atomic>

template <class A>
bool cmpxchg_weak_loop(A& atomic, typename A::value_type& expected, typename A::value_type desired) {
  for (int i = 0; i < 10; i++) {
    if (atomic.compare_exchange_weak(expected, desired) == true) {
      return true;
    }
  }

  return false;
}

template <class A>
bool cmpxchg_weak_loop(A& atomic, typename A::value_type& expected, typename A::value_type desired,
                       std::memory_order success,
                       std::memory_order failure) {
  for (int i = 0; i < 10; i++) {
    if (atomic.compare_exchange_weak(expected, desired, success,
                                     failure) == true) {
      return true;
    }
  }

  return false;
}

template <class A>
bool c_cmpxchg_weak_loop(A* atomic, typename A::value_type* expected, typename A::value_type desired) {
  for (int i = 0; i < 10; i++) {
    if (std::atomic_compare_exchange_weak(atomic, expected, desired) == true) {
      return true;
    }
  }

  return false;
}

template <class A>
bool c_cmpxchg_weak_loop(A* atomic, typename A::value_type* expected, typename A::value_type desired,
                         std::memory_order success,
                         std::memory_order failure) {
  for (int i = 0; i < 10; i++) {
    if (std::atomic_compare_exchange_weak_explicit(atomic, expected, desired,
                                                   success, failure) == true) {
      return true;
    }
  }

  return false;
}
