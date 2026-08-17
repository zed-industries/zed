// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/aead.h>

#include <string.h>

#include <openssl/chacha.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/poly1305.h>
#include <openssl/type_check.h>

#include "../chacha/internal.h"
#include "../fipsmodule/cipher/internal.h"
#include "../internal.h"
#include "internal.h"

#define CHACHA_KEY_LEN 32
#define CHACHA_IV_LEN 12
#define CHACHA_BLOCK_LEN 64
#define CHACHA_CTR_IV_LEN 16

// ChaCha-Poly specific context within an EVP_CIPHER_CTX
#define CCP_CTX(ctx) ((CIPHER_CHACHA_POLY_CTX *) ctx->cipher_data)
// Return the CIPHER_CHACHA_KEY from a CIPHER_CHACHA_POLY_CTX
#define CC_KEY(ccp) (&(ccp)->key)
// Return the poly1305_state from a CIPHER_CHACHA_POLY_CTX
#define POLY_CTX(ccp) (&(ccp)->poly_ctx)

// Struct for Poly1305 key within an EVP_AEAD_CTX
typedef struct aead_chacha20_poly1305_ctx {
  uint8_t key[CHACHA_KEY_LEN];
} AEAD_CHACHA_POLY_CTX;

// Struct for ChaCha key within an EVP_CIPHER_CTX
typedef struct {
  uint32_t key[CHACHA_KEY_LEN / 4];
  // Buffer containing both the counter and nonce.
  // The index 0 is the counter and the remaining portion is the nonce.
  uint32_t counter_nonce[CHACHA_CTR_IV_LEN / 4];
  // Buffer for any partially used keys
  uint8_t buf[CHACHA_BLOCK_LEN];
  uint32_t partial_len;
} CIPHER_CHACHA_KEY;

typedef struct cipher_chacha_poly_ctx {
  CIPHER_CHACHA_KEY key;
  uint32_t iv[CHACHA_IV_LEN / 4];
  uint8_t tag_len;
  uint8_t tag[POLY1305_TAG_LEN];
  // Use 64-bit integers so this struct can be passed directly into poly1305
  struct { uint64_t aad, text; } len;
  int32_t poly_initialized;
  int32_t pad_aad;
  poly1305_state poly_ctx;
} CIPHER_CHACHA_POLY_CTX;

OPENSSL_STATIC_ASSERT(sizeof(((EVP_AEAD_CTX *)NULL)->state) >=
                          sizeof(AEAD_CHACHA_POLY_CTX),
                      AEAD_state_is_too_small)
OPENSSL_STATIC_ASSERT(alignof(union evp_aead_ctx_st_state) >=
                          alignof(AEAD_CHACHA_POLY_CTX),
                      AEAD_state_has_insufficient_alignment)

