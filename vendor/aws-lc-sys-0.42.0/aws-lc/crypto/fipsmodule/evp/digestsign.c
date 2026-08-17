// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006,2007 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <openssl/err.h>

#include "../pqdsa/internal.h"
#include "../delocate.h"
#include "../digest/internal.h"
#include "internal.h"

enum evp_sign_verify_t {
  evp_sign,
  evp_verify,
};

DEFINE_LOCAL_DATA(struct evp_md_pctx_ops, EVP_MD_pctx_ops) {
  out->free = EVP_PKEY_CTX_free;
  out->dup = EVP_PKEY_CTX_dup;
}

static int uses_prehash(EVP_MD_CTX *ctx, enum evp_sign_verify_t op) {
  // Pre-hash modes of ML-DSA that uses an external mu calculation differs from
  // other signing algorithms, so we specifically check for NIDs of type NID_MLDSAXX.
  if (ctx->pctx->pkey->type == EVP_PKEY_PQDSA &&
      ctx->pctx->pkey->pkey.pqdsa_key != NULL) {
    int nid = ctx->pctx->pkey->pkey.pqdsa_key->pqdsa->nid;

    if (nid == NID_MLDSA44 || nid == NID_MLDSA65 || nid == NID_MLDSA87) {
      return 0;
    }
  }

    return (op == evp_sign) ? (ctx->pctx->pmeth->sign != NULL)
                            : (ctx->pctx->pmeth->verify != NULL);
}

static int hmac_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  HMAC_PKEY_CTX *hctx = ctx->pctx->data;
  // HMAC_Update returns 1 on success and 0 on failure.
  return HMAC_Update(&hctx->ctx, data, count);
}

