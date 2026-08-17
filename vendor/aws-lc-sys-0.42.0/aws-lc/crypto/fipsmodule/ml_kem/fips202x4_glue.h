// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//
// This is a shim establishing the FIPS-202 API required by
// mlkem-native from the API exposed by AWS-LC.
//

#ifndef MLK_AWSLC_FIPS202X4_GLUE_H
#define MLK_AWSLC_FIPS202X4_GLUE_H

#include <stddef.h>
#include <stdint.h>

#include "fips202_glue.h"

#define mlk_shake128x4ctx KECCAK1600_CTX_x4

static MLK_INLINE void mlk_shake128x4_absorb_once(mlk_shake128x4ctx *state,
						  const uint8_t *in0,
						  const uint8_t *in1,
						  const uint8_t *in2,
						  const uint8_t *in3, size_t inlen) {
  // Return code check can be omitted
  // since mlkem-native adheres to call discipline
  (void) SHAKE128_Absorb_once_x4(state, in0, in1, in2, in3, inlen);
}

static MLK_INLINE void mlk_shake128x4_squeezeblocks(uint8_t *out0, uint8_t *out1,
						    uint8_t *out2, uint8_t *out3,
						    size_t nblocks,
						    mlk_shake128x4ctx *state) {
  // Return code check can be omitted
  // since mlkem-native adheres to call discipline
  (void) SHAKE128_Squeezeblocks_x4(out0, out1, out2, out3, state, nblocks);
}

static MLK_INLINE void mlk_shake128x4_init(mlk_shake128x4ctx *state) {
  // Return code check can be omitted
  // since mlkem-native adheres to call discipline
  (void) SHAKE128_Init_x4(state);
}

static MLK_INLINE void mlk_shake128x4_release(mlk_shake128x4ctx *state) {
  (void) state;
}

static MLK_INLINE void mlk_shake256x4(uint8_t *out0, uint8_t *out1, uint8_t *out2,
				      uint8_t *out3, size_t outlen, uint8_t *in0,
				      uint8_t *in1, uint8_t *in2, uint8_t *in3,
				      size_t inlen) {
  // Return code check can be omitted
  // since SHAKE256_x4 is documented not to fail for valid inputs.
  (void) SHAKE256_x4(in0, in1, in2, in3, inlen,
		     out0, out1, out2, out3, outlen);
}

#endif // MLK_AWSLC_FIPS202X4_GLUE_H
