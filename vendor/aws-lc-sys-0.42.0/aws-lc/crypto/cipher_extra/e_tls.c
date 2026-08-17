// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/aead.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/hmac.h>
#include <openssl/md5.h>
#include <openssl/mem.h>
#include <openssl/sha.h>
#include <openssl/type_check.h>

#include "../fipsmodule/cipher/internal.h"
#include "../internal.h"
#include "internal.h"


typedef struct {
  EVP_CIPHER_CTX cipher_ctx;
  HMAC_CTX hmac_ctx;
  // mac_key is the portion of the key used for the MAC. It is retained
  // separately for the constant-time CBC code.
  uint8_t mac_key[EVP_MAX_MD_SIZE];
  uint8_t mac_key_len;
  // implicit_iv is one iff this is a pre-TLS-1.1 CBC cipher without an explicit
  // IV.
  char implicit_iv;
} AEAD_TLS_CTX;

OPENSSL_STATIC_ASSERT(EVP_MAX_MD_SIZE < 256,
                      mac_key_len_does_not_fit_in_uint8_t)

static void aead_tls_cleanup(EVP_AEAD_CTX *ctx) {
  AEAD_TLS_CTX *tls_ctx = (AEAD_TLS_CTX *)ctx->state.ptr;
  EVP_CIPHER_CTX_cleanup(&tls_ctx->cipher_ctx);
  HMAC_CTX_cleanup(&tls_ctx->hmac_ctx);
  OPENSSL_free(tls_ctx);
  ctx->state.ptr = NULL;
}

static int aead_tls_init(EVP_AEAD_CTX *ctx, const uint8_t *key, size_t key_len,
                         size_t tag_len, enum evp_aead_direction_t dir,
                         const EVP_CIPHER *cipher, const EVP_MD *md,
                         char implicit_iv) {
  if (tag_len != EVP_AEAD_DEFAULT_TAG_LENGTH && tag_len != EVP_MD_size(md)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_TAG_SIZE);
    return 0;
  }

  if (key_len != EVP_AEAD_key_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_KEY_LENGTH);
    return 0;
  }

  size_t mac_key_len = EVP_MD_size(md);
  size_t enc_key_len = EVP_CIPHER_key_length(cipher);
  assert(mac_key_len + enc_key_len +
             (implicit_iv ? EVP_CIPHER_iv_length(cipher) : 0) ==
         key_len);

  AEAD_TLS_CTX *tls_ctx = OPENSSL_malloc(sizeof(AEAD_TLS_CTX));
  if (tls_ctx == NULL) {
    return 0;
  }
  ctx->state.ptr = (void *)tls_ctx;

  EVP_CIPHER_CTX_init(&tls_ctx->cipher_ctx);
  HMAC_CTX_init(&tls_ctx->hmac_ctx);
  assert(mac_key_len <= EVP_MAX_MD_SIZE);
  OPENSSL_memcpy(tls_ctx->mac_key, key, mac_key_len);
  tls_ctx->mac_key_len = (uint8_t)mac_key_len;
  tls_ctx->implicit_iv = implicit_iv;

  if (!EVP_CipherInit_ex(&tls_ctx->cipher_ctx, cipher, NULL, &key[mac_key_len],
                         implicit_iv ? &key[mac_key_len + enc_key_len] : NULL,
                         dir == evp_aead_seal) ||
      !HMAC_Init_ex(&tls_ctx->hmac_ctx, key, mac_key_len, md, NULL)) {
    aead_tls_cleanup(ctx);
    return 0;
  }
  EVP_CIPHER_CTX_set_padding(&tls_ctx->cipher_ctx, 0);

  return 1;
}

static size_t aead_tls_tag_len(const EVP_AEAD_CTX *ctx, const size_t in_len,
                               const size_t extra_in_len) {
  assert(extra_in_len == 0);
  const AEAD_TLS_CTX *tls_ctx = (AEAD_TLS_CTX *)ctx->state.ptr;

  const size_t hmac_len = HMAC_size(&tls_ctx->hmac_ctx);
  if (EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) != EVP_CIPH_CBC_MODE) {
    // The NULL cipher.
    return hmac_len;
  }

  const size_t block_size = EVP_CIPHER_CTX_block_size(&tls_ctx->cipher_ctx);
  // An overflow of |in_len + hmac_len| doesn't affect the result mod
  // |block_size|, provided that |block_size| is a smaller power of two.
  assert(block_size != 0 && (block_size & (block_size - 1)) == 0);
  const size_t pad_len = block_size - (in_len + hmac_len) % block_size;
  return hmac_len + pad_len;
}

