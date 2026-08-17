// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//
// This is a shim establishing the FIPS-202 API required by
// mldsa-native from the API exposed by AWS-LC.
//

#ifndef MLD_AWSLC_FIPS202X4_GLUE_H
#define MLD_AWSLC_FIPS202X4_GLUE_H

#include <stddef.h>
#include <stdint.h>

#include "fips202_glue.h"

// Use AWS-LC's existing KECCAK1600_CTX_x4 structure for SHAKE128
#define mld_shake128x4ctx KECCAK1600_CTX_x4

// For SHAKE256 x4, we need a custom structure since AWS-LC only has batched SHAKE128
typedef struct mld_shake256x4ctx_s {
  KECCAK1600_CTX s[4];
} mld_shake256x4ctx;

static MLD_INLINE void mld_shake128x4_absorb_once(mld_shake128x4ctx *state,
						  const uint8_t *in0,
						  const uint8_t *in1,
						  const uint8_t *in2,
						  const uint8_t *in3, size_t inlen) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE128_Absorb_once_x4(state, in0, in1, in2, in3, inlen);
}

static MLD_INLINE void mld_shake128x4_squeezeblocks(uint8_t *out0, uint8_t *out1,
						    uint8_t *out2, uint8_t *out3,
						    size_t nblocks,
						    mld_shake128x4ctx *state) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE128_Squeezeblocks_x4(out0, out1, out2, out3, state, nblocks);
}

static MLD_INLINE void mld_shake128x4_init(mld_shake128x4ctx *state) {
  // Return code check can be omitted
  // since mldsa-native adheres to call discipline
  (void) SHAKE128_Init_x4(state);
}

static MLD_INLINE void mld_shake128x4_release(mld_shake128x4ctx *state) {
  (void) state;
}

// AWS-LC doesn't have SHAKE256 x4 batched operations like it does for SHAKE128
// We provide serial implementations that process each instance separately
static MLD_INLINE void mld_shake256x4_absorb_once(mld_shake256x4ctx *state,
						  const uint8_t *in0,
						  const uint8_t *in1,
						  const uint8_t *in2,
						  const uint8_t *in3, size_t inlen) {
  // Process four independent SHAKE256 operations serially
  mld_shake256_init(&state->s[0]);
  mld_shake256_absorb_once(&state->s[0], in0, inlen);
  mld_shake256_init(&state->s[1]);
  mld_shake256_absorb_once(&state->s[1], in1, inlen);
  mld_shake256_init(&state->s[2]);
  mld_shake256_absorb_once(&state->s[2], in2, inlen);
  mld_shake256_init(&state->s[3]);
  mld_shake256_absorb_once(&state->s[3], in3, inlen);
}

static MLD_INLINE void mld_shake256x4_squeezeblocks(uint8_t *out0, uint8_t *out1,
						    uint8_t *out2, uint8_t *out3,
						    size_t nblocks,
						    mld_shake256x4ctx *state) {
  // Process four independent squeeze operations serially
  mld_shake256_squeezeblocks(out0, nblocks, &state->s[0]);
  mld_shake256_squeezeblocks(out1, nblocks, &state->s[1]);
  mld_shake256_squeezeblocks(out2, nblocks, &state->s[2]);
  mld_shake256_squeezeblocks(out3, nblocks, &state->s[3]);
}

static MLD_INLINE void mld_shake256x4_init(mld_shake256x4ctx *state) {
  // Initialize four independent states
  mld_shake256_init(&state->s[0]);
  mld_shake256_init(&state->s[1]);
  mld_shake256_init(&state->s[2]);
  mld_shake256_init(&state->s[3]);
}

static MLD_INLINE void mld_shake256x4_release(mld_shake256x4ctx *state) {
  (void) state;
}

static MLD_INLINE void mld_shake256x4(uint8_t *out0, uint8_t *out1, uint8_t *out2,
				      uint8_t *out3, size_t outlen, uint8_t *in0,
				      uint8_t *in1, uint8_t *in2, uint8_t *in3,
				      size_t inlen) {
  // Process four independent SHAKE256 operations serially
  mld_shake256(out0, outlen, in0, inlen);
  mld_shake256(out1, outlen, in1, inlen);
  mld_shake256(out2, outlen, in2, inlen);
  mld_shake256(out3, outlen, in3, inlen);
}

#endif // MLD_AWSLC_FIPS202X4_GLUE_H
