// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_PORT_ATOMIC_H_
#define UPB_PORT_ATOMIC_H_

#include "upb/port/def.inc"

#ifdef UPB_USE_C11_ATOMICS

// IWYU pragma: begin_exports
#include <stdatomic.h>
#include <stdbool.h>
// IWYU pragma: end_exports

#define upb_Atomic_Init(addr, val) atomic_init(addr, val)
#define upb_Atomic_Load(addr, order) atomic_load_explicit(addr, order)
#define upb_Atomic_Store(addr, val, order) \
  atomic_store_explicit(addr, val, order)
#define upb_Atomic_Exchange(addr, val, order) \
  atomic_exchange_explicit(addr, val, order)
#define upb_Atomic_CompareExchangeStrong(addr, expected, desired,      \
                                         success_order, failure_order) \
  atomic_compare_exchange_strong_explicit(addr, expected, desired,     \
                                          success_order, failure_order)
#define upb_Atomic_CompareExchangeWeak(addr, expected, desired, success_order, \
                                       failure_order)                          \
  atomic_compare_exchange_weak_explicit(addr, expected, desired,               \
                                        success_order, failure_order)

#elif defined(UPB_USE_MSC_ATOMICS)
#include <intrin.h>
#include <stdbool.h>
#include <stdint.h>

#define upb_Atomic_Init(addr, val) (*(addr) = val)

#if defined(_WIN64)
// MSVC, without C11 atomics, does not have any way in pure C to force
// load-acquire store-release behavior, so we hack it with exchanges.
#pragma intrinsic(_InterlockedExchange64)
#define upb_Atomic_Store(addr, val, order) \
  (void)_InterlockedExchange64((uint64_t volatile *)addr, (uint64_t)val)

#pragma intrinsic(_InterlockedCompareExchange64)
static uintptr_t upb_Atomic_LoadMsc(uint64_t volatile *addr) {
  // Compare exchange with an unlikely value reduces the risk of a spurious
  // (but harmless) store
  return _InterlockedCompareExchange64(addr, 0xDEADC0DEBAADF00D,
                                       0xDEADC0DEBAADF00D);
}
// If _Generic is available, use it to avoid emitting a "'uintptr_t' differs in
// levels of indirection from 'void *'" or -Wint-conversion compiler warning.
#if __STDC_VERSION__ >= 201112L
#define upb_Atomic_Load(addr, order)                           \
  _Generic(addr,                                               \
      UPB_ATOMIC(uintptr_t) *: upb_Atomic_LoadMsc(             \
                                 (uint64_t volatile *)(addr)), \
      default: (void *)upb_Atomic_LoadMsc((uint64_t volatile *)(addr)))

#define upb_Atomic_Exchange(addr, val, order)                                 \
  _Generic(addr,                                                              \
      UPB_ATOMIC(uintptr_t) *: _InterlockedExchange64(                        \
                                 (uint64_t volatile *)(addr), (uint64_t)val), \
      default: (void *)_InterlockedExchange64((uint64_t volatile *)addr,      \
                                              (uint64_t)val))
#else
// Compare exchange with an unlikely value reduces the risk of a spurious
// (but harmless) store
#define upb_Atomic_Load(addr, order) \
  (void *)upb_Atomic_LoadMsc((uint64_t volatile *)(addr))

#define upb_Atomic_Exchange(addr, val, order) \
  (void *)_InterlockedExchange64((uint64_t volatile *)addr, (uint64_t)val)
#endif

#pragma intrinsic(_InterlockedCompareExchange64)
static bool upb_Atomic_CompareExchangeMscP(uint64_t volatile *addr,
                                           uint64_t *expected,
                                           uint64_t desired) {
  uint64_t expect_val = *expected;
  uint64_t actual_val =
      _InterlockedCompareExchange64(addr, desired, expect_val);
  if (expect_val != actual_val) {
    *expected = actual_val;
    return false;
  }
  return true;
}

#define upb_Atomic_CompareExchangeStrong(addr, expected, desired,      \
                                         success_order, failure_order) \
  upb_Atomic_CompareExchangeMscP((uint64_t volatile *)addr,            \
                                 (uint64_t *)expected, (uint64_t)desired)

#define upb_Atomic_CompareExchangeWeak(addr, expected, desired, success_order, \
                                       failure_order)                          \
  upb_Atomic_CompareExchangeMscP((uint64_t volatile *)addr,                    \
                                 (uint64_t *)expected, (uint64_t)desired)

