// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_CIPHER_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_CIPHER_INTERNAL_H

#include <openssl/base.h>

#include <openssl/aead.h>
#include <openssl/aes.h>

#include "../../internal.h"
#include "../aes/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// EVP_CIPH_MODE_MASK contains the bits of |flags| that represent the mode.
#define EVP_CIPH_MODE_MASK 0x3f

// EVP_AEAD represents a specific AEAD algorithm.
struct evp_aead_st {
  uint8_t key_len;
  uint8_t nonce_len;
  uint8_t overhead;
  uint8_t max_tag_len;
  int seal_scatter_supports_extra_in;

  // init initialises an |EVP_AEAD_CTX|. If this call returns zero then
  // |cleanup| will not be called for that context.
  int (*init)(EVP_AEAD_CTX *, const uint8_t *key, size_t key_len,
              size_t tag_len);
  int (*init_with_direction)(EVP_AEAD_CTX *, const uint8_t *key, size_t key_len,
                             size_t tag_len, enum evp_aead_direction_t dir);
  void (*cleanup)(EVP_AEAD_CTX *);

  int (*open)(const EVP_AEAD_CTX *ctx, uint8_t *out, size_t *out_len,
              size_t max_out_len, const uint8_t *nonce, size_t nonce_len,
              const uint8_t *in, size_t in_len, const uint8_t *ad,
              size_t ad_len);

  int (*seal_scatter)(const EVP_AEAD_CTX *ctx, uint8_t *out, uint8_t *out_tag,
                      size_t *out_tag_len, size_t max_out_tag_len,
                      const uint8_t *nonce, size_t nonce_len, const uint8_t *in,
                      size_t in_len, const uint8_t *extra_in,
                      size_t extra_in_len, const uint8_t *ad, size_t ad_len);

  int (*open_gather)(const EVP_AEAD_CTX *ctx, uint8_t *out,
                     const uint8_t *nonce, size_t nonce_len, const uint8_t *in,
                     size_t in_len, const uint8_t *in_tag, size_t in_tag_len,
                     const uint8_t *ad, size_t ad_len);

  int (*get_iv)(const EVP_AEAD_CTX *ctx, const uint8_t **out_iv,
                size_t *out_len);

  size_t (*tag_len)(const EVP_AEAD_CTX *ctx, size_t in_Len,
                    size_t extra_in_len);
};

struct evp_cipher_st {
  // type contains a NID identifying the cipher. (e.g. NID_aes_128_gcm.)
  int nid;

  // block_size contains the block size, in bytes, of the cipher, or 1 for a
  // stream cipher.
  unsigned block_size;

  // key_len contains the key size, in bytes, for the cipher. If the cipher
  // takes a variable key size then this contains the default size.
  unsigned key_len;

  // iv_len contains the IV size, in bytes, or zero if inapplicable.
  unsigned iv_len;

  // ctx_size contains the size, in bytes, of the per-key context for this
  // cipher.
  unsigned ctx_size;

  // flags contains the OR of a number of flags. See |EVP_CIPH_*|.
  uint32_t flags;

  int (*init)(EVP_CIPHER_CTX *ctx, const uint8_t *key, const uint8_t *iv,
              int enc);

  int (*cipher)(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                size_t inl);

  // cleanup, if non-NULL, releases memory associated with the context. It is
  // called if |EVP_CTRL_INIT| succeeds. Note that |init| may not have been
  // called at this point.
  void (*cleanup)(EVP_CIPHER_CTX *);

  int (*ctrl)(EVP_CIPHER_CTX *, int type, int arg, void *ptr);
};

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_CIPHER_INTERNAL_H