static int aead_chacha20_poly1305_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                       size_t key_len, size_t tag_len) {
  AEAD_CHACHA_POLY_CTX *c20_ctx =
      (AEAD_CHACHA_POLY_CTX *)&ctx->state;

  if (tag_len == 0) {
    tag_len = POLY1305_TAG_LEN;
  }

  if (tag_len > POLY1305_TAG_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (key_len != sizeof(c20_ctx->key)) {
    return 0;  // internal error - EVP_AEAD_CTX_init should catch this.
  }

  OPENSSL_memcpy(c20_ctx->key, key, key_len);
  ctx->tag_len = tag_len;

  return 1;
}

static void aead_chacha20_poly1305_cleanup(EVP_AEAD_CTX *ctx) {}

static void poly1305_update_length(poly1305_state *poly1305, size_t data_len) {
  uint8_t length_bytes[8];

  for (unsigned i = 0; i < sizeof(length_bytes); i++) {
    length_bytes[i] = data_len;
    data_len >>= 8;
  }

  CRYPTO_poly1305_update(poly1305, length_bytes, sizeof(length_bytes));
}

// calc_tag fills |tag| with the authentication tag for the given inputs.
static void calc_tag(uint8_t tag[POLY1305_TAG_LEN], const uint8_t *key,
                     const uint8_t nonce[CHACHA_IV_LEN], const uint8_t *ad,
                     size_t ad_len, const uint8_t *ciphertext,
                     size_t ciphertext_len, const uint8_t *ciphertext_extra,
                     size_t ciphertext_extra_len) {
  alignas(16) uint8_t poly1305_key[CHACHA_KEY_LEN];
  OPENSSL_memset(poly1305_key, 0, sizeof(poly1305_key));
  CRYPTO_chacha_20(poly1305_key, poly1305_key, sizeof(poly1305_key), key, nonce,
                   0);

  static const uint8_t padding[16] = {0};  // Padding is all zeros.
  poly1305_state ctx;
  CRYPTO_poly1305_init(&ctx, poly1305_key);
  CRYPTO_poly1305_update(&ctx, ad, ad_len);
  if (ad_len % 16 != 0) {
    CRYPTO_poly1305_update(&ctx, padding, sizeof(padding) - (ad_len % 16));
  }
  CRYPTO_poly1305_update(&ctx, ciphertext, ciphertext_len);
  CRYPTO_poly1305_update(&ctx, ciphertext_extra, ciphertext_extra_len);
  const size_t ciphertext_total = ciphertext_len + ciphertext_extra_len;
  if (ciphertext_total % 16 != 0) {
    CRYPTO_poly1305_update(&ctx, padding,
                           sizeof(padding) - (ciphertext_total % 16));
  }
  poly1305_update_length(&ctx, ad_len);
  poly1305_update_length(&ctx, ciphertext_total);
  CRYPTO_poly1305_finish(&ctx, tag);
}

static int chacha20_poly1305_seal_scatter(
    const uint8_t *key, uint8_t *out, uint8_t *out_tag, size_t *out_tag_len,
    size_t max_out_tag_len, const uint8_t *nonce, size_t nonce_len,
    const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len, size_t tag_len) {
  if (extra_in_len + tag_len < tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }
  if (max_out_tag_len < tag_len + extra_in_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }
  if (nonce_len != CHACHA_IV_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  // |CRYPTO_chacha_20| uses a 32-bit block counter. Therefore we disallow
  // individual operations that work on more than 256GB at a time.
  // |in_len_64| is needed because, on 32-bit platforms, size_t is only
  // 32-bits and this produces a warning because it's always false.
  // Casting to uint64_t inside the conditional is not sufficient to stop
  // the warning.
  const uint64_t in_len_64 = in_len;
  if (in_len_64 >= (UINT64_C(1) << 32) * 64 - 64) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (max_out_tag_len < tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }

  // The the extra input is given, it is expected to be very short and so is
  // encrypted byte-by-byte first.
  if (extra_in_len) {
    static const size_t kChaChaBlockSize = 64;
    uint32_t block_counter = (uint32_t)(1 + (in_len / kChaChaBlockSize));
    size_t offset = in_len % kChaChaBlockSize;
    uint8_t block[64 /* kChaChaBlockSize */];

    for (size_t done = 0; done < extra_in_len; block_counter++) {
      memset(block, 0, sizeof(block));
      CRYPTO_chacha_20(block, block, sizeof(block), key, nonce, block_counter);
      for (size_t i = offset; i < sizeof(block) && done < extra_in_len;
           i++, done++) {
        out_tag[done] = extra_in[done] ^ block[i];
      }
      offset = 0;
    }
  }

  union chacha20_poly1305_seal_data data;
  if (chacha20_poly1305_asm_capable()) {
    OPENSSL_memcpy(data.in.key, key, CHACHA_KEY_LEN);
    data.in.counter = 0;
    OPENSSL_memcpy(data.in.nonce, nonce, CHACHA_IV_LEN);
    data.in.extra_ciphertext = out_tag;
    data.in.extra_ciphertext_len = extra_in_len;
    chacha20_poly1305_seal(out, in, in_len, ad, ad_len, &data);
  } else {
    CRYPTO_chacha_20(out, in, in_len, key, nonce, 1);
    calc_tag(data.out.tag, key, nonce, ad, ad_len, out, in_len, out_tag,
             extra_in_len);
  }

  OPENSSL_memcpy(out_tag + extra_in_len, data.out.tag, tag_len);
  *out_tag_len = extra_in_len + tag_len;
  return 1;
}

static int aead_chacha20_poly1305_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  const AEAD_CHACHA_POLY_CTX *c20_ctx =
      (AEAD_CHACHA_POLY_CTX *)&ctx->state;

  return chacha20_poly1305_seal_scatter(
      c20_ctx->key, out, out_tag, out_tag_len, max_out_tag_len, nonce,
      nonce_len, in, in_len, extra_in, extra_in_len, ad, ad_len, ctx->tag_len);
}

static int aead_xchacha20_poly1305_seal_scatter(
    const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
    size_t *out_tag_len, size_t max_out_tag_len, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *extra_in,
    size_t extra_in_len, const uint8_t *ad, size_t ad_len) {
  const AEAD_CHACHA_POLY_CTX *c20_ctx =
      (AEAD_CHACHA_POLY_CTX *)&ctx->state;

  if (nonce_len != 24) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  alignas(4) uint8_t derived_key[CHACHA_KEY_LEN];
  alignas(4) uint8_t derived_nonce[CHACHA_IV_LEN];
  CRYPTO_hchacha20(derived_key, c20_ctx->key, nonce);
  OPENSSL_memset(derived_nonce, 0, 4);
  OPENSSL_memcpy(&derived_nonce[4], &nonce[16], 8);

  return chacha20_poly1305_seal_scatter(
      derived_key, out, out_tag, out_tag_len, max_out_tag_len, derived_nonce,
      sizeof(derived_nonce), in, in_len, extra_in, extra_in_len, ad, ad_len,
      ctx->tag_len);
}

static int chacha20_poly1305_open_gather(const uint8_t *key, uint8_t *out,
                                         const uint8_t *nonce, size_t nonce_len,
                                         const uint8_t *in, size_t in_len,
                                         const uint8_t *in_tag,
                                         size_t in_tag_len, const uint8_t *ad,
                                         size_t ad_len, size_t tag_len) {
  if (nonce_len != CHACHA_IV_LEN) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  if (in_tag_len != tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  // |CRYPTO_chacha_20| uses a 32-bit block counter. Therefore we disallow
  // individual operations that work on more than 256GB at a time.
  // |in_len_64| is needed because, on 32-bit platforms, size_t is only
  // 32-bits and this produces a warning because it's always false.
  // Casting to uint64_t inside the conditional is not sufficient to stop
  // the warning.
  const uint64_t in_len_64 = in_len;
  if (in_len_64 >= (UINT64_C(1) << 32) * 64 - 64) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  union chacha20_poly1305_open_data data;
  if (chacha20_poly1305_asm_capable()) {
    OPENSSL_memcpy(data.in.key, key, CHACHA_KEY_LEN);
    data.in.counter = 0;
    OPENSSL_memcpy(data.in.nonce, nonce, CHACHA_IV_LEN);
    chacha20_poly1305_open(out, in, in_len, ad, ad_len, &data);
  } else {
    calc_tag(data.out.tag, key, nonce, ad, ad_len, in, in_len, NULL, 0);
    CRYPTO_chacha_20(out, in, in_len, key, nonce, 1);
  }

  if (CRYPTO_memcmp(data.out.tag, in_tag, tag_len) != 0) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  return 1;
}

static int aead_chacha20_poly1305_open_gather(
    const EVP_AEAD_CTX *ctx, uint8_t *out, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *in_tag,
    size_t in_tag_len, const uint8_t *ad, size_t ad_len) {
  const AEAD_CHACHA_POLY_CTX *c20_ctx =
      (AEAD_CHACHA_POLY_CTX *)&ctx->state;

  return chacha20_poly1305_open_gather(c20_ctx->key, out, nonce, nonce_len, in,
                                       in_len, in_tag, in_tag_len, ad, ad_len,
                                       ctx->tag_len);
}

static int aead_xchacha20_poly1305_open_gather(
    const EVP_AEAD_CTX *ctx, uint8_t *out, const uint8_t *nonce,
    size_t nonce_len, const uint8_t *in, size_t in_len, const uint8_t *in_tag,
    size_t in_tag_len, const uint8_t *ad, size_t ad_len) {
  const AEAD_CHACHA_POLY_CTX *c20_ctx =
      (AEAD_CHACHA_POLY_CTX *)&ctx->state;

  if (nonce_len != 24) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE);
    return 0;
  }

  alignas(4) uint8_t derived_key[CHACHA_KEY_LEN];
  alignas(4) uint8_t derived_nonce[CHACHA_IV_LEN];
  CRYPTO_hchacha20(derived_key, c20_ctx->key, nonce);
  OPENSSL_memset(derived_nonce, 0, 4);
  OPENSSL_memcpy(&derived_nonce[4], &nonce[16], 8);

  return chacha20_poly1305_open_gather(
      derived_key, out, derived_nonce, sizeof(derived_nonce), in, in_len,
      in_tag, in_tag_len, ad, ad_len, ctx->tag_len);
}

