// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_RAND_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_RAND_INTERNAL_H

#include <openssl/aes.h>
#include <openssl/ctrdrbg.h>
#include <openssl/rand.h>

#include "../../internal.h"
#include "../modes/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

// kCtrDrbgReseedInterval is the number of generate calls made to CTR-DRBG,
// for a specific state, before reseeding.
static const uint64_t kCtrDrbgReseedInterval = 4096;

#define RAND_NO_USER_PRED_RESISTANCE 0
#define RAND_USE_USER_PRED_RESISTANCE 1

OPENSSL_EXPORT uint64_t get_private_thread_generate_calls_since_seed(void);
OPENSSL_EXPORT uint64_t get_private_thread_reseed_calls_since_initialization(void);

OPENSSL_EXPORT uint64_t get_public_thread_generate_calls_since_seed(void);
OPENSSL_EXPORT uint64_t get_public_thread_reseed_calls_since_initialization(void);

// rand_thread_local_state_clear_all_FOR_TESTING runs the same shutdown
// zeroization sequence that is registered via |atexit|. It is exposed for tests
// that need to verify shutdown behavior (e.g. that a thread exiting after
// shutdown does not deadlock). After this is called, no further |RAND_bytes|
// calls in the test process will return output, so the caller is responsible
// for arranging that no other code paths in the same process require
// randomness afterwards.
OPENSSL_EXPORT void rand_thread_local_state_clear_all_FOR_TESTING(void);

// CTR_DRBG_STATE contains the state of a CTR_DRBG based on AES-256. See SP
// 800-90Ar1.
struct ctr_drbg_state_st {
  AES_KEY ks;
  block128_f block;
  ctr128_f ctr;
  uint8_t counter[16];
  uint64_t reseed_counter;
};
OPENSSL_STATIC_ASSERT((sizeof((struct ctr_drbg_state_st*)0)->reseed_counter) * 8 >= 48, value_can_overflow)

// CTR_DRBG_init initialises |*drbg| given |CTR_DRBG_ENTROPY_LEN| bytes of
// entropy in |entropy| and, optionally, a personalization string up to
// |CTR_DRBG_ENTROPY_LEN| bytes in length. It returns one on success and zero
// on error. |entropy| and |personalization| must not alias.
OPENSSL_EXPORT int CTR_DRBG_init(CTR_DRBG_STATE *drbg,
                                 const uint8_t entropy[CTR_DRBG_ENTROPY_LEN],
                                 const uint8_t *personalization,
                                 size_t personalization_len);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_RAND_INTERNAL_H
