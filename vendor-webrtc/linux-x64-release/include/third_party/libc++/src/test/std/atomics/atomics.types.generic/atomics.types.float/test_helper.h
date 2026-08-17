//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_ATOMICS_ATOMICS_TYPES_FLOAT_TEST_HELPER_H
#define TEST_STD_ATOMICS_ATOMICS_TYPES_FLOAT_TEST_HELPER_H

#include <atomic>
#include <cassert>
#include <cmath>
#include <vector>

#include "test_macros.h"

#ifndef TEST_HAS_NO_THREADS
#  include "make_test_thread.h"
#  include <thread>
#endif

template <class T>
bool approximately_equals(T x, T y) {
  T epsilon = T(0.001);
  return std::abs(x - y) < epsilon;
}

// Test that all threads see the exact same sequence of events
// Test will pass 100% if store_op and load_op are correctly
// affecting the memory with seq_cst order
template <class T, template <class> class MaybeVolatile, class StoreOp, class LoadOp>
void test_seq_cst(StoreOp store_op, LoadOp load_op) {
#ifndef TEST_HAS_NO_THREADS
  for (int i = 0; i < 100; ++i) {
    T old_value = 0.0;
    T new_value = 1.0;

    MaybeVolatile<std::atomic<T>> x(old_value);
    MaybeVolatile<std::atomic<T>> y(old_value);

    std::atomic_bool x_updated_first(false);
    std::atomic_bool y_updated_first(false);

    auto t1 = support::make_test_thread([&] { store_op(x, old_value, new_value); });

    auto t2 = support::make_test_thread([&] { store_op(y, old_value, new_value); });

    auto t3 = support::make_test_thread([&] {
      while (!approximately_equals(load_op(x), new_value)) {
        std::this_thread::yield();
      }
      if (!approximately_equals(load_op(y), new_value)) {
        x_updated_first.store(true, std::memory_order_relaxed);
      }
    });

    auto t4 = support::make_test_thread([&] {
      while (!approximately_equals(load_op(y), new_value)) {
        std::this_thread::yield();
      }
      if (!approximately_equals(load_op(x), new_value)) {
        y_updated_first.store(true, std::memory_order_relaxed);
      }
    });

    t1.join();
    t2.join();
    t3.join();
    t4.join();
    // thread 3 and thread 4 cannot see different orders of storing x and y
    assert(!(x_updated_first && y_updated_first));
  }
#else
  (void)store_op;
  (void)load_op;
#endif
}

// Test that all writes before the store are seen by other threads after the load
// Test will pass 100% if store_op and load_op are correctly
// affecting the memory with acquire-release order
template <class T, template <class> class MaybeVolatile, class StoreOp, class LoadOp>
void test_acquire_release(StoreOp store_op, LoadOp load_op) {
#ifndef TEST_HAS_NO_THREADS
  for (auto i = 0; i < 100; ++i) {
    T old_value = 0.0;
    T new_value = 1.0;

    MaybeVolatile<std::atomic<T>> at(old_value);
    int non_atomic = 5;

    constexpr auto number_of_threads = 8;
    std::vector<std::thread> threads;
    threads.reserve(number_of_threads);

    for (auto j = 0; j < number_of_threads; ++j) {
      threads.push_back(support::make_test_thread([&at, &non_atomic, load_op, new_value] {
        while (!approximately_equals(load_op(at), new_value)) {
          std::this_thread::yield();
        }
        // Other thread's writes before the release store are visible
        // in this thread's read after the acquire load
        assert(non_atomic == 6);
      }));
    }

    non_atomic = 6;
    store_op(at, old_value, new_value);

    for (auto& thread : threads) {
      thread.join();
    }
  }
#else
  (void)store_op;
  (void)load_op;
#endif
}

#endif // TEST_STD_ATOMICS_ATOMICS_TYPES_FLOAT_TEST_HELPER_H
