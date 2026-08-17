// Copyright 2011-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/aead.h>

#include <assert.h>

#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../aes/internal.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"
#include "internal.h"


struct ccm128_context {
  block128_f block;
  ctr128_f ctr;
  unsigned M, L;
};

struct ccm128_state {
  alignas(16) uint8_t nonce[16];
  alignas(16) uint8_t cmac[16];
};

static int CRYPTO_ccm128_init(struct ccm128_context *ctx, const AES_KEY *key,
                              block128_f block, ctr128_f ctr, unsigned M,
                              unsigned L) {
  if (M < 4 || M > 16 || (M & 1) != 0 || L < 2 || L > 8) {
    return 0;
  }
  ctx->block = block;
  ctx->ctr = ctr;
  ctx->M = M;
  ctx->L = L;
  return 1;
}

static size_t CRYPTO_ccm128_max_input(const struct ccm128_context *ctx) {
  return ctx->L >= sizeof(size_t) ? SIZE_MAX
                                  : (((size_t)1) << (ctx->L * 8)) - 1;
}

static int ccm128_init_state(const struct ccm128_context *ctx,
                             struct ccm128_state *state, const AES_KEY *key,
                             const uint8_t *nonce, size_t nonce_len,
                             const uint8_t *aad, size_t aad_len,
                             size_t plaintext_len) {
  const block128_f block = ctx->block;
  const unsigned M = ctx->M;
  const unsigned L = ctx->L;

  // |L| determines the expected |nonce_len| and the limit for |plaintext_len|.
  if (plaintext_len > CRYPTO_ccm128_max_input(ctx)  //
      || nonce_len != 15 - L) {
    return 0;
  }

  // Assemble the first block for computing the MAC.
  OPENSSL_memset(state, 0, sizeof(*state));
  state->nonce[0] = (uint8_t)((L - 1) | ((M - 2) / 2) << 3);
  if (aad_len != 0) {
    state->nonce[0] |= 0x40;  // Set AAD Flag
  }
  OPENSSL_memcpy(&state->nonce[1], nonce, nonce_len);
  for (unsigned i = 0; i < L; i++) {
    state->nonce[15 - i] = (uint8_t)(plaintext_len >> (8 * i));
  }

  (*block)(state->nonce, state->cmac, key);
  size_t blocks = 1;

  if (aad_len != 0) {
    unsigned i;
    // Cast to u64 to avoid the compiler complaining about invalid shifts.
    uint64_t aad_len_u64 = aad_len;
    if (aad_len_u64 < 0x10000 - 0x100) {
      state->cmac[0] ^= (uint8_t)(aad_len_u64 >> 8);
      state->cmac[1] ^= (uint8_t)aad_len_u64;
      i = 2;
    } else if (aad_len_u64 <= 0xffffffff) {
      state->cmac[0] ^= 0xff;
      state->cmac[1] ^= 0xfe;
      state->cmac[2] ^= (uint8_t)(aad_len_u64 >> 24);
      state->cmac[3] ^= (uint8_t)(aad_len_u64 >> 16);
      state->cmac[4] ^= (uint8_t)(aad_len_u64 >> 8);
      state->cmac[5] ^= (uint8_t)aad_len_u64;
      i = 6;
    } else {
      state->cmac[0] ^= 0xff;
      state->cmac[1] ^= 0xff;
      state->cmac[2] ^= (uint8_t)(aad_len_u64 >> 56);
      state->cmac[3] ^= (uint8_t)(aad_len_u64 >> 48);
      state->cmac[4] ^= (uint8_t)(aad_len_u64 >> 40);
      state->cmac[5] ^= (uint8_t)(aad_len_u64 >> 32);
      state->cmac[6] ^= (uint8_t)(aad_len_u64 >> 24);
      state->cmac[7] ^= (uint8_t)(aad_len_u64 >> 16);
      state->cmac[8] ^= (uint8_t)(aad_len_u64 >> 8);
      state->cmac[9] ^= (uint8_t)aad_len_u64;
      i = 10;
    }

    do {
      for (; i < 16 && aad_len != 0; i++) {
        state->cmac[i] ^= *aad;
        aad++;
        aad_len--;
      }
      (*block)(state->cmac, state->cmac, key);
      blocks++;
      i = 0;
    } while (aad_len != 0);
  }

  // Per RFC 3610, section 2.6, the total number of block cipher operations done
  // must not exceed 2^61. There are two block cipher operations remaining per
  // message block, plus one block at the end to encrypt the MAC.
  size_t remaining_blocks = 2 * ((plaintext_len + 15) / 16) + 1;
  if (plaintext_len + 15 < plaintext_len ||
      remaining_blocks + blocks < blocks ||
      (uint64_t)remaining_blocks + blocks > UINT64_C(1) << 61) {
    return 0;
  }

  // Assemble the first block for encrypting and decrypting. The bottom |L|
  // bytes are replaced with a counter and all bit the encoding of |L| is
  // cleared in the first byte.
  state->nonce[0] &= 7;
  return 1;
}

