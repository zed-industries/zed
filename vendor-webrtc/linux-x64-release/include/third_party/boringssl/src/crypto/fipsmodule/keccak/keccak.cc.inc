// Copyright 2023 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/base.h>

#include <assert.h>
#include <stdlib.h>

#include "../../internal.h"
#include "./internal.h"


// keccak_f implements the Keccak-1600 permutation as described at
// https://keccak.team/keccak_specs_summary.html. Each lane is represented as a
// 64-bit value and the 5×5 lanes are stored as an array in row-major order.
static void keccak_f(uint64_t state[25]) {
  static const int kNumRounds = 24;
  for (int round = 0; round < kNumRounds; round++) {
    // θ step
    uint64_t c[5];
    for (int x = 0; x < 5; x++) {
      c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^
             state[x + 20];
    }

    for (int x = 0; x < 5; x++) {
      const uint64_t d = c[(x + 4) % 5] ^ CRYPTO_rotl_u64(c[(x + 1) % 5], 1);
      for (int y = 0; y < 5; y++) {
        state[y * 5 + x] ^= d;
      }
    }

    // ρ and π steps.
    //
    // These steps involve a mapping of the state matrix. Each input point,
    // (x,y), is rotated and written to the point (y, 2x + 3y). In the Keccak
    // pseudo-code a separate array is used because an in-place operation would
    // overwrite some values that are subsequently needed. However, the mapping
    // forms a trail through 24 of the 25 values so we can do it in place with
    // only a single temporary variable.
    //
    // Start with (1, 0). The value here will be mapped and end up at (0, 2).
    // That value will end up at (2, 1), then (1, 2), and so on. After 24
    // steps, 24 of the 25 values have been hit (as this mapping is injective)
    // and the sequence will repeat. All that remains is to handle the element
    // at (0, 0), but the rotation for that element is zero, and it goes to (0,
    // 0), so we can ignore it.
    uint64_t prev_value = state[1];
#define PI_RHO_STEP(index, rotation)                              \
  do {                                                            \
    const uint64_t value = CRYPTO_rotl_u64(prev_value, rotation); \
    prev_value = state[index];                                    \
    state[index] = value;                                         \
  } while (0)

    PI_RHO_STEP(10, 1);
    PI_RHO_STEP(7, 3);
    PI_RHO_STEP(11, 6);
    PI_RHO_STEP(17, 10);
    PI_RHO_STEP(18, 15);
    PI_RHO_STEP(3, 21);
    PI_RHO_STEP(5, 28);
    PI_RHO_STEP(16, 36);
    PI_RHO_STEP(8, 45);
    PI_RHO_STEP(21, 55);
    PI_RHO_STEP(24, 2);
    PI_RHO_STEP(4, 14);
    PI_RHO_STEP(15, 27);
    PI_RHO_STEP(23, 41);
    PI_RHO_STEP(19, 56);
    PI_RHO_STEP(13, 8);
    PI_RHO_STEP(12, 25);
    PI_RHO_STEP(2, 43);
    PI_RHO_STEP(20, 62);
    PI_RHO_STEP(14, 18);
    PI_RHO_STEP(22, 39);
    PI_RHO_STEP(9, 61);
    PI_RHO_STEP(6, 20);
    PI_RHO_STEP(1, 44);

#undef PI_RHO_STEP

    // χ step
    for (int y = 0; y < 5; y++) {
      const int row_index = 5 * y;
      const uint64_t orig_x0 = state[row_index];
      const uint64_t orig_x1 = state[row_index + 1];
      state[row_index] ^= ~orig_x1 & state[row_index + 2];
      state[row_index + 1] ^= ~state[row_index + 2] & state[row_index + 3];
      state[row_index + 2] ^= ~state[row_index + 3] & state[row_index + 4];
      state[row_index + 3] ^= ~state[row_index + 4] & orig_x0;
      state[row_index + 4] ^= ~orig_x0 & orig_x1;
    }

    // ι step
    //
    // From https://keccak.team/files/Keccak-reference-3.0.pdf, section
    // 1.2, the round constants are based on the output of a LFSR. Thus, as
    // suggested in the appendix of of
    // https://keccak.team/keccak_specs_summary.html, the values are
    // simply encoded here.
    static const uint64_t kRoundConstants[24] = {
        0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
        0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
        0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
        0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
        0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
        0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
        0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
    };

    state[0] ^= kRoundConstants[round];
  }
}

static void keccak_init(struct BORINGSSL_keccak_st *ctx,
                        size_t *out_required_out_len,
                        enum boringssl_keccak_config_t config) {
  size_t capacity_bytes;
  switch (config) {
    case boringssl_sha3_256:
      capacity_bytes = 512 / 8;
      *out_required_out_len = 32;
      break;
    case boringssl_sha3_512:
      capacity_bytes = 1024 / 8;
      *out_required_out_len = 64;
      break;
    case boringssl_shake128:
      capacity_bytes = 256 / 8;
      *out_required_out_len = 0;
      break;
    case boringssl_shake256:
      capacity_bytes = 512 / 8;
      *out_required_out_len = 0;
      break;
    default:
      abort();
  }

  OPENSSL_memset(ctx, 0, sizeof(*ctx));
  ctx->config = config;
  ctx->phase = boringssl_keccak_phase_absorb;
  ctx->rate_bytes = 200 - capacity_bytes;
  assert(ctx->rate_bytes % 8 == 0);
}

