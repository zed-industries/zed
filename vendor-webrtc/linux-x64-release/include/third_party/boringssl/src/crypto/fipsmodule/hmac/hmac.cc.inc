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

#include <openssl/hmac.h>

#include <assert.h>
#include <string.h>

#include <openssl/digest.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../service_indicator/internal.h"


uint8_t *HMAC(const EVP_MD *evp_md, const void *key, size_t key_len,
              const uint8_t *data, size_t data_len, uint8_t *out,
              unsigned int *out_len) {
  HMAC_CTX ctx;
  HMAC_CTX_init(&ctx);

  // The underlying hash functions should not set the FIPS service indicator
  // until all operations have completed.
  FIPS_service_indicator_lock_state();
  const int ok = HMAC_Init_ex(&ctx, key, key_len, evp_md, NULL) &&
                 HMAC_Update(&ctx, data, data_len) &&
                 HMAC_Final(&ctx, out, out_len);
  FIPS_service_indicator_unlock_state();

  HMAC_CTX_cleanup(&ctx);

  if (!ok) {
    return NULL;
  }

  HMAC_verify_service_indicator(evp_md);
  return out;
}

void HMAC_CTX_init(HMAC_CTX *ctx) {
  ctx->md = NULL;
  EVP_MD_CTX_init(&ctx->i_ctx);
  EVP_MD_CTX_init(&ctx->o_ctx);
  EVP_MD_CTX_init(&ctx->md_ctx);
}

HMAC_CTX *HMAC_CTX_new(void) {
  HMAC_CTX *ctx =
      reinterpret_cast<HMAC_CTX *>(OPENSSL_malloc(sizeof(HMAC_CTX)));
  if (ctx != NULL) {
    HMAC_CTX_init(ctx);
  }
  return ctx;
}

void HMAC_CTX_cleanup(HMAC_CTX *ctx) {
  EVP_MD_CTX_cleanup(&ctx->i_ctx);
  EVP_MD_CTX_cleanup(&ctx->o_ctx);
  EVP_MD_CTX_cleanup(&ctx->md_ctx);
  OPENSSL_cleanse(ctx, sizeof(HMAC_CTX));
}

void HMAC_CTX_cleanse(HMAC_CTX *ctx) {
  EVP_MD_CTX_cleanse(&ctx->i_ctx);
  EVP_MD_CTX_cleanse(&ctx->o_ctx);
  EVP_MD_CTX_cleanse(&ctx->md_ctx);
  OPENSSL_cleanse(ctx, sizeof(HMAC_CTX));
}

void HMAC_CTX_free(HMAC_CTX *ctx) {
  if (ctx == NULL) {
    return;
  }

  HMAC_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

int HMAC_Init_ex(HMAC_CTX *ctx, const void *key, size_t key_len,
                 const EVP_MD *md, ENGINE *impl) {
  int ret = 0;
  FIPS_service_indicator_lock_state();

  if (md == NULL) {
    md = ctx->md;
  }

  // If either |key| is non-NULL or |md| has changed, initialize with a new key
  // rather than rewinding the previous one.
  //
  // TODO(davidben,eroman): Passing the previous |md| with a NULL |key| is
  // ambiguous between using the empty key and reusing the previous key. There
  // exist callers which intend the latter, but the former is an awkward edge
  // case. Fix to API to avoid this.
  if (md != ctx->md || key != NULL) {
    uint8_t pad[EVP_MAX_MD_BLOCK_SIZE];
    uint8_t key_block[EVP_MAX_MD_BLOCK_SIZE];
    unsigned key_block_len;

    size_t block_size = EVP_MD_block_size(md);
    assert(block_size <= sizeof(key_block));
    assert(EVP_MD_size(md) <= block_size);
    if (block_size < key_len) {
      // Long keys are hashed.
      if (!EVP_DigestInit_ex(&ctx->md_ctx, md, impl) ||
          !EVP_DigestUpdate(&ctx->md_ctx, key, key_len) ||
          !EVP_DigestFinal_ex(&ctx->md_ctx, key_block, &key_block_len)) {
        goto out;
      }
    } else {
      assert(key_len <= sizeof(key_block));
      OPENSSL_memcpy(key_block, key, key_len);
      key_block_len = (unsigned)key_len;
    }
    // Keys are then padded with zeros.
    OPENSSL_memset(key_block + key_block_len, 0, block_size - key_block_len);

    for (size_t i = 0; i < block_size; i++) {
      pad[i] = 0x36 ^ key_block[i];
    }
    if (!EVP_DigestInit_ex(&ctx->i_ctx, md, impl) ||
        !EVP_DigestUpdate(&ctx->i_ctx, pad, block_size)) {
      goto out;
    }

    for (size_t i = 0; i < block_size; i++) {
      pad[i] = 0x5c ^ key_block[i];
    }
    if (!EVP_DigestInit_ex(&ctx->o_ctx, md, impl) ||
        !EVP_DigestUpdate(&ctx->o_ctx, pad, block_size)) {
      goto out;
    }

    ctx->md = md;
  }

  ret = EVP_MD_CTX_copy_ex(&ctx->md_ctx, &ctx->i_ctx);

out:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int HMAC_Update(HMAC_CTX *ctx, const uint8_t *data, size_t data_len) {
  return EVP_DigestUpdate(&ctx->md_ctx, data, data_len);
}

int HMAC_Final(HMAC_CTX *ctx, uint8_t *out, unsigned int *out_len) {
  int ret = 0;
  unsigned int i;
  uint8_t buf[EVP_MAX_MD_SIZE];

  FIPS_service_indicator_lock_state();
  // TODO(davidben): The only thing that can officially fail here is
  // |EVP_MD_CTX_copy_ex|, but even that should be impossible in this case.
  if (!EVP_DigestFinal_ex(&ctx->md_ctx, buf, &i) ||
      !EVP_MD_CTX_copy_ex(&ctx->md_ctx, &ctx->o_ctx) ||
      !EVP_DigestUpdate(&ctx->md_ctx, buf, i) ||
      !EVP_DigestFinal_ex(&ctx->md_ctx, out, out_len)) {
    *out_len = 0;
    goto out;
  }

  ret = 1;

out:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    HMAC_verify_service_indicator(ctx->md);
  }
  return ret;
}

size_t HMAC_size(const HMAC_CTX *ctx) { return EVP_MD_size(ctx->md); }

const EVP_MD *HMAC_CTX_get_md(const HMAC_CTX *ctx) { return ctx->md; }

int HMAC_CTX_copy_ex(HMAC_CTX *dest, const HMAC_CTX *src) {
  if (!EVP_MD_CTX_copy_ex(&dest->i_ctx, &src->i_ctx) ||
      !EVP_MD_CTX_copy_ex(&dest->o_ctx, &src->o_ctx) ||
      !EVP_MD_CTX_copy_ex(&dest->md_ctx, &src->md_ctx)) {
    return 0;
  }

  dest->md = src->md;
  return 1;
}

void HMAC_CTX_reset(HMAC_CTX *ctx) {
  HMAC_CTX_cleanup(ctx);
  HMAC_CTX_init(ctx);
}

int HMAC_Init(HMAC_CTX *ctx, const void *key, int key_len, const EVP_MD *md) {
  if (key && md) {
    HMAC_CTX_init(ctx);
  }
  return HMAC_Init_ex(ctx, key, key_len, md, NULL);
}

int HMAC_CTX_copy(HMAC_CTX *dest, const HMAC_CTX *src) {
  HMAC_CTX_init(dest);
  return HMAC_CTX_copy_ex(dest, src);
}
