// Copyright (c) 2008 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/aead.h>

#include <assert.h>

#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../delocate.h"
#include "../modes/internal.h"
#include "../service_indicator/internal.h"
#include "internal.h"

#define EVP_AEAD_AES_CCM_MIN_TAG_LEN 4
#define EVP_AEAD_AES_CCM_MAX_TAG_LEN 16
#define CCM_MAX_NONCE_LEN 13

typedef struct ccm128_context {
  block128_f block;
  ctr128_f ctr;
  uint32_t M, L;
} CCM128_CTX;

typedef struct ccm128_state {
  alignas(16) uint8_t nonce[16];
  alignas(16) uint8_t cmac[16];
} CCM128_STATE;

typedef struct cipher_aes_ccm_ctx {
  union {
    uint64_t align;
    AES_KEY ks;
  } ks; // AES key schedule to use

  CCM128_CTX ccm;
  CCM128_STATE ccm_state;

  // Boolean flags
  uint8_t key_set;
  uint8_t iv_set;
  uint8_t tag_set;
  uint8_t len_set;
  uint8_t ccm_set;

  // L and M parameters from RFC3610
  uint32_t L; // Number of octets in length field
  uint32_t M; // Number of octets in authentication field

  size_t message_len;
  uint8_t tag[EVP_AEAD_AES_CCM_MAX_TAG_LEN];
  uint8_t nonce[CCM_MAX_NONCE_LEN];
} CIPHER_AES_CCM_CTX;

// The "inner" CCM128_CTX struct within a CIPHER_AES_CCM_CTX
#define CCM_INNER_CTX(ccm_ctx) (&ccm_ctx->ccm)
// The CCM128 state struct within a CIPHER_AES_CCM_CTX
#define CCM_INNER_STATE(ccm_ctx) (&ccm_ctx->ccm_state)

// As per RFC3610, the nonce length in bytes is 15 - L.
#define CCM_L_TO_NONCE_LEN(L) (15 - (L))

