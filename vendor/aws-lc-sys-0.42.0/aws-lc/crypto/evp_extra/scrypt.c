// Copyright 2015-2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <assert.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/type_check.h>

#include "../internal.h"


// This file implements scrypt, described in RFC 7914.
//
// Note scrypt refers to both "blocks" and a "block size" parameter, r. These
// are two different notions of blocks. A Salsa20 block is 64 bytes long,
// represented in this implementation by 16 |uint32_t|s. |r| determines the
// number of 64-byte Salsa20 blocks in a scryptBlockMix block, which is 2 * |r|
// Salsa20 blocks. This implementation refers to them as Salsa20 blocks and
// scrypt blocks, respectively.

// A block_t is a Salsa20 block.
#define SCRYPT_BLOCK_WORD_CNT 16
typedef struct { uint32_t words[SCRYPT_BLOCK_WORD_CNT]; } block_t;

OPENSSL_STATIC_ASSERT(sizeof(block_t) == 64, block_t_has_padding)

// salsa208_word_specification implements the Salsa20/8 core function, also
// described in RFC 7914, section 3. It modifies the block at |inout|
// in-place.
static void salsa208_word_specification(block_t *inout) {
  block_t x;
  OPENSSL_memcpy(&x, inout, sizeof(x));

  for (int i = 8; i > 0; i -= 2) {
    x.words[4] ^= CRYPTO_rotl_u32(x.words[0] + x.words[12], 7);
    x.words[8] ^= CRYPTO_rotl_u32(x.words[4] + x.words[0], 9);
    x.words[12] ^= CRYPTO_rotl_u32(x.words[8] + x.words[4], 13);
    x.words[0] ^= CRYPTO_rotl_u32(x.words[12] + x.words[8], 18);
    x.words[9] ^= CRYPTO_rotl_u32(x.words[5] + x.words[1], 7);
    x.words[13] ^= CRYPTO_rotl_u32(x.words[9] + x.words[5], 9);
    x.words[1] ^= CRYPTO_rotl_u32(x.words[13] + x.words[9], 13);
    x.words[5] ^= CRYPTO_rotl_u32(x.words[1] + x.words[13], 18);
    x.words[14] ^= CRYPTO_rotl_u32(x.words[10] + x.words[6], 7);
    x.words[2] ^= CRYPTO_rotl_u32(x.words[14] + x.words[10], 9);
    x.words[6] ^= CRYPTO_rotl_u32(x.words[2] + x.words[14], 13);
    x.words[10] ^= CRYPTO_rotl_u32(x.words[6] + x.words[2], 18);
    x.words[3] ^= CRYPTO_rotl_u32(x.words[15] + x.words[11], 7);
    x.words[7] ^= CRYPTO_rotl_u32(x.words[3] + x.words[15], 9);
    x.words[11] ^= CRYPTO_rotl_u32(x.words[7] + x.words[3], 13);
    x.words[15] ^= CRYPTO_rotl_u32(x.words[11] + x.words[7], 18);
    x.words[1] ^= CRYPTO_rotl_u32(x.words[0] + x.words[3], 7);
    x.words[2] ^= CRYPTO_rotl_u32(x.words[1] + x.words[0], 9);
    x.words[3] ^= CRYPTO_rotl_u32(x.words[2] + x.words[1], 13);
    x.words[0] ^= CRYPTO_rotl_u32(x.words[3] + x.words[2], 18);
    x.words[6] ^= CRYPTO_rotl_u32(x.words[5] + x.words[4], 7);
    x.words[7] ^= CRYPTO_rotl_u32(x.words[6] + x.words[5], 9);
    x.words[4] ^= CRYPTO_rotl_u32(x.words[7] + x.words[6], 13);
    x.words[5] ^= CRYPTO_rotl_u32(x.words[4] + x.words[7], 18);
    x.words[11] ^= CRYPTO_rotl_u32(x.words[10] + x.words[9], 7);
    x.words[8] ^= CRYPTO_rotl_u32(x.words[11] + x.words[10], 9);
    x.words[9] ^= CRYPTO_rotl_u32(x.words[8] + x.words[11], 13);
    x.words[10] ^= CRYPTO_rotl_u32(x.words[9] + x.words[8], 18);
    x.words[12] ^= CRYPTO_rotl_u32(x.words[15] + x.words[14], 7);
    x.words[13] ^= CRYPTO_rotl_u32(x.words[12] + x.words[15], 9);
    x.words[14] ^= CRYPTO_rotl_u32(x.words[13] + x.words[12], 13);
    x.words[15] ^= CRYPTO_rotl_u32(x.words[14] + x.words[13], 18);
  }

  for (int i = 0; i < 16; ++i) {
    inout->words[i] += x.words[i];
  }
}

// xor_block sets |*out| to be |*a| XOR |*b|.
static void xor_block(block_t *out, const block_t *a, const block_t *b) {
  for (size_t i = 0; i < 16; i++) {
    out->words[i] = a->words[i] ^ b->words[i];
  }
}

// scryptBlockMix implements the function described in RFC 7914, section 4. B'
// is written to |out|. |out| and |B| may not alias and must be each one scrypt
// block (2 * |r| Salsa20 blocks) long.
static void scryptBlockMix(block_t *out, const block_t *B, uint64_t r) {
  assert(out != B);

  block_t X;
  OPENSSL_memcpy(&X, &B[r * 2 - 1], sizeof(X));
  for (uint64_t i = 0; i < r * 2; i++) {
    xor_block(&X, &X, &B[i]);
    salsa208_word_specification(&X);

    // This implements the permutation in step 3.
    OPENSSL_memcpy(&out[i / 2 + (i & 1) * r], &X, sizeof(X));
  }
}

