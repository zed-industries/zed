// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef MLD_AWSLC_FIPS202_GLUE_H
#define MLD_AWSLC_FIPS202_GLUE_H
#include <stddef.h>
#include <stdint.h>

#include "../sha/internal.h"

#define SHAKE128_RATE 168
#define SHAKE256_RATE 136
#define SHA3_256_RATE 136
#define SHA3_512_RATE 72

#define mld_shake128ctx KECCAK1600_CTX
#define mld_shake256ctx KECCAK1600_CTX

static MLD_INLINE void mld_shake128_init(mld_shake128ctx *state) {
  // Return code checks can be omitted
  // SHAKE_Init always returns 1 when called with correct block size value.
  (void) SHAKE_Init(state, SHAKE128_BLOCKSIZE);
}

static MLD_INLINE void mld_shake128_release(mld_shake128ctx *state) {
  (void) state;
}

static MLD_INLINE void mld_shake128_absorb_once(mld_shake128ctx *state,
						const uint8_t *input, size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE_Absorb(state, input, inlen);
}

static MLD_INLINE void mld_shake128_absorb(mld_shake128ctx *state,
					   const uint8_t *input, size_t inlen) {
  (void) SHAKE_Absorb(state, input, inlen);
}

static MLD_INLINE void mld_shake128_finalize(mld_shake128ctx *state) {
  // Finalization is implicit in AWS-LC's implementation
  // The state is ready for squeezing after absorb
  (void) state;
}

static MLD_INLINE void mld_shake128_squeeze(uint8_t *output, size_t outlen,
					    mld_shake128ctx *state) {
  (void) SHAKE_Squeeze(output, state, outlen);
}

static MLD_INLINE void mld_shake128_squeezeblocks(uint8_t *output, size_t nblocks,
						  mld_shake128ctx *state) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE_Squeeze(output, state, nblocks * SHAKE128_RATE);
}

static MLD_INLINE void mld_shake256_init(mld_shake256ctx *state) {
  // Return code checks can be omitted
  // SHAKE_Init always returns 1 when called with correct block size value.
  (void) SHAKE_Init(state, SHAKE256_BLOCKSIZE);
}

static MLD_INLINE void mld_shake256_release(mld_shake256ctx *state) {
  (void) state;
}

static MLD_INLINE void mld_shake256_absorb_once(mld_shake256ctx *state,
						const uint8_t *input, size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE_Absorb(state, input, inlen);
}

static MLD_INLINE void mld_shake256_absorb(mld_shake256ctx *state,
					   const uint8_t *input, size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE_Absorb(state, input, inlen);
}

static MLD_INLINE void mld_shake256_finalize(mld_shake256ctx *state) {
  // Finalization is implicit in AWS-LC's implementation
  // The state is ready for squeezing after absorb
  (void) state;
}

static MLD_INLINE void mld_shake256_squeeze(uint8_t *output, size_t outlen,
					    mld_shake256ctx *state) {
  (void) SHAKE_Squeeze(output, state, outlen);
}

static MLD_INLINE void mld_shake256_squeezeblocks(uint8_t *output, size_t nblocks,
						  mld_shake256ctx *state) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE_Squeeze(output, state, nblocks * SHAKE256_RATE);
}

static MLD_INLINE void mld_shake256(uint8_t *output, size_t outlen,
				    const uint8_t *input, size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE256(input, inlen, output, outlen);
}

static MLD_INLINE void mld_sha3_256(uint8_t *output, const uint8_t *input,
				    size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHA3_256(input, inlen, output);
}

static MLD_INLINE void mld_sha3_512(uint8_t *output, const uint8_t *input,
				    size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHA3_512(input, inlen, output);
}

#endif // MLD_AWSLC_FIPS202_GLUE_H