static int aead_tls_seal_scatter(const EVP_AEAD_CTX *ctx, uint8_t *out,
                                 uint8_t *out_tag, size_t *out_tag_len,
                                 const size_t max_out_tag_len,
                                 const uint8_t *nonce, const size_t nonce_len,
                                 const uint8_t *in, const size_t in_len,
                                 const uint8_t *extra_in,
                                 const size_t extra_in_len, const uint8_t *ad,
                                 const size_t ad_len) {
  AEAD_TLS_CTX *tls_ctx = (AEAD_TLS_CTX *)ctx->state.ptr;

  if (!tls_ctx->cipher_ctx.encrypt) {
    // Unlike a normal AEAD, a TLS AEAD may only be used in one direction.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_OPERATION);
    return 0;
  }

  if (in_len > INT_MAX) {
    // EVP_CIPHER takes int as input.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  if (max_out_tag_len < aead_tls_tag_len(ctx, in_len, extra_in_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (nonce_len != EVP_AEAD_nonce_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (ad_len != 13 - 2 /* length bytes */) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_AD_SIZE);
    return 0;
  }

  // To allow for CBC mode which changes cipher length, |ad| doesn't include the
  // length for legacy ciphers.
  uint8_t ad_extra[2];
  ad_extra[0] = (uint8_t)(in_len >> 8);
  ad_extra[1] = (uint8_t)(in_len & 0xff);

  // Compute the MAC. This must be first in case the operation is being done
  // in-place.
  uint8_t mac[EVP_MAX_MD_SIZE];
  unsigned mac_len;
  if (!HMAC_Init_ex(&tls_ctx->hmac_ctx, NULL, 0, NULL, NULL) ||
      !HMAC_Update(&tls_ctx->hmac_ctx, ad, ad_len) ||
      !HMAC_Update(&tls_ctx->hmac_ctx, ad_extra, sizeof(ad_extra)) ||
      !HMAC_Update(&tls_ctx->hmac_ctx, in, in_len) ||
      !HMAC_Final(&tls_ctx->hmac_ctx, mac, &mac_len)) {
    return 0;
  }

  // Configure the explicit IV.
  if (EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) == EVP_CIPH_CBC_MODE &&
      !tls_ctx->implicit_iv &&
      !EVP_EncryptInit_ex(&tls_ctx->cipher_ctx, NULL, NULL, NULL, nonce)) {
    return 0;
  }

  // Encrypt the input.
  int len;
  if (!EVP_EncryptUpdate(&tls_ctx->cipher_ctx, out, &len, in, (int)in_len)) {
    return 0;
  }

  unsigned block_size = EVP_CIPHER_CTX_block_size(&tls_ctx->cipher_ctx);

  // Feed the MAC into the cipher in two steps. First complete the final partial
  // block from encrypting the input and split the result between |out| and
  // |out_tag|. Then feed the rest.

  const size_t early_mac_len =
      (block_size - (in_len % block_size)) % block_size;
  if (early_mac_len != 0) {
    assert(len + block_size - early_mac_len == in_len);
    uint8_t buf[EVP_MAX_BLOCK_LENGTH];
    int buf_len;
    if (!EVP_EncryptUpdate(&tls_ctx->cipher_ctx, buf, &buf_len, mac,
                           (int)early_mac_len)) {
      return 0;
    }
    assert(buf_len == (int)block_size);
    OPENSSL_memcpy(out + len, buf, block_size - early_mac_len);
    OPENSSL_memcpy(out_tag, buf + block_size - early_mac_len, early_mac_len);
  }
  size_t tag_len = early_mac_len;

  if (!EVP_EncryptUpdate(&tls_ctx->cipher_ctx, out_tag + tag_len, &len,
                         mac + tag_len, mac_len - tag_len)) {
    return 0;
  }
  tag_len += len;

  if (block_size > 1) {
    assert(block_size <= 256);
    assert(EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) == EVP_CIPH_CBC_MODE);

    // Compute padding and feed that into the cipher.
    uint8_t padding[256];
    unsigned padding_len = block_size - ((in_len + mac_len) % block_size);
    OPENSSL_memset(padding, padding_len - 1, padding_len);
    if (!EVP_EncryptUpdate(&tls_ctx->cipher_ctx, out_tag + tag_len, &len,
                           padding, (int)padding_len)) {
      return 0;
    }
    tag_len += len;
  }

  if (!EVP_EncryptFinal_ex(&tls_ctx->cipher_ctx, out_tag + tag_len, &len)) {
    return 0;
  }
  assert(len == 0);  // Padding is explicit.
  assert(tag_len == aead_tls_tag_len(ctx, in_len, extra_in_len));

  *out_tag_len = tag_len;
  return 1;
}

