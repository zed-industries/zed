// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is an internal atomic implementation, use atomicops.h instead.
//
// This implementation uses C++11 atomics' member functions. The code base is
// currently written assuming atomicity revolves around accesses instead of
// C++11's memory locations. The burden is on the programmer to ensure that all
// memory locations accessed atomically are never accessed non-atomically (tsan
// should help with this).
//
// TODO(jfb) Modify the atomicops.h API and user code to declare atomic
//           locations as truly atomic. See the static_assert below.
//
// Of note in this implementation:
//  * All NoBarrier variants are implemented as relaxed.
//  * All Barrier variants are implemented as sequentially-consistent.
//  * Compare exchange's failure ordering is always the same as the success one
//    (except for release, which fails as relaxed): using a weaker ordering is
//    only valid under certain uses of compare exchange.
//  * Atomic increment is expected to return the post-incremented value, whereas
//    C11 fetch add returns the previous value. The implementation therefore
//    needs to increment twice (which the compiler should be able to detect and
//    optimize).

#ifndef BASE_ATOMICOPS_INTERNALS_PORTABLE_H_
#define BASE_ATOMICOPS_INTERNALS_PORTABLE_H_

#include <atomic>

#include "base/numerics/wrapping_math.h"
#include "build/build_config.h"

namespace base {
namespace subtle {

// This implementation is transitional and maintains the original API for
// atomicops.h. This requires casting memory locations to the atomic types, and
// assumes that the API and the C++11 implementation are layout-compatible,
// which isn't true for all implementations or hardware platforms. The static
// assertion should detect this issue, were it to fire then this header
// shouldn't be used.
//
// TODO(jfb) If this header manages to stay committed then the API should be
//           modified, and all call sites updated.
typedef volatile std::atomic<Atomic32>* AtomicLocation32;
static_assert(sizeof(*(AtomicLocation32) nullptr) == sizeof(Atomic32),
              "incompatible 32-bit atomic layout");

inline Atomic32 NoBarrier_CompareAndSwap(volatile Atomic32* ptr,
                                         Atomic32 old_value,
                                         Atomic32 new_value) {
  ((AtomicLocation32)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_relaxed,
                                std::memory_order_relaxed);
  return old_value;
}

inline Atomic32 NoBarrier_AtomicExchange(volatile Atomic32* ptr,
                                         Atomic32 new_value) {
  return ((AtomicLocation32)ptr)
      ->exchange(new_value, std::memory_order_relaxed);
}

inline Atomic32 NoBarrier_AtomicIncrement(volatile Atomic32* ptr,
                                          Atomic32 increment) {
  return base::WrappingAdd(
      ((AtomicLocation32)ptr)->fetch_add(increment, std::memory_order_relaxed),
      increment);
}

inline Atomic32 Barrier_AtomicIncrement(volatile Atomic32* ptr,
                                        Atomic32 increment) {
  return base::WrappingAdd(((AtomicLocation32)ptr)->fetch_add(increment),
                           increment);
}

inline Atomic32 Acquire_CompareAndSwap(volatile Atomic32* ptr,
                                       Atomic32 old_value,
                                       Atomic32 new_value) {
  ((AtomicLocation32)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_acquire,
                                std::memory_order_acquire);
  return old_value;
}

inline Atomic32 Release_CompareAndSwap(volatile Atomic32* ptr,
                                       Atomic32 old_value,
                                       Atomic32 new_value) {
  ((AtomicLocation32)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_release,
                                std::memory_order_relaxed);
  return old_value;
}

inline void NoBarrier_Store(volatile Atomic32* ptr, Atomic32 value) {
  ((AtomicLocation32)ptr)->store(value, std::memory_order_relaxed);
}

inline void Release_Store(volatile Atomic32* ptr, Atomic32 value) {
  ((AtomicLocation32)ptr)->store(value, std::memory_order_release);
}

inline Atomic32 NoBarrier_Load(volatile const Atomic32* ptr) {
  return ((AtomicLocation32)ptr)->load(std::memory_order_relaxed);
}

inline Atomic32 Acquire_Load(volatile const Atomic32* ptr) {
  return ((AtomicLocation32)ptr)->load(std::memory_order_acquire);
}

#if defined(ARCH_CPU_64_BITS)

using AtomicU64 = std::make_unsigned_t<Atomic64>;

typedef volatile std::atomic<Atomic64>* AtomicLocation64;
static_assert(sizeof(*(AtomicLocation64) nullptr) == sizeof(Atomic64),
              "incompatible 64-bit atomic layout");

inline Atomic64 NoBarrier_CompareAndSwap(volatile Atomic64* ptr,
                                         Atomic64 old_value,
                                         Atomic64 new_value) {
  ((AtomicLocation64)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_relaxed,
                                std::memory_order_relaxed);
  return old_value;
}

inline Atomic64 NoBarrier_AtomicExchange(volatile Atomic64* ptr,
                                         Atomic64 new_value) {
  return ((AtomicLocation64)ptr)
      ->exchange(new_value, std::memory_order_relaxed);
}

inline Atomic64 NoBarrier_AtomicIncrement(volatile Atomic64* ptr,
                                          Atomic64 increment) {
  return base::WrappingAdd(
      ((AtomicLocation64)ptr)->fetch_add(increment, std::memory_order_relaxed),
      increment);
}

inline Atomic64 Barrier_AtomicIncrement(volatile Atomic64* ptr,
                                        Atomic64 increment) {
  return base::WrappingAdd(((AtomicLocation64)ptr)->fetch_add(increment),
                           increment);
}

inline Atomic64 Acquire_CompareAndSwap(volatile Atomic64* ptr,
                                       Atomic64 old_value,
                                       Atomic64 new_value) {
  ((AtomicLocation64)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_acquire,
                                std::memory_order_acquire);
  return old_value;
}

inline Atomic64 Release_CompareAndSwap(volatile Atomic64* ptr,
                                       Atomic64 old_value,
                                       Atomic64 new_value) {
  ((AtomicLocation64)ptr)
      ->compare_exchange_strong(old_value, new_value, std::memory_order_release,
                                std::memory_order_relaxed);
  return old_value;
}

inline void Release_Store(volatile Atomic64* ptr, Atomic64 value) {
  ((AtomicLocation64)ptr)->store(value, std::memory_order_release);
}

inline Atomic64 NoBarrier_Load(volatile const Atomic64* ptr) {
  return ((AtomicLocation64)ptr)->load(std::memory_order_relaxed);
}

inline Atomic64 Acquire_Load(volatile const Atomic64* ptr) {
  return ((AtomicLocation64)ptr)->load(std::memory_order_acquire);
}

#endif  // defined(ARCH_CPU_64_BITS)
}  // namespace subtle
}  // namespace base

#endif  // BASE_ATOMICOPS_INTERNALS_PORTABLE_H_