static int ccm128_encrypt(const struct ccm128_context *ctx,
                          struct ccm128_state *state, const AES_KEY *key,
                          uint8_t *out, const uint8_t *in, size_t len) {
  // The counter for encryption begins at one.
  for (unsigned i = 0; i < ctx->L; i++) {
    state->nonce[15 - i] = 0;
  }
  state->nonce[15] = 1;

  uint8_t partial_buf[16];
  unsigned num = 0;
  CRYPTO_ctr128_encrypt_ctr32(in, out, len, key, state->nonce, partial_buf,
                              &num, ctx->ctr);
  return 1;
}

static int ccm128_compute_mac(const struct ccm128_context *ctx,
                              struct ccm128_state *state, const AES_KEY *key,
                              uint8_t *out_tag, size_t tag_len,
                              const uint8_t *in, size_t len) {
  block128_f block = ctx->block;
  if (tag_len != ctx->M) {
    return 0;
  }

  // Incorporate |in| into the MAC.
  while (len >= 16) {
    CRYPTO_xor16(state->cmac, state->cmac, in);
    (*block)(state->cmac, state->cmac, key);
    in += 16;
    len -= 16;
  }
  if (len > 0) {
    for (size_t i = 0; i < len; i++) {
      state->cmac[i] ^= in[i];
    }
    (*block)(state->cmac, state->cmac, key);
  }

  // Encrypt the MAC with counter zero.
  for (unsigned i = 0; i < ctx->L; i++) {
    state->nonce[15 - i] = 0;
  }
  alignas(16) uint8_t tmp[16];
  (*block)(state->nonce, tmp, key);
  CRYPTO_xor16(state->cmac, state->cmac, tmp);

  OPENSSL_memcpy(out_tag, state->cmac, tag_len);
  return 1;
}

static int CRYPTO_ccm128_encrypt(const struct ccm128_context *ctx,
                                 const AES_KEY *key, uint8_t *out,
                                 uint8_t *out_tag, size_t tag_len,
                                 const uint8_t *nonce, size_t nonce_len,
                                 const uint8_t *in, size_t len,
                                 const uint8_t *aad, size_t aad_len) {
  struct ccm128_state state;
  return ccm128_init_state(ctx, &state, key, nonce, nonce_len, aad, aad_len,
                           len) &&
         ccm128_compute_mac(ctx, &state, key, out_tag, tag_len, in, len) &&
         ccm128_encrypt(ctx, &state, key, out, in, len);
}

static int CRYPTO_ccm128_decrypt(const struct ccm128_context *ctx,
                                 const AES_KEY *key, uint8_t *out,
                                 uint8_t *out_tag, size_t tag_len,
                                 const uint8_t *nonce, size_t nonce_len,
                                 const uint8_t *in, size_t len,
                                 const uint8_t *aad, size_t aad_len) {
  struct ccm128_state state;
  return ccm128_init_state(ctx, &state, key, nonce, nonce_len, aad, aad_len,
                           len) &&
         ccm128_encrypt(ctx, &state, key, out, in, len) &&
         ccm128_compute_mac(ctx, &state, key, out_tag, tag_len, out, len);
}

#define EVP_AEAD_AES_CCM_MAX_TAG_LEN 16

namespace {
struct aead_aes_ccm_ctx {
  union {
    double align;
    AES_KEY ks;
  } ks;
  struct ccm128_context ccm;
};
}  // namespace

static_assert(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                  sizeof(struct aead_aes_ccm_ctx),
              "AEAD state is too small");
static_assert(alignof(union evp_aead_ctx_st_state) >=
                  alignof(struct aead_aes_ccm_ctx),
              "AEAD state has insufficient alignment");

