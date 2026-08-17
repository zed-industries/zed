// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/sha.h>

#include <string.h>

#include <openssl/mem.h>

#include "internal.h"
#include "../../internal.h"


// The 32-bit hash algorithms share a common byte-order neutral collector and
// padding function implementations that operate on unaligned data,
// ../digest/md32_common.h. SHA-512 is the only 64-bit hash algorithm, as of
// this writing, so there is no need for a common collector/padding
// implementation yet.

static int sha512_final_impl(uint8_t *out, size_t md_len, SHA512_CTX *sha);

int SHA384_Init(SHA512_CTX *sha) {
  sha->h[0] = UINT64_C(0xcbbb9d5dc1059ed8);
  sha->h[1] = UINT64_C(0x629a292a367cd507);
  sha->h[2] = UINT64_C(0x9159015a3070dd17);
  sha->h[3] = UINT64_C(0x152fecd8f70e5939);
  sha->h[4] = UINT64_C(0x67332667ffc00b31);
  sha->h[5] = UINT64_C(0x8eb44a8768581511);
  sha->h[6] = UINT64_C(0xdb0c2e0d64f98fa7);
  sha->h[7] = UINT64_C(0x47b5481dbefa4fa4);

  sha->Nl = 0;
  sha->Nh = 0;
  sha->num = 0;
  sha->md_len = SHA384_DIGEST_LENGTH;
  return 1;
}


int SHA512_Init(SHA512_CTX *sha) {
  sha->h[0] = UINT64_C(0x6a09e667f3bcc908);
  sha->h[1] = UINT64_C(0xbb67ae8584caa73b);
  sha->h[2] = UINT64_C(0x3c6ef372fe94f82b);
  sha->h[3] = UINT64_C(0xa54ff53a5f1d36f1);
  sha->h[4] = UINT64_C(0x510e527fade682d1);
  sha->h[5] = UINT64_C(0x9b05688c2b3e6c1f);
  sha->h[6] = UINT64_C(0x1f83d9abfb41bd6b);
  sha->h[7] = UINT64_C(0x5be0cd19137e2179);

  sha->Nl = 0;
  sha->Nh = 0;
  sha->num = 0;
  sha->md_len = SHA512_DIGEST_LENGTH;
  return 1;
}

int SHA512_224_Init(SHA512_CTX *sha) {
  sha->h[0] = UINT64_C(0x8c3d37c819544da2);
  sha->h[1] = UINT64_C(0x73e1996689dcd4d6);
  sha->h[2] = UINT64_C(0x1dfab7ae32ff9c82);
  sha->h[3] = UINT64_C(0x679dd514582f9fcf);
  sha->h[4] = UINT64_C(0x0f6d2b697bd44da8);
  sha->h[5] = UINT64_C(0x77e36f7304c48942);
  sha->h[6] = UINT64_C(0x3f9d85a86a1d36c8);
  sha->h[7] = UINT64_C(0x1112e6ad91d692a1);

  sha->Nl = 0;
  sha->Nh = 0;
  sha->num = 0;
  sha->md_len = SHA512_224_DIGEST_LENGTH;
  return 1;
}

int SHA512_256_Init(SHA512_CTX *sha) {
  sha->h[0] = UINT64_C(0x22312194fc2bf72c);
  sha->h[1] = UINT64_C(0x9f555fa3c84c64c2);
  sha->h[2] = UINT64_C(0x2393b86b6f53b151);
  sha->h[3] = UINT64_C(0x963877195940eabd);
  sha->h[4] = UINT64_C(0x96283ee2a88effe3);
  sha->h[5] = UINT64_C(0xbe5e1e2553863992);
  sha->h[6] = UINT64_C(0x2b0199fc2c85b8aa);
  sha->h[7] = UINT64_C(0x0eb72ddc81c52ca2);

  sha->Nl = 0;
  sha->Nh = 0;
  sha->num = 0;
  sha->md_len = SHA512_256_DIGEST_LENGTH;
  return 1;
}

