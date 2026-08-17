// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"


#if defined(OPENSSL_C11_ATOMIC)

#include <assert.h>
#include <stdalign.h>
#include <stdatomic.h>
#include <stdlib.h>

#include <openssl/type_check.h>


// See comment above the typedef of CRYPTO_refcount_t about these tests.
OPENSSL_STATIC_ASSERT(alignof(CRYPTO_refcount_t) == alignof(_Atomic CRYPTO_refcount_t),
              _Atomic_alters_the_needed_alignment_of_a_reference_count)
OPENSSL_STATIC_ASSERT(sizeof(CRYPTO_refcount_t) == sizeof(_Atomic CRYPTO_refcount_t),
              _Atomic_alters_the_size_of_a_reference_count)

OPENSSL_STATIC_ASSERT((CRYPTO_refcount_t)-1 == CRYPTO_REFCOUNT_MAX,
              CRYPTO_REFCOUNT_MAX_is_incorrect)

void CRYPTO_refcount_inc(CRYPTO_refcount_t *in_count) {
  _Atomic CRYPTO_refcount_t *count = (_Atomic CRYPTO_refcount_t *) in_count;
  uint32_t expected = atomic_load(count);

  while (expected != CRYPTO_REFCOUNT_MAX) {
    uint32_t new_value = expected + 1;
    if (atomic_compare_exchange_weak(count, &expected, new_value)) {
      break;
    }
  }
}

int CRYPTO_refcount_dec_and_test_zero(CRYPTO_refcount_t *in_count) {
  _Atomic CRYPTO_refcount_t *count = (_Atomic CRYPTO_refcount_t *)in_count;
  uint32_t expected = atomic_load(count);

  for (;;) {
    if (expected == 0) {
      abort();
    } else if (expected == CRYPTO_REFCOUNT_MAX) {
      return 0;
    } else {
      const uint32_t new_value = expected - 1;
      if (atomic_compare_exchange_weak(count, &expected, new_value)) {
        return new_value == 0;
      }
    }
  }
}

#endif  // OPENSSL_C11_ATOMIC