#else  // 32 bit pointers
#pragma intrinsic(_InterlockedExchange)
#define upb_Atomic_Store(addr, val, order) \
  (void)_InterlockedExchange((uint32_t volatile *)addr, (uint32_t)val)

#pragma intrinsic(_InterlockedCompareExchange)
static uintptr_t upb_Atomic_LoadMsc(uint32_t volatile *addr) {
  // Compare exchange with an unlikely value reduces the risk of a spurious
  // (but harmless) store
  return _InterlockedCompareExchange(addr, 0xDEADC0DE, 0xDEADC0DE);
}
// If _Generic is available, use it to avoid emitting 'uintptr_t' differs in
// levels of indirection from 'void *'
#if __STDC_VERSION__ >= 201112L
#define upb_Atomic_Load(addr, order)                           \
  _Generic(addr,                                               \
      UPB_ATOMIC(uintptr_t) *: upb_Atomic_LoadMsc(             \
                                 (uint32_t volatile *)(addr)), \
      default: (void *)upb_Atomic_LoadMsc((uint32_t volatile *)(addr)))

#define upb_Atomic_Exchange(addr, val, order)                                 \
  _Generic(addr,                                                              \
      UPB_ATOMIC(uintptr_t) *: _InterlockedExchange(                          \
                                 (uint32_t volatile *)(addr), (uint32_t)val), \
      default: (void *)_InterlockedExchange64((uint32_t volatile *)addr,      \
                                              (uint32_t)val))
#else
#define upb_Atomic_Load(addr, order) \
  (void *)upb_Atomic_LoadMsc((uint32_t volatile *)(addr))

#define upb_Atomic_Exchange(addr, val, order) \
  (void *)_InterlockedExchange((uint32_t volatile *)addr, (uint32_t)val)
#endif

#pragma intrinsic(_InterlockedCompareExchange)
static bool upb_Atomic_CompareExchangeMscP(uint32_t volatile *addr,
                                           uint32_t *expected,
                                           uint32_t desired) {
  uint32_t expect_val = *expected;
  uint32_t actual_val = _InterlockedCompareExchange(addr, desired, expect_val);
  if (expect_val != actual_val) {
    *expected = actual_val;
    return false;
  }
  return true;
}

#define upb_Atomic_CompareExchangeStrong(addr, expected, desired,      \
                                         success_order, failure_order) \
  upb_Atomic_CompareExchangeMscP((uint32_t volatile *)addr,            \
                                 (uint32_t *)expected, (uint32_t)desired)

#define upb_Atomic_CompareExchangeWeak(addr, expected, desired, success_order, \
                                       failure_order)                          \
  upb_Atomic_CompareExchangeMscP((uint32_t volatile *)addr,                    \
                                 (uint32_t *)expected, (uint32_t)desired)
#endif

#else  // No atomics

#if !defined(UPB_SUPPRESS_MISSING_ATOMICS)
// NOLINTNEXTLINE
#error Your compiler does not support atomic instructions, which UPB uses. If you do not use UPB on multiple threads, you can suppress this error by defining UPB_SUPPRESS_MISSING_ATOMICS.
#endif

#include <string.h>

#define upb_Atomic_Init(addr, val) (*addr = val)
#define upb_Atomic_Load(addr, order) (*addr)
#define upb_Atomic_Store(addr, val, order) (*(addr) = val)

UPB_INLINE void* _upb_NonAtomic_Exchange(void* addr, void* value) {
  void* old;
  memcpy(&old, addr, sizeof(value));
  memcpy(addr, &value, sizeof(value));
  return old;
}

#define upb_Atomic_Exchange(addr, val, order) _upb_NonAtomic_Exchange(addr, val)

// `addr` and `expected` are logically double pointers.
UPB_INLINE bool _upb_NonAtomic_CompareExchangeStrongP(void* addr,
                                                      void* expected,
                                                      void* desired) {
  if (memcmp(addr, expected, sizeof(desired)) == 0) {
    memcpy(addr, &desired, sizeof(desired));
    return true;
  } else {
    memcpy(expected, addr, sizeof(desired));
    return false;
  }
}

#define upb_Atomic_CompareExchangeStrong(addr, expected, desired,      \
                                         success_order, failure_order) \
  _upb_NonAtomic_CompareExchangeStrongP((void*)addr, (void*)expected,  \
                                        (void*)desired)
#define upb_Atomic_CompareExchangeWeak(addr, expected, desired, success_order, \
                                       failure_order)                          \
  upb_Atomic_CompareExchangeStrong(addr, expected, desired, 0, 0)

#endif

#include "upb/port/undef.inc"

#endif  // UPB_PORT_ATOMIC_H_
