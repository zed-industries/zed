// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#include <stdlib.h>

#include <openssl/type_check.h>


#if !defined(OPENSSL_C11_ATOMIC) && !defined(OPENSSL_WINDOWS_ATOMIC)

OPENSSL_STATIC_ASSERT((CRYPTO_refcount_t)-1 == CRYPTO_REFCOUNT_MAX,
                      CRYPTO_REFCOUNT_MAX_is_incorrect)

static struct CRYPTO_STATIC_MUTEX g_refcount_lock = CRYPTO_STATIC_MUTEX_INIT;

void CRYPTO_refcount_inc(CRYPTO_refcount_t *count) {
  CRYPTO_STATIC_MUTEX_lock_write(&g_refcount_lock);
  if (*count < CRYPTO_REFCOUNT_MAX) {
    (*count)++;
  }
  CRYPTO_STATIC_MUTEX_unlock_write(&g_refcount_lock);
}

int CRYPTO_refcount_dec_and_test_zero(CRYPTO_refcount_t *count) {
  int ret;

  CRYPTO_STATIC_MUTEX_lock_write(&g_refcount_lock);
  if (*count == 0) {
    abort();
  }
  if (*count < CRYPTO_REFCOUNT_MAX) {
    (*count)--;
  }
  ret = (*count == 0);
  CRYPTO_STATIC_MUTEX_unlock_write(&g_refcount_lock);

  return ret;
}

#endif  // !OPENSSL_C11_ATOMIC && !OPENSSL_WINDOWS_ATOMICS
