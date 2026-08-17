// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/aead.h>
#include <openssl/cipher.h>
#include <openssl/crypto.h>
#include <openssl/err.h>
#include <openssl/sha.h>

#include "../fipsmodule/cipher/internal.h"


#define EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN SHA256_DIGEST_LENGTH
#define EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN 12

struct aead_aes_ctr_hmac_sha256_ctx {
  union {
    double align;
    AES_KEY ks;
  } ks;
  ctr128_f ctr;
  block128_f block;
  SHA256_CTX inner_init_state;
  SHA256_CTX outer_init_state;
};

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(struct aead_aes_ctr_hmac_sha256_ctx),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(struct aead_aes_ctr_hmac_sha256_ctx),
                      AEAD_state_has_insufficient_alignment)

static void hmac_init(SHA256_CTX *out_inner, SHA256_CTX *out_outer,
                      const uint8_t hmac_key[32]) {
  static const size_t hmac_key_len = 32;
  uint8_t block[SHA256_CBLOCK];
  OPENSSL_memcpy(block, hmac_key, hmac_key_len);
  OPENSSL_memset(block + hmac_key_len, 0x36, sizeof(block) - hmac_key_len);

  unsigned i;
  for (i = 0; i < hmac_key_len; i++) {
    block[i] ^= 0x36;
  }

  SHA256_Init(out_inner);
  SHA256_Update(out_inner, block, sizeof(block));

  OPENSSL_memset(block + hmac_key_len, 0x5c, sizeof(block) - hmac_key_len);
  for (i = 0; i < hmac_key_len; i++) {
    block[i] ^= (0x36 ^ 0x5c);
  }

  SHA256_Init(out_outer);
  SHA256_Update(out_outer, block, sizeof(block));
}

static int aead_aes_ctr_hmac_sha256_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                         size_t key_len, size_t tag_len) {
  struct aead_aes_ctr_hmac_sha256_ctx *aes_ctx =
      (struct aead_aes_ctr_hmac_sha256_ctx *)&ctx->state;
  static const size_t hmac_key_len = 32;

  if (key_len < hmac_key_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_KEY_LENGTH);
    return 0;  // EVP_AEAD_CTX_init should catch this.
  }

  const size_t aes_key_len = key_len - hmac_key_len;
  if (aes_key_len != 16 && aes_key_len != 32) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_KEY_LENGTH);
    return 0;  // EVP_AEAD_CTX_init should catch this.
  }

  if (tag_len == EVP_AEAD_DEFAULT_TAG_LENGTH) {
    tag_len = EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN;
  }

  if (tag_len > EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TAG_TOO_LARGE);
    return 0;
  }

  aes_ctx->ctr =
      aes_ctr_set_key(&aes_ctx->ks.ks, NULL, &aes_ctx->block, key, aes_key_len);
  ctx->tag_len = tag_len;
  hmac_init(&aes_ctx->inner_init_state, &aes_ctx->outer_init_state,
            key + aes_key_len);

  return 1;
}

static void aead_aes_ctr_hmac_sha256_cleanup(EVP_AEAD_CTX *ctx) {}

static void hmac_update_uint64(SHA256_CTX *sha256, uint64_t value) {
  unsigned i;
  uint8_t bytes[8];

  for (i = 0; i < sizeof(bytes); i++) {
    bytes[i] = value & 0xff;
    value >>= 8;
  }
  SHA256_Update(sha256, bytes, sizeof(bytes));
}

static void hmac_calculate(uint8_t out[SHA256_DIGEST_LENGTH],
                           const SHA256_CTX *inner_init_state,
                           const SHA256_CTX *outer_init_state,
                           const uint8_t *ad, size_t ad_len,
                           const uint8_t *nonce, const uint8_t *ciphertext,
                           size_t ciphertext_len) {
  SHA256_CTX sha256;
  OPENSSL_memcpy(&sha256, inner_init_state, sizeof(sha256));
  hmac_update_uint64(&sha256, ad_len);
  hmac_update_uint64(&sha256, ciphertext_len);
  SHA256_Update(&sha256, nonce, EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN);
  SHA256_Update(&sha256, ad, ad_len);

  // Pad with zeros to the end of the SHA-256 block.
  const unsigned num_padding =
      (SHA256_CBLOCK - ((sizeof(uint64_t) * 2 +
                         EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN + ad_len) %
                        SHA256_CBLOCK)) %
      SHA256_CBLOCK;
  uint8_t padding[SHA256_CBLOCK];
  OPENSSL_memset(padding, 0, num_padding);
  SHA256_Update(&sha256, padding, num_padding);

  SHA256_Update(&sha256, ciphertext, ciphertext_len);

  uint8_t inner_digest[SHA256_DIGEST_LENGTH];
  SHA256_Final(inner_digest, &sha256);

  OPENSSL_memcpy(&sha256, outer_init_state, sizeof(sha256));
  SHA256_Update(&sha256, inner_digest, sizeof(inner_digest));
  SHA256_Final(out, &sha256);
}

