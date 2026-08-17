// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/sha.h>

#include <string.h>

#include <openssl/mem.h>

#include "../../internal.h"
#include "../digest/md32_common.h"
#include "internal.h"


int SHA224_Init(SHA256_CTX *sha) {
  OPENSSL_memset(sha, 0, sizeof(SHA256_CTX));
  sha->h[0] = 0xc1059ed8UL;
  sha->h[1] = 0x367cd507UL;
  sha->h[2] = 0x3070dd17UL;
  sha->h[3] = 0xf70e5939UL;
  sha->h[4] = 0xffc00b31UL;
  sha->h[5] = 0x68581511UL;
  sha->h[6] = 0x64f98fa7UL;
  sha->h[7] = 0xbefa4fa4UL;
  sha->md_len = SHA224_DIGEST_LENGTH;
  return 1;
}

int SHA256_Init(SHA256_CTX *sha) {
  OPENSSL_memset(sha, 0, sizeof(SHA256_CTX));
  sha->h[0] = 0x6a09e667UL;
  sha->h[1] = 0xbb67ae85UL;
  sha->h[2] = 0x3c6ef372UL;
  sha->h[3] = 0xa54ff53aUL;
  sha->h[4] = 0x510e527fUL;
  sha->h[5] = 0x9b05688cUL;
  sha->h[6] = 0x1f83d9abUL;
  sha->h[7] = 0x5be0cd19UL;
  sha->md_len = SHA256_DIGEST_LENGTH;
  return 1;
}

OPENSSL_STATIC_ASSERT(SHA256_CHAINING_LENGTH==SHA224_CHAINING_LENGTH,
                      sha256_and_sha224_have_same_chaining_length)

// sha256_init_from_state_impl is the implementation of
// SHA256_Init_from_state and SHA224_Init_from_state
// Note that the state h is always SHA256_CHAINING_LENGTH-byte long
static int sha256_init_from_state_impl(SHA256_CTX *sha, int md_len,
                                       const uint8_t h[SHA256_CHAINING_LENGTH],
                                       uint64_t n) {
  if(n % ((uint64_t) SHA256_CBLOCK * 8) != 0) {
    // n is not a multiple of the block size in bits, so it fails
    return 0;
  }

  OPENSSL_memset(sha, 0, sizeof(SHA256_CTX));
  sha->md_len = md_len;

  const size_t out_words = SHA256_CHAINING_LENGTH / 4;
  for (size_t i = 0; i < out_words; i++) {
    sha->h[i] = CRYPTO_load_u32_be(h);
    h += 4;
  }

  sha->Nh = n >> 32;
  sha->Nl = n & 0xffffffff;

  return 1;
}

int SHA224_Init_from_state(SHA256_CTX *sha,
                           const uint8_t h[SHA224_CHAINING_LENGTH],
                           uint64_t n) {
  return sha256_init_from_state_impl(sha, SHA224_DIGEST_LENGTH, h, n);
}

int SHA256_Init_from_state(SHA256_CTX *sha,
                           const uint8_t h[SHA256_CHAINING_LENGTH],
                           uint64_t n) {
  return sha256_init_from_state_impl(sha, SHA256_DIGEST_LENGTH, h, n);
}