static int HMAC_DigestFinal_ex(EVP_MD_CTX *ctx, uint8_t *out_sig,
                               size_t *out_sig_len) {
  unsigned int mdlen;
  if (*out_sig_len < EVP_MD_CTX_size(ctx)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  HMAC_PKEY_CTX *hctx = ctx->pctx->data;
  if (!HMAC_Final(&hctx->ctx, out_sig, &mdlen)) {
    return 0;
  }

  *out_sig_len = (size_t)mdlen;
  return 1;
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
  ctx->pctx_ops = EVP_MD_pctx_ops();

  if (op == evp_verify) {
    if (!EVP_PKEY_verify_init(ctx->pctx)) {
      return 0;
    }
  } else {
    if (pkey->type == EVP_PKEY_HMAC) {
      // |ctx->update| gets repurposed as a hook to call |HMAC_Update|.
      // |ctx->update| is normally copied from |ctx->digest->update|, but
      // |EVP_PKEY_HMAC| has its own definition. We suppress the automatic
      // setting of |mctx->update| and the rest of its initialization here.
      ctx->pctx->operation = EVP_PKEY_OP_SIGN;
      ctx->flags |= EVP_MD_CTX_HMAC;
      ctx->update = hmac_update;
    } else {
      if (!EVP_PKEY_sign_init(ctx->pctx)) {
        return 0;
      }
    }
  }

  if (type != NULL && !EVP_PKEY_CTX_set_signature_md(ctx->pctx, type)) {
    return 0;
  }

  if (uses_prehash(ctx, op) || used_for_hmac(ctx)) {
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
  SET_DIT_AUTO_RESET;
  return do_sigver_init(ctx, pctx, type, e, pkey, evp_sign);
}

int EVP_DigestVerifyInit(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
                         const EVP_MD *type, ENGINE *e, EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET;
  return do_sigver_init(ctx, pctx, type, e, pkey, evp_verify);
}

int EVP_DigestSignUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  SET_DIT_AUTO_RESET;
  if (!uses_prehash(ctx, evp_sign) && !used_for_hmac(ctx)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_DigestVerifyUpdate(EVP_MD_CTX *ctx, const void *data, size_t len) {
  SET_DIT_AUTO_RESET;
  if (!uses_prehash(ctx, evp_verify) || used_for_hmac(ctx)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return EVP_DigestUpdate(ctx, data, len);
}

int EVP_DigestSignFinal(EVP_MD_CTX *ctx, uint8_t *out_sig,
                        size_t *out_sig_len) {
  SET_DIT_AUTO_RESET;
  if (!uses_prehash(ctx, evp_sign) && !used_for_hmac(ctx)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  if (out_sig) {
    EVP_MD_CTX tmp_ctx;
    int ret = 0;
    uint8_t md[EVP_MAX_MD_SIZE];
    unsigned int mdlen;
    // We have to avoid the underlying SHA services updating the indicator
    // state, so we lock the state here.
    FIPS_service_indicator_lock_state();
    EVP_MD_CTX_init(&tmp_ctx);
    if (EVP_MD_CTX_copy_ex(&tmp_ctx, ctx)) {
      if (used_for_hmac(ctx)) {
        ret = HMAC_DigestFinal_ex(&tmp_ctx, out_sig, out_sig_len);
      } else {
        ret = EVP_DigestFinal_ex(&tmp_ctx, md, &mdlen) &&
              EVP_PKEY_sign(ctx->pctx, out_sig, out_sig_len, md, mdlen);
      }
    }

    EVP_MD_CTX_cleanup(&tmp_ctx);
    FIPS_service_indicator_unlock_state();
    if (ret > 0) {
      EVP_DigestSign_verify_service_indicator(ctx);
    }
    return ret;
  } else {
    // This only determines the size of the signature. This case of
    // |EVP_DigestSignFinal| should not return an approval check because no
    // crypto is being done.
    if (used_for_hmac(ctx)) {
      // This is only defined in |EVP_PKEY_HMAC|.
      *out_sig_len = EVP_MD_CTX_size(ctx);
      return 1;
    } else {
      size_t s = EVP_MD_size(ctx->digest);
      return EVP_PKEY_sign(ctx->pctx, out_sig, out_sig_len, NULL, s);
    }
  }
}

int EVP_DigestVerifyFinal(EVP_MD_CTX *ctx, const uint8_t *sig, size_t sig_len) {
  SET_DIT_AUTO_RESET;
  if (!uses_prehash(ctx, evp_verify) || used_for_hmac(ctx)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  // We have to avoid the underlying SHA services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  EVP_MD_CTX tmp_ctx;
  int ret;
  uint8_t md[EVP_MAX_MD_SIZE];
  unsigned int mdlen;

  EVP_MD_CTX_init(&tmp_ctx);
  ret = EVP_MD_CTX_copy_ex(&tmp_ctx, ctx) &&
        EVP_DigestFinal_ex(&tmp_ctx, md, &mdlen) &&
        EVP_PKEY_verify(ctx->pctx, sig, sig_len, md, mdlen);
  EVP_MD_CTX_cleanup(&tmp_ctx);

  FIPS_service_indicator_unlock_state();
  if (ret > 0) {
    EVP_DigestVerify_verify_service_indicator(ctx);
  }
  return ret;
}

int EVP_DigestSign(EVP_MD_CTX *ctx, uint8_t *out_sig, size_t *out_sig_len,
                   const uint8_t *data, size_t data_len) {
  GUARD_PTR(ctx->pctx);
  // We have to avoid the underlying |EVP_DigestSignFinal| services updating
  // the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SET_DIT_AUTO_RESET;
  int ret = 0;

  if (uses_prehash(ctx, evp_sign) || used_for_hmac(ctx)) {
    // If |out_sig| is NULL, the caller is only querying the maximum output
    // length. |data| should only be incorporated in the final call.
    if (out_sig != NULL && !EVP_DigestSignUpdate(ctx, data, data_len)) {
      goto end;
    }

    ret = EVP_DigestSignFinal(ctx, out_sig, out_sig_len);
    goto end;
  }

  if (ctx->pctx->pmeth->sign_message == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  // This is executed when |uses_prehash| is not true, which is the case for
  // Ed25519 and ML-DSA when in pure mode.
  ret = ctx->pctx->pmeth->sign_message(ctx->pctx, out_sig, out_sig_len, data,
                                       data_len);
end:
  FIPS_service_indicator_unlock_state();
  if (ret > 0 && out_sig != NULL) {
    // Indicator should only be set if we performed crypto, don't set if we only
    // performed a size check.
    EVP_DigestSign_verify_service_indicator(ctx);
  }
  return ret;
}

int EVP_DigestVerify(EVP_MD_CTX *ctx, const uint8_t *sig, size_t sig_len,
                     const uint8_t *data, size_t len) {
  GUARD_PTR(ctx->pctx);
  // We have to avoid the underlying |EVP_DigestSignFinal| services updating
  // the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  SET_DIT_AUTO_RESET;
  int ret = 0;

  if (uses_prehash(ctx, evp_verify) && !used_for_hmac(ctx)) {
    ret = EVP_DigestVerifyUpdate(ctx, data, len) &&
          EVP_DigestVerifyFinal(ctx, sig, sig_len);
    goto end;
  }

  if (ctx->pctx->pmeth->verify_message == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  // This is executed when |uses_prehash| is not true, which is the case for
  // Ed25519 and ML-DSA when in pure mode.
  ret = ctx->pctx->pmeth->verify_message(ctx->pctx, sig, sig_len, data, len);

end:
  FIPS_service_indicator_unlock_state();
  if (ret > 0) {
    EVP_DigestVerify_verify_service_indicator(ctx);
  }
  return ret;
}

void EVP_MD_CTX_set_pkey_ctx(EVP_MD_CTX *ctx, EVP_PKEY_CTX *pctx) {
  SET_DIT_AUTO_RESET;
  // |pctx| could be null, so we have to deal with the cleanup job here.
  if (!(ctx->flags & EVP_MD_CTX_FLAG_KEEP_PKEY_CTX)) {
    EVP_PKEY_CTX_free(ctx->pctx);
  }

  ctx->pctx = pctx;
  ctx->pctx_ops = EVP_MD_pctx_ops();

  if (pctx != NULL) {
    // make sure |pctx| is not freed when destroying |EVP_MD_CTX|
    ctx->flags |= EVP_MD_CTX_FLAG_KEEP_PKEY_CTX;
  } else {
    // if |pctx| is null, we remove the flag.
    ctx->flags &= ~EVP_MD_CTX_FLAG_KEEP_PKEY_CTX;
  }
}

EVP_PKEY_CTX *EVP_MD_CTX_get_pkey_ctx(const EVP_MD_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if(ctx == NULL) {
    return NULL;
  }
  return ctx->pctx;
}

EVP_PKEY_CTX *EVP_MD_CTX_pkey_ctx(const EVP_MD_CTX *ctx) {
  return EVP_MD_CTX_get_pkey_ctx(ctx);
}
