// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#include <assert.h>
#include <string.h>

#include "internal.h"
#include "../../internal.h"


// byte_reverse reverses the order of the bytes in |b->c|.
static void byte_reverse(uint8_t b[16]) {
  uint64_t hi = CRYPTO_load_u64_le(b);
  uint64_t lo = CRYPTO_load_u64_le(b + 8);
  CRYPTO_store_u64_le(b, CRYPTO_bswap8(lo));
  CRYPTO_store_u64_le(b + 8, CRYPTO_bswap8(hi));
}

// reverse_and_mulX_ghash interprets |b| as a reversed element of the GHASH
// field, multiplies that by 'x' and serialises the result back into |b|, but
// with GHASH's backwards bit ordering.
static void reverse_and_mulX_ghash(uint8_t b[16]) {
  uint64_t hi = CRYPTO_load_u64_le(b);
  uint64_t lo = CRYPTO_load_u64_le(b + 8);
  const crypto_word_t carry = constant_time_eq_w(hi & 1, 1);
  hi >>= 1;
  hi |= lo << 63;
  lo >>= 1;
  lo ^= ((uint64_t) constant_time_select_w(carry, 0xe1, 0)) << 56;

  CRYPTO_store_u64_le(b, CRYPTO_bswap8(lo));
  CRYPTO_store_u64_le(b + 8, CRYPTO_bswap8(hi));
}

// POLYVAL(H, X_1, ..., X_n) =
// ByteReverse(GHASH(mulX_GHASH(ByteReverse(H)), ByteReverse(X_1), ...,
// ByteReverse(X_n))).
//
// See https://www.rfc-editor.org/rfc/rfc8452.html#appendix-A.

void CRYPTO_POLYVAL_init(struct polyval_ctx *ctx, const uint8_t key[16]) {
  alignas(8) uint8_t H[16];
  OPENSSL_memcpy(H, key, 16);
  reverse_and_mulX_ghash(H);

  int is_avx;
  CRYPTO_ghash_init(&ctx->gmult, &ctx->ghash, ctx->Htable, &is_avx, H);
  OPENSSL_memset(&ctx->S, 0, sizeof(ctx->S));
}

void CRYPTO_POLYVAL_update_blocks(struct polyval_ctx *ctx, const uint8_t *in,
                                  size_t in_len) {
  assert((in_len & 15) == 0);
  alignas(8) uint8_t buf[32 * 16];

  while (in_len > 0) {
    size_t todo = in_len;
    if (todo > sizeof(buf)) {
      todo = sizeof(buf);
    }
    OPENSSL_memcpy(buf, in, todo);
    in += todo;
    in_len -= todo;

    size_t blocks = todo / 16;
    for (size_t i = 0; i < blocks; i++) {
      byte_reverse(buf + 16 * i);
    }

    ctx->ghash(ctx->S, ctx->Htable, buf, todo);
  }
}

void CRYPTO_POLYVAL_finish(const struct polyval_ctx *ctx, uint8_t out[16]) {
  OPENSSL_memcpy(out, &ctx->S, 16);
  byte_reverse(out);
}