static void aead_aes_ctr_hmac_sha256_crypt(
    const struct aead_aes_ctr_hmac_sha256_ctx *aes_ctx, uint8_t *out,
    const uint8_t *in, size_t len, const uint8_t *nonce) {
  // Since the AEAD operation is one-shot, keeping a buffer of unused keystream
  // bytes is pointless. However, |CRYPTO_ctr128_encrypt| requires it.
  uint8_t partial_block_buffer[AES_BLOCK_SIZE];
  unsigned partial_block_offset = 0;
  OPENSSL_memset(partial_block_buffer, 0, sizeof(partial_block_buffer));

  uint8_t counter[AES_BLOCK_SIZE];
  OPENSSL_memcpy(counter, nonce, EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN);
  OPENSSL_memset(counter + EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN, 0, 4);

  if (aes_ctx->ctr) {
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, &aes_ctx->ks.ks, counter,
                                partial_block_buffer, &partial_block_offset,
                                aes_ctx->ctr);
  } else {
    CRYPTO_ctr128_encrypt(in, out, len, &aes_ctx->ks.ks, counter,
                          partial_block_buffer, &partial_block_offset,
                          aes_ctx->block);
  }
}

static int aead_aes_ctr_hmac_sha256_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  const struct aead_aes_ctr_hmac_sha256_ctx *aes_ctx =
      (struct aead_aes_ctr_hmac_sha256_ctx *)&ctx->state;
  const uint64_t in_len_64 = in_len;

  if (in_len_64 >= (UINT64_C(1) << 32) * AES_BLOCK_SIZE) {
    // This input is so large it would overflow the 32-bit block counter.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (max_out_tag_len < ctx->tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (nonce_len != EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  aead_aes_ctr_hmac_sha256_crypt(aes_ctx, out, in, in_len, nonce);

  uint8_t hmac_result[SHA256_DIGEST_LENGTH];
  hmac_calculate(hmac_result, &aes_ctx->inner_init_state,
                 &aes_ctx->outer_init_state, ad, ad_len, nonce, out, in_len);
  OPENSSL_memcpy(out_tag, hmac_result, ctx->tag_len);
  *out_tag_len = ctx->tag_len;

  return 1;
}

static int aead_aes_ctr_hmac_sha256_open_gather(
    const EVP_AEAD_CTX *ctx, uint8_t *out, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *in_tag,
    size_t in_tag_len, const uint8_t *ad, size_t ad_len) {
  const struct aead_aes_ctr_hmac_sha256_ctx *aes_ctx =
      (struct aead_aes_ctr_hmac_sha256_ctx *)&ctx->state;

  if (in_tag_len != ctx->tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  if (nonce_len != EVP_AEAD_AES_CTR_HMAC_SHA256_NONCE_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  uint8_t hmac_result[SHA256_DIGEST_LENGTH];
  hmac_calculate(hmac_result, &aes_ctx->inner_init_state,
                 &aes_ctx->outer_init_state, ad, ad_len, nonce, in, in_len);
  if (CRYPTO_memcmp(hmac_result, in_tag, ctx->tag_len) != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  aead_aes_ctr_hmac_sha256_crypt(aes_ctx, out, in, in_len, nonce);

  return 1;
}

static const EVP_AEAD aead_aes_128_ctr_hmac_sha256 = {
    16 /* AES key */ + 32 /* HMAC key */,
    12,                                    // nonce length
    EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN,  // overhead
    EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN,  // max tag length
    AEAD_AES_128_CTR_HMAC_SHA256_ID,       // evp_aead_id
    0,                                     // seal_scatter_supports_extra_in

    aead_aes_ctr_hmac_sha256_init,
    NULL /* init_with_direction */,
    aead_aes_ctr_hmac_sha256_cleanup,
    NULL /* open */,
    aead_aes_ctr_hmac_sha256_seal_scatter,
    aead_aes_ctr_hmac_sha256_open_gather,
    NULL /* get_iv */,
    NULL /* tag_len */,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_256_ctr_hmac_sha256 = {
    32 /* AES key */ + 32 /* HMAC key */,
    12,                                    // nonce length
    EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN,  // overhead
    EVP_AEAD_AES_CTR_HMAC_SHA256_TAG_LEN,  // max tag length
    AEAD_AES_256_CTR_HMAC_SHA256_ID,       // evp_aead_id
    0,                                     // seal_scatter_supports_extra_in

    aead_aes_ctr_hmac_sha256_init,
    NULL /* init_with_direction */,
    aead_aes_ctr_hmac_sha256_cleanup,
    NULL /* open */,
    aead_aes_ctr_hmac_sha256_seal_scatter,
    aead_aes_ctr_hmac_sha256_open_gather,
    NULL /* get_iv */,
    NULL /* tag_len */,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

const EVP_AEAD *EVP_aead_aes_128_ctr_hmac_sha256(void) {
  return &aead_aes_128_ctr_hmac_sha256;
}

const EVP_AEAD *EVP_aead_aes_256_ctr_hmac_sha256(void) {
  return &aead_aes_256_ctr_hmac_sha256;
}
