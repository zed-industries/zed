// Copyright 2014 The BoringSSL Authors
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
#include <string.h>

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
  EVP_AEAD_CTX *ctx =
      reinterpret_cast<EVP_AEAD_CTX *>(OPENSSL_malloc(sizeof(EVP_AEAD_CTX)));
  if (!ctx) {
    return NULL;
  }
  EVP_AEAD_CTX_zero(ctx);

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
  size_t plaintext_len;
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

  plaintext_len = in_len - ctx->tag_len;
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
    OPENSSL_PUT_ERROR(CIPHER, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
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
