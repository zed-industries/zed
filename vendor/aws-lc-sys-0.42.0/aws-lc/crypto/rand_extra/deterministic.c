// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_DETERMINISTIC)

#include <string.h>

#include <openssl/chacha.h>

#include "../internal.h"


// g_num_calls is the number of calls to |CRYPTO_sysrand| that have occurred.
//
// This is intentionally not thread-safe. If the fuzzer mode is ever used in a
// multi-threaded program, replace this with a thread-local. (A mutex would not
// be deterministic.)
static uint64_t g_num_calls = 0;
static struct CRYPTO_STATIC_MUTEX g_num_calls_lock = CRYPTO_STATIC_MUTEX_INIT;

void RAND_reset_for_fuzzing(void) { g_num_calls = 0; }

void CRYPTO_sysrand(uint8_t *out, size_t requested) {
  static const uint8_t kZeroKey[32];

  CRYPTO_STATIC_MUTEX_lock_write(&g_num_calls_lock);
  uint64_t num_calls = g_num_calls++;
  CRYPTO_STATIC_MUTEX_unlock_write(&g_num_calls_lock);

  uint8_t nonce[12];
  OPENSSL_memset(nonce, 0, sizeof(nonce));
  OPENSSL_memcpy(nonce, &num_calls, sizeof(num_calls));

  OPENSSL_memset(out, 0, requested);
  CRYPTO_chacha_20(out, out, requested, kZeroKey, nonce, 0);
}

#endif  // OPENSSL_RAND_DETERMINISTIC
