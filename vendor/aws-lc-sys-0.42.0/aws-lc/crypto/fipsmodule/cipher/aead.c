// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/aead.h>

#include <assert.h>
#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


size_t EVP_AEAD_key_length(const EVP_AEAD *aead) { return aead->key_len; }

size_t EVP_AEAD_nonce_length(const EVP_AEAD *aead) { return aead->nonce_len; }

size_t EVP_AEAD_max_overhead(const EVP_AEAD *aead) { return aead->overhead; }

size_t EVP_AEAD_max_tag_len(const EVP_AEAD *aead) { return aead->max_tag_len; }

void EVP_AEAD_CTX_zero(EVP_AEAD_CTX *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(EVP_AEAD_CTX));
}

EVP_AEAD_CTX *EVP_AEAD_CTX_new(const EVP_AEAD *aead, const uint8_t *key,
                               size_t key_len, size_t tag_len) {
  EVP_AEAD_CTX *ctx = OPENSSL_zalloc(sizeof(EVP_AEAD_CTX));
  if (ctx == NULL) {
    return NULL;
  }
  // NO-OP: struct already zeroed
  //EVP_AEAD_CTX_zero(ctx);

  if (EVP_AEAD_CTX_init(ctx, aead, key, key_len, tag_len, NULL)) {
    return ctx;
  }

  EVP_AEAD_CTX_free(ctx);
  return NULL;
}

void EVP_AEAD_CTX_free(EVP_AEAD_CTX *ctx) {
  if (ctx == NULL) {
    return;
  }
  EVP_AEAD_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

int EVP_AEAD_CTX_init(EVP_AEAD_CTX *ctx, const EVP_AEAD *aead,
                      const uint8_t *key, size_t key_len, size_t tag_len,
                      ENGINE *impl) {
  if (!aead->init) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_NO_DIRECTION_SET);
    ctx->aead = NULL;
    return 0;
  }
  return EVP_AEAD_CTX_init_with_direction(ctx, aead, key, key_len, tag_len,
                                          evp_aead_open);
}

int EVP_AEAD_CTX_init_with_direction(EVP_AEAD_CTX *ctx, const EVP_AEAD *aead,
                                     const uint8_t *key, size_t key_len,
                                     size_t tag_len,
                                     enum evp_aead_direction_t dir) {
  SET_DIT_AUTO_RESET;
  if (key_len != aead->key_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_UNSUPPORTED_KEY_SIZE);
    ctx->aead = NULL;
    return 0;
  }

  ctx->aead = aead;

  int ok;
  if (aead->init) {
    ok = aead->init(ctx, key, key_len, tag_len);
  } else {
    ok = aead->init_with_direction(ctx, key, key_len, tag_len, dir);
  }

  if (!ok) {
    ctx->aead = NULL;
  }

  return ok;
}

void EVP_AEAD_CTX_cleanup(EVP_AEAD_CTX *ctx) {
  if (ctx->aead == NULL) {
    return;
  }
  ctx->aead->cleanup(ctx);
  ctx->aead = NULL;
}

// check_alias returns 1 if |out| is compatible with |in| and 0 otherwise. If
// |in| and |out| alias, we require that |in| == |out|.
static int check_alias(const uint8_t *in, size_t in_len, const uint8_t *out,
                       size_t out_len) {
  if (!buffers_alias(in, in_len, out, out_len)) {
    return 1;
  }

  return in == out;
}

int EVP_AEAD_CTX_seal(const EVP_AEAD_CTX *ctx, uint8_t *out, size_t *out_len,
                      size_t max_out_len, const uint8_t *nonce,
                      size_t nonce_len, const uint8_t *in, size_t in_len,
                      const uint8_t *ad, size_t ad_len) {
  SET_DIT_AUTO_RESET;
  if (in_len + ctx->aead->overhead < in_len /* overflow */) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_TOO_LARGE);
    goto error;
  }

  if (max_out_len < in_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    goto error;
  }

  if (!check_alias(in, in_len, out, max_out_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_OUTPUT_ALIASES_INPUT);
    goto error;
  }

  size_t out_tag_len;
  if (ctx->aead->seal_scatter(ctx, out, out + in_len, &out_tag_len,
                              max_out_len - in_len, nonce, nonce_len, in,
                              in_len, NULL, 0, ad, ad_len)) {
    *out_len = in_len + out_tag_len;
    return 1;
  }

error:
  // In the event of an error, clear the output buffer so that a caller
  // that doesn't check the return value doesn't send raw data.
  OPENSSL_memset(out, 0, max_out_len);
  *out_len = 0;
  return 0;
}

