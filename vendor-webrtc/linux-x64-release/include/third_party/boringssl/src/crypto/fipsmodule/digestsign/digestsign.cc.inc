// Copyright 2006-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/evp.h>

#include <openssl/err.h>

#include "../../evp/internal.h"
#include "../delocate.h"
#include "../digest/internal.h"
#include "../service_indicator/internal.h"


enum evp_sign_verify_t {
  evp_sign,
  evp_verify,
};

DEFINE_LOCAL_DATA(struct evp_md_pctx_ops, md_pctx_ops) {
  out->free = EVP_PKEY_CTX_free;
  out->dup = EVP_PKEY_CTX_dup;
}

static int uses_prehash(EVP_MD_CTX *ctx, enum evp_sign_verify_t op) {
  return (op == evp_sign) ? (ctx->pctx->pmeth->sign != NULL)
                          : (ctx->pctx->pmeth->verify != NULL);
}

static int do_sigver_init(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
                          const EVP_MD *type, ENGINE *e, EVP_PKEY *pkey,
                          enum evp_sign_verify_t op) {
  if (ctx->pctx == NULL) {
    ctx->pctx = EVP_PKEY_CTX_new(pkey, e);
  }
  if (ctx->pctx == NULL) {
    return 0;
  }
  ctx->pctx_ops = md_pctx_ops();

  if (op == evp_verify) {
    if (!EVP_PKEY_verify_init(ctx->pctx)) {
      return 0;
    }
  } else {
    if (!EVP_PKEY_sign_init(ctx->pctx)) {
      return 0;
    }
  }

  if (type != NULL &&
      !EVP_PKEY_CTX_set_signature_md(ctx->pctx, type)) {
    return 0;
  }

  if (uses_prehash(ctx, op)) {
    if (type == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_DEFAULT_DIGEST);
      return 0;
    }
    if (!EVP_DigestInit_ex(ctx, type, e)) {
      return 0;
    }
  }

  if (pctx) {
    *pctx = ctx->pctx;
  }
  return 1;
}

int EVP_DigestSignInit(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx, const EVP_MD *type,
                       ENGINE *e, EVP_PKEY *pkey) {
  return do_sigver_init(ctx, pctx, type, e, pkey, evp_sign);
}

int EVP_DigestVerifyInit(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
                         const EVP_MD *type, ENGINE *e, EVP_PKEY *pkey) {
  return do_sigver_init(ctx, pctx, type, e, pkey, evp_verify);
}

int EVP_DigestSignUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  if (!uses_prehash(ctx, evp_sign)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_DigestVerifyUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  if (!uses_prehash(ctx, evp_verify)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_DigestSignFinal(EVP_MD_CTX *ctx, uint8_t *out_sig,
                        size_t *out_sig_len) {
  if (!uses_prehash(ctx, evp_sign)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  if (out_sig) {
    EVP_MD_CTX tmp_ctx;
    int ret;
    uint8_t md[EVP_MAX_MD_SIZE];
    unsigned int mdlen;

    FIPS_service_indicator_lock_state();
    EVP_MD_CTX_init(&tmp_ctx);
    ret = EVP_MD_CTX_copy_ex(&tmp_ctx, ctx) &&
          EVP_DigestFinal_ex(&tmp_ctx, md, &mdlen) &&
          EVP_PKEY_sign(ctx->pctx, out_sig, out_sig_len, md, mdlen);
    EVP_MD_CTX_cleanup(&tmp_ctx);
    FIPS_service_indicator_unlock_state();

    if (ret) {
      EVP_DigestSign_verify_service_indicator(ctx);
    }

    return ret;
  } else {
    size_t s = EVP_MD_size(ctx->digest);
    return EVP_PKEY_sign(ctx->pctx, out_sig, out_sig_len, NULL, s);
  }
}

int EVP_DigestVerifyFinal(EVP_MD_CTX *ctx, const uint8_t *sig,
                          size_t sig_len) {
  if (!uses_prehash(ctx, evp_verify)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  EVP_MD_CTX tmp_ctx;
  int ret;
  uint8_t md[EVP_MAX_MD_SIZE];
  unsigned int mdlen;

  FIPS_service_indicator_lock_state();
  EVP_MD_CTX_init(&tmp_ctx);
  ret = EVP_MD_CTX_copy_ex(&tmp_ctx, ctx) &&
        EVP_DigestFinal_ex(&tmp_ctx, md, &mdlen) &&
        EVP_PKEY_verify(ctx->pctx, sig, sig_len, md, mdlen);
  FIPS_service_indicator_unlock_state();
  EVP_MD_CTX_cleanup(&tmp_ctx);

  if (ret) {
    EVP_DigestVerify_verify_service_indicator(ctx);
  }

  return ret;
}

int EVP_DigestSign(EVP_MD_CTX *ctx, uint8_t *out_sig, size_t *out_sig_len,
                   const uint8_t *data, size_t data_len) {
  FIPS_service_indicator_lock_state();
  int ret = 0;

  if (uses_prehash(ctx, evp_sign)) {
    // If |out_sig| is NULL, the caller is only querying the maximum output
    // length. |data| should only be incorporated in the final call.
    if (out_sig != NULL &&
        !EVP_DigestSignUpdate(ctx, data, data_len)) {
      goto end;
    }

    ret = EVP_DigestSignFinal(ctx, out_sig, out_sig_len);
    goto end;
  }

  if (ctx->pctx->pmeth->sign_message == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  ret = ctx->pctx->pmeth->sign_message(ctx->pctx, out_sig, out_sig_len, data,
                                       data_len);

end:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    EVP_DigestSign_verify_service_indicator(ctx);
  }
  return ret;
}

int EVP_DigestVerify(EVP_MD_CTX *ctx, const uint8_t *sig, size_t sig_len,
                     const uint8_t *data, size_t len) {
  FIPS_service_indicator_lock_state();
  int ret = 0;

  if (uses_prehash(ctx, evp_verify)) {
    ret = EVP_DigestVerifyUpdate(ctx, data, len) &&
          EVP_DigestVerifyFinal(ctx, sig, sig_len);
    goto end;
  }

  if (ctx->pctx->pmeth->verify_message == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  ret = ctx->pctx->pmeth->verify_message(ctx->pctx, sig, sig_len, data, len);

end:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    EVP_DigestVerify_verify_service_indicator(ctx);
  }
  return ret;
}
