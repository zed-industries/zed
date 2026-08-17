// Copyright (c) 2023, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#if defined(OPENSSL_WINDOWS_ATOMIC)

#include <windows.h>


// See comment above the typedef of CRYPTO_refcount_t about these tests.
OPENSSL_STATIC_ASSERT(alignof(CRYPTO_refcount_t) == alignof(LONG),
              CRYPTO_refcount_t_does_not_match_LONG_alignment);
OPENSSL_STATIC_ASSERT(sizeof(CRYPTO_refcount_t) == sizeof(LONG),
              CRYPTO_refcount_t_does_not_match_LONG_size);

OPENSSL_STATIC_ASSERT((CRYPTO_refcount_t)-1 == CRYPTO_REFCOUNT_MAX,
              CRYPTO_REFCOUNT_MAX_is_incorrect);

static uint32_t atomic_load_u32(volatile LONG *ptr) {
  // This is not ideal because it still writes to a cacheline. MSVC is not able
  // to optimize this to a true atomic read, and Windows does not provide an
  // InterlockedLoad function.
  //
  // The Windows documentation [1] does say "Simple reads and writes to
  // properly-aligned 32-bit variables are atomic operations", but this is not
  // phrased in terms of the C11 and C++11 memory models, and indeed a read or
  // write seems to produce slightly different code on MSVC than a sequentially
  // consistent std::atomic::load in C++. Moreover, it is unclear if non-MSVC
  // compilers on Windows provide the same guarantees. Thus we avoid relying on
  // this and instead still use an interlocked function. This is still
  // preferable a global mutex, and eventually this code will be replaced by
  // [2]. Additionally, on clang-cl, we'll use the |OPENSSL_C11_ATOMIC| path.
  //
  // [1] https://learn.microsoft.com/en-us/windows/win32/sync/interlocked-variable-access
  // [2] https://devblogs.microsoft.com/cppblog/c11-atomics-in-visual-studio-2022-version-17-5-preview-2/
  return (uint32_t)InterlockedCompareExchange(ptr, 0, 0);
}

static int atomic_compare_exchange_u32(volatile LONG *ptr, uint32_t *expected32,
                                       uint32_t desired) {
  LONG expected = (LONG)*expected32;
  LONG actual = InterlockedCompareExchange(ptr, (LONG)desired, expected);
  *expected32 = (uint32_t)actual;
  return actual == expected;
}

void CRYPTO_refcount_inc(CRYPTO_refcount_t *in_count) {
  volatile LONG *count = (volatile LONG *)in_count;
  uint32_t expected = atomic_load_u32(count);

  while (expected != CRYPTO_REFCOUNT_MAX) {
    const uint32_t new_value = expected + 1;
    if (atomic_compare_exchange_u32(count, &expected, new_value)) {
      break;
    }
  }
}

int CRYPTO_refcount_dec_and_test_zero(CRYPTO_refcount_t *in_count) {
  volatile LONG *count = (volatile LONG *)in_count;
  uint32_t expected = atomic_load_u32(count);

  for (;;) {
    if (expected == 0) {
      abort();
    } else if (expected == CRYPTO_REFCOUNT_MAX) {
      return 0;
    } else {
      const uint32_t new_value = expected - 1;
      if (atomic_compare_exchange_u32(count, &expected, new_value)) {
        return new_value == 0;
      }
    }
  }
}

#endif  // OPENSSL_WINDOWS_ATOMIC