static const EVP_AEAD aead_chacha20_poly1305 = {
    CHACHA_KEY_LEN,             // key len
    CHACHA_IV_LEN,              // nonce len
    POLY1305_TAG_LEN,           // overhead
    POLY1305_TAG_LEN,           // max tag length
    AEAD_CHACHA20_POLY1305_ID,  // evp_aead_id
    1,                          // seal_scatter_supports_extra_in

    aead_chacha20_poly1305_init,
    NULL,  // init_with_direction
    aead_chacha20_poly1305_cleanup,
    NULL /* open */,
    aead_chacha20_poly1305_seal_scatter,
    aead_chacha20_poly1305_open_gather,
    NULL,  // get_iv
    NULL,  // tag_len
    NULL,  // serialize_state
    NULL,  // deserialize_state
};

static const EVP_AEAD aead_xchacha20_poly1305 = {
    CHACHA_KEY_LEN,              // key len
    24,                          // nonce len
    POLY1305_TAG_LEN,            // overhead
    POLY1305_TAG_LEN,            // max tag length
    AEAD_XCHACHA20_POLY1305_ID,  // evp_aead_id
    1,                           // seal_scatter_supports_extra_in

    aead_chacha20_poly1305_init,
    NULL,  // init_with_direction
    aead_chacha20_poly1305_cleanup,
    NULL /* open */,
    aead_xchacha20_poly1305_seal_scatter,
    aead_xchacha20_poly1305_open_gather,
    NULL,  // get_iv
    NULL,  // tag_len
    NULL,  // serialize_state
    NULL,  // deserialize_state
};