// scryptROMix implements the function described in RFC 7914, section 5.  |B| is
// an scrypt block (2 * |r| Salsa20 blocks) and is modified in-place. |T| and
// |V| are scratch space allocated by the caller. |T| must have space for one
// scrypt block (2 * |r| Salsa20 blocks). |V| must have space for |N| scrypt
// blocks (2 * |r| * |N| Salsa20 blocks).
static void scryptROMix(block_t *B, uint64_t r, uint64_t N, block_t *T,
                        block_t *V) {
  // Steps 1 and 2.
#ifdef OPENSSL_BIG_ENDIAN
  for(size_t i = 0; i < (2 * r * SCRYPT_BLOCK_WORD_CNT); i++) {
    CRYPTO_store_u32_le(&V->words[i], B->words[i]);
  }
#else
  OPENSSL_memcpy(V, B, 2 * r * sizeof(block_t));
#endif
  for (uint64_t i = 1; i < N; i++) {
    scryptBlockMix(&V[2 * r * i /* scrypt block i */],
                   &V[2 * r * (i - 1) /* scrypt block i-1 */], r);
  }
  scryptBlockMix(B, &V[2 * r * (N - 1) /* scrypt block N-1 */], r);

  // Step 3.
  for (uint64_t i = 0; i < N; i++) {
    // Note this assumes |N| <= 2^32 and is a power of 2.
    uint32_t j = B[2 * r - 1].words[0] & (N - 1);
    for (size_t k = 0; k < 2 * r; k++) {
      xor_block(&T[k], &B[k], &V[2 * r * j + k]);
    }
    scryptBlockMix(B, T, r);
  }
#ifdef OPENSSL_BIG_ENDIAN
  for(size_t i = 0; i < (2 * r * SCRYPT_BLOCK_WORD_CNT); i++) {
    CRYPTO_store_u32_le(&B->words[i], B->words[i]);
  }
#endif
}

// SCRYPT_PR_MAX is the maximum value of p * r. This is equivalent to the
// bounds on p in section 6:
//
//   p <= ((2^32-1) * hLen) / MFLen iff
//   p <= ((2^32-1) * 32) / (128 * r) iff
//   p * r <= (2^30-1)
#define SCRYPT_PR_MAX ((1 << 30) - 1)

// SCRYPT_MAX_MEM is the default maximum memory that may be allocated by
// |EVP_PBE_scrypt|.
#define SCRYPT_MAX_MEM (1024 * 1024 * 32)

int EVP_PBE_scrypt(const char *password, size_t password_len,
                   const uint8_t *salt, size_t salt_len, uint64_t N, uint64_t r,
                   uint64_t p, size_t max_mem, uint8_t *out_key,
                   size_t key_len) {
  if (r == 0 || p == 0 || p > SCRYPT_PR_MAX / r ||
      // |N| must be a power of two.
      N < 2 || (N & (N - 1)) ||
      // We only support |N| <= 2^32 in |scryptROMix|.
      N > UINT64_C(1) << 32 ||
      // Check that |N| < 2^(128×r / 8).
      (16 * r <= 63 && N >= UINT64_C(1) << (16 * r))) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    return 0;
  }

  // Determine the amount of memory needed. B, T, and V are |p|, 1, and |N|
  // scrypt blocks, respectively. Each scrypt block is 2*|r| |block_t|s.
  if (max_mem == 0) {
    max_mem = SCRYPT_MAX_MEM;
  }

  size_t max_scrypt_blocks = max_mem / (2 * r * sizeof(block_t));
  if (max_scrypt_blocks < p + 1 ||
      max_scrypt_blocks - p - 1 < N) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MEMORY_LIMIT_EXCEEDED);
    return 0;
  }

  // Allocate and divide up the scratch space. |max_mem| fits in a size_t, which
  // is no bigger than uint64_t, so none of these operations may overflow.
  OPENSSL_STATIC_ASSERT(UINT64_MAX >= SIZE_MAX, size_t_exceeds_uint64_t)
  size_t B_blocks = p * 2 * r;
  size_t B_bytes = B_blocks * sizeof(block_t);
  size_t T_blocks = 2 * r;
  size_t V_blocks = N * 2 * r;
  block_t *B = OPENSSL_calloc((B_blocks + T_blocks + V_blocks), sizeof(block_t));
  if (B == NULL) {
    return 0;
  }

  int ret = 0;
  block_t *T = B + B_blocks;
  block_t *V = T + T_blocks;

  // NOTE: PKCS5_PBKDF2_HMAC can only fail due to allocation failure
  // or |iterations| of 0 (we pass 1 here). This is consistent with
  // the documented failure conditions of EVP_PBE_scrypt.
  if (!PKCS5_PBKDF2_HMAC(password, password_len, salt, salt_len, 1,
                         EVP_sha256(), B_bytes, (uint8_t *)B)) {
    goto err;
  }

  for (uint64_t i = 0; i < p; i++) {
    scryptROMix(B + 2 * r * i, r, N, T, V);
  }

  if (!PKCS5_PBKDF2_HMAC(password, password_len, (const uint8_t *)B, B_bytes, 1,
                         EVP_sha256(), key_len, out_key)) {
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(B);
  return ret;
}