static int aead_aes_ccm_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                             size_t key_len, size_t tag_len, unsigned M,
                             unsigned L) {
  assert(M == EVP_AEAD_max_overhead(ctx->aead));
  assert(M == EVP_AEAD_max_tag_len(ctx->aead));
  assert(15 - L == EVP_AEAD_nonce_length(ctx->aead));

  if (key_len != EVP_AEAD_key_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_KEY_LENGTH);
    return 0;  // EVP_AEAD_CTX_init should catch this.
  }

  if (tag_len == EVP_AEAD_DEFAULT_TAG_LENGTH) {
    tag_len = M;
  }

  if (tag_len != M) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TAG_TOO_LARGE);
    return 0;
  }

  struct aead_aes_ccm_ctx *ccm_ctx = (struct aead_aes_ccm_ctx *)&ctx->state;

  block128_f block;
  ctr128_f ctr = aes_ctr_set_key(&ccm_ctx->ks.ks, NULL, &block, key, key_len);
  ctx->tag_len = tag_len;
  if (!CRYPTO_ccm128_init(&ccm_ctx->ccm, &ccm_ctx->ks.ks, block, ctr, M, L)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

static void aead_aes_ccm_cleanup(EVP_AEAD_CTX *ctx) {}

static int aead_aes_ccm_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  const struct aead_aes_ccm_ctx *ccm_ctx =
      (struct aead_aes_ccm_ctx *)&ctx->state;

  if (in_len > CRYPTO_ccm128_max_input(&ccm_ctx->ccm)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (max_out_tag_len < ctx->tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (nonce_len != EVP_AEAD_nonce_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (!CRYPTO_ccm128_encrypt(&ccm_ctx->ccm, &ccm_ctx->ks.ks, out, out_tag,
                             ctx->tag_len, nonce, nonce_len, in, in_len, ad,
                             ad_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  *out_tag_len = ctx->tag_len;
  AEAD_CCM_verify_service_indicator(ctx);
  return 1;
}

static int aead_aes_ccm_open_gather(const EVP_AEAD_CTX *ctx, uint8_t *out,
                                    const uint8_t *nonce, size_t nonce_len,
                                    const uint8_t *in, size_t in_len,
                                    const uint8_t *in_tag, size_t in_tag_len,
                                    const uint8_t *ad, size_t ad_len) {
  const struct aead_aes_ccm_ctx *ccm_ctx =
      (struct aead_aes_ccm_ctx *)&ctx->state;

  if (in_len > CRYPTO_ccm128_max_input(&ccm_ctx->ccm)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (nonce_len != EVP_AEAD_nonce_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (in_tag_len != ctx->tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  uint8_t tag[EVP_AEAD_AES_CCM_MAX_TAG_LEN];
  assert(ctx->tag_len <= EVP_AEAD_AES_CCM_MAX_TAG_LEN);
  if (!CRYPTO_ccm128_decrypt(&ccm_ctx->ccm, &ccm_ctx->ks.ks, out, tag,
                             ctx->tag_len, nonce, nonce_len, in, in_len, ad,
                             ad_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (CRYPTO_memcmp(tag, in_tag, ctx->tag_len) != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  AEAD_CCM_verify_service_indicator(ctx);
  return 1;
}

static int aead_aes_ccm_bluetooth_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                       size_t key_len, size_t tag_len) {
  return aead_aes_ccm_init(ctx, key, key_len, tag_len, 4, 2);
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_ccm_bluetooth) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = 13;
  out->overhead = 4;
  out->max_tag_len = 4;

  out->init = aead_aes_ccm_bluetooth_init;
  out->cleanup = aead_aes_ccm_cleanup;
  out->seal_scatter = aead_aes_ccm_seal_scatter;
  out->open_gather = aead_aes_ccm_open_gather;
}

static int aead_aes_ccm_bluetooth_8_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                         size_t key_len, size_t tag_len) {
  return aead_aes_ccm_init(ctx, key, key_len, tag_len, 8, 2);
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_ccm_bluetooth_8) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = 13;
  out->overhead = 8;
  out->max_tag_len = 8;

  out->init = aead_aes_ccm_bluetooth_8_init;
  out->cleanup = aead_aes_ccm_cleanup;
  out->seal_scatter = aead_aes_ccm_seal_scatter;
  out->open_gather = aead_aes_ccm_open_gather;
}

static int aead_aes_ccm_matter_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                    size_t key_len, size_t tag_len) {
  return aead_aes_ccm_init(ctx, key, key_len, tag_len, 16, 2);
}

DEFINE_METHOD_FUNCTION(EVP_AEAD, EVP_aead_aes_128_ccm_matter) {
  memset(out, 0, sizeof(EVP_AEAD));

  out->key_len = 16;
  out->nonce_len = 13;
  out->overhead = 16;
  out->max_tag_len = 16;

  out->init = aead_aes_ccm_matter_init;
  out->cleanup = aead_aes_ccm_cleanup;
  out->seal_scatter = aead_aes_ccm_seal_scatter;
  out->open_gather = aead_aes_ccm_open_gather;
}