const EVP_AEAD *EVP_aead_chacha20_poly1305(void) {
  return &aead_chacha20_poly1305;
}

const EVP_AEAD *EVP_aead_xchacha20_poly1305(void) {
  return &aead_xchacha20_poly1305;
}

static int cipher_chacha20_poly1305_init_key(CIPHER_CHACHA_POLY_CTX *ctx,
                                 const uint8_t user_key[CHACHA_KEY_LEN],
                                 const uint8_t counter_nonce[CHACHA_CTR_IV_LEN])
{
  CIPHER_CHACHA_KEY *key = CC_KEY(ctx);
  uint32_t i;
  if (user_key) {
    for (i = 0; i < (CHACHA_KEY_LEN / 4); i++) {
      key->key[i] = CRYPTO_load_u32_le(user_key + (i * 4));
    }
  }
  if (counter_nonce) {
    for (i = 0; i < CHACHA_CTR_IV_LEN / 4; i++) {
      key->counter_nonce[i] = CRYPTO_load_u32_le(counter_nonce + (i * 4));
    }
  }
  key->partial_len = 0;
  return 1;
}

static int cipher_chacha20_poly1305_init(EVP_CIPHER_CTX *ctx,
                                         const uint8_t *key,
                                         const uint8_t *iv, int32_t enc) {
  CIPHER_CHACHA_POLY_CTX *cipher_ctx = CCP_CTX(ctx);
  cipher_ctx->len.aad = 0;
  cipher_ctx->len.text = 0;
  cipher_ctx->pad_aad = 0;
  cipher_ctx->poly_initialized = 0;
  if (!key && !iv) {
    return 1;
  }
  // Init can be called multiple times before starting the cipher to
  // independently initialize any combination of Key/IV/NULL.
  if (iv != NULL) {
    // Start the counter at 0 and copy over the nonce(iv)
    uint8_t counter_nonce[CHACHA_CTR_IV_LEN] = {0};
    OPENSSL_memcpy(counter_nonce + CHACHA_CTR_IV_LEN - CHACHA_IV_LEN, iv,
           CHACHA_IV_LEN);
    cipher_chacha20_poly1305_init_key(cipher_ctx, key, counter_nonce);
    // Nonce occupies the last 3 indices of the array
    cipher_ctx->iv[0] = cipher_ctx->key.counter_nonce[1];
    cipher_ctx->iv[1] = cipher_ctx->key.counter_nonce[2];
    cipher_ctx->iv[2] = cipher_ctx->key.counter_nonce[3];
  } else {
    cipher_chacha20_poly1305_init_key(cipher_ctx, key, NULL);
  }
  return 1;
}

