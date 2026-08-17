// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/evp.h>

#include <openssl/curve25519.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"

typedef struct {
  uint8_t context[255];
  size_t context_len;
} ED25519PH_PKEY_CTX;

static int pkey_ed25519ph_init(EVP_PKEY_CTX *ctx) {
  ED25519PH_PKEY_CTX *dctx = OPENSSL_zalloc(sizeof(ED25519PH_PKEY_CTX));
  if (dctx == NULL) {
    return 0;
  }
  ctx->data = dctx;
  return 1;
}

static void pkey_ed25519ph_cleanup(EVP_PKEY_CTX *ctx) {
  ED25519PH_PKEY_CTX *dctx = ctx->data;
  if (!dctx) {
    return;
  }

  OPENSSL_free(dctx);
}

static int pkey_ed25519ph_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  if (!pkey_ed25519ph_init(dst)) {
    return 0;
  }

  ED25519PH_PKEY_CTX *dctx = dst->data;
  ED25519PH_PKEY_CTX *sctx = src->data;
  GUARD_PTR(dctx);
  GUARD_PTR(sctx);

  OPENSSL_memcpy(dctx->context, sctx->context, sizeof(sctx->context));
  dctx->context_len = sctx->context_len;

  return 1;
}

static int pkey_ed25519ph_sign(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *siglen,
                               const uint8_t *tbs, size_t tbslen) {
  ED25519_KEY *key = ctx->pkey->pkey.ptr;
  if (!key->has_private) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NOT_A_PRIVATE_KEY);
    return 0;
  }

  if (sig == NULL) {
    *siglen = ED25519_SIGNATURE_LEN;
    return 1;
  }

  if (*siglen < ED25519_SIGNATURE_LEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (tbslen < SHA512_DIGEST_LENGTH) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  ED25519PH_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);

  if (!ED25519ph_sign_digest(sig, tbs, key->key, dctx->context,
                             dctx->context_len)) {
    return 0;
  }

  *siglen = ED25519_SIGNATURE_LEN;
  return 1;
}

static int pkey_ed25519ph_verify(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                                 size_t siglen, const uint8_t *tbs,
                                 size_t tbslen) {
  ED25519_KEY *key = ctx->pkey->pkey.ptr;
  ED25519PH_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);

  if (siglen != ED25519_SIGNATURE_LEN || tbslen < SHA512_DIGEST_LENGTH ||
      !ED25519ph_verify_digest(tbs, sig, key->key + ED25519_PUBLIC_KEY_OFFSET,
                               dctx->context, dctx->context_len)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_SIGNATURE);
    return 0;
  }

  return 1;
}

static int pkey_ed25519ph_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  GUARD_PTR(ctx);
  ED25519PH_PKEY_CTX *dctx = (ED25519PH_PKEY_CTX *)ctx->data;
  switch (type) {
    case EVP_PKEY_CTRL_MD: {
      const EVP_MD *md = p2;
      int md_type = EVP_MD_type(md);
      // MUST be SHA-512
      if (md_type != NID_sha512) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
        return 0;
      }
      break;
    }
    case EVP_PKEY_CTRL_SIGNING_CONTEXT: {
      EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS *params = p2;
      if (!params || !dctx ||
          params->context_len > sizeof(dctx->context) ||
          (params->context_len > 0 && !params->context)) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
        return 0;
      }
      OPENSSL_cleanse(dctx->context, sizeof(dctx->context));
      if (params->context_len > 0) {
        OPENSSL_memcpy(dctx->context, params->context, params->context_len);
      }
      dctx->context_len = params->context_len;
      break;
    }
    case EVP_PKEY_CTRL_GET_SIGNING_CONTEXT: {
      EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS *params = p2;
      if (!params || !dctx) {
        return 0;
      }
      if (dctx->context_len == 0) {
        params->context = NULL;
        params->context_len = 0;
      } else {
        params->context = dctx->context;
        params->context_len = dctx->context_len;
      }
      return 1;
    }
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_ed25519ph_pkey_meth) {
  out->pkey_id = EVP_PKEY_ED25519PH;
  out->init = pkey_ed25519ph_init;
  out->copy = pkey_ed25519ph_copy;
  out->cleanup = pkey_ed25519ph_cleanup;
  out->keygen = NULL;
  out->sign_init = NULL;
  out->sign = pkey_ed25519ph_sign;
  out->verify_init = NULL;
  out->verify = pkey_ed25519ph_verify;
  out->verify_message = NULL;
  out->verify_recover = NULL;
  out->encrypt = NULL;
  out->decrypt = NULL;
  out->derive = NULL;
  out->paramgen = NULL;
  out->ctrl = pkey_ed25519ph_ctrl;
  out->ctrl_str = NULL;
  out->keygen_deterministic = NULL;
  out->encapsulate_deterministic = NULL;
  out->encapsulate = NULL;
  out->decapsulate = NULL;
}
