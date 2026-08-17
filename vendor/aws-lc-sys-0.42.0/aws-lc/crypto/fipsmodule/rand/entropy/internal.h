// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CRYPTO_RAND_ENTROPY_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_RAND_ENTROPY_INTERNAL_H

#include <openssl/ctrdrbg.h>
#include <openssl/rand.h>

#include "../../cpucap/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

#define OVERRIDDEN_ENTROPY_SOURCE 0
#define TREE_DRBG_JITTER_ENTROPY_SOURCE 1
#define OPT_OUT_CPU_JITTER_ENTROPY_SOURCE 2

#define ENTROPY_JITTER_MAX_NUM_TRIES (3)

// TREE_JITTER_GLOBAL_DRBG_MAX_GENERATE = 2^24
#define TREE_JITTER_GLOBAL_DRBG_MAX_GENERATE 0x1000000
// TREE_JITTER_THREAD_DRBG_MAX_GENERATE = 2^20
#define TREE_JITTER_THREAD_DRBG_MAX_GENERATE 0x100000

struct entropy_source_t {
  void *state;
  const struct entropy_source_methods *methods;
};

struct entropy_source_methods {
  int (*initialize)(struct entropy_source_t *entropy_source);
  void (*zeroize_thread)(struct entropy_source_t *entropy_source);
  void (*free_thread)(struct entropy_source_t *entropy_source);
  int (*get_seed)(const struct entropy_source_t *entropy_source,
    uint8_t seed[CTR_DRBG_ENTROPY_LEN]);
  int (*get_extra_entropy)(const struct entropy_source_t *entropy_source,
    uint8_t extra_entropy[CTR_DRBG_ENTROPY_LEN]);
  int (*get_prediction_resistance)(const struct entropy_source_t *entropy_source,
    uint8_t pred_resistance[RAND_PRED_RESISTANCE_LEN]);
  int id;
};

// get_entropy_source will return an entropy source configured for the platform.
struct entropy_source_t * get_entropy_source(void);

// override_entropy_source_method_FOR_TESTING will override the global
// entropy source that is assigned when calling |get_entropy_source|.
// |override_entropy_source_method_FOR_TESTING| can be called multiple times but
// it's designed to allow overriding the entropy source for testing purposes at
// the start of a process.
OPENSSL_EXPORT void override_entropy_source_method_FOR_TESTING(
  const struct entropy_source_methods *override_entropy_source_methods);

OPENSSL_EXPORT int get_entropy_source_method_id_FOR_TESTING(void);

#if !defined(DISABLE_CPU_JITTER_ENTROPY)
  OPENSSL_EXPORT int tree_jitter_initialize(struct entropy_source_t *entropy_source);
  OPENSSL_EXPORT void tree_jitter_zeroize_thread_drbg(struct entropy_source_t *entropy_source);
  OPENSSL_EXPORT void tree_jitter_free_thread_drbg(struct entropy_source_t *entropy_source);
  OPENSSL_EXPORT int tree_jitter_get_seed(
    const struct entropy_source_t *entropy_source, uint8_t seed[CTR_DRBG_ENTROPY_LEN]);
#else // !defined(DISABLE_CPU_JITTER_ENTROPY)
  // Define stubs for tree-DRBG functions that implements the entropy source
  // interface.
  static inline int tree_jitter_initialize(struct entropy_source_t *entropy_source) { return 0; }
  static inline void tree_jitter_zeroize_thread_drbg(struct entropy_source_t *entropy_source) { abort(); }
  static inline void tree_jitter_free_thread_drbg(struct entropy_source_t *entropy_source) { abort(); }
  static inline int tree_jitter_get_seed(const struct entropy_source_t *entropy_source, uint8_t seed[CTR_DRBG_ENTROPY_LEN]) { return 0; }
#endif // !defined(DISABLE_CPU_JITTER_ENTROPY)

// rndr_multiple8 writes |len| number of bytes to |buf| generated using the
// rndr instruction. |len| must be a multiple of 8.
// Outputs 1 on success, 0 otherwise.
OPENSSL_EXPORT int rndr_multiple8(uint8_t *buf, const size_t len);