static int cipher_chacha20_do_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                                     const uint8_t *in, size_t in_len)
{
  CIPHER_CHACHA_POLY_CTX  *cipher_ctx = CCP_CTX(ctx);
  CIPHER_CHACHA_KEY *key = CC_KEY(cipher_ctx);
  uint32_t n, rem, counter;

  // Complete any partial block
  n = key->partial_len;
  if (n) {
    // Compute the cipher using our partially used key and any new input up
    // to the next block
    while (in_len && n < CHACHA_BLOCK_LEN) {
      // Compute 1-byte of output by xor'ing 1-byte of input with the
      // corresponding key byte and increment it all for the next byte.
      *out++ = *in++ ^ key->buf[n++];
      in_len--;
    }
    key->partial_len = n;

    // If we consumed all the input, we're done
    if (in_len == 0) {
      return 1;
    }

    // If we completed a block, increment the counter
    if (n == CHACHA_BLOCK_LEN) {
      key->partial_len = 0;
      // If this overflows we let the cipher wrap. This would be a bug in the
      // calling code as overflow behavior is not defined in RFC 8439.
      key->counter_nonce[0]++;
    }
  }

#ifdef OPENSSL_BIG_ENDIAN
  // |CRYPTO_chacha_20| expects the input as a little-endian byte array.
  uint8_t chacha_key[CHACHA_KEY_LEN];
  uint8_t nonce[CHACHA_IV_LEN];
  for(size_t i = 0; i < CHACHA_KEY_LEN / 4; i++) {
    CRYPTO_store_u32_le(chacha_key + (i * sizeof(uint32_t)),
                        cipher_ctx->key.key[i]);
  }
  for(size_t i = 0; i < CHACHA_IV_LEN / 4; i++) {
    CRYPTO_store_u32_le(nonce + (i * sizeof(uint32_t)),
                        cipher_ctx->iv[i]);
  }
#else
  const uint8_t *chacha_key = (const uint8_t *) cipher_ctx->key.key;
  const uint8_t *nonce = (const uint8_t *) cipher_ctx->iv;
#endif

  // Truncate down to the last complete block prior to the bulk cipher
  rem = (uint32_t)(in_len % CHACHA_BLOCK_LEN);
  in_len -= rem;
  counter = key->counter_nonce[0];
  while (in_len >= CHACHA_BLOCK_LEN) {
    size_t blocks = in_len / CHACHA_BLOCK_LEN;
    // 1<<28 is just a not-so-small yet not-so-large number... Below
    // condition is practically never met, but it has to be checked for code
    // correctness.
    if (sizeof(size_t) > sizeof(uint32_t) && blocks > (1U<<28)) {
      blocks = (1U << 28);
    }

    // As ChaCha20_ctr32 operates on 32-bit counter, caller has to handle
    // overflow. 'if' below detects the overflow, which is then handled by
    // limiting the amount of blocks to the exact overflow point. This while
    // loop then continues the cipher by wrapping around with counter=0.
    counter += (uint32_t) blocks;
    if (counter < blocks) {
      blocks -= counter;
      counter = 0;
    }
    blocks *= CHACHA_BLOCK_LEN;
    CRYPTO_chacha_20(out, in, blocks, chacha_key, nonce,
                     key->counter_nonce[0]);
    in_len -= blocks;
    in += blocks;
    out += blocks;

    key->counter_nonce[0] = counter;
  }

  // Start the next block if we have any leftover input
  if (rem) {
    OPENSSL_memset(key->buf, 0, sizeof(key->buf));
    // Obtain the current key and store it in the context
    CRYPTO_chacha_20(key->buf, key->buf, CHACHA_BLOCK_LEN, chacha_key, nonce,
                     key->counter_nonce[0]);
    for (n = 0; n < rem; n++) {
      out[n] = in[n] ^ key->buf[n];
    }
    key->partial_len = rem;
  }

  return 1;
}

