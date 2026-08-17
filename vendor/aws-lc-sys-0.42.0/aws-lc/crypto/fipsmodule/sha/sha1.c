// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/sha.h>

#include <string.h>

#include <openssl/mem.h>

#include "../../internal.h"
#include "../digest/md32_common.h"
#include "internal.h"


int SHA1_Init(SHA_CTX *sha) {
  OPENSSL_memset(sha, 0, sizeof(SHA_CTX));
  sha->h[0] = 0x67452301UL;
  sha->h[1] = 0xefcdab89UL;
  sha->h[2] = 0x98badcfeUL;
  sha->h[3] = 0x10325476UL;
  sha->h[4] = 0xc3d2e1f0UL;
  return 1;
}

int SHA1_Init_from_state(SHA_CTX *sha, const uint8_t h[SHA1_CHAINING_LENGTH],
                         uint64_t n) {
  if (n % ((uint64_t)SHA_CBLOCK * 8) != 0) {
    // n is not a multiple of the block size in bits, so it fails
    return 0;
  }

  OPENSSL_memset(sha, 0, sizeof(SHA_CTX));

  const size_t out_words = SHA1_CHAINING_LENGTH / 4;
  for (size_t i = 0; i < out_words; i++) {
    sha->h[i] = CRYPTO_load_u32_be(h);
    h += 4;
  }

  sha->Nh = n >> 32;
  sha->Nl = n & 0xffffffff;

  return 1;
}