// have_hw_rng_aarch64_for_testing wraps |have_hw_rng_aarch64| to allow usage
// in testing.
OPENSSL_EXPORT int have_hw_rng_aarch64_for_testing(void);

#if defined(OPENSSL_AARCH64) && !defined(OPENSSL_NO_ASM)

// CRYPTO_rndr_multiple8 writes |len| number of bytes to |buf| generated using
// the rndr instruction. |len| must be a multiple of 8 and positive.
// Outputs 1 on success, 0 otherwise.
int CRYPTO_rndr_multiple8(uint8_t *out, size_t out_len);

// Returns 1 if Armv8-A instruction rndr is available, 0 otherwise.
OPENSSL_INLINE int have_hw_rng_aarch64(void) {
  return CRYPTO_is_ARMv8_RNDR_capable();
}

#else  // defined(OPENSSL_AARCH64) && !defined(OPENSSL_NO_ASM)

OPENSSL_INLINE int CRYPTO_rndr_multiple8(uint8_t *out, size_t out_len) {
  return 0;
}

OPENSSL_INLINE int have_hw_rng_aarch64(void) {
  return 0;
}

#endif  // defined(OPENSSL_AARCH64) && !defined(OPENSSL_NO_ASM)

// rdrand_multiple8 writes |len| number of bytes to |buf| generated using the
// rdrand instruction. |len| must be a multiple of 8. Retries 
// Outputs 1 on success, 0 otherwise.
OPENSSL_EXPORT int rdrand_multiple8(uint8_t *buf, size_t len);

// have_hw_rng_x86_64_for_testing wraps |have_hw_rng_x86_64| to allow usage
// in testing.
OPENSSL_EXPORT int have_hw_rng_x86_64_for_testing(void);

#if defined(OPENSSL_X86_64) && !defined(OPENSSL_NO_ASM)

// Certain operating environments will disable RDRAND for both security and
// performance reasons. See initialization of CPU capability vector for details.
// At the moment, we must implement this logic there because the CPU capability
// vector does not carry CPU family/model information which is required to
// determine restrictions.
OPENSSL_INLINE int have_hw_rng_x86_64(void) {
  return CRYPTO_is_RDRAND_capable();
}

// CRYPTO_rdrand_multiple8 writes |len| number of bytes to |buf| generated using
// the rdrand instruction. |len| must be a multiple of 8 and positive.
// Outputs 1 on success, 0 otherwise.
int CRYPTO_rdrand_multiple8(uint8_t *buf, size_t len);

#else  // defined(OPENSSL_X86_64) && !defined(OPENSSL_NO_ASM)

OPENSSL_INLINE int CRYPTO_rdrand_multiple8(uint8_t *buf, size_t len) {
  return 0;
}

OPENSSL_INLINE int have_hw_rng_x86_64(void) {
  return 0;
}

#endif  // defined(OPENSSL_X86_64) && !defined(OPENSSL_NO_ASM)

struct test_tree_drbg_t {
  uint64_t thread_generate_calls_since_seed;
  uint64_t thread_reseed_calls_since_initialization;
  uint64_t global_generate_calls_since_seed;
  uint64_t global_reseed_calls_since_initialization;
};

// get_thread_and_global_tree_drbg_calls_FOR_TESTING returns the number of
// generate calls since seed/reseed for the tread-local and global tree-DRBG.
// In addition, it returns the number of reseeds applied on the tread-local and
// global tree-DRBG. These values of copied to |test_tree_drbg|.
OPENSSL_EXPORT int get_thread_and_global_tree_drbg_calls_FOR_TESTING(
  const struct entropy_source_t *entropy_source,
  struct test_tree_drbg_t *test_tree_drbg);

// set_thread_and_global_tree_drbg_reseed_counter_FOR_TESTING sets the reseed
// counter for either the tread-local and/or global tree-DRBG. If either of
// |thread_reseed_calls| or |global_reseed_calls| are equal to 0, their
// reseed counter is not set.
OPENSSL_EXPORT int set_thread_and_global_tree_drbg_reseed_counter_FOR_TESTING(
  struct entropy_source_t *entropy_source, uint64_t thread_reseed_calls,
  uint64_t global_reseed_calls);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_RAND_ENTROPY_INTERNAL_H