static int cipher_chacha20_poly1305_do_cipher(
    EVP_CIPHER_CTX *ctx, unsigned char *out, const unsigned char *in,
    size_t in_len) {
  CIPHER_CHACHA_POLY_CTX *cipher_ctx = CCP_CTX(ctx);
  poly1305_state *poly_ctx = POLY_CTX(cipher_ctx);
  size_t remainder;

  if (!cipher_ctx->poly_initialized) {
#ifdef OPENSSL_BIG_ENDIAN
    // |CRYPTO_chacha_20| expects the input as a little-endian byte array.
    uint8_t chacha_key[CHACHA_KEY_LEN];
    uint8_t nonce[CHACHA_IV_LEN];
    for(int i = 0; i < CHACHA_KEY_LEN / 4; i++) {
      CRYPTO_store_u32_le(chacha_key + (i * sizeof(uint32_t)),
                          cipher_ctx->key.key[i]);
    }
    for(size_t i = 0; i < CHACHA_IV_LEN / 4; i++) {
      CRYPTO_store_u32_le(nonce + (i * sizeof(uint32_t)),
                          cipher_ctx->iv[i]);
    }
#else
    const uint8_t *chacha_key = (const uint8_t *) cipher_ctx->key.key;
    const uint8_t *nonce = (const uint8_t *) cipher_ctx->iv;
#endif
    // Obtain the poly1305 key by computing the 0th chacha20 key
    alignas(16) uint8_t poly1305_key[CHACHA_KEY_LEN];
    OPENSSL_memset(poly1305_key, 0, sizeof(poly1305_key));
    CRYPTO_chacha_20(poly1305_key, poly1305_key, sizeof(poly1305_key),
                     chacha_key, nonce, 0);

    // Initialize the poly1305 context
    CRYPTO_poly1305_init(poly_ctx, poly1305_key);
    cipher_ctx->key.counter_nonce[0] = 1;
    cipher_ctx->key.partial_len = 0;
    cipher_ctx->len.aad = 0;
    cipher_ctx->len.text = 0;
    cipher_ctx->poly_initialized = 1;
  }

  // Handle an |EVP_CipherUpdate|
  if (in) {
    if (out == NULL) {
      // NULL |out| signals an AAD update
      CRYPTO_poly1305_update(poly_ctx, in, in_len);
      cipher_ctx->len.aad += in_len;
      cipher_ctx->pad_aad = 1;
      return (int32_t) in_len;
    } else {
      // Finish AAD by applying padding
      if (cipher_ctx->pad_aad) {
        remainder = cipher_ctx->len.aad % POLY1305_TAG_LEN;
        if (remainder != 0) {
          static const uint8_t padding[POLY1305_TAG_LEN] = {0};
          CRYPTO_poly1305_update(poly_ctx, padding, sizeof(padding) - remainder);
        }
        cipher_ctx->pad_aad = 0;
      }
      // cipher/plain text |EVP_CipherUpdate|
      if (EVP_CIPHER_CTX_encrypting(ctx)) {
        // Encryption
        cipher_chacha20_do_cipher(ctx, out, in, in_len);
        // Update poly1305 with computed ciphertext
        CRYPTO_poly1305_update(poly_ctx, out, in_len);
        cipher_ctx->len.text += in_len;
      } else {
        // Decryption
        // Update poly1305 with incoming ciphertext
        CRYPTO_poly1305_update(poly_ctx, in, in_len);
        cipher_chacha20_do_cipher(ctx, out, in, in_len);
        cipher_ctx->len.text += in_len;
      }
    }
  }

  // Process an |EVP_CipherFinal|
  if (in == NULL) {
    uint8_t temp[POLY1305_TAG_LEN];
    static const uint8_t padding[POLY1305_TAG_LEN] = {0};

    // Finish AAD inp case there were no intermediate Update() calls
    if (cipher_ctx->pad_aad) {
      remainder = cipher_ctx->len.aad % POLY1305_TAG_LEN;
      if (remainder != 0) {
        CRYPTO_poly1305_update(poly_ctx, padding, sizeof(padding) - remainder);
      }
      cipher_ctx->pad_aad = 0;
    }

    // Apply padding for the text
    remainder = cipher_ctx->len.text % POLY1305_TAG_LEN;
    if (remainder != 0) {
      CRYPTO_poly1305_update(poly_ctx, padding, sizeof(padding) - remainder);
    }

    // ChaCha20-Poly1305 passes the AAD and CT lengths through Poly1305 as two
    // 64-bit little-endian integers.
#ifdef OPENSSL_BIG_ENDIAN
    uint8_t length_bytes[2 * sizeof(uint64_t)];
    CRYPTO_store_u64_le(length_bytes, cipher_ctx->len.aad);
    CRYPTO_store_u64_le(length_bytes + sizeof(uint64_t), cipher_ctx->len.text);
#else
    // For a little-endian platform, the struct's layout in memory works as-is.
    const uint8_t *length_bytes = (const uint8_t *) &cipher_ctx->len;
#endif
    CRYPTO_poly1305_update(poly_ctx, length_bytes, 2 * sizeof(uint64_t));

    // Compute the tag and write it to scratch or the cipher context
    CRYPTO_poly1305_finish(poly_ctx, EVP_CIPHER_CTX_encrypting(ctx) ?
      cipher_ctx->tag : temp);
    cipher_ctx->poly_initialized = 0;

    // Check the tags if we're decrypting
    if (!EVP_CIPHER_CTX_encrypting(ctx)) {
      if (CRYPTO_memcmp(temp, cipher_ctx->tag, cipher_ctx->tag_len)) {
        return -1;
      }
    }
  }
  return (int32_t) in_len;
}