static int aead_tls_open(const EVP_AEAD_CTX *ctx, uint8_t *out, size_t *out_len,
                         size_t max_out_len, const uint8_t *nonce,
                         size_t nonce_len, const uint8_t *in, size_t in_len,
                         const uint8_t *ad, size_t ad_len) {
  AEAD_TLS_CTX *tls_ctx = (AEAD_TLS_CTX *)ctx->state.ptr;

  if (tls_ctx->cipher_ctx.encrypt) {
    // Unlike a normal AEAD, a TLS AEAD may only be used in one direction.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_OPERATION);
    return 0;
  }

  if (in_len < HMAC_size(&tls_ctx->hmac_ctx)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  if (max_out_len < in_len) {
    // This requires that the caller provide space for the MAC, even though it
    // will always be removed on return.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (nonce_len != EVP_AEAD_nonce_length(ctx->aead)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_NONCE_SIZE);
    return 0;
  }

  if (ad_len != 13 - 2 /* length bytes */) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_AD_SIZE);
    return 0;
  }

  if (in_len > INT_MAX) {
    // EVP_CIPHER takes int as input.
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    return 0;
  }

  // Configure the explicit IV.
  if (EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) == EVP_CIPH_CBC_MODE &&
      !tls_ctx->implicit_iv &&
      !EVP_DecryptInit_ex(&tls_ctx->cipher_ctx, NULL, NULL, NULL, nonce)) {
    return 0;
  }

  // Decrypt to get the plaintext + MAC + padding.
  size_t total = 0;
  int len;
  if (!EVP_DecryptUpdate(&tls_ctx->cipher_ctx, out, &len, in, (int)in_len)) {
    return 0;
  }
  total += len;
  if (!EVP_DecryptFinal_ex(&tls_ctx->cipher_ctx, out + total, &len)) {
    return 0;
  }
  total += len;
  assert(total == in_len);

  CONSTTIME_SECRET(out, total);

  // Remove CBC padding. Code from here on is timing-sensitive with respect to
  // |padding_ok| and |data_plus_mac_len| for CBC ciphers.
  size_t data_plus_mac_len;
  crypto_word_t padding_ok;
  if (EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) == EVP_CIPH_CBC_MODE) {
    if (!EVP_tls_cbc_remove_padding(
            &padding_ok, &data_plus_mac_len, out, total,
            EVP_CIPHER_CTX_block_size(&tls_ctx->cipher_ctx),
            HMAC_size(&tls_ctx->hmac_ctx))) {
      // Publicly invalid. This can be rejected in non-constant time.
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }
  } else {
    padding_ok = CONSTTIME_TRUE_W;
    data_plus_mac_len = total;
    // |data_plus_mac_len| = |total| = |in_len| at this point. |in_len| has
    // already been checked against the MAC size at the top of the function.
    assert(data_plus_mac_len >= HMAC_size(&tls_ctx->hmac_ctx));
  }
  size_t data_len = data_plus_mac_len - HMAC_size(&tls_ctx->hmac_ctx);

  // At this point, if the padding is valid, the first |data_plus_mac_len| bytes
  // after |out| are the plaintext and MAC. Otherwise, |data_plus_mac_len| is
  // still large enough to extract a MAC, but it will be irrelevant.

  // To allow for CBC mode which changes cipher length, |ad| doesn't include the
  // length for legacy ciphers.
  uint8_t ad_fixed[13];
  OPENSSL_memcpy(ad_fixed, ad, 11);
  ad_fixed[11] = (uint8_t)(data_len >> 8);
  ad_fixed[12] = (uint8_t)(data_len & 0xff);
  ad_len += 2;

  // Compute the MAC and extract the one in the record.
  uint8_t mac[EVP_MAX_MD_SIZE];
  size_t mac_len;
  uint8_t record_mac_tmp[EVP_MAX_MD_SIZE];
  uint8_t *record_mac;
  if (EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) == EVP_CIPH_CBC_MODE &&
      EVP_tls_cbc_record_digest_supported(tls_ctx->hmac_ctx.md)) {
    if (!EVP_tls_cbc_digest_record(tls_ctx->hmac_ctx.md, mac, &mac_len,
                                   ad_fixed, out, data_len, total,
                                   tls_ctx->mac_key, tls_ctx->mac_key_len)) {
      OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
      return 0;
    }
    assert(mac_len == HMAC_size(&tls_ctx->hmac_ctx));

    record_mac = record_mac_tmp;
    EVP_tls_cbc_copy_mac(record_mac, mac_len, out, data_plus_mac_len, total);
  } else {
    // We should support the constant-time path for all CBC-mode ciphers
    // implemented.
    assert(EVP_CIPHER_CTX_mode(&tls_ctx->cipher_ctx) != EVP_CIPH_CBC_MODE);

    unsigned mac_len_u;
    if (!HMAC_Init_ex(&tls_ctx->hmac_ctx, NULL, 0, NULL, NULL) ||
        !HMAC_Update(&tls_ctx->hmac_ctx, ad_fixed, ad_len) ||
        !HMAC_Update(&tls_ctx->hmac_ctx, out, data_len) ||
        !HMAC_Final(&tls_ctx->hmac_ctx, mac, &mac_len_u)) {
      return 0;
    }
    mac_len = mac_len_u;

    assert(mac_len == HMAC_size(&tls_ctx->hmac_ctx));
    record_mac = &out[data_len];
  }

  // Perform the MAC check and the padding check in constant-time. It should be
  // safe to simply perform the padding check first, but it would not be under a
  // different choice of MAC location on padding failure. See
  // EVP_tls_cbc_remove_padding.
  crypto_word_t good =
      constant_time_eq_int(CRYPTO_memcmp(record_mac, mac, mac_len), 0);
  good &= padding_ok;
  CONSTTIME_DECLASSIFY(&good, sizeof(good));
  if (!good) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    return 0;
  }

  CONSTTIME_DECLASSIFY(&data_len, sizeof(data_len));
  CONSTTIME_DECLASSIFY(out, data_len);

  // End of timing-sensitive code.

  *out_len = data_len;
  return 1;
}