OPENSSL_STATIC_ASSERT(SHA512_CHAINING_LENGTH==SHA384_CHAINING_LENGTH,
                      sha512_and_sha384_have_same_chaining_length)
OPENSSL_STATIC_ASSERT(SHA512_CHAINING_LENGTH==SHA512_224_CHAINING_LENGTH,
                      sha512_and_sha512_224_have_same_chaining_length)
OPENSSL_STATIC_ASSERT(SHA512_CHAINING_LENGTH==SHA512_256_CHAINING_LENGTH,
                      sha512_and_sha512_256_have_same_chaining_length)

// sha512_init_from_state_impl is the implementation of
// SHA512_Init_from_state and SHA224_Init_from_state
// Note that the state h is always SHA512_CHAINING_LENGTH-byte long
static int sha512_init_from_state_impl(SHA512_CTX *sha, int md_len,
                                       const uint8_t h[SHA512_CHAINING_LENGTH],
                                       uint64_t n) {
  if(n % ((uint64_t) SHA512_CBLOCK * 8) != 0) {
    // n is not a multiple of the block size in bits, so it fails
    return 0;
  }

  OPENSSL_memset(sha, 0, sizeof(SHA512_CTX));
  sha->md_len = md_len;

  const size_t out_words = SHA512_CHAINING_LENGTH / 8;
  for (size_t i = 0; i < out_words; i++) {
    sha->h[i] = CRYPTO_load_u64_be(h);
    h += 8;
  }

  sha->Nh = 0;
  sha->Nl = n;

  return 1;
}

int SHA384_Init_from_state(SHA512_CTX *sha,
                           const uint8_t h[SHA384_CHAINING_LENGTH],
                           uint64_t n) {
  return sha512_init_from_state_impl(sha, SHA384_DIGEST_LENGTH, h, n);
}

int SHA512_Init_from_state(SHA512_CTX *sha,
                           const uint8_t h[SHA512_CHAINING_LENGTH],
                           uint64_t n) {
  return sha512_init_from_state_impl(sha, SHA512_DIGEST_LENGTH, h, n);
}

int SHA512_224_Init_from_state(SHA512_CTX *sha,
                           const uint8_t h[SHA512_224_CHAINING_LENGTH],
                           uint64_t n) {
  return sha512_init_from_state_impl(sha, SHA512_224_DIGEST_LENGTH, h, n);
}

int SHA512_256_Init_from_state(SHA512_CTX *sha,
                           const uint8_t h[SHA512_256_CHAINING_LENGTH],
                           uint64_t n) {
  return sha512_init_from_state_impl(sha, SHA512_256_DIGEST_LENGTH, h, n);
}