static int CRYPTO_ccm128_init(struct ccm128_context *ctx, block128_f block,
                              ctr128_f ctr, unsigned M, unsigned L) {
  if (M < EVP_AEAD_AES_CCM_MIN_TAG_LEN || M > EVP_AEAD_AES_CCM_MAX_TAG_LEN
      || (M & 1) != 0 || L < 2 || L > 8) {
    return 0;
  }
  if (block) {
    ctx->block = block;
  }
  if (ctr) {
    ctx->ctr = ctr;
  }
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
  const uint32_t M = ctx->M;
  const uint32_t L = ctx->L;

  // |L| determines the expected |nonce_len| and the limit for |plaintext_len|.
  if (plaintext_len > CRYPTO_ccm128_max_input(ctx) ||
      nonce_len != CCM_L_TO_NONCE_LEN(L)) {
    return 0;
  }

  // Assemble the first block for computing the MAC.
  OPENSSL_memset(state, 0, sizeof(*state));
  state->nonce[0] = (uint8_t)((L - 1) | ((M - 2) / 2) << 3);
  if (aad_len != 0) {
    state->nonce[0] |= 0x40;  // Set AAD Flag
  }
  OPENSSL_memcpy(&state->nonce[1], nonce, nonce_len);
  // Explicitly cast plaintext_len up to 64-bits so that we don't shift out of
  // bounds on 32-bit machines when encoding the message length.
  uint64_t plaintext_len_64 = plaintext_len;
  for (uint32_t i = 0; i < L; i++) {
    state->nonce[15 - i] = (uint8_t)(plaintext_len_64 >> (8 * i));
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
      (uint64_t) remaining_blocks + blocks > UINT64_C(1) << 61) {
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
  if (ctx->ctr != NULL) {
    CRYPTO_ctr128_encrypt_ctr32(in, out, len, key, state->nonce, partial_buf,
                                &num, ctx->ctr);
  } else {
    CRYPTO_ctr128_encrypt(in, out, len, key, state->nonce, partial_buf, &num,
                          ctx->block);
  }
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

struct aead_aes_ccm_ctx {
  union {
    double align;
    AES_KEY ks;
  } ks;
  struct ccm128_context ccm;
};

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(struct aead_aes_ccm_ctx),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(struct aead_aes_ccm_ctx),
                      AEAD_state_has_insufficient_alignment)

static int aead_aes_ccm_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                             size_t key_len, size_t tag_len, unsigned M,
                             unsigned L) {
  assert(M == EVP_AEAD_max_overhead(ctx->aead));
  assert(M == EVP_AEAD_max_tag_len(ctx->aead));
  assert(CCM_L_TO_NONCE_LEN(L) == EVP_AEAD_nonce_length(ctx->aead));

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
  if (!CRYPTO_ccm128_init(&ccm_ctx->ccm, block, ctr, M, L)) {
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
  out->aead_id = AEAD_AES_128_CCM_BLUETOOTH_ID;
  out->seal_scatter_supports_extra_in = 0;

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
  out->aead_id = AEAD_AES_128_CCM_BLUETOOTH_8_ID;
  out->seal_scatter_supports_extra_in = 0;

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
  out->aead_id = AEAD_AES_128_CCM_MATTER_ID;
  out->max_tag_len = 16;

  out->init = aead_aes_ccm_matter_init;
  out->cleanup = aead_aes_ccm_cleanup;
  out->seal_scatter = aead_aes_ccm_seal_scatter;
  out->open_gather = aead_aes_ccm_open_gather;
}

#if defined(OPENSSL_32_BIT)
#define CIPHER_AES_CCM_CTX_PADDING (4 + 8)
#else
#define CIPHER_AES_CCM_CTX_PADDING 8
#endif

// This is the same handling as EVP_AES_GCM_CTX which is also a context
// that is 16-byte aligned.
// TODO: possibly refactor the code instead of repeating it from e_aes.c
static CIPHER_AES_CCM_CTX *aes_ccm_from_cipher_ctx(EVP_CIPHER_CTX *ctx) {
  OPENSSL_STATIC_ASSERT(
      alignof(CIPHER_AES_CCM_CTX) <= 16,
      EVP_AES_CCM_CTX_needs_more_alignment_than_this_function_provides)

  // |malloc| guarantees up to 4-byte alignment on 32-bit and 8-byte alignment
  // on 64-bit systems, so we need to adjust to reach 16-byte alignment.
  assert(ctx->cipher->ctx_size ==
         sizeof(CIPHER_AES_CCM_CTX) + CIPHER_AES_CCM_CTX_PADDING);

  char *ptr = ctx->cipher_data;
#if defined(OPENSSL_32_BIT)
  assert((uintptr_t)ptr % 4 == 0);
  ptr += (uintptr_t)ptr & 4;
#endif
  assert((uintptr_t)ptr % 8 == 0);
  ptr += (uintptr_t)ptr & 8;
  return (CIPHER_AES_CCM_CTX *)ptr;
}

static int cipher_aes_ccm_init(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                        const uint8_t *iv, int enc) {
  CIPHER_AES_CCM_CTX *cipher_ctx = aes_ccm_from_cipher_ctx(ctx);
  if (!iv && !key) {
    return 1;
  }
  if (key) {
    block128_f block;
    ctr128_f ctr = aes_ctr_set_key(&cipher_ctx->ks.ks, NULL, &block, key,
                                   ctx->key_len);
    if (!CRYPTO_ccm128_init(&cipher_ctx->ccm, block, ctr, cipher_ctx->M,
                       cipher_ctx->L)) {
      return 0;
    }
    cipher_ctx->key_set = 1;
  }
  if (iv) {
    if (!CRYPTO_ccm128_init(&cipher_ctx->ccm, NULL, NULL, cipher_ctx->M,
                       cipher_ctx->L)) {
      return 0;
    }
    OPENSSL_memcpy(cipher_ctx->nonce, iv, CCM_L_TO_NONCE_LEN(cipher_ctx->L));
    cipher_ctx->iv_set = 1;
  }
  return 1;
}

static int cipher_aes_ccm_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                                 const uint8_t *in, size_t len) {
  CIPHER_AES_CCM_CTX *cipher_ctx = aes_ccm_from_cipher_ctx(ctx);
  CCM128_CTX *ccm_ctx = CCM_INNER_CTX(cipher_ctx);
  CCM128_STATE *ccm_state = CCM_INNER_STATE(cipher_ctx);

  // Implicit EVP_*Final call. CCM does all the work in EVP_*Update
  // n.b. |out| is non-NULL and |in| is NULL despite being a no-op.
  if (in == NULL && out != NULL) {
    return 0;
  }

  if (!cipher_ctx->iv_set || !cipher_ctx->key_set) {
    return -1;
  }

  if (!out) {
    if (!in) {
      // If |out| and |in| are both NULL, |len| is the total length of the
      // message which we need to include that in the 0th block of the CBC-MAC.
      cipher_ctx->message_len = len;
      cipher_ctx->len_set = 1;
      return len;
    } else {
      // If only |out| is NULL then this is the AAD.
      // The message length must be set apriori.
      if (!cipher_ctx->len_set && len) {
        return -1;
      }
      // We now have everything we need to initialize the CBC-MAC state
      if (ccm128_init_state(ccm_ctx, ccm_state,
                            &cipher_ctx->ks.ks, cipher_ctx->nonce,
                            CCM_L_TO_NONCE_LEN(cipher_ctx->L), in, len,
                            cipher_ctx->message_len)) {
        cipher_ctx->ccm_set = 1;
        return len;
      } else {
        return -1;
      }
    }
  }

  // The tag must be set before decrypting any data.
  if (!EVP_CIPHER_CTX_encrypting(ctx) && !cipher_ctx->tag_set) {
    return -1;
  }
  if (!cipher_ctx->len_set) {
    return -1;
  }
  if (!cipher_ctx->ccm_set) {
    // Initialize the ccm_state if this did not happen during the AAD update.
    if (!ccm128_init_state(ccm_ctx, ccm_state, &cipher_ctx->ks.ks,
                           cipher_ctx->nonce, CCM_L_TO_NONCE_LEN(cipher_ctx->L),
                           NULL, 0, cipher_ctx->message_len)) {
      return -1;
    }
    cipher_ctx->ccm_set = 1;
  }

  if (EVP_CIPHER_CTX_encrypting(ctx)) {
    // Encryption path. Compute CBC-MAC on plaintext and then encrypt.
    if (!ccm128_compute_mac(ccm_ctx, ccm_state, &cipher_ctx->ks.ks,
                            cipher_ctx->tag, cipher_ctx->M, in, len)) {
      return -1;
    }
    if (!ccm128_encrypt(ccm_ctx, ccm_state, &cipher_ctx->ks.ks, out, in, len)) {
      return -1;
    }
    cipher_ctx->tag_set = 1;
  } else {
    // Decryption path. Compute the plaintext then compute its CBC-MAC.
    // n.b. The method says encrypt, but it works both ways.
    if (!ccm128_encrypt(ccm_ctx, ccm_state, &cipher_ctx->ks.ks, out, in, len)) {
      return -1;
    }
    uint8_t computed_tag[EVP_AEAD_AES_CCM_MAX_TAG_LEN] = {0};
    if (!ccm128_compute_mac(ccm_ctx, ccm_state, &cipher_ctx->ks.ks,
                            computed_tag, cipher_ctx->M, out, len)) {
      OPENSSL_cleanse(out, len);
      return -1;
    }
    // Validate the tag and invalidate the output if it doesn't match.
    if (CRYPTO_memcmp(cipher_ctx->tag, computed_tag, cipher_ctx->M)) {
      OPENSSL_cleanse(out, len);
      return -1;
    }
    cipher_ctx->iv_set = 0;
    cipher_ctx->tag_set = 0;
    cipher_ctx->len_set = 0;
    cipher_ctx->ccm_set = 0;
  }
  return (int) len;
}