static int aead_aes_128_cbc_sha1_tls_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                          size_t key_len, size_t tag_len,
                                          enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_128_cbc(),
                       EVP_sha1(), 0);
}

static int aead_aes_128_cbc_sha1_tls_implicit_iv_init(
    EVP_AEAD_CTX *ctx, const uint8_t *key, size_t key_len, size_t tag_len,
    enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_128_cbc(),
                       EVP_sha1(), 1);
}

static int aead_aes_256_cbc_sha1_tls_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                          size_t key_len, size_t tag_len,
                                          enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_256_cbc(),
                       EVP_sha1(), 0);
}

static int aead_aes_256_cbc_sha1_tls_implicit_iv_init(
    EVP_AEAD_CTX *ctx, const uint8_t *key, size_t key_len, size_t tag_len,
    enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_256_cbc(),
                       EVP_sha1(), 1);
}

static int aead_aes_128_cbc_sha256_tls_init(EVP_AEAD_CTX *ctx,
                                            const uint8_t *key, size_t key_len,
                                            size_t tag_len,
                                            enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_128_cbc(),
                       EVP_sha256(), 0);
}

static int aead_aes_128_cbc_sha256_tls_implicit_iv_init(
    EVP_AEAD_CTX *ctx, const uint8_t *key, size_t key_len, size_t tag_len,
    enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_128_cbc(),
                       EVP_sha256(), 1);
}

static int aead_aes_256_cbc_sha384_tls_init(EVP_AEAD_CTX *ctx,
                                            const uint8_t *key, size_t key_len,
                                            size_t tag_len,
                                            enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_aes_256_cbc(),
                       EVP_sha384(), 0);
}