int EVP_AEAD_CTX_seal_scatter(const EVP_AEAD_CTX *ctx, uint8_t *out,
                              uint8_t *out_tag, size_t *out_tag_len,
                              size_t max_out_tag_len, const uint8_t *nonce,
                              size_t nonce_len, const uint8_t *in,
                              size_t in_len, const uint8_t *extra_in,
                              size_t extra_in_len, const uint8_t *ad,
                              size_t ad_len) {
  SET_DIT_AUTO_RESET; //check that it was preserved
  // |in| and |out| may alias exactly, |out_tag| may not alias.
  if (!check_alias(in, in_len, out, in_len) ||
      buffers_alias(out, in_len, out_tag, max_out_tag_len) ||
      buffers_alias(in, in_len, out_tag, max_out_tag_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_OUTPUT_ALIASES_INPUT);
    goto error;
  }

  if (!ctx->aead->seal_scatter_supports_extra_in && extra_in_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_INVALID_OPERATION);
    goto error;
  }

  if (ctx->aead->seal_scatter(ctx, out, out_tag, out_tag_len, max_out_tag_len,
                              nonce, nonce_len, in, in_len, extra_in,
                              extra_in_len, ad, ad_len)) {
    return 1;
  }

error:
  // In the event of an error, clear the output buffer so that a caller
  // that doesn't check the return value doesn't send raw data.
  OPENSSL_memset(out, 0, in_len);
  OPENSSL_memset(out_tag, 0, max_out_tag_len);
  *out_tag_len = 0;
  return 0;
}

int EVP_AEAD_CTX_open(const EVP_AEAD_CTX *ctx, uint8_t *out, size_t *out_len,
                      size_t max_out_len, const uint8_t *nonce,
                      size_t nonce_len, const uint8_t *in, size_t in_len,
                      const uint8_t *ad, size_t ad_len) {
  SET_DIT_AUTO_RESET;
  if (!check_alias(in, in_len, out, max_out_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_OUTPUT_ALIASES_INPUT);
    goto error;
  }

  if (ctx->aead->open) {
    if (!ctx->aead->open(ctx, out, out_len, max_out_len, nonce, nonce_len, in,
                         in_len, ad, ad_len)) {
      goto error;
    }
    return 1;
  }

  // AEADs that use the default implementation of open() must set |tag_len| at
  // initialization time.
  assert(ctx->tag_len);

  if (in_len < ctx->tag_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BAD_DECRYPT);
    goto error;
  }

  size_t plaintext_len = in_len - ctx->tag_len;
  if (max_out_len < plaintext_len) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_BUFFER_TOO_SMALL);
    goto error;
  }
  if (EVP_AEAD_CTX_open_gather(ctx, out, nonce, nonce_len, in, plaintext_len,
                               in + plaintext_len, ctx->tag_len, ad, ad_len)) {
    *out_len = plaintext_len;
    return 1;
  }

error:
  // In the event of an error, clear the output buffer so that a caller
  // that doesn't check the return value doesn't try and process bad
  // data.
  OPENSSL_memset(out, 0, max_out_len);
  *out_len = 0;
  return 0;
}

int EVP_AEAD_CTX_open_gather(const EVP_AEAD_CTX *ctx, uint8_t *out,
                             const uint8_t *nonce, size_t nonce_len,
                             const uint8_t *in, size_t in_len,
                             const uint8_t *in_tag, size_t in_tag_len,
                             const uint8_t *ad, size_t ad_len) {
  SET_DIT_AUTO_RESET;
  if (!check_alias(in, in_len, out, in_len)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_OUTPUT_ALIASES_INPUT);
    goto error;
  }

  if (!ctx->aead->open_gather) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_CTRL_NOT_IMPLEMENTED);
    goto error;
  }

  if (ctx->aead->open_gather(ctx, out, nonce, nonce_len, in, in_len, in_tag,
                             in_tag_len, ad, ad_len)) {
    return 1;
  }

error:
  // In the event of an error, clear the output buffer so that a caller
  // that doesn't check the return value doesn't try and process bad
  // data.
  OPENSSL_memset(out, 0, in_len);
  return 0;
}

const EVP_AEAD *EVP_AEAD_CTX_aead(const EVP_AEAD_CTX *ctx) { return ctx->aead; }

int EVP_AEAD_CTX_get_iv(const EVP_AEAD_CTX *ctx, const uint8_t **out_iv,
                        size_t *out_len) {
  if (ctx->aead->get_iv == NULL) {
    return 0;
  }

  return ctx->aead->get_iv(ctx, out_iv, out_len);
}