static void cipher_chacha20_poly1305_cleanup(EVP_CIPHER_CTX *ctx) {
  if (ctx->cipher_data) {
    OPENSSL_cleanse(ctx->cipher_data, sizeof(CIPHER_CHACHA_POLY_CTX));
  }
}

static int32_t cipher_chacha20_poly1305_ctrl(EVP_CIPHER_CTX *ctx, int32_t type,
     int32_t arg, void *ptr) {
  CIPHER_CHACHA_POLY_CTX *cipher_ctx = CCP_CTX(ctx);
  switch (type) {
    case EVP_CTRL_INIT:
      if (cipher_ctx == NULL) {
        cipher_ctx = ctx->cipher_data = OPENSSL_zalloc(ctx->cipher->ctx_size);
        if (cipher_ctx == NULL) {
          OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INITIALIZATION_ERROR);
          return 0;
        }
      } else {
        cipher_ctx->len.aad = 0;
        cipher_ctx->len.text = 0;
        cipher_ctx->pad_aad = 0;
        cipher_ctx->poly_initialized = 0;
        cipher_ctx->tag_len = 0;
      }

      return 1;
    case EVP_CTRL_COPY:
      if (cipher_ctx && cipher_ctx->poly_initialized) {
        // The poly1305 context needs to be aligned on a 64-byte boundary.
        // The destination context doesn't necessarily have the same
        // alignment so we have to fix that here.
        EVP_CIPHER_CTX *dst = (EVP_CIPHER_CTX *) ptr;
        void *source_base = align_pointer((void *) POLY_CTX(CCP_CTX(ctx)), 64);
        void *dest_base = align_pointer((void *) POLY_CTX(CCP_CTX(dst)), 64);
        // We have 63 bytes of padding for alignment, so the actual size of
        // the poly1305 context is the difference of that and the total buffer.
        size_t length = sizeof(poly1305_state) - 63;
        OPENSSL_memcpy(dest_base, source_base, length);
      }
      return 1;
    case EVP_CTRL_AEAD_SET_IVLEN:
      if (arg != CHACHA_IV_LEN) {
        return 0;
      }
      return 1;
    case EVP_CTRL_AEAD_GET_TAG:
      if (arg <= 0 || arg > POLY1305_TAG_LEN ||
              !EVP_CIPHER_CTX_encrypting(ctx)) {
        return 0;
      }
      OPENSSL_memcpy(ptr, cipher_ctx->tag, arg);
      return 1;
    case EVP_CTRL_AEAD_SET_TAG:
      if (arg <= 0 || arg > POLY1305_TAG_LEN ||
              EVP_CIPHER_CTX_encrypting(ctx)) {
        return 0;
      }
      if (ptr != NULL) {
        OPENSSL_memcpy(cipher_ctx->tag, ptr, arg);
        cipher_ctx->tag_len = arg;
      }
      return 1;
    default:
      return -1;
  }
}

static EVP_CIPHER cipher_chacha20_poly1305 = {
  NID_chacha20_poly1305,
  1, // stream cipher
  CHACHA_KEY_LEN,
  CHACHA_IV_LEN,
  sizeof(CIPHER_CHACHA_POLY_CTX),
  EVP_CIPH_FLAG_AEAD_CIPHER | EVP_CIPH_CUSTOM_IV | EVP_CIPH_ALWAYS_CALL_INIT |
  EVP_CIPH_CTRL_INIT | EVP_CIPH_CUSTOM_COPY | EVP_CIPH_FLAG_CUSTOM_CIPHER,
  cipher_chacha20_poly1305_init,
  cipher_chacha20_poly1305_do_cipher,
  cipher_chacha20_poly1305_cleanup,
  cipher_chacha20_poly1305_ctrl
};

const EVP_CIPHER *EVP_chacha20_poly1305(void)
{
  return(&cipher_chacha20_poly1305);
}