uint8_t *SHA224(const uint8_t *data, size_t len,
                uint8_t out[SHA224_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA256_CTX ctx;
  const int ok = SHA224_Init(&ctx) &&
                 SHA224_Update(&ctx, data, len) &&
                 SHA224_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

uint8_t *SHA256(const uint8_t *data, size_t len,
                uint8_t out[SHA256_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA256_CTX ctx;
  const int ok = SHA256_Init(&ctx) &&
                 SHA256_Update(&ctx, data, len) &&
                 SHA256_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

#if !defined(SHA256_ASM)
static void sha256_block_data_order(uint32_t state[8], const uint8_t *in,
                                    size_t num);
#endif

void SHA256_Transform(SHA256_CTX *c, const uint8_t data[SHA256_CBLOCK]) {
  sha256_block_data_order(c->h, data, 1);
}

int SHA256_Update(SHA256_CTX *c, const void *data, size_t len) {
  crypto_md32_update(&sha256_block_data_order, c->h, c->data, SHA256_CBLOCK,
                     &c->num, &c->Nh, &c->Nl, data, len);
  return 1;
}

int SHA224_Update(SHA256_CTX *ctx, const void *data, size_t len) {
  return SHA256_Update(ctx, data, len);
}

static int sha256_final_impl(uint8_t *out, size_t md_len, SHA256_CTX *c) {
  crypto_md32_final(&sha256_block_data_order, c->h, c->data, SHA256_CBLOCK,
                    &c->num, c->Nh, c->Nl, /*is_big_endian=*/1);
  if (c->md_len != md_len) {
    return 0;
  }

  assert(md_len % 4 == 0);
  const size_t out_words = md_len / 4;
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u32_be(out, c->h[i]);
    out += 4;
  }
  FIPS_service_indicator_update_state();
  return 1;
}

int SHA256_Final(uint8_t out[SHA256_DIGEST_LENGTH], SHA256_CTX *c) {
  return sha256_final_impl(out, SHA256_DIGEST_LENGTH, c);
}

int SHA224_Final(uint8_t out[SHA224_DIGEST_LENGTH], SHA256_CTX *ctx) {
  return sha256_final_impl(out, SHA224_DIGEST_LENGTH, ctx);
}

// sha256_get_state_impl is the implementation of
// SHA256_get_state and SHA224_get_state
// Note that the state out_h is always SHA256_CHAINING_LENGTH-byte long
static int sha256_get_state_impl(SHA256_CTX *ctx,
                                 uint8_t out_h[SHA256_CHAINING_LENGTH],
                                 uint64_t *out_n) {
  if (ctx->Nl % ((uint64_t)SHA256_CBLOCK * 8) != 0) {
    // ctx->Nl is not a multiple of the block size in bits, so it fails
    return 0;
  }

  const size_t out_words = SHA256_CHAINING_LENGTH / 4;
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u32_be(out_h, ctx->h[i]);
    out_h += 4;
  }

  *out_n = (((uint64_t)ctx->Nh) << 32) + ctx->Nl;

  return 1;
}

int SHA224_get_state(SHA256_CTX *ctx, uint8_t out_h[SHA224_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha256_get_state_impl(ctx, out_h, out_n);
}

int SHA256_get_state(SHA256_CTX *ctx, uint8_t out_h[SHA256_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha256_get_state_impl(ctx, out_h, out_n);
}

#if !defined(SHA256_ASM)

#if !defined(SHA256_ASM_NOHW)
static const uint32_t K256[64] = {
    0x428a2f98UL, 0x71374491UL, 0xb5c0fbcfUL, 0xe9b5dba5UL, 0x3956c25bUL,
    0x59f111f1UL, 0x923f82a4UL, 0xab1c5ed5UL, 0xd807aa98UL, 0x12835b01UL,
    0x243185beUL, 0x550c7dc3UL, 0x72be5d74UL, 0x80deb1feUL, 0x9bdc06a7UL,
    0xc19bf174UL, 0xe49b69c1UL, 0xefbe4786UL, 0x0fc19dc6UL, 0x240ca1ccUL,
    0x2de92c6fUL, 0x4a7484aaUL, 0x5cb0a9dcUL, 0x76f988daUL, 0x983e5152UL,
    0xa831c66dUL, 0xb00327c8UL, 0xbf597fc7UL, 0xc6e00bf3UL, 0xd5a79147UL,
    0x06ca6351UL, 0x14292967UL, 0x27b70a85UL, 0x2e1b2138UL, 0x4d2c6dfcUL,
    0x53380d13UL, 0x650a7354UL, 0x766a0abbUL, 0x81c2c92eUL, 0x92722c85UL,
    0xa2bfe8a1UL, 0xa81a664bUL, 0xc24b8b70UL, 0xc76c51a3UL, 0xd192e819UL,
    0xd6990624UL, 0xf40e3585UL, 0x106aa070UL, 0x19a4c116UL, 0x1e376c08UL,
    0x2748774cUL, 0x34b0bcb5UL, 0x391c0cb3UL, 0x4ed8aa4aUL, 0x5b9cca4fUL,
    0x682e6ff3UL, 0x748f82eeUL, 0x78a5636fUL, 0x84c87814UL, 0x8cc70208UL,
    0x90befffaUL, 0xa4506cebUL, 0xbef9a3f7UL, 0xc67178f2UL};

// See FIPS 180-4, section 4.1.2.
#define Sigma0(x)                                       \
  (CRYPTO_rotr_u32((x), 2) ^ CRYPTO_rotr_u32((x), 13) ^ \
   CRYPTO_rotr_u32((x), 22))
#define Sigma1(x)                                       \
  (CRYPTO_rotr_u32((x), 6) ^ CRYPTO_rotr_u32((x), 11) ^ \
   CRYPTO_rotr_u32((x), 25))
#define sigma0(x) \
  (CRYPTO_rotr_u32((x), 7) ^ CRYPTO_rotr_u32((x), 18) ^ ((x) >> 3))
#define sigma1(x) \
  (CRYPTO_rotr_u32((x), 17) ^ CRYPTO_rotr_u32((x), 19) ^ ((x) >> 10))

#define Ch(x, y, z) (((x) & (y)) ^ ((~(x)) & (z)))
#define Maj(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))

#define ROUND_00_15(i, a, b, c, d, e, f, g, h)   \
  do {                                           \
    T1 += h + Sigma1(e) + Ch(e, f, g) + K256[i]; \
    h = Sigma0(a) + Maj(a, b, c);                \
    d += T1;                                     \
    h += T1;                                     \
  } while (0)

#define ROUND_16_63(i, a, b, c, d, e, f, g, h, X)      \
  do {                                                 \
    s0 = X[(i + 1) & 0x0f];                            \
    s0 = sigma0(s0);                                   \
    s1 = X[(i + 14) & 0x0f];                           \
    s1 = sigma1(s1);                                   \
    T1 = X[(i) & 0x0f] += s0 + s1 + X[(i + 9) & 0x0f]; \
    ROUND_00_15(i, a, b, c, d, e, f, g, h);            \
  } while (0)

static void sha256_block_data_order_nohw(uint32_t state[8], const uint8_t *data,
                                         size_t num) {
  uint32_t a, b, c, d, e, f, g, h, s0, s1, T1;
  uint32_t X[16];
  int i;

  while (num--) {
    a = state[0];
    b = state[1];
    c = state[2];
    d = state[3];
    e = state[4];
    f = state[5];
    g = state[6];
    h = state[7];

    T1 = X[0] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(0, a, b, c, d, e, f, g, h);
    T1 = X[1] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(1, h, a, b, c, d, e, f, g);
    T1 = X[2] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(2, g, h, a, b, c, d, e, f);
    T1 = X[3] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(3, f, g, h, a, b, c, d, e);
    T1 = X[4] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(4, e, f, g, h, a, b, c, d);
    T1 = X[5] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(5, d, e, f, g, h, a, b, c);
    T1 = X[6] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(6, c, d, e, f, g, h, a, b);
    T1 = X[7] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(7, b, c, d, e, f, g, h, a);
    T1 = X[8] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(8, a, b, c, d, e, f, g, h);
    T1 = X[9] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(9, h, a, b, c, d, e, f, g);
    T1 = X[10] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(10, g, h, a, b, c, d, e, f);
    T1 = X[11] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(11, f, g, h, a, b, c, d, e);
    T1 = X[12] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(12, e, f, g, h, a, b, c, d);
    T1 = X[13] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(13, d, e, f, g, h, a, b, c);
    T1 = X[14] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(14, c, d, e, f, g, h, a, b);
    T1 = X[15] = CRYPTO_load_u32_be(data);
    data += 4;
    ROUND_00_15(15, b, c, d, e, f, g, h, a);

    for (i = 16; i < 64; i += 8) {
      ROUND_16_63(i + 0, a, b, c, d, e, f, g, h, X);
      ROUND_16_63(i + 1, h, a, b, c, d, e, f, g, X);
      ROUND_16_63(i + 2, g, h, a, b, c, d, e, f, X);
      ROUND_16_63(i + 3, f, g, h, a, b, c, d, e, X);
      ROUND_16_63(i + 4, e, f, g, h, a, b, c, d, X);
      ROUND_16_63(i + 5, d, e, f, g, h, a, b, c, X);
      ROUND_16_63(i + 6, c, d, e, f, g, h, a, b, X);
      ROUND_16_63(i + 7, b, c, d, e, f, g, h, a, X);
    }

    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;
  }
}

#endif  // !defined(SHA256_ASM_NOHW)

static void sha256_block_data_order(uint32_t state[8], const uint8_t *data,
                                    size_t num) {
#if defined(SHA256_ASM_HW)
  if (sha256_hw_capable()) {
    sha256_block_data_order_hw(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  if (sha256_avx_capable()) {
    sha256_block_data_order_avx(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_SSSE3)
  if (sha256_ssse3_capable()) {
    sha256_block_data_order_ssse3(state, data, num);
    return;
  }
#endif
#if defined(SHA256_ASM_NEON)
  if (CRYPTO_is_NEON_capable()) {
    sha256_block_data_order_neon(state, data, num);
    return;
  }
#endif
  sha256_block_data_order_nohw(state, data, num);
}

#endif  // !defined(SHA256_ASM)


void SHA256_TransformBlocks(uint32_t state[8], const uint8_t *data,
                            size_t num_blocks) {
  sha256_block_data_order(state, data, num_blocks);
}

#undef Sigma0
#undef Sigma1
#undef sigma0
#undef sigma1
#undef Ch
#undef Maj
#undef ROUND_00_15
#undef ROUND_16_63