int EVP_AEAD_CTX_tag_len(const EVP_AEAD_CTX *ctx, size_t *out_tag_len,
                         const size_t in_len, const size_t extra_in_len) {
  assert(ctx->aead->seal_scatter_supports_extra_in || !extra_in_len);

  if (ctx->aead->tag_len) {
    *out_tag_len = ctx->aead->tag_len(ctx, in_len, extra_in_len);
    return 1;
  }

  if (extra_in_len + ctx->tag_len < extra_in_len) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_OVERFLOW);
    *out_tag_len = 0;
    return 0;
  }
  *out_tag_len = extra_in_len + ctx->tag_len;
  return 1;
}

int EVP_AEAD_get_iv_from_ipv4_nanosecs(
    const uint32_t ipv4_address, const uint64_t nanosecs,
    uint8_t out_iv[FIPS_AES_GCM_NONCE_LENGTH]) {
  if (out_iv == NULL) {
    return 0;
  }

  CRYPTO_store_u32_le(&out_iv[0], ipv4_address);
  CRYPTO_store_u64_le(&out_iv[sizeof(ipv4_address)], nanosecs);

  return 1;
}

#define EVP_AEAD_CTX_SERDE_VERSION 1

int EVP_AEAD_CTX_serialize_state(const EVP_AEAD_CTX *ctx, CBB *cbb) {
  // EVP_AEAD_CTX must be initialized by EVP_AEAD_CTX_init first.
  if (!ctx->aead) {
    return 0;
  }

  size_t aead_id = EVP_AEAD_CTX_get_aead_id(ctx);

  // We shouldn't serialize if we don't have a proper identifier
  if (aead_id == AEAD_UNKNOWN_ID) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  CBB seq;

  if (!CBB_add_asn1(cbb, &seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&seq, EVP_AEAD_CTX_SERDE_VERSION) ||
      !CBB_add_asn1_uint64(&seq, aead_id)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  CBB state;

  // 50 here is just an initial capacity based on some estimated calculations
  // of the AES GCM state structure encoding with headroom:
  //
  // -- 2 bytes for sequence tag+length
  // AeadAesGCMTls13State ::= SEQUENCE {
  //   -- 2 bytes for tag+length and 8 bytes if a full uint64
  //   serializationVersion AeadAesGCMTls13StateSerializationVersion,
  //   -- 2 bytes for tag+length and 8 bytes if a full uint64
  //   minNextNonce   INTEGER,
  //   -- 2 bytes for tag+length and 8 bytes if a full uint64
  //   mask           INTEGER,
  //   -- 2 bytes for tag+length and 1 byte
  //   first          BOOLEAN
  // }
  if (!CBB_init(&state, 50)) {
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  if (ctx->aead->serialize_state) {
    if (!ctx->aead->serialize_state(ctx, &state)) {
      CBB_cleanup(&state);
      OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
      return 0;
    }
  }

  if (!CBB_add_asn1_octet_string(&seq, CBB_data(&state), CBB_len(&state))) {
    CBB_cleanup(&state);
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  CBB_cleanup(&state);
  return CBB_flush(cbb);
}

int EVP_AEAD_CTX_deserialize_state(const EVP_AEAD_CTX *ctx, CBS *cbs) {
  // EVP_AEAD_CTX must be initialized by EVP_AEAD_CTX_init first.
  if (!ctx->aead) {
    return 0;
  }

  CBS seq;
  uint64_t version;
  uint64_t aead_id;
  CBS state;

  if (!CBS_get_asn1(cbs, &seq, CBS_ASN1_SEQUENCE)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }

  if (!CBS_get_asn1_uint64(&seq, &version) ||
      version != EVP_AEAD_CTX_SERDE_VERSION) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_SERDE_VERSION);
    return 0;
  }

  if (!CBS_get_asn1_uint64(&seq, &aead_id) || aead_id > UINT16_MAX ||
      aead_id != EVP_AEAD_CTX_get_aead_id(ctx)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_CIPHER_ID);
    return 0;
  }

  if (!CBS_get_asn1(&seq, &state, CBS_ASN1_OCTETSTRING)) {
    OPENSSL_PUT_ERROR(CIPHER, CIPHER_R_SERIALIZATION_INVALID_EVP_AEAD_CTX);
    return 0;
  }

  if (!ctx->aead->deserialize_state) {
    return CBS_len(&state) == 0;
  }

  return ctx->aead->deserialize_state(ctx, &state);
}

uint16_t EVP_AEAD_CTX_get_aead_id(const EVP_AEAD_CTX *ctx) {
  if (!ctx->aead) {
    return AEAD_UNKNOWN_ID;
  }

  return ctx->aead->aead_id;
}