uint8_t *SHA1(const uint8_t *data, size_t len, uint8_t out[SHA_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA_CTX ctx;
  const int ok = SHA1_Init(&ctx) &&
                 SHA1_Update(&ctx, data, len) &&
                 SHA1_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

#if !defined(SHA1_ASM) && !defined(SHA1_ALTIVEC)
static void sha1_block_data_order(uint32_t state[5], const uint8_t *data,
                                  size_t num);
#endif

void SHA1_Transform(SHA_CTX *c, const uint8_t data[SHA_CBLOCK]) {
  sha1_block_data_order(c->h, data, 1);
}

int SHA1_Update(SHA_CTX *c, const void *data, size_t len) {
  crypto_md32_update(&sha1_block_data_order, c->h, c->data, SHA_CBLOCK, &c->num,
                     &c->Nh, &c->Nl, data, len);
  return 1;
}

int SHA1_Final(uint8_t out[SHA_DIGEST_LENGTH], SHA_CTX *c) {
  crypto_md32_final(&sha1_block_data_order, c->h, c->data, SHA_CBLOCK, &c->num,
                    c->Nh, c->Nl, /*is_big_endian=*/1);

  CRYPTO_store_u32_be(out, c->h[0]);
  CRYPTO_store_u32_be(out + 4, c->h[1]);
  CRYPTO_store_u32_be(out + 8, c->h[2]);
  CRYPTO_store_u32_be(out + 12, c->h[3]);
  CRYPTO_store_u32_be(out + 16, c->h[4]);
  FIPS_service_indicator_update_state();
  return 1;
}

int SHA1_get_state(SHA_CTX *ctx, uint8_t out_h[SHA1_CHAINING_LENGTH],
                   uint64_t *out_n) {
  if (ctx->Nl % ((uint64_t)SHA_CBLOCK * 8) != 0) {
    // ctx->Nl is not a multiple of the block size in bits, so it fails
    return 0;
  }

  const size_t out_words = SHA1_CHAINING_LENGTH / 4;
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u32_be(out_h, ctx->h[i]);
    out_h += 4;
  }

  *out_n = (((uint64_t)ctx->Nh) << 32) + ctx->Nl;

  return 1;
}

#define Xupdate(a, ix, ia, ib, ic, id)    \
  do {                                    \
    (a) = ((ia) ^ (ib) ^ (ic) ^ (id));    \
    (ix) = (a) = CRYPTO_rotl_u32((a), 1); \
  } while (0)

#define K_00_19 0x5a827999UL
#define K_20_39 0x6ed9eba1UL
#define K_40_59 0x8f1bbcdcUL
#define K_60_79 0xca62c1d6UL

// As  pointed out by Wei Dai <weidai@eskimo.com>, F() below can be simplified
// to the code in F_00_19.  Wei attributes these optimisations to Peter
// Gutmann's SHS code, and he attributes it to Rich Schroeppel. #define
// F(x,y,z) (((x) & (y))  |  ((~(x)) & (z))) I've just become aware of another
// tweak to be made, again from Wei Dai, in F_40_59, (x&a)|(y&a) -> (x|y)&a
#define F_00_19(b, c, d) ((((c) ^ (d)) & (b)) ^ (d))
#define F_20_39(b, c, d) ((b) ^ (c) ^ (d))
#define F_40_59(b, c, d) (((b) & (c)) | (((b) | (c)) & (d)))
#define F_60_79(b, c, d) F_20_39(b, c, d)

#define BODY_00_15(i, a, b, c, d, e, f, xi)                \
  do {                                                     \
    (f) = (xi) + (e) + K_00_19 + CRYPTO_rotl_u32((a), 5) + \
          F_00_19((b), (c), (d));                          \
    (b) = CRYPTO_rotl_u32((b), 30);                        \
  } while (0)

#define BODY_16_19(i, a, b, c, d, e, f, xi, xa, xb, xc, xd)                  \
  do {                                                                       \
    Xupdate(f, xi, xa, xb, xc, xd);                                          \
    (f) += (e) + K_00_19 + CRYPTO_rotl_u32((a), 5) + F_00_19((b), (c), (d)); \
    (b) = CRYPTO_rotl_u32((b), 30);                                          \
  } while (0)

#define BODY_20_31(i, a, b, c, d, e, f, xi, xa, xb, xc, xd)                  \
  do {                                                                       \
    Xupdate(f, xi, xa, xb, xc, xd);                                          \
    (f) += (e) + K_20_39 + CRYPTO_rotl_u32((a), 5) + F_20_39((b), (c), (d)); \
    (b) = CRYPTO_rotl_u32((b), 30);                                          \
  } while (0)

#define BODY_32_39(i, a, b, c, d, e, f, xa, xb, xc, xd)                      \
  do {                                                                       \
    Xupdate(f, xa, xa, xb, xc, xd);                                          \
    (f) += (e) + K_20_39 + CRYPTO_rotl_u32((a), 5) + F_20_39((b), (c), (d)); \
    (b) = CRYPTO_rotl_u32((b), 30);                                          \
  } while (0)

#define BODY_40_59(i, a, b, c, d, e, f, xa, xb, xc, xd)                      \
  do {                                                                       \
    Xupdate(f, xa, xa, xb, xc, xd);                                          \
    (f) += (e) + K_40_59 + CRYPTO_rotl_u32((a), 5) + F_40_59((b), (c), (d)); \
    (b) = CRYPTO_rotl_u32((b), 30);                                          \
  } while (0)

#define BODY_60_79(i, a, b, c, d, e, f, xa, xb, xc, xd)    \
  do {                                                     \
    Xupdate(f, xa, xa, xb, xc, xd);                        \
    (f) = (xa) + (e) + K_60_79 + CRYPTO_rotl_u32((a), 5) + \
          F_60_79((b), (c), (d));                          \
    (b) = CRYPTO_rotl_u32((b), 30);                        \
  } while (0)

#ifdef X
#undef X
#endif

/* Originally X was an array. As it's automatic it's natural
* to expect RISC compiler to accomodate at least part of it in
* the register bank, isn't it? Unfortunately not all compilers
* "find" this expectation reasonable:-( On order to make such
* compilers generate better code I replace X[] with a bunch of
* X0, X1, etc. See the function body below...
*         <appro@fy.chalmers.se> */
#define X(i)  XX##i

#if !defined(SHA1_ASM) && !defined(SHA1_ALTIVEC)

#if !defined(SHA1_ASM_NOHW)
static void sha1_block_data_order_nohw(uint32_t state[5], const uint8_t *data,
                                       size_t num) {
  register uint32_t A, B, C, D, E, T;
  uint32_t XX0, XX1, XX2, XX3, XX4, XX5, XX6, XX7, XX8, XX9, XX10,
      XX11, XX12, XX13, XX14, XX15;

  A = state[0];
  B = state[1];
  C = state[2];
  D = state[3];
  E = state[4];

  for (;;) {
    X(0) = CRYPTO_load_u32_be(data);
    data += 4;
    X(1) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(0, A, B, C, D, E, T, X(0));
    X(2) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(1, T, A, B, C, D, E, X(1));
    X(3) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(2, E, T, A, B, C, D, X(2));
    X(4) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(3, D, E, T, A, B, C, X(3));
    X(5) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(4, C, D, E, T, A, B, X(4));
    X(6) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(5, B, C, D, E, T, A, X(5));
    X(7) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(6, A, B, C, D, E, T, X(6));
    X(8) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(7, T, A, B, C, D, E, X(7));
    X(9) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(8, E, T, A, B, C, D, X(8));
    X(10) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(9, D, E, T, A, B, C, X(9));
    X(11) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(10, C, D, E, T, A, B, X(10));
    X(12) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(11, B, C, D, E, T, A, X(11));
    X(13) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(12, A, B, C, D, E, T, X(12));
    X(14) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(13, T, A, B, C, D, E, X(13));
    X(15) = CRYPTO_load_u32_be(data);
    data += 4;
    BODY_00_15(14, E, T, A, B, C, D, X(14));
    BODY_00_15(15, D, E, T, A, B, C, X(15));

    BODY_16_19(16, C, D, E, T, A, B, X(0), X(0), X(2), X(8), X(13));
    BODY_16_19(17, B, C, D, E, T, A, X(1), X(1), X(3), X(9), X(14));
    BODY_16_19(18, A, B, C, D, E, T, X(2), X(2), X(4), X(10), X(15));
    BODY_16_19(19, T, A, B, C, D, E, X(3), X(3), X(5), X(11), X(0));

    BODY_20_31(20, E, T, A, B, C, D, X(4), X(4), X(6), X(12), X(1));
    BODY_20_31(21, D, E, T, A, B, C, X(5), X(5), X(7), X(13), X(2));
    BODY_20_31(22, C, D, E, T, A, B, X(6), X(6), X(8), X(14), X(3));
    BODY_20_31(23, B, C, D, E, T, A, X(7), X(7), X(9), X(15), X(4));
    BODY_20_31(24, A, B, C, D, E, T, X(8), X(8), X(10), X(0), X(5));
    BODY_20_31(25, T, A, B, C, D, E, X(9), X(9), X(11), X(1), X(6));
    BODY_20_31(26, E, T, A, B, C, D, X(10), X(10), X(12), X(2), X(7));
    BODY_20_31(27, D, E, T, A, B, C, X(11), X(11), X(13), X(3), X(8));
    BODY_20_31(28, C, D, E, T, A, B, X(12), X(12), X(14), X(4), X(9));
    BODY_20_31(29, B, C, D, E, T, A, X(13), X(13), X(15), X(5), X(10));
    BODY_20_31(30, A, B, C, D, E, T, X(14), X(14), X(0), X(6), X(11));
    BODY_20_31(31, T, A, B, C, D, E, X(15), X(15), X(1), X(7), X(12));

    BODY_32_39(32, E, T, A, B, C, D, X(0), X(2), X(8), X(13));
    BODY_32_39(33, D, E, T, A, B, C, X(1), X(3), X(9), X(14));
    BODY_32_39(34, C, D, E, T, A, B, X(2), X(4), X(10), X(15));
    BODY_32_39(35, B, C, D, E, T, A, X(3), X(5), X(11), X(0));
    BODY_32_39(36, A, B, C, D, E, T, X(4), X(6), X(12), X(1));
    BODY_32_39(37, T, A, B, C, D, E, X(5), X(7), X(13), X(2));
    BODY_32_39(38, E, T, A, B, C, D, X(6), X(8), X(14), X(3));
    BODY_32_39(39, D, E, T, A, B, C, X(7), X(9), X(15), X(4));

    BODY_40_59(40, C, D, E, T, A, B, X(8), X(10), X(0), X(5));
    BODY_40_59(41, B, C, D, E, T, A, X(9), X(11), X(1), X(6));
    BODY_40_59(42, A, B, C, D, E, T, X(10), X(12), X(2), X(7));
    BODY_40_59(43, T, A, B, C, D, E, X(11), X(13), X(3), X(8));
    BODY_40_59(44, E, T, A, B, C, D, X(12), X(14), X(4), X(9));
    BODY_40_59(45, D, E, T, A, B, C, X(13), X(15), X(5), X(10));
    BODY_40_59(46, C, D, E, T, A, B, X(14), X(0), X(6), X(11));
    BODY_40_59(47, B, C, D, E, T, A, X(15), X(1), X(7), X(12));
    BODY_40_59(48, A, B, C, D, E, T, X(0), X(2), X(8), X(13));
    BODY_40_59(49, T, A, B, C, D, E, X(1), X(3), X(9), X(14));
    BODY_40_59(50, E, T, A, B, C, D, X(2), X(4), X(10), X(15));
    BODY_40_59(51, D, E, T, A, B, C, X(3), X(5), X(11), X(0));
    BODY_40_59(52, C, D, E, T, A, B, X(4), X(6), X(12), X(1));
    BODY_40_59(53, B, C, D, E, T, A, X(5), X(7), X(13), X(2));
    BODY_40_59(54, A, B, C, D, E, T, X(6), X(8), X(14), X(3));
    BODY_40_59(55, T, A, B, C, D, E, X(7), X(9), X(15), X(4));
    BODY_40_59(56, E, T, A, B, C, D, X(8), X(10), X(0), X(5));
    BODY_40_59(57, D, E, T, A, B, C, X(9), X(11), X(1), X(6));
    BODY_40_59(58, C, D, E, T, A, B, X(10), X(12), X(2), X(7));
    BODY_40_59(59, B, C, D, E, T, A, X(11), X(13), X(3), X(8));

    BODY_60_79(60, A, B, C, D, E, T, X(12), X(14), X(4), X(9));
    BODY_60_79(61, T, A, B, C, D, E, X(13), X(15), X(5), X(10));
    BODY_60_79(62, E, T, A, B, C, D, X(14), X(0), X(6), X(11));
    BODY_60_79(63, D, E, T, A, B, C, X(15), X(1), X(7), X(12));
    BODY_60_79(64, C, D, E, T, A, B, X(0), X(2), X(8), X(13));
    BODY_60_79(65, B, C, D, E, T, A, X(1), X(3), X(9), X(14));
    BODY_60_79(66, A, B, C, D, E, T, X(2), X(4), X(10), X(15));
    BODY_60_79(67, T, A, B, C, D, E, X(3), X(5), X(11), X(0));
    BODY_60_79(68, E, T, A, B, C, D, X(4), X(6), X(12), X(1));
    BODY_60_79(69, D, E, T, A, B, C, X(5), X(7), X(13), X(2));
    BODY_60_79(70, C, D, E, T, A, B, X(6), X(8), X(14), X(3));
    BODY_60_79(71, B, C, D, E, T, A, X(7), X(9), X(15), X(4));
    BODY_60_79(72, A, B, C, D, E, T, X(8), X(10), X(0), X(5));
    BODY_60_79(73, T, A, B, C, D, E, X(9), X(11), X(1), X(6));
    BODY_60_79(74, E, T, A, B, C, D, X(10), X(12), X(2), X(7));
    BODY_60_79(75, D, E, T, A, B, C, X(11), X(13), X(3), X(8));
    BODY_60_79(76, C, D, E, T, A, B, X(12), X(14), X(4), X(9));
    BODY_60_79(77, B, C, D, E, T, A, X(13), X(15), X(5), X(10));
    BODY_60_79(78, A, B, C, D, E, T, X(14), X(0), X(6), X(11));
    BODY_60_79(79, T, A, B, C, D, E, X(15), X(1), X(7), X(12));

    state[0] = (state[0] + E) & 0xffffffffL;
    state[1] = (state[1] + T) & 0xffffffffL;
    state[2] = (state[2] + A) & 0xffffffffL;
    state[3] = (state[3] + B) & 0xffffffffL;
    state[4] = (state[4] + C) & 0xffffffffL;

    if (--num == 0) {
      break;
    }

    A = state[0];
    B = state[1];
    C = state[2];
    D = state[3];
    E = state[4];
  }
}
#endif  // !SHA1_ASM_NOHW

static void sha1_block_data_order(uint32_t state[5], const uint8_t *data,
                                  size_t num) {
#if defined(SHA1_ASM_HW)
  if (sha1_hw_capable()) {
    sha1_block_data_order_hw(state, data, num);
    return;
  }
#endif
#if defined(SHA1_ASM_AVX2) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  if (sha1_avx2_capable()) {
    sha1_block_data_order_avx2(state, data, num);
    return;
  }
#endif
#if defined(SHA1_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  if (sha1_avx_capable()) {
    sha1_block_data_order_avx(state, data, num);
    return;
  }
#endif
#if defined(SHA1_ASM_SSSE3)
  if (sha1_ssse3_capable()) {
    sha1_block_data_order_ssse3(state, data, num);
    return;
  }
#endif
#if defined(SHA1_ASM_NEON)
  if (CRYPTO_is_NEON_capable()) {
    sha1_block_data_order_neon(state, data, num);
    return;
  }
#endif
  sha1_block_data_order_nohw(state, data, num);
}

#endif  // !SHA1_ASM && !SHA1_ALTIVEC

#undef Xupdate
#undef K_00_19
#undef K_20_39
#undef K_40_59
#undef K_60_79
#undef F_00_19
#undef F_20_39
#undef F_40_59
#undef F_60_79
#undef BODY_00_15
#undef BODY_16_19
#undef BODY_20_31
#undef BODY_32_39
#undef BODY_40_59
#undef BODY_60_79
#undef X