void BORINGSSL_keccak(uint8_t *out, size_t out_len, const uint8_t *in,
                      size_t in_len, enum boringssl_keccak_config_t config) {
  struct BORINGSSL_keccak_st ctx;
  size_t required_out_len;
  keccak_init(&ctx, &required_out_len, config);
  if (required_out_len != 0 && out_len != required_out_len) {
    abort();
  }
  BORINGSSL_keccak_absorb(&ctx, in, in_len);
  BORINGSSL_keccak_squeeze(&ctx, out, out_len);
}

void BORINGSSL_keccak_init(struct BORINGSSL_keccak_st *ctx,
                           enum boringssl_keccak_config_t config) {
  size_t required_out_len;
  keccak_init(ctx, &required_out_len, config);
  if (required_out_len != 0) {
    abort();
  }
}

void BORINGSSL_keccak_absorb(struct BORINGSSL_keccak_st *ctx, const uint8_t *in,
                             size_t in_len) {
  if (ctx->phase == boringssl_keccak_phase_squeeze) {
    // It's illegal to call absorb() again after calling squeeze().
    abort();
  }

  const size_t rate_words = ctx->rate_bytes / 8;
  // XOR the input. Accessing |ctx->state| as a |uint8_t*| is allowed by strict
  // aliasing because we require |uint8_t| to be a character type.
  uint8_t *state_bytes = (uint8_t *)ctx->state;

  // Absorb partial block.
  if (ctx->absorb_offset != 0) {
    assert(ctx->absorb_offset < ctx->rate_bytes);
    size_t first_block_len = ctx->rate_bytes - ctx->absorb_offset;
    for (size_t i = 0; i < first_block_len && i < in_len; i++) {
      state_bytes[ctx->absorb_offset + i] ^= in[i];
    }

    // This input didn't fill the block.
    if (first_block_len > in_len) {
      ctx->absorb_offset += in_len;
      return;
    }

    keccak_f(ctx->state);
    in += first_block_len;
    in_len -= first_block_len;
  }

  // Absorb full blocks.
  while (in_len >= ctx->rate_bytes) {
    for (size_t i = 0; i < rate_words; i++) {
      ctx->state[i] ^= CRYPTO_load_u64_le(in + 8 * i);
    }
    keccak_f(ctx->state);
    in += ctx->rate_bytes;
    in_len -= ctx->rate_bytes;
  }

  // Absorb partial block.
  assert(in_len < ctx->rate_bytes);
  for (size_t i = 0; i < in_len; i++) {
    state_bytes[i] ^= in[i];
  }
  ctx->absorb_offset = in_len;
}

static void keccak_finalize(struct BORINGSSL_keccak_st *ctx) {
  uint8_t terminator;
  switch (ctx->config) {
    case boringssl_sha3_256:
    case boringssl_sha3_512:
      terminator = 0x06;
      break;
    case boringssl_shake128:
    case boringssl_shake256:
      terminator = 0x1f;
      break;
    default:
      abort();
  }

  // XOR the terminator. Accessing |ctx->state| as a |uint8_t*| is allowed by
  // strict aliasing because we require |uint8_t| to be a character type.
  uint8_t *state_bytes = (uint8_t *)ctx->state;
  state_bytes[ctx->absorb_offset] ^= terminator;
  state_bytes[ctx->rate_bytes - 1] ^= 0x80;
  keccak_f(ctx->state);
}

void BORINGSSL_keccak_squeeze(struct BORINGSSL_keccak_st *ctx, uint8_t *out,
                              size_t out_len) {
  if (ctx->phase == boringssl_keccak_phase_absorb) {
    keccak_finalize(ctx);
    ctx->phase = boringssl_keccak_phase_squeeze;
  }

  // Accessing |ctx->state| as a |uint8_t*| is allowed by strict aliasing
  // because we require |uint8_t| to be a character type.
  const uint8_t *state_bytes = (const uint8_t *)ctx->state;
  while (out_len) {
    if (ctx->squeeze_offset == ctx->rate_bytes) {
      keccak_f(ctx->state);
      ctx->squeeze_offset = 0;
    }

    size_t remaining = ctx->rate_bytes - ctx->squeeze_offset;
    size_t todo = out_len;
    if (todo > remaining) {
      todo = remaining;
    }
    OPENSSL_memcpy(out, &state_bytes[ctx->squeeze_offset], todo);
    out += todo;
    out_len -= todo;
    ctx->squeeze_offset += todo;
  }
}