static int aead_des_ede3_cbc_sha1_tls_init(EVP_AEAD_CTX *ctx,
                                           const uint8_t *key, size_t key_len,
                                           size_t tag_len,
                                           enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_des_ede3_cbc(),
                       EVP_sha1(), 0);
}

static int aead_des_ede3_cbc_sha1_tls_implicit_iv_init(
    EVP_AEAD_CTX *ctx, const uint8_t *key, size_t key_len, size_t tag_len,
    enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_des_ede3_cbc(),
                       EVP_sha1(), 1);
}

static int aead_tls_get_iv(const EVP_AEAD_CTX *ctx, const uint8_t **out_iv,
                           size_t *out_iv_len) {
  const AEAD_TLS_CTX *tls_ctx = (AEAD_TLS_CTX *)ctx->state.ptr;
  const size_t iv_len = EVP_CIPHER_CTX_iv_length(&tls_ctx->cipher_ctx);
  if (iv_len <= 1) {
    return 0;
  }

  *out_iv = tls_ctx->cipher_ctx.iv;
  *out_iv_len = iv_len;
  return 1;
}

static int aead_null_sha1_tls_init(EVP_AEAD_CTX *ctx, const uint8_t *key,
                                   size_t key_len, size_t tag_len,
                                   enum evp_aead_direction_t dir) {
  return aead_tls_init(ctx, key, key_len, tag_len, dir, EVP_enc_null(),
                       EVP_sha1(), 1 /* implicit iv */);
}

