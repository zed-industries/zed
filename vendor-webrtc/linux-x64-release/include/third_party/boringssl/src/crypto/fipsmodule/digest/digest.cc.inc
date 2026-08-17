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

#include <openssl/digest.h>

#include <assert.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


int EVP_MD_type(const EVP_MD *md) { return md->type; }

int EVP_MD_nid(const EVP_MD *md) { return EVP_MD_type(md); }

uint32_t EVP_MD_flags(const EVP_MD *md) { return md->flags; }

size_t EVP_MD_size(const EVP_MD *md) { return md->md_size; }

size_t EVP_MD_block_size(const EVP_MD *md) { return md->block_size; }


void EVP_MD_CTX_init(EVP_MD_CTX *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(EVP_MD_CTX));
}

EVP_MD_CTX *EVP_MD_CTX_new(void) {
  EVP_MD_CTX *ctx =
      reinterpret_cast<EVP_MD_CTX *>(OPENSSL_malloc(sizeof(EVP_MD_CTX)));

  if (ctx) {
    EVP_MD_CTX_init(ctx);
  }

  return ctx;
}

EVP_MD_CTX *EVP_MD_CTX_create(void) { return EVP_MD_CTX_new(); }

int EVP_MD_CTX_cleanup(EVP_MD_CTX *ctx) {
  OPENSSL_free(ctx->md_data);

  assert(ctx->pctx == NULL || ctx->pctx_ops != NULL);
  if (ctx->pctx_ops) {
    ctx->pctx_ops->free(ctx->pctx);
  }

  EVP_MD_CTX_init(ctx);

  return 1;
}

void EVP_MD_CTX_cleanse(EVP_MD_CTX *ctx) {
  OPENSSL_cleanse(ctx->md_data, ctx->digest->ctx_size);
  EVP_MD_CTX_cleanup(ctx);
}

void EVP_MD_CTX_free(EVP_MD_CTX *ctx) {
  if (!ctx) {
    return;
  }

  EVP_MD_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

void EVP_MD_CTX_destroy(EVP_MD_CTX *ctx) { EVP_MD_CTX_free(ctx); }

int EVP_DigestFinalXOF(EVP_MD_CTX *ctx, uint8_t *out, size_t len) {
  OPENSSL_PUT_ERROR(DIGEST, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
  return 0;
}

uint32_t EVP_MD_meth_get_flags(const EVP_MD *md) { return EVP_MD_flags(md); }

void EVP_MD_CTX_set_flags(EVP_MD_CTX *ctx, int flags) {}

int EVP_MD_CTX_copy_ex(EVP_MD_CTX *out, const EVP_MD_CTX *in) {
  // |in->digest| may be NULL if this is a signing |EVP_MD_CTX| for, e.g.,
  // Ed25519 which does not hash with |EVP_MD_CTX|.
  if (in == NULL || (in->pctx == NULL && in->digest == NULL)) {
    OPENSSL_PUT_ERROR(DIGEST, DIGEST_R_INPUT_NOT_INITIALIZED);
    return 0;
  }

  EVP_PKEY_CTX *pctx = NULL;
  assert(in->pctx == NULL || in->pctx_ops != NULL);
  if (in->pctx) {
    pctx = in->pctx_ops->dup(in->pctx);
    if (!pctx) {
      return 0;
    }
  }

  uint8_t *tmp_buf = NULL;
  if (in->digest != NULL) {
    if (out->digest != in->digest) {
      assert(in->digest->ctx_size != 0);
      tmp_buf =
          reinterpret_cast<uint8_t *>(OPENSSL_malloc(in->digest->ctx_size));
      if (tmp_buf == NULL) {
        if (pctx) {
          in->pctx_ops->free(pctx);
        }
        return 0;
      }
    } else {
      // |md_data| will be the correct size in this case. It's removed from
      // |out| so that |EVP_MD_CTX_cleanup| doesn't free it, and then it's
      // reused.
      tmp_buf = reinterpret_cast<uint8_t *>(out->md_data);
      out->md_data = NULL;
    }
  }

  EVP_MD_CTX_cleanup(out);

  out->digest = in->digest;
  out->md_data = tmp_buf;
  if (in->digest != NULL) {
    OPENSSL_memcpy(out->md_data, in->md_data, in->digest->ctx_size);
  }
  out->pctx = pctx;
  out->pctx_ops = in->pctx_ops;
  assert(out->pctx == NULL || out->pctx_ops != NULL);

  return 1;
}

void EVP_MD_CTX_move(EVP_MD_CTX *out, EVP_MD_CTX *in) {
  EVP_MD_CTX_cleanup(out);
  // While not guaranteed, |EVP_MD_CTX| is currently safe to move with |memcpy|.
  // bssl-crypto currently relies on this, however, so if we change this, we
  // need to box the |HMAC_CTX|. (Relying on this is only fine because we assume
  // BoringSSL and bssl-crypto will always be updated atomically. We do not
  // allow any version skew between the two.)
  OPENSSL_memcpy(out, in, sizeof(EVP_MD_CTX));
  EVP_MD_CTX_init(in);
}

int EVP_MD_CTX_copy(EVP_MD_CTX *out, const EVP_MD_CTX *in) {
  EVP_MD_CTX_init(out);
  return EVP_MD_CTX_copy_ex(out, in);
}

int EVP_MD_CTX_reset(EVP_MD_CTX *ctx) {
  EVP_MD_CTX_cleanup(ctx);
  EVP_MD_CTX_init(ctx);
  return 1;
}

int EVP_DigestInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type, ENGINE *engine) {
  if (ctx->digest != type) {
    assert(type->ctx_size != 0);
    uint8_t *md_data =
        reinterpret_cast<uint8_t *>(OPENSSL_malloc(type->ctx_size));
    if (md_data == NULL) {
      return 0;
    }

    OPENSSL_free(ctx->md_data);
    ctx->md_data = md_data;
    ctx->digest = type;
  }

  assert(ctx->pctx == NULL || ctx->pctx_ops != NULL);

  ctx->digest->init(ctx);
  return 1;
}

int EVP_DigestInit(EVP_MD_CTX *ctx, const EVP_MD *type) {
  EVP_MD_CTX_init(ctx);
  return EVP_DigestInit_ex(ctx, type, NULL);
}

int EVP_DigestUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  ctx->digest->update(ctx, data, len);
  return 1;
}