static int cipher_aes_ccm_ctrl_set_L(CIPHER_AES_CCM_CTX *ctx, int L) {
  if (L < 2 || L > 8) {
    return 0;
  }
  ctx->L = L;
  return 1;
}

static int cipher_aes_ccm_ctrl(EVP_CIPHER_CTX *ctx, int type, int arg,
                               void *ptr) {
  CIPHER_AES_CCM_CTX *cipher_ctx = aes_ccm_from_cipher_ctx(ctx);
  switch (type) {
    case EVP_CTRL_INIT:
      OPENSSL_cleanse(cipher_ctx, sizeof(CIPHER_AES_CCM_CTX));
      cipher_ctx->key_set = 0;
      cipher_ctx->iv_set = 0;
      cipher_ctx->tag_set = 0;
      cipher_ctx->len_set = 0;
      cipher_ctx->ccm_set = 0;
      cipher_ctx->L = 8;
      cipher_ctx->M = 14;
      cipher_ctx->message_len = 0;
      return 1;
    case EVP_CTRL_GET_IVLEN:
      *(int *)ptr = CCM_L_TO_NONCE_LEN(cipher_ctx->L);
      return 1;
    case EVP_CTRL_AEAD_SET_IVLEN:
      // The nonce (IV) length is 15-L, compute L here and set it below to "set"
      // the IV length.
      return cipher_aes_ccm_ctrl_set_L(cipher_ctx, 15 - arg);
    case EVP_CTRL_CCM_SET_L:
      return cipher_aes_ccm_ctrl_set_L(cipher_ctx, arg);
    case EVP_CTRL_AEAD_SET_TAG:
      // |arg| is the tag length in bytes.
      if ((arg & 1) || arg < EVP_AEAD_AES_CCM_MIN_TAG_LEN
          || arg > EVP_AEAD_AES_CCM_MAX_TAG_LEN) {
        return 0;
      }

      // If encrypting, we don't expect incoming tag data
      if (ctx->encrypt && ptr) {
        return 0;
      }

      if (ptr) {
        // Set the tag for validation when decrypting.
        OPENSSL_memcpy(cipher_ctx->tag, ptr, arg);
        cipher_ctx->tag_set = 1;
      }

      // Set the value of M (i.e. the tag length) when encrypting.
      cipher_ctx->M = arg;
      return 1;
    case EVP_CTRL_AEAD_GET_TAG:
      if (!ctx->encrypt || !cipher_ctx->tag_set) {
        return 0;
      }
      if ((size_t) arg != cipher_ctx->M) {
        return 0;
      }
      OPENSSL_memcpy(ptr, cipher_ctx->tag, cipher_ctx->M);
      cipher_ctx->tag_set = 0;
      cipher_ctx->iv_set = 0;
      cipher_ctx->len_set = 0;
      cipher_ctx->ccm_set = 0;
      return 1;

    case EVP_CTRL_COPY: {
      EVP_CIPHER_CTX *out = ptr;
      CIPHER_AES_CCM_CTX *cipher_ctx_out = aes_ccm_from_cipher_ctx(out);

      // |EVP_CIPHER_CTX_copy| copies this generically, but we must redo it in
      // case |out->cipher_data| and |in->cipher_data| are differently aligned.
      OPENSSL_memcpy(cipher_ctx_out, cipher_ctx, sizeof(CIPHER_AES_CCM_CTX));
      return 1;
    }

    default:
      return -1;
  }
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_128_ccm) {
  memset(out, 0, sizeof(EVP_CIPHER));
  out->nid = NID_aes_128_ccm;
  out->block_size = 1; // stream cipher
  out->key_len = 16;
  out->iv_len = 13;
  out->ctx_size = sizeof(CIPHER_AES_CCM_CTX) + CIPHER_AES_CCM_CTX_PADDING;
  out->flags = EVP_CIPH_CCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = cipher_aes_ccm_init;
  out->cipher = cipher_aes_ccm_cipher;
  out->cleanup = NULL;
  out->ctrl = cipher_aes_ccm_ctrl;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_192_ccm) {
  memset(out, 0, sizeof(EVP_CIPHER));
  out->nid = NID_aes_192_ccm;
  out->block_size = 1; // stream cipher
  out->key_len = 24;
  out->iv_len = 13;
  out->ctx_size = sizeof(CIPHER_AES_CCM_CTX) + CIPHER_AES_CCM_CTX_PADDING;
  out->flags = EVP_CIPH_CCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = cipher_aes_ccm_init;
  out->cipher = cipher_aes_ccm_cipher;
  out->cleanup = NULL;
  out->ctrl = cipher_aes_ccm_ctrl;
}

DEFINE_METHOD_FUNCTION(EVP_CIPHER, EVP_aes_256_ccm) {
  memset(out, 0, sizeof(EVP_CIPHER));
  out->nid = NID_aes_256_ccm;
  out->block_size = 1; // stream cipher
  out->key_len = 32;
  out->iv_len = 13;
  out->ctx_size = sizeof(CIPHER_AES_CCM_CTX) + CIPHER_AES_CCM_CTX_PADDING;
  out->flags = EVP_CIPH_CCM_MODE | EVP_CIPH_CUSTOM_IV | EVP_CIPH_CUSTOM_COPY |
               EVP_CIPH_FLAG_CUSTOM_CIPHER | EVP_CIPH_ALWAYS_CALL_INIT |
               EVP_CIPH_CTRL_INIT | EVP_CIPH_FLAG_AEAD_CIPHER;
  out->init = cipher_aes_ccm_init;
  out->cipher = cipher_aes_ccm_cipher;
  out->cleanup = NULL;
  out->ctrl = cipher_aes_ccm_ctrl;
}