uint8_t *SHA384(const uint8_t *data, size_t len,
                uint8_t out[SHA384_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA512_CTX ctx;
  const int ok = SHA384_Init(&ctx) &&
                 SHA384_Update(&ctx, data, len) &&
                 SHA384_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

uint8_t *SHA512(const uint8_t *data, size_t len,
                uint8_t out[SHA512_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA512_CTX ctx;
  const int ok = SHA512_Init(&ctx) &&
                 SHA512_Update(&ctx, data, len) &&
                 SHA512_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

uint8_t *SHA512_224(const uint8_t *data, size_t len,
                    uint8_t out[SHA512_224_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA512_CTX ctx;
  const int ok = SHA512_224_Init(&ctx) &&
                 SHA512_224_Update(&ctx, data, len) &&
                 SHA512_224_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

uint8_t *SHA512_256(const uint8_t *data, size_t len,
                    uint8_t out[SHA512_256_DIGEST_LENGTH]) {
  // We have to verify that all the SHA services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SHA512_CTX ctx;
  const int ok = SHA512_256_Init(&ctx) &&
                 SHA512_256_Update(&ctx, data, len) &&
                 SHA512_256_Final(out, &ctx);
  FIPS_service_indicator_unlock_state();
  if(ok) {
    FIPS_service_indicator_update_state();
  }
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  return out;
}

#if !defined(SHA512_ASM)
static void sha512_block_data_order(uint64_t state[8], const uint8_t *in,
                                    size_t num_blocks);
#endif


int SHA384_Final(uint8_t out[SHA384_DIGEST_LENGTH], SHA512_CTX *sha) {
  // This function must be paired with |SHA384_Init|, which sets |sha->md_len|
  // to |SHA384_DIGEST_LENGTH|.
  assert(sha->md_len == SHA384_DIGEST_LENGTH);
  return sha512_final_impl(out, SHA384_DIGEST_LENGTH, sha);
}

int SHA384_Update(SHA512_CTX *sha, const void *data, size_t len) {
  return SHA512_Update(sha, data, len);
}

int SHA512_224_Update(SHA512_CTX *sha, const void *data, size_t len) {
  return SHA512_Update(sha, data, len);
}

int SHA512_224_Final(uint8_t out[SHA512_224_DIGEST_LENGTH], SHA512_CTX *sha) {
  // This function must be paired with |SHA512_224_Init|, which sets
  // |sha->md_len| to |SHA512_224_DIGEST_LENGTH|.
  assert(sha->md_len == SHA512_224_DIGEST_LENGTH);
  return sha512_final_impl(out, SHA512_224_DIGEST_LENGTH, sha);
}

int SHA512_256_Update(SHA512_CTX *sha, const void *data, size_t len) {
  return SHA512_Update(sha, data, len);
}

int SHA512_256_Final(uint8_t out[SHA512_256_DIGEST_LENGTH], SHA512_CTX *sha) {
  // This function must be paired with |SHA512_256_Init|, which sets
  // |sha->md_len| to |SHA512_256_DIGEST_LENGTH|.
  assert(sha->md_len == SHA512_256_DIGEST_LENGTH);
  return sha512_final_impl(out, SHA512_256_DIGEST_LENGTH, sha);
}

void SHA512_Transform(SHA512_CTX *c, const uint8_t block[SHA512_CBLOCK]) {
  sha512_block_data_order(c->h, block, 1);
}

int SHA512_Update(SHA512_CTX *c, const void *in_data, size_t len) {
  uint64_t l;
  uint8_t *p = c->p;
  const uint8_t *data = in_data;

  if (len == 0) {
    return 1;
  }

  l = (c->Nl + (((uint64_t)len) << 3)) & UINT64_C(0xffffffffffffffff);
  if (l < c->Nl) {
    c->Nh++;
  }
  if (sizeof(len) >= 8) {
    c->Nh += (((uint64_t)len) >> 61);
  }
  c->Nl = l;

  if (c->num != 0) {
    size_t n = sizeof(c->p) - c->num;

    if (len < n) {
      OPENSSL_memcpy(p + c->num, data, len);
      c->num += (unsigned int)len;
      return 1;
    } else {
      OPENSSL_memcpy(p + c->num, data, n), c->num = 0;
      len -= n;
      data += n;
      sha512_block_data_order(c->h, p, 1);
    }
  }

  if (len >= sizeof(c->p)) {
    sha512_block_data_order(c->h, data, len / sizeof(c->p));
    data += len;
    len %= sizeof(c->p);
    data -= len;
  }

  if (len != 0) {
    OPENSSL_memcpy(p, data, len);
    c->num = (int)len;
  }

  return 1;
}

int SHA512_Final(uint8_t out[SHA512_DIGEST_LENGTH], SHA512_CTX *sha) {
  // Ideally we would assert |sha->md_len| is |SHA512_DIGEST_LENGTH| to match
  // the size hint, but calling code often pairs |SHA384_Init| with
  // |SHA512_Final| and expects |sha->md_len| to carry the size over.
  //
  // TODO(davidben): Add an assert and fix code to match them up.
  return sha512_final_impl(out, sha->md_len, sha);
}

static int sha512_final_impl(uint8_t *out, size_t md_len, SHA512_CTX *sha) {
  uint8_t *p = sha->p;
  size_t n = sha->num;

  p[n] = 0x80;  // There always is a room for one
  n++;
  if (n > (sizeof(sha->p) - 16)) {
    OPENSSL_memset(p + n, 0, sizeof(sha->p) - n);
    n = 0;
    sha512_block_data_order(sha->h, p, 1);
  }

  OPENSSL_memset(p + n, 0, sizeof(sha->p) - 16 - n);
  CRYPTO_store_u64_be(p + sizeof(sha->p) - 16, sha->Nh);
  CRYPTO_store_u64_be(p + sizeof(sha->p) - 8, sha->Nl);

  sha512_block_data_order(sha->h, p, 1);

  if (out == NULL) {
    // TODO(davidben): This NULL check is absent in other low-level hash 'final'
    // functions and is one of the few places one can fail.
    return 0;
  }

  const size_t out_words = md_len / 8;
  assert(md_len % 8 == 0 || md_len == SHA512_224_DIGEST_LENGTH);
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u64_be(out, sha->h[i]);
    out += 8;
  }

  // SHA-512 and SHA-512/256 are aligned to 8-byte words, SHA-512/224 is not.
  // If the digest size is not aligned to 8-byte words, we need to process the
  // non-word-aligned "trailer".
  if (md_len == SHA512_224_DIGEST_LENGTH) {
    uint64_t trailer;
    CRYPTO_store_u64_be(&trailer, sha->h[out_words]);
    OPENSSL_memcpy(out, &trailer, SHA512_224_DIGEST_LENGTH % 8);
  }

  FIPS_service_indicator_update_state();
  return 1;
}

// sha512_get_state_impl is the implementation of
// SHA512_get_state and SHA224_get_state
// Note that the state out_h is always SHA512_CHAINING_LENGTH-byte long
static int sha512_get_state_impl(SHA512_CTX *ctx,
                                 uint8_t out_h[SHA512_CHAINING_LENGTH],
                                 uint64_t *out_n) {
  if (ctx->Nl % ((uint64_t)SHA512_CBLOCK * 8) != 0) {
    // ctx->Nl is not a multiple of the block size in bits, so it fails
    return 0;
  }

  if (ctx->Nh != 0) {
    // |sha512_get_state_impl| assumes that at most 2^64 bits have been
    // processed by the hash function
    return 0;
  }

  const size_t out_words = SHA512_CHAINING_LENGTH / 8;
  for (size_t i = 0; i < out_words; i++) {
    CRYPTO_store_u64_be(out_h, ctx->h[i]);
    out_h += 8;
  }

  *out_n = ctx->Nl;  // we know that ctx->Nh = 0

  return 1;
}

int SHA384_get_state(SHA512_CTX *ctx, uint8_t out_h[SHA384_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha512_get_state_impl(ctx, out_h, out_n);
}

int SHA512_get_state(SHA512_CTX *ctx, uint8_t out_h[SHA512_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha512_get_state_impl(ctx, out_h, out_n);
}

int SHA512_224_get_state(SHA512_CTX *ctx, uint8_t out_h[SHA512_224_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha512_get_state_impl(ctx, out_h, out_n);
}

int SHA512_256_get_state(SHA512_CTX *ctx, uint8_t out_h[SHA512_256_CHAINING_LENGTH],
                     uint64_t *out_n) {
  return sha512_get_state_impl(ctx, out_h, out_n);
}

#if !defined(SHA512_ASM)

#if !defined(SHA512_ASM_NOHW)
static const uint64_t K512[80] = {
    UINT64_C(0x428a2f98d728ae22), UINT64_C(0x7137449123ef65cd),
    UINT64_C(0xb5c0fbcfec4d3b2f), UINT64_C(0xe9b5dba58189dbbc),
    UINT64_C(0x3956c25bf348b538), UINT64_C(0x59f111f1b605d019),
    UINT64_C(0x923f82a4af194f9b), UINT64_C(0xab1c5ed5da6d8118),
    UINT64_C(0xd807aa98a3030242), UINT64_C(0x12835b0145706fbe),
    UINT64_C(0x243185be4ee4b28c), UINT64_C(0x550c7dc3d5ffb4e2),
    UINT64_C(0x72be5d74f27b896f), UINT64_C(0x80deb1fe3b1696b1),
    UINT64_C(0x9bdc06a725c71235), UINT64_C(0xc19bf174cf692694),
    UINT64_C(0xe49b69c19ef14ad2), UINT64_C(0xefbe4786384f25e3),
    UINT64_C(0x0fc19dc68b8cd5b5), UINT64_C(0x240ca1cc77ac9c65),
    UINT64_C(0x2de92c6f592b0275), UINT64_C(0x4a7484aa6ea6e483),
    UINT64_C(0x5cb0a9dcbd41fbd4), UINT64_C(0x76f988da831153b5),
    UINT64_C(0x983e5152ee66dfab), UINT64_C(0xa831c66d2db43210),
    UINT64_C(0xb00327c898fb213f), UINT64_C(0xbf597fc7beef0ee4),
    UINT64_C(0xc6e00bf33da88fc2), UINT64_C(0xd5a79147930aa725),
    UINT64_C(0x06ca6351e003826f), UINT64_C(0x142929670a0e6e70),
    UINT64_C(0x27b70a8546d22ffc), UINT64_C(0x2e1b21385c26c926),
    UINT64_C(0x4d2c6dfc5ac42aed), UINT64_C(0x53380d139d95b3df),
    UINT64_C(0x650a73548baf63de), UINT64_C(0x766a0abb3c77b2a8),
    UINT64_C(0x81c2c92e47edaee6), UINT64_C(0x92722c851482353b),
    UINT64_C(0xa2bfe8a14cf10364), UINT64_C(0xa81a664bbc423001),
    UINT64_C(0xc24b8b70d0f89791), UINT64_C(0xc76c51a30654be30),
    UINT64_C(0xd192e819d6ef5218), UINT64_C(0xd69906245565a910),
    UINT64_C(0xf40e35855771202a), UINT64_C(0x106aa07032bbd1b8),
    UINT64_C(0x19a4c116b8d2d0c8), UINT64_C(0x1e376c085141ab53),
    UINT64_C(0x2748774cdf8eeb99), UINT64_C(0x34b0bcb5e19b48a8),
    UINT64_C(0x391c0cb3c5c95a63), UINT64_C(0x4ed8aa4ae3418acb),
    UINT64_C(0x5b9cca4f7763e373), UINT64_C(0x682e6ff3d6b2b8a3),
    UINT64_C(0x748f82ee5defb2fc), UINT64_C(0x78a5636f43172f60),
    UINT64_C(0x84c87814a1f0ab72), UINT64_C(0x8cc702081a6439ec),
    UINT64_C(0x90befffa23631e28), UINT64_C(0xa4506cebde82bde9),
    UINT64_C(0xbef9a3f7b2c67915), UINT64_C(0xc67178f2e372532b),
    UINT64_C(0xca273eceea26619c), UINT64_C(0xd186b8c721c0c207),
    UINT64_C(0xeada7dd6cde0eb1e), UINT64_C(0xf57d4f7fee6ed178),
    UINT64_C(0x06f067aa72176fba), UINT64_C(0x0a637dc5a2c898a6),
    UINT64_C(0x113f9804bef90dae), UINT64_C(0x1b710b35131c471b),
    UINT64_C(0x28db77f523047d84), UINT64_C(0x32caab7b40c72493),
    UINT64_C(0x3c9ebe0a15c9bebc), UINT64_C(0x431d67c49c100d4c),
    UINT64_C(0x4cc5d4becb3e42b6), UINT64_C(0x597f299cfc657e2a),
    UINT64_C(0x5fcb6fab3ad6faec), UINT64_C(0x6c44198c4a475817),
};

#define Sigma0(x)                                        \
  (CRYPTO_rotr_u64((x), 28) ^ CRYPTO_rotr_u64((x), 34) ^ \
   CRYPTO_rotr_u64((x), 39))
#define Sigma1(x)                                        \
  (CRYPTO_rotr_u64((x), 14) ^ CRYPTO_rotr_u64((x), 18) ^ \
   CRYPTO_rotr_u64((x), 41))
#define sigma0(x) \
  (CRYPTO_rotr_u64((x), 1) ^ CRYPTO_rotr_u64((x), 8) ^ ((x) >> 7))
#define sigma1(x) \
  (CRYPTO_rotr_u64((x), 19) ^ CRYPTO_rotr_u64((x), 61) ^ ((x) >> 6))

#define Ch(x, y, z) (((x) & (y)) ^ ((~(x)) & (z)))
#define Maj(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))


#if defined(__i386) || defined(__i386__) || defined(_M_IX86)
// This code should give better results on 32-bit CPU with less than
// ~24 registers, both size and performance wise...
static void sha512_block_data_order_nohw(uint64_t state[8], const uint8_t *in,
                                         size_t num) {
  uint64_t A, E, T;
  uint64_t X[9 + 80], *F;
  int i;

  while (num--) {
    F = X + 80;
    A = state[0];
    F[1] = state[1];
    F[2] = state[2];
    F[3] = state[3];
    E = state[4];
    F[5] = state[5];
    F[6] = state[6];
    F[7] = state[7];

    for (i = 0; i < 16; i++, F--) {
      T = CRYPTO_load_u64_be(in + i * 8);
      F[0] = A;
      F[4] = E;
      F[8] = T;
      T += F[7] + Sigma1(E) + Ch(E, F[5], F[6]) + K512[i];
      E = F[3] + T;
      A = T + Sigma0(A) + Maj(A, F[1], F[2]);
    }

    for (; i < 80; i++, F--) {
      T = sigma0(F[8 + 16 - 1]);
      T += sigma1(F[8 + 16 - 14]);
      T += F[8 + 16] + F[8 + 16 - 9];

      F[0] = A;
      F[4] = E;
      F[8] = T;
      T += F[7] + Sigma1(E) + Ch(E, F[5], F[6]) + K512[i];
      E = F[3] + T;
      A = T + Sigma0(A) + Maj(A, F[1], F[2]);
    }

    state[0] += A;
    state[1] += F[1];
    state[2] += F[2];
    state[3] += F[3];
    state[4] += E;
    state[5] += F[5];
    state[6] += F[6];
    state[7] += F[7];

    in += 16 * 8;
  }
}

#else

#define ROUND_00_15(i, a, b, c, d, e, f, g, h)   \
  do {                                           \
    T1 += h + Sigma1(e) + Ch(e, f, g) + K512[i]; \
    h = Sigma0(a) + Maj(a, b, c);                \
    d += T1;                                     \
    h += T1;                                     \
  } while (0)

#define ROUND_16_80(i, j, a, b, c, d, e, f, g, h, X)   \
  do {                                                 \
    s0 = X[(j + 1) & 0x0f];                            \
    s0 = sigma0(s0);                                   \
    s1 = X[(j + 14) & 0x0f];                           \
    s1 = sigma1(s1);                                   \
    T1 = X[(j) & 0x0f] += s0 + s1 + X[(j + 9) & 0x0f]; \
    ROUND_00_15(i + j, a, b, c, d, e, f, g, h);        \
  } while (0)

static void sha512_block_data_order_nohw(uint64_t state[8], const uint8_t *in,
                                         size_t num) {
  uint64_t a, b, c, d, e, f, g, h, s0, s1, T1;
  uint64_t X[16];
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

    T1 = X[0] = CRYPTO_load_u64_be(in);
    ROUND_00_15(0, a, b, c, d, e, f, g, h);
    T1 = X[1] = CRYPTO_load_u64_be(in + 8);
    ROUND_00_15(1, h, a, b, c, d, e, f, g);
    T1 = X[2] = CRYPTO_load_u64_be(in + 2 * 8);
    ROUND_00_15(2, g, h, a, b, c, d, e, f);
    T1 = X[3] = CRYPTO_load_u64_be(in + 3 * 8);
    ROUND_00_15(3, f, g, h, a, b, c, d, e);
    T1 = X[4] = CRYPTO_load_u64_be(in + 4 * 8);
    ROUND_00_15(4, e, f, g, h, a, b, c, d);
    T1 = X[5] = CRYPTO_load_u64_be(in + 5 * 8);
    ROUND_00_15(5, d, e, f, g, h, a, b, c);
    T1 = X[6] = CRYPTO_load_u64_be(in + 6 * 8);
    ROUND_00_15(6, c, d, e, f, g, h, a, b);
    T1 = X[7] = CRYPTO_load_u64_be(in + 7 * 8);
    ROUND_00_15(7, b, c, d, e, f, g, h, a);
    T1 = X[8] = CRYPTO_load_u64_be(in + 8 * 8);
    ROUND_00_15(8, a, b, c, d, e, f, g, h);
    T1 = X[9] = CRYPTO_load_u64_be(in + 9 * 8);
    ROUND_00_15(9, h, a, b, c, d, e, f, g);
    T1 = X[10] = CRYPTO_load_u64_be(in + 10 * 8);
    ROUND_00_15(10, g, h, a, b, c, d, e, f);
    T1 = X[11] = CRYPTO_load_u64_be(in + 11 * 8);
    ROUND_00_15(11, f, g, h, a, b, c, d, e);
    T1 = X[12] = CRYPTO_load_u64_be(in + 12 * 8);
    ROUND_00_15(12, e, f, g, h, a, b, c, d);
    T1 = X[13] = CRYPTO_load_u64_be(in + 13 * 8);
    ROUND_00_15(13, d, e, f, g, h, a, b, c);
    T1 = X[14] = CRYPTO_load_u64_be(in + 14 * 8);
    ROUND_00_15(14, c, d, e, f, g, h, a, b);
    T1 = X[15] = CRYPTO_load_u64_be(in + 15 * 8);
    ROUND_00_15(15, b, c, d, e, f, g, h, a);

    for (i = 16; i < 80; i += 16) {
      ROUND_16_80(i, 0, a, b, c, d, e, f, g, h, X);
      ROUND_16_80(i, 1, h, a, b, c, d, e, f, g, X);
      ROUND_16_80(i, 2, g, h, a, b, c, d, e, f, X);
      ROUND_16_80(i, 3, f, g, h, a, b, c, d, e, X);
      ROUND_16_80(i, 4, e, f, g, h, a, b, c, d, X);
      ROUND_16_80(i, 5, d, e, f, g, h, a, b, c, X);
      ROUND_16_80(i, 6, c, d, e, f, g, h, a, b, X);
      ROUND_16_80(i, 7, b, c, d, e, f, g, h, a, X);
      ROUND_16_80(i, 8, a, b, c, d, e, f, g, h, X);
      ROUND_16_80(i, 9, h, a, b, c, d, e, f, g, X);
      ROUND_16_80(i, 10, g, h, a, b, c, d, e, f, X);
      ROUND_16_80(i, 11, f, g, h, a, b, c, d, e, X);
      ROUND_16_80(i, 12, e, f, g, h, a, b, c, d, X);
      ROUND_16_80(i, 13, d, e, f, g, h, a, b, c, X);
      ROUND_16_80(i, 14, c, d, e, f, g, h, a, b, X);
      ROUND_16_80(i, 15, b, c, d, e, f, g, h, a, X);
    }

    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;

    in += 16 * 8;
  }
}

#endif

#endif  // !SHA512_ASM_NOHW

static void sha512_block_data_order(uint64_t state[8], const uint8_t *data,
                                    size_t num) {
#if defined(SHA512_ASM_HW)
  if (sha512_hw_capable()) {
    sha512_block_data_order_hw(state, data, num);
    return;
  }
#endif
#if defined(SHA512_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
  if (sha512_avx_capable()) {
    sha512_block_data_order_avx(state, data, num);
    return;
  }
#endif
#if defined(SHA512_ASM_NEON)
  if (CRYPTO_is_NEON_capable()) {
    sha512_block_data_order_neon(state, data, num);
    return;
  }
#endif
  sha512_block_data_order_nohw(state, data, num);
}

#endif  // !SHA512_ASM

#undef Sigma0
#undef Sigma1
#undef sigma0
#undef sigma1
#undef Ch
#undef Maj
#undef ROUND_00_15
#undef ROUND_16_80