int EVP_DigestFinal_ex(EVP_MD_CTX *ctx, uint8_t *md_out, unsigned int *size) {
  assert(ctx->digest->md_size <= EVP_MAX_MD_SIZE);
  ctx->digest->final(ctx, md_out);
  if (size != NULL) {
    *size = ctx->digest->md_size;
  }
  OPENSSL_cleanse(ctx->md_data, ctx->digest->ctx_size);
  return 1;
}

int EVP_DigestFinal(EVP_MD_CTX *ctx, uint8_t *md, unsigned int *size) {
  (void)EVP_DigestFinal_ex(ctx, md, size);
  EVP_MD_CTX_cleanup(ctx);
  return 1;
}

int EVP_Digest(const void *data, size_t count, uint8_t *out_md,
               unsigned int *out_size, const EVP_MD *type, ENGINE *impl) {
  bssl::ScopedEVP_MD_CTX ctx;
  return EVP_DigestInit_ex(ctx.get(), type, impl) &&
         EVP_DigestUpdate(ctx.get(), data, count) &&
         EVP_DigestFinal_ex(ctx.get(), out_md, out_size);
}

const EVP_MD *EVP_MD_CTX_get0_md(const EVP_MD_CTX *ctx) {
  if (ctx == NULL) {
    return NULL;
  }
  return ctx->digest;
}

const EVP_MD *EVP_MD_CTX_md(const EVP_MD_CTX *ctx) {
  return EVP_MD_CTX_get0_md(ctx);
}

size_t EVP_MD_CTX_size(const EVP_MD_CTX *ctx) {
  return EVP_MD_size(EVP_MD_CTX_get0_md(ctx));
}

size_t EVP_MD_CTX_block_size(const EVP_MD_CTX *ctx) {
  return EVP_MD_block_size(EVP_MD_CTX_get0_md(ctx));
}

int EVP_MD_CTX_type(const EVP_MD_CTX *ctx) {
  return EVP_MD_type(EVP_MD_CTX_get0_md(ctx));
}

int EVP_add_digest(const EVP_MD *digest) { return 1; }
