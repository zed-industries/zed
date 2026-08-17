// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <limits.h>

#include <openssl/digest.h>
#include <openssl/err.h>

#include "../fipsmodule/evp/internal.h"


int EVP_SignInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type, ENGINE *impl) {
  return EVP_DigestInit_ex(ctx, type, impl);
}

int EVP_SignInit(EVP_MD_CTX *ctx, const EVP_MD *type) {
  return EVP_DigestInit(ctx, type);
}

int EVP_SignUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_SignFinal(const EVP_MD_CTX *ctx, uint8_t *sig, unsigned *out_sig_len,
                  EVP_PKEY *pkey) {
  uint8_t m[EVP_MAX_MD_SIZE];
  unsigned m_len;
  int ret = 0;
  EVP_MD_CTX tmp_ctx;
  EVP_PKEY_CTX *pkctx = NULL;
  size_t sig_len = EVP_PKEY_size(pkey);

  // Ensure the final result will fit in |unsigned|.
  if (sig_len > UINT_MAX) {
    sig_len = UINT_MAX;
  }

  *out_sig_len = 0;
  EVP_MD_CTX_init(&tmp_ctx);
  if (!EVP_MD_CTX_copy_ex(&tmp_ctx, ctx) ||
      !EVP_DigestFinal_ex(&tmp_ctx, m, &m_len)) {
    goto out;
  }
  EVP_MD_CTX_cleanup(&tmp_ctx);

  pkctx = EVP_PKEY_CTX_new(pkey, NULL);
  if (!pkctx ||  //
      !EVP_PKEY_sign_init(pkctx) ||
      !EVP_PKEY_CTX_set_signature_md(pkctx, ctx->digest) ||
      !EVP_PKEY_sign(pkctx, sig, &sig_len, m, m_len)) {
    goto out;
  }
  *out_sig_len = (unsigned)sig_len;
  ret = 1;

out:
  EVP_PKEY_CTX_free(pkctx);
  return ret;
}

int EVP_VerifyInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type, ENGINE *impl) {
  return EVP_DigestInit_ex(ctx, type, impl);
}

int EVP_VerifyInit(EVP_MD_CTX *ctx, const EVP_MD *type) {
  return EVP_DigestInit(ctx, type);
}

int EVP_VerifyUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_VerifyFinal(EVP_MD_CTX *ctx, const uint8_t *sig, size_t sig_len,
                    EVP_PKEY *pkey) {
  uint8_t m[EVP_MAX_MD_SIZE];
  unsigned m_len;
  int ret = 0;
  EVP_MD_CTX tmp_ctx;
  EVP_PKEY_CTX *pkctx = NULL;

  EVP_MD_CTX_init(&tmp_ctx);
  if (!EVP_MD_CTX_copy_ex(&tmp_ctx, ctx) ||
      !EVP_DigestFinal_ex(&tmp_ctx, m, &m_len)) {
    EVP_MD_CTX_cleanup(&tmp_ctx);
    goto out;
  }
  EVP_MD_CTX_cleanup(&tmp_ctx);

  pkctx = EVP_PKEY_CTX_new(pkey, NULL);
  if (!pkctx ||
      !EVP_PKEY_verify_init(pkctx) ||
      !EVP_PKEY_CTX_set_signature_md(pkctx, ctx->digest)) {
    goto out;
  }
  ret = EVP_PKEY_verify(pkctx, sig, sig_len, m, m_len);

out:
  EVP_PKEY_CTX_free(pkctx);
  return ret;
}