static const EVP_AEAD aead_aes_128_cbc_sha1_tls = {
    SHA_DIGEST_LENGTH + 16,        // key len (SHA1 + AES128)
    16,                            // nonce len (IV)
    16 + SHA_DIGEST_LENGTH,        // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,             // max tag length
    AEAD_AES_128_CBC_SHA1_TLS_ID,  // evp_aead_id
    0,                             // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_128_cbc_sha1_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_128_cbc_sha1_tls_implicit_iv = {
    SHA_DIGEST_LENGTH + 16 + 16,               // key len (SHA1 + AES128 + IV)
    0,                                         // nonce len
    16 + SHA_DIGEST_LENGTH,                    // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,                         // max tag length
    AEAD_AES_128_CBC_SHA1_TLS_IMPLICIT_IV_ID,  // evp_aead_id
    0,                                         // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_128_cbc_sha1_tls_implicit_iv_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,             // open_gather
    aead_tls_get_iv,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_256_cbc_sha1_tls = {
    SHA_DIGEST_LENGTH + 32,        // key len (SHA1 + AES256)
    16,                            // nonce len (IV)
    16 + SHA_DIGEST_LENGTH,        // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,             // max tag length
    AEAD_AES_256_CBC_SHA1_TLS_ID,  // evp_aead_id
    0,                             // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_256_cbc_sha1_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_256_cbc_sha1_tls_implicit_iv = {
    SHA_DIGEST_LENGTH + 32 + 16,               // key len (SHA1 + AES256 + IV)
    0,                                         // nonce len
    16 + SHA_DIGEST_LENGTH,                    // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,                         // max tag length
    AEAD_AES_256_CBC_SHA1_TLS_IMPLICIT_IV_ID,  // evp_aead_id
    0,                                         // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_256_cbc_sha1_tls_implicit_iv_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,             // open_gather
    aead_tls_get_iv,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_128_cbc_sha256_tls = {
    SHA256_DIGEST_LENGTH + 16,       // key len (SHA256 + AES128)
    16,                              // nonce len (IV)
    16 + SHA256_DIGEST_LENGTH,       // overhead (padding + SHA256)
    SHA256_DIGEST_LENGTH,            // max tag length
    AEAD_AES_128_CBC_SHA256_TLS_ID,  // evp_aead_id
    0,                               // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_128_cbc_sha256_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_128_cbc_sha256_tls_implicit_iv = {
    SHA256_DIGEST_LENGTH + 16 + 16,  // key len (SHA256 + AES128 + IV)
    0,                               // nonce len
    16 + SHA256_DIGEST_LENGTH,       // overhead (padding + SHA256)
    SHA256_DIGEST_LENGTH,            // max tag length
    AEAD_AES_128_CBC_SHA256_TLS_IMPLICIT_IV_ID,  // evp_aead_id
    0,  // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_128_cbc_sha256_tls_implicit_iv_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,             // open_gather
    aead_tls_get_iv,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_aes_256_cbc_sha384_tls = {
    SHA384_DIGEST_LENGTH + 32,       // key len (SHA384 + AES256)
    16,                              // nonce len (IV)
    16 + SHA384_DIGEST_LENGTH,       // overhead (padding + SHA384)
    SHA384_DIGEST_LENGTH,            // max tag length
    AEAD_AES_256_CBC_SHA384_TLS_ID,  // evp_aead_id
    0,                               // seal_scatter_supports_extra_in

    NULL,  // init
    aead_aes_256_cbc_sha384_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_des_ede3_cbc_sha1_tls = {
    SHA_DIGEST_LENGTH + 24,         // key len (SHA1 + 3DES)
    8,                              // nonce len (IV)
    8 + SHA_DIGEST_LENGTH,          // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,              // max tag length
    AEAD_DES_EDE3_CBC_SHA1_TLS_ID,  // evp_aead_id
    0,                              // seal_scatter_supports_extra_in

    NULL,  // init
    aead_des_ede3_cbc_sha1_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_des_ede3_cbc_sha1_tls_implicit_iv = {
    SHA_DIGEST_LENGTH + 24 + 8,                 // key len (SHA1 + 3DES + IV)
    0,                                          // nonce len
    8 + SHA_DIGEST_LENGTH,                      // overhead (padding + SHA1)
    SHA_DIGEST_LENGTH,                          // max tag length
    AEAD_DES_EDE3_CBC_SHA1_TLS_IMPLICIT_IV_ID,  // evp_aead_id
    0,  // seal_scatter_supports_extra_in

    NULL,  // init
    aead_des_ede3_cbc_sha1_tls_implicit_iv_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,             // open_gather
    aead_tls_get_iv,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

static const EVP_AEAD aead_null_sha1_tls = {
    SHA_DIGEST_LENGTH,      // key len
    0,                      // nonce len
    SHA_DIGEST_LENGTH,      // overhead (SHA1)
    SHA_DIGEST_LENGTH,      // max tag length
    AEAD_NULL_SHA1_TLS_ID,  // evp_aead_id
    0,                      // seal_scatter_supports_extra_in

    NULL,  // init
    aead_null_sha1_tls_init,
    aead_tls_cleanup,
    aead_tls_open,
    aead_tls_seal_scatter,
    NULL,  // open_gather
    NULL,  // get_iv
    aead_tls_tag_len,
    NULL /* serialize_state */,
    NULL /* deserialize_state */,
};

const EVP_AEAD *EVP_aead_aes_128_cbc_sha1_tls(void) {
  return &aead_aes_128_cbc_sha1_tls;
}

const EVP_AEAD *EVP_aead_aes_128_cbc_sha1_tls_implicit_iv(void) {
  return &aead_aes_128_cbc_sha1_tls_implicit_iv;
}

const EVP_AEAD *EVP_aead_aes_256_cbc_sha1_tls(void) {
  return &aead_aes_256_cbc_sha1_tls;
}

const EVP_AEAD *EVP_aead_aes_256_cbc_sha1_tls_implicit_iv(void) {
  return &aead_aes_256_cbc_sha1_tls_implicit_iv;
}

const EVP_AEAD *EVP_aead_aes_128_cbc_sha256_tls(void) {
  return &aead_aes_128_cbc_sha256_tls;
}

const EVP_AEAD *EVP_aead_aes_128_cbc_sha256_tls_implicit_iv(void) {
  return &aead_aes_128_cbc_sha256_tls_implicit_iv;
}

const EVP_AEAD *EVP_aead_aes_256_cbc_sha384_tls(void) {
  return &aead_aes_256_cbc_sha384_tls;
}

const EVP_AEAD *EVP_aead_des_ede3_cbc_sha1_tls(void) {
  return &aead_des_ede3_cbc_sha1_tls;
}

const EVP_AEAD *EVP_aead_des_ede3_cbc_sha1_tls_implicit_iv(void) {
  return &aead_des_ede3_cbc_sha1_tls_implicit_iv;
}

const EVP_AEAD *EVP_aead_null_sha1_tls(void) { return &aead_null_sha1_tls; }
